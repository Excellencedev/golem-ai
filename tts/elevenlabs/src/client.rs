use golem_tts::error::Error;
use golem_tts::exports::golem::tts::voices::{LanguageInfo as WitLanguageInfo, VoiceFilter as WitVoiceFilter, VoiceInfo as WitVoiceInfo};
use golem_tts::exports::golem::tts::streaming::{StreamSession as WitStreamSession, StreamStatus as WitStreamStatus};
use golem_tts::exports::golem::tts::advanced::AudioSample as WitAudioSample;
use golem_tts::golem::tts::types::{
    TextInput as WitTextInput, SynthesisResult as WitSynthesisResult,
    AudioChunk as WitAudioChunk, TtsError as WitTtsError,
};
use golem_tts::exports::golem::tts::synthesis::SynthesisOptions as WitSynthesisOptions;
use golem_tts::http::WstdHttpClient;
use log::trace;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use uuid::Uuid;

static STREAM_SESSIONS: OnceLock<Mutex<HashMap<String, StreamSessionState>>> = OnceLock::new();

struct StreamSessionState {
    chunks: Vec<Vec<u8>>,
    current_index: usize,
    finished: bool,
    status: WitStreamStatus,
}

pub struct ElevenLabsClient {
    api_key: String,
    base_url: String,
}

impl ElevenLabsClient {
    pub fn new(api_key: String) -> Self {
        let base_url = std::env::var("ELEVENLABS_BASE_URL")
            .unwrap_or_else(|_| "https://api.elevenlabs.io/v1".to_string());
        
        Self { api_key, base_url }
    }

    pub fn list_voices(&self, _filter: Option<WitVoiceFilter>) -> Result<Vec<WitVoiceInfo>, WitTtsError> {
        trace!("Listing voices from ElevenLabs");
        let http = WstdHttpClient::new();
        
        let response = http
            .get(&format!("{}/voices", self.base_url))
            .header("xi-api-key", &self.api_key)
            .send()?
            .error_for_status()?;

        let voices_response: VoicesResponse = response.json()?;
        Ok(voices_response.voices.into_iter().map(|v| v.into()).collect())
    }

    pub fn get_voice(&self, voice_id: String) -> Result<WitVoiceInfo, WitTtsError> {
        trace!("Getting voice {} from ElevenLabs", voice_id);
        let http = WstdHttpClient::new();
        
        let response = http
            .get(&format!("{}/voices/{}", self.base_url, voice_id))
            .header("xi-api-key", &self.api_key)
            .send()?
            .error_for_status()?;

        let voice: ElevenLabsVoice = response.json()?;
        Ok(voice.into())
    }

