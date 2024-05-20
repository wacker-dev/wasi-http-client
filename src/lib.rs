#![cfg_attr(docsrs, feature(doc_cfg))]

//! # wasi-http-client
//!
//! `wasi_http_client` is an HTTP client library for [WASI Preview 2](https://github.com/WebAssembly/WASI/tree/main/preview2),
//! making it easier to send http(s) requests in WASI components.
//!
//! ```
//! # use anyhow::Result;
//! # use std::time::Duration;
//! # use wasi_http_client::Client;
//! # fn run() -> Result<()> {
//! let resp = Client::new()
//!     .post("https://httpbin.org/post")
//!     .connect_timeout(Duration::from_secs(5))
//!     .send()?;
//!
//! println!("status code: {}", resp.status());
//! # Ok(())
//! # }
//! ```

mod client;
mod request;
mod response;

pub use self::{client::Client, request::RequestBuilder, response::Response};
pub use wasi::http::types::Method;
