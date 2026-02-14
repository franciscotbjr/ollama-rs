//! Response message type for chat responses.

use serde::{Deserialize, Serialize};

#[cfg(feature = "tools")]
use crate::tools::ToolCall;

/// Message in a chat response.
///
/// Contains the assistant's response, including the text content,
/// optional thinking/reasoning output, and any tool calls requested.
///
/// # Examples
///
/// ```no_run
/// use ollama_oxide::ResponseMessage;
///
/// // Deserialize from API response
/// let json = r#"{
///     "role": "assistant",
///     "content": "Hello! How can I help you today?"
/// }"#;
///
/// let msg: ResponseMessage = serde_json::from_str(json).unwrap();
/// assert_eq!(msg.content.as_deref(), Some("Hello! How can I help you today?"));
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseMessage {
    /// Role of the message (always "assistant" for responses).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    /// Assistant's text response.
    ///
    /// May be empty or None when tool_calls are present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Thinking/reasoning output (if think was enabled in the request).
    ///
    /// Contains the model's reasoning process when thinking mode is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thinking: Option<String>,

    /// Tool calls requested by the assistant.
    ///
    /// When present, the assistant is requesting your application to
    /// execute one or more functions. Execute them and send the results
    /// back in a follow-up request.
    ///
    /// Requires the `tools` feature.
    #[cfg(feature = "tools")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,

    /// Optional images in the response.
    ///
    /// Some models may return generated images.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
}

