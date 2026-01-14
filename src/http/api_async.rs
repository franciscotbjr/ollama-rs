//! Async API trait and implementations

use crate::{Error, Result, VersionResponse};
use async_trait::async_trait;
use std::time::Duration;

use super::endpoints::Endpoints;
use super::OllamaClient;

/// Async API operations trait
///
/// This trait defines all asynchronous methods for interacting with the Ollama API.
/// All methods return futures that can be awaited.
///
/// # Thread Safety
///
/// Implementations of this trait must be `Send + Sync` to support concurrent usage
/// across async tasks.
///
/// # Examples
///
/// ```no_run
/// use ollama_oxide::{OllamaClient, OllamaApiAsync};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = OllamaClient::default()?;
///     let version = client.version().await?;
///     println!("Ollama version: {}", version.version);
///     Ok(())
/// }
/// ```
#[async_trait]
pub trait OllamaApiAsync: Send + Sync {
    /// Get Ollama server version (async)
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
    /// use ollama_oxide::{OllamaClient, OllamaApiAsync};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OllamaClient::default()?;
    /// let response = client.version().await?;
    /// println!("Version: {}", response.version);
    /// # Ok(())
    /// # }
    /// ```
    async fn version(&self) -> Result<VersionResponse>;
}

#[async_trait]
impl OllamaApiAsync for OllamaClient {
    async fn version(&self) -> Result<VersionResponse> {
        let url = self.config.url(Endpoints::VERSION);

        for attempt in 0..=self.config.max_retries {
            match self.client.get(&url).send().await {
                Ok(response) => {
                    // Check if response indicates an error (5xx should retry, 4xx should not)
                    if response.status().is_server_error() && attempt < self.config.max_retries {
                        // Retry on server errors
                        tokio::time::sleep(Duration::from_millis(100 * (attempt as u64 + 1))).await;
                        continue;
                    }

                    let version_response = response.json::<VersionResponse>().await?;
                    return Ok(version_response);
                }
                Err(_e) => {
                    if attempt < self.config.max_retries {
                        // Exponential backoff
                        tokio::time::sleep(Duration::from_millis(100 * (attempt as u64 + 1))).await;
                    }
                }
            }
        }

        Err(Error::MaxRetriesExceededError(self.config.max_retries))
    }
}
