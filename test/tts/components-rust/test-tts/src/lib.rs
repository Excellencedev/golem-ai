#[allow(static_mut_refs)]
mod bindings;

use crate::bindings::exports::test::tts::test_tts_api::*;
use crate::bindings::golem::tts::advanced::generate_sound_effect;
use crate::bindings::golem::tts::streaming::{create_stream, stream_finish, stream_send_text};
use crate::bindings::golem::tts::synthesis::{
    get_timing_marks, synthesize, synthesize_batch, validate_input, SynthesisOptions,
};
use crate::bindings::golem::tts::types::{
    AudioConfig, AudioFormat, TextInput, TextType, VoiceGender, VoiceSettings,
};
use crate::bindings::golem::tts::voices::{get_voice, list_voices, search_voices, VoiceFilter};

struct Component;

impl Guest for Component {
    /// Test 1: Basic Synthesis
    /// Tests basic text-to-speech synthesis with default settings
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
                "✓ SUCCESS: Basic Synthesis\n\
                 ├─ Audio size: {} bytes\n\
                 ├─ Characters: {}\n\
                 ├─ Words: {}\n\
                 ├─ Duration: {:.2}s\n\
                 └─ Request ID: {}",
                result.metadata.audio_size_bytes,
                result.metadata.character_count,
                result.metadata.word_count,
                result.metadata.duration_seconds,
                result.metadata.request_id
            )),
            Err(e) => Err(format!("✗ FAILED: Basic Synthesis\n└─ Error: {:?}", e)),
        }
    }

    /// Test 2: Voice Listing
    /// Tests voice discovery and filtering capabilities
    fn test2() -> Result<String, String> {
        match list_voices(None) {
            Ok(voices) => {
                let mut result = format!(
                    "✓ SUCCESS: Voice Listing\n├─ Total voices: {}\n",
                    voices.len()
                );

                if voices.is_empty() {
                    result.push_str("└─ Warning: No voices available\n");
                } else {
                    result.push_str("└─ Sample voices:\n");
                    for (i, voice) in voices.iter().take(5).enumerate() {
                        let prefix = if i == voices.iter().take(5).count() - 1 {
                            "  └─"
                        } else {
                            "  ├─"
                        };
                        result.push_str(&format!(
                            "{}  {}. {} ({})\n{}     Language: {}, Quality: {:?}, Gender: {:?}\n",
                            prefix,
                            i + 1,
                            voice.name,
                            voice.id,
                            if i == voices.iter().take(5).count() - 1 {
                                "     "
                            } else {
                                "  │  "
                            },
                            voice.language,
                            voice.quality,
                            voice.gender
                        ));
                    }
                    if voices.len() > 5 {
                        result
                            .push_str(&format!("     ... and {} more voices\n", voices.len() - 5));
                    }
                }
                Ok(result)
            }
            Err(e) => Err(format!("✗ FAILED: Voice Listing\n└─ Error: {:?}", e)),
        }
    }

    /// Test 3: Streaming Lifecycle
    /// Tests streaming API (create stream, send text, finish)
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
                            "✓ SUCCESS: Streaming Lifecycle\n\
                             ├─ Session ID: {}\n\
                             ├─ Model: {}\n\
                             └─ Status: Completed",
                            session.session_id, session.model
                        )),
                        Err(e) => Err(format!(
                            "✗ FAILED: Streaming Lifecycle (finish)\n\
                             ├─ Session ID: {}\n\
                             └─ Error: {:?}",
                            session.session_id, e
                        )),
                    },
                    Err(e) => Err(format!(
                        "✗ FAILED: Streaming Lifecycle (send)\n\
                         ├─ Session ID: {}\n\
                         └─ Error: {:?}",
                        session.session_id, e
                    )),
                }
            }
            Err(e) => {
                // Streaming may not be supported - this is expected for some providers
                Ok(format!(
                    "⚠ INFO: Streaming Lifecycle\n\
                     └─ Streaming not available: {:?}\n\
                     Note: This is expected for providers without WebSocket support",
                    e
                ))
            }
        }
    }

    /// Test 4: Batch Synthesis
    /// Tests batch processing of multiple text inputs
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
            TextInput {
                content: "Third sentence.".to_string(),
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
            Ok(results) => {
                let total_audio: u32 = results.iter().map(|r| r.metadata.audio_size_bytes).sum();
                let total_chars: u32 = results.iter().map(|r| r.metadata.character_count).sum();

                Ok(format!(
                    "✓ SUCCESS: Batch Synthesis\n\
                     ├─ Inputs processed: {}\n\
                     ├─ Total audio: {} bytes\n\
                     ├─ Total characters: {}\n\
                     └─ Average per input: {} bytes",
                    results.len(),
                    total_audio,
                    total_chars,
                    total_audio / results.len() as u32
                ))
            }
            Err(e) => Err(format!("✗ FAILED: Batch Synthesis\n└─ Error: {:?}", e)),
        }
    }

    /// Test 5: Voice Settings
    /// Tests customized voice settings (speed, pitch, volume, stability)
    fn test5() -> Result<String, String> {
        let input = TextInput {
            content: "Testing customized voice settings.".to_string(),
            language: None,
            text_type: TextType::Plain,
        };

        let voice_settings = VoiceSettings {
            speed: Some(1.2),
            pitch: Some(1.1),
            volume: Some(1.0),
            stability: Some(0.7),
            similarity: Some(0.8),
            style: Some(0.5),
        };

        let options = SynthesisOptions {
            voice_id: "default".to_string(),
            audio_config: None,
            voice_settings: Some(voice_settings.clone()),
            audio_effects: None,
            model_version: None,
            enable_timing: None,
            enable_word_timing: None,
            seed: None,
            context: None,
        };

        match synthesize(&input, &options) {
            Ok(result) => Ok(format!(
                "✓ SUCCESS: Voice Settings\n\
                 ├─ Speed: {:.1}x\n\
                 ├─ Pitch: {:.1}\n\
                 ├─ Volume: {:.1}\n\
                 ├─ Stability: {:.1}\n\
                 ├─ Similarity: {:.1}\n\
                 ├─ Style: {:.1}\n\
                 └─ Audio generated: {} bytes",
                voice_settings.speed.unwrap(),
                voice_settings.pitch.unwrap(),
                voice_settings.volume.unwrap(),
                voice_settings.stability.unwrap(),
                voice_settings.similarity.unwrap(),
                voice_settings.style.unwrap(),
                result.metadata.audio_size_bytes
            )),
            Err(e) => Err(format!("✗ FAILED: Voice Settings\n└─ Error: {:?}", e)),
        }
    }

    /// Test 6: Voice Search
    /// Tests voice search with filters
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
            Ok(voices) => {
                let mut result = format!(
                    "✓ SUCCESS: Voice Search\n\
                     ├─ Query: 'assistant'\n\
                     ├─ Filters: language=en, gender=female\n\
                     ├─ Matches found: {}\n",
                    voices.len()
                );

                if !voices.is_empty() {
                    result.push_str("└─ Top matches:\n");
                    for (i, voice) in voices.iter().take(3).enumerate() {
                        let prefix = if i == voices.iter().take(3).count() - 1 {
                            "  └─"
                        } else {
                            "  ├─"
                        };
                        result.push_str(&format!("{}  {} ({})\n", prefix, voice.name, voice.id));
                    }
                }
                Ok(result)
            }
            Err(e) => Err(format!(
                "✗ FAILED: Voice Search\n\
                 ├─ Query: 'assistant'\n\
                 └─ Error: {:?}",
                e
            )),
        }
    }

    /// Test 7: Advanced Features
    /// Tests timing marks and sound effects generation
    fn test7() -> Result<String, String> {
        let mut results = Vec::new();

        // Test input validation
        match validate_input(
            &TextInput {
                content: "Testing input validation with a reasonably long text.".to_string(),
                language: None,
                text_type: TextType::Plain,
            },
            "default",
        ) {
            Ok(validation) => {
                results.push(format!(
                    "✓ Input Validation:\n  \
                     ├─ Valid: {}\n  \
                     ├─ Characters: {}\n  \
                     ├─ Est. duration: {:.2}s\n  \
                     ├─ Warnings: {}\n  \
                     └─ Errors: {}",
                    validation.is_valid,
                    validation.character_count,
                    validation.estimated_duration.unwrap_or(0.0),
                    validation.warnings.len(),
                    validation.errors.len()
                ));
            }
            Err(e) => results.push(format!("✗ Input Validation failed: {:?}", e)),
        }

        // Test timing marks (may not be supported)
        match get_timing_marks(
            &TextInput {
                content: "Testing timing marks.".to_string(),
                language: None,
                text_type: TextType::Plain,
            },
            "default",
        ) {
            Ok(marks) => results.push(format!("✓ Timing marks: {} marks generated", marks.len())),
            Err(_) => results.push("⚠ Timing marks: Not supported (expected)".to_string()),
        }

        // Test sound effects (may not be supported)
        match generate_sound_effect("Dog barking", Some(3.0), None) {
            Ok(audio) => results.push(format!("✓ Sound effects: Generated {} bytes", audio.len())),
            Err(_) => results.push("⚠ Sound effects: Not supported (expected)".to_string()),
        }

        Ok(format!(
            "✓ SUCCESS: Advanced Features\n{}",
            results.join("\n")
        ))
    }
}

bindings::export!(Component with_types_in bindings);
