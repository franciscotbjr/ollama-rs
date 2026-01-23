//! Embed input primitive type

use serde::{Deserialize, Serialize};

/// Input for embedding generation
///
/// Can be a single text string or an array of text strings.
/// Uses untagged serde deserialization to accept either format.
///
/// # Examples
///
/// ```
/// use ollama_oxide::EmbedInput;
///
/// // Single text input
/// let single = EmbedInput::Single("Hello, world!".to_string());
///
/// // Multiple text inputs
/// let multiple = EmbedInput::Multiple(vec![
///     "First text".to_string(),
///     "Second text".to_string(),
/// ]);
/// ```
///
/// # JSON Serialization
///
/// Single input serializes as a string:
/// ```json
/// "Hello, world!"
/// ```
///
/// Multiple inputs serialize as an array:
/// ```json
/// ["First text", "Second text"]
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmbedInput {
    /// Single text input
    Single(String),
    /// Multiple text inputs
    Multiple(Vec<String>),
}

impl EmbedInput {
    /// Create a single text input
    ///
    /// # Arguments
    ///
    /// * `text` - The text to embed
    ///
    /// # Example
    ///
    /// ```
    /// use ollama_oxide::EmbedInput;
    ///
    /// let input = EmbedInput::single("Hello, world!");
    /// ```
    pub fn single(text: impl Into<String>) -> Self {
        Self::Single(text.into())
    }

    /// Create a multiple text input
    ///
    /// # Arguments
    ///
    /// * `texts` - Iterator of texts to embed
    ///
    /// # Example
    ///
    /// ```
    /// use ollama_oxide::EmbedInput;
    ///
    /// let input = EmbedInput::multiple(["First", "Second", "Third"]);
    /// ```
    pub fn multiple<I, S>(texts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self::Multiple(texts.into_iter().map(|s| s.into()).collect())
    }

    /// Get the number of texts in the input
    ///
    /// # Example
    ///
    /// ```
    /// use ollama_oxide::EmbedInput;
    ///
    /// let single = EmbedInput::single("Hello");
    /// assert_eq!(single.len(), 1);
    ///
    /// let multiple = EmbedInput::multiple(["A", "B", "C"]);
    /// assert_eq!(multiple.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        match self {
            Self::Single(_) => 1,
            Self::Multiple(v) => v.len(),
        }
    }

    /// Check if the input is empty
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Single(s) => s.is_empty(),
            Self::Multiple(v) => v.is_empty(),
        }
    }
}

impl From<String> for EmbedInput {
    fn from(s: String) -> Self {
        Self::Single(s)
    }
}

impl From<&str> for EmbedInput {
    fn from(s: &str) -> Self {
        Self::Single(s.to_string())
    }
}

impl From<Vec<String>> for EmbedInput {
    fn from(v: Vec<String>) -> Self {
        Self::Multiple(v)
    }
}

impl<const N: usize> From<[&str; N]> for EmbedInput {
    fn from(arr: [&str; N]) -> Self {
        Self::Multiple(arr.iter().map(|s| (*s).to_string()).collect())
    }
}
