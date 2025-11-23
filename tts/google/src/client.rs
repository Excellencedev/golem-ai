// Google Cloud TTS client with service account authentication
use golem_tts::config::{
    get_config_with_default, get_max_retries_config, get_timeout_config, validate_config_key,
};
use golem_tts::error::{from_reqwest_error, internal_error, tts_error_from_status};
use golem_tts::golem::tts::types::TtsError;
use log::trace;
use reqwest::{Client, Method};
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
pub struct GoogleTtsClient {
    client: Client,
    access_token: String,
    project_id: String,
    rate_limit_config: RateLimitConfig,
}

impl GoogleTtsClient {
    pub fn new() -> Result<Self, TtsError> {
        // Try environment variables first (simpler for WASI)
        let access_token = validate_config_key("GOOGLE_ACCESS_TOKEN")?;
        let project_id = get_config_with_default("GOOGLE_PROJECT_ID", "default-project");

        let client = Client::builder()
            .timeout(Duration::from_secs(get_timeout_config()))
            .build()
            .map_err(|e| internal_error(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            client,
            access_token,
            project_id,
            rate_limit_config: RateLimitConfig::default(),
        })
    }

    pub fn synthesize_speech(
        &self,
        text: &str,
        voice_name: &str,
        language_code: &str,
    ) -> Result<Vec<u8>, TtsError> {
        let url = "https://texttospeech.googleapis.com/v1/text:synthesize";

        #[derive(Serialize)]
        struct Voice {
            #[serde(rename = "languageCode")]
            language_code: String,
            name: String,
        }

        #[derive(Serialize)]
        struct AudioConfig {
            #[serde(rename = "audioEncoding")]
            audio_encoding: String,
        }

        #[derive(Serialize)]
        struct Input {
            text: String,
        }

        #[derive(Serialize)]
        struct Request {
            input: Input,
            voice: Voice,
            #[serde(rename = "audioConfig")]
            audio_config: AudioConfig,
        }

        let body = Request {
            input: Input {
                text: text.to_string(),
            },
            voice: Voice {
                language_code: language_code.to_string(),
                name: voice_name.to_string(),
            },
            audio_config: AudioConfig {
                audio_encoding: "MP3".to_string(),
            },
        };

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .map_err(|e| from_reqwest_error("Google TTS synthesize", e))?;

        if !response.status().is_success() {
            return Err(tts_error_from_status(response.status()));
        }

        #[derive(Deserialize)]
        struct Response {
            #[serde(rename = "audioContent")]
            audio_content: String,
        }

        let response_body: Response = response
            .json()
            .map_err(|e| from_reqwest_error("Parsing Google response", e))?;

        base64::decode(&response_body.audio_content)
            .map_err(|e| internal_error(format!("Base64 decode error: {}", e)))
    }

    pub fn list_voices() -> Vec<GoogleVoice> {
        // Hardcoded list of popular Google voices
        vec![
            GoogleVoice {
                name: "en-US-Neural2-A".to_string(),
                display_name: "Neural2 A".to_string(),
                language_code: "en-US".to_string(),
                gender: "Female".to_string(),
            },
            GoogleVoice {
                name: "en-US-Neural2-C".to_string(),
                display_name: "Neural2 C".to_string(),
                language_code: "en-US".to_string(),
                gender: "Female".to_string(),
            },
            GoogleVoice {
                name: "en-US-Neural2-D".to_string(),
                display_name: "Neural2 D".to_string(),
                language_code: "en-US".to_string(),
                gender: "Male".to_string(),
            },
            GoogleVoice {
                name: "en-US-Wavenet-A".to_string(),
                display_name: "Wavenet A".to_string(),
                language_code: "en-US".to_string(),
                gender: "Male".to_string(),
            },
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleVoice {
    pub name: String,
    pub display_name: String,
    pub language_code: String,
    pub gender: String,
}
