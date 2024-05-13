//! # wasi-http-client
//!
//! `wasi_http_client` is an HTTP client library for [WASI Preview 2](https://github.com/WebAssembly/WASI/tree/main/preview2),
//! making it easier to send http(s) requests in WASI components.
//!
//! ```
//! # use std::time::Duration;
//! # use wasi_http_client::Client;
//! # fn run() {
//! let resp = Client::new()
//!     .post("https://httpbin.org/post")
//!     .body("hello".as_bytes())
//!     .connect_timeout(Duration::from_secs(5))
//!     .send()
//!     .unwrap();
//!
//! println!("status code: {}", resp.status());
//! # }
//! ```

mod client;
mod request;
mod response;

pub use self::client::Client;
pub use self::request::RequestBuilder;
pub use self::response::Response;
pub use wasi::http::types::Method;
