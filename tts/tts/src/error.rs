use crate::exports::golem::tts::types::TtsError as WitTtsError;
use derive_more::From;

#[derive(Debug, From)]
pub enum Error {
    Http(http::Error),
    WstdHttp(wstd::http::Error),
    Json(serde_json::Error),
    Url(url::ParseError),
    Utf8(std::string::FromUtf8Error),
    IoError(std::io::Error),
    WitTts(WitTtsError),
}

impl From<Error> for WitTtsError {
    fn from(error: Error) -> Self {
        match error {
            Error::Http(e) => WitTtsError::NetworkError(format!("HTTP error: {:?}", e)),
            Error::WstdHttp(e) => WitTtsError::NetworkError(format!("WSTD HTTP error: {:?}", e)),
            Error::Json(e) => WitTtsError::InternalError(format!("JSON error: {}", e)),
            Error::Url(e) => WitTtsError::InvalidConfiguration(format!("URL error: {}", e)),
            Error::Utf8(e) => WitTtsError::InternalError(format!("UTF-8 error: {}", e)),
            Error::IoError(e) => WitTtsError::InternalError(format!("IO error: {}", e)),
            Error::WitTts(e) => e,
        }
    }
}
