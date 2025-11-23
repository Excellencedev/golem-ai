// ElevenLabs client with metadata support
use golem_tts::config::{
    get_endpoint_config, get_max_retries_config, get_timeout_config, validate_config_key,
};
use golem_tts::error::{from_reqwest_error, tts_error_from_status};
use golem_tts::golem::tts::types::{SynthesisMetadata, TtsError};
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
pub struct ElevenLabsClient {
    client: Client,
    api_key: String,
    base_url: String,
    rate_limit_config: RateLimitConfig,
}

impl ElevenLabsClient {
    pub fn new() -> Result<Self, TtsError> {
        let api_key = validate_config_key("ELEVENLABS_API_KEY")?;
        let client = Client::builder()
            .timeout(Duration::from_secs(get_timeout_config()))
            .build()
            .unwrap();

        let base_url = get_endpoint_config("https://api.elevenlabs.io");

        Ok(Self {
            client,
            api_key,
            base_url,
            rate_limit_config: RateLimitConfig::default(),
        })
    }

    fn create_request(&self, method: Method, url: &str) -> RequestBuilder {
        self.client
            .request(method, url)
            .header("xi-api-key", &self.api_key)
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
                        return Ok(response);
                    } else if response.status().as_u16() == 429 && attempt < max_retries {
                        trace!("ElevenLabs rate limited, retrying");
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
                        trace!("ElevenLabs server error, retrying");
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

        Err(TtsError::NetworkError("Max retries exceeded".to_string()))
    }

    pub fn text_to_speech(
        &self,
        text: &str,
        voice_id: &str,
    ) -> Result<SynthesisResponse, TtsError> {
        let url = format!("{}/v1/text-to-speech/{}", self.base_url, voice_id);

        #[derive(Serialize)]
        struct Request {
            text: String,
            model_id: String,
        }

        let body = Request {
            text: text.to_string(),
            model_id: "eleven_monolingual_v1".to_string(),
        };

        let char_count = text.chars().count() as u32;

        let response = self.execute_with_retry(|| {
            self.create_request(Method::POST, &url)
                .json(&body)
                .send()
                .map_err(|e| from_reqwest_error("ElevenLabs text_to_speech", e))
        })?;

        let audio_data = response
            .bytes()
            .map(|b| b.to_vec())
            .map_err(|e| from_reqwest_error("Reading ElevenLabs response", e))?;

        // Create metadata
        let audio_size = audio_data.len() as u32;
        let duration_seconds = estimate_audio_duration(&audio_data);
        let word_count = text.split_whitespace().count() as u32;

        let metadata = SynthesisMetadata {
            duration_seconds,
            character_count: char_count,
            word_count,
            audio_size_bytes: audio_size,
            request_id: format!("elevenlabs-{}", chrono::Utc::now().timestamp()),
            provider_info: Some("ElevenLabs TTS - MP3 44.1kHz".to_string()),
        };

        Ok(SynthesisResponse {
            audio_data,
            metadata,
        })
    }

    pub fn list_voices(&self) -> Result<Vec<Voice>, TtsError> {
        let url = format!("{}/v1/voices", self.base_url);

        let response = self.execute_with_retry(|| {
            self.create_request(Method::GET, &url)
                .send()
                .map_err(|e| from_reqwest_error("ElevenLabs list_voices", e))
        })?;

        #[derive(Deserialize)]
        struct VoicesResponse {
            voices: Vec<Voice>,
        }

        let voices_response: VoicesResponse = response
            .json()
            .map_err(|e| from_reqwest_error("Parsing ElevenLabs voices", e))?;

        Ok(voices_response.voices)
    }
}

#[derive(Debug, Clone)]
pub struct SynthesisResponse {
    pub audio_data: Vec<u8>,
    pub metadata: SynthesisMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voice {
    pub voice_id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub preview_url: Option<String>,
}

fn estimate_audio_duration(audio_data: &[u8]) -> f32 {
    // MP3 at 128kbps ~= 16000 bytes/second
    if audio_data.is_empty() {
        return 0.0;
    }
    (audio_data.len() as f32) / 16000.0
}
