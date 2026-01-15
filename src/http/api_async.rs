//! Async API trait and implementations

use crate::{ListResponse, Result, VersionResponse};
use async_trait::async_trait;

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

    /// List locally available models (async)
    ///
    /// Returns a list of models installed on the Ollama server with their details.
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
    /// let response = client.list_models().await?;
    /// for model in &response.models {
    ///     println!("Model: {}", model.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    async fn list_models(&self) -> Result<ListResponse>;
}

#[async_trait]
impl OllamaApiAsync for OllamaClient {
    async fn version(&self) -> Result<VersionResponse> {
        let url = self.config.url(Endpoints::VERSION);
        self.get_with_retry(&url).await
    }

    async fn list_models(&self) -> Result<ListResponse> {
        let url = self.config.url(Endpoints::TAGS);
        self.get_with_retry(&url).await
    }
}
