mod client;
mod conversions;

use client::ElevenLabsClient;
use conversions::*;
use golem_tts::{
    config::with_config_key,
    durability::{DurableTts, ExtendedGuest},
    guest::{TtsAdvancedGuest, TtsSynthesisGuest, TtsStreamingGuest, TtsVoicesGuest},
    LOGGING_STATE,
};
use golem_tts::golem::tts::types::{
    TextInput as WitTextInput, TimingInfo as WitTimingInfo, SynthesisResult as WitSynthesisResult,
    AudioChunk as WitAudioChunk, TtsError as WitTtsError,
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
use golem_tts::exports::golem::tts::advanced::{
    AudioSample as WitAudioSample, VoiceDesignParams as WitVoiceDesignParams,
    PronunciationEntry as WitPronunciationEntry, LongFormJob as WitLongFormJob,
    LongFormResult as WitLongFormResult,
};

struct ElevenLabsComponent;

impl ElevenLabsComponent {
    const ENV_VAR_NAME: &'static str = "ELEVENLABS_API_KEY";
}

impl TtsVoicesGuest for ElevenLabsComponent {
    fn list_voices(filter: Option<WitVoiceFilter>) -> Result<Vec<WitVoiceInfo>, WitTtsError> {
        with_config_key(Self::ENV_VAR_NAME, Err, |api_key| {
            let client = ElevenLabsClient::new(api_key);
            client.list_voices(filter)
        })
    }

    fn get_voice(voice_id: String) -> Result<WitVoiceInfo, WitTtsError> {
        with_config_key(Self::ENV_VAR_NAME, Err, |api_key| {
            let client = ElevenLabsClient::new(api_key);
            client.get_voice(voice_id)
        })
    }

    fn search_voices(query: String, filter: Option<WitVoiceFilter>) -> Result<Vec<WitVoiceInfo>, WitTtsError> {
        with_config_key(Self::ENV_VAR_NAME, Err, |api_key| {
            let client = ElevenLabsClient::new(api_key);
            client.search_voices(query, filter)
        })
    }

    fn list_languages() -> Result<Vec<WitLanguageInfo>, WitTtsError> {
        with_config_key(Self::ENV_VAR_NAME, Err, |api_key| {
            let client = ElevenLabsClient::new(api_key);
            client.list_languages()
        })
    }
}

impl TtsSynthesisGuest for ElevenLabsComponent {
    fn synthesize(input: WitTextInput, options: WitSynthesisOptions) -> Result<WitSynthesisResult, WitTtsError> {
        with_config_key(Self::ENV_VAR_NAME, Err, |api_key| {
            let client = ElevenLabsClient::new(api_key);
            client.synthesize(input, options)
        })
    }

    fn synthesize_batch(inputs: Vec<WitTextInput>, options: WitSynthesisOptions) -> Result<Vec<WitSynthesisResult>, WitTtsError> {
        with_config_key(Self::ENV_VAR_NAME, Err, |api_key| {
            let client = ElevenLabsClient::new(api_key);
            client.synthesize_batch(inputs, options)
        })
    }

    fn get_timing_marks(_input: WitTextInput, _voice_id: String) -> Result<Vec<WitTimingInfo>, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("ElevenLabs does not provide timing marks separately".to_string()))
    }

    fn validate_input(input: WitTextInput, _voice_id: String) -> Result<WitValidationResult, WitTtsError> {
        let char_count = input.content.len() as u32;
        let is_valid = char_count > 0 && char_count <= 5000;
        
        Ok(WitValidationResult {
            is_valid,
            character_count: char_count,
            estimated_duration: Some(char_count as f32 * 0.05),
            warnings: if char_count > 3000 {
                vec!["Text is quite long, consider splitting".to_string()]
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

impl TtsStreamingGuest for ElevenLabsComponent {
    fn create_stream(options: WitSynthesisOptions) -> Result<WitStreamSession, WitTtsError> {
        with_config_key(Self::ENV_VAR_NAME, Err, |api_key| {
            let client = ElevenLabsClient::new(api_key);
            client.create_stream(options)
        })
    }

    fn stream_send_text(session_id: String, input: WitTextInput) -> Result<(), WitTtsError> {
        with_config_key(Self::ENV_VAR_NAME, Err, |api_key| {
            let client = ElevenLabsClient::new(api_key);
            client.stream_send_text(session_id, input)
        })
    }

    fn stream_finish(session_id: String) -> Result<(), WitTtsError> {
        with_config_key(Self::ENV_VAR_NAME, Err, |api_key| {
            let client = ElevenLabsClient::new(api_key);
            client.stream_finish(session_id)
        })
    }

    fn stream_receive_chunk(session_id: String) -> Result<Option<WitAudioChunk>, WitTtsError> {
        with_config_key(Self::ENV_VAR_NAME, Err, |api_key| {
            let client = ElevenLabsClient::new(api_key);
            client.stream_receive_chunk(session_id)
        })
    }

    fn stream_has_pending(session_id: String) -> Result<bool, WitTtsError> {
        with_config_key(Self::ENV_VAR_NAME, Err, |api_key| {
            let client = ElevenLabsClient::new(api_key);
            client.stream_has_pending(session_id)
        })
    }

    fn stream_get_status(session_id: String) -> Result<WitStreamStatus, WitTtsError> {
        with_config_key(Self::ENV_VAR_NAME, Err, |api_key| {
            let client = ElevenLabsClient::new(api_key);
            client.stream_get_status(session_id)
        })
    }

    fn stream_close(session_id: String) -> Result<(), WitTtsError> {
        with_config_key(Self::ENV_VAR_NAME, Err, |api_key| {
            let client = ElevenLabsClient::new(api_key);
            client.stream_close(session_id)
        })
    }
}

impl TtsAdvancedGuest for ElevenLabsComponent {
    fn create_voice_clone(name: String, audio_samples: Vec<WitAudioSample>, description: Option<String>) -> Result<String, WitTtsError> {
        with_config_key(Self::ENV_VAR_NAME, Err, |api_key| {
            let client = ElevenLabsClient::new(api_key);
            client.create_voice_clone(name, audio_samples, description)
        })
    }

    fn design_voice(_name: String, _characteristics: WitVoiceDesignParams) -> Result<String, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("ElevenLabs does not support voice design".to_string()))
    }

    fn convert_voice(input_audio: Vec<u8>, target_voice_id: String, preserve_timing: Option<bool>) -> Result<Vec<u8>, WitTtsError> {
        with_config_key(Self::ENV_VAR_NAME, Err, |api_key| {
            let client = ElevenLabsClient::new(api_key);
            client.convert_voice(input_audio, target_voice_id, preserve_timing)
        })
    }

    fn generate_sound_effect(description: String, duration_seconds: Option<f32>, style_influence: Option<f32>) -> Result<Vec<u8>, WitTtsError> {
        with_config_key(Self::ENV_VAR_NAME, Err, |api_key| {
            let client = ElevenLabsClient::new(api_key);
            client.generate_sound_effect(description, duration_seconds, style_influence)
        })
    }

    fn create_lexicon(_name: String, _language: String, _entries: Option<Vec<WitPronunciationEntry>>) -> Result<String, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("ElevenLabs does not support custom lexicons".to_string()))
    }

    fn add_lexicon_entry(_lexicon_id: String, _entry: WitPronunciationEntry) -> Result<(), WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("ElevenLabs does not support custom lexicons".to_string()))
    }

    fn remove_lexicon_entry(_lexicon_id: String, _word: String) -> Result<(), WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("ElevenLabs does not support custom lexicons".to_string()))
    }

    fn export_lexicon(_lexicon_id: String) -> Result<String, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("ElevenLabs does not support custom lexicons".to_string()))
    }

    fn synthesize_long_form(_content: String, _voice_id: String, _output_location: String, _chapter_breaks: Option<Vec<u32>>) -> Result<WitLongFormJob, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("ElevenLabs long-form synthesis not yet implemented".to_string()))
    }

    fn get_long_form_status(_job_id: String) -> Result<WitLongFormResult, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("ElevenLabs long-form synthesis not yet implemented".to_string()))
    }

    fn cancel_long_form(_job_id: String) -> Result<(), WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("ElevenLabs long-form synthesis not yet implemented".to_string()))
    }
}

impl ExtendedGuest for ElevenLabsComponent {}

type DurableElevenLabsComponent = DurableTts<ElevenLabsComponent>;

golem_tts::export_tts!(DurableElevenLabsComponent with_types_in golem_tts);
