//! Generate request primitive type

use serde::{Deserialize, Serialize};

use super::{FormatSetting, KeepAliveSetting, ModelOptions, ThinkSetting};

/// Request body for POST /api/generate endpoint
///
/// Generates a text completion for the provided prompt.
///
/// # Examples
///
/// Basic request:
/// ```ignore
/// use ollama_oxide::GenerateRequest;
///
/// let request = GenerateRequest::new("qwen3:0.6b", "Why is the sky blue?");
/// ```
///
/// With options:
/// ```ignore
/// use ollama_oxide::{GenerateRequest, ModelOptions};
///
/// let request = GenerateRequest::new("qwen3:0.6b", "Tell me a joke")
///     .with_system("You are a comedian.")
///     .with_options(ModelOptions::default().with_temperature(0.9));
/// ```
///
/// With JSON output format:
/// ```ignore
/// use ollama_oxide::{GenerateRequest, FormatSetting};
///
/// let request = GenerateRequest::new("qwen3:0.6b", "List 3 colors as JSON")
///     .with_format(FormatSetting::json());
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateRequest {
    /// Name of the model to use
    pub model: String,

    /// Text prompt to generate a response from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    /// Text that appears after the prompt (for fill-in-the-middle)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,

    /// Base64-encoded images for multimodal models
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,

    /// Output format (string like "json" or JSON schema object)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<FormatSetting>,

    /// System prompt for the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    /// Whether to stream the response (always false for v0.1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Control thinking output (bool or "high"/"medium"/"low")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub think: Option<ThinkSetting>,

    /// Whether to return raw model output without templating
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<bool>,

    /// How long to keep the model loaded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<KeepAliveSetting>,

    /// Runtime options for generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<ModelOptions>,

    /// Whether to return log probabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,

    /// Number of top log probabilities to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<i32>,
}

impl GenerateRequest {
    /// Create a new generate request
    ///
    /// Creates a non-streaming request with the specified model and prompt.
    ///
    /// # Arguments
    ///
    /// * `model` - Name of the model to use
    /// * `prompt` - Text prompt for generation
    pub fn new(model: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            prompt: Some(prompt.into()),
            suffix: None,
            images: None,
            format: None,
            system: None,
            stream: Some(false), // Non-streaming for v0.1.0
            think: None,
            raw: None,
            keep_alive: None,
            options: None,
            logprobs: None,
            top_logprobs: None,
        }
    }

    /// Set the suffix (for fill-in-the-middle)
    pub fn with_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Add an image (base64-encoded)
    pub fn with_image(mut self, image: impl Into<String>) -> Self {
        self.images.get_or_insert_with(Vec::new).push(image.into());
        self
    }

    /// Set multiple images
    pub fn with_images<I, S>(mut self, images: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.images = Some(images.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Set the output format
    pub fn with_format(mut self, format: impl Into<FormatSetting>) -> Self {
        self.format = Some(format.into());
        self
    }

    /// Set the system prompt
    pub fn with_system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(system.into());
        self
    }

    /// Set the think option
    pub fn with_think(mut self, think: impl Into<ThinkSetting>) -> Self {
        self.think = Some(think.into());
        self
    }

    /// Enable raw mode (no prompt templating)
    pub fn with_raw(mut self, raw: bool) -> Self {
        self.raw = Some(raw);
        self
    }

    /// Set the keep_alive duration
    pub fn with_keep_alive(mut self, keep_alive: impl Into<KeepAliveSetting>) -> Self {
        self.keep_alive = Some(keep_alive.into());
        self
    }

    /// Set model options
    pub fn with_options(mut self, options: ModelOptions) -> Self {
        self.options = Some(options);
        self
    }

    /// Enable log probabilities
    pub fn with_logprobs(mut self, logprobs: bool) -> Self {
        self.logprobs = Some(logprobs);
        self
    }

    /// Set number of top log probabilities to return
    pub fn with_top_logprobs(mut self, n: i32) -> Self {
        self.top_logprobs = Some(n);
        self
    }
}
