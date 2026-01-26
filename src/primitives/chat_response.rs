//! Chat response type for POST /api/chat endpoint.

use serde::{Deserialize, Serialize};

#[cfg(feature = "tools")]
use super::ToolCall;
use super::{Logprob, ResponseMessage};

/// Response from POST /api/chat endpoint.
///
/// Contains the assistant's message and timing/usage metrics.
///
/// # Examples
///
/// ```ignore
/// use ollama_oxide::ChatResponse;
///
/// // Deserialize from API response
/// let json = r#"{
///     "model": "qwen3:0.6b",
///     "created_at": "2025-10-17T23:14:07.414671Z",
///     "message": {
///         "role": "assistant",
///         "content": "Hello! How can I help you today?"
///     },
///     "done": true,
///     "done_reason": "stop"
/// }"#;
///
/// let response: ChatResponse = serde_json::from_str(json).unwrap();
/// assert_eq!(response.content(), Some("Hello! How can I help you today?"));
/// assert!(response.is_done());
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ChatResponse {
    /// Model that generated the response.
    #[serde(default)]
    pub model: Option<String>,

    /// ISO 8601 timestamp of response creation.
    #[serde(default)]
    pub created_at: Option<String>,

    /// The assistant's response message.
    ///
    /// Contains the text content, optional thinking output,
    /// and any tool calls requested.
    #[serde(default)]
    pub message: Option<ResponseMessage>,

    /// Indicates whether generation has finished.
    #[serde(default)]
    pub done: Option<bool>,

    /// Reason the generation stopped.
    ///
    /// Common values: "stop" (natural end), "length" (max tokens reached).
    #[serde(default)]
    pub done_reason: Option<String>,

    /// Total time spent generating the response in nanoseconds.
    #[serde(default)]
    pub total_duration: Option<i64>,

    /// Time spent loading the model in nanoseconds.
    #[serde(default)]
    pub load_duration: Option<i64>,

    /// Number of input tokens in the prompt.
    #[serde(default)]
    pub prompt_eval_count: Option<i32>,

    /// Time spent evaluating the prompt in nanoseconds.
    #[serde(default)]
    pub prompt_eval_duration: Option<i64>,

    /// Number of output tokens generated.
    #[serde(default)]
    pub eval_count: Option<i32>,

    /// Time spent generating tokens in nanoseconds.
    #[serde(default)]
    pub eval_duration: Option<i64>,

    /// Log probability information (if logprobs was enabled).
    #[serde(default)]
    pub logprobs: Option<Vec<Logprob>>,
}

impl ChatResponse {
    /// Get the assistant's text response.
    ///
    /// # Returns
    ///
    /// The content as a string slice, or `None` if no content.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ollama_oxide::ChatResponse;
    ///
    /// let json = r#"{"message": {"role": "assistant", "content": "Hello!"}}"#;
    /// let response: ChatResponse = serde_json::from_str(json).unwrap();
    /// assert_eq!(response.content(), Some("Hello!"));
    /// ```
    pub fn content(&self) -> Option<&str> {
        self.message.as_ref().and_then(|m| m.content.as_deref())
    }

    /// Get the thinking output (if available).
    ///
    /// Thinking output is only present when the request had `think` enabled.
    ///
    /// # Returns
    ///
    /// The thinking content as a string slice, or `None` if not available.
    pub fn thinking(&self) -> Option<&str> {
        self.message.as_ref().and_then(|m| m.thinking.as_deref())
    }

    /// Get tool calls from the response.
    ///
    /// Requires the `tools` feature.
    ///
    /// # Returns
    ///
    /// A slice of tool calls, or `None` if no tool calls.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ollama_oxide::ChatResponse;
    ///
    /// let json = r#"{
    ///     "message": {
    ///         "role": "assistant",
    ///         "content": "",
    ///         "tool_calls": [{"function": {"name": "test"}}]
    ///     }
    /// }"#;
    ///
    /// let response: ChatResponse = serde_json::from_str(json).unwrap();
    /// assert!(response.tool_calls().is_some());
    /// ```
    #[cfg(feature = "tools")]
    pub fn tool_calls(&self) -> Option<&[ToolCall]> {
        self.message.as_ref().and_then(|m| m.tool_calls.as_deref())
    }

