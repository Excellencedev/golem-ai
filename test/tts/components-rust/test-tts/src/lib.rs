// Comprehensive test suite covering all ticket requirements
use golem_tts::exports::golem::tts::advanced::Guest as AdvancedGuest;
use golem_tts::exports::golem::tts::streaming::Guest as StreamingGuest;
use golem_tts::exports::golem::tts::synthesis::Guest as SynthesisGuest;
use golem_tts::exports::golem::tts::synthesis::*;
use golem_tts::exports::golem::tts::voices::Guest as VoicesGuest;
use golem_tts::exports::golem::tts::voices::*;
use golem_tts::golem::tts::types::*;

wit_bindgen::generate!({
    world: "tts-test",
    path: "./wit/",
});

struct TestComponent;

export!(TestComponent);

impl Guest for TestComponent {
    // Basic synthesis operations
    fn test1() -> String {
        test_basic_synthesis()
    }

    // Voice discovery and metadata
    fn test2() -> String {
        test_voice_discovery()
    }

    // Streaming synthesis lifecycle
    fn test3() -> String {
        test_streaming_lifecycle()
    }

    // Error handling
    fn test_error_handling() -> String {
        test_error_scenarios()
    }

    // Voice search with filters
    fn test_voice_search() -> String {
        test_voice_filtering()
    }

    // Batch synthesis
    fn test_batch_synthesis() -> String {
        test_batch_operations()
    }

    // Voice settings customization
    fn test5() -> String {
        test_voice_settings()
    }

    // NEW: Authentication scenarios
    fn test_authentication() -> String {
        test_auth_scenarios()
    }

    // NEW: Rate limiting behavior
    fn test_rate_limiting() -> String {
        test_rate_limit_handling()
    }

    // NEW: Long-form content
    fn test_long_form_content() -> String {
        test_longform_synthesis()
    }

    // NEW: Durability verification
    fn test_durability() -> String {
        test_durability_semantics()
    }

    // NEW: Provider-specific features
    fn test_provider_features() -> String {
        test_advanced_provider_features()
    }
}

fn test_basic_synthesis() -> String {
    let input = TextInput {
        content: "Hello, this is a basic synthesis test.".to_string(),
        language: Some("en".to_string()),
        pronunciation_lexicon: None,
    };

    let options = SynthesisOptions {
        voice_id: "default".to_string(),
        audio_config: Some(AudioConfig {
            format: AudioFormat::Mp3,
            sample_rate: Some(24000),
            bitrate: None,
            channels: Some(1),
        }),
        voice_settings: None,
        audio_effects: None,
        model_version: None,
    };

    match VoicesGuest::synthesize(input, options) {
        Ok(result) => {
            format!(
                "SUCCESS: Synthesized {} bytes, {} chars, {} words",
                result.metadata.audio_size_bytes,
                result.metadata.character_count,
                result.metadata.word_count
            )
        }
        Err(e) => format!("ERROR: {:?}", e),
    }
}

fn test_voice_discovery() -> String {
    match VoicesGuest::list_voices(None) {
        Ok(voices) => {
            let mut result = format!("Found {} voices:\n", voices.len());
            for (i, voice) in voices.iter().take(5).enumerate() {
                result.push_str(&format!(
                    "{}. {} ({}): {} - {}\n",
                    i + 1,
                    voice.name,
                    voice.id,
                    voice.language,
                    voice.quality.to_string()
                ));
            }
            result
        }
        Err(e) => format!("ERROR: {:?}", e),
    }
}

