//! Generate response primitive type

use serde::{Deserialize, Serialize};

use super::Logprob;

/// Response from POST /api/generate endpoint
///
/// Contains the generated text and timing/usage metrics.
///
/// # Example Response
///
/// ```json
/// {
///   "model": "qwen3:0.6b",
///   "created_at": "2025-10-17T23:14:07.414671Z",
///   "response": "The sky is blue because...",
///   "done": true,
///   "done_reason": "stop",
///   "total_duration": 174560334,
///   "load_duration": 101397084,
///   "prompt_eval_count": 11,
///   "prompt_eval_duration": 13074791,
///   "eval_count": 18,
///   "eval_duration": 52479709
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GenerateResponse {
    /// Model that generated the response
    #[serde(default)]
    pub model: Option<String>,

    /// ISO 8601 timestamp of response creation
    #[serde(default)]
    pub created_at: Option<String>,

    /// The model's generated text response
    #[serde(default)]
    pub response: Option<String>,

    /// The model's generated thinking output (if think was enabled)
    #[serde(default)]
    pub thinking: Option<String>,

    /// Indicates whether generation has finished
    #[serde(default)]
    pub done: Option<bool>,

    /// Reason the generation stopped (e.g., "stop", "length")
    #[serde(default)]
    pub done_reason: Option<String>,

    /// Total time spent generating the response in nanoseconds
    #[serde(default)]
    pub total_duration: Option<i64>,

    /// Time spent loading the model in nanoseconds
    #[serde(default)]
    pub load_duration: Option<i64>,

    /// Number of input tokens in the prompt
    #[serde(default)]
    pub prompt_eval_count: Option<i32>,

    /// Time spent evaluating the prompt in nanoseconds
    #[serde(default)]
    pub prompt_eval_duration: Option<i64>,

    /// Number of output tokens generated
    #[serde(default)]
    pub eval_count: Option<i32>,

    /// Time spent generating tokens in nanoseconds
    #[serde(default)]
    pub eval_duration: Option<i64>,

    /// Log probability information (if logprobs was enabled)
    #[serde(default)]
    pub logprobs: Option<Vec<Logprob>>,
}

impl GenerateResponse {
    /// Get the generated text response
    pub fn text(&self) -> Option<&str> {
        self.response.as_deref()
    }

    /// Get the thinking output (if available)
    pub fn thinking_text(&self) -> Option<&str> {
        self.thinking.as_deref()
    }

    /// Check if generation is complete
    pub fn is_done(&self) -> bool {
        self.done.unwrap_or(false)
    }

    /// Get total duration in milliseconds
    pub fn total_duration_ms(&self) -> Option<f64> {
        self.total_duration.map(|ns| ns as f64 / 1_000_000.0)
    }

    /// Get load duration in milliseconds
    pub fn load_duration_ms(&self) -> Option<f64> {
        self.load_duration.map(|ns| ns as f64 / 1_000_000.0)
    }

    /// Get prompt evaluation duration in milliseconds
    pub fn prompt_eval_duration_ms(&self) -> Option<f64> {
        self.prompt_eval_duration.map(|ns| ns as f64 / 1_000_000.0)
    }

    /// Get evaluation duration in milliseconds
    pub fn eval_duration_ms(&self) -> Option<f64> {
        self.eval_duration.map(|ns| ns as f64 / 1_000_000.0)
    }

    /// Calculate tokens per second for generation
    pub fn tokens_per_second(&self) -> Option<f64> {
        match (self.eval_count, self.eval_duration) {
            (Some(count), Some(duration)) if duration > 0 => {
                Some(count as f64 / (duration as f64 / 1_000_000_000.0))
            }
            _ => None,
        }
    }
}
