//! Server and application configuration.
//!
//! See the [configuration guide] for full details.
//!
//! [configuration guide]: https://rocket.rs/v0.5-rc/guide/configuration/
//!
//! ## Extracting Configuration Parameters
//!
//! Rocket exposes the active [`Figment`] via [`Rocket::figment()`]. Any value
//! that implements [`Deserialize`] can be extracted from the figment:
//!
//! ```rust
//! use rocket::fairing::AdHoc;
//!
//! #[derive(serde::Deserialize)]
//! struct AppConfig {
//!     id: Option<usize>,
//!     port: u16,
//! }
//!
//! #[rocket::launch]
//! fn rocket() -> _ {
//!     rocket::build().attach(AdHoc::config::<AppConfig>())
//! }
//! ```
//!
//! [`Figment`]: figment::Figment
//! [`Rocket::figment()`]: crate::Rocket::figment()
//! [`Rocket::figment()`]: crate::Rocket::figment()
//! [`Deserialize`]: serde::Deserialize
//!
//! ## Workers
//!
//! The `workers` parameter sets the number of threads used for parallel task
//! execution; there is no limit to the number of concurrent tasks. Due to a
//! limitation in upstream async executers, unlike other values, the `workers`
//! configuration value cannot be reconfigured or be configured from sources
//! other than those provided by [`Config::figment()`]. In other words, only the
//! values set by the `ROCKET_WORKERS` environment variable or in the `workers`
//! property of `Rocket.toml` will be considered - all other `workers` values
//! are ignored.
//!
//! ## Custom Providers
//!
//! A custom provider can be set via [`rocket::custom()`], which replaces calls to
//! [`rocket::build()`]. The configured provider can be built on top of
//! [`Config::figment()`], [`Config::default()`], both, or neither. The
//! [Figment](figment) documentation has full details on instantiating existing
//! providers like [`Toml`]() and [`Env`] as well as creating custom providers for
//! more complex cases.
//!
//! Configuration values can be overridden at runtime by merging figment's tuple
//! providers with Rocket's default provider:
//!
//! ```rust
//! # #[macro_use] extern crate rocket;
//! use rocket::data::{Limits, ToByteUnit};
//!
//! #[launch]
//! fn rocket() -> _ {
//!     let figment = rocket::Config::figment()
//!         .merge(("port", 1111))
//!         .merge(("limits", Limits::new().limit("json", 2.mebibytes())));
//!
//!     rocket::custom(figment).mount("/", routes![/* .. */])
//! }
//! ```
//!
//! An application that wants to use Rocket's defaults for [`Config`], but not
//! its configuration sources, while allowing the application to be configured
//! via an `App.toml` file that uses top-level keys as profiles (`.nested()`)
//! and `APP_` environment variables as global overrides (`.global()`), and
//! `APP_PROFILE` to configure the selected profile, can be structured as
//! follows:
//!
//! ```rust
//! # #[macro_use] extern crate rocket;
//! use serde::{Serialize, Deserialize};
//! use figment::{Figment, Profile, providers::{Format, Toml, Serialized, Env}};
//! use rocket::fairing::AdHoc;
//!
//! #[derive(Debug, Deserialize, Serialize)]
//! struct Config {
//!     app_value: usize,
//!     /* and so on.. */
//! }
//!
//! impl Default for Config {
//!     fn default() -> Config {
//!         Config { app_value: 3, }
//!     }
//! }
//!
//! #[launch]
//! fn rocket() -> _ {
//!     let figment = Figment::from(rocket::Config::default())
//!         .merge(Serialized::defaults(Config::default()))
//!         .merge(Toml::file("App.toml").nested())
//!         .merge(Env::prefixed("APP_").global())
//!         .select(Profile::from_env_or("APP_PROFILE", "default"));
//!
//!     rocket::custom(figment)
//!         .mount("/", routes![/* .. */])
//!         .attach(AdHoc::config::<Config>())
//! }
//! ```
//!
//! [`rocket::custom()`]: crate::custom()
//! [`rocket::build()`]: crate::build()
//! [`Toml`]: figment::providers::Toml
//! [`Env`]: figment::providers::Env

#[macro_use]
mod ident;
mod config;
mod shutdown;

#[cfg(feature = "tls")]
mod tls;

