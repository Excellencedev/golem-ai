//! Simple token-based authentication for Google Cloud TTS
//! Users should provide pre-generated OAuth2 access tokens

use golem_tts::golem::tts::types::TtsError as WitTtsError;

/// Get access token from environment variable
pub fn get_access_token_from_env() -> Result<String, WitTtsError> {
    std::env::var("GOOGLE_ACCESS_TOKEN").map_err(|_| {
        WitTtsError::InternalError(
            "GOOGLE_ACCESS_TOKEN environment variable not set. \
             Please set it to a valid OAuth2 access token. \
             You can generate one using: gcloud auth print-access-token"
                .to_string(),
        )
    })
}

/// Get project ID from environment variable
pub fn get_project_id_from_env() -> Result<String, WitTtsError> {
    std::env::var("GOOGLE_PROJECT_ID").map_err(|_| {
        WitTtsError::InternalError("GOOGLE_PROJECT_ID environment variable not set".to_string())
    })
}
