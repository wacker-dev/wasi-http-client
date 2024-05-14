use anyhow::{anyhow, Result};
#[cfg(feature = "json")]
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use wasi::http::types::{IncomingResponse, StatusCode};
use wasi::io::streams::StreamError;

pub struct Response {
    status: StatusCode,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Response {
    pub fn new(incoming_response: IncomingResponse) -> Result<Self> {
        let status = incoming_response.status();

        let mut headers: HashMap<String, String> = HashMap::new();
        let headers_handle = incoming_response.headers();
        for (key, value) in headers_handle.entries() {
            headers.insert(key, String::from_utf8(value)?);
        }
        drop(headers_handle);

        let incoming_body = incoming_response
            .consume()
            .map_err(|()| anyhow!("incoming response has no body stream"))?;
        drop(incoming_response);

        let input_stream = incoming_body.stream().unwrap();
        let mut body = vec![];
        loop {
            let mut body_chunk = match input_stream.read(1024 * 1024) {
                Ok(c) => c,
                Err(StreamError::Closed) => break,
                Err(e) => Err(anyhow!("input_stream read failed: {e:?}"))?,
            };

            if !body_chunk.is_empty() {
                body.append(&mut body_chunk);
            }
        }

        Ok(Self {
            status,
            headers,
            body,
        })
    }

    pub fn status(&self) -> &StatusCode {
        &self.status
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn body(&self) -> &Vec<u8> {
        &self.body
    }

    /// Deserialize the response body as JSON.
    ///
    /// # Optional
    ///
    /// This requires the `json` feature enabled.
    #[cfg(feature = "json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    pub fn json<T: DeserializeOwned>(&self) -> Result<T> {
        Ok(serde_json::from_slice(&self.body)?)
    }
}
