use crate::error::Error;
use bytes::Bytes;
use http::Request;
use log::trace;
use reqwest::Client;
use std::sync::OnceLock;
use wstd::runtime::block_on;

/// Global HTTP client instance
static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();

/// HTTP client using wstd (WASI) backend
pub struct WstdHttpClient;

impl WstdHttpClient {
    pub fn new() -> Self {
        HTTP_CLIENT.get_or_init(|| {
            reqwest::Client::builder()
                .user_agent("golem-tts/0.0.0")
                .build()
                .expect("Failed to create HTTP client")
        });
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
        
        let client = HTTP_CLIENT.get().ok_or_else(|| {
            Error::Http(http::Error::from(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "HTTP client not initialized",
            )))
        })?;

        let mut req = match builder.method {
            http::Method::GET => client.get(&builder.url),
            http::Method::POST => client.post(&builder.url),
            http::Method::DELETE => client.delete(&builder.url),
            _ => {
                return Err(Error::Http(http::Error::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Unsupported HTTP method",
                ))))
            }
        };

        for (key, value) in builder.headers {
            req = req.header(key, value);
        }

        if let Some(body) = builder.body {
            req = req.body(body);
        }

        let response = block_on(async { req.send().await })?;
        let status = response.status();
        let headers = response.headers().clone();
        let body = block_on(async { response.bytes().await })?;

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
            body,
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
            Err(Error::Http(http::Error::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("HTTP {}: {}", self.status, error_text),
            ))))
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
