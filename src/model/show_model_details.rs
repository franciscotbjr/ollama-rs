//! Show model details primitive type

use serde::{Deserialize, Serialize};

/// Model details returned by POST /api/show endpoint
///
/// Contains high-level information about the model's format,
/// family, and quantization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ShowModelDetails {
    /// Parent model name (empty string if this is a base model)
    #[serde(default)]
    pub parent_model: Option<String>,

    /// Model format (e.g., "gguf")
    #[serde(default)]
    pub format: Option<String>,

    /// Model family (e.g., "gemma3", "llama")
    #[serde(default)]
    pub family: Option<String>,

    /// List of model families this model belongs to
    #[serde(default)]
    pub families: Option<Vec<String>>,

    /// Parameter size (e.g., "4.3B", "7B", "13B")
    #[serde(default)]
    pub parameter_size: Option<String>,

    /// Quantization level (e.g., "Q4_K_M", "Q8_0")
    #[serde(default)]
    pub quantization_level: Option<String>,
}
