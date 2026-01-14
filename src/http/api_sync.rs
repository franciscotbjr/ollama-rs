//! Sync (blocking) API trait and implementations

use crate::{Error, Result, VersionResponse};
use std::time::Duration;

use super::OllamaClient;

/// Sync API operations trait
///
/// This trait defines all synchronous (blocking) methods for interacting with
/// the Ollama API. All methods block the current thread until completion.
///
/// # Thread Safety
///
/// Implementations of this trait must be `Send + Sync` to support concurrent usage
/// across threads.
///
/// # Examples
///
/// ```no_run
/// use ollama_oxide::{OllamaClient, OllamaApiSync};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = OllamaClient::default()?;
///     let version = client.version_blocking()?;
///     println!("Ollama version: {}", version.version);
///     Ok(())
/// }
/// ```
pub trait OllamaApiSync: Send + Sync {
    /// Get Ollama server version (blocking)
    ///
    /// This method blocks the current thread until the request completes.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Network request fails
    /// - Maximum retry attempts exceeded
    /// - Response cannot be deserialized
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::{OllamaClient, OllamaApiSync};
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OllamaClient::default()?;
    /// let response = client.version_blocking()?;
    /// println!("Version: {}", response.version);
    /// # Ok(())
    /// # }
    /// ```
    fn version_blocking(&self) -> Result<VersionResponse>;
}

impl OllamaApiSync for OllamaClient {
    fn version_blocking(&self) -> Result<VersionResponse> {
        let url = format!("{}/api/version", self.config.base_url);

        // Use reqwest blocking client
        let blocking_client = reqwest::blocking::Client::builder()
            .timeout(self.config.timeout)
            .build()?;

        for attempt in 0..=self.config.max_retries {
            match blocking_client.get(&url).send() {
                Ok(response) => {
                    // Check if response indicates an error (5xx should retry, 4xx should not)
                    if response.status().is_server_error() && attempt < self.config.max_retries {
                        // Retry on server errors
                        std::thread::sleep(Duration::from_millis(100 * (attempt as u64 + 1)));
                        continue;
                    }

                    let version_response = response.json::<VersionResponse>()?;
                    return Ok(version_response);
                }
                Err(_e) => {
                    if attempt < self.config.max_retries {
                        // Blocking sleep
                        std::thread::sleep(Duration::from_millis(100 * (attempt as u64 + 1)));
                    }
                }
            }
        }

        Err(Error::MaxRetriesExceededError(self.config.max_retries))
    }
}
