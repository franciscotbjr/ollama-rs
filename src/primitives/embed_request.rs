//! Embed request primitive type

use serde::{Deserialize, Serialize};

use super::{EmbedInput, ModelOptions};

/// Request body for POST /api/embed endpoint
///
/// Generates vector embeddings for the provided input text(s).
///
/// # Examples
///
/// Basic single text request:
/// ```
/// use ollama_oxide::EmbedRequest;
///
/// let request = EmbedRequest::new("nomic-embed-text", "Hello, world!");
/// ```
///
/// Multiple texts:
/// ```
/// use ollama_oxide::{EmbedRequest, EmbedInput};
///
/// let request = EmbedRequest::new(
///     "nomic-embed-text",
///     EmbedInput::multiple(["First text", "Second text"])
/// );
/// ```
///
/// With options:
/// ```
/// use ollama_oxide::{EmbedRequest, ModelOptions};
///
/// let request = EmbedRequest::new("nomic-embed-text", "Hello")
///     .with_truncate(true)
///     .with_dimensions(768)
///     .with_keep_alive("5m");
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbedRequest {
    /// Name of the embedding model to use
    pub model: String,

    /// Text or array of texts to generate embeddings for
    pub input: EmbedInput,

    /// If true, truncate inputs that exceed context window (default: true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncate: Option<bool>,

    /// Number of dimensions for the embedding (model-specific)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<i32>,

    /// How long to keep the model loaded (e.g., "5m", "1h")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<String>,

    /// Runtime options for embedding generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<ModelOptions>,
}

impl EmbedRequest {
    /// Create a new embed request
    ///
    /// # Arguments
    ///
    /// * `model` - Name of the embedding model to use
    /// * `input` - Text or array of texts to embed
    ///
    /// # Example
    ///
    /// ```
    /// use ollama_oxide::EmbedRequest;
    ///
    /// let request = EmbedRequest::new("nomic-embed-text", "Hello, world!");
    /// ```
    pub fn new(model: impl Into<String>, input: impl Into<EmbedInput>) -> Self {
        Self {
            model: model.into(),
            input: input.into(),
            truncate: None,
            dimensions: None,
            keep_alive: None,
            options: None,
        }
    }

    /// Set the truncate option
    ///
    /// If true, truncate inputs that exceed the model's context window.
    /// If false, returns an error for inputs that are too long.
    pub fn with_truncate(mut self, truncate: bool) -> Self {
        self.truncate = Some(truncate);
        self
    }

    /// Set the embedding dimensions
    ///
    /// Some models support generating embeddings with different dimensions.
    pub fn with_dimensions(mut self, dimensions: i32) -> Self {
        self.dimensions = Some(dimensions);
        self
    }

    /// Set the keep_alive duration
    ///
    /// Controls how long the model stays loaded in memory (e.g., "5m", "1h").
    pub fn with_keep_alive(mut self, keep_alive: impl Into<String>) -> Self {
        self.keep_alive = Some(keep_alive.into());
        self
    }

    /// Set the model options
    pub fn with_options(mut self, options: ModelOptions) -> Self {
        self.options = Some(options);
        self
    }
}
