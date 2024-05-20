use crate::Response;
use anyhow::{anyhow, Error, Result};
use serde::Serialize;
use std::time::Duration;
use url::Url;
use wasi::http::{
    outgoing_handler,
    types::{
        FieldKey, FieldValue, Headers, Method, OutgoingBody, OutgoingRequest, RequestOptions,
        Scheme,
    },
};

pub struct RequestBuilder {
    // all errors generated while building the request will be deferred and returned when `send` the request.
    request: Result<Request>,
}

impl RequestBuilder {
    pub(crate) fn new(method: Method, url: &str) -> Self {
        Self {
            request: Url::parse(url)
                .map_or_else(|e| Err(Error::new(e)), |url| Ok(Request::new(method, url))),
        }
    }

    /// Add a header to the Request.
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use wasi_http_client::Client;
    /// # fn run() -> Result<()> {
    /// let resp = Client::new().get("https://httpbin.org/get")
    ///     .header("Content-Type", "application/json")
    ///     .send()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn header<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<FieldKey>,
        V: Into<FieldValue>,
    {
        let mut err = None;
        if let Ok(ref mut req) = self.request {
            if let Err(e) = req.headers.set(&key.into(), &[value.into()]) {
                err = Some(e);
            }
        }
        if let Some(e) = err {
            self.request = Err(e.into());
        }
        self
    }

    /// Add a set of headers to the Request.
    ///
    /// Existing headers will be overwritten.
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use wasi_http_client::Client;
    /// # fn run() -> Result<()> {
    /// let resp = Client::new().get("https://httpbin.org/get")
    ///     .headers([("Content-Type", "application/json"), ("Accept", "*/*")])
    ///     .send()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn headers<K, V, I>(mut self, headers: I) -> Self
    where
        K: Into<FieldKey>,
        V: Into<FieldValue>,
        I: IntoIterator<Item = (K, V)>,
    {
        let mut err = None;
        if let Ok(ref mut req) = self.request {
            let entries: Vec<(FieldKey, FieldValue)> = headers
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect();
            match Headers::from_list(&entries) {
                Ok(fields) => req.headers = fields,
                Err(e) => err = Some(e),
            }
        }
        if let Some(e) = err {
            self.request = Err(e.into());
        }
        self
    }

    /// Modify the query string of the Request URL.
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use wasi_http_client::Client;
    /// # fn run() -> Result<()> {
    /// let resp = Client::new().get("https://httpbin.org/get")
    ///     .query(&[("a", "b"), ("c", "d")])
    ///     .send()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn query<T: Serialize + ?Sized>(mut self, query: &T) -> Self {
        let mut err = None;
        if let Ok(ref mut req) = self.request {
            let mut pairs = req.url.query_pairs_mut();
            let serializer = serde_urlencoded::Serializer::new(&mut pairs);
            if let Err(e) = query.serialize(serializer) {
                err = Some(e);
            }
        }
        if let Some(e) = err {
            self.request = Err(e.into());
        }
        self
    }

    /// Set the request body.
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use wasi_http_client::Client;
    /// # fn run() -> Result<()> {
    /// let resp = Client::new().post("https://httpbin.org/post")
    ///     .body("hello".as_bytes())
    ///     .send()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn body(mut self, body: &[u8]) -> Self {
        if let Ok(ref mut req) = self.request {
            req.body = Some(body.into());
        }
        self
    }

    /// Send a JSON body.
    ///
    /// # Optional
    ///
    /// This requires the `json` feature enabled.
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use std::collections::HashMap;
    /// # use wasi_http_client::Client;
    /// # fn run() -> Result<()> {
    /// let resp = Client::new().post("https://httpbin.org/post")
    ///     .json(&HashMap::from([("data", "hello")]))
    ///     .send()?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    pub fn json<T: Serialize + ?Sized>(mut self, json: &T) -> Self {
        let mut err = None;
        if let Ok(ref mut req) = self.request {
            if let Err(e) = req
                .headers
                .set(&"Content-Type".to_string(), &["application/json".into()])
            {
                err = Some(e.into());
            }
            match serde_json::to_vec(json) {
                Ok(data) => req.body = Some(data),
                Err(e) => err = Some(e.into()),
            }
        }
        if let Some(e) = err {
            self.request = Err(e);
        }
        self
    }

    /// Send a form body.
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use wasi_http_client::Client;
    /// # fn run() -> Result<()> {
    /// let resp = Client::new().post("https://httpbin.org/post")
    ///     .form(&[("a", "b"), ("c", "d")])
    ///     .send()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn form<T: Serialize + ?Sized>(mut self, form: &T) -> Self {
        let mut err = None;
        if let Ok(ref mut req) = self.request {
            if let Err(e) = req.headers.set(
                &"Content-Type".to_string(),
                &["application/x-www-form-urlencoded".into()],
            ) {
                err = Some(e.into());
            }
            match serde_urlencoded::to_string(form) {
                Ok(data) => req.body = Some(data.into()),
                Err(e) => err = Some(e.into()),
            }
        }
        if let Some(e) = err {
            self.request = Err(e);
        }
        self
    }

    /// Set the timeout for the initial connect to the HTTP Server.
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use std::time::Duration;
    /// # use wasi_http_client::Client;
    /// # fn run() -> Result<()> {
    /// let resp = Client::new().post("https://httpbin.org/post")
    ///     .connect_timeout(Duration::from_secs(5))
    ///     .send()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        if let Ok(ref mut req) = self.request {
            req.connect_timeout = Some(timeout.as_nanos() as u64);
        }
        self
    }

    /// Send the Request, returning a [`Response`].
    pub fn send(self) -> Result<Response> {
        match self.request {
            Ok(req) => req.send(),
            Err(e) => Err(e),
        }
    }
}

struct Request {
    method: Method,
    url: Url,
    headers: Headers,
    body: Option<Vec<u8>>,
    connect_timeout: Option<u64>,
}

impl Request {
    fn new(method: Method, url: Url) -> Self {
        Self {
            method,
            url,
            headers: Headers::new(),
            body: None,
            connect_timeout: None,
        }
    }

    fn send(self) -> Result<Response> {
        let req = OutgoingRequest::new(self.headers);
        req.set_method(&self.method)
            .map_err(|()| anyhow!("failed to set method"))?;

        let scheme = match self.url.scheme() {
            "http" => Scheme::Http,
            "https" => Scheme::Https,
            other => Scheme::Other(other.to_string()),
        };
        req.set_scheme(Some(&scheme))
            .map_err(|()| anyhow!("failed to set scheme"))?;

        req.set_authority(Some(self.url.authority()))
            .map_err(|()| anyhow!("failed to set authority"))?;

        let path = match self.url.query() {
            Some(query) => format!("{}?{query}", self.url.path()),
            None => self.url.path().to_string(),
        };
        req.set_path_with_query(Some(&path))
            .map_err(|()| anyhow!("failed to set path_with_query"))?;

        let outgoing_body = req
            .body()
            .map_err(|_| anyhow!("outgoing request write failed"))?;
        if let Some(body) = self.body {
            let request_body = outgoing_body
                .write()
                .map_err(|_| anyhow!("outgoing request write failed"))?;
            request_body.blocking_write_and_flush(&body)?;
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
