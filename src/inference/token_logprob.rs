//! Token log probability primitive type

use serde::{Deserialize, Serialize};

/// Log probability information for a single token alternative
///
/// Contains the token text, its log probability, and byte representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenLogprob {
    /// The text representation of the token
    #[serde(default)]
    pub token: Option<String>,

    /// The log probability of this token
    #[serde(default)]
    pub logprob: Option<f64>,

    /// The raw byte representation of the token
    #[serde(default)]
    pub bytes: Option<Vec<u8>>,
}
