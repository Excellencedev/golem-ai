use golem_tts::{
    config::with_config_key,
    durability::{DurableTts, ExtendedGuest},
    guest::{TtsAdvancedGuest, TtsSynthesisGuest, TtsStreamingGuest, TtsVoicesGuest},
    LOGGING_STATE,
};
use golem_tts::golem::tts::types::{
    TextInput as WitTextInput, TimingInfo as WitTimingInfo, SynthesisResult as WitSynthesisResult,
    AudioChunk as WitAudioChunk, TtsError as WitTtsError, SynthesisMetadata, VoiceGender, VoiceQuality,
};
use golem_tts::exports::golem::tts::voices::{
    LanguageInfo as WitLanguageInfo, VoiceFilter as WitVoiceFilter, VoiceInfo as WitVoiceInfo,
};
use golem_tts::exports::golem::tts::synthesis::{
    SynthesisOptions as WitSynthesisOptions, ValidationResult as WitValidationResult,
};
use golem_tts::exports::golem::tts::streaming::{
    StreamSession as WitStreamSession, StreamStatus as WitStreamStatus,
};
use golem_tts::exports::golem::t ts::advanced::{
    AudioSample as WitAudioSample, VoiceDesignParams as WitVoiceDesignParams,
    PronunciationEntry as WitPronunciationEntry, LongFormJob as WitLongFormJob,
    LongFormResult as WitLongFormResult,
};

struct PollyComponent;

impl TtsVoicesGuest for PollyComponent {
    fn list_voices(_filter: Option<WitVoiceFilter>) -> Result<Vec<WitVoiceInfo>, WitTtsError> {
        // Stub: Return common Polly voices
        Ok(vec![
            WitVoiceInfo {
                id: "Joanna".to_string(),
                name: "Joanna".to_string(),
                language: "en-US".to_string(),
                additional_languages: vec![],
                gender: VoiceGender::Female,
                quality: VoiceQuality::Neural,
                description: Some("US English female voice".to_string()),
                provider: "AWS Polly".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["general".to_string()],
            },
            WitVoiceInfo {
                id: "Matthew".to_string(),
                name: "Matthew".to_string(),
                language: "en-US".to_string(),
                additional_languages: vec![],
                gender: VoiceGender::Male,
                quality: VoiceQuality::Neural,
                description: Some("US English male voice".to_string()),
                provider: "AWS Polly".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["general".to_string()],
            },
        ])
    }

    fn get_voice(voice_id: String) -> Result<WitVoiceInfo, WitTtsError> {
        let voices = Self::list_voices(None)?;
        voices.into_iter()
            .find(|v| v.id == voice_id)
            .ok_or_else(|| WitTtsError::VoiceNotFound(voice_id))
    }

    fn search_voices(query: String, filter: Option<WitVoiceFilter>) -> Result<Vec<WitVoiceInfo>, WitTtsError> {
        let all_voices = Self::list_voices(filter)?;
        let query_lower = query.to_lowercase();
        Ok(all_voices.into_iter().filter(|v| v.name.to_lowercase().contains(&query_lower)).collect())
    }

    fn list_languages() -> Result<Vec<WitLanguageInfo>, WitTtsError> {
        Ok(vec![
            WitLanguageInfo {
                code: "en-US".to_string(),
                name: "English (US)".to_string(),
                native_name: "English (US)".to_string(),
                voice_count: 10,
            },
        ])
    }
}

impl TtsSynthesisGuest for PollyComponent {
    fn synthesize(input: WitTextInput, options: WitSynthesisOptions) -> Result<WitSynthesisResult, WitTtsError> {
        // Stub implementation
        let char_count = input.content.len() as u32;
        Ok(WitSynthesisResult {
            audio_data: vec![], // Would contain actual MP3 data from Polly
            metadata: SynthesisMetadata {
                duration_seconds: char_count as f32 * 0.05,
                character_count: char_count,
                word_count: input.content.split_whitespace().count() as u32,
                audio_size_bytes: 0,
                request_id: uuid::Uuid::new_v4().to_string(),
                provider_info: Some("AWS Polly (stub)".to_string()),
            },
        })
    }