    pub fn search_voices(&self, query: String, filter: Option<WitVoiceFilter>) -> Result<Vec<WitVoiceInfo>, WitTtsError> {
        let all_voices = self.list_voices(filter)?;
        let query_lower = query.to_lowercase();
        
        Ok(all_voices
            .into_iter()
            .filter(|v| {
                v.name.to_lowercase().contains(&query_lower) ||
                v.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&query_lower))
            })
            .collect())
    }

    pub fn list_languages(&self) -> Result<Vec<WitLanguageInfo>, WitTtsError > {
        // ElevenLabs supports these languages
        Ok(vec![
            WitLanguageInfo {
                code: "en".to_string(),
                name: "English".to_string(),
                native_name: "English".to_string(),
                voice_count: 0,
            },
            WitLanguageInfo {
                code: "de".to_string(),
                name: "German".to_string(),
                native_name: "Deutsch".to_string(),
                voice_count: 0,
            },
            WitLanguageInfo {
                code: "pl".to_string(),
                name: "Polish".to_string(),
                native_name: "Polski".to_string(),
                voice_count: 0,
            },
            WitLanguageInfo {
                code: "es".to_string(),
                name: "Spanish".to_string(),
                native_name: "Español".to_string(),
                voice_count: 0,
            },
            WitLanguageInfo {
                code: "it".to_string(),
                name: "Italian".to_string(),
                native_name: "Italiano".to_string(),
                voice_count: 0,
            },
            WitLanguageInfo {
                code: "fr".to_string(),
                name: "French".to_string(),
                native_name: "Français".to_string(),
                voice_count: 0,
            },
            WitLanguageInfo {
                code: "pt".to_string(),
                name: "Portuguese".to_string(),
                native_name: "Português".to_string(),
                voice_count: 0,
            },
            WitLanguageInfo {
                code: "hi".to_string(),
                name: "Hindi".to_string(),
                native_name: "हिन्दी".to_string(),
                voice_count: 0,
            },
        ])
    }

    pub fn synthesize(&self, input: WitTextInput, options: WitSynthesisOptions) -> Result<WitSynthesisResult, WitTtsError> {
        trace!("Synthesizing speech with ElevenLabs voice {}", options.voice_id);
        let http = WstdHttpClient::new();

        let request_body = SynthesizeRequest {
            text: input.content.clone(),
            model_id: options.model_version.clone().unwrap_or_else(|| "eleven_monolingual_v1".to_string()),
            voice_settings: options.voice_settings.as_ref().map(|vs| VoiceSettings {
                stability: vs.stability.unwrap_or(0.5),
                similarity_boost: vs.similarity.unwrap_or(0.75),
                style: vs.style.unwrap_or(0.0),
                use_speaker_boost: true,
            }),
        };

        let response = http
            .post(&format!("{}/text-to-speech/{}", self.base_url, options.voice_id))
            .header("xi-api-key", &self.api_key)
            .header("accept", "audio/mpeg")
            .json(&request_body)?
            .send()?
            .error_for_status()?;

        let audio_data = response.bytes().to_vec();
        let char_count = input.content.len() as u32;

        Ok(WitSynthesisResult {
            audio_data: audio_data.clone(),
            metadata: golem_tts::golem::tts::types::SynthesisMetadata {
                duration_seconds: (char_count as f32 * 0.05),
                character_count: char_count,
                word_count: input.content.split_whitespace().count() as u32,
                audio_size_bytes: audio_data.len() as u32,
                request_id: Uuid::new_v4().to_string(),
                provider_info: Some("ElevenLabs".to_string()),
            },
        })
    }

    pub fn synthesize_batch(&self, inputs: Vec<WitTextInput>, options: WitSynthesisOptions) -> Result<Vec<WitSynthesisResult>, WitTtsError> {
        inputs
            .into_iter()
            .map(|input| self.synthesize(input, options.clone()))
            .collect()
    }

    pub fn create_stream(&self, options: WitSynthesisOptions) -> Result<WitStreamSession, WitTtsError> {
        let session_id = Uuid::new_v4().to_string();
        
        let sessions = STREAM_SESSIONS.get_or_init(|| Mutex::new(HashMap::new()));
        let mut sessions = sessions.lock().unwrap();
        
        sessions.insert(session_id.clone(), StreamSessionState {
            chunks: vec![],
            current_index: 0,
            finished: false,
            status: WitStreamStatus::Ready,
        });

        Ok(WitStreamSession {
            session_id,
            status: WitStreamStatus::Ready,
            pending_chunks: 0,
        })
    }

    pub fn stream_send_text(&self, session_id: String, input: WitTextInput) -> Result<(), WitTtsError> {
        let sessions = STREAM_SESSIONS.get().ok_or_else(|| {
            WitTtsError::InvalidConfiguration("Stream sessions not initialized".to_string())
        })?;
        let mut sessions = sessions.lock().unwrap();
        
        let session = sessions.get_mut(&session_id).ok_or_else(|| {
            WitTtsError::InvalidConfiguration(format!("Session {} not found", session_id))
        })?;

        // Synthesize immediately for simplicity (ElevenLabs streaming requires WebSocket)
        let http = WstdHttpClient::new();
        let model_id = std::env::var("ELEVENLABS_MODEL_VERSION")
            .unwrap_or_else(|_| "eleven_monolingual_v1".to_string());

        let request_body = SynthesizeRequest {
            text: input.content,
            model_id,
            voice_settings: None,
        };

        let response = http
            .post(&format!("{}/text-to-speech/placeholder", self.base_url))
            .header("xi-api-key", &self.api_key)
            .header("accept", "audio/mpeg")
            .json(&request_body)?
            .send()?
            .error_for_status()?;

        session.chunks.push(response.bytes().to_vec());
        session.status = WitStreamStatus::Processing;

        Ok(())
    }

    pub fn stream_finish(&self, session_id: String) -> Result<(), WitTtsError> {
        let sessions = STREAM_SESSIONS.get().ok_or_else(|| {
            WitTtsError::InvalidConfiguration("Stream sessions not initialized".to_string())
        })?;
        let mut sessions = sessions.lock().unwrap();
        
        let session = sessions.get_mut(&session_id).ok_or_else(|| {
            WitTtsError::InvalidConfiguration(format!("Session {} not found", session_id))
        })?;

        session.finished = true;
        session.status = WitStreamStatus::Finished;

        Ok(())
    }

    pub fn stream_receive_chunk(&self, session_id: String) -> Result<Option<WitAudioChunk>, WitTtsError> {
        let sessions = STREAM_SESSIONS.get().ok_or_else(|| {
            WitTtsError::InvalidConfiguration("Stream sessions not initialized".to_string())
        })?;
        let mut sessions = sessions.lock().unwrap();
        
        let session = sessions.get_mut(&session_id).ok_or_else(|| {
            WitTtsError::InvalidConfiguration(format!("Session {} not found", session_id))
        })?;

        if session.current_index < session.chunks.len() {
            let chunk_data = session.chunks[session.current_index].clone();
            let is_final = session.current_index == session.chunks.len() - 1 && session.finished;
            let seq_num = session.current_index as u32;
            session.current_index += 1;

            Ok(Some(WitAudioChunk {
                data: chunk_data,
                sequence_number: seq_num,
                is_final,
                timing_info: None,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn stream_has_pending(&self, session_id: String) -> Result<bool, WitTtsError> {
        let sessions = STREAM_SESSIONS.get().ok_or_else(|| {
            WitTtsError::InvalidConfiguration("Stream sessions not initialized".to_string())
        })?;
        let sessions = sessions.lock().unwrap();
        
        let session = sessions.get(&session_id).ok_or_else(|| {
            WitTtsError::InvalidConfiguration(format!("Session {} not found", session_id))
        })?;

        Ok(session.current_index < session.chunks.len())
    }

    pub fn stream_get_status(&self, session_id: String) -> Result<WitStreamStatus, WitTtsError> {
        let sessions = STREAM_SESSIONS.get().ok_or_else(|| {
            WitTtsError::InvalidConfiguration("Stream sessions not initialized".to_string())
        })?;
        let sessions = sessions.lock().unwrap();
        
        let session = sessions.get(&session_id).ok_or_else(|| {
            WitTtsError::InvalidConfiguration(format!("Session {} not found", session_id))
        })?;

        Ok(session.status)
    }

    pub fn stream_close(&self, session_id: String) -> Result<(), WitTtsError> {
        let sessions = STREAM_SESSIONS.get().ok_or_else(|| {
            WitTtsError::InvalidConfiguration("Stream sessions not initialized".to_string())
        })?;
        let mut sessions = sessions.lock().unwrap();
        
        sessions.remove(&session_id);
        Ok(())
    }

    pub fn create_voice_clone(&self, name: String, audio_samples: Vec<WitAudioSample>, description: Option<String>) -> Result<String, WitTtsError> {
        trace!("Creating voice clone: {}", name);
        
        // In a real implementation, this would use multipart/form-data
        // For now, return unsupported
        Err(WitTtsError::UnsupportedOperation("Voice cloning requires multipart upload, not yet implemented".to_string()))
    }

    pub fn convert_voice(&self, _input_audio: Vec<u8>, _target_voice_id: String, _preserve_timing: Option<bool>) -> Result<Vec<u8>, WitTtsError> {
        Err(WitTtsError::UnsupportedOperation("Voice conversion not yet implemented".to_string()))
    }

    pub fn generate_sound_effect(&self, description: String, duration_seconds: Option<f32>, _style_influence: Option<f32>) -> Result<Vec<u8>, WitTtsError> {
        trace!("Generating sound effect: {}", description);
        let http = WstdHttpClient::new();

        #[derive(Serialize)]
        struct SoundEffectRequest {
            text: String,
            duration_seconds: Option<f32>,
        }

        let request_body = SoundEffectRequest {
            text: description,
            duration_seconds,
        };

        let response = http
            .post(&format!("{}/sound-generation", self.base_url))
            .header("xi-api-key", &self.api_key)
            .json(&request_body)?
            .send()?
            .error_for_status()?;

        Ok(response.bytes().to_vec())
    }
}

#[derive(Debug, Deserialize)]
struct VoicesResponse {
    voices: Vec<ElevenLabsVoice>,
}

#[derive(Debug, Deserialize)]
struct ElevenLabsVoice {
    voice_id: String,
    name: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    labels: HashMap<String, String>,
    #[serde(default)]
    preview_url: Option<String>,
}

#[derive(Debug, Serialize)]
struct SynthesizeRequest {
    text: String,
    model_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    voice_settings: Option<VoiceSettings>,
}

#[derive(Debug, Serialize)]
struct VoiceSettings {
    stability: f32,
    similarity_boost: f32,
    style: f32,
    use_speaker_boost: bool,
}
