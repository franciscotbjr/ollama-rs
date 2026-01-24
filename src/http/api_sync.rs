//! Sync (blocking) API trait and implementations

use crate::{
    CopyRequest, DeleteRequest, EmbedRequest, EmbedResponse, GenerateRequest, GenerateResponse,
    ListResponse, PsResponse, Result, ShowRequest, ShowResponse, VersionResponse,
};

use super::OllamaClient;
use super::endpoints::Endpoints;

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
    /// Creates a copy of an existing model with a new name. This is useful for
    /// creating backups or variants of models without downloading them again.
    /// This method blocks the current thread until the request completes.
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

    /// List currently running models (blocking)
    ///
    /// Returns a list of models that are currently loaded in memory and ready
    /// for inference. This includes information about VRAM usage, context length,
    /// and expiration time.
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
    /// let response = client.list_running_models_blocking()?;
    /// for model in &response.models {
    ///     println!("Running: {} (VRAM: {:?})", model.model, model.size_vram);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn list_running_models_blocking(&self) -> Result<PsResponse>;

    /// Delete a model (blocking)
    ///
    /// Permanently removes a model from the Ollama server. This operation
    /// cannot be undone.
    /// This method blocks the current thread until the request completes.
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
    /// use ollama_oxide::{OllamaClient, OllamaApiSync, DeleteRequest};
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OllamaClient::default()?;
    /// let request = DeleteRequest::new("llama3.1-backup");
    /// client.delete_model_blocking(&request)?;
    /// println!("Model deleted successfully!");
    /// # Ok(())
    /// # }
    /// ```
    fn delete_model_blocking(&self, request: &DeleteRequest) -> Result<()>;

    /// Show detailed information about a model (blocking)
    ///
    /// Retrieves comprehensive metadata including parameters,
    /// license, capabilities, and model-specific configuration.
    /// This method blocks the current thread until the request completes.
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
    /// use ollama_oxide::{OllamaClient, OllamaApiSync, ShowRequest};
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OllamaClient::default()?;
    ///
    /// // Basic request
    /// let request = ShowRequest::new("llama3.1");
    /// let response = client.show_model_blocking(&request)?;
    /// println!("Capabilities: {:?}", response.capabilities);
    ///
    /// // Verbose request
    /// let verbose_request = ShowRequest::verbose("llama3.1");
    /// let verbose_response = client.show_model_blocking(&verbose_request)?;
    /// # Ok(())
    /// # }
    /// ```
    fn show_model_blocking(&self, request: &ShowRequest) -> Result<ShowResponse>;

    /// Generate embeddings for text (blocking)
    ///
    /// Creates vector embeddings representing the input text(s).
    /// This method blocks the current thread until the request completes.
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
    /// ```no_run
    /// use ollama_oxide::{OllamaClient, OllamaApiSync, EmbedRequest};
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OllamaClient::default()?;
    /// let request = EmbedRequest::new("nomic-embed-text", "Hello, world!");
    /// let response = client.embed_blocking(&request)?;
    /// println!("Embedding dimensions: {:?}", response.dimensions());
    /// # Ok(())
    /// # }
    /// ```
    fn embed_blocking(&self, request: &EmbedRequest) -> Result<EmbedResponse>;

    /// Generate text completion (blocking, non-streaming)
    ///
    /// Generates a text completion for the provided prompt.
    /// This method blocks the current thread until completion.
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
    /// use ollama_oxide::{OllamaClient, OllamaApiSync, GenerateRequest};
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OllamaClient::default()?;
    /// let request = GenerateRequest::new("qwen3:0.6b", "Tell me a joke.");
    /// let response = client.generate_blocking(&request)?;
    /// println!("{}", response.text().unwrap_or("No response"));
    /// # Ok(())
    /// # }
    /// ```
    fn generate_blocking(&self, request: &GenerateRequest) -> Result<GenerateResponse>;
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

    fn list_running_models_blocking(&self) -> Result<PsResponse> {
        let url = self.config.url(Endpoints::PS);
        self.get_blocking_with_retry(&url)
    }

    fn delete_model_blocking(&self, request: &DeleteRequest) -> Result<()> {
        let url = self.config.url(Endpoints::DELETE);
        self.delete_empty_blocking_with_retry(&url, request)
    }

    fn show_model_blocking(&self, request: &ShowRequest) -> Result<ShowResponse> {
        let url = self.config.url(Endpoints::SHOW);
        self.post_blocking_with_retry(&url, request)
    }

    fn embed_blocking(&self, request: &EmbedRequest) -> Result<EmbedResponse> {
        let url = self.config.url(Endpoints::EMBED);
        self.post_blocking_with_retry(&url, request)
    }

    fn generate_blocking(&self, request: &GenerateRequest) -> Result<GenerateResponse> {
        let url = self.config.url(Endpoints::GENERATE);
        self.post_blocking_with_retry(&url, request)
    }
}
