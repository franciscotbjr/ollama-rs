//! Stop setting primitive type

use serde::{Deserialize, Serialize};

/// Stop setting for generation
///
/// Controls stop sequences that will halt generation.
/// Can be a single string or an array of strings.
///
/// # Examples
///
/// ```
/// use ollama_oxide::StopSetting;
///
/// let single = StopSetting::single("\n");
/// let multiple = StopSetting::multiple([".", "!", "?"]);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StopSetting {
    /// Single stop sequence
    Single(String),
    /// Multiple stop sequences
    Multiple(Vec<String>),
}

impl StopSetting {
    /// Create a single stop sequence
    pub fn single(stop: impl Into<String>) -> Self {
        Self::Single(stop.into())
    }

    /// Create multiple stop sequences
    pub fn multiple<I, S>(stops: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self::Multiple(stops.into_iter().map(|s| s.into()).collect())
    }
}

impl From<&str> for StopSetting {
    fn from(s: &str) -> Self {
        Self::Single(s.to_string())
    }
}

impl From<Vec<String>> for StopSetting {
    fn from(v: Vec<String>) -> Self {
        Self::Multiple(v)
    }
}
