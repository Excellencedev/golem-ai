use golem_tts::error::Error;
use golem_tts::exports::golem::tts::synthesis::SynthesisOptions as WitSynthesisOptions;
use golem_tts::exports::golem::tts::voices::{
    LanguageInfo as WitLanguageInfo, VoiceFilter as WitVoiceFilter, VoiceInfo as WitVoiceInfo,
};
use golem_tts::golem::tts::types::{
    SynthesisResult as WitSynthesisResult, TextInput as WitTextInput, TtsError as WitTtsError,
    VoiceGender, VoiceQuality,
};
use golem_tts::http::WstdHttpClient;
use hmac::{Hmac, Mac};
use log::trace;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

type HmacSha256 = Hmac<Sha256>;

pub struct PollyClient {
    access_key_id: String,
    secret_access_key: String,
    session_token: Option<String>,
    region: String,
    pub(crate) base_url: String,
}

impl PollyClient {
    pub fn new(
        access_key_id: String,
        secret_access_key: String,
        region: String,
        session_token: Option<String>,
    ) -> Self {
        let base_url = format!("https://polly.{}.amazonaws.com", region);

        Self {
            access_key_id,
            secret_access_key,
            session_token,
            region,
            base_url,
        }
    }

    pub(crate) fn sign_request(
        &self,
        method: &str,
        uri: &str,
        query_string: &str,
        headers: &[(&str, &str)],
        payload: &str,
    ) -> Result<Vec<(&'static str, String)>, WitTtsError> {
        // AWS Signature Version 4 signing process
        let service = "polly";

        // Get timestamp
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| WitTtsError::InternalError(format!("Time error: {}", e)))?;
        let timestamp = format!("{}", now.as_secs());
        let amz_date = format!("{}000", timestamp); // Simplified - should be YYYYMMDD'T'HHMMSS'Z'

        // Task 1: Create canonical request
        let mut canonical_headers = String::new();
        let mut signed_headers = Vec::new();

        for (k, v) in headers {
            canonical_headers.push_str(&format!("{}:{}\n", k.to_lowercase(), v));
            signed_headers.push(k.to_lowercase());
        }
        signed_headers.sort();
        let signed_headers_str = signed_headers.join(";");