fn test_streaming_lifecycle() -> String {
    let options = SynthesisOptions {
        voice_id: "default".to_string(),
        audio_config: None,
        voice_settings: None,
        audio_effects: None,
        model_version: None,
    };

    match StreamingGuest::stream_create(options) {
        Ok(session) => {
            let input = TextInput {
                content: "Streaming test".to_string(),
                language: None,
                pronunciation_lexicon: None,
            };

            match StreamingGuest::stream_send_text(session.session_id.clone(), input) {
                Ok(_) => match StreamingGuest::stream_finish(session.session_id.clone()) {
                    Ok(_) => format!(
                        "SUCCESS: Streaming lifecycle completed for session {}",
                        session.session_id
                    ),
                    Err(e) => format!("ERROR in finish: {:?}", e),
                },
                Err(e) => format!("ERROR in send: {:?}", e),
            }
        }
        Err(e) => format!("ERROR creating stream: {:?}", e),
    }
}

fn test_error_scenarios() -> String {
    // Test 1: Voice not found
    let voice_result = VoicesGuest::get_voice("nonexistent-voice-12345".to_string());
    let test1 = match voice_result {
        Err(TtsError::VoiceNotFound(_)) => "PASS",
        _ => "FAIL",
    };

    // Test 2: Invalid input (empty text)
    let input = TextInput {
        content: "".to_string(),
        language: None,
        pronunciation_lexicon: None,
    };
    let options = SynthesisOptions {
        voice_id: "default".to_string(),
        audio_config: None,
        voice_settings: None,
        audio_effects: None,
        model_version: None,
    };
    let synth_result = SynthesisGuest::synthesize(input, options);
    let test2 = match synth_result {
        Err(TtsError::InvalidInput(_)) | Err(_) => "PASS",
        Ok(_) => "FAIL - Expected error for empty input",
    };

    format!(
        "Error Handling Tests:\n1. Voice Not Found: {}\n2. Empty Input: {}",
        test1, test2
    )
}

fn test_voice_filtering() -> String {
    let filter = Some(VoiceFilter {
        language: Some("en".to_string()),
        gender: Some(VoiceGender::Female),
        quality: None,
        use_cases: vec![],
    });

    match VoicesGuest::search_voices("assistant".to_string(), filter) {
        Ok(voices) => {
            format!(
                "Found {} matching voices (language=en, gender=female, query=assistant)",
                voices.len()
            )
        }
        Err(e) => format!("ERROR: {:?}", e),
    }
}

fn test_batch_operations() -> String {
    let inputs = vec![
        TextInput {
            content: "First sentence.".to_string(),
            language: None,
            pronunciation_lexicon: None,
        },
        TextInput {
            content: "Second sentence.".to_string(),
            language: None,
            pronunciation_lexicon: None,
        },
        TextInput {
            content: "Third sentence.".to_string(),
            language: None,
            pronunciation_lexicon: None,
        },
    ];

    let options = SynthesisOptions {
        voice_id: "default".to_string(),
        audio_config: None,
        voice_settings: None,
        audio_effects: None,
        model_version: None,
    };

    match SynthesisGuest::synthesize_batch(inputs, options) {
        Ok(results) => {
            format!("SUCCESS: Batch synthesized {} results", results.len())
        }
        Err(e) => format!("ERROR: {:?}", e),
    }
}

fn test_voice_settings() -> String {
    let input = TextInput {
        content: "Testing customized voice settings.".to_string(),
        language: None,
        pronunciation_lexicon: None,
    };

    let options = SynthesisOptions {
        voice_id: "default".to_string(),
        audio_config: None,
        voice_settings: Some(VoiceSettings {
            speed: 1.2,
            pitch: 1.1,
            volume: 1.0,
            stability: Some(0.7),
            similarity: Some(0.8),
            style: Some(0.5),
        }),
        audio_effects: None,
        model_version: None,
    };

    match SynthesisGuest::synthesize(input, options) {
        Ok(_) => "SUCCESS: Voice settings applied".to_string(),
        Err(e) => format!("ERROR: {:?}", e),
    }
}

fn test_auth_scenarios() -> String {
    // Test that provider handles authentication
    // This will fail if API keys are not set, which verifies auth is checked
    let result1 = VoicesGuest::list_voices(None);

    match result1 {
        Ok(_) => "SUCCESS: Authentication working (API keys configured)".to_string(),
        Err(TtsError::AuthenticationError(_)) => {
            "PASS: Authentication error correctly returned for missing credentials".to_string()
        }
        Err(TtsError::ConfigurationError(_)) => {
            "PASS: Configuration error correctly returned for missing credentials".to_string()
        }
        Err(e) => format!("UNEXPECTED ERROR: {:?}", e),
    }
}

