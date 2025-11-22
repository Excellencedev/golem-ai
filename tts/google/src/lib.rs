mod auth;
mod client;
mod conversions;

use client::GoogleCloudTtsClient;
use golem_tts::exports::golem::tts::advanced::{
    AudioSample as WitAudioSample, LongFormJob as WitLongFormJob,
    LongFormResult as WitLongFormResult, PronunciationEntry as WitPronunciationEntry,
    VoiceDesignParams as WitVoiceDesignParams,
};
use golem_tts::exports::golem::tts::streaming::{
    StreamSession as WitStreamSession, StreamStatus as WitStreamStatus,
};
use golem_tts::exports::golem::tts::synthesis::{
    SynthesisOptions as WitSynthesisOptions, ValidationResult as WitValidationResult,
};
use golem_tts::exports::golem::tts::voices::{
    LanguageInfo as WitLanguageInfo, VoiceFilter as WitVoiceFilter, VoiceInfo as WitVoiceInfo,
};
use golem_tts::golem::tts::types::{
    AudioChunk as WitAudioChunk, SynthesisResult as WitSynthesisResult, TextInput as WitTextInput,
    TimingInfo as WitTimingInfo, TtsError as WitTtsError,
};
use golem_tts::{
    config::with_config_key,
    durability::{DurableTts, ExtendedGuest},
    guest::{TtsAdvancedGuest, TtsStreamingGuest, TtsSynthesisGuest, TtsVoicesGuest},
};

struct GoogleComponent;

impl GoogleComponent {
    const ENV_VAR_NAME: &'static str = "GOOGLE_APPLICATION_CREDENTIALS";
}

impl TtsVoicesGuest for GoogleComponent {
    fn list_voices(filter: Option<WitVoiceFilter>) -> Result<Vec<WitVoiceInfo>, WitTtsError> {
        with_config_key(Self::ENV_VAR_NAME, Err, |credentials_path| {
            let client = GoogleCloudTtsClient::new(credentials_path)?;
            let all_voices = client.list_voices()?;

            if let Some(f) = filter {
                Ok(all_voices
                    .into_iter()
                    .filter(|v| {
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
                        true
                    })
                    .collect())
            } else {
                Ok(all_voices)
            }
        })
    }

    fn get_voice(voice_id: String) -> Result<WitVoiceInfo, WitTtsError> {
        with_config_key(Self::ENV_VAR_NAME, Err, |credentials_path| {
            let client = GoogleCloudTtsClient::new(credentials_path)?;
            client.get_voice(voice_id)
        })
    }

    fn search_voices(
        query: String,
        filter: Option<WitVoiceFilter>,
    ) -> Result<Vec<WitVoiceInfo>, WitTtsError> {
        with_config_key(Self::ENV_VAR_NAME, Err, |credentials_path| {
            let client = GoogleCloudTtsClient::new(credentials_path)?;
            client.search_voices(query, filter)
        })
    }

    fn list_languages() -> Result<Vec<WitLanguageInfo>, WitTtsError> {
        with_config_key(Self::ENV_VAR_NAME, Err, |credentials_path| {
            let client = GoogleCloudTtsClient::new(credentials_path)?;
            client.list_languages()
        })
    }
}

impl TtsSynthesisGuest for GoogleComponent {
    fn synthesize(
        input: WitTextInput,
        options: WitSynthesisOptions,
    ) -> Result<WitSynthesisResult, WitTtsError> {
        with_config_key(Self::ENV_VAR_NAME, Err, |credentials_path| {
            let client = GoogleCloudTtsClient::new(credentials_path)?;
            client.synthesize(input, options)
        })
    }

    fn synthesize_batch(
        inputs: Vec<WitTextInput>,
        options: WitSynthesisOptions,
    ) -> Result<Vec<WitSynthesisResult>, WitTtsError> {
        with_config_key(Self::ENV_VAR_NAME, Err, |credentials_path| {
            let client = GoogleCloudTtsClient::new(credentials_path)?;
            client.synthesize_batch(inputs, options)
        })
    }

    fn get_timing_marks(
        _input: WitTextInput,
        _voice_id: String,
    ) -> Result<Vec<WitTimingInfo>, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("Google Cloud TTS timing marks available via timepoints in synthesis, not as separate call".to_string()))
    }

    fn validate_input(
        input: WitTextInput,
        _voice_id: String,
    ) -> Result<WitValidationResult, WitTtsError> {
        let char_count = input.content.len() as u32;
        let is_valid = char_count > 0 && char_count <= 5000;

        Ok(WitValidationResult {
            is_valid,
            character_count: char_count,
            estimated_duration: Some(char_count as f32 * 0.05),
            warnings: if char_count > 4000 {
                vec!["Text is quite long, consider splitting for better performance".to_string()]
            } else {
                vec![]
            },
            errors: if !is_valid {
                vec!["Text must be between 1 and 5000 characters".to_string()]
            } else {
                vec![]
            },
        })
    }
}

