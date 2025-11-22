use golem_tts::error::Error;
use golem_tts::exports::golem::tts::synthesis::SynthesisOptions as WitSynthesisOptions;
use golem_tts::exports::golem::tts::voices::{
    LanguageInfo as WitLanguageInfo, VoiceFilter as WitVoiceFilter, VoiceInfo as WitVoiceInfo,
};
use golem_tts::golem::tts::types::{
    SynthesisResult as WitSynthesisResult, TextInput as WitTextInput, TtsError as WitTtsError,
};
use golem_tts::http::WstdHttpClient;
use log::trace;
use serde::{Deserialize, Serialize};

pub struct DeepgramClient {
    api_key: String,
    base_url: String,
    api_version: String,
}

impl DeepgramClient {
    pub fn new(api_key: String) -> Self {
        let base_url = std::env::var("DEEPGRAM_BASE_URL")
            .unwrap_or_else(|_| "https://api.deepgram.com".to_string());
        let api_version =
            std::env::var("DEEPGRAM_API_VERSION").unwrap_or_else(|_| "v1".to_string());

        Self {
            api_key,
            base_url,
            api_version,
        }
    }

    pub fn list_voices(&self) -> Result<Vec<WitVoiceInfo>, WitTtsError> {
        trace!("Listing Deepgram Aura voices");

        // Deepgram Aura voices (as of 2024)
        Ok(vec![
            WitVoiceInfo {
                id: "aura-asteria-en".to_string(),
                name: "Asteria".to_string(),
                language: "en".to_string(),
                additional_languages: vec![],
                gender: golem_tts::golem::tts::types::VoiceGender::Female,
                quality: golem_tts::golem::tts::types::VoiceQuality::Neural,
                description: Some("Conversational female voice".to_string()),
                provider: "Deepgram Aura".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["conversational".to_string(), "agent".to_string()],
            },
            WitVoiceInfo {
                id: "aura-luna-en".to_string(),
                name: "Luna".to_string(),
                language: "en".to_string(),
                additional_languages: vec![],
                gender: golem_tts::golem::tts::types::VoiceGender::Female,
                quality: golem_tts::golem::tts::types::VoiceQuality::Neural,
                description: Some("Warm and expressive female voice".to_string()),
                provider: "Deepgram Aura".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["conversational".to_string(), "narration".to_string()],
            },
            WitVoiceInfo {
                id: "aura-stella-en".to_string(),
                name: "Stella".to_string(),
                language: "en".to_string(),
                additional_languages: vec![],
                gender: golem_tts::golem::tts::types::VoiceGender::Female,
                quality: golem_tts::golem::tts::types::VoiceQuality::Neural,
                description: Some("Professional female voice".to_string()),
                provider: "Deepgram Aura".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["professional".to_string(), "news".to_string()],
            },
            WitVoiceInfo {
                id: "aura-athena-en".to_string(),
                name: "Athena".to_string(),
                language: "en".to_string(),
                additional_languages: vec![],
                gender: golem_tts::golem::tts::types::VoiceGender::Female,
                quality: golem_tts::golem::tts::types::VoiceQuality::Neural,
                description: Some("Clear and authoritative female voice".to_string()),
                provider: "Deepgram Aura".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["professional".to_string(), "assistant".to_string()],
            },
            WitVoiceInfo {
                id: "aura-hera-en".to_string(),
                name: "Hera".to_string(),
                language: "en".to_string(),
                additional_languages: vec![],
                gender: golem_tts::golem::tts::types::VoiceGender::Female,
                quality: golem_tts::golem::tts::types::VoiceQuality::Neural,
                description: Some("Sophisticated female voice".to_string()),
                provider: "Deepgram Aura".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["professional".to_string(), "education".to_string()],
            },
            WitVoiceInfo {
                id: "aura-orion-en".to_string(),
                name: "Orion".to_string(),
                language: "en".to_string(),
                additional_languages: vec![],
                gender: golem_tts::golem::tts::types::VoiceGender::Male,
                quality: golem_tts::golem::tts::types::VoiceQuality::Neural,
                description: Some("Deep male voice".to_string()),
                provider: "Deepgram Aura".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["conversational".to_string(), "narration".to_string()],
            },
            WitVoiceInfo {
                id: "aura-arcas-en".to_string(),
                name: "Arcas".to_string(),
                language: "en".to_string(),
                additional_languages: vec![],
                gender: golem_tts::golem::tts::types::VoiceGender::Male,
                quality: golem_tts::golem::tts::types::VoiceQuality::Neural,
                description: Some("Confident male voice".to_string()),
                provider: "Deepgram Aura".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["professional".to_string(), "assistant".to_string()],
            },
            WitVoiceInfo {
                id: "aura-perseus-en".to_string(),
                name: "Perseus".to_string(),
                language: "en".to_string(),
                additional_languages: vec![],
                gender: golem_tts::golem::tts::types::VoiceGender::Male,
                quality: golem_tts::golem::tts::types::VoiceQuality::Neural,
                description: Some("Energetic male voice".to_string()),
                provider: "Deepgram Aura".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["conversational".to_string(), "agent".to_string()],
            },
            WitVoiceInfo {
                id: "aura-angus-en".to_string(),
                name: "Angus".to_string(),
                language: "en".to_string(),
                additional_languages: vec![],
                gender: golem_tts::golem::tts::types::VoiceGender::Male,
                quality: golem_tts::golem::tts::types::VoiceQuality::Neural,
                description: Some("Friendly Irish-accented male voice".to_string()),
                provider: "Deepgram Aura".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["conversational".to_string(), "storytelling".to_string()],
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

    pub fn search_voices(
        &self,
        query: String,
        filter: Option<WitVoiceFilter>,
    ) -> Result<Vec<WitVoiceInfo>, WitTtsError> {
        let all_voices = self.list_voices()?;
        let query_lower = query.to_lowercase();

        Ok(all_voices
            .into_iter()
            .filter(|v| {
                v.name.to_lowercase().contains(&query_lower)
                    || v.description
                        .as_ref()
                        .map_or(false, |d| d.to_lowercase().contains(&query_lower))
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
        Ok(vec![WitLanguageInfo {
            code: "en".to_string(),
            name: "English".to_string(),
            native_name: "English".to_string(),
            voice_count: 9,
        }])
    }

    pub fn synthesize(
        &self,
        input: WitTextInput,
        options: WitSynthesisOptions,
    ) -> Result<WitSynthesisResult, WitTtsError> {
        trace!(
            "Synthesizing speech with Deepgram voice {}",
            options.voice_id
        );
        let http = WstdHttpClient::new();

        #[derive(Serialize)]
        struct SpeakRequest {
            text: String,
        }

        let request_body = SpeakRequest {
            text: input.content.clone(),
        };

        // Deepgram uses query parameters for configuration
        let mut url = format!(
            "{}/{}/speak?model={}",
            self.base_url, self.api_version, options.voice_id
        );

        // Add encoding format
        if let Some(ref audio_config) = options.audio_config {
            let encoding = match audio_config.format {
                golem_tts::golem::tts::types::AudioFormat::Mp3 => "mp3",
                golem_tts::golem::tts::types::AudioFormat::Wav => "wav",
                golem_tts::golem::tts::types::AudioFormat::Pcm => "linear16",
                golem_tts::golem::tts::types::AudioFormat::OggOpus => "opus",
                golem_tts::golem::tts::types::AudioFormat::Aac => "aac",
                golem_tts::golem::tts::types::AudioFormat::Flac => "flac",
                _ => "mp3",
            };
            url = format!("{}&encoding={}", url, encoding);

            if let Some(sample_rate) = audio_config.sample_rate {
                url = format!("{}&sample_rate={}", url, sample_rate);
            }
        }

        let response = http
            .post(&url)
            .header("Authorization", &format!("Token {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)?
            .send()?
            .error_for_status()?;

        let audio_data = response.bytes().to_vec();
        let char_count = input.content.len() as u32;

        Ok(WitSynthesisResult {
            audio_data: audio_data.clone(),
            metadata: golem_tts::golem::tts::types::SynthesisMetadata {
                duration_seconds: (char_count as f32 * 0.05),
                character_count: char_count,
                word_count: input.content.split_whitespace().count() as u32,
                audio_size_bytes: audio_data.len() as u32,
                request_id: uuid::Uuid::new_v4().to_string(),
                provider_info: Some("Deepgram Aura".to_string()),
            },
        })
    }

    pub fn synthesize_batch(
        &self,
        inputs: Vec<WitTextInput>,
        options: WitSynthesisOptions,
    ) -> Result<Vec<WitSynthesisResult>, WitTtsError> {
        inputs
            .into_iter()
            .map(|input| self.synthesize(input, options.clone()))
            .collect()
    }
}
