use golem_tts::error::Error;
use golem_tts::exports::golem::tts::voices::{LanguageInfo as WitLanguageInfo, VoiceFilter as WitVoiceFilter, VoiceInfo as WitVoiceInfo};
use golem_tts::golem::tts::types::{
    TextInput as WitTextInput, SynthesisResult as WitSynthesisResult, TtsError as WitTtsError,
    VoiceGender, VoiceQuality,
};
use golem_tts::exports::golem::tts::synthesis::SynthesisOptions as WitSynthesisOptions;
use golem_tts::http::WstdHttpClient;
use log::trace;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct GoogleCloudTtsClient {
    access_token: String,
    project_id: String,
    base_url: String,
}

#[derive(Deserialize)]
struct ServiceAccountKey {
    #[serde(rename = "type")]
    key_type: String,
    project_id: String,
    private_key_id: String,
    private_key: String,
    client_email: String,
    client_id: String,
    auth_uri: String,
    token_uri: String,
}

#[derive(Serialize)]
struct JwtClaims {
    iss: String,
    scope: String,
    aud: String,
    exp: i64,
    iat: i64,
}

#[derive(Serialize)]
struct TokenRequest {
    grant_type: String,
    assertion: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u32,
    token_type: String,
}

impl GoogleCloudTtsClient {
    pub fn new(credentials_path: String) -> Result<Self, WitTtsError> {
        trace!("Creating Google Cloud TTS client with credentials from {}", credentials_path);
        
        // Read and parse service account JSON
        let credentials_json = std::fs::read_to_string(&credentials_path)
            .map_err(|e| WitTtsError::ConfigurationError(format!("Failed to read credentials file: {}", e)))?;
        
        let service_account: ServiceAccountKey = serde_json::from_str(&credentials_json)
            .map_err(|e| WitTtsError::ConfigurationError(format!("Invalid service account JSON: {}", e)))?;

        // Get access token using service account
        let access_token = Self::get_access_token(&service_account)?;
        
        let base_url = std::env::var("GOOGLE_TTS_BASE_URL")
            .unwrap_or_else(|_| "https://texttospeech.googleapis.com/v1".to_string());

        Ok(Self {
            access_token,
            project_id: service_account.project_id,
            base_url,
        })
    }

    fn get_access_token(service_account: &ServiceAccountKey) -> Result<String, WitTtsError> {
        // Note: This is a simplified OAuth2 flow
        // In production, you'd use proper JWT signing with RS256
        // For now, returning a placeholder that would need proper implementation
        
        Err(WitTtsError::ConfigurationError(
            "OAuth2 JWT signing not yet implemented - requires RS256 crypto library".to_string()
        ))
        
        // TODO: Implement proper JWT signing:
        // 1. Create JWT header and claims
        // 2. Sign with private_key using RS256
        // 3. Exchange JWT for access token at token_uri
        // 4. Return access_token
    }

