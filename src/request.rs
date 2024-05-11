use crate::Response;
use anyhow::{anyhow, Result};
use std::time::Duration;
use url::Url;
use wasi::http::{
    outgoing_handler,
    types::{FieldValue, Headers, Method, OutgoingBody, OutgoingRequest, RequestOptions, Scheme},
};

pub struct RequestBuilder {
    method: Method,
    url: String,
    headers: Headers,
    body: Vec<u8>,
    connect_timeout: Option<u64>,
}

impl RequestBuilder {
    pub fn new(method: Method, url: &str) -> Self {
        Self {
            method,
            url: url.to_string(),
            headers: Headers::new(),
            body: vec![],
            connect_timeout: None,
        }
    }

    pub fn header(self, key: &str, value: &str) -> Result<Self> {
        self.headers
            .set(&key.to_string(), &[FieldValue::from(value)])?;
        Ok(self)
    }

    pub fn body(mut self, body: &[u8]) -> Self {
        self.body = Vec::from(body);
        self
    }

    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = Some(timeout.as_nanos() as u64);
        self
    }

    pub fn send(self) -> Result<Response> {
        let req = OutgoingRequest::new(self.headers);
        req.set_method(&self.method)
            .map_err(|()| anyhow!("failed to set method"))?;

        let url = Url::parse(self.url.as_str())?;
        let scheme = match url.scheme() {
            "http" => Scheme::Http,
            "https" => Scheme::Https,
            other => Scheme::Other(other.to_string()),
        };
        req.set_scheme(Some(&scheme))
            .map_err(|()| anyhow!("failed to set scheme"))?;

        req.set_authority(Some(url.authority()))
            .map_err(|()| anyhow!("failed to set authority"))?;

        let path = match url.query() {
            Some(query) => format!("{}?{query}", url.path()),
            None => url.path().to_string(),
        };
        req.set_path_with_query(Some(&path))
            .map_err(|()| anyhow!("failed to set path_with_query"))?;

        let outgoing_body = req
            .body()
            .map_err(|_| anyhow!("outgoing request write failed"))?;
        if !self.body.is_empty() {
            let request_body = outgoing_body
                .write()
                .map_err(|_| anyhow!("outgoing request write failed"))?;
            request_body.blocking_write_and_flush(&self.body)?;
        }
        OutgoingBody::finish(outgoing_body, None)?;

        let options = RequestOptions::new();
        options
            .set_connect_timeout(self.connect_timeout)
            .map_err(|()| anyhow!("failed to set connect_timeout"))?;

        let future_response = outgoing_handler::handle(req, Some(options))?;
        let incoming_response = match future_response.get() {
            Some(result) => result.map_err(|()| anyhow!("response already taken"))?,
            None => {
                let pollable = future_response.subscribe();
                pollable.block();

                future_response
                    .get()
                    .expect("incoming response available")
                    .map_err(|()| anyhow!("response already taken"))?
            }
        }?;
        drop(future_response);

        Response::new(incoming_response)
    }
}
