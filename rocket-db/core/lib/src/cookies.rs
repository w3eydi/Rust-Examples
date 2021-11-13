use std::fmt;

use parking_lot::Mutex;

use crate::Config;
use crate::http::private::cookie;

#[doc(inline)]
pub use self::cookie::{Cookie, SameSite, Iter};

/// Collection of one or more HTTP cookies.
///
/// `CookieJar` allows for retrieval of cookies from an incoming request. It
/// also tracks modifications (additions and removals) and marks them as
/// pending.
///
/// # Pending
///
/// Changes to a `CookieJar` are _not_ visible via the normal [`get()`] and
/// [`get_private()`] methods. This is typically the desired effect as a
/// `CookieJar` always reflects the cookies in an incoming request. In cases
/// where this is not desired, the [`get_pending()`] and
/// [`get_private_pending()`] methods are available, which always return the
/// latest changes.
///
/// ```rust
/// # #[macro_use] extern crate rocket;
/// use rocket::http::{CookieJar, Cookie};
///
/// #[get("/message")]
/// fn message(jar: &CookieJar<'_>) {
///     jar.add(Cookie::new("message", "hello!"));
///     jar.add(Cookie::new("other", "bye!"));
///
///     // `get()` does not reflect changes.
///     assert!(jar.get("other").is_none());
///     # assert_eq!(jar.get("message").map(|c| c.value()), Some("hi"));
///
///     // `get_pending()` does.
///     let other_pending = jar.get_pending("other");
///     let message_pending = jar.get_pending("message");
///     assert_eq!(other_pending.as_ref().map(|c| c.value()), Some("bye!"));
///     assert_eq!(message_pending.as_ref().map(|c| c.value()), Some("hello!"));
///     # jar.remove(Cookie::named("message"));
///     # assert_eq!(jar.get("message").map(|c| c.value()), Some("hi"));
///     # assert!(jar.get_pending("message").is_none());
/// }
/// # fn main() {
/// #     use rocket::local::blocking::Client;
/// #     let client = Client::debug_with(routes![message]).unwrap();
/// #     let response = client.get("/message")
/// #         .cookie(Cookie::new("message", "hi"))
/// #         .dispatch();
/// #
/// #     assert!(response.status().class().is_success());
/// # }
/// ```
///
/// # Usage
///
/// A type of `&CookieJar` can be retrieved via its `FromRequest` implementation
/// as a request guard or via the [`Request::cookies()`] method. Individual
/// cookies can be retrieved via the [`get()`] and [`get_private()`] methods.
/// Pending changes can be observed via the [`get_pending()`] and
/// [`get_private_pending()`] methods. Cookies can be added or removed via the
/// [`add()`], [`add_private()`], [`remove()`], and [`remove_private()`]
/// methods.
///
/// [`Request::cookies()`]: crate::Request::cookies()
/// [`get()`]: #method.get
/// [`get_private()`]: #method.get_private
/// [`get_pending()`]: #method.get_pending
/// [`get_private_pending()`]: #method.get_private_pending
/// [`add()`]: #method.add
/// [`add_private()`]: #method.add_private
/// [`remove()`]: #method.remove
/// [`remove_private()`]: #method.remove_private
///
/// ## Examples
///
/// The following example shows `&CookieJar` being used as a request guard in a
/// handler to retrieve the value of a "message" cookie.
///
/// ```rust
/// # #[macro_use] extern crate rocket;
/// use rocket::http::CookieJar;
///
/// #[get("/message")]
/// fn message<'a>(jar: &'a CookieJar<'_>) -> Option<&'a str> {
///     jar.get("message").map(|cookie| cookie.value())
/// }
/// # fn main() {  }
/// ```
///
/// The following snippet shows `&CookieJar` being retrieved from a `Request` in
/// a custom request guard implementation for `User`. A [private cookie]
/// containing a user's ID is retrieved. If the cookie exists and the ID parses
/// as an integer, a `User` structure is validated. Otherwise, the guard
/// forwards.
///
/// [private cookie]: #method.add_private
///
/// ```rust
/// # #[macro_use] extern crate rocket;
/// # #[cfg(feature = "secrets")] {
/// use rocket::http::Status;
/// use rocket::outcome::IntoOutcome;
/// use rocket::request::{self, Request, FromRequest};
///
/// // In practice, we'd probably fetch the user from the database.
/// struct User(usize);
///
/// #[rocket::async_trait]
/// impl<'r> FromRequest<'r> for User {
///     type Error = std::convert::Infallible;
///
///     async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
///         request.cookies()
///             .get_private("user_id")
///             .and_then(|c| c.value().parse().ok())
///             .map(|id| User(id))
///             .or_forward(())
///     }
/// }
/// # }
/// # fn main() { }
/// ```
///
/// # Private Cookies
///
/// _Private_ cookies are just like regular cookies except that they are
/// encrypted using authenticated encryption, a form of encryption which
/// simultaneously provides confidentiality, integrity, and authenticity. This
/// means that private cookies cannot be inspected, tampered with, or
/// manufactured by clients. If you prefer, you can think of private cookies as
/// being signed and encrypted.
///
/// Private cookies can be retrieved, added, and removed from a `CookieJar`
/// collection via the [`get_private()`], [`add_private()`], and
/// [`remove_private()`] methods.
///
/// ## Encryption Key
///
/// To encrypt private cookies, Rocket uses the 256-bit key specified in the
/// `secret_key` configuration parameter. If one is not specified, Rocket will
/// automatically generate a fresh key. Note, however, that a private cookie can
/// only be decrypted with the same key with which it was encrypted. As such, it
/// is important to set a `secret_key` configuration parameter when using
/// private cookies so that cookies decrypt properly after an application
/// restart. Rocket will emit a warning if an application is run in production
/// mode without a configured `secret_key`.
///
/// Generating a string suitable for use as a `secret_key` configuration value
/// is usually done through tools like `openssl`. Using `openssl`, for instance,
/// a 256-bit base64 key can be generated with the command `openssl rand -base64
/// 32`.
pub struct CookieJar<'a> {
    jar: cookie::CookieJar,
    ops: Mutex<Vec<Op>>,
    config: &'a Config,
}

