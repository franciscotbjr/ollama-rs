//! Think setting primitive type

use serde::{Deserialize, Serialize};

/// Think setting for generate requests
///
/// Controls whether and how the model outputs its thinking process.
/// Can be a boolean (true/false) or a string ("high", "medium", "low").
///
/// # Examples
///
/// ```
/// use ollama_oxide::ThinkSetting;
///
/// let enabled = ThinkSetting::enabled();
/// let high = ThinkSetting::high();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThinkSetting {
    /// Boolean: true to enable, false to disable
    Enabled(bool),
    /// Level: "high", "medium", or "low"
    Level(String),
}

impl ThinkSetting {
    /// Enable thinking output
    pub fn enabled() -> Self {
        Self::Enabled(true)
    }

    /// Disable thinking output
    pub fn disabled() -> Self {
        Self::Enabled(false)
    }

    /// Set thinking level
    pub fn level(level: impl Into<String>) -> Self {
        Self::Level(level.into())
    }

    /// High thinking level
    pub fn high() -> Self {
        Self::Level("high".to_string())
    }

    /// Medium thinking level
    pub fn medium() -> Self {
        Self::Level("medium".to_string())
    }

    /// Low thinking level
    pub fn low() -> Self {
        Self::Level("low".to_string())
    }
}

impl From<bool> for ThinkSetting {
    fn from(b: bool) -> Self {
        Self::Enabled(b)
    }
}

impl From<&str> for ThinkSetting {
    fn from(s: &str) -> Self {
        Self::Level(s.to_string())
    }
}
