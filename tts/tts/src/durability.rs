use std::marker::PhantomData;

use crate::exports::golem::tts::advanced::Guest as WitAdvancedGuest;
use crate::exports::golem::tts::advanced::Guest as TtsAdvancedGuest;
use crate::exports::golem::tts::streaming::Guest as WitStreamingGuest;
use crate::exports::golem::tts::streaming::Guest as TtsStreamingGuest;
use crate::exports::golem::tts::synthesis::Guest as WitSynthesisGuest;
use crate::exports::golem::tts::synthesis::Guest as TtsSynthesisGuest;
use crate::exports::golem::tts::voices::Guest as WitVoicesGuest;
use crate::exports::golem::tts::voices::Guest as TtsVoicesGuest;

pub struct DurableTts<Impl> {
    phantom: PhantomData<Impl>,
}

pub trait ExtendedGuest:
    TtsVoicesGuest + TtsSynthesisGuest + TtsStreamingGuest + TtsAdvancedGuest + 'static
{
}

#[cfg(not(feature = "durability"))]
mod passthrough_impl {
    use super::*;
    use crate::exports::golem::tts::advanced::{
        AudioSample as WitAudioSample, LongFormJob as WitLongFormJob,
        LongFormResult as WitLongFormResult, PronunciationEntry as WitPronunciationEntry,
        VoiceDesignParams as WitVoiceDesignParams,
    };
    use crate::exports::golem::tts::streaming::{
        StreamSession as WitStreamSession, StreamStatus as WitStreamStatus,
    };
    use crate::exports::golem::tts::synthesis::{
        SynthesisOptions as WitSynthesisOptions, ValidationResult as WitValidationResult,
    };
    use crate::exports::golem::tts::types::TtsError as WitTtsError;
    use crate::exports::golem::tts::voices::{
        LanguageInfo as WitLanguageInfo, VoiceFilter as WitVoiceFilter, VoiceInfo as WitVoiceInfo,
    };
    use crate::golem::tts::types::{
        AudioChunk as WitAudioChunk, SynthesisResult as WitSynthesisResult,
        TextInput as WitTextInput, TimingInfo as WitTimingInfo,
    };
    use crate::LOGGING_STATE;

    impl<Impl: ExtendedGuest> WitVoicesGuest for DurableTts<Impl> {
        fn list_voices(filter: Option<WitVoiceFilter>) -> Result<Vec<WitVoiceInfo>, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::list_voices(filter)
        }

        fn get_voice(voice_id: String) -> Result<WitVoiceInfo, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::get_voice(voice_id)
        }

        fn search_voices(
            query: String,
            filter: Option<WitVoiceFilter>,
        ) -> Result<Vec<WitVoiceInfo>, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::search_voices(query, filter)
        }

        fn list_languages() -> Result<Vec<WitLanguageInfo>, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::list_languages()
        }
    }

    impl<Impl: ExtendedGuest> WitSynthesisGuest for DurableTts<Impl> {
        fn synthesize(
            input: WitTextInput,
            options: WitSynthesisOptions,
        ) -> Result<WitSynthesisResult, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::synthesize(input, options)
        }

        fn synthesize_batch(
            inputs: Vec<WitTextInput>,
            options: WitSynthesisOptions,
        ) -> Result<Vec<WitSynthesisResult>, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::synthesize_batch(inputs, options)
        }

        fn get_timing_marks(
            input: WitTextInput,
            voice_id: String,
        ) -> Result<Vec<WitTimingInfo>, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::get_timing_marks(input, voice_id)
        }

        fn validate_input(
            input: WitTextInput,
            voice_id: String,
        ) -> Result<WitValidationResult, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::validate_input(input, voice_id)
        }
    }

    impl<Impl: ExtendedGuest> WitStreamingGuest for DurableTts<Impl> {
        fn create_stream(options: WitSynthesisOptions) -> Result<WitStreamSession, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::create_stream(options)
        }

        fn stream_send_text(session_id: String, input: WitTextInput) -> Result<(), WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::stream_send_text(session_id, input)
        }

        fn stream_finish(session_id: String) -> Result<(), WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::stream_finish(session_id)
        }

        fn stream_receive_chunk(session_id: String) -> Result<Option<WitAudioChunk>, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::stream_receive_chunk(session_id)
        }

        fn stream_has_pending(session_id: String) -> Result<bool, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::stream_has_pending(session_id)
        }

        fn stream_get_status(session_id: String) -> Result<WitStreamStatus, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::stream_get_status(session_id)
        }

        fn stream_close(session_id: String) -> Result<(), WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::stream_close(session_id)
        }
    }

    impl<Impl: ExtendedGuest> WitAdvancedGuest for DurableTts<Impl> {
        fn create_voice_clone(
            name: String,
            audio_samples: Vec<WitAudioSample>,
            description: Option<String>,
        ) -> Result<String, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::create_voice_clone(name, audio_samples, description)
        }

        fn design_voice(
            name: String,
            characteristics: WitVoiceDesignParams,
        ) -> Result<String, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::design_voice(name, characteristics)
        }

        fn convert_voice(
            input_audio: Vec<u8>,
            target_voice_id: String,
            preserve_timing: Option<bool>,
        ) -> Result<Vec<u8>, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::convert_voice(input_audio, target_voice_id, preserve_timing)
        }

        fn generate_sound_effect(
            description: String,
            duration_seconds: Option<f32>,
            style_influence: Option<f32>,
        ) -> Result<Vec<u8>, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::generate_sound_effect(description, duration_seconds, style_influence)
        }

        fn create_lexicon(
            name: String,
            language: String,
            entries: Option<Vec<WitPronunciationEntry>>,
        ) -> Result<String, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::create_lexicon(name, language, entries)
        }

        fn add_lexicon_entry(
            lexicon_id: String,
            entry: WitPronunciationEntry,
        ) -> Result<(), WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::add_lexicon_entry(lexicon_id, entry)
        }

        fn remove_lexicon_entry(lexicon_id: String, word: String) -> Result<(), WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::remove_lexicon_entry(lexicon_id, word)
        }

        fn export_lexicon(lexicon_id: String) -> Result<String, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::export_lexicon(lexicon_id)
        }

        fn synthesize_long_form(
            content: String,
            voice_id: String,
            output_location: String,
            chapter_breaks: Option<Vec<u32>>,
        ) -> Result<WitLongFormJob, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::synthesize_long_form(content, voice_id, output_location, chapter_breaks)
        }

        fn get_long_form_status(job_id: String) -> Result<WitLongFormResult, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::get_long_form_status(job_id)
        }

        fn cancel_long_form(job_id: String) -> Result<(), WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::cancel_long_form(job_id)
        }
    }
}

