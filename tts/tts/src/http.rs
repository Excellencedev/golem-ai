use crate::error::Error;
use bytes::Bytes;
use log::trace;
use wstd::http::{Client, Request};
use wstd::io::AsyncRead;
use wstd::runtime::block_on;

/// HTTP client using wstd (WASI) backend  
pub struct WstdHttpClient;

impl WstdHttpClient {
    pub fn new() -> Self {
        Self
    }

    pub fn get(&self, url: &str) -> RequestBuilder {
        RequestBuilder {
            method: http::Method::GET,
            url: url.to_string(),
            headers: vec![],
            body: None,
        }
    }

    pub fn post(&self, url: &str) -> RequestBuilder {
        RequestBuilder {
            method: http::Method::POST,
            url: url.to_string(),
            headers: vec![],
            body: None,
        }
    }

    pub fn delete(&self, url: &str) -> RequestBuilder {
        RequestBuilder {
            method: http::Method::DELETE,
            url: url.to_string(),
            headers: vec![],
            body: None,
        }
    }

    pub fn execute(&self, builder: RequestBuilder) -> Result<Response, Error> {
        trace!("HTTP {} {}", builder.method, builder.url);

        // Build http::Request
        let mut req_builder = match builder.method {
            http::Method::GET => Request::get(&builder.url),
            http::Method::POST => Request::post(&builder.url),
            http::Method::DELETE => Request::delete(&builder.url),
            _ => {
                return Err(Error::IoError(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Unsupported HTTP method",
                )))
            }
        };

        // Add headers
        for (key, value) in builder.headers {
            req_builder = req_builder.header(&key, &value);
        }

        // Build request with body using custom BodyWrapper
        let body_data = builder.body.unwrap_or_default();
        let http_req = req_builder.body(BodyWrapper::new(body_data))?;

        // Use Client to send the request
        let client = Client::new();
        let mut response = block_on(async { client.send(http_req).await })?;

        let status = response.status();
        let headers = response.headers().clone();

        // Read body bytes from IncomingBody
        let body = block_on(async {
            let mut buf = Vec::new();
            response.body_mut().read_to_end(&mut buf).await?;
            Ok::<Vec<u8>, wstd::http::Error>(buf)
        })?;

        trace!("HTTP response status: {}", status);

        Ok(Response {
            status: status.as_u16(),
            headers: headers
                .iter()
                .map(|(k, v)| {
                    (
                        k.as_str().to_string(),
                        v.to_str().unwrap_or_default().to_string(),
                    )
                })
                .collect(),
            body: Bytes::from(body),
        })
    }
}

pub struct RequestBuilder {
    method: http::Method,
    url: String,
    headers: Vec<(String, String)>,
    body: Option<Vec<u8>>,
}

impl RequestBuilder {
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }

    pub fn json<T: serde::Serialize>(mut self, body: &T) -> Result<Self, Error> {
        let json = serde_json::to_vec(body)?;
        self.body = Some(json);
        self.headers
            .push(("content-type".to_string(), "application/json".to_string()));
        Ok(self)
    }

    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    pub fn send(self) -> Result<Response, Error> {
        WstdHttpClient::new().execute(self)
    }
}

pub struct Response {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Bytes,
}

impl Response {
    pub fn error_for_status(self) -> Result<Self, Error> {
        if self.status >= 400 {
            let error_text = String::from_utf8_lossy(&self.body).to_string();
            Err(Error::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("HTTP {}: {}", self.status, error_text),
            )))
        } else {
            Ok(self)
        }
    }

    pub fn json<T: serde::de::DeserializeOwned>(self) -> Result<T, Error> {
        Ok(serde_json::from_slice(&self.body)?)
    }

    pub fn text(self) -> Result<String, Error> {
        Ok(String::from_utf8(self.body.to_vec())?)
    }

    pub fn bytes(self) -> Bytes {
        self.body
    }
}

/// Custom body wrapper that implements wstd::http::Body and AsyncRead
struct BodyWrapper {
    data: Vec<u8>,
    position: usize,
}

impl BodyWrapper {
    fn new(data: Vec<u8>) -> Self {
        Self { data, position: 0 }
    }
}

impl wstd::http::Body for BodyWrapper {
    fn len(&self) -> Option<usize> {
        Some(self.data.len() - self.position)
    }
}

impl wstd::io::AsyncRead for BodyWrapper {
    async fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.position >= self.data.len() {
            return Ok(0); // EOF
        }
        let remaining = &self.data[self.position..];
        let amt = std::cmp::min(buf.len(), remaining.len());
        buf[..amt].copy_from_slice(&remaining[..amt]);
        self.position += amt;
        Ok(amt)
    }
}