    /// Check if the response contains tool calls.
    ///
    /// Requires the `tools` feature.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ollama_oxide::ChatResponse;
    ///
    /// let response = ChatResponse::default();
    /// assert!(!response.has_tool_calls());
    /// ```
    #[cfg(feature = "tools")]
    pub fn has_tool_calls(&self) -> bool {
        self.tool_calls().map(|tc| !tc.is_empty()).unwrap_or(false)
    }

    /// Check if generation is complete.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ollama_oxide::ChatResponse;
    ///
    /// let json = r#"{"done": true}"#;
    /// let response: ChatResponse = serde_json::from_str(json).unwrap();
    /// assert!(response.is_done());
    /// ```
    pub fn is_done(&self) -> bool {
        self.done.unwrap_or(false)
    }

    /// Get total duration in milliseconds.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ollama_oxide::ChatResponse;
    ///
    /// let json = r#"{"total_duration": 1000000000}"#;
    /// let response: ChatResponse = serde_json::from_str(json).unwrap();
    /// assert_eq!(response.total_duration_ms(), Some(1000.0));
    /// ```
    pub fn total_duration_ms(&self) -> Option<f64> {
        self.total_duration.map(|ns| ns as f64 / 1_000_000.0)
    }

    /// Get load duration in milliseconds.
    pub fn load_duration_ms(&self) -> Option<f64> {
        self.load_duration.map(|ns| ns as f64 / 1_000_000.0)
    }

    /// Get prompt evaluation duration in milliseconds.
    pub fn prompt_eval_duration_ms(&self) -> Option<f64> {
        self.prompt_eval_duration.map(|ns| ns as f64 / 1_000_000.0)
    }

    /// Get token generation duration in milliseconds.
    pub fn eval_duration_ms(&self) -> Option<f64> {
        self.eval_duration.map(|ns| ns as f64 / 1_000_000.0)
    }

    /// Calculate tokens per second for generation.
    ///
    /// Returns the rate at which tokens were generated.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ollama_oxide::ChatResponse;
    ///
    /// let json = r#"{"eval_count": 100, "eval_duration": 1000000000}"#;
    /// let response: ChatResponse = serde_json::from_str(json).unwrap();
    /// assert_eq!(response.tokens_per_second(), Some(100.0));
    /// ```
    pub fn tokens_per_second(&self) -> Option<f64> {
        match (self.eval_count, self.eval_duration) {
            (Some(count), Some(duration)) if duration > 0 => {
                Some(count as f64 / (duration as f64 / 1_000_000_000.0))
            }
            _ => None,
        }
    }

    /// Get the model name.
    pub fn model(&self) -> Option<&str> {
        self.model.as_deref()
    }

    /// Get the creation timestamp.
    pub fn created_at(&self) -> Option<&str> {
        self.created_at.as_deref()
    }

    /// Get the done reason.
    pub fn done_reason(&self) -> Option<&str> {
        self.done_reason.as_deref()
    }

    /// Get the prompt token count.
    pub fn prompt_tokens(&self) -> Option<i32> {
        self.prompt_eval_count
    }

    /// Get the generated token count.
    pub fn completion_tokens(&self) -> Option<i32> {
        self.eval_count
    }

    /// Get total tokens (prompt + completion).
    pub fn total_tokens(&self) -> Option<i32> {
        match (self.prompt_eval_count, self.eval_count) {
            (Some(p), Some(c)) => Some(p + c),
            (Some(p), None) => Some(p),
            (None, Some(c)) => Some(c),
            (None, None) => None,
        }
    }

    /// Check if the response has content (non-empty text).
    pub fn has_content(&self) -> bool {
        self.content().map(|c| !c.is_empty()).unwrap_or(false)
    }

    /// Check if thinking output is available.
    pub fn has_thinking(&self) -> bool {
        self.thinking().map(|t| !t.is_empty()).unwrap_or(false)
    }

    /// Get the response message if available.
    pub fn message(&self) -> Option<&ResponseMessage> {
        self.message.as_ref()
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    #[cfg(feature = "tools")]
    use crate::ToolCallFunction;
    use serde_json::json;

    #[test]
    fn test_chat_response_default() {
        let response = ChatResponse::default();
        assert!(response.model.is_none());
        assert!(response.message.is_none());
        assert!(response.done.is_none());
    }

    #[test]
    fn test_chat_response_content() {
        let json = r#"{"message": {"role": "assistant", "content": "Hello!"}}"#;
        let response: ChatResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.content(), Some("Hello!"));
    }

