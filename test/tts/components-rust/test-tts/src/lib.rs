use crate::bindings::golem::tts::types::*;
use crate::bindings::golem::tts::voices;
use crate::bindings::golem::tts::synthesis;
use crate::bindings::golem::tts::streaming;
use crate::bindings::golem::tts::advanced;
use crate::bindings::exports::test::tts::test_tts::Guest;

#[allow(warnings)]
mod bindings;

struct Component;

impl Guest for Component {
    /// Test 1: Simple text-to-speech synthesis
    fn test1() -> String {
        let input = TextInput {
            content: "Hello world! This is a test of the text-to-speech system.".to_string(),
            text_type: TextType::PlainText,
        };

        let options = synthesis::SynthesisOptions {
            voice_id: "Joanna".to_string(), // Works for Polly, will be overridden for others
            audio_config: Some(AudioConfig {
                format: AudioFormat::Mp3,
                sample_rate: Some(24000),
                bit_rate: Some(128),
            }),
            voice_settings: None,
            audio_effects: None,
        };

        match synthesis::synthesize(input, options) {
            Ok(result) => {
                format!(
                    "✅ Synthesis successful! Audio size: {} bytes, Duration: {:.2}s, Characters: {}",
                    result.metadata.audio_size_bytes,
                    result.metadata.duration_seconds,
                    result.metadata.character_count
                )
            }
            Err(e) => format!("❌ Synthesis failed: {:?}", e),
        }
    }

    /// Test 2: Voice listing and discovery
    fn test2() -> String {
        match voices::list_voices(None) {
            Ok(voice_list) => {
                let count = voice_list.len();
                let sample_voices: Vec<String> = voice_list
                    .iter()
                    .take(3)
                    .map(|v| format!("{} ({})", v.name, v.language))
                    .collect();
                
                format!(
                    "✅ Found {} voices. Sample: {}",
                    count,
                    sample_voices.join(", ")
                )
            }
            Err(e) => format!("❌ Failed to list voices: {:?}", e),
        }
    }

    /// Test 3: Voice search with filters
    fn test_voice_search() -> String {
        // Search for female voices
        let filter = Some(voices::VoiceFilter {
            language: Some("en".to_string()),
            gender: Some(VoiceGender::Female),
            quality: None,
            voice_type: None,
        });

        match voices::list_voices(filter) {
            Ok(voice_list) => {
                let count = voice_list.len();
                let names: Vec<String> = voice_list
                    .iter()
                    .take(3)
                    .map(|v| v.name.clone())
                    .collect();
                
                format!(
                    "✅ Found {} female English voices. Examples: {}",
                    count,
                    names.join(", ")
                )
            }
            Err(e) => format!("❌ Voice search failed: {:?}", e),
        }
    }

    /// Test 4: Batch synthesis
    fn test_batch_synthesis() -> String {
        let inputs = vec![
            TextInput {
                content: "First sentence.".to_string(),
                text_type: TextType::PlainText,
            },
            TextInput {
                content: "Second sentence.".to_string(),
                text_type: TextType::PlainText,
            },
            TextInput {
                content: "Third sentence.".to_string(),
                text_type: TextType::PlainText,
            },
        ];

        let options = synthesis::SynthesisOptions {
            voice_id: "Joanna".to_string(),
            audio_config: Some(AudioConfig {
                format: AudioFormat::Mp3,
                sample_rate: Some(24000),
                bit_rate: Some(128),
            }),
            voice_settings: None,
            audio_effects: None,
        };

        match synthesis::synthesize_batch(inputs, options) {
            Ok(results) => {
                let total_size: u32 = results.iter().map(|r| r.metadata.audio_size_bytes).sum();
                format!(
                    "✅ Batch synthesis successful! {} segments, Total size: {} bytes",
                    results.len(),
                    total_size
                )
            }
            Err(e) => format!("❌ Batch synthesis failed: {:?}", e),
        }
    }

    /// Test 5: Voice settings customization
    fn test5() -> String {
        let input = TextInput {
            content: "Testing custom voice settings.".to_string(),
            text_type: TextType::PlainText,
        };

        let options = synthesis::SynthesisOptions {
            voice_id: "Matthew".to_string(),
            audio_config: Some(AudioConfig {
                format: AudioFormat::Mp3,
                sample_rate: Some(24000),
                bit_rate: Some(128),
            }),
            voice_settings: Some(VoiceSettings {
                speed: 1.2,
                pitch: 0.0,
                volume: 1.0,
                stability: Some(0.5),
                similarity_boost: Some(0.75),
                style: Some(0.0),
                use_speaker_boost: Some(false),
            }),
            audio_effects: None,
        };

        match synthesis::synthesize(input, options) {
            Ok(result) => {
                format!(
                    "✅ Synthesis with custom settings successful! Duration: {:.2}s",
                    result.metadata.duration_seconds
                )
            }
            Err(e) => format!("❌ Custom synthesis failed: {:?}", e),
        }
    }

    /// Test 3: Streaming synthesis (if supported)
    fn test3() -> String {
        let options = synthesis::SynthesisOptions {
            voice_id: "aura-asteria-en".to_string(), // Deepgram voice
            audio_config: Some(AudioConfig {
                format: AudioFormat::Mp3,
                sample_rate: Some(24000),
                bit_rate: Some(128),
            }),
            voice_settings: None,
            audio_effects: None,
        };

        match streaming::create_stream(options) {
            Ok(session) => {
                let input = TextInput {
                    content: "This is a streaming test.".to_string(),
                    text_type: TextType::PlainText,
                };
                
                match streaming::stream_send_text(session.session_id.clone(), input) {
                    Ok(_) => {
                        match streaming::stream_finish(session.session_id.clone()) {
                            Ok(_) => {
                                // Try to receive chunks
                                let mut chunk_count = 0;
                                loop {
                                    match streaming::stream_receive_chunk(session.session_id.clone()) {
                                        Ok(Some(_)) => chunk_count += 1,
                                        Ok(None) => break,
                                        Err(e) => return format!("❌ Chunk receive failed: {:?}", e),
                                    }
                                }
                                let _ = streaming::stream_close(session.session_id);
                                format!("✅ Streaming test completed! Received {} chunks", chunk_count)
                            }
                            Err(e) => format!("❌ Stream finish failed: {:?}", e),
                        }
                    }
                    Err(e) => format!("❌ Stream send failed: {:?}", e),
                }
            }
            Err(e) => format!("❌ Streaming not supported or failed: {:?}", e),
        }
    }

    /// Test: Error handling
    fn test_error_handling() -> String {
        // Try to get a non-existent voice
        match voices::get_voice("non-existent-voice-12345".to_string()) {
            Ok(_) => "❌ Expected error but got success!".to_string(),
            Err(TtsError::VoiceNotFound(id)) => {
                format!("✅ Correctly handled missing voice error: {}", id)
            }
            Err(e) => format!("❌ Got unexpected error: {:?}", e),
        }
    }
}

bindings::export!(Component with_types_in bindings);