#[cfg(feature = "secrets")]
mod secret_key;

#[doc(hidden)]
pub use config::pretty_print_error;
pub use config::Config;
pub use crate::log::LogLevel;
pub use shutdown::Shutdown;
pub use ident::Ident;

#[cfg(feature = "tls")]
pub use tls::{TlsConfig, CipherSuite};

#[cfg(feature = "mtls")]
pub use tls::MutualTls;

#[cfg(feature = "secrets")]
pub use secret_key::SecretKey;

#[cfg(unix)]
pub use shutdown::Sig;

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;
    use figment::{Figment, Profile};
    use pretty_assertions::assert_eq;

    use crate::log::LogLevel;
    use crate::data::{Limits, ToByteUnit};
    use crate::config::Config;

    #[test]
    fn test_figment_is_default() {
        figment::Jail::expect_with(|_| {
            let mut default: Config = Config::figment().extract().unwrap();
            default.profile = Config::default().profile;
            assert_eq!(default, Config::default());
            Ok(())
        });
    }

    #[test]
    fn test_default_round_trip() {
        figment::Jail::expect_with(|_| {
            let original = Config::figment();
            let roundtrip = Figment::from(Config::from(&original));
            for figment in &[original, roundtrip] {
                let config = Config::from(figment);
                assert_eq!(config, Config::default());
            }

            Ok(())
        });
    }

    #[test]
    fn test_profile_env() {
        figment::Jail::expect_with(|jail| {
            jail.set_env("ROCKET_PROFILE", "debug");
            let figment = Config::figment();
            assert_eq!(figment.profile(), "debug");

            jail.set_env("ROCKET_PROFILE", "release");
            let figment = Config::figment();
            assert_eq!(figment.profile(), "release");

            jail.set_env("ROCKET_PROFILE", "random");
            let figment = Config::figment();
            assert_eq!(figment.profile(), "random");

            Ok(())
        });
    }

    #[test]
    fn test_toml_file() {
        figment::Jail::expect_with(|jail| {
            jail.create_file("Rocket.toml", r#"
                [default]
                address = "1.2.3.4"
                ident = "Something Cool"
                port = 1234
                workers = 20
                keep_alive = 10
                log_level = "off"
                cli_colors = 0
            "#)?;

            let config = Config::from(Config::figment());
            assert_eq!(config, Config {
                address: Ipv4Addr::new(1, 2, 3, 4).into(),
                port: 1234,
                workers: 20,
                ident: ident!("Something Cool"),
                keep_alive: 10,
                log_level: LogLevel::Off,
                cli_colors: false,
                ..Config::default()
            });

            jail.create_file("Rocket.toml", r#"
                [global]
                address = "1.2.3.4"
                ident = "Something Else Cool"
                port = 1234
                workers = 20
                keep_alive = 10
                log_level = "off"
                cli_colors = 0
            "#)?;

            let config = Config::from(Config::figment());
            assert_eq!(config, Config {
                address: Ipv4Addr::new(1, 2, 3, 4).into(),
                port: 1234,
                workers: 20,
                ident: ident!("Something Else Cool"),
                keep_alive: 10,
                log_level: LogLevel::Off,
                cli_colors: false,
                ..Config::default()
            });

            jail.set_env("ROCKET_CONFIG", "Other.toml");
            jail.create_file("Other.toml", r#"
                [default]
                address = "1.2.3.4"
                port = 1234
                workers = 20
                keep_alive = 10
                log_level = "off"
                cli_colors = 0
            "#)?;

            let config = Config::from(Config::figment());
            assert_eq!(config, Config {
                address: Ipv4Addr::new(1, 2, 3, 4).into(),
                port: 1234,
                workers: 20,
                keep_alive: 10,
                log_level: LogLevel::Off,
                cli_colors: false,
                ..Config::default()
            });

            Ok(())
        });
    }

    #[test]
    #[cfg(feature = "tls")]
    fn test_tls_config_from_file() {
        use crate::config::{TlsConfig, CipherSuite, Ident, Shutdown};

        figment::Jail::expect_with(|jail| {
            jail.create_file("Rocket.toml", r#"
                [global]
                shutdown.ctrlc = 0
                ident = false

                [global.tls]
                certs = "/ssl/cert.pem"
                key = "/ssl/key.pem"

                [global.limits]
                forms = "1mib"
                json = "10mib"
                stream = "50kib"
            "#)?;

            let config = Config::from(Config::figment());
            assert_eq!(config, Config {
                shutdown: Shutdown { ctrlc: false, ..Default::default() },
                ident: Ident::none(),
                tls: Some(TlsConfig::from_paths("/ssl/cert.pem", "/ssl/key.pem")),
                limits: Limits::default()
                    .limit("forms", 1.mebibytes())
                    .limit("json", 10.mebibytes())
                    .limit("stream", 50.kibibytes()),
                ..Config::default()
            });

            jail.create_file("Rocket.toml", r#"
                [global.tls]
                certs = "cert.pem"
                key = "key.pem"
            "#)?;

            let config = Config::from(Config::figment());
            assert_eq!(config, Config {
                tls: Some(TlsConfig::from_paths(
                    jail.directory().join("cert.pem"),
                    jail.directory().join("key.pem")
                )),
                ..Config::default()
            });

            jail.create_file("Rocket.toml", r#"
                [global.tls]
                certs = "cert.pem"
                key = "key.pem"
                prefer_server_cipher_order = true
                ciphers = [
                    "TLS_CHACHA20_POLY1305_SHA256",
                    "TLS_AES_256_GCM_SHA384",
                    "TLS_AES_128_GCM_SHA256",
                    "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256",
                    "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
                    "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
                ]
            "#)?;

            let config = Config::from(Config::figment());
            let cert_path = jail.directory().join("cert.pem");
            let key_path = jail.directory().join("key.pem");
            assert_eq!(config, Config {
                tls: Some(TlsConfig::from_paths(cert_path, key_path)
                         .with_preferred_server_cipher_order(true)
                         .with_ciphers([
                             CipherSuite::TLS_CHACHA20_POLY1305_SHA256,
                             CipherSuite::TLS_AES_256_GCM_SHA384,
                             CipherSuite::TLS_AES_128_GCM_SHA256,
                             CipherSuite::TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256,
                             CipherSuite::TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384,
                             CipherSuite::TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256,
                         ])),
                ..Config::default()
            });

            jail.create_file("Rocket.toml", r#"
                [global]
                shutdown.ctrlc = 0
                ident = false

                [global.tls]
                certs = "/ssl/cert.pem"
                key = "/ssl/key.pem"

                [global.limits]
                forms = "1mib"
                json = "10mib"
                stream = "50kib"
            "#)?;

            let config = Config::from(Config::figment());
            assert_eq!(config, Config {
                shutdown: Shutdown { ctrlc: false, ..Default::default() },
                ident: Ident::none(),
                tls: Some(TlsConfig::from_paths("/ssl/cert.pem", "/ssl/key.pem")),
                limits: Limits::default()
                    .limit("forms", 1.mebibytes())
                    .limit("json", 10.mebibytes())
                    .limit("stream", 50.kibibytes()),
                ..Config::default()
            });

            jail.create_file("Rocket.toml", r#"
                [global.tls]
                certs = "cert.pem"
                key = "key.pem"
            "#)?;

            let config = Config::from(Config::figment());
            assert_eq!(config, Config {
                tls: Some(TlsConfig::from_paths(
                    jail.directory().join("cert.pem"),
                    jail.directory().join("key.pem")
                )),
                ..Config::default()
            });

            jail.create_file("Rocket.toml", r#"
                [global.tls]
                certs = "cert.pem"
                key = "key.pem"
                prefer_server_cipher_order = true
                ciphers = [
                    "TLS_CHACHA20_POLY1305_SHA256",
                    "TLS_AES_256_GCM_SHA384",
                    "TLS_AES_128_GCM_SHA256",
                    "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256",
                    "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
                    "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
                ]
            "#)?;

            let config = Config::from(Config::figment());
            let cert_path = jail.directory().join("cert.pem");
            let key_path = jail.directory().join("key.pem");
            assert_eq!(config, Config {
                tls: Some(TlsConfig::from_paths(cert_path, key_path)
                         .with_preferred_server_cipher_order(true)
                         .with_ciphers([
                             CipherSuite::TLS_CHACHA20_POLY1305_SHA256,
                             CipherSuite::TLS_AES_256_GCM_SHA384,
                             CipherSuite::TLS_AES_128_GCM_SHA256,
                             CipherSuite::TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256,
                             CipherSuite::TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384,
                             CipherSuite::TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256,
                         ])),
                ..Config::default()
            });

            Ok(())
        });
    }

    #[test]
    #[cfg(feature = "mtls")]
    fn test_mtls_config() {
        use std::path::Path;

        figment::Jail::expect_with(|jail| {
            jail.create_file("Rocket.toml", r#"
                [default.tls]
                certs = "/ssl/cert.pem"
                key = "/ssl/key.pem"
            "#)?;

            let config = Config::from(Config::figment());
            assert!(config.tls.is_some());
            assert!(config.tls.as_ref().unwrap().mutual.is_none());
            assert!(config.tls_enabled());
            assert!(!config.mtls_enabled());

            jail.create_file("Rocket.toml", r#"
                [default.tls]
                certs = "/ssl/cert.pem"
                key = "/ssl/key.pem"
                mutual = { ca_certs = "/ssl/ca.pem" }
            "#)?;

            let config = Config::from(Config::figment());
            assert!(config.tls_enabled());
            assert!(config.mtls_enabled());

            let mtls = config.tls.as_ref().unwrap().mutual.as_ref().unwrap();
            assert_eq!(mtls.ca_certs().unwrap_left(), Path::new("/ssl/ca.pem"));
            assert!(!mtls.mandatory);

            jail.create_file("Rocket.toml", r#"
                [default.tls]
                certs = "/ssl/cert.pem"
                key = "/ssl/key.pem"

                [default.tls.mutual]
                ca_certs = "/ssl/ca.pem"
                mandatory = true
            "#)?;

            let config = Config::from(Config::figment());
            let mtls = config.tls.as_ref().unwrap().mutual.as_ref().unwrap();
            assert_eq!(mtls.ca_certs().unwrap_left(), Path::new("/ssl/ca.pem"));
            assert!(mtls.mandatory);

            jail.create_file("Rocket.toml", r#"
                [default.tls]
                certs = "/ssl/cert.pem"
                key = "/ssl/key.pem"
                mutual = { ca_certs = "relative/ca.pem" }
            "#)?;

            let config = Config::from(Config::figment());
            let mtls = config.tls.as_ref().unwrap().mutual().unwrap();
            assert_eq!(mtls.ca_certs().unwrap_left(),
                jail.directory().join("relative/ca.pem"));

            Ok(())
        });
    }

    #[test]
    fn test_profiles_merge() {
        figment::Jail::expect_with(|jail| {
            jail.create_file("Rocket.toml", r#"
                [default.limits]
                stream = "50kb"

                [global]
                limits = { forms = "2kb" }

                [debug.limits]
                file = "100kb"
            "#)?;

            jail.set_env("ROCKET_PROFILE", "unknown");
            let config = Config::from(Config::figment());
            assert_eq!(config, Config {
                profile: Profile::const_new("unknown"),
                limits: Limits::default()
                    .limit("stream", 50.kilobytes())
                    .limit("forms", 2.kilobytes()),
                ..Config::default()
            });

            jail.set_env("ROCKET_PROFILE", "debug");
            let config = Config::from(Config::figment());
            assert_eq!(config, Config {
                profile: Profile::const_new("debug"),
                limits: Limits::default()
                    .limit("stream", 50.kilobytes())
                    .limit("forms", 2.kilobytes())
                    .limit("file", 100.kilobytes()),
                ..Config::default()
            });

            Ok(())
        });
    }

    #[test]
    #[cfg(feature = "tls")]
    fn test_env_vars_merge() {
        use crate::config::{TlsConfig, Ident};

        figment::Jail::expect_with(|jail| {
            jail.set_env("ROCKET_PORT", 9999);
            let config = Config::from(Config::figment());
            assert_eq!(config, Config {
                port: 9999,
                ..Config::default()
            });

            jail.set_env("ROCKET_TLS", r#"{certs="certs.pem"}"#);
            let first_figment = Config::figment();
            jail.set_env("ROCKET_TLS", r#"{key="key.pem"}"#);
            let prev_figment = Config::figment().join(&first_figment);
            let config = Config::from(&prev_figment);
            assert_eq!(config, Config {
                port: 9999,
                tls: Some(TlsConfig::from_paths("certs.pem", "key.pem")),
                ..Config::default()
            });

            jail.set_env("ROCKET_TLS", r#"{certs="new.pem"}"#);
            let config = Config::from(Config::figment().join(&prev_figment));
            assert_eq!(config, Config {
                port: 9999,
                tls: Some(TlsConfig::from_paths("new.pem", "key.pem")),
                ..Config::default()
            });

            jail.set_env("ROCKET_LIMITS", r#"{stream=100kiB}"#);
            let config = Config::from(Config::figment().join(&prev_figment));
            assert_eq!(config, Config {
                port: 9999,
                tls: Some(TlsConfig::from_paths("new.pem", "key.pem")),
                limits: Limits::default().limit("stream", 100.kibibytes()),
                ..Config::default()
            });

            jail.set_env("ROCKET_IDENT", false);
            let config = Config::from(Config::figment().join(&prev_figment));
            assert_eq!(config, Config {
                port: 9999,
                tls: Some(TlsConfig::from_paths("new.pem", "key.pem")),
                limits: Limits::default().limit("stream", 100.kibibytes()),
                ident: Ident::none(),
                ..Config::default()
            });

            Ok(())
        });
    }

    #[test]
    fn test_precedence() {
        figment::Jail::expect_with(|jail| {
            jail.create_file("Rocket.toml", r#"
                [global.limits]
                forms = "1mib"
                stream = "50kb"
                file = "100kb"
            "#)?;

            let config = Config::from(Config::figment());
            assert_eq!(config, Config {
                limits: Limits::default()
                    .limit("forms", 1.mebibytes())
                    .limit("stream", 50.kilobytes())
                    .limit("file", 100.kilobytes()),
                ..Config::default()
            });

            jail.set_env("ROCKET_LIMITS", r#"{stream=3MiB,capture=2MiB}"#);
            let config = Config::from(Config::figment());
            assert_eq!(config, Config {
                limits: Limits::default()
                    .limit("file", 100.kilobytes())
                    .limit("forms", 1.mebibytes())
                    .limit("stream", 3.mebibytes())
                    .limit("capture", 2.mebibytes()),
                ..Config::default()
            });

            jail.set_env("ROCKET_PROFILE", "foo");
            let val: Result<String, _> = Config::figment().extract_inner("profile");
            assert!(val.is_err());

            Ok(())
        });
    }

    #[test]
    #[cfg(feature = "secrets")]
    #[should_panic]
    fn test_err_on_non_debug_and_no_secret_key() {
        figment::Jail::expect_with(|jail| {
            jail.set_env("ROCKET_PROFILE", "release");
            let rocket = crate::custom(Config::figment());
            let _result = crate::local::blocking::Client::untracked(rocket);
            Ok(())
        });
    }

    #[test]
    #[cfg(feature = "secrets")]
    #[should_panic]
    fn test_err_on_non_debug2_and_no_secret_key() {
        figment::Jail::expect_with(|jail| {
            jail.set_env("ROCKET_PROFILE", "boop");
            let rocket = crate::custom(Config::figment());
            let _result = crate::local::blocking::Client::tracked(rocket);
            Ok(())
        });
    }

    #[test]
    fn test_no_err_on_debug_and_no_secret_key() {
        figment::Jail::expect_with(|jail| {
            jail.set_env("ROCKET_PROFILE", "debug");
            let figment = Config::figment();
            assert!(crate::local::blocking::Client::untracked(crate::custom(&figment)).is_ok());
            crate::async_main(async {
                let rocket = crate::custom(&figment);
                assert!(crate::local::asynchronous::Client::tracked(rocket).await.is_ok());
            });

            Ok(())
        });
    }

    #[test]
    fn test_no_err_on_release_and_custom_secret_key() {
        figment::Jail::expect_with(|jail| {
            jail.set_env("ROCKET_PROFILE", "release");
            let key = "hPRYyVRiMyxpw5sBB1XeCMN1kFsDCqKvBi2QJxBVHQk=";
            let figment = Config::figment().merge(("secret_key", key));

            assert!(crate::local::blocking::Client::tracked(crate::custom(&figment)).is_ok());
            crate::async_main(async {
                let rocket = crate::custom(&figment);
                assert!(crate::local::asynchronous::Client::untracked(rocket).await.is_ok());
            });

            Ok(())
        });
    }
}
