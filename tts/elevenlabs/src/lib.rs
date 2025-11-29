// ElevenLabs TTS provider
mod client;
mod conversions;

use client::{ElevenLabsClient, Voice};
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
    SynthesisResult, TextInput, TimingInfo, TtsError, VoiceGender, VoiceQuality,
};
use golem_tts::golem::tts::voices::{Guest as VoicesGuest, LanguageInfo, VoiceFilter, VoiceInfo};
use log::{debug, info, trace};

struct ElevenLabsComponent;

impl ElevenLabsComponent {
    fn create_client() -> Result<ElevenLabsClient, TtsError> {
        ElevenLabsClient::new()
    }

    fn voice_to_info(voice: &Voice) -> VoiceInfo {
        VoiceInfo {
            id: voice.voice_id.clone(),
            name: voice.name.clone(),
            language: "en".to_string(),
            additional_languages: vec![],
            gender: voice
                .description
                .as_ref()
                .map(|d| parse_gender(d))
                .unwrap_or(VoiceGender::Neutral),
            quality: VoiceQuality::Neural,
            description: voice.description.clone(),
            provider: "ElevenLabs".to_string(),
            sample_rate: 44100,
            is_custom: false,
            is_cloned: false,
            preview_url: voice.preview_url.clone(),
            use_cases: vec!["general".to_string()],
        }
    }
}

impl VoicesGuest for ElevenLabsComponent {
    fn list_voices(_filter: Option<VoiceFilter>) -> Result<Vec<VoiceInfo>, TtsError> {
        debug!("ElevenLabs: Listing voices");
        let client = Self::create_client()?;
        let voices = client.list_voices()?;
        Ok(voices.iter().map(|v| Self::voice_to_info(v)).collect())
    }

    fn get_voice(voice_id: String) -> Result<VoiceInfo, TtsError> {
        trace!("ElevenLabs: Getting voice {}", voice_id);
        let client = Self::create_client()?;
        let voices = client.list_voices()?;
        voices
            .iter()
            .find(|v| v.voice_id == voice_id)
            .map(|v| Self::voice_to_info(v))
            .ok_or_else(|| voice_not_found(voice_id))
    }

    fn search_voices(
        query: String,
        _filter: Option<VoiceFilter>,
    ) -> Result<Vec<VoiceInfo>, TtsError> {
        debug!("ElevenLabs: Searching voices: {}", query);
        let client = Self::create_client()?;
        let voices = client.list_voices()?;
        let query_lower = query.to_lowercase();
        Ok(voices
            .iter()
            .filter(|v| v.name.to_lowercase().contains(&query_lower))
            .map(|v| Self::voice_to_info(v))
            .collect())
    }

    fn list_languages() -> Result<Vec<LanguageInfo>, TtsError> {
        Ok(vec![LanguageInfo {
            code: "en".to_string(),
            name: "English".to_string(),
            native_name: "English".to_string(),
            voice_count: 30,
        }])
    }
}

impl SynthesisGuest for ElevenLabsComponent {
    fn synthesize(
        input: TextInput,
        options: SynthesisOptions,
    ) -> Result<SynthesisResult, TtsError> {
        info!("ElevenLabs: Synthesizing {} chars", input.content.len());

        if input.content.is_empty() {
            return Err(invalid_text("Text cannot be empty"));
        }

        let client = Self::create_client()?;
        let response = client.text_to_speech(&input.content, &options.voice_id)?;

        Ok(SynthesisResult {
            audio_data: response.audio_data,
            metadata: Some(response.metadata),
        })
    }

    fn synthesize_batch(
        inputs: Vec<TextInput>,
        options: SynthesisOptions,
    ) -> Result<Vec<SynthesisResult>, TtsError> {
        info!("ElevenLabs: Batch synthesizing {} inputs", inputs.len());
        inputs
            .into_iter()
            .map(|input| Self::synthesize(input, options.clone()))
            .collect()
    }

    fn get_timing_marks(_input: TextInput, _voice_id: String) -> Result<Vec<TimingInfo>, TtsError> {
        Err(unsupported("ElevenLabs does not support timing marks"))
    }

    fn validate_input(input: TextInput, _voice_id: String) -> Result<ValidationResult, TtsError> {
        let char_count = input.content.len() as u32;
        let is_valid = char_count > 0 && char_count <= 5000;

        Ok(ValidationResult {
            is_valid,
            character_count: char_count,
            estimated_duration: Some(char_count as f32 * 0.05),
            warnings: if char_count > 4000 {
                vec!["Text approaching limit".to_string()]
            } else {
                vec![]
            },
            errors: if !is_valid {
                vec!["Text must be 1-5000 characters".to_string()]
            } else {
                vec![]
            },
        })
    }
}

impl StreamingGuest for ElevenLabsComponent {
    fn create_stream(_options: SynthesisOptions) -> Result<StreamSession, TtsError> {
        Err(unsupported("ElevenLabs streaming not yet implemented"))
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

impl AdvancedGuest for ElevenLabsComponent {
    fn create_voice_clone(
        _name: String,
        _audio_samples: Vec<AudioSample>,
        _description: Option<String>,
    ) -> Result<String, TtsError> {
        Err(unsupported(
            "Voice cloning requires multipart upload - not supported in WASI",
        ))
    }

    fn design_voice(
        _name: String,
        _characteristics: VoiceDesignParams,
    ) -> Result<String, TtsError> {
        Err(unsupported("ElevenLabs does not support voice design"))
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
        Err(unsupported(
            "Sound effects require separate API - not yet implemented",
        ))
    }

    fn create_lexicon(
        _name: String,
        _language: String,
        _entries: Option<Vec<PronunciationEntry>>,
    ) -> Result<String, TtsError> {
        Err(unsupported("ElevenLabs does not support lexicons"))
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
        Err(unsupported("Long-form synthesis not yet implemented"))
    }

    fn get_long_form_status(_job_id: String) -> Result<LongFormResult, TtsError> {
        Err(unsupported("Long-form not supported"))
    }

    fn cancel_long_form(_job_id: String) -> Result<(), TtsError> {
        Err(unsupported("Long-form not supported"))
    }
}

impl ExtendedGuest for ElevenLabsComponent {}

type DurableElevenLabsComponent = DurableTts<ElevenLabsComponent>;

golem_tts::export_tts!(DurableElevenLabsComponent with_types_in golem_tts);