impl ResponseMessage {
    /// Create a new response message with content.
    ///
    /// # Arguments
    ///
    /// * `content` - The assistant's response text
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::ResponseMessage;
    ///
    /// let msg = ResponseMessage::new("Hello!");
    /// assert_eq!(msg.content(), Some("Hello!"));
    /// ```
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            role: Some("assistant".to_string()),
            content: Some(content.into()),
            thinking: None,
            #[cfg(feature = "tools")]
            tool_calls: None,
            images: None,
        }
    }

    /// Create an empty response message.
    ///
    /// Useful for responses that only contain tool calls.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::ResponseMessage;
    ///
    /// let msg = ResponseMessage::empty();
    /// assert!(msg.content().is_none());
    /// ```
    pub fn empty() -> Self {
        Self {
            role: Some("assistant".to_string()),
            content: None,
            thinking: None,
            #[cfg(feature = "tools")]
            tool_calls: None,
            images: None,
        }
    }

    /// Get the text content of the message.
    ///
    /// # Returns
    ///
    /// The content as a string slice, or `None` if no content.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::ResponseMessage;
    ///
    /// let msg = ResponseMessage::new("Hi there!");
    /// assert_eq!(msg.content(), Some("Hi there!"));
    /// ```
    pub fn content(&self) -> Option<&str> {
        self.content.as_deref()
    }

    /// Get the thinking output.
    ///
    /// # Returns
    ///
    /// The thinking content as a string slice, or `None` if not available.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::ResponseMessage;
    ///
    /// let msg = ResponseMessage::default();
    /// assert!(msg.thinking().is_none());
    /// ```
    pub fn thinking(&self) -> Option<&str> {
        self.thinking.as_deref()
    }

    /// Get the tool calls if any.
    ///
    /// Requires the `tools` feature.
    ///
    /// # Returns
    ///
    /// A slice of tool calls, or `None` if no tool calls.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::ResponseMessage;
    ///
    /// let msg = ResponseMessage::new("Hello");
    /// assert!(msg.tool_calls().is_none());
    /// ```
    #[cfg(feature = "tools")]
    pub fn tool_calls(&self) -> Option<&[ToolCall]> {
        self.tool_calls.as_deref()
    }

    /// Check if the message contains tool calls.
    ///
    /// Requires the `tools` feature.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::ResponseMessage;
    ///
    /// let msg = ResponseMessage::new("No tools here");
    /// assert!(!msg.has_tool_calls());
    /// ```
    #[cfg(feature = "tools")]
    pub fn has_tool_calls(&self) -> bool {
        self.tool_calls
            .as_ref()
            .map(|tc| !tc.is_empty())
            .unwrap_or(false)
    }

    /// Check if the message has content.
    ///
    /// Returns true if content is present and non-empty.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::ResponseMessage;
    ///
    /// let msg = ResponseMessage::new("Hello");
    /// assert!(msg.has_content());
    ///
    /// let empty = ResponseMessage::new("");
    /// assert!(!empty.has_content());
    /// ```
    pub fn has_content(&self) -> bool {
        self.content
            .as_ref()
            .map(|c| !c.is_empty())
            .unwrap_or(false)
    }

    /// Check if thinking output is available.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::ResponseMessage;
    ///
    /// let msg = ResponseMessage::default();
    /// assert!(!msg.has_thinking());
    /// ```
    pub fn has_thinking(&self) -> bool {
        self.thinking
            .as_ref()
            .map(|t| !t.is_empty())
            .unwrap_or(false)
    }

    /// Check if the message has images.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::ResponseMessage;
    ///
    /// let msg = ResponseMessage::new("No images");
    /// assert!(!msg.has_images());
    /// ```
    pub fn has_images(&self) -> bool {
        self.images.as_ref().map(|i| !i.is_empty()).unwrap_or(false)
    }

    /// Get the images if any.
    ///
    /// # Returns
    ///
    /// A slice of base64-encoded images, or `None` if no images.
    pub fn images(&self) -> Option<&[String]> {
        self.images.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "tools")]
    use crate::ToolCallFunction;
    #[allow(unused_imports)]
    use serde_json::json;

    #[test]
    fn test_response_message_new() {
        let msg = ResponseMessage::new("Hello");
        assert_eq!(msg.role, Some("assistant".to_string()));
        assert_eq!(msg.content, Some("Hello".to_string()));
        assert!(msg.thinking.is_none());
        #[cfg(feature = "tools")]
        assert!(msg.tool_calls.is_none());
        assert!(msg.images.is_none());
    }

    #[test]
    fn test_response_message_empty() {
        let msg = ResponseMessage::empty();
        assert_eq!(msg.role, Some("assistant".to_string()));
        assert!(msg.content.is_none());
    }

    #[test]
    fn test_response_message_default() {
        let msg = ResponseMessage::default();
        assert!(msg.role.is_none());
        assert!(msg.content.is_none());
        assert!(msg.thinking.is_none());
        #[cfg(feature = "tools")]
        assert!(msg.tool_calls.is_none());
        assert!(msg.images.is_none());
    }

    #[test]
    fn test_response_message_content() {
        let msg = ResponseMessage::new("Test content");
        assert_eq!(msg.content(), Some("Test content"));

        let empty = ResponseMessage::empty();
        assert!(empty.content().is_none());
    }

    #[test]
    fn test_response_message_thinking() {
        let mut msg = ResponseMessage::new("Response");
        msg.thinking = Some("Thinking process...".to_string());
        assert_eq!(msg.thinking(), Some("Thinking process..."));
    }

    #[cfg(feature = "tools")]
    #[test]
    fn test_response_message_tool_calls() {
        let call = ToolCall::new(ToolCallFunction::new("test"));
        let mut msg = ResponseMessage::empty();
        msg.tool_calls = Some(vec![call]);

        assert!(msg.tool_calls().is_some());
        assert_eq!(msg.tool_calls().unwrap().len(), 1);
    }

    #[cfg(feature = "tools")]
    #[test]
    fn test_response_message_has_tool_calls() {
        let msg = ResponseMessage::new("No tools");
        assert!(!msg.has_tool_calls());

        let mut msg_with_tools = ResponseMessage::empty();
        msg_with_tools.tool_calls = Some(vec![ToolCall::new(ToolCallFunction::new("f"))]);
        assert!(msg_with_tools.has_tool_calls());

        let mut empty_tools = ResponseMessage::empty();
        empty_tools.tool_calls = Some(vec![]);
        assert!(!empty_tools.has_tool_calls());
    }

    #[test]
    fn test_response_message_has_content() {
        let msg = ResponseMessage::new("Hello");
        assert!(msg.has_content());

        let empty_string = ResponseMessage::new("");
        assert!(!empty_string.has_content());

        let no_content = ResponseMessage::empty();
        assert!(!no_content.has_content());
    }

    #[test]
    fn test_response_message_has_thinking() {
        let msg = ResponseMessage::new("Test");
        assert!(!msg.has_thinking());

        let mut with_thinking = ResponseMessage::new("Test");
        with_thinking.thinking = Some("I'm thinking...".to_string());
        assert!(with_thinking.has_thinking());

        let mut empty_thinking = ResponseMessage::new("Test");
        empty_thinking.thinking = Some("".to_string());
        assert!(!empty_thinking.has_thinking());
    }

    #[test]
    fn test_response_message_has_images() {
        let msg = ResponseMessage::new("Test");
        assert!(!msg.has_images());

        let mut with_images = ResponseMessage::new("Test");
        with_images.images = Some(vec!["base64data".to_string()]);
        assert!(with_images.has_images());
    }

    #[test]
    fn test_response_message_images() {
        let mut msg = ResponseMessage::new("Test");
        msg.images = Some(vec!["img1".to_string(), "img2".to_string()]);

        let images = msg.images().unwrap();
        assert_eq!(images.len(), 2);
        assert_eq!(images[0], "img1");
    }

    #[test]
    fn test_response_message_serialize() {
        let msg = ResponseMessage::new("Hello!");
        let json = serde_json::to_value(&msg).unwrap();

        assert_eq!(json["role"], "assistant");
        assert_eq!(json["content"], "Hello!");
        assert!(json.get("thinking").is_none()); // Skipped when None
        #[cfg(feature = "tools")]
        assert!(json.get("tool_calls").is_none());
        assert!(json.get("images").is_none());
    }

    #[cfg(feature = "tools")]
    #[test]
    fn test_response_message_serialize_with_tool_calls() {
        let mut msg = ResponseMessage::empty();
        msg.tool_calls = Some(vec![ToolCall::new(ToolCallFunction::with_arguments(
            "test",
            json!({"x": 1}),
        ))]);

        let json = serde_json::to_value(&msg).unwrap();
        assert!(json.get("tool_calls").is_some());
        assert_eq!(json["tool_calls"][0]["function"]["name"], "test");
    }

    #[test]
    fn test_response_message_deserialize() {
        let json = r#"{
            "role": "assistant",
            "content": "Hello!"
        }"#;

        let msg: ResponseMessage = serde_json::from_str(json).unwrap();
        assert_eq!(msg.role, Some("assistant".to_string()));
        assert_eq!(msg.content, Some("Hello!".to_string()));
    }

    #[test]
    fn test_response_message_deserialize_with_thinking() {
        let json = r#"{
            "role": "assistant",
            "content": "The answer is 42.",
            "thinking": "Let me calculate this step by step..."
        }"#;

        let msg: ResponseMessage = serde_json::from_str(json).unwrap();
        assert_eq!(msg.content(), Some("The answer is 42."));
        assert_eq!(
            msg.thinking(),
            Some("Let me calculate this step by step...")
        );
    }

    #[cfg(feature = "tools")]
    #[test]
    fn test_response_message_deserialize_with_tool_calls() {
        let json = r#"{
            "role": "assistant",
            "content": "",
            "tool_calls": [
                {
                    "function": {
                        "name": "get_weather",
                        "arguments": {"location": "Paris"}
                    }
                }
            ]
        }"#;

        let msg: ResponseMessage = serde_json::from_str(json).unwrap();
        assert!(msg.has_tool_calls());
        let calls = msg.tool_calls().unwrap();
        assert_eq!(calls[0].function_name(), Some("get_weather"));
    }

    #[test]
    fn test_response_message_deserialize_empty() {
        let json = "{}";
        let msg: ResponseMessage = serde_json::from_str(json).unwrap();
        assert!(msg.role.is_none());
        assert!(msg.content.is_none());
    }

    #[test]
    fn test_response_message_clone() {
        let mut msg = ResponseMessage::new("Test");
        msg.thinking = Some("Thinking".to_string());
        #[cfg(feature = "tools")]
        {
            msg.tool_calls = Some(vec![ToolCall::new(ToolCallFunction::new("f"))]);
        }

        let cloned = msg.clone();
        assert_eq!(msg, cloned);
    }

    #[test]
    fn test_response_message_equality() {
        let msg1 = ResponseMessage::new("Hello");
        let msg2 = ResponseMessage::new("Hello");
        let msg3 = ResponseMessage::new("World");

        assert_eq!(msg1, msg2);
        assert_ne!(msg1, msg3);
    }
}
