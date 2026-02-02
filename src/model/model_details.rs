//! Model details primitive type

use serde::{Deserialize, Serialize};

/// Additional information about a model's format and family
///
/// Contains metadata about the model's file format, family classification,
/// and quantization settings.
///
/// # Example
///
/// ```json
/// {
///   "format": "gguf",
///   "family": "gemma",
///   "families": ["gemma"],
///   "parameter_size": "4.3B",
///   "quantization_level": "Q4_K_M"
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelDetails {
    /// Model file format (e.g., "gguf")
    #[serde(default)]
    pub format: Option<String>,

    /// Primary model family (e.g., "llama", "gemma")
    #[serde(default)]
    pub family: Option<String>,

    /// All families the model belongs to
    #[serde(default)]
    pub families: Option<Vec<String>>,

    /// Approximate parameter count label (e.g., "7B", "13B")
    #[serde(default)]
    pub parameter_size: Option<String>,

    /// Quantization level used (e.g., "Q4_0", "Q4_K_M")
    #[serde(default)]
    pub quantization_level: Option<String>,
}