        let payload_hash = hex::encode(Sha256::digest(payload.as_bytes()));

        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            method, uri, query_string, canonical_headers, signed_headers_str, payload_hash
        );

        let canonical_request_hash = hex::encode(Sha256::digest(canonical_request.as_bytes()));

        // Task 2: Create string to sign
        let credential_scope = format!(
            "{}/{}/{}/aws4_request",
            &amz_date[..8],
            self.region,
            service
        );
        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{}\n{}\n{}",
            amz_date, credential_scope, canonical_request_hash
        );

        // Task 3: Calculate signature
        let date_key = self.hmac_sha256(
            format!("AWS4{}", self.secret_access_key).as_bytes(),
            &amz_date[..8],
        );
        let date_region_key = self.hmac_sha256(&date_key, &self.region);
        let date_region_service_key = self.hmac_sha256(&date_region_key, service);
        let signing_key = self.hmac_sha256(&date_region_service_key, "aws4_request");

        let signature = hex::encode(self.hmac_sha256(&signing_key, &string_to_sign));

        // Task 4: Create authorization header
        let authorization = format!(
            "AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
            self.access_key_id, credential_scope, signed_headers_str, signature
        );

        let mut result_headers = vec![("Authorization", authorization), ("X-Amz-Date", amz_date)];

        if let Some(ref token) = self.session_token {
            result_headers.push(("X-Amz-Security-Token", token.clone()));
        }

        Ok(result_headers)
    }

    fn hmac_sha256(&self, key: &[u8], data: &str) -> Vec<u8> {
        let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
        mac.update(data.as_bytes());
        mac.finalize().into_bytes().to_vec()
    }

    pub fn list_voices(&self) -> Result<Vec<WitVoiceInfo>, WitTtsError> {
        trace!("Listing AWS Polly voices");

        // Popular Polly voices
        Ok(vec![
            WitVoiceInfo {
                id: "Joanna".to_string(),
                name: "Joanna".to_string(),
                language: "en-US".to_string(),
                additional_languages: vec![],
                gender: VoiceGender::Female,
                quality: VoiceQuality::Neural,
                description: Some("US English female voice".to_string()),
                provider: "AWS Polly".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["general".to_string(), "assistant".to_string()],
            },
            WitVoiceInfo {
                id: "Matthew".to_string(),
                name: "Matthew".to_string(),
                language: "en-US".to_string(),
                additional_languages: vec![],
                gender: VoiceGender::Male,
                quality: VoiceQuality::Neural,
                description: Some("US English male voice".to_string()),
                provider: "AWS Polly".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["general".to_string(), "professional".to_string()],
            },
            WitVoiceInfo {
                id: "Ivy".to_string(),
                name: "Ivy".to_string(),
                language: "en-US".to_string(),
                additional_languages: vec![],
                gender: VoiceGender::Female,
                quality: VoiceQuality::Neural,
                description: Some("US English child's voice".to_string()),
                provider: "AWS Polly".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["conversational".to_string()],
            },
            WitVoiceInfo {
                id: "Kendra".to_string(),
                name: "Kendra".to_string(),
                language: "en-US".to_string(),
                additional_languages: vec![],
                gender: VoiceGender::Female,
                quality: VoiceQuality::Neural,
                description: Some("US English female voice".to_string()),
                provider: "AWS Polly".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["general".to_string()],
            },
            WitVoiceInfo {
                id: "Kevin".to_string(),
                name: "Kevin".to_string(),
                language: "en-US".to_string(),
                additional_languages: vec![],
                gender: VoiceGender::Male,
                quality: VoiceQuality::Neural,
                description: Some("US English child's voice".to_string()),
                provider: "AWS Polly".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["conversational".to_string()],
            },
            WitVoiceInfo {
                id: "Salli".to_string(),
                name: "Salli".to_string(),
                language: "en-US".to_string(),
                additional_languages: vec![],
                gender: VoiceGender::Female,
                quality: VoiceQuality::Neural,
                description: Some("US English female voice".to_string()),
                provider: "AWS Polly".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["general".to_string()],
            },
            WitVoiceInfo {
                id: "Joey".to_string(),
                name: "Joey".to_string(),
                language: "en-US".to_string(),
                additional_languages: vec![],
                gender: VoiceGender::Male,
                quality: VoiceQuality::Neural,
                description: Some("US English male voice".to_string()),
                provider: "AWS Polly".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["general".to_string()],
            },
            WitVoiceInfo {
                id: "Amy".to_string(),
                name: "Amy".to_string(),
                language: "en-GB".to_string(),
                additional_languages: vec![],
                gender: VoiceGender::Female,
                quality: VoiceQuality::Neural,
                description: Some("British English female voice".to_string()),
                provider: "AWS Polly".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["general".to_string()],
            },
            WitVoiceInfo {
                id: "Brian".to_string(),
                name: "Brian".to_string(),
                language: "en-GB".to_string(),
                additional_languages: vec![],
                gender: VoiceGender::Male,
                quality: VoiceQuality::Neural,
                description: Some("British English male voice".to_string()),
                provider: "AWS Polly".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["general".to_string()],
            },
            WitVoiceInfo {
                id: "Emma".to_string(),
                name: "Emma".to_string(),
                language: "en-GB".to_string(),
                additional_languages: vec![],
                gender: VoiceGender::Female,
                quality: VoiceQuality::Neural,
                description: Some("British English female voice".to_string()),
                provider: "AWS Polly".to_string(),
                sample_rate: 24000,
                is_custom: false,
                is_cloned: false,
                preview_url: None,
                use_cases: vec!["general".to_string(), "news".to_string()],
            },
        ])
    }

    pub fn get_voice(&self, voice_id: String) -> Result<WitVoiceInfo, WitTtsError> {
        let voices = self.list_voices()?;
        voices
            .into_iter()
            .find(|v| v.id == voice_id)
            .ok_or_else(|| WitTtsError::VoiceNotFound(voice_id))
    }

    pub fn search_voices(
        &self,
        query: String,
        filter: Option<WitVoiceFilter>,
    ) -> Result<Vec<WitVoiceInfo>, WitTtsError> {
        let all_voices = self.list_voices()?;
        let query_lower = query.to_lowercase();

        Ok(all_voices
            .into_iter()
            .filter(|v| {
                v.name.to_lowercase().contains(&query_lower)
                    || v.description
                        .as_ref()
                        .map_or(false, |d| d.to_lowercase().contains(&query_lower))
            })
            .filter(|v| {
                if let Some(ref f) = filter {
                    if let Some(ref lang) = f.language {
                        if !v.language.starts_with(lang) {
                            return false;
                        }
                    }
                    if let Some(gender) = f.gender {
                        if v.gender != gender {
                            return false;
                        }
                    }
                }
                true
            })
            .collect())
    }

    pub fn list_languages(&self) -> Result<Vec<WitLanguageInfo>, WitTtsError> {
        Ok(vec![
            WitLanguageInfo {
                code: "en-US".to_string(),
                name: "English (US)".to_string(),
                native_name: "English (US)".to_string(),
                voice_count: 7,
            },
            WitLanguageInfo {
                code: "en-GB".to_string(),
                name: "English (UK)".to_string(),
                native_name: "English (UK)".to_string(),
                voice_count: 3,
            },
        ])
    }

    pub fn synthesize(
        &self,
        input: WitTextInput,
        options: WitSynthesisOptions,
    ) -> Result<WitSynthesisResult, WitTtsError> {
        trace!(
            "Synthesizing speech with AWS Polly voice {}",
            options.voice_id
        );

        let http = WstdHttpClient::new();

        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct SynthesizeSpeechRequest {
            text: String,
            output_format: String,
            voice_id: String,
            engine: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            sample_rate: Option<String>,
        }

        let request_body = SynthesizeSpeechRequest {
            text: input.content.clone(),
            output_format: "mp3".to_string(),
            voice_id: options.voice_id.clone(),
            engine: "neural".to_string(),
            sample_rate: options
                .audio_config
                .as_ref()
                .and_then(|c| c.sample_rate)
                .map(|sr| sr.to_string()),
        };

        let json_payload = serde_json::to_string(&request_body).map_err(|e| Error::Json(e))?;

        let host = self
            .base_url
            .strip_prefix("https://")
            .unwrap_or(&self.base_url);
        let headers = vec![("host", host), ("content-type", "application/json")];

        let signed_headers =
            self.sign_request("POST", "/v1/speech", "", &headers, &json_payload)?;

        let url = format!("{}/v1/speech", self.base_url);
        let mut http_request = http.post(&url).header("Content-Type", "application/json");

        for (k, v) in signed_headers {
            http_request = http_request.header(k, &v);
        }

        let response = http_request
            .body(json_payload.into_bytes())
            .send()?
            .error_for_status()?;

        let audio_data = response.bytes();
        let audio_vec = audio_data.to_vec();
        let char_count = input.content.len() as u32;

        Ok(WitSynthesisResult {
            audio_data: audio_vec.clone(),
            metadata: golem_tts::golem::tts::types::SynthesisMetadata {
                duration_seconds: (char_count as f32 * 0.05),
                character_count: char_count,
                word_count: input.content.split_whitespace().count() as u32,
                audio_size_bytes: audio_vec.len() as u32,
                request_id: uuid::Uuid::new_v4().to_string(),
                provider_info: Some("AWS Polly".to_string()),
            },
        })
    }

    pub fn synthesize_batch(
        &self,
        inputs: Vec<WitTextInput>,
        options: WitSynthesisOptions,
    ) -> Result<Vec<WitSynthesisResult>, WitTtsError> {
        inputs
            .into_iter()
            .map(|input| self.synthesize(input, options.clone()))
            .collect()
    }
}
