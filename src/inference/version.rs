//! Version response primitive type

use serde::{Deserialize, Serialize};

/// Response from GET /api/version endpoint
///
/// Contains the version string of the Ollama server.
///
/// # Example
///
/// ```json
/// {
///   "version": "0.12.6"
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VersionResponse {
    /// Version of Ollama
    pub version: String,
}
