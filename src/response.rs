use anyhow::{anyhow, Result};
#[cfg(feature = "json")]
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use wasi::http::types::{IncomingBody, IncomingResponse};
use wasi::io::streams::{InputStream, StreamError};

pub struct Response {
    status: u16,
    headers: HashMap<String, String>,
    // input-stream resource is a child: it must be dropped before the parent incoming-body is dropped
    input_stream: InputStream,
    _incoming_body: IncomingBody,
}

impl Response {
    pub(crate) fn new(incoming_response: IncomingResponse) -> Result<Self> {
        let status = incoming_response.status();

        let headers_handle = incoming_response.headers();
        let headers: HashMap<String, String> = headers_handle
            .entries()
            .into_iter()
            .map(|(key, value)| (key, String::from_utf8_lossy(&value).to_string()))
            .collect();
        drop(headers_handle);

        // The consume() method can only be called once
        let incoming_body = incoming_response.consume().unwrap();
        drop(incoming_response);

        // The stream() method can only be called once
        let input_stream = incoming_body.stream().unwrap();
        Ok(Self {
            status,
            headers,
            input_stream,
            _incoming_body: incoming_body,
        })
    }

    /// Get the status code of the response.
    pub fn status(&self) -> u16 {
        self.status
    }

    /// Get the headers of the response.
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Get a chunk of the response body.
    ///
    /// It will block until at least one byte can be read or the stream is closed.
    pub fn chunk(&self, len: u64) -> Result<Option<Vec<u8>>> {
        match self.input_stream.blocking_read(len) {
            Ok(c) => Ok(Some(c)),
            Err(StreamError::Closed) => Ok(None),
            Err(e) => Err(anyhow!("input_stream read failed: {e:?}"))?,
        }
    }

    /// Get the full response body.
    ///
    /// It will block until the stream is closed.
    pub fn body(self) -> Result<Vec<u8>> {
        let mut body = Vec::new();
        while let Some(mut chunk) = self.chunk(1024 * 1024)? {
            body.append(&mut chunk);
        }
        Ok(body)
    }

    /// Deserialize the response body as JSON.
    ///
    /// # Optional
    ///
    /// This requires the `json` feature enabled.
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use serde::Deserialize;
    /// # use wasi_http_client::Client;
    /// # fn run() -> Result<()> {
    /// #[derive(Deserialize)]
    /// struct Data {
    ///     origin: String,
    ///     url: String,
    /// }
    ///
    /// let resp = Client::new().get("https://httpbin.org/get").send()?;
    /// let json_data = resp.json::<Data>()?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    pub fn json<T: DeserializeOwned>(self) -> Result<T> {
        Ok(serde_json::from_slice(self.body()?.as_ref())?)
    }
}
