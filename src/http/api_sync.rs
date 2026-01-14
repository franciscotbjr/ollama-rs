//! Sync (blocking) API trait and implementations

use crate::{Result, VersionResponse};

use super::endpoints::Endpoints;
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
        let url = self.config.url(Endpoints::VERSION);
        self.get_blocking_with_retry(&url)
    }
}
