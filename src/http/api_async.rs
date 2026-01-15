//! Async API trait and implementations

use crate::{CopyRequest, ListResponse, PsResponse, Result, VersionResponse};
use async_trait::async_trait;

use super::OllamaClient;
use super::endpoints::Endpoints;

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

    /// Copy a model (async)
    ///
    /// Creates a copy of an existing model with a new name. This is useful for
    /// creating backups or variants of models without downloading them again.
    ///
    /// # Arguments
    ///
    /// * `request` - Copy request containing source and destination model names
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Source model doesn't exist (404)
    /// - Destination model name is invalid
    /// - Network request fails
    /// - Maximum retry attempts exceeded
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::{OllamaClient, OllamaApiAsync, CopyRequest};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OllamaClient::default()?;
    /// let request = CopyRequest::new("llama3.1", "llama3.1-backup");
    /// client.copy_model(&request).await?;
    /// println!("Model copied successfully!");
    /// # Ok(())
    /// # }
    /// ```
    async fn copy_model(&self, request: &CopyRequest) -> Result<()>;

    /// List currently running models (async)
    ///
    /// Returns a list of models that are currently loaded in memory and ready
    /// for inference. This includes information about VRAM usage, context length,
    /// and expiration time.
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
    /// let response = client.list_running_models().await?;
    /// for model in &response.models {
    ///     println!("Running: {} (VRAM: {:?})", model.model, model.size_vram);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    async fn list_running_models(&self) -> Result<PsResponse>;
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

    async fn copy_model(&self, request: &CopyRequest) -> Result<()> {
        let url = self.config.url(Endpoints::COPY);
        self.post_empty_with_retry(&url, request).await
    }

    async fn list_running_models(&self) -> Result<PsResponse> {
        let url = self.config.url(Endpoints::PS);
        self.get_with_retry(&url).await
    }
}
