use std::io;
use std::sync::Arc;
use std::time::Duration;

use yansi::Paint;
use tokio::sync::oneshot;
use futures::stream::StreamExt;
use futures::future::{self, FutureExt, Future, TryFutureExt, BoxFuture};

use crate::{route, Rocket, Orbit, Request, Response, Data, Config};
use crate::form::Form;
use crate::outcome::Outcome;
use crate::error::{Error, ErrorKind};
use crate::ext::{AsyncReadExt, CancellableListener, CancellableIo};
use crate::request::ConnectionMeta;

use crate::http::{uri::Origin, hyper, Method, Status, Header};
use crate::http::private::{bind_tcp, Listener, Connection, Incoming};

// A token returned to force the execution of one method before another.
pub(crate) struct RequestToken;

async fn handle<Fut, T, F>(name: Option<&str>, run: F) -> Option<T>
    where F: FnOnce() -> Fut, Fut: Future<Output = T>,
{
    use std::panic::AssertUnwindSafe;

    macro_rules! panic_info {
        ($name:expr, $e:expr) => {{
            match $name {
                Some(name) => error_!("Handler {} panicked.", Paint::white(name)),
                None => error_!("A handler panicked.")
            };

            info_!("This is an application bug.");
            info_!("A panic in Rust must be treated as an exceptional event.");
            info_!("Panicking is not a suitable error handling mechanism.");
            info_!("Unwinding, the result of a panic, is an expensive operation.");
            info_!("Panics will degrade application performance.");
            info_!("Instead of panicking, return `Option` and/or `Result`.");
            info_!("Values of either type can be returned directly from handlers.");
            warn_!("A panic is treated as an internal server error.");
            $e
        }}
    }

    let run = AssertUnwindSafe(run);
    let fut = std::panic::catch_unwind(move || run())
        .map_err(|e| panic_info!(name, e))
        .ok()?;

    AssertUnwindSafe(fut)
        .catch_unwind()
        .await
        .map_err(|e| panic_info!(name, e))
        .ok()
}

// This function tries to hide all of the Hyper-ness from Rocket. It essentially
// converts Hyper types into Rocket types, then calls the `dispatch` function,
// which knows nothing about Hyper. Because responding depends on the
// `HyperResponse` type, this function does the actual response processing.
async fn hyper_service_fn(
    rocket: Arc<Rocket<Orbit>>,
    conn: ConnectionMeta,
    hyp_req: hyper::Request<hyper::Body>,
) -> Result<hyper::Response<hyper::Body>, io::Error> {
    // This future must return a hyper::Response, but the response body might
    // borrow from the request. Instead, write the body in another future that
    // sends the response metadata (and a body channel) prior.
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        // Convert a Hyper request into a Rocket request.
        let (h_parts, mut h_body) = hyp_req.into_parts();
        match Request::from_hyp(&rocket, &h_parts, Some(conn)) {
            Ok(mut req) => {
                // Convert into Rocket `Data`, dispatch request, write response.
                let mut data = Data::from(&mut h_body);
                let token = rocket.preprocess_request(&mut req, &mut data).await;
                let response = rocket.dispatch(token, &mut req, data).await;
                rocket.send_response(response, tx).await;
            },
            Err(e) => {
                // TODO: We don't have a request to pass in, so we fabricate
                // one. This is weird. Instead, let the user know that we failed
                // to parse a request (a special handler?).
                error!("Bad incoming request: {}", e);
                let dummy = Request::new(&rocket, Method::Get, Origin::ROOT);
                let response = rocket.handle_error(Status::BadRequest, &dummy).await;
                rocket.send_response(response, tx).await;
            }
        }
    });

    // Receive the response written to `tx` by the task above.
    rx.await.map_err(|e| io::Error::new(io::ErrorKind::BrokenPipe, e))
}

impl Rocket<Orbit> {
    /// Wrapper around `_send_response` to log a success or failure.
    #[inline]
    async fn send_response(
        &self,
        response: Response<'_>,
        tx: oneshot::Sender<hyper::Response<hyper::Body>>,
    ) {
        let remote_hungup = |e: &io::Error| match e.kind() {
            | io::ErrorKind::BrokenPipe
            | io::ErrorKind::ConnectionReset
            | io::ErrorKind::ConnectionAborted => true,
            _ => false,
        };

        match self._send_response(response, tx).await {
            Ok(()) => info_!("{}", Paint::green("Response succeeded.")),
            Err(e) if remote_hungup(&e) => warn_!("Remote left: {}.", e),
            Err(e) => warn_!("Failed to write response: {}.", e),
        }
    }