    #[test]
    fn test_chat_response_content_empty() {
        let json = r#"{"message": {"role": "assistant", "content": ""}}"#;
        let response: ChatResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.content(), Some(""));
        assert!(!response.has_content());
    }

    #[test]
    fn test_chat_response_content_none() {
        let response = ChatResponse::default();
        assert!(response.content().is_none());
    }

    #[test]
    fn test_chat_response_thinking() {
        let json = r#"{
            "message": {
                "role": "assistant",
                "content": "42",
                "thinking": "Let me calculate..."
            }
        }"#;
        let response: ChatResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.thinking(), Some("Let me calculate..."));
        assert!(response.has_thinking());
    }

    #[cfg(feature = "tools")]
    #[test]
    fn test_chat_response_tool_calls() {
        let json = r#"{
            "message": {
                "role": "assistant",
                "content": "",
                "tool_calls": [
                    {"function": {"name": "get_weather", "arguments": {"location": "Paris"}}}
                ]
            }
        }"#;
        let response: ChatResponse = serde_json::from_str(json).unwrap();

        assert!(response.has_tool_calls());
        let calls = response.tool_calls().unwrap();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].function_name(), Some("get_weather"));
    }

    #[cfg(feature = "tools")]
    #[test]
    fn test_chat_response_no_tool_calls() {
        let json = r#"{"message": {"role": "assistant", "content": "Hello"}}"#;
        let response: ChatResponse = serde_json::from_str(json).unwrap();
        assert!(!response.has_tool_calls());
    }

    #[test]
    fn test_chat_response_is_done() {
        let json_true = r#"{"done": true}"#;
        let response: ChatResponse = serde_json::from_str(json_true).unwrap();
        assert!(response.is_done());

        let json_false = r#"{"done": false}"#;
        let response: ChatResponse = serde_json::from_str(json_false).unwrap();
        assert!(!response.is_done());

        let response = ChatResponse::default();
        assert!(!response.is_done());
    }

    #[test]
    fn test_chat_response_duration_conversions() {
        let json = r#"{
            "total_duration": 1000000000,
            "load_duration": 500000000,
            "prompt_eval_duration": 200000000,
            "eval_duration": 300000000
        }"#;
        let response: ChatResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.total_duration_ms(), Some(1000.0));
        assert_eq!(response.load_duration_ms(), Some(500.0));
        assert_eq!(response.prompt_eval_duration_ms(), Some(200.0));
        assert_eq!(response.eval_duration_ms(), Some(300.0));
    }

    #[test]
    fn test_chat_response_tokens_per_second() {
        let json = r#"{"eval_count": 100, "eval_duration": 2000000000}"#;
        let response: ChatResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.tokens_per_second(), Some(50.0));
    }

    #[test]
    fn test_chat_response_tokens_per_second_zero_duration() {
        let json = r#"{"eval_count": 100, "eval_duration": 0}"#;
        let response: ChatResponse = serde_json::from_str(json).unwrap();
        assert!(response.tokens_per_second().is_none());
    }

    #[test]
    fn test_chat_response_tokens_per_second_missing() {
        let response = ChatResponse::default();
        assert!(response.tokens_per_second().is_none());
    }

    #[test]
    fn test_chat_response_model() {
        let json = r#"{"model": "qwen3:0.6b"}"#;
        let response: ChatResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.model(), Some("qwen3:0.6b"));
    }

    #[test]
    fn test_chat_response_created_at() {
        let json = r#"{"created_at": "2025-10-17T23:14:07Z"}"#;
        let response: ChatResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.created_at(), Some("2025-10-17T23:14:07Z"));
    }

    #[test]
    fn test_chat_response_done_reason() {
        let json = r#"{"done_reason": "stop"}"#;
        let response: ChatResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.done_reason(), Some("stop"));
    }

    #[test]
    fn test_chat_response_token_counts() {
        let json = r#"{"prompt_eval_count": 10, "eval_count": 50}"#;
        let response: ChatResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.prompt_tokens(), Some(10));
        assert_eq!(response.completion_tokens(), Some(50));
        assert_eq!(response.total_tokens(), Some(60));
    }

    #[test]
    fn test_chat_response_total_tokens_partial() {
        let json = r#"{"prompt_eval_count": 10}"#;
        let response: ChatResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.total_tokens(), Some(10));

        let json = r#"{"eval_count": 50}"#;
        let response: ChatResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.total_tokens(), Some(50));
    }

    #[test]
    fn test_chat_response_message() {
        let json = r#"{"message": {"role": "assistant", "content": "Hi"}}"#;
        let response: ChatResponse = serde_json::from_str(json).unwrap();

        let msg = response.message().unwrap();
        assert_eq!(msg.content(), Some("Hi"));
    }

    #[test]
    fn test_chat_response_deserialize_full() {
        let json = r#"{
            "model": "qwen3:0.6b",
            "created_at": "2025-10-17T23:14:07.414671Z",
            "message": {
                "role": "assistant",
                "content": "Hello! How can I help you today?"
            },
            "done": true,
            "done_reason": "stop",
            "total_duration": 174560334,
            "load_duration": 101397084,
            "prompt_eval_count": 11,
            "prompt_eval_duration": 13074791,
            "eval_count": 18,
            "eval_duration": 52479709
        }"#;

        let response: ChatResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.model(), Some("qwen3:0.6b"));
        assert_eq!(response.content(), Some("Hello! How can I help you today?"));
        assert!(response.is_done());
        assert_eq!(response.done_reason(), Some("stop"));
        assert_eq!(response.prompt_tokens(), Some(11));
        assert_eq!(response.completion_tokens(), Some(18));
        assert!(response.tokens_per_second().is_some());
    }

    #[cfg(feature = "tools")]
    #[test]
    fn test_chat_response_deserialize_with_tool_calls() {
        let json = r#"{
            "model": "qwen3:0.6b",
            "message": {
                "role": "assistant",
                "content": "",
                "tool_calls": [
                    {
                        "function": {
                            "name": "get_weather",
                            "arguments": {"location": "Paris", "unit": "celsius"}
                        }
                    }
                ]
            },
            "done": true,
            "done_reason": "stop"
        }"#;

        let response: ChatResponse = serde_json::from_str(json).unwrap();

        assert!(response.has_tool_calls());
        assert!(!response.has_content());

        let calls = response.tool_calls().unwrap();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].function_name(), Some("get_weather"));

        let args = calls[0].arguments().unwrap();
        assert_eq!(args["location"], "Paris");
    }

    #[test]
    fn test_chat_response_deserialize_empty() {
        let json = "{}";
        let response: ChatResponse = serde_json::from_str(json).unwrap();

        assert!(response.model.is_none());
        assert!(response.message.is_none());
        assert!(response.done.is_none());
    }

    #[test]
    fn test_chat_response_serialize() {
        let mut response = ChatResponse::default();
        response.model = Some("test".to_string());
        response.done = Some(true);
        response.message = Some(ResponseMessage::new("Hello"));

        let json = serde_json::to_value(&response).unwrap();

        assert_eq!(json["model"], "test");
        assert_eq!(json["done"], true);
        assert_eq!(json["message"]["content"], "Hello");
    }

    #[test]
    fn test_chat_response_clone() {
        let json = r#"{
            "model": "test",
            "message": {"role": "assistant", "content": "Hi"},
            "done": true
        }"#;
        let response: ChatResponse = serde_json::from_str(json).unwrap();
        let cloned = response.clone();

        assert_eq!(response, cloned);
    }

    #[test]
    fn test_chat_response_equality() {
        let json1 = r#"{"model": "a", "done": true}"#;
        let json2 = r#"{"model": "a", "done": true}"#;
        let json3 = r#"{"model": "b", "done": true}"#;

        let r1: ChatResponse = serde_json::from_str(json1).unwrap();
        let r2: ChatResponse = serde_json::from_str(json2).unwrap();
        let r3: ChatResponse = serde_json::from_str(json3).unwrap();

        assert_eq!(r1, r2);
        assert_ne!(r1, r3);
    }
}