    pub fn list_voices(&self) -> Result<Vec<WitVoiceInfo>, WitTtsError> {
        trace!("Listing Google Cloud TTS voices");
        
        // For now, return a curated list of popular Google voices
        // In production, call: GET https://texttospeech.googleapis.com/v1/voices
        Ok(vec![
            // English (US) voices
            WitVoiceInfo {
                id: "en-US-Neural2-A".to_string(),
                name: "en-US-Neural2-A".to_string(),
                language: "en-US".to_string(),
                additional_languages: vec![],
                gender: VoiceGender::Female,
                quality: VoiceQuality::Neural,
                description: Some("US English female voice (Neural2)".to_string()),
                provider: "Google Cloud TTS".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["general".to_string(), "assistant".to_string()],
            },
            WitVoiceInfo {
                id: "en-US-Neural2-C".to_string(),
                name: "en-US-Neural2-C".to_string(),
                language: "en-US".to_string(),
                additional_languages: vec![],
                gender: VoiceGender::Female,
                quality: VoiceQuality::Neural,
                description: Some("US English female voice (Neural2)".to_string()),
                provider: "Google Cloud TTS".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["general".to_string()],
            },
            WitVoiceInfo {
                id: "en-US-Neural2-D".to_string(),
                name: "en-US-Neural2-D".to_string(),
                language: "en-US".to_string(),
                additional_languages: vec![],
                gender: VoiceGender::Male,
                quality: VoiceQuality::Neural,
                description: Some("US English male voice (Neural2)".to_string()),
                provider: "Google Cloud TTS".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["general".to_string(), "assistant".to_string()],
            },
            WitVoiceInfo {
                id: "en-US-Neural2-F".to_string(),
                name: "en-US-Neural2-F".to_string(),
                language: "en-US".to_string(),
                additional_languages: vec![],
                gender: VoiceGender::Female,
                quality: VoiceQuality::Neural,
                description: Some("US English female voice (Neural2)".to_string()),
                provider: "Google Cloud TTS".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["general".to_string()],
            },
            // WaveNet voices
            WitVoiceInfo {
                id: "en-US-Wavenet-A".to_string(),
                name: "en-US-Wavenet-A".to_string(),
                language: "en-US".to_string(),
                additional_languages: vec![],
                gender: VoiceGender::Male,
                quality: VoiceQuality::Neural,
                description: Some("US English male voice (WaveNet)".to_string()),
                provider: "Google Cloud TTS".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["general".to_string()],
            },
            WitVoiceInfo {
                id: "en-US-Wavenet-B".to_string(),
                name: "en-US-Wavenet-B".to_string(),
                language: "en-US".to_string(),
                additional_languages: vec![],
                gender: VoiceGender::Male,
                quality: VoiceQuality::Neural,
                description: Some("US English male voice (WaveNet)".to_string()),
                provider: "Google Cloud TTS".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["general".to_string()],
            },
            WitVoiceInfo {
                id: "en-US-Wavenet-C".to_string(),
                name: "en-US-Wavenet-C".to_string(),
                language: "en-US".to_string(),
                additional_languages: vec![],
                gender: VoiceGender::Female,
                quality: VoiceQuality::Neural,
                description: Some("US English female voice (WaveNet)".to_string()),
                provider: "Google Cloud TTS".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["general".to_string()],
            },
            WitVoiceInfo {
                id: "en-US-Wavenet-D".to_string(),
                name: "en-US-Wavenet-D".to_string(),
                language: "en-US".to_string(),
                additional_languages: vec![],
                gender: VoiceGender::Male,
                quality: VoiceQuality::Neural,
                description: Some("US English male voice (WaveNet)".to_string()),
                provider: "Google Cloud TTS".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["general".to_string()],
            },
            // English (GB)
            WitVoiceInfo {
                id: "en-GB-Neural2-A".to_string(),
                name: "en-GB-Neural2-A".to_string(),
                language: "en-GB".to_string(),
                additional_languages: vec![],
                gender: VoiceGender::Female,
                quality: VoiceQuality::Neural,
                description: Some("British English female voice (Neural2)".to_string()),
                provider: "Google Cloud TTS".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["general".to_string()],
            },
            WitVoiceInfo {
                id: "en-GB-Neural2-B".to_string(),
                name: "en-GB-Neural2-B".to_string(),
                language: "en-GB".to_string(),
                additional_languages: vec![],
                gender: VoiceGender::Male,
                quality: VoiceQuality::Neural,
                description: Some("British English male voice (Neural2)".to_string()),
                provider: "Google Cloud TTS".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["general".to_string()],
            },
        ])
    }

    pub fn get_voice(&self, voice_id: String) -> Result<WitVoiceInfo, WitTtsError> {
        let voices = self.list_voices()?;
        voices
            .into_iter()
            .find(|v| v.id == voice_id)
            .ok_or_else(|| WitTtsError::VoiceNotFound(voice_id))
    }

    pub fn search_voices(&self, query: String, filter: Option<WitVoiceFilter>) -> Result<Vec<WitVoiceInfo>, WitTtsError> {
        let all_voices = self.list_voices()?;
        let query_lower = query.to_lowercase();
        
        Ok(all_voices
            .into_iter()
            .filter(|v| {
                v.name.to_lowercase().contains(&query_lower) ||
                v.language.to_lowercase().contains(&query_lower)
            })
            .filter(|v| {
                if let Some(ref f) = filter {
                    if let Some(ref lang) = f.language {
                        if !v.language.starts_with(lang) {
                            return false;
                        }
                    }
                    if let Some(gender) = f.gender {
                        if v.gender != gender {
                            return false;
                        }
                    }
                }
                true
            })
            .collect())
    }