    fn synthesize_batch(inputs: Vec<WitTextInput>, options: WitSynthesisOptions) -> Result<Vec<WitSynthesisResult>, WitTtsError> {
        inputs.into_iter().map(|input| Self::synthesize(input, options.clone())).collect()
    }

    fn get_timing_marks(_input: WitTextInput, _voice_id: String) -> Result<Vec<WitTimingInfo>, WitTtsError> {
        Ok(vec![])
    }

    fn validate_input(input: WitTextInput, _voice_id: String) -> Result<WitValidationResult, WitTtsError> {
        let char_count = input.content.len() as u32;
        Ok(WitValidationResult {
            is_valid: char_count > 0 && char_count <= 3000,
            character_count: char_count,
            estimated_duration: Some(char_count as f32 * 0.05),
            warnings: vec![],
            errors: vec![],
        })
    }
}

impl TtsStreamingGuest for PollyComponent {
    fn create_stream(_options: WitSynthesisOptions) -> Result<WitStreamSession, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("AWS Polly does not have native streaming support".to_string()))
    }

    fn stream_send_text(_session_id: String, _input: WitTextInput) -> Result<(), WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("AWS Polly does not have native streaming support".to_string()))
    }

    fn stream_finish(_session_id: String) -> Result<(), WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("AWS Polly does not have native streaming support".to_string()))
    }

    fn stream_receive_chunk(_session_id: String) -> Result<Option<WitAudioChunk>, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("AWS Polly does not have native streaming support".to_string()))
    }

    fn stream_has_pending(_session_id: String) -> Result<bool, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("AWS Polly does not have native streaming support".to_string()))
    }

    fn stream_get_status(_session_id: String) -> Result<WitStreamStatus, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("AWS Polly does not have native streaming support".to_string()))
    }

    fn stream_close(_session_id: String) -> Result<(), WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("AWS Polly does not have native streaming support".to_string()))
    }
}

impl TtsAdvancedGuest for PollyComponent {
    fn create_voice_clone(_name: String, _audio_samples: Vec<WitAudioSample>, _description: Option<String>) -> Result<String, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("AWS Polly does not support voice cloning".to_string()))
    }

    fn design_voice(_name: String, _characteristics: WitVoiceDesignParams) -> Result<String, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("AWS Polly does not support voice design".to_string()))
    }

    fn convert_voice(_input_audio: Vec<u8>, _target_voice_id: String, _preserve_timing: Option<bool>) -> Result<Vec<u8>, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("AWS Polly does not support voice conversion".to_string()))
    }

    fn generate_sound_effect(_description: String, _duration_seconds: Option<f32>, _style_influence: Option<f32>) -> Result<Vec<u8>, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("AWS Polly does not support sound effect generation".to_string()))
    }

    fn create_lexicon(_name: String, _language: String, _entries: Option<Vec<WitPronunciationEntry>>) -> Result<String, WitTtsError> {
        // Polly supports lexicons but marking as stub for now
        Err(WitTtsError::UnsupportedOperation("Lexicon support not yet implemented".to_string()))
    }

    fn add_lexicon_entry(_lexicon_id: String, _entry: WitPronunciationEntry) -> Result<(), WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("Lexicon support not yet implemented".to_string()))
    }

    fn remove_lexicon_entry(_lexicon_id: String, _word: String) -> Result<(), WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("Lexicon support not yet implemented".to_string()))
    }

    fn export_lexicon(_lexicon_id: String) -> Result<String, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("Lexicon support not yet implemented".to_string()))
    }

    fn synthesize_long_form(_content: String, _voice_id: String, _output_location: String, _chapter_breaks: Option<Vec<u32>>) -> Result<WitLongFormJob, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("Long-form synthesis not yet implemented".to_string()))
    }

    fn get_long_form_status(_job_id: String) -> Result<WitLongFormResult, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("Long-form synthesis not yet implemented".to_string()))
    }

    fn cancel_long_form(_job_id: String) -> Result<(), WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("Long-form synthesis not yet implemented".to_string()))
    }
}

impl ExtendedGuest for PollyComponent {}

type DurablePollyComponent = DurableTts<PollyComponent>;

golem_tts::export_tts!(DurablePollyComponent with_types_in golem_tts);
