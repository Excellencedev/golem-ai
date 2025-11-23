use crate::exports::golem::tts::advanced::{
    AudioSample as WitAudioSample, Guest as WitAdvancedGuest, LongFormJob as WitLongFormJob,
    LongFormResult as WitLongFormResult, PronunciationEntry as WitPronunciationEntry,
    VoiceDesignParams as WitVoiceDesignParams,
};
use crate::exports::golem::tts::streaming::{
    Guest as WitStreamingGuest, StreamSession as WitStreamSession, StreamStatus as WitStreamStatus,
};
use crate::exports::golem::tts::synthesis::{
    Guest as WitSynthesisGuest, SynthesisContext as WitSynthesisContext,
    SynthesisOptions as WitSynthesisOptions, ValidationResult as WitValidationResult,
};
use crate::exports::golem::tts::types::TtsError as WitTtsError;
use crate::exports::golem::tts::voices::{
    Guest as WitVoicesGuest, LanguageInfo as WitLanguageInfo, VoiceFilter as WitVoiceFilter,
    VoiceInfo as WitVoiceInfo,
};
use crate::golem::tts::types::{
    AudioChunk as WitAudioChunk, SynthesisResult as WitSynthesisResult, TextInput as WitTextInput,
    TimingInfo as WitTimingInfo,
};

/// Trait for voice management operations
pub trait TtsVoicesGuest {
    fn list_voices(filter: Option<WitVoiceFilter>) -> Result<Vec<WitVoiceInfo>, WitTtsError>;
    fn get_voice(voice_id: String) -> Result<WitVoiceInfo, WitTtsError>;
    fn search_voices(
        query: String,
        filter: Option<WitVoiceFilter>,
    ) -> Result<Vec<WitVoiceInfo>, WitTtsError>;
    fn list_languages() -> Result<Vec<WitLanguageInfo>, WitTtsError>;
}

/// Trait for synthesis operations
pub trait TtsSynthesisGuest {
    fn synthesize(
        input: WitTextInput,
        options: WitSynthesisOptions,
    ) -> Result<WitSynthesisResult, WitTtsError>;
    fn synthesize_batch(
        inputs: Vec<WitTextInput>,
        options: WitSynthesisOptions,
    ) -> Result<Vec<WitSynthesisResult>, WitTtsError>;
    fn get_timing_marks(
        input: WitTextInput,
        voice_id: String,
    ) -> Result<Vec<WitTimingInfo>, WitTtsError>;
    fn validate_input(
        input: WitTextInput,
        voice_id: String,
    ) -> Result<WitValidationResult, WitTtsError>;
}

/// Trait for streaming operations
pub trait TtsStreamingGuest {
    fn create_stream(options: WitSynthesisOptions) -> Result<WitStreamSession, WitTtsError>;
    fn stream_send_text(session_id: String, input: WitTextInput) -> Result<(), WitTtsError>;
    fn stream_finish(session_id: String) -> Result<(), WitTtsError>;
    fn stream_receive_chunk(session_id: String) -> Result<Option<WitAudioChunk>, WitTtsError>;
    fn stream_has_pending(session_id: String) -> Result<bool, WitTtsError>;
    fn stream_get_status(session_id: String) -> Result<WitStreamStatus, WitTtsError>;
    fn stream_close(session_id: String) -> Result<(), WitTtsError>;
}

/// Trait for advanced features
pub trait TtsAdvancedGuest {
    fn create_voice_clone(
        name: String,
        audio_samples: Vec<WitAudioSample>,
        description: Option<String>,
    ) -> Result<String, WitTtsError>;
    fn design_voice(
        name: String,
        characteristics: WitVoiceDesignParams,
    ) -> Result<String, WitTtsError>;
    fn convert_voice(
        input_audio: Vec<u8>,
        target_voice_id: String,
        preserve_timing: Option<bool>,
    ) -> Result<Vec<u8>, WitTtsError>;
    fn generate_sound_effect(
        description: String,
        duration_seconds: Option<f32>,
        style_influence: Option<f32>,
    ) -> Result<Vec<u8>, WitTtsError>;
    fn create_lexicon(
        name: String,
        language: String,
        entries: Option<Vec<WitPronunciationEntry>>,
    ) -> Result<String, WitTtsError>;
    fn add_lexicon_entry(
        lexicon_id: String,
        entry: WitPronunciationEntry,
    ) -> Result<(), WitTtsError>;
    fn remove_lexicon_entry(lexicon_id: String, word: String) -> Result<(), WitTtsError>;
    fn export_lexicon(lexicon_id: String) -> Result<String, WitTtsError>;
    fn synthesize_long_form(
        content: String,
        voice_id: String,
        output_location: String,
        chapter_breaks: Option<Vec<u32>>,
    ) -> Result<WitLongFormJob, WitTtsError>;
    fn get_long_form_status(job_id: String) -> Result<WitLongFormResult, WitTtsError>;
    fn cancel_long_form(job_id: String) -> Result<(), WitTtsError>;
}
