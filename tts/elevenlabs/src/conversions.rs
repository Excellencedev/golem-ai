use super::client::ElevenLabsVoice;
use golem_tts::exports::golem::tts::voices::VoiceInfo as WitVoiceInfo;
use golem_tts::golem::tts::types::{
    AudioFormat, TtsError as WitTtsError, VoiceGender, VoiceQuality,
};

impl From<ElevenLabsVoice> for WitVoiceInfo {
    fn from(voice: ElevenLabsVoice) -> Self {
        let gender = voice
            .labels
            .get("gender")
            .and_then(|g| match g.as_str() {
                "male" => Some(VoiceGender::Male),
                "female" => Some(VoiceGender::Female),
                _ => Some(VoiceGender::Neutral),
            })
            .unwrap_or(VoiceGender::Neutral);

        let is_custom = voice
            .labels
            .get("use_case")
            .map_or(false, |u| u.contains("custom"));

        WitVoiceInfo {
            id: voice.voice_id,
            name: voice.name.clone(),
            language: "en".to_string(),
            additional_languages: vec![],
            gender,
            quality: VoiceQuality::Neural,
            description: voice.description,
            provider: "ElevenLabs".to_string(),
            sample_rate: 44100,
            is_custom,
            is_cloned: is_custom,
            preview_url: voice.preview_url,
            use_cases: voice
                .labels
                .get("use_case")
                .map(|u| vec![u.clone()])
                .unwrap_or_default(),
        }
    }
}

/// Convert AudioFormat to ElevenLabs output format
pub fn audio_format_to_elevenlabs(format: AudioFormat) -> &'static str {
    match format {
        AudioFormat::Mp3 => "mp3_44100_128",
        AudioFormat::Pcm => "pcm_44100",
        AudioFormat::Flac => "mp3_44100_128", // ElevenLabs doesn't support FLAC, fallback to MP3
        AudioFormat::Aac => "mp3_44100_128",  // Fallback to MP3
        AudioFormat::Wav => "pcm_44100",
        _ => "mp3_44100_128", // Default fallback
    }
}

/// Parse ElevenLabs error response
pub fn parse_elevenlabs_error(status: u16, body: &str) -> WitTtsError {
    // Try to parse JSON error response
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(body) {
        if let Some(detail) = json.get("detail").and_then(|d| d.as_str()) {
            return match status {
                400 => WitTtsError::InvalidText(detail.to_string()),
                401 => WitTtsError::Unauthorized(detail.to_string()),
                403 => WitTtsError::AccessDenied(detail.to_string()),
                422 => WitTtsError::InvalidConfiguration(detail.to_string()),
                429 => WitTtsError::RateLimited(60),
                _ => WitTtsError::SynthesisFailed(format!("HTTP {}: {}", status, detail)),
            };
        }
    }

    // Fallback to generic error
    match status {
        400 => WitTtsError::InvalidText(body.to_string()),
        401 => WitTtsError::Unauthorized("Invalid API key".to_string()),
        403 => WitTtsError::AccessDenied("Access denied".to_string()),
        422 => WitTtsError::InvalidConfiguration(body.to_string()),
        429 => WitTtsError::RateLimited(60),
        503 => WitTtsError::ServiceUnavailable("ElevenLabs API unavailable".to_string()),
        _ => WitTtsError::SynthesisFailed(format!("HTTP {}: {}", status, body)),
    }
}

/// Validate voice settings for ElevenLabs
pub fn validate_voice_settings(
    stability: Option<f32>,
    similarity: Option<f32>,
    style: Option<f32>,
) -> Result<(), WitTtsError> {
    if let Some(s) = stability {
        if !(0.0..=1.0).contains(&s) {
            return Err(WitTtsError::InvalidConfiguration(format!(
                "Stability must be between 0.0 and 1.0, got {}",
                s
            )));
        }
    }

    if let Some(s) = similarity {
        if !(0.0..=1.0).contains(&s) {
            return Err(WitTtsError::InvalidConfiguration(format!(
                "Similarity must be between 0.0 and 1.0, got {}",
                s
            )));
        }
    }

    if let Some(s) = style {
        if !(0.0..=1.0).contains(&s) {
            return Err(WitTtsError::InvalidConfiguration(format!(
                "Style must be between 0.0 and 1.0, got {}",
                s
            )));
        }
    }

    Ok(())
}