    /// Attempts to create a hyper response from `response` and send it to `tx`.
    #[inline]
    async fn _send_response(
        &self,
        mut response: Response<'_>,
        tx: oneshot::Sender<hyper::Response<hyper::Body>>,
    ) -> io::Result<()> {
        let mut hyp_res = hyper::Response::builder();

        hyp_res = hyp_res.status(response.status().code);
        for header in response.headers().iter() {
            let name = header.name.as_str();
            let value = header.value.as_bytes();
            hyp_res = hyp_res.header(name, value);
        }

        let body = response.body_mut();
        if let Some(n) = body.size().await {
            hyp_res = hyp_res.header(hyper::header::CONTENT_LENGTH, n);
        }

        let (mut sender, hyp_body) = hyper::Body::channel();
        let hyp_response = hyp_res.body(hyp_body)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        tx.send(hyp_response).map_err(|_| {
            let msg = "client disconnect before response started";
            io::Error::new(io::ErrorKind::BrokenPipe, msg)
        })?;

        let max_chunk_size = body.max_chunk_size();
        let mut stream = body.into_bytes_stream(max_chunk_size);
        while let Some(next) = stream.next().await {
            sender.send_data(next?).await
                .map_err(|e| io::Error::new(io::ErrorKind::BrokenPipe, e))?;
        }

        Ok(())
    }

    /// Preprocess the request for Rocket things. Currently, this means:
    ///
    ///   * Rewriting the method in the request if _method form field exists.
    ///   * Run the request fairings.
    ///
    /// Keep this in-sync with derive_form when preprocessing form fields.
    pub(crate) async fn preprocess_request(
        &self,
        req: &mut Request<'_>,
        data: &mut Data<'_>
    ) -> RequestToken {
        // Check if this is a form and if the form contains the special _method
        // field which we use to reinterpret the request's method.
        let (min_len, max_len) = ("_method=get".len(), "_method=delete".len());
        let peek_buffer = data.peek(max_len).await;
        let is_form = req.content_type().map_or(false, |ct| ct.is_form());

        if is_form && req.method() == Method::Post && peek_buffer.len() >= min_len {
            let method = std::str::from_utf8(peek_buffer).ok()
                .and_then(|raw_form| Form::values(raw_form).next())
                .filter(|field| field.name == "_method")
                .and_then(|field| field.value.parse().ok());

            if let Some(method) = method {
                req._set_method(method);
            }
        }

        // Run request fairings.
        self.fairings.handle_request(req, data).await;

        RequestToken
    }