#[cfg(feature = "durability")]
mod durable_impl {
    use super::*;
    use golem_rust::bindings::golem::durability::durability::DurableFunctionType;
    use golem_rust::durability::Durability;
    use golem_rust::{with_persistence_level, FromValueAndType, IntoValue, PersistenceLevel};

    use crate::exports::golem::tts::advanced::{
        AudioSample as WitAudioSample, LongFormJob as WitLongFormJob,
        LongFormResult as WitLongFormResult, PronunciationEntry as WitPronunciationEntry,
        VoiceDesignParams as WitVoiceDesignParams,
    };
    use crate::exports::golem::tts::streaming::{
        StreamSession as WitStreamSession, StreamStatus as WitStreamStatus,
    };
    use crate::exports::golem::tts::synthesis::{
        SynthesisOptions as WitSynthesisOptions, ValidationResult as WitValidationResult,
    };
    use crate::exports::golem::tts::types::TtsError as WitTtsError;
    use crate::exports::golem::tts::voices::{
        LanguageInfo as WitLanguageInfo, VoiceFilter as WitVoiceFilter, VoiceInfo as WitVoiceInfo,
    };
    use crate::golem::tts::types::{
        AudioChunk as WitAudioChunk, SynthesisResult as WitSynthesisResult,
        TextInput as WitTextInput, TimingInfo as WitTimingInfo,
    };
    use crate::LOGGING_STATE;

    impl<Impl: ExtendedGuest> WitVoicesGuest for DurableTts<Impl> {
        fn list_voices(filter: Option<WitVoiceFilter>) -> Result<Vec<WitVoiceInfo>, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::list_voices(filter)
        }

