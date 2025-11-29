// Deepgram Aura TTS provider - matching PR #90 architecture
mod client;
mod conversions;

use client::{get_available_models, DeepgramTtsApi};
use conversions::*;
use golem_tts::config::with_config_key;
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
use golem_tts::golem::tts::types::{SynthesisResult, TextInput, TimingInfo, TtsError};
use golem_tts::golem::tts::voices::{Guest as VoicesGuest, LanguageInfo, VoiceFilter, VoiceInfo};
use log::{debug, info, trace};

struct DeepgramComponent;

impl DeepgramComponent {
    const API_KEY_ENV: &'static str = "DEEPGRAM_API_KEY";

    fn create_client() -> Result<DeepgramTtsApi, TtsError> {
        with_config_key(Self::API_KEY_ENV, Err, |api_key| {
            Ok(DeepgramTtsApi::new(api_key, "v1".to_string()))
        })
    }
}

impl VoicesGuest for DeepgramComponent {
    fn list_voices(_filter: Option<VoiceFilter>) -> Result<Vec<VoiceInfo>, TtsError> {
        debug!("Deepgram: Listing voices");
        let models = get_available_models();
        Ok(models
            .into_iter()
            .map(deepgram_model_to_voice_info)
            .collect())
    }

    fn get_voice(voice_id: String) -> Result<VoiceInfo, TtsError> {
        trace!("Deepgram: Getting voice {}", voice_id);
        let models = get_available_models();
        models
            .into_iter()
            .find(|m| m.voice_id == voice_id)
            .map(deepgram_model_to_voice_info)
            .ok_or_else(|| voice_not_found(voice_id))
    }

    fn search_voices(
        query: String,
        filter: Option<VoiceFilter>,
    ) -> Result<Vec<VoiceInfo>, TtsError> {
        debug!("Deepgram: Searching voices: {}", query);
        let models = get_available_models();
        let query_lower = query.to_lowercase();

        Ok(models
            .into_iter()
            .filter(|m| {
                // Search in name, voice_id, characteristics, or use_cases
                m.name.to_lowercase().contains(&query_lower)
                    || m.voice_id.to_lowercase().contains(&query_lower)
                    || m.characteristics
                        .iter()
                        .any(|c| c.to_lowercase().contains(&query_lower))
                    || m.use_cases
                        .iter()
                        .any(|u| u.to_lowercase().contains(&query_lower))
                    || m.accent.to_lowercase().contains(&query_lower)
            })
            .filter(|m| {
                // Apply optional filters
                if let Some(ref f) = filter {
                    if let Some(ref lang) = f.language {
                        if !m.language.starts_with(lang) {
                            return false;
                        }
                    }
                    if let Some(gender) = f.gender {
                        let model_gender = parse_gender(&m.gender);
                        if model_gender != gender {
                            return false;
                        }
                    }
                }
                true
            })
            .map(deepgram_model_to_voice_info)
            .collect())
    }

    fn list_languages() -> Result<Vec<LanguageInfo>, TtsError> {
        Ok(vec![LanguageInfo {
            code: "en".to_string(),
            name: "English".to_string(),
            native_name: "English".to_string(),
            voice_count: 12,
        }])
    }
}

impl SynthesisGuest for DeepgramComponent {
    fn synthesize(
        input: TextInput,
        options: SynthesisOptions,
    ) -> Result<SynthesisResult, TtsError> {
        info!("Deepgram: Synthesizing {} chars", input.content.len());

        if input.content.is_empty() {
            return Err(invalid_text("Text cannot be empty"));
        }

        let client = Self::create_client()?;
        let (request, params) =
            synthesis_options_to_tts_request(input.content.clone(), Some(options))?;
        let response = client.text_to_speech_with_metadata(&request, params.as_ref())?;

        // Convert to SynthesisResult with metadata
        let encoding = params
            .as_ref()
            .and_then(|p| p.encoding.clone())
            .unwrap_or_else(|| "linear16".to_string());
        let sample_rate = params.as_ref().and_then(|p| p.sample_rate).unwrap_or(24000);

        Ok(audio_data_to_synthesis_result(
            response.audio_data,
            &input.content,
            &encoding,
            sample_rate,
        ))
    }