    #[inline]
    pub(crate) async fn dispatch<'s, 'r: 's>(
        &'s self,
        _token: RequestToken,
        request: &'r Request<'s>,
        data: Data<'r>
    ) -> Response<'r> {
        info!("{}:", request);

        // Remember if the request is `HEAD` for later body stripping.
        let was_head_request = request.method() == Method::Head;

        // Route the request and run the user's handlers.
        let mut response = self.route_and_process(request, data).await;

        // Add a default 'Server' header if it isn't already there.
        // TODO: If removing Hyper, write out `Date` header too.
        if let Some(ident) = request.rocket().config.ident.as_str() {
            if !response.headers().contains("Server") {
                response.set_header(Header::new("Server", ident));
            }
        }

        // Run the response fairings.
        self.fairings.handle_response(request, &mut response).await;

        // Strip the body if this is a `HEAD` request.
        if was_head_request {
            response.strip_body();
        }

        response
    }

    async fn route_and_process<'s, 'r: 's>(
        &'s self,
        request: &'r Request<'s>,
        data: Data<'r>
    ) -> Response<'r> {
        let mut response = match self.route(request, data).await {
            Outcome::Success(response) => response,
            Outcome::Forward(data) if request.method() == Method::Head => {
                info_!("Autohandling {} request.", Paint::default("HEAD").bold());

                // Dispatch the request again with Method `GET`.
                request._set_method(Method::Get);
                match self.route(request, data).await {
                    Outcome::Success(response) => response,
                    Outcome::Failure(status) => self.handle_error(status, request).await,
                    Outcome::Forward(_) => self.handle_error(Status::NotFound, request).await,
                }
            }
            Outcome::Forward(_) => self.handle_error(Status::NotFound, request).await,
            Outcome::Failure(status) => self.handle_error(status, request).await,
        };

        // Set the cookies. Note that error responses will only include cookies
        // set by the error handler. See `handle_error` for more.
        let delta_jar = request.cookies().take_delta_jar();
        for cookie in delta_jar.delta() {
            response.adjoin_header(cookie);
        }

        response
    }

    /// Tries to find a `Responder` for a given `request`. It does this by
    /// routing the request and calling the handler for each matching route
    /// until one of the handlers returns success or failure, or there are no
    /// additional routes to try (forward). The corresponding outcome for each
    /// condition is returned.
    #[inline]
    async fn route<'s, 'r: 's>(
        &'s self,
        request: &'r Request<'s>,
        mut data: Data<'r>,
    ) -> route::Outcome<'r> {
        // Go through the list of matching routes until we fail or succeed.
        for route in self.router.route(request) {
            // Retrieve and set the requests parameters.
            info_!("Matched: {}", route);
            request.set_route(route);

            let name = route.name.as_deref();
            let outcome = handle(name, || route.handler.handle(request, data)).await
                .unwrap_or(Outcome::Failure(Status::InternalServerError));

            // Check if the request processing completed (Some) or if the
            // request needs to be forwarded. If it does, continue the loop
            // (None) to try again.
            info_!("{} {}", Paint::default("Outcome:").bold(), outcome);
            match outcome {
                o@Outcome::Success(_) | o@Outcome::Failure(_) => return o,
                Outcome::Forward(unused_data) => data = unused_data,
            }
        }

        error_!("No matching routes for {}.", request);
        Outcome::Forward(data)
    }

    /// Invokes the handler with `req` for catcher with status `status`.
    ///
    /// In order of preference, invoked handler is:
    ///   * the user's registered handler for `status`
    ///   * the user's registered `default` handler
    ///   * Rocket's default handler for `status`
    ///
    /// Return `Ok(result)` if the handler succeeded. Returns `Ok(Some(Status))`
    /// if the handler ran to completion but failed. Returns `Ok(None)` if the
    /// handler panicked while executing.
    async fn invoke_catcher<'s, 'r: 's>(
        &'s self,
        status: Status,
        req: &'r Request<'s>
    ) -> Result<Response<'r>, Option<Status>> {
        // For now, we reset the delta state to prevent any modifications
        // from earlier, unsuccessful paths from being reflected in error
        // response. We may wish to relax this in the future.
        req.cookies().reset_delta();

        if let Some(catcher) = self.router.catch(status, req) {
            warn_!("Responding with registered {} catcher.", catcher);
            let name = catcher.name.as_deref();
            handle(name, || catcher.handler.handle(status, req)).await
                .map(|result| result.map_err(Some))
                .unwrap_or_else(|| Err(None))
        } else {
            let code = Paint::blue(status.code).bold();
            warn_!("No {} catcher registered. Using Rocket default.", code);
            Ok(crate::catcher::default_handler(status, req))
        }
    }

    // Invokes the catcher for `status`. Returns the response on success.
    //
    // On catcher failure, the 500 error catcher is attempted. If _that_ fails,
    // the (infallible) default 500 error cather is used.
    pub(crate) async fn handle_error<'s, 'r: 's>(
        &'s self,
        mut status: Status,
        req: &'r Request<'s>
    ) -> Response<'r> {
        // Dispatch to the `status` catcher.
        if let Ok(r) = self.invoke_catcher(status, req).await {
            return r;
        }

        // If it fails and it's not a 500, try the 500 catcher.
        if status != Status::InternalServerError {
            error_!("Catcher failed. Attemping 500 error catcher.");
            status = Status::InternalServerError;
            if let Ok(r) = self.invoke_catcher(status, req).await {
                return r;
            }
        }

        // If it failed again or if it was already a 500, use Rocket's default.
        error_!("{} catcher failed. Using Rocket default 500.", status.code);
        crate::catcher::default_handler(Status::InternalServerError, req)
    }

    pub(crate) async fn default_tcp_http_server<C>(mut self, ready: C) -> Result<(), Error>
        where C: for<'a> Fn(&'a Self) -> BoxFuture<'a, ()>
    {
        use std::net::ToSocketAddrs;

        // Determine the address we're going to serve on.
        let addr = format!("{}:{}", self.config.address, self.config.port);
        let mut addr = addr.to_socket_addrs()
            .map(|mut addrs| addrs.next().expect(">= 1 socket addr"))
            .map_err(|e| Error::new(ErrorKind::Io(e)))?;

        #[cfg(feature = "tls")]
        if self.config.tls_enabled() {
            if let Some(ref config) = self.config.tls {
                use crate::http::tls::TlsListener;

                let conf = config.to_native_config().map_err(ErrorKind::Io)?;
                let l = TlsListener::bind(addr, conf).await.map_err(ErrorKind::Bind)?;
                addr = l.local_addr().unwrap_or(addr);
                self.config.address = addr.ip();
                self.config.port = addr.port();
                ready(&mut self).await;
                return self.http_server(l).await;
            }
        }

        let l = bind_tcp(addr).await.map_err(ErrorKind::Bind)?;
        addr = l.local_addr().unwrap_or(addr);
        self.config.address = addr.ip();
        self.config.port = addr.port();
        ready(&mut self).await;
        self.http_server(l).await
    }

    // TODO.async: Solidify the Listener APIs and make this function public
    pub(crate) async fn http_server<L>(self, listener: L) -> Result<(), Error>
        where L: Listener + Send, <L as Listener>::Connection: Send + Unpin + 'static
    {
        // Emit a warning if we're not running inside of Rocket's async runtime.
        if self.config.profile == Config::DEBUG_PROFILE {
            tokio::task::spawn_blocking(|| {
                let this  = std::thread::current();
                if !this.name().map_or(false, |s| s.starts_with("rocket-worker")) {
                    warn!("Rocket is executing inside of a custom runtime.");
                    info_!("Rocket's runtime is enabled via `#[rocket::main]` or `#[launch]`.");
                    info_!("Forced shutdown is disabled. Runtime settings may be suboptimal.");
                }
            });
        }

        // Determine keep-alives.
        let http1_keepalive = self.config.keep_alive != 0;
        let http2_keep_alive = match self.config.keep_alive {
            0 => None,
            n => Some(Duration::from_secs(n as u64))
        };

        // Set up cancellable I/O from the given listener. Shutdown occurs when
        // `Shutdown` (`TripWire`) resolves. This can occur directly through a
        // notification or indirectly through an external signal which, when
        // received, results in triggering the notify.
        let shutdown = self.shutdown();
        let sig_stream = self.config.shutdown.signal_stream();
        let force_shutdown = self.config.shutdown.force;
        let grace = self.config.shutdown.grace as u64;
        let mercy = self.config.shutdown.mercy as u64;

        // Start a task that listens for external signals and notifies shutdown.
        if let Some(mut stream) = sig_stream {
            let shutdown = shutdown.clone();
            tokio::spawn(async move {
                while let Some(sig) = stream.next().await {
                    if shutdown.0.tripped() {
                        warn!("Received {}. Shutdown already in progress.", sig);
                    } else {
                        warn!("Received {}. Requesting shutdown.", sig);
                    }

                    shutdown.0.trip();
                }
            });
        }

        // Create the Hyper `Service`.
        let rocket = Arc::new(self);
        let service_fn = move |conn: &CancellableIo<_, L::Connection>| {
            let rocket = rocket.clone();
            let connection = ConnectionMeta {
                remote: conn.peer_address(),
                client_certificates: conn.peer_certificates().map(Arc::new),
            };

            async move {
                Ok::<_, std::convert::Infallible>(hyper::service::service_fn(move |req| {
                    hyper_service_fn(rocket.clone(), connection.clone(), req)
                }))
            }
        };

        // NOTE: `hyper` uses `tokio::spawn()` as the default executor.
        let listener = CancellableListener::new(shutdown.clone(), listener, grace, mercy);
        let server = hyper::Server::builder(Incoming::new(listener))
            .http1_keepalive(http1_keepalive)
            .http1_preserve_header_case(true)
            .http2_keep_alive_interval(http2_keep_alive)
            .serve(hyper::service::make_service_fn(service_fn))
            .with_graceful_shutdown(shutdown.clone())
            .map_err(|e| Error::new(ErrorKind::Runtime(Box::new(e))));

        // Wait for a shutdown notification or for the server to somehow fail.
        tokio::pin!(server);
        match future::select(shutdown, server).await {
            future::Either::Left((_, server)) => {
                // If a task has some runaway I/O, like an infinite loop, the
                // runtime will block indefinitely when it is dropped. To
                // subvert, we start a ticking process-exit time bomb here.
                if force_shutdown {
                    // Only a worker thread will have the specified thread name.
                    tokio::task::spawn_blocking(move || {
                        // We only hit our `exit()` if the process doesn't
                        // otherwise exit since this `spawn()` won't block.
                        let this = std::thread::current();
                        std::thread::spawn(move || {
                            std::thread::sleep(Duration::from_secs(grace + mercy));
                            std::thread::sleep(Duration::from_millis(500));
                            if this.name().map_or(false, |s| s.starts_with("rocket-worker")) {
                                error!("Server failed to shutdown cooperatively. Terminating.");
                                std::process::exit(1);
                            } else {
                                warn!("Server failed to shutdown cooperatively.");
                                warn_!("Server is executing inside of a custom runtime.");
                                info_!("Rocket's runtime is `#[rocket::main]` or `#[launch]`.");
                                warn_!("Refusing to terminate runaway custom runtime.");
                            }
                        });
                    });
                }

                info!("Received shutdown request. Waiting for pending I/O...");
                server.await
            }
            future::Either::Right((result, _)) => result,
        }
    }
}