    pub fn list_languages(&self) -> Result<Vec<WitLanguageInfo>, WitTtsError> {
        Ok(vec![
            WitLanguageInfo {
                code: "en-US".to_string(),
                name: "English (US)".to_string(),
                native_name: "English (US)".to_string(),
                voice_count: 8,
            },
            WitLanguageInfo {
                code: "en-GB".to_string(),
                name: "English (UK)".to_string(),
                native_name: "English (UK)".to_string(),
                voice_count: 2,
            },
        ])
    }

    pub fn synthesize(&self, input: WitTextInput, options: WitSynthesisOptions) -> Result<WitSynthesisResult, WitTtsError> {
        trace!("Synthesizing speech with Google Cloud TTS voice {}", options.voice_id);
        
        // Note: This requires proper access token
        return Err(WitTtsError::AuthenticationError("OAuth2 authentication not yet implemented".to_string()));
        
        // TODO: Implement actual synthesis:
        /*
        let http = WstdHttpClient::new();

        #[derive(Serialize)]
        struct SynthesizeRequest {
            input: SynthesisInput,
            voice: VoiceSelectionParams,
            audio_config: AudioConfig,
        }

        #[derive(Serialize)]
        struct SynthesisInput {
            text: String,
        }

        #[derive(Serialize)]
        struct VoiceSelectionParams {
            language_code: String,
            name: String,
        }

        #[derive(Serialize)]
        struct AudioConfig {
            audio_encoding: String,
            sample_rate_hertz: Option<u32>,
            speaking_rate: Option<f32>,
            pitch: Option<f32>,
            volume_gain_db: Option<f32>,
        }

        let language_code = options.voice_id.split('-').take(2).collect::<Vec<_>>().join("-");
        
        let request_body = SynthesizeRequest {
            input: SynthesisInput {
                text: input.content.clone(),
            },
            voice: VoiceSelectionParams {
                language_code,
                name: options.voice_id.clone(),
            },
            audio_config: AudioConfig {
                audio_encoding: "MP3".to_string(),
                sample_rate_hertz: options.audio_config.as_ref().and_then(|c| c.sample_rate),
                speaking_rate: options.voice_settings.as_ref().map(|s| s.speed),
                pitch: options.voice_settings.as_ref().map(|s| s.pitch),
                volume_gain_db: options.voice_settings.as_ref().map(|s| (s.volume - 1.0) * 10.0),
            },
        };

        let url = format!("{}/text:synthesize", self.base_url);
        let response = http
            .post(&url)
            .header("Authorization", &format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .json(&request_body)?
            .send()?
            .error_for_status()?;

        #[derive(Deserialize)]
        struct SynthesizeResponse {
            audio_content: String, // base64 encoded
        }

        let response_body: SynthesizeResponse = response.json()?;
        let audio_data = base64::decode(&response_body.audio_content)
            .map_err(|e| Error::from(e))?;

        let char_count = input.content.len() as u32;

        Ok(WitSynthesisResult {
            audio_data: audio_data.clone(),
            metadata: golem_tts::golem::tts::types::SynthesisMetadata {
                duration_seconds: (char_count as f32 * 0.05),
                character_count: char_count,
                word_count: input.content.split_whitespace().count() as u32,
                audio_size_bytes: audio_data.len() as u32,
                request_id: uuid::Uuid::new_v4().to_string(),
                provider_info: Some("Google Cloud TTS".to_string()),
            },
        })
        */
    }

    pub fn synthesize_batch(&self, inputs: Vec<WitTextInput>, options: WitSynthesisOptions) -> Result<Vec<WitSynthesisResult>, WitTtsError> {
        inputs
            .into_iter()
            .map(|input| self.synthesize(input, options.clone()))
            .collect()
    }
}
