// AWS Polly TTS provider
mod client;
mod conversions;

use client::{PollyClient, PollyVoice};
use conversions::*;
use golem_tts::durability::{DurableTts, ExtendedGuest};
use golem_tts::error::{invalid_text, unsupported, voice_not_found};
use golem_tts::golem::tts::advanced::{
    AudioSample, Guest as AdvancedGuest, LongFormJob, LongFormResult, PronunciationEntry,
    VoiceDesignParams,
};
use golem_tts::golem::tts::streaming::{Guest as StreamingGuest, StreamSession, StreamStatus};
use golem_tts::golem::tts::synthesis::{
    Guest as SynthesisGuest, SynthesisOptions, ValidationResult,
};
use golem_tts::golem::tts::types::{
    SynthesisResult, TextInput, TimingInfo, TtsError, VoiceQuality,
};
use golem_tts::golem::tts::voices::{Guest as VoicesGuest, LanguageInfo, VoiceFilter, VoiceInfo};
use log::{debug, info, trace};

struct PollyComponent;

impl PollyComponent {
    fn create_client() -> Result<PollyClient, TtsError> {
        PollyClient::new()
    }

    fn voice_to_info(voice: &PollyVoice) -> VoiceInfo {
        VoiceInfo {
            id: voice.id.clone(),
            name: voice.name.clone(),
            language: voice.language_code.clone(),
            additional_languages: vec![],
            gender: parse_gender(&voice.gender),
            quality: VoiceQuality::Neural,
            description: Some(format!("{} voice", voice.gender)),
            provider: "AWS Polly".to_string(),
            sample_rate: 24000,
            is_custom: false,
            is_cloned: false,
            preview_url: None,
            use_cases: vec!["general".to_string()],
        }
    }
}

impl VoicesGuest for PollyComponent {
    fn list_voices(_filter: Option<VoiceFilter>) -> Result<Vec<VoiceInfo>, TtsError> {
        debug!("Polly: Listing voices");
        let voices = PollyClient::list_voices();
        Ok(voices.iter().map(|v| Self::voice_to_info(v)).collect())
    }

    fn get_voice(voice_id: String) -> Result<VoiceInfo, TtsError> {
        trace!("Polly: Getting voice {}", voice_id);
        let voices = PollyClient::list_voices();
        voices
            .iter()
            .find(|v| v.id == voice_id)
            .map(|v| Self::voice_to_info(v))
            .ok_or_else(|| voice_not_found(voice_id))
    }

    fn search_voices(
        query: String,
        _filter: Option<VoiceFilter>,
    ) -> Result<Vec<VoiceInfo>, TtsError> {
        debug!("Polly: Searching voices: {}", query);
        let voices = PollyClient::list_voices();
        let query_lower = query.to_lowercase();
        Ok(voices
            .iter()
            .filter(|v| v.name.to_lowercase().contains(&query_lower))
            .map(|v| Self::voice_to_info(v))
            .collect())
    }

    fn list_languages() -> Result<Vec<LanguageInfo>, TtsError> {
        Ok(vec![LanguageInfo {
            code: "en-US".to_string(),
            name: "English (US)".to_string(),
            native_name: "English".to_string(),
            voice_count: 4,
        }])
    }
}

impl SynthesisGuest for PollyComponent {
    fn synthesize(
        input: TextInput,
        options: SynthesisOptions,
    ) -> Result<SynthesisResult, TtsError> {
        info!("Polly: Synthesizing {} chars", input.content.len());

        if input.content.is_empty() {
            return Err(invalid_text("Text cannot be empty"));
        }

        let client = Self::create_client()?;
        let format = audio_format_to_polly(options.audio_config.format);
        let audio_data = client.synthesize_speech(&input.content, &options.voice_id, format)?;

        Ok(SynthesisResult {
            audio_data,
            metadata: None,
        })
    }

    fn synthesize_batch(
        inputs: Vec<TextInput>,
        options: SynthesisOptions,
    ) -> Result<Vec<SynthesisResult>, TtsError> {
        info!("Polly: Batch synthesizing {} inputs", inputs.len());
        inputs
            .into_iter()
            .map(|input| Self::synthesize(input, options.clone()))
            .collect()
    }

