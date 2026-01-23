//! Model options primitive type

use serde::{Deserialize, Serialize};

/// Runtime options that control model behavior
///
/// These options can be used to customize embedding generation.
/// All fields are optional and will use model defaults if not specified.
///
/// # Example
///
/// ```
/// use ollama_oxide::ModelOptions;
///
/// let options = ModelOptions::default()
///     .with_temperature(0.7)
///     .with_num_ctx(4096);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ModelOptions {
    /// Random seed for reproducible outputs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,

    /// Controls randomness in generation (higher = more random)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Limits next token selection to the K most likely
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,

    /// Cumulative probability threshold for nucleus sampling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Minimum probability threshold for token selection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_p: Option<f32>,

    /// Context length size (number of tokens)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_ctx: Option<i32>,

    /// Maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_predict: Option<i32>,
}

impl ModelOptions {
    /// Create empty options (all defaults)
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the random seed
    pub fn with_seed(mut self, seed: i64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Set the temperature
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set the top_k value
    pub fn with_top_k(mut self, top_k: i32) -> Self {
        self.top_k = Some(top_k);
        self
    }

    /// Set the top_p value
    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// Set the min_p value
    pub fn with_min_p(mut self, min_p: f32) -> Self {
        self.min_p = Some(min_p);
        self
    }

    /// Set the context length
    pub fn with_num_ctx(mut self, num_ctx: i32) -> Self {
        self.num_ctx = Some(num_ctx);
        self
    }

    /// Set the max tokens to generate
    pub fn with_num_predict(mut self, num_predict: i32) -> Self {
        self.num_predict = Some(num_predict);
        self
    }

    /// Check if any options are set
    pub fn is_empty(&self) -> bool {
        self.seed.is_none()
            && self.temperature.is_none()
            && self.top_k.is_none()
            && self.top_p.is_none()
            && self.min_p.is_none()
            && self.num_ctx.is_none()
            && self.num_predict.is_none()
    }
}
