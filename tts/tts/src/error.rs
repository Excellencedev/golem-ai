use crate::exports::golem::tts::types::TtsError as WitTtsError;
use crate::golem::tts::types::TtsError;
use derive_more::From;

#[derive(Debug, From)]
pub enum Error {
    Http(http::Error),
    Reqwest(reqwest::Error),
    Json(serde_json::Error),
    Url(url::ParseError),
    Utf8(std::string::FromUtf8Error),
    IoError(std::io::Error),
    TtsError(TtsError),
}

impl From<Error> for WitTtsError {
    fn from(error: Error) -> Self {
        match error {
            Error::Http(e) => WitTtsError::NetworkError(format!("HTTP error: {:?}", e)),
            Error::Reqwest(e) => {
                if e.is_timeout() {
                    WitTtsError::NetworkError("Request timeout".to_string())
                } else if e.is_connect() {
                    WitTtsError::NetworkError(format!("Connection error: {}", e))
                } else if e.is_status() {
                    if let Some(status) = e.status() {
                        match status.as_u16() {
                            401 => WitTtsError::Unauthorized("Unauthorized".to_string()),
                            403 => WitTtsError::AccessDenied("Access denied".to_string()),
                            429 => WitTtsError::RateLimited(60),
                            503 => WitTtsError::ServiceUnavailable("Service unavailable".to_string()),
                            _ => WitTtsError::InternalError(format!("HTTP {}: {}", status, e)),
                        }
                    } else {
                        WitTtsError::InternalError(format!("Request error: {}", e))
                    }
                } else {
                    WitTtsError::NetworkError(format!("Request error: {}", e))
                }
            }
            Error::Json(e) => WitTtsError::InternalError(format!("JSON error: {}", e)),
            Error::Url(e) => WitTtsError::InvalidConfiguration(format!("URL error: {}", e)),
            Error::Utf8(e) => WitTtsError::InternalError(format!("UTF-8 error: {}", e)),
            Error::IoError(e) => WitTtsError::InternalError(format!("IO error: {}", e)),
            Error::TtsError(e) => e,
        }
    }
}

impl From<TtsError> for WitTtsError {
    fn from(error: TtsError) -> Self {
        error
    }
}