    fn get_timing_marks(_input: TextInput, _voice_id: String) -> Result<Vec<TimingInfo>, TtsError> {
        Err(unsupported("Polly timing marks require speech marks API"))
    }

    fn validate_input(input: TextInput, _voice_id: String) -> Result<ValidationResult, TtsError> {
        let char_count = input.content.len() as u32;
        let is_valid = char_count > 0 && char_count <= 3000;

        Ok(ValidationResult {
            is_valid,
            character_count: char_count,
            estimated_duration: Some(char_count as f32 * 0.05),
            warnings: if char_count > 2500 {
                vec!["Text approaching limit".to_string()]
            } else {
                vec![]
            },
            errors: if !is_valid {
                vec!["Text must be 1-3000 characters".to_string()]
            } else {
                vec![]
            },
        })
    }
}

impl StreamingGuest for PollyComponent {
    fn create_stream(_options: SynthesisOptions) -> Result<StreamSession, TtsError> {
        Err(unsupported("Polly streaming not supported"))
    }

    fn stream_send_text(_session_id: String, _input: TextInput) -> Result<(), TtsError> {
        Err(unsupported("Streaming not supported"))
    }

    fn stream_finish(_session_id: String) -> Result<(), TtsError> {
        Err(unsupported("Streaming not supported"))
    }

    fn stream_receive_chunk(_session_id: String) -> Result<Option<Vec<u8>>, TtsError> {
        Err(unsupported("Streaming not supported"))
    }

    fn stream_has_pending(_session_id: String) -> Result<bool, TtsError> {
        Err(unsupported("Streaming not supported"))
    }

    fn stream_get_status(_session_id: String) -> Result<StreamStatus, TtsError> {
        Err(unsupported("Streaming not supported"))
    }

    fn stream_close(_session_id: String) -> Result<(), TtsError> {
        Err(unsupported("Streaming not supported"))
    }
}

impl AdvancedGuest for PollyComponent {
    fn create_voice_clone(
        _name: String,
        _audio_samples: Vec<AudioSample>,
        _description: Option<String>,
    ) -> Result<String, TtsError> {
        Err(unsupported("Polly does not support voice cloning"))
    }

    fn design_voice(
        _name: String,
        _characteristics: VoiceDesignParams,
    ) -> Result<String, TtsError> {
        Err(unsupported("Polly does not support voice design"))
    }

    fn convert_voice(
        _input_audio: Vec<u8>,
        _target_voice_id: String,
        _preserve_timing: Option<bool>,
    ) -> Result<Vec<u8>, TtsError> {
        Err(unsupported("Voice conversion not supported"))
    }

    fn generate_sound_effect(
        _description: String,
        _duration_seconds: Option<f32>,
        _style_influence: Option<f32>,
    ) -> Result<Vec<u8>, TtsError> {
        Err(unsupported("Polly does not support sound effects"))
    }

    fn create_lexicon(
        _name: String,
        _language: String,
        _entries: Option<Vec<PronunciationEntry>>,
    ) -> Result<String, TtsError> {
        Err(unsupported("Lexicon management requires separate API"))
    }

    fn add_lexicon_entry(_lexicon_id: String, _entry: PronunciationEntry) -> Result<(), TtsError> {
        Err(unsupported("Lexicon not implemented"))
    }

    fn remove_lexicon_entry(_lexicon_id: String, _word: String) -> Result<(), TtsError> {
        Err(unsupported("Lexicon not implemented"))
    }

    fn export_lexicon(_lexicon_id: String) -> Result<String, TtsError> {
        Err(unsupported("Lexicon not implemented"))
    }

    fn synthesize_long_form(
        _content: String,
        _voice_id: String,
        _output_location: String,
        _chapter_breaks: Option<Vec<u32>>,
    ) -> Result<LongFormJob, TtsError> {
        Err(unsupported("Long-form synthesis not yet implemented"))
    }

    fn get_long_form_status(_job_id: String) -> Result<LongFormResult, TtsError> {
        Err(unsupported("Long-form not supported"))
    }

    fn cancel_long_form(_job_id: String) -> Result<(), TtsError> {
        Err(unsupported("Long-form not supported"))
    }
}

impl ExtendedGuest for PollyComponent {}

type DurablePollyComponent = DurableTts<PollyComponent>;

golem_tts::export_tts!(DurablePollyComponent with_types_in golem_tts);
