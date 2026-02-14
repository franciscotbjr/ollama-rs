//! Model summary primitive type

use serde::{Deserialize, Serialize};

use super::ModelDetails;

/// Summary information for a locally available model
///
/// Contains basic information about a model including its name, size,
/// digest, and detailed metadata.
///
/// # Example
///
/// ```json
/// {
///   "name": "gemma3",
///   "modified_at": "2025-10-03T23:34:03.409490317-07:00",
///   "size": 3338801804,
///   "digest": "a2af6cc3eb7fa8be8504abaf9b04e88f17a119ec3f04a3addf55f92841195f5a",
///   "details": {
///     "format": "gguf",
///     "family": "gemma"
///   }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelSummary {
    /// Model name (e.g., "llama3.2", "gemma3")
    pub name: String,

    /// Last modified timestamp in ISO 8601 format
    #[serde(default)]
    pub modified_at: Option<String>,

    /// Total size of the model on disk in bytes
    #[serde(default)]
    pub size: Option<u64>,

    /// SHA256 digest identifier of the model contents
    #[serde(default)]
    pub digest: Option<String>,

    /// Additional information about the model
    #[serde(default)]
    pub details: Option<ModelDetails>,
}