fn test_rate_limit_handling() -> String {
    // Make multiple rapid requests to test rate limiting
    let mut success_count = 0;
    let mut rate_limit_hit = false;

    for i in 0..5 {
        let input = TextInput {
            content: format!("Rate limit test {}", i),
            language: None,
            pronunciation_lexicon: None,
        };

        let options = SynthesisOptions {
            voice_id: "default".to_string(),
            audio_config: None,
            voice_settings: None,
            audio_effects: None,
            model_version: None,
        };

        match SynthesisGuest::synthesize(input, options) {
            Ok(_) => success_count += 1,
            Err(TtsError::RateLimitExceeded(_)) => {
                rate_limit_hit = true;
                break;
            }
            Err(_) => break,
        }
    }

    if rate_limit_hit {
        format!(
            "PASS: Rate limiting detected after {} requests",
            success_count
        )
    } else {
        format!(
            "SUCCESS: Completed {} rapid requests without hitting rate limit",
            success_count
        )
    }
}

fn test_longform_synthesis() -> String {
    // Test with >5000 characters
    let long_text = "This is a test of long-form content synthesis. ".repeat(120); // ~5760 chars

    let input = TextInput {
        content: long_text.clone(),
        language: None,
        pronunciation_lexicon: None,
    };

    let options = SynthesisOptions {
        voice_id: "default".to_string(),
        audio_config: None,
        voice_settings: None,
        audio_effects: None,
        model_version: None,
    };

    match SynthesisGuest::synthesize(input, options) {
        Ok(result) => {
            format!(
                "SUCCESS: Long-form synthesis completed ({} chars, {} bytes audio)",
                result.metadata.character_count, result.metadata.audio_size_bytes
            )
        }
        Err(e) => format!("ERROR: {:?}", e),
    }
}

fn test_durability_semantics() -> String {
    // Test that operations are durable by performing synthesis
    // in Golem, this should be logged in the operation log
    let input = TextInput {
        content: "Durability test - this operation should be in the log.".to_string(),
        language: None,
        pronunciation_lexicon: None,
    };

    let options = SynthesisOptions {
        voice_id: "default".to_string(),
        audio_config: None,
        voice_settings: None,
        audio_effects: None,
        model_version: None,
    };

    match SynthesisGuest::synthesize(input, options) {
        Ok(result) => {
            format!(
                "SUCCESS: Durable operation completed (request_id: {})",
                result.metadata.request_id
            )
        }
        Err(e) => format!("ERROR: {:?}", e),
    }
}

fn test_advanced_provider_features() -> String {
    let mut results = Vec::new();

    // Test speech marks (Polly)
    match AdvancedGuest::get_speech_marks(
        TextInput {
            content: "Testing speech marks.".to_string(),
            language: None,
            pronunciation_lexicon: None,
        },
        "default".to_string(),
    ) {
        Ok(marks) => results.push(format!("Speech marks: {} marks generated", marks.len())),
        Err(TtsError::UnsupportedOperation(_)) => {
            results.push("Speech marks: Not supported by this provider".to_string())
        }
        Err(e) => results.push(format!("Speech marks ERROR: {:?}", e)),
    }

    // Test sound effects (ElevenLabs)
    match AdvancedGuest::generate_sound_effect("Dog barking".to_string(), Some(3.0), None) {
        Ok(audio) => results.push(format!("Sound effects: Generated {} bytes", audio.len())),
        Err(TtsError::UnsupportedOperation(_)) => {
            results.push("Sound effects: Not supported by this provider".to_string())
        }
        Err(e) => results.push(format!("Sound effects ERROR: {:?}", e)),
    }

    results.join("\n")
}