    fn synthesize_batch(
        inputs: Vec<TextInput>,
        options: SynthesisOptions,
    ) -> Result<Vec<SynthesisResult>, TtsError> {
        info!("Deepgram: Batch synthesizing {} inputs", inputs.len());
        inputs
            .into_iter()
            .map(|input| Self::synthesize(input, options.clone()))
            .collect()
    }

    fn get_timing_marks(_input: TextInput, _voice_id: String) -> Result<Vec<TimingInfo>, TtsError> {
        Err(unsupported("Deepgram does not support timing marks"))
    }

    fn validate_input(input: TextInput, _voice_id: String) -> Result<ValidationResult, TtsError> {
        let char_count = input.content.len() as u32;
        let is_valid = char_count > 0 && char_count <= 2000;

        Ok(ValidationResult {
            is_valid,
            character_count: char_count,
            estimated_duration: Some(char_count as f32 * 0.05),
            warnings: if char_count > 1500 {
                vec!["Text is approaching Deepgram's limit".to_string()]
            } else {
                vec![]
            },
            errors: if !is_valid {
                vec!["Text must be between 1 and 2000 characters".to_string()]
            } else {
                vec![]
            },
        })
    }
}

impl StreamingGuest for DeepgramComponent {
    fn create_stream(_options: SynthesisOptions) -> Result<StreamSession, TtsError> {
        Err(unsupported("Deepgram streaming not yet implemented"))
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

impl AdvancedGuest for DeepgramComponent {
    fn create_voice_clone(
        _name: String,
        _audio_samples: Vec<AudioSample>,
        _description: Option<String>,
    ) -> Result<String, TtsError> {
        Err(unsupported("Deepgram does not support voice cloning"))
    }

    fn design_voice(
        _name: String,
        _characteristics: VoiceDesignParams,
    ) -> Result<String, TtsError> {
        Err(unsupported("Deepgram does not support voice design"))
    }

    fn convert_voice(
        _input_audio: Vec<u8>,
        _target_voice_id: String,
        _preserve_timing: Option<bool>,
    ) -> Result<Vec<u8>, TtsError> {
        Err(unsupported("Deepgram does not support voice conversion"))
    }

    fn generate_sound_effect(
        _description: String,
        _duration_seconds: Option<f32>,
        _style_influence: Option<f32>,
    ) -> Result<Vec<u8>, TtsError> {
        Err(unsupported("Deepgram does not support sound effects"))
    }

    fn create_lexicon(
        _name: String,
        _language: String,
        _entries: Option<Vec<PronunciationEntry>>,
    ) -> Result<String, TtsError> {
        Err(unsupported("Deepgram does not support lexicons"))
    }

    fn add_lexicon_entry(_lexicon_id: String, _entry: PronunciationEntry) -> Result<(), TtsError> {
        Err(unsupported("Lexicon not supported"))
    }

    fn remove_lexicon_entry(_lexicon_id: String, _word: String) -> Result<(), TtsError> {
        Err(unsupported("Lexicon not supported"))
    }

    fn export_lexicon(_lexicon_id: String) -> Result<String, TtsError> {
        Err(unsupported("Lexicon not supported"))
    }

    fn synthesize_long_form(
        _content: String,
        _voice_id: String,
        _output_location: String,
        _chapter_breaks: Option<Vec<u32>>,
    ) -> Result<LongFormJob, TtsError> {
        Err(unsupported("Deepgram does not support long-form synthesis"))
    }

    fn get_long_form_status(_job_id: String) -> Result<LongFormResult, TtsError> {
        Err(unsupported("Long-form not supported"))
    }

    fn cancel_long_form(_job_id: String) -> Result<(), TtsError> {
        Err(unsupported("Long-form not supported"))
    }
}

impl ExtendedGuest for DeepgramComponent {}

type DurableDeepgramComponent = DurableTts<DeepgramComponent>;

golem_tts::export_tts!(DurableDeepgramComponent with_types_in golem_tts);
