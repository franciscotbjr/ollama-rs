//! Async API trait and implementations

use crate::{
    CopyRequest, DeleteRequest, EmbedRequest, EmbedResponse, GenerateRequest, GenerateResponse,
    ListResponse, PsResponse, Result, ShowRequest, ShowResponse, VersionResponse,
};
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

    /// Delete a model (async)
    ///
    /// Permanently removes a model from the Ollama server. This operation
    /// cannot be undone.
    ///
    /// # Arguments
    ///
    /// * `request` - Delete request containing the model name to delete
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Model doesn't exist (404)
    /// - Network request fails
    /// - Maximum retry attempts exceeded
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::{OllamaClient, OllamaApiAsync, DeleteRequest};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OllamaClient::default()?;
    /// let request = DeleteRequest::new("llama3.1-backup");
    /// client.delete_model(&request).await?;
    /// println!("Model deleted successfully!");
    /// # Ok(())
    /// # }
    /// ```
    async fn delete_model(&self, request: &DeleteRequest) -> Result<()>;

    /// Show detailed information about a model (async)
    ///
    /// Retrieves comprehensive metadata including parameters,
    /// license, capabilities, and model-specific configuration.
    ///
    /// # Arguments
    ///
    /// * `request` - ShowRequest containing the model name
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The model does not exist (404)
    /// - Network error occurs
    /// - Response cannot be deserialized
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::{OllamaClient, OllamaApiAsync, ShowRequest};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OllamaClient::default()?;
    ///
    /// // Basic request
    /// let request = ShowRequest::new("llama3.1");
    /// let response = client.show_model(&request).await?;
    /// println!("Capabilities: {:?}", response.capabilities);
    ///
    /// // Verbose request
    /// let verbose_request = ShowRequest::verbose("llama3.1");
    /// let verbose_response = client.show_model(&verbose_request).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn show_model(&self, request: &ShowRequest) -> Result<ShowResponse>;

    /// Generate embeddings for text (async)
    ///
    /// Creates vector embeddings representing the input text(s).
    /// Embeddings are useful for semantic search, similarity comparison,
    /// and machine learning tasks.
    ///
    /// # Arguments
    ///
    /// * `request` - Embed request containing model name and input text(s)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Model doesn't exist (404)
    /// - Input exceeds context window and truncate is false
    /// - Network request fails
    /// - Maximum retry attempts exceeded
    ///
    /// # Examples
    ///
    /// Single text embedding:
    /// ```no_run
    /// use ollama_oxide::{OllamaClient, OllamaApiAsync, EmbedRequest};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OllamaClient::default()?;
    /// let request = EmbedRequest::new("nomic-embed-text", "Hello, world!");
    /// let response = client.embed(&request).await?;
    /// println!("Embedding dimensions: {:?}", response.dimensions());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Multiple text embeddings:
    /// ```no_run
    /// use ollama_oxide::{OllamaClient, OllamaApiAsync, EmbedRequest, EmbedInput};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OllamaClient::default()?;
    /// let request = EmbedRequest::new(
    ///     "nomic-embed-text",
    ///     EmbedInput::multiple(["First text", "Second text"])
    /// );
    /// let response = client.embed(&request).await?;
    /// println!("Got {} embeddings", response.len());
    /// # Ok(())
    /// # }
    /// ```
    async fn embed(&self, request: &EmbedRequest) -> Result<EmbedResponse>;

    /// Generate text completion (async, non-streaming)
    ///
    /// Generates a text completion for the provided prompt.
    /// This method uses non-streaming mode.
    ///
    /// # Arguments
    ///
    /// * `request` - Generate request containing model, prompt, and options
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Model doesn't exist (404)
    /// - Network request fails
    /// - Maximum retry attempts exceeded
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::{OllamaClient, OllamaApiAsync, GenerateRequest};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OllamaClient::default()?;
    /// let request = GenerateRequest::new("qwen3:0.6b", "Why is the sky blue?");
    /// let response = client.generate(&request).await?;
    /// println!("Response: {:?}", response.text());
    /// # Ok(())
    /// # }
    /// ```
    async fn generate(&self, request: &GenerateRequest) -> Result<GenerateResponse>;
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

    async fn delete_model(&self, request: &DeleteRequest) -> Result<()> {
        let url = self.config.url(Endpoints::DELETE);
        self.delete_empty_with_retry(&url, request).await
    }

    async fn show_model(&self, request: &ShowRequest) -> Result<ShowResponse> {
        let url = self.config.url(Endpoints::SHOW);
        self.post_with_retry(&url, request).await
    }

    async fn embed(&self, request: &EmbedRequest) -> Result<EmbedResponse> {
        let url = self.config.url(Endpoints::EMBED);
        self.post_with_retry(&url, request).await
    }

    async fn generate(&self, request: &GenerateRequest) -> Result<GenerateResponse> {
        let url = self.config.url(Endpoints::GENERATE);
        self.post_with_retry(&url, request).await
    }
}
