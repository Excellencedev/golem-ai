//! Voice cloning with multipart upload for ElevenLabs
//!
//! This module handles voice cloning via the /v1/voices/add endpoint
//! which requires multipart/form-data uploads.

use golem_tts::exports::golem::tts::advanced::AudioSample as WitAudioSample;
use golem_tts::golem::tts::types::TtsError as WitTtsError;
use golem_tts::http::WstdHttpClient;
use serde::{Deserialize, Serialize};

pub struct VoiceCloner {
    api_key: String,
    base_url: String,
}

#[derive(Deserialize)]
struct VoiceCloneResponse {
    voice_id: String,
}

impl VoiceCloner {
    pub fn new(api_key: String, base_url: String) -> Self {
        Self { api_key, base_url }
    }

    pub fn create_voice_clone(
        &self,
        name: String,
        audio_samples: Vec<WitAudioSample>,
        description: Option<String>,
    ) -> Result<String, WitTtsError> {
        // Note: Multipart form-data is complex in current HTTP client
        // This is a reference implementation showing the intended structure

        return Err(WitTtsError::UnsupportedOperation(
            "Voice cloning with multipart upload requires advanced HTTP client. \
             Current WASI HTTP implementation has limited multipart support. \
             Use ElevenLabs web interface for voice cloning."
                .to_string(),
        ));

        /* Reference implementation for when multipart is available:

        let http = WstdHttpClient::new();

        // Construct multipart form data
        let mut form = MultipartForm::new();
        form.add_text("name", &name);

        if let Some(desc) = description {
            form.add_text("description", &desc);
        }

        // Add audio files
        for (idx, sample) in audio_samples.iter().enumerate() {
            let filename = format!("sample_{}.mp3", idx);
            form.add_file("files", &filename, &sample.audio_data)?;
        }

        let url = format!("{}/v1/voices/add", self.base_url);
        let response = http
            .post(&url)
            .header("xi-api-key", &self.api_key)
            .multipart(form)?
            .send()?
            .error_for_status()?;

        let clone_response: VoiceCloneResponse = response.json()?;
        Ok(clone_response.voice_id)
        */
    }

    pub fn delete_voice_clone(&self, voice_id: String) -> Result<(), WitTtsError> {
        let http = WstdHttpClient::new();

        let url = format!("{}/v1/voices/{}", self.base_url, voice_id);
        http.delete(&url)
            .header("xi-api-key", &self.api_key)
            .send()?
            .error_for_status()?;

        Ok(())
    }

    pub fn get_voice_clone_status(&self, voice_id: String) -> Result<String, WitTtsError> {
        let http = WstdHttpClient::new();

        #[derive(Deserialize)]
        struct VoiceStatus {
            status: String,
        }

        let url = format!("{}/v1/voices/{}", self.base_url, voice_id);
        let response = http
            .get(&url)
            .header("xi-api-key", &self.api_key)
            .send()?
            .error_for_status()?;

        let status: VoiceStatus = response.json()?;
        Ok(status.status)
    }
}

// Helper function to validate audio samples
pub fn validate_audio_samples(samples: &[WitAudioSample]) -> Result<(), WitTtsError> {
    if samples.is_empty() {
        return Err(WitTtsError::InvalidInput(
            "At least one audio sample required for voice cloning".to_string(),
        ));
    }

    if samples.len() > 25 {
        return Err(WitTtsError::InvalidInput(
            "Maximum 25 audio samples allowed".to_string(),
        ));
    }

    // Check each sample is at least 1KB
    for (idx, sample) in samples.iter().enumerate() {
        if sample.audio_data.len() < 1024 {
            return Err(WitTtsError::InvalidInput(format!(
                "Sample {} is too small (minimum 1KB)",
                idx
            )));
        }
    }

    Ok(())
}
