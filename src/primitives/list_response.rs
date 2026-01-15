//! List models response primitive type

use serde::{Deserialize, Serialize};

use super::ModelSummary;

/// Response from GET /api/tags endpoint
///
/// Contains a list of locally available models with their details.
///
/// # Example
///
/// ```json
/// {
///   "models": [
///     {
///       "name": "gemma3",
///       "modified_at": "2025-10-03T23:34:03.409490317-07:00",
///       "size": 3338801804,
///       "digest": "a2af6cc3..."
///     }
///   ]
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListResponse {
    /// List of available models
    #[serde(default)]
    pub models: Vec<ModelSummary>,
}
