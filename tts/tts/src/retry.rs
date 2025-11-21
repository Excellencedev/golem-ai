use log::trace;
use std::time::Duration;
use wstd::runtime::block_on;

const MAX_RETRIES: u32 = 10;
const INITIAL_BACKOFF_MS: u64 = 100;

/// Retry a function with exponential backoff
pub fn retry_with_backoff<F, T, E>(mut f: F) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    let max_retries = std::env::var("TTS_PROVIDER_MAX_RETRIES")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(MAX_RETRIES);

    let mut attempt = 0;
    loop {
        match f() {
            Ok(result) => return Ok(result),
            Err(e) if attempt >= max_retries => return Err(e),
            Err(e) => {
                attempt += 1;
                let backoff_ms = INITIAL_BACKOFF_MS * 2_u64.pow(attempt - 1);
                trace!("Retry attempt {} after {}ms", attempt, backoff_ms);
                
                // Sleep using wstd
                block_on(async {
                    wstd::time::sleep(Duration::from_millis(backoff_ms)).await;
                });
                
                // Continue to next iteration which will call f() again
                continue;
            }
        }
    }
}
