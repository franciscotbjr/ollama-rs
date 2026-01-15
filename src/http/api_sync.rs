//! Sync (blocking) API trait and implementations

use crate::{CopyRequest, ListResponse, Result, VersionResponse};

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

    /// List locally available models (blocking)
    ///
    /// Returns a list of models installed on the Ollama server with their details.
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
    /// let response = client.list_models_blocking()?;
    /// for model in &response.models {
    ///     println!("Model: {}", model.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn list_models_blocking(&self) -> Result<ListResponse>;

    /// Copy a model (blocking)
    ///
    /// Creates a copy of an existing model with a new name.
    /// This method blocks the current thread until the request completes.
    ///
    /// # Arguments
    ///
    /// * `request` - Copy request containing source and destination model names
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Source model doesn't exist
    /// - Destination model name is invalid
    /// - Network request fails
    /// - Maximum retry attempts exceeded
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::{OllamaClient, OllamaApiSync, CopyRequest};
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OllamaClient::default()?;
    /// let request = CopyRequest::new("llama3.1", "llama3.1-backup");
    /// client.copy_model_blocking(&request)?;
    /// println!("Model copied successfully!");
    /// # Ok(())
    /// # }
    /// ```
    fn copy_model_blocking(&self, request: &CopyRequest) -> Result<()>;
}

impl OllamaApiSync for OllamaClient {
    fn version_blocking(&self) -> Result<VersionResponse> {
        let url = self.config.url(Endpoints::VERSION);
        self.get_blocking_with_retry(&url)
    }

    fn list_models_blocking(&self) -> Result<ListResponse> {
        let url = self.config.url(Endpoints::TAGS);
        self.get_blocking_with_retry(&url)
    }

    fn copy_model_blocking(&self, request: &CopyRequest) -> Result<()> {
        let url = self.config.url(Endpoints::COPY);
        self.post_empty_blocking_with_retry(&url, request)
    }
}
