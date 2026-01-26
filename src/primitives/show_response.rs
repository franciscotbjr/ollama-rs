//! Show model response primitive type

use serde::{Deserialize, Serialize};

use super::ShowModelDetails;

/// Response from POST /api/show endpoint
///
/// Contains comprehensive information about a model including
/// parameters, license, capabilities, and detailed metadata.
///
/// # Example
///
/// ```json
/// {
///   "license": "MIT License",
///   "parameters": "temperature 0.7\nnum_ctx 4096",
///   "template": "{{ .System }}\n{{ .Prompt }}",
///   "capabilities": ["completion", "vision"],
///   "details": {
///     "format": "gguf",
///     "family": "llama",
///     "parameter_size": "8B",
///     "quantization_level": "Q4_K_M"
///   },
///   "modified_at": "2024-01-15T10:30:00Z"
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ShowResponse {
    /// Model parameter settings serialized as text
    ///
    /// Contains configuration like temperature, num_ctx, etc.
    #[serde(default)]
    pub parameters: Option<String>,

    /// The license of the model
    #[serde(default)]
    pub license: Option<String>,

    /// Last modified timestamp in ISO 8601 format
    #[serde(default)]
    pub modified_at: Option<String>,

    /// High-level model details
    #[serde(default)]
    pub details: Option<ShowModelDetails>,

    /// The template used by the model to render prompts
    #[serde(default)]
    pub template: Option<String>,

    /// List of supported features (e.g., "completion", "vision")
    #[serde(default)]
    pub capabilities: Option<Vec<String>>,

    /// Additional model metadata
    ///
    /// This is a flexible key-value structure that contains
    /// model-specific information like attention head counts,
    /// context length, embedding dimensions, etc.
    ///
    /// Use `serde_json::Value` to access nested properties.
    #[serde(default)]
    pub model_info: Option<serde_json::Value>,
}

impl ShowResponse {
    /// Check if the model supports a specific capability
    ///
    /// # Arguments
    ///
    /// * `capability` - The capability to check (e.g., "completion", "vision")
    ///
    /// # Example
    ///
    /// ```ignore
    /// use ollama_oxide::ShowResponse;
    ///
    /// let response = ShowResponse {
    ///     capabilities: Some(vec!["completion".to_string(), "vision".to_string()]),
    ///     ..Default::default()
    /// };
    ///
    /// assert!(response.has_capability("vision"));
    /// assert!(!response.has_capability("tools"));
    /// ```
    pub fn has_capability(&self, capability: &str) -> bool {
        self.capabilities
            .as_ref()
            .is_some_and(|caps| caps.iter().any(|c| c == capability))
    }
}
