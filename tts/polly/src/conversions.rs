// Type conversions for AWS Polly
use golem_tts::golem::tts::types::{AudioFormat, TtsError as WitTtsError};

/// Convert AudioFormat to AWS Polly output format
pub fn audio_format_to_polly(format: AudioFormat) -> &'static str {
    match format {
        AudioFormat::Mp3 => "mp3",
        AudioFormat::Pcm => "pcm",
        AudioFormat::Ogg => "ogg_vorbis",
        AudioFormat::Wav => "pcm", // Polly returns raw PCM, can be wrapped as WAV
        _ => "mp3",                // Default fallback
    }
}

/// Map voice to recommended engine (neural vs standard)
pub fn recommended_engine_for_voice(voice_id: &str) -> &'static str {
    // All modern voices should use neural
    // Legacy voices like Ivy, Joanna (v1) might need standard
    match voice_id {
        "Ivy" | "Joey" | "Justin" | "Kevin" | "Kimberly" | "Salli" => "standard",
        _ => "neural",
    }
}

/// Convert sample rate to valid Polly format
pub fn validate_sample_rate(rate: Option<u32>, format: &str) -> u32 {
    match format {
        "mp3" => 24000,                 // MP3 supports 8000, 16000, 22050, 24000
        "ogg_vorbis" => 24000,          // OGG supports 8000, 16000, 22050, 24000
        "pcm" => rate.unwrap_or(16000), // PCM supports 8000, 16000, 24000
        _ => 24000,
    }
}

/// Parse AWS Polly error response
pub fn parse_polly_error(status: u16, body: &str) -> WitTtsError {
    // Try to parse JSON error response
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(body) {
        if let Some(message) = json.get("message").and_then(|m| m.as_str()) {
            return match status {
                400 => {
                    if message.contains("Text is too long") {
                        WitTtsError::TextTooLong(3000)
                    } else if message.contains("SSML") {
                        WitTtsError::InvalidSsml(message.to_string())
                    } else {
                        WitTtsError::InvalidText(message.to_string())
                    }
                }
                403 => WitTtsError::Unauthorized("Invalid AWS credentials".to_string()),
                404 => {
                    if message.contains("voice") {
                        WitTtsError::VoiceNotFound(message.to_string())
                    } else {
                        WitTtsError::ServiceUnavailable(message.to_string())
                    }
                }
                429 => WitTtsError::RateLimited(60),
                503 => WitTtsError::ServiceUnavailable("AWS Polly unavailable".to_string()),
                _ => WitTtsError::SynthesisFailed(format!("AWS error {}: {}", status, message)),
            };
        }
    }

    // Fallback to generic error
    match status {
        400 => WitTtsError::InvalidText(body.to_string()),
        403 => WitTtsError::Unauthorized("Invalid AWS credentials".to_string()),
        404 => WitTtsError::VoiceNotFound("Voice not found".to_string()),
        429 => WitTtsError::RateLimited(60),
        503 => WitTtsError::ServiceUnavailable("AWS Polly unavailable".to_string()),
        _ => WitTtsError::SynthesisFailed(format!("HTTP {}: {}", status, body)),
    }
}

/// Validate text length for AWS Polly
pub fn validate_text_length(text: &str) -> Result<(), WitTtsError> {
    let len = text.len();
    if len == 0 {
        return Err(WitTtsError::InvalidText("Text cannot be empty".to_string()));
    }
    if len > 3000 {
        return Err(WitTtsError::TextTooLong(3000));
    }
    Ok(())
}

/// Validate SSML for AWS Polly  
pub fn is_valid_ssml(text: &str) -> bool {
    text.trim_start().starts_with("<speak>") && text.trim_end().ends_with("</speak>")
}
