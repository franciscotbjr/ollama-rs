//! Embed response primitive type

use serde::{Deserialize, Serialize};

/// Response from POST /api/embed endpoint
///
/// Contains the generated embeddings and timing information.
///
/// # Example Response
///
/// ```json
/// {
///   "model": "nomic-embed-text",
///   "embeddings": [[0.010071, -0.001759, 0.050072, ...]],
///   "total_duration": 14143917,
///   "load_duration": 1019500,
///   "prompt_eval_count": 8
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EmbedResponse {
    /// Model that produced the embeddings
    #[serde(default)]
    pub model: Option<String>,

    /// Array of embedding vectors (one per input text)
    ///
    /// Each inner vector contains the embedding dimensions (e.g., 768 or 1024 floats).
    #[serde(default)]
    pub embeddings: Vec<Vec<f64>>,

    /// Total time spent generating embeddings in nanoseconds
    #[serde(default)]
    pub total_duration: Option<i64>,

    /// Time spent loading the model in nanoseconds
    #[serde(default)]
    pub load_duration: Option<i64>,

    /// Number of input tokens processed
    #[serde(default)]
    pub prompt_eval_count: Option<i32>,
}

impl EmbedResponse {
    /// Get the number of embeddings returned
    ///
    /// This corresponds to the number of input texts provided.
    pub fn len(&self) -> usize {
        self.embeddings.len()
    }

    /// Check if there are no embeddings
    pub fn is_empty(&self) -> bool {
        self.embeddings.is_empty()
    }

    /// Get the dimension of the embeddings
    ///
    /// Returns None if there are no embeddings.
    pub fn dimensions(&self) -> Option<usize> {
        self.embeddings.first().map(|e| e.len())
    }

    /// Get the first embedding (convenience for single-input requests)
    ///
    /// Returns None if there are no embeddings.
    pub fn first_embedding(&self) -> Option<&Vec<f64>> {
        self.embeddings.first()
    }

    /// Get total duration in milliseconds (convenience method)
    ///
    /// Converts from nanoseconds to milliseconds.
    pub fn total_duration_ms(&self) -> Option<f64> {
        self.total_duration.map(|ns| ns as f64 / 1_000_000.0)
    }

    /// Get load duration in milliseconds (convenience method)
    ///
    /// Converts from nanoseconds to milliseconds.
    pub fn load_duration_ms(&self) -> Option<f64> {
        self.load_duration.map(|ns| ns as f64 / 1_000_000.0)
    }
}
