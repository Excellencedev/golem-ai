use crate::exports::golem::tts::types::TtsError as WitTtsError;
use std::env;

/// Helper to read required configuration from environment
pub fn with_config_key<F, R>(key: &str, error_fn: fn(WitTtsError) -> R, f: F) -> R
where
    F: FnOnce(String) -> R,
{
    match env::var(key) {
        Ok(value) if !value.is_empty() => f(value),
        _ => error_fn(WitTtsError::InvalidConfiguration(format!(
            "Required environment variable {} not set or empty",
            key
        ))),
    }
}

/// Helper to read optional configuration from environment
pub fn get_optional_config(key: &str) -> Option<String> {
    env::var(key).ok().filter(|v| !v.is_empty())
}

/// Helper to read configuration with a default value
pub fn get_config_or_default(key: &str, default: &str) -> String {
    env::var(key)
        .ok()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| default.to_string())
}

/// Parse integer configuration with validation
pub fn parse_config_u32(key: &str, default: u32) -> u32 {
    env::var(key)
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(default)
}