impl<'a> Clone for CookieJar<'a> {
    fn clone(&self) -> Self {
        CookieJar {
            jar: self.jar.clone(),
            ops: Mutex::new(self.ops.lock().clone()),
            config: self.config,
        }
    }
}

#[derive(Clone)]
enum Op {
    Add(Cookie<'static>, bool),
    Remove(Cookie<'static>, bool),
}

impl Op {
    fn cookie(&self) -> &Cookie<'static> {
        match self {
            Op::Add(c, _) | Op::Remove(c, _) => c
        }
    }
}

impl<'a> CookieJar<'a> {
    #[inline(always)]
    pub(crate) fn new(config: &'a Config) -> Self {
        CookieJar::from(cookie::CookieJar::new(), config)
    }

    pub(crate) fn from(jar: cookie::CookieJar, config: &'a Config) -> Self {
        CookieJar { jar, config, ops: Mutex::new(Vec::new()) }
    }

    /// Returns a reference to the _original_ `Cookie` inside this container
    /// with the name `name`. If no such cookie exists, returns `None`.
    ///
    /// **Note:** This method _does not_ obverse changes made via additions and
    /// removals to the cookie jar. To observe those changes, use
    /// [`CookieJar::get_pending()`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # #[macro_use] extern crate rocket;
    /// use rocket::http::{Cookie, CookieJar};
    ///
    /// #[get("/")]
    /// fn handler(jar: &CookieJar<'_>) {
    ///     let cookie = jar.get("name");
    /// }
    /// ```
    pub fn get(&self, name: &str) -> Option<&Cookie<'static>> {
        self.jar.get(name)
    }

