//! Format setting primitive type

use serde::{Deserialize, Serialize};

/// Format setting for generate requests
///
/// Controls the output format of the model response.
/// Can be a string (like "json") or a JSON schema object.
///
/// # Examples
///
/// ```
/// use ollama_oxide::FormatSetting;
///
/// let json = FormatSetting::json();
/// let schema = FormatSetting::schema(serde_json::json!({
///     "type": "object",
///     "properties": { "name": { "type": "string" } }
/// }));
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FormatSetting {
    /// Simple format string (e.g., "json")
    String(String),
    /// JSON schema object for structured output
    Schema(serde_json::Value),
}

impl FormatSetting {
    /// Create JSON format
    pub fn json() -> Self {
        Self::String("json".to_string())
    }

    /// Create custom format string
    pub fn string(format: impl Into<String>) -> Self {
        Self::String(format.into())
    }

    /// Create schema-based format
    pub fn schema(schema: serde_json::Value) -> Self {
        Self::Schema(schema)
    }
}

impl From<&str> for FormatSetting {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<serde_json::Value> for FormatSetting {
    fn from(v: serde_json::Value) -> Self {
        Self::Schema(v)
    }
}
