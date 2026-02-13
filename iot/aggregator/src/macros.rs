/// Load a variable from the environement, with an optional default value
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate aggregator;
/// # use std::sync::LazyLock;
/// // Panics if the variable is not set.
/// static FOOBAR: LazyLock<String> = LazyLock::new(|| load_var!("FOOBAR"));
/// // Use a default value if the variable is not set
/// static BARFOO: LazyLock<String> = LazyLock::new(|| load_var!("BARFOO", "bar"));
/// ```
#[macro_export]
macro_rules! load_var {
    ($var:expr) => {
        ::std::env::var($var)
            .map(|s| s.leak() as &str)
            .expect(concat!("`", $var, "` must be set"))
    };
    ($var:expr, $default:expr) => {
        ::std::env::var($var)
            .map(|s| s.leak() as &str)
            .unwrap_or($default.into())
    };
    ($var:expr => int) => {
        ::std::env::var($var)
            .map(|s| s.leak() as &str)
            .expect(concat!("`", $var, "` must be set"))
            .parse()
            .expect(concat!("`", $var, "` value must be an integer"))
    };
    ($var:expr => uint) => {
        ::std::env::var($var)
            .map(|s| s.leak() as &str)
            .expect(concat!("`", $var, "` must be set"))
            .parse()
            .expect(concat!("`", $var, "` value must be a non-negative integer"))
    };
}

/// Automatically bubble up an error as a `500`.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate web_test;
/// # use actix_web::{HttpResponse, Responder};
/// #[rustfmt::skip]
/// fn faillible() -> Result<&str, ()> { todo!() }
///
/// async fn main() -> impl Responder {
///     // If `faillible` were to return an `Err`,
///     // this function would return `HttpResponse::InternalServerError`
///     let foo = or_fail!(faillible());
///
///     HttpResponse::Ok().body(foo)
/// }
/// ```
#[macro_export]
macro_rules! or_fail {
    ($e:expr) => {
        match { $e } {
            Ok(__val) => __val,
            Err(__e) => {
                return actix_web::HttpResponse::InternalServerError().body(__e.to_string());
            },
        }
    };
}