        fn get_voice(voice_id: String) -> Result<WitVoiceInfo, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::get_voice(voice_id)
        }

        fn search_voices(
            query: String,
            filter: Option<WitVoiceFilter>,
        ) -> Result<Vec<WitVoiceInfo>, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::search_voices(query, filter)
        }

        fn list_languages() -> Result<Vec<WitLanguageInfo>, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::list_languages()
        }
    }

    impl<Impl: ExtendedGuest> WitSynthesisGuest for DurableTts<Impl> {
        fn synthesize(
            input: WitTextInput,
            options: WitSynthesisOptions,
        ) -> Result<WitSynthesisResult, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());

            let durability = Durability::<WitSynthesisResult, WitTtsError>::new(
                "golem_tts",
                "synthesize",
                DurableFunctionType::WriteRemote,
            );

            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::synthesize(input.clone(), options.clone())
                });
                durability.persist(SynthesizeInput { input, options }, result)
            } else {
                durability.replay()
            }
        }

        fn synthesize_batch(
            inputs: Vec<WitTextInput>,
            options: WitSynthesisOptions,
        ) -> Result<Vec<WitSynthesisResult>, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());

            let durability = Durability::<Vec<WitSynthesisResult>, WitTtsError>::new(
                "golem_tts",
                "synthesize_batch",
                DurableFunctionType::WriteRemote,
            );

            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::synthesize_batch(inputs.clone(), options.clone())
                });
                durability.persist(SynthesizeBatchInput { inputs, options }, result)
            } else {
                durability.replay()
            }
        }

        fn get_timing_marks(
            input: WitTextInput,
            voice_id: String,
        ) -> Result<Vec<WitTimingInfo>, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::get_timing_marks(input, voice_id)
        }

        fn validate_input(
            input: WitTextInput,
            voice_id: String,
        ) -> Result<WitValidationResult, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::validate_input(input, voice_id)
        }
    }

    impl<Impl: ExtendedGuest> WitStreamingGuest for DurableTts<Impl> {
        fn create_stream(options: WitSynthesisOptions) -> Result<WitStreamSession, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::create_stream(options)
        }

        fn stream_send_text(session_id: String, input: WitTextInput) -> Result<(), WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::stream_send_text(session_id, input)
        }

        fn stream_finish(session_id: String) -> Result<(), WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::stream_finish(session_id)
        }

        fn stream_receive_chunk(session_id: String) -> Result<Option<WitAudioChunk>, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::stream_receive_chunk(session_id)
        }

        fn stream_has_pending(session_id: String) -> Result<bool, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::stream_has_pending(session_id)
        }

        fn stream_get_status(session_id: String) -> Result<WitStreamStatus, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::stream_get_status(session_id)
        }

        fn stream_close(session_id: String) -> Result<(), WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::stream_close(session_id)
        }
    }

    impl<Impl: ExtendedGuest> WitAdvancedGuest for DurableTts<Impl> {
        fn create_voice_clone(
            name: String,
            audio_samples: Vec<WitAudioSample>,
            description: Option<String>,
        ) -> Result<String, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::create_voice_clone(name, audio_samples, description)
        }

        fn design_voice(
            name: String,
            characteristics: WitVoiceDesignParams,
        ) -> Result<String, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::design_voice(name, characteristics)
        }

        fn convert_voice(
            input_audio: Vec<u8>,
            target_voice_id: String,
            preserve_timing: Option<bool>,
        ) -> Result<Vec<u8>, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::convert_voice(input_audio, target_voice_id, preserve_timing)
        }

        fn generate_sound_effect(
            description: String,
            duration_seconds: Option<f32>,
            style_influence: Option<f32>,
        ) -> Result<Vec<u8>, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::generate_sound_effect(description, duration_seconds, style_influence)
        }

        fn create_lexicon(
            name: String,
            language: String,
            entries: Option<Vec<WitPronunciationEntry>>,
        ) -> Result<String, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::create_lexicon(name, language, entries)
        }

        fn add_lexicon_entry(
            lexicon_id: String,
            entry: WitPronunciationEntry,
        ) -> Result<(), WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::add_lexicon_entry(lexicon_id, entry)
        }

        fn remove_lexicon_entry(lexicon_id: String, word: String) -> Result<(), WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::remove_lexicon_entry(lexicon_id, word)
        }

        fn export_lexicon(lexicon_id: String) -> Result<String, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::export_lexicon(lexicon_id)
        }

        fn synthesize_long_form(
            content: String,
            voice_id: String,
            output_location: String,
            chapter_breaks: Option<Vec<u32>>,
        ) -> Result<WitLongFormJob, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::synthesize_long_form(content, voice_id, output_location, chapter_breaks)
        }

        fn get_long_form_status(job_id: String) -> Result<WitLongFormResult, WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::get_long_form_status(job_id)
        }

        fn cancel_long_form(job_id: String) -> Result<(), WitTtsError> {
            LOGGING_STATE.with_borrow_mut(|state| state.init());
            Impl::cancel_long_form(job_id)
        }
    }

    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct SynthesizeInput {
        input: WitTextInput,
        options: WitSynthesisOptions,
    }

    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct SynthesizeBatchInput {
        inputs: Vec<WitTextInput>,
        options: WitSynthesisOptions,
    }

    impl From<&WitTtsError> for WitTtsError {
        fn from(error: &WitTtsError) -> Self {
            error.clone()
        }
    }
}