    /// Retrives the _original_ `Cookie` inside this collection with the name
    /// `name` and authenticates and decrypts the cookie's value. If the cookie
    /// cannot be found, or the cookie fails to authenticate or decrypt, `None`
    /// is returned.
    ///
    /// **Note:** This method _does not_ obverse changes made via additions and
    /// removals to the cookie jar. To observe those changes, use
    /// [`CookieJar::get_private_pending()`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # #[macro_use] extern crate rocket;
    /// use rocket::http::{Cookie, CookieJar};
    ///
    /// #[get("/")]
    /// fn handler(jar: &CookieJar<'_>) {
    ///     let cookie = jar.get_private("name");
    /// }
    /// ```
    #[cfg(feature = "secrets")]
    #[cfg_attr(nightly, doc(cfg(feature = "secrets")))]
    pub fn get_private(&self, name: &str) -> Option<Cookie<'static>> {
        self.jar.private(&self.config.secret_key.key).get(name)
    }

    /// Returns a reference to the _original or pending_ `Cookie` inside this
    /// container with the name `name`. If no such cookie exists, returns
    /// `None`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #[macro_use] extern crate rocket;
    /// use rocket::http::{Cookie, CookieJar};
    ///
    /// #[get("/")]
    /// fn handler(jar: &CookieJar<'_>) {
    ///     let pending_cookie = jar.get_pending("name");
    /// }
    /// ```
    pub fn get_pending(&self, name: &str) -> Option<Cookie<'static>> {
        let ops = self.ops.lock();
        for op in ops.iter().rev().filter(|op| op.cookie().name() == name) {
            match op {
                Op::Add(c, _) => return Some(c.clone()),
                Op::Remove(_, _) => return None,
            }
        }

        drop(ops);
        self.get(name).cloned()
    }

    /// Retrives the _original or pending_ `Cookie` inside this collection with
    /// the name `name` and authenticates and decrypts the cookie's value. If
    /// the cookie cannot be found, or the cookie fails to authenticate or
    /// decrypt, `None` is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #[macro_use] extern crate rocket;
    /// use rocket::http::{Cookie, CookieJar};
    ///
    /// #[get("/")]
    /// fn handler(jar: &CookieJar<'_>) {
    ///     let pending_cookie = jar.get_private_pending("name");
    /// }
    /// ```
    #[cfg(feature = "secrets")]
    #[cfg_attr(nightly, doc(cfg(feature = "secrets")))]
    pub fn get_private_pending(&self, name: &str) -> Option<Cookie<'static>> {
        let cookie = self.get_pending(name)?;
        self.jar.private(&self.config.secret_key.key).decrypt(cookie)
    }

    /// Adds `cookie` to this collection.
    ///
    /// Unless a value is set for the given property, the following defaults are
    /// set on `cookie` before being added to `self`:
    ///
    ///    * `path`: `"/"`
    ///    * `SameSite`: `Strict`
    ///
    /// # Example
    ///
    /// ```rust
    /// # #[macro_use] extern crate rocket;
    /// use rocket::http::{Cookie, SameSite, CookieJar};
    ///
    /// #[get("/")]
    /// fn handler(jar: &CookieJar<'_>) {
    ///     jar.add(Cookie::new("first", "value"));
    ///
    ///     let cookie = Cookie::build("other", "value_two")
    ///         .path("/")
    ///         .secure(true)
    ///         .same_site(SameSite::Lax);
    ///
    ///     jar.add(cookie.finish());
    /// }
    /// ```
    pub fn add(&self, mut cookie: Cookie<'static>) {
        Self::set_defaults(&mut cookie);
        self.ops.lock().push(Op::Add(cookie, false));
    }

    /// Adds `cookie` to the collection. The cookie's value is encrypted with
    /// authenticated encryption assuring confidentiality, integrity, and
    /// authenticity. The cookie can later be retrieved using
    /// [`get_private`](#method.get_private) and removed using
    /// [`remove_private`](#method.remove_private).
    ///
    /// Unless a value is set for the given property, the following defaults are
    /// set on `cookie` before being added to `self`:
    ///
    ///    * `path`: `"/"`
    ///    * `SameSite`: `Strict`
    ///    * `HttpOnly`: `true`
    ///    * `Expires`: 1 week from now
    ///
    /// These defaults ensure maximum usability and security. For additional
    /// security, you may wish to set the `secure` flag.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #[macro_use] extern crate rocket;
    /// use rocket::http::{Cookie, CookieJar};
    ///
    /// #[get("/")]
    /// fn handler(jar: &CookieJar<'_>) {
    ///     jar.add_private(Cookie::new("name", "value"));
    /// }
    /// ```
    #[cfg(feature = "secrets")]
    #[cfg_attr(nightly, doc(cfg(feature = "secrets")))]
    pub fn add_private(&self, mut cookie: Cookie<'static>) {
        Self::set_private_defaults(&mut cookie);
        self.ops.lock().push(Op::Add(cookie, true));
    }

    /// Removes `cookie` from this collection and generates a "removal" cookies
    /// to send to the client on response. For correctness, `cookie` must
    /// contain the same `path` and `domain` as the cookie that was initially
    /// set. Failure to provide the initial `path` and `domain` will result in
    /// cookies that are not properly removed. For convenience, if a path is not
    /// set on `cookie`, the `"/"` path will automatically be set.
    ///
    /// A "removal" cookie is a cookie that has the same name as the original
    /// cookie but has an empty value, a max-age of 0, and an expiration date
    /// far in the past.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #[macro_use] extern crate rocket;
    /// use rocket::http::{Cookie, CookieJar};
    ///
    /// #[get("/")]
    /// fn handler(jar: &CookieJar<'_>) {
    ///     jar.remove(Cookie::named("name"));
    /// }
    /// ```
    pub fn remove(&self, mut cookie: Cookie<'static>) {
        if cookie.path().is_none() {
            cookie.set_path("/");
        }

        self.ops.lock().push(Op::Remove(cookie, false));
    }

    /// Removes the private `cookie` from the collection.
    ///
    /// For correct removal, the passed in `cookie` must contain the same `path`
    /// and `domain` as the cookie that was initially set. If a path is not set
    /// on `cookie`, the `"/"` path will automatically be set.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #[macro_use] extern crate rocket;
    /// use rocket::http::{Cookie, CookieJar};
    ///
    /// #[get("/")]
    /// fn handler(jar: &CookieJar<'_>) {
    ///     jar.remove_private(Cookie::named("name"));
    /// }
    /// ```
    #[cfg(feature = "secrets")]
    #[cfg_attr(nightly, doc(cfg(feature = "secrets")))]
    pub fn remove_private(&self, mut cookie: Cookie<'static>) {
        if cookie.path().is_none() {
            cookie.set_path("/");
        }

        self.ops.lock().push(Op::Remove(cookie, true));
    }

    /// Returns an iterator over all of the _original_ cookies present in this
    /// collection.
    ///
    /// **Note:** This method _does not_ obverse changes made via additions and
    /// removals to the cookie jar.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #[macro_use] extern crate rocket;
    /// use rocket::http::{Cookie, CookieJar};
    ///
    /// #[get("/")]
    /// fn handler(jar: &CookieJar<'_>) {
    ///     for c in jar.iter() {
    ///         println!("Name: {:?}, Value: {:?}", c.name(), c.value());
    ///     }
    /// }
    /// ```
    pub fn iter(&self) -> impl Iterator<Item=&Cookie<'static>> {
        self.jar.iter()
    }

    /// Removes all delta cookies.
    #[inline(always)]
    pub(crate) fn reset_delta(&self) {
        self.ops.lock().clear();
    }

    /// TODO: This could be faster by just returning the cookies directly via
    /// an ordered hash-set of sorts.
    pub(crate) fn take_delta_jar(&self) -> cookie::CookieJar {
        let ops = std::mem::take(&mut *self.ops.lock());
        let mut jar = cookie::CookieJar::new();

        for op in ops {
            match op {
                Op::Add(c, false) => jar.add(c),
                #[cfg(feature = "secrets")]
                Op::Add(c, true) => {
                    jar.private_mut(&self.config.secret_key.key).add(c);
                }
                Op::Remove(mut c, _) => {
                    if self.jar.get(c.name()).is_some() {
                        c.make_removal();
                        jar.add(c);
                    } else {
                        jar.remove(c);
                    }
                }
                #[allow(unreachable_patterns)]
                _ => unreachable!()
            }
        }

        jar
    }

    /// Adds an original `cookie` to this collection.
    #[inline(always)]
    pub(crate) fn add_original(&mut self, cookie: Cookie<'static>) {
        self.jar.add_original(cookie)
    }

    /// Adds an original, private `cookie` to the collection.
    #[cfg(feature = "secrets")]
    #[cfg_attr(nightly, doc(cfg(feature = "secrets")))]
    #[inline(always)]
    pub(crate) fn add_original_private(&mut self, cookie: Cookie<'static>) {
        self.jar.private_mut(&self.config.secret_key.key).add_original(cookie);
    }

    /// For each property mentioned below, this method checks if there is a
    /// provided value and if there is none, sets a default value. Default
    /// values are:
    ///
    ///    * `path`: `"/"`
    ///    * `SameSite`: `Strict`
    ///
    fn set_defaults(cookie: &mut Cookie<'static>) {
        if cookie.path().is_none() {
            cookie.set_path("/");
        }

        if cookie.same_site().is_none() {
            cookie.set_same_site(SameSite::Strict);
        }
    }

    /// For each property mentioned below, this method checks if there is a
    /// provided value and if there is none, sets a default value. Default
    /// values are:
    ///
    ///    * `path`: `"/"`
    ///    * `SameSite`: `Strict`
    ///    * `HttpOnly`: `true`
    ///    * `Expires`: 1 week from now
    ///
    #[cfg(feature = "secrets")]
    #[cfg_attr(nightly, doc(cfg(feature = "secrets")))]
    fn set_private_defaults(cookie: &mut Cookie<'static>) {
        if cookie.path().is_none() {
            cookie.set_path("/");
        }

        if cookie.same_site().is_none() {
            cookie.set_same_site(SameSite::Strict);
        }

        if cookie.http_only().is_none() {
            cookie.set_http_only(true);
        }

        if cookie.expires().is_none() {
            cookie.set_expires(time::OffsetDateTime::now_utc() + time::Duration::weeks(1));
        }
    }
}

impl fmt::Debug for CookieJar<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pending: Vec<_> = self.ops.lock()
            .iter()
            .map(|c| c.cookie())
            .cloned()
            .collect();

        f.debug_struct("CookieJar")
            .field("original", &self.jar)
            .field("pending", &pending)
            .finish()
    }

}
