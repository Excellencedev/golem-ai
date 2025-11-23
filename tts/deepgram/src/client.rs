// Deepgram Aura TTS client using reqwest
// Matching PR #90 architecture

use golem_tts::config::{get_endpoint_config, get_max_retries_config, get_timeout_config};
use golem_tts::error::{from_reqwest_error, tts_error_from_status};
use golem_tts::golem::tts::types::TtsError;
use log::trace;
use reqwest::{Client, Method, RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub max_retries: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_retries: get_max_retries_config(),
            initial_delay: Duration::from_millis(1000),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
        }
    }
}

#[derive(Clone)]
pub struct DeepgramTtsApi {
    client: Client,
    api_key: String,
    base_url: String,
    api_version: String,
    rate_limit_config: RateLimitConfig,
}

impl DeepgramTtsApi {
    pub fn new(api_key: String, api_version: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(get_timeout_config()))
            .build()
            .unwrap();

        let base_url = get_endpoint_config("https://api.deepgram.com");

        Self {
            client,
            api_key,
            base_url,
            api_version,
            rate_limit_config: RateLimitConfig::default(),
        }
    }

    pub fn with_rate_limit_config(mut self, config: RateLimitConfig) -> Self {
        self.rate_limit_config = config;
        self
    }

    fn create_request(&self, method: Method, url: &str) -> RequestBuilder {
        self.client
            .request(method, url)
            .header("Authorization", format!("Token {}", self.api_key))
            .header("Content-Type", "application/json")
    }

    fn execute_with_retry<F>(&self, operation: F) -> Result<Response, TtsError>
    where
        F: Fn() -> Result<Response, TtsError>,
    {
        let mut delay = self.rate_limit_config.initial_delay;
        let max_retries = self.rate_limit_config.max_retries;

        for attempt in 0..=max_retries {
            match operation() {
                Ok(response) => {
                    if response.status().is_success() {
                        if attempt > 0 {
                            trace!("Deepgram TTS request succeeded after {} retries", attempt);
                        }
                        return Ok(response);
                    } else if response.status().as_u16() == 429 && attempt < max_retries {
                        trace!("Deepgram API rate limited (429), waiting before retry");
                        std::thread::sleep(delay);
                        delay = std::cmp::min(
                            Duration::from_millis(
                                (delay.as_millis() as f64
                                    * self.rate_limit_config.backoff_multiplier)
                                    as u64,
                            ),
                            self.rate_limit_config.max_delay,
                        );
                        continue;
                    } else if response.status().as_u16() >= 500 && attempt < max_retries {
                        trace!(
                            "Deepgram API server error ({}), waiting before retry",
                            response.status().as_u16()
                        );
                        std::thread::sleep(delay);
                        delay = std::cmp::min(
                            Duration::from_millis(
                                (delay.as_millis() as f64
                                    * self.rate_limit_config.backoff_multiplier)
                                    as u64,
                            ),
                            self.rate_limit_config.max_delay,
                        );
                        continue;
                    } else {
                        return Err(tts_error_from_status(response.status()));
                    }
                }
                Err(e) => {
                    if attempt < max_retries {
                        trace!("Deepgram API network error, waiting before retry");
                        std::thread::sleep(delay);
                        delay = std::cmp::min(
                            Duration::from_millis(
                                (delay.as_millis() as f64
                                    * self.rate_limit_config.backoff_multiplier)
                                    as u64,
                            ),
                            self.rate_limit_config.max_delay,
                        );
                        continue;
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        Err(TtsError::InternalError("Max retries exceeded".to_string()))
    }

    pub fn text_to_speech(
        &self,
        request: &TextToSpeechRequest,
        params: Option<&TextToSpeechParams>,
    ) -> Result<Vec<u8>, TtsError> {
        let response = self.text_to_speech_with_metadata(request, params)?;
        Ok(response.audio_data)
    }

    pub fn text_to_speech_with_metadata(
        &self,
        request: &TextToSpeechRequest,
        params: Option<&TextToSpeechParams>,
    ) -> Result<TtsResponse, TtsError> {
        let url = if let Some(p) = params {
            format!(
                "{}/{}/speak?{}",
                self.base_url,
                self.api_version,
                p.to_query_string()
            )
        } else {
            format!("{}/{}/speak", self.base_url, self.api_version)
        };

        trace!("Making TTS request to: {}", url);

        let request_clone = request.clone();

        let operation = || {
            let req = self.create_request(Method::POST, &url).json(&request_clone);
            match req.send() {
                Ok(response) => Ok(response),
                Err(e) => Err(from_reqwest_error("TTS request failed", e)),
            }
        };

        let response = self.execute_with_retry(operation)?;

        if !response.status().is_success() {
            return Err(tts_error_from_status(response.status()));
        }

        let metadata = TtsResponseMetadata::from_response_headers(response.headers())
            .unwrap_or_else(|| TtsResponseMetadata::default());

        match response.bytes() {
            Ok(bytes) => Ok(TtsResponse {
                audio_data: bytes.to_vec(),
                metadata,
            }),
            Err(e) => Err(from_reqwest_error("Failed to read response bytes", e)),
        }
    }
}

// Type definitions matching PR #90

#[derive(Debug, Clone)]
pub struct TtsResponseMetadata {
    pub content_type: String,
    pub dg_request_id: String,
    pub dg_model_name: String,
    pub dg_model_uuid: String,
    pub dg_char_count: u32,
    pub content_length: Option<u64>,
    pub date: String,
}

impl Default for TtsResponseMetadata {
    fn default() -> Self {
        Self {
            content_type: "audio/wav".to_string(),
            dg_request_id: "unknown".to_string(),
            dg_model_name: "aura-asteria-en".to_string(),
            dg_model_uuid: "unknown".to_string(),
            dg_char_count: 0,
            content_length: None,
            date: "unknown".to_string(),
        }
    }
}

impl TtsResponseMetadata {
    pub fn from_response_headers(headers: &reqwest::header::HeaderMap) -> Option<Self> {
        Some(Self {
            content_type: headers.get("content-type")?.to_str().ok()?.to_string(),
            dg_request_id: headers.get("dg-request-id")?.to_str().ok()?.to_string(),
            dg_model_name: headers.get("dg-model-name")?.to_str().ok()?.to_string(),
            dg_model_uuid: headers.get("dg-model-uuid")?.to_str().ok()?.to_string(),
            dg_char_count: headers.get("dg-char-count")?.to_str().ok()?.parse().ok()?,
            content_length: headers
                .get("content-length")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok()),
            date: headers
                .get("date")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("Unknown")
                .to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct TtsResponse {
    pub audio_data: Vec<u8>,
    pub metadata: TtsResponseMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextToSpeechRequest {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextToSpeechParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_rate: Option<u32>,
}

impl Default for TextToSpeechParams {
    fn default() -> Self {
        Self {
            model: None,
            encoding: Some("linear16".to_string()),
            container: None,
            sample_rate: Some(24000),
            bit_rate: None,
        }
    }
}

impl TextToSpeechParams {
    pub fn to_query_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(model) = &self.model {
            params.push(format!("model={}", model));
        }
        if let Some(encoding) = &self.encoding {
            params.push(format!("encoding={}", encoding));
        }
        if let Some(container) = &self.container {
            params.push(format!("container={}", container));
        }
        if let Some(sample_rate) = self.sample_rate {
            params.push(format!("sample_rate={}", sample_rate));
        }
        if let Some(bit_rate) = self.bit_rate {
            params.push(format!("bit_rate={}", bit_rate));
        }

        params.join("&")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    pub voice_id: String,
    pub language: String,
    pub accent: String,
    pub gender: String,
    pub age: String,
    pub characteristics: Vec<String>,
    pub use_cases: Vec<String>,
}

pub fn get_available_models() -> Vec<Model> {
    vec![
        Model {
            name: "Aura Asteria".to_string(),
            voice_id: "aura-asteria-en".to_string(),
            language: "en".to_string(),
            accent: "American".to_string(),
            gender: "Female".to_string(),
            age: "Adult".to_string(),
            characteristics: vec!["warm".to_string(), "professional".to_string()],
            use_cases: vec!["general".to_string(), "narration".to_string()],
        },
        Model {
            name: "Aura Luna".to_string(),
            voice_id: "aura-luna-en".to_string(),
            language: "en".to_string(),
            accent: "American".to_string(),
            gender: "Female".to_string(),
            age: "Young Adult".to_string(),
            characteristics: vec!["friendly".to_string(), "conversational".to_string()],
            use_cases: vec!["general".to_string(), "chat".to_string()],
        },
        Model {
            name: "Aura Stella".to_string(),
            voice_id: "aura-stella-en".to_string(),
            language: "en".to_string(),
            accent: "American".to_string(),
            gender: "Female".to_string(),
            age: "Adult".to_string(),
            characteristics: vec!["clear".to_string(), "articulate".to_string()],
            use_cases: vec!["presentation".to_string(), "professional".to_string()],
        },
        Model {
            name: "Aura Athena".to_string(),
            voice_id: "aura-athena-en".to_string(),
            language: "en".to_string(),
            accent: "British".to_string(),
            gender: "Female".to_string(),
            age: "Adult".to_string(),
            characteristics: vec!["elegant".to_string(), "sophisticated".to_string()],
            use_cases: vec!["narration".to_string(), "presentation".to_string()],
        },
        Model {
            name: "Aura Hera".to_string(),
            voice_id: "aura-hera-en".to_string(),
            language: "en".to_string(),
            accent: "American".to_string(),
            gender: "Female".to_string(),
            age: "Adult".to_string(),
            characteristics: vec!["warm".to_string(), "empathetic".to_string()],
            use_cases: vec!["customer service".to_string(), "assistant".to_string()],
        },
        Model {
            name: "Aura Orion".to_string(),
            voice_id: "aura-orion-en".to_string(),
            language: "en".to_string(),
            accent: "American".to_string(),
            gender: "Male".to_string(),
            age: "Adult".to_string(),
            characteristics: vec!["confident".to_string(), "professional".to_string()],
            use_cases: vec!["presentation".to_string(), "narration".to_string()],
        },
        Model {
            name: "Aura Arcas".to_string(),
            voice_id: "aura-arcas-en".to_string(),
            language: "en".to_string(),
            accent: "American".to_string(),
            gender: "Male".to_string(),
            age: "Middle Aged".to_string(),
            characteristics: vec!["authoritative".to_string(), "clear".to_string()],
            use_cases: vec!["news".to_string(), "announcements".to_string()],
        },
        Model {
            name: "Aura Perseus".to_string(),
            voice_id: "aura-perseus-en".to_string(),
            language: "en".to_string(),
            accent: "American".to_string(),
            gender: "Male".to_string(),
            age: "Adult".to_string(),
            characteristics: vec!["dynamic".to_string(), "engaging".to_string()],
            use_cases: vec!["podcast".to_string(), "entertainment".to_string()],
        },
        Model {
            name: "Aura Angus".to_string(),
            voice_id: "aura-angus-en".to_string(),
            language: "en".to_string(),
            accent: "Irish".to_string(),
            gender: "Male".to_string(),
            age: "Young Adult".to_string(),
            characteristics: vec!["friendly".to_string(), "warm".to_string()],
            use_cases: vec!["conversational".to_string(), "casual".to_string()],
        },
    ]
}
