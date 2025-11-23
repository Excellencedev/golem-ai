#[allow(static_mut_refs)]
mod bindings;

use crate::bindings::exports::test::tts::test_tts_api::*;
use crate::bindings::golem::tts::advanced::generate_sound_effect;
use crate::bindings::golem::tts::streaming::{create_stream, stream_finish, stream_send_text};
use crate::bindings::golem::tts::synthesis::{
    get_timing_marks, synthesize, synthesize_batch, SynthesisOptions,
};
use crate::bindings::golem::tts::types::{
    AudioConfig, AudioFormat, TextInput, TextType, VoiceGender, VoiceSettings,
};
use crate::bindings::golem::tts::voices::{get_voice, list_voices, search_voices, VoiceFilter};

struct Component;

impl Guest for Component {
    fn test1() -> Result<String, String> {
        let input = TextInput {
            content: "Hello, this is a basic synthesis test.".to_string(),
            language: Some("en".to_string()),
            text_type: TextType::Plain,
        };

        let options = SynthesisOptions {
            voice_id: "default".to_string(),
            audio_config: Some(AudioConfig {
                format: AudioFormat::Mp3,
                sample_rate: Some(24000),
                bit_rate: None,
                channels: Some(1),
            }),
            voice_settings: None,
            audio_effects: None,
            model_version: None,
            enable_timing: None,
            enable_word_timing: None,
            seed: None,
            context: None,
        };

        match synthesize(&input, &options) {
            Ok(result) => Ok(format!(
                "SUCCESS: Synthesized {} bytes, {} chars, {} words",
                result.metadata.audio_size_bytes,
                result.metadata.character_count,
                result.metadata.word_count
            )),
            Err(e) => Err(format!("ERROR: {:?}", e)),
        }
    }

    fn test2() -> Result<String, String> {
        match list_voices(None) {
            Ok(voices) => {
                let mut result = format!("Found {} voices:\n", voices.len());
                for (i, voice) in voices.iter().take(5).enumerate() {
                    result.push_str(&format!(
                        "{}. {} ({}): {} - {:?}\n",
                        i + 1,
                        voice.name,
                        voice.id,
                        voice.language,
                        voice.quality
                    ));
                }
                Ok(result)
            }
            Err(e) => Err(format!("ERROR: {:?}", e)),
        }
    }

    fn test3() -> Result<String, String> {
        let options = SynthesisOptions {
            voice_id: "default".to_string(),
            audio_config: None,
            voice_settings: None,
            audio_effects: None,
            model_version: None,
            enable_timing: None,
            enable_word_timing: None,
            seed: None,
            context: None,
        };

        match create_stream(&options) {
            Ok(session) => {
                let input = TextInput {
                    content: "Streaming test".to_string(),
                    language: None,
                    text_type: TextType::Plain,
                };

                match stream_send_text(&session.session_id, &input) {
                    Ok(_) => match stream_finish(&session.session_id) {
                        Ok(_) => Ok(format!(
                            "SUCCESS: Streaming lifecycle completed for session {}",
                            session.session_id
                        )),
                        Err(e) => Err(format!("ERROR in finish: {:?}", e)),
                    },
                    Err(e) => Err(format!("ERROR in send: {:?}", e)),
                }
            }
            Err(e) => Err(format!("ERROR creating stream: {:?}", e)),
        }
    }

    fn test4() -> Result<String, String> {
        let inputs = vec![
            TextInput {
                content: "First sentence.".to_string(),
                language: None,
                text_type: TextType::Plain,
            },
            TextInput {
                content: "Second sentence.".to_string(),
                language: None,
                text_type: TextType::Plain,
            },
        ];

        let options = SynthesisOptions {
            voice_id: "default".to_string(),
            audio_config: None,
            voice_settings: None,
            audio_effects: None,
            model_version: None,
            enable_timing: None,
            enable_word_timing: None,
            seed: None,
            context: None,
        };

        match synthesize_batch(&inputs, &options) {
            Ok(results) => Ok(format!(
                "SUCCESS: Batch synthesized {} results",
                results.len()
            )),
            Err(e) => Err(format!("ERROR: {:?}", e)),
        }
    }

    fn test5() -> Result<String, String> {
        let input = TextInput {
            content: "Testing customized voice settings.".to_string(),
            language: None,
            text_type: TextType::Plain,
        };

        let options = SynthesisOptions {
            voice_id: "default".to_string(),
            audio_config: None,
            voice_settings: Some(VoiceSettings {
                speed: Some(1.2),
                pitch: Some(1.1),
                volume: Some(1.0),
                stability: Some(0.7),
                similarity: Some(0.8),
                style: Some(0.5),
            }),
            audio_effects: None,
            model_version: None,
            enable_timing: None,
            enable_word_timing: None,
            seed: None,
            context: None,
        };

        match synthesize(&input, &options) {
            Ok(_) => Ok("SUCCESS: Voice settings applied".to_string()),
            Err(e) => Err(format!("ERROR: {:?}", e)),
        }
    }

    fn test6() -> Result<String, String> {
        let filter = Some(VoiceFilter {
            language: Some("en".to_string()),
            gender: Some(VoiceGender::Female),
            quality: None,
            supports_ssml: None,
            provider: None,
            search_query: None,
        });

        match search_voices("assistant", filter.as_ref()) {
            Ok(voices) => Ok(format!(
                "Found {} matching voices (language=en, gender=female, query=assistant)",
                voices.len()
            )),
            Err(e) => Err(format!("ERROR: {:?}", e)),
        }
    }

    fn test7() -> Result<String, String> {
        let mut results = Vec::new();

        // Test timing marks (was speech marks)
        match get_timing_marks(
            &TextInput {
                content: "Testing timing marks.".to_string(),
                language: None,
                text_type: TextType::Plain,
            },
            "default",
        ) {
            Ok(marks) => results.push(format!("Timing marks: {} marks generated", marks.len())),
            Err(e) => results.push(format!("Timing marks result: {:?}", e)),
        }

        // Test sound effects
        match generate_sound_effect("Dog barking", Some(3.0), None) {
            Ok(audio) => results.push(format!("Sound effects: Generated {} bytes", audio.len())),
            Err(e) => results.push(format!("Sound effects result: {:?}", e)),
        }

        Ok(results.join("\n"))
    }
}

bindings::export!(Component with_types_in bindings);
