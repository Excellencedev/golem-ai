// Common retry utilities for TTS providers
use crate::exports::golem::tts::types::TtsError;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay_ms: 1000,
            max_delay_ms: 30000,
            backoff_multiplier: 2.0,
        }
    }
}

impl RetryConfig {
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn with_initial_delay(mut self, delay_ms: u64) -> Self {
        self.initial_delay_ms = delay_ms;
        self
    }

    pub fn calculate_delay(&self, attempt: u32) -> Duration {
        let delay = self.initial_delay_ms as f64 * self.backoff_multiplier.powi(attempt as i32);
        let capped = delay.min(self.max_delay_ms as f64);
        Duration::from_millis(capped as u64)
    }
}

/// Retry an operation with exponential backoff
pub fn retry_with_config<F, T>(config: &RetryConfig, mut operation: F) -> Result<T, TtsError>
where
    F: FnMut() -> Result<T, TtsError>,
{
    let mut last_error = None;

    for attempt in 0..=config.max_retries {
        match operation() {
            Ok(result) => return Ok(result),
            Err(e) => {
                // Check if error is retryable
                if !is_retryable(&e) || attempt >= config.max_retries {
                    return Err(e);
                }

                last_error = Some(e);
                let delay = config.calculate_delay(attempt);
                std::thread::sleep(delay);
            }
        }
    }

    Err(last_error.unwrap_or_else(|| TtsError::InternalError("Retry failed".to_string())))
}

/// Check if an error should be retried
fn is_retryable(error: &TtsError) -> bool {
    matches!(
        error,
        TtsError::NetworkError(_)
            | TtsError::RateLimited(_)
            | TtsError::ServiceUnavailable(_)
            | TtsError::InternalError(_)
    )
}

/// Retry with default configuration
pub fn retry_default<F, T>(operation: F) -> Result<T, TtsError>
where
    F: FnMut() -> Result<T, TtsError>,
{
    retry_with_config(&RetryConfig::default(), operation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_config_delay_calculation() {
        let config = RetryConfig::default();

        assert_eq!(config.calculate_delay(0), Duration::from_millis(1000));
        assert_eq!(config.calculate_delay(1), Duration::from_millis(2000));
        assert_eq!(config.calculate_delay(2), Duration::from_millis(4000));

        // Should cap at max_delay_ms
        assert_eq!(config.calculate_delay(10), Duration::from_millis(30000));
    }

    #[test]
    fn test_is_retryable() {
        assert!(is_retryable(&TtsError::NetworkError("test".to_string())));
        assert!(is_retryable(&TtsError::RateLimited(60)));
        assert!(is_retryable(&TtsError::ServiceUnavailable(
            "test".to_string()
        )));

        assert!(!is_retryable(&TtsError::InvalidText("test".to_string())));
        assert!(!is_retryable(&TtsError::VoiceNotFound("test".to_string())));
    }
}