impl TtsStreamingGuest for GoogleComponent {
    fn create_stream(_options: WitSynthesisOptions) -> Result<WitStreamSession, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation(
            "Google Cloud TTS streaming not yet implemented".to_string(),
        ))
    }

    fn stream_send_text(_session_id: String, _input: WitTextInput) -> Result<(), WitTtsError> {
        Err(WitTtsError::UnsupportedOperation(
            "Streaming not yet implemented".to_string(),
        ))
    }

    fn stream_finish(_session_id: String) -> Result<(), WitTtsError> {
        Err(WitTtsError::UnsupportedOperation(
            "Streaming not yet implemented".to_string(),
        ))
    }

    fn stream_receive_chunk(_session_id: String) -> Result<Option<WitAudioChunk>, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation(
            "Streaming not yet implemented".to_string(),
        ))
    }

    fn stream_has_pending(_session_id: String) -> Result<bool, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation(
            "Streaming not yet implemented".to_string(),
        ))
    }

    fn stream_get_status(_session_id: String) -> Result<WitStreamStatus, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation(
            "Streaming not yet implemented".to_string(),
        ))
    }

    fn stream_close(_session_id: String) -> Result<(), WitTtsError> {
        Err(WitTtsError::UnsupportedOperation(
            "Streaming not yet implemented".to_string(),
        ))
    }
}

impl TtsAdvancedGuest for GoogleComponent {
    fn create_voice_clone(
        _name: String,
        _audio_samples: Vec<WitAudioSample>,
        _description: Option<String>,
    ) -> Result<String, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation(
            "Google Cloud TTS does not support voice cloning via API".to_string(),
        ))
    }

    fn design_voice(
        _name: String,
        _characteristics: WitVoiceDesignParams,
    ) -> Result<String, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation(
            "Google Cloud TTS does not support voice design via API".to_string(),
        ))
    }

    fn convert_voice(
        _input_audio: Vec<u8>,
        _target_voice_id: String,
        _preserve_timing: Option<bool>,
    ) -> Result<Vec<u8>, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation(
            "Google Cloud TTS does not support voice conversion".to_string(),
        ))
    }

    fn generate_sound_effect(
        _description: String,
        _duration_seconds: Option<f32>,
        _style_influence: Option<f32>,
    ) -> Result<Vec<u8>, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation(
            "Google Cloud TTS does not support sound effect generation".to_string(),
        ))
    }

    fn create_lexicon(
        _name: String,
        _language: String,
        _entries: Option<Vec<WitPronunciationEntry>>,
    ) -> Result<String, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation(
            "Lexicon support not yet implemented".to_string(),
        ))
    }

    fn add_lexicon_entry(
        _lexicon_id: String,
        _entry: WitPronunciationEntry,
    ) -> Result<(), WitTtsError> {
        Err(WitTtsError::UnsupportedOperation(
            "Lexicon support not yet implemented".to_string(),
        ))
    }

    fn remove_lexicon_entry(_lexicon_id: String, _word: String) -> Result<(), WitTtsError> {
        Err(WitTtsError::UnsupportedOperation(
            "Lexicon support not yet implemented".to_string(),
        ))
    }

    fn export_lexicon(_lexicon_id: String) -> Result<String, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation(
            "Lexicon support not yet implemented".to_string(),
        ))
    }

    fn synthesize_long_form(
        _content: String,
        _voice_id: String,
        _output_location: String,
        _chapter_breaks: Option<Vec<u32>>,
    ) -> Result<WitLongFormJob, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation(
            "Long-form synthesis not yet implemented".to_string(),
        ))
    }

    fn get_long_form_status(_job_id: String) -> Result<WitLongFormResult, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation(
            "Long-form synthesis not yet implemented".to_string(),
        ))
    }

    fn cancel_long_form(_job_id: String) -> Result<(), WitTtsError> {
        Err(WitTtsError::UnsupportedOperation(
            "Long-form synthesis not yet implemented".to_string(),
        ))
    }
}

impl ExtendedGuest for GoogleComponent {}

type DurableGoogleComponent = DurableTts<GoogleComponent>;

golem_tts::export_tts!(DurableGoogleComponent with_types_in golem_tts);
