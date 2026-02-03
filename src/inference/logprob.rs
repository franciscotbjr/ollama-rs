//! Log probability primitive type

use serde::{Deserialize, Serialize};

use super::TokenLogprob;

/// Log probability information for a generated token
///
/// Contains the token, its log probability, byte representation,
/// and alternative tokens with their probabilities.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Logprob {
    /// The text representation of the token
    #[serde(default)]
    pub token: Option<String>,

    /// The log probability of this token
    #[serde(default)]
    pub logprob: Option<f64>,

    /// The raw byte representation of the token
    #[serde(default)]
    pub bytes: Option<Vec<u8>>,

    /// Most likely tokens and their log probabilities at this position
    #[serde(default)]
    pub top_logprobs: Option<Vec<TokenLogprob>>,
}
