//! OAuth2 authentication for Google Cloud TTS using JWT and service account

use golem_tts::golem::tts::types::TtsError as WitTtsError;
use golem_tts::http::WstdHttpClient;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
pub struct ServiceAccountKey {
    #[serde(rename = "type")]
    pub key_type: String,
    pub project_id: String,
    pub private_key_id: String,
    pub private_key: String,
    pub client_email: String,
    pub client_id: String,
    pub auth_uri: String,
    pub token_uri: String,
}

#[derive(Serialize)]
struct JwtClaims {
    iss: String,
    scope: String,
    aud: String,
    exp: i64,
    iat: i64,
}

#[derive(Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: u32,
    pub token_type: String,
}

pub fn get_access_token(service_account: &ServiceAccountKey) -> Result<String, WitTtsError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| WitTtsError::InternalError(format!("Time error: {}", e)))?
        .as_secs() as i64;

    let claims = JwtClaims {
        iss: service_account.client_email.clone(),
        scope: "https://www.googleapis.com/auth/cloud-platform".to_string(),
        aud: service_account.token_uri.clone(),
        exp: now + 3600, // 1 hour
        iat: now,
    };

    let header = Header::new(Algorithm::RS256);
    let encoding_key = EncodingKey::from_rsa_pem(service_account.private_key.as_bytes())
        .map_err(|e| WitTtsError::AuthenticationError(format!("Invalid private key: {}", e)))?;

    let jwt = encode(&header, &claims, &encoding_key)
        .map_err(|e| WitTtsError::AuthenticationError(format!("JWT encoding failed: {}", e)))?;

    // Exchange JWT for access token
    let http = WstdHttpClient::new();

    #[derive(Serialize)]
    struct TokenRequest {
        grant_type: String,
        assertion: String,
    }

    let token_request = TokenRequest {
        grant_type: "urn:ietf:params:oauth:grant-type:jwt-bearer".to_string(),
        assertion: jwt,
    };

    let response = http
        .post(&service_account.token_uri)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&token_request)?
        .send()?
        .error_for_status()?;

    let token_response: TokenResponse = response.json()?;
    Ok(token_response.access_token)
}
