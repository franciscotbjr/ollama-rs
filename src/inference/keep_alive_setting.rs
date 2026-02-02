//! Keep alive setting primitive type

use serde::{Deserialize, Serialize};

/// Keep alive setting for model caching
///
/// Controls how long the model stays loaded in memory.
/// Can be a string (e.g., "5m", "1h") or a number (seconds).
///
/// # Examples
///
/// ```no_run
/// use ollama_oxide::KeepAliveSetting;
///
/// let duration = KeepAliveSetting::duration("5m");
/// let seconds = KeepAliveSetting::seconds(300);
/// let unload = KeepAliveSetting::unload_immediately();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeepAliveSetting {
    /// Duration string (e.g., "5m", "1h", "30s")
    Duration(String),
    /// Duration in seconds
    Seconds(i64),
}

impl KeepAliveSetting {
    /// Create from duration string (e.g., "5m", "1h")
    pub fn duration(d: impl Into<String>) -> Self {
        Self::Duration(d.into())
    }

    /// Create from seconds
    pub fn seconds(s: i64) -> Self {
        Self::Seconds(s)
    }

    /// Unload model immediately after request
    pub fn unload_immediately() -> Self {
        Self::Seconds(0)
    }
}

impl From<&str> for KeepAliveSetting {
    fn from(s: &str) -> Self {
        Self::Duration(s.to_string())
    }
}

impl From<i64> for KeepAliveSetting {
    fn from(s: i64) -> Self {
        Self::Seconds(s)
    }
}
