//! Chat message type for conversation history.

use serde::{Deserialize, Serialize};

use super::ChatRole;
#[cfg(feature = "tools")]
use super::ToolCall;

/// A message in a chat conversation.
///
/// Represents a single message in the conversation history, which can be
/// from the system, user, assistant, or a tool response.
///
/// # Examples
///
/// ## Creating Messages with Factory Methods
///
/// ```no_run
/// use ollama_oxide::ChatMessage;
///
/// let system = ChatMessage::system("You are a helpful assistant.");
/// let user = ChatMessage::user("Hello!");
/// let assistant = ChatMessage::assistant("Hi there! How can I help?");
/// let tool = ChatMessage::tool(r#"{"result": 42}"#);
/// ```
///
/// ## Building Conversations
///
/// ```no_run
/// use ollama_oxide::ChatMessage;
///
/// let messages = vec![
///     ChatMessage::system("You are a helpful assistant."),
///     ChatMessage::user("What is Rust?"),
///     ChatMessage::assistant("Rust is a systems programming language."),
///     ChatMessage::user("What are its main features?"),
/// ];
/// ```
///
/// ## Adding Images (Multimodal)
///
/// ```no_run
/// use ollama_oxide::ChatMessage;
///
/// let message = ChatMessage::user("What's in this image?")
///     .with_image("base64_encoded_image_data_here");
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Role of the message author (system, user, assistant, or tool).
    pub role: ChatRole,

    /// Text content of the message.
    pub content: String,

    /// Optional base64-encoded images for multimodal models.
    ///
    /// When using vision-capable models, you can include images
    /// as base64-encoded strings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,

    /// Tool calls made by the assistant (for assistant messages).
    ///
    /// When replaying a conversation that included tool calls,
    /// include the original tool_calls in the assistant message.
    ///
    /// Requires the `tools` feature.
    #[cfg(feature = "tools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

impl ChatMessage {
    /// Create a new message with the specified role and content.
    ///
    /// # Arguments
    ///
    /// * `role` - The role of the message author
    /// * `content` - The text content of the message
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::{ChatMessage, ChatRole};
    ///
    /// let msg = ChatMessage::new(ChatRole::User, "Hello!");
    /// assert_eq!(msg.role, ChatRole::User);
    /// assert_eq!(msg.content, "Hello!");
    /// ```
    pub fn new(role: ChatRole, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
            images: None,
            #[cfg(feature = "tools")]
            tool_calls: None,
        }
    }

    /// Create a system message.
    ///
    /// System messages set the behavior and context for the conversation.
    /// They are typically placed at the beginning of the conversation.
    ///
    /// # Arguments
    ///
    /// * `content` - Instructions or context for the assistant
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::{ChatMessage, ChatRole};
    ///
    /// let msg = ChatMessage::system("You are a helpful coding assistant.");
    /// assert_eq!(msg.role, ChatRole::System);
    /// ```
    pub fn system(content: impl Into<String>) -> Self {
        Self::new(ChatRole::System, content)
    }

    /// Create a user message.
    ///
    /// User messages represent input from the human interacting with
    /// the model.
    ///
    /// # Arguments
    ///
    /// * `content` - The user's message text
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::{ChatMessage, ChatRole};
    ///
    /// let msg = ChatMessage::user("What is the capital of France?");
    /// assert_eq!(msg.role, ChatRole::User);
    /// ```
    pub fn user(content: impl Into<String>) -> Self {
        Self::new(ChatRole::User, content)
    }

    /// Create an assistant message.
    ///
    /// Assistant messages represent previous responses from the model.
    /// Include these when building multi-turn conversations.
    ///
    /// # Arguments
    ///
    /// * `content` - The assistant's response text
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::{ChatMessage, ChatRole};
    ///
    /// let msg = ChatMessage::assistant("The capital of France is Paris.");
    /// assert_eq!(msg.role, ChatRole::Assistant);
    /// ```
    pub fn assistant(content: impl Into<String>) -> Self {
        Self::new(ChatRole::Assistant, content)
    }

    /// Create a tool response message.
    ///
    /// Tool messages contain the results of function calls. When the model
    /// requests a tool call, execute it in your application and send
    /// the result back as a tool message.
    ///
    /// # Arguments
    ///
    /// * `content` - The result of the tool execution (typically JSON)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::{ChatMessage, ChatRole};
    ///
    /// let result = r#"{"temperature": 22, "unit": "celsius"}"#;
    /// let msg = ChatMessage::tool(result);
    /// assert_eq!(msg.role, ChatRole::Tool);
    /// ```
    pub fn tool(content: impl Into<String>) -> Self {
        Self::new(ChatRole::Tool, content)
    }

    /// Add an image to the message (base64-encoded).
    ///
    /// For use with multimodal/vision models. Images should be
    /// base64-encoded.
    ///
    /// # Arguments
    ///
    /// * `image` - Base64-encoded image data
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::ChatMessage;
    ///
    /// let msg = ChatMessage::user("Describe this image.")
    ///     .with_image("iVBORw0KGgoAAAANSUhEUgAAAAEAAAAB...");
    /// assert!(msg.images.is_some());
    /// assert_eq!(msg.images.as_ref().unwrap().len(), 1);
    /// ```
    pub fn with_image(mut self, image: impl Into<String>) -> Self {
        self.images.get_or_insert_with(Vec::new).push(image.into());
        self
    }

    /// Add multiple images to the message.
    ///
    /// # Arguments
    ///
    /// * `images` - Iterator of base64-encoded image data
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::ChatMessage;
    ///
    /// let msg = ChatMessage::user("Compare these images.")
    ///     .with_images(vec!["base64_img1", "base64_img2"]);
    /// assert_eq!(msg.images.as_ref().unwrap().len(), 2);
    /// ```
    pub fn with_images<I, S>(mut self, images: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.images = Some(images.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Add tool calls to the message (for assistant messages).
    ///
    /// When replaying a conversation that included tool calls,
    /// include the original tool_calls in the assistant message.
    ///
    /// Requires the `tools` feature.
    ///
    /// # Arguments
    ///
    /// * `tool_calls` - The tool calls made by the assistant
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::{ChatMessage, ToolCall, ToolCallFunction};
    /// use serde_json::json;
    ///
    /// let call = ToolCall::new(ToolCallFunction::with_arguments(
    ///     "get_weather",
    ///     json!({"location": "Paris"})
    /// ));
    ///
    /// let msg = ChatMessage::assistant("")
    ///     .with_tool_calls(vec![call]);
    /// assert!(msg.tool_calls.is_some());
    /// ```
    #[cfg(feature = "tools")]
    pub fn with_tool_calls(mut self, tool_calls: Vec<ToolCall>) -> Self {
        self.tool_calls = Some(tool_calls);
        self
    }

    /// Check if this message has any images attached.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::ChatMessage;
    ///
    /// let without_images = ChatMessage::user("Hello");
    /// assert!(!without_images.has_images());
    ///
    /// let with_images = ChatMessage::user("Look").with_image("base64...");
    /// assert!(with_images.has_images());
    /// ```
    pub fn has_images(&self) -> bool {
        self.images.as_ref().map(|i| !i.is_empty()).unwrap_or(false)
    }

    /// Check if this message has any tool calls.
    ///
    /// Requires the `tools` feature.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::{ChatMessage, ToolCall, ToolCallFunction};
    ///
    /// let without_tools = ChatMessage::assistant("Hello");
    /// assert!(!without_tools.has_tool_calls());
    ///
    /// let with_tools = ChatMessage::assistant("")
    ///     .with_tool_calls(vec![ToolCall::new(ToolCallFunction::new("test"))]);
    /// assert!(with_tools.has_tool_calls());
    /// ```
    #[cfg(feature = "tools")]
    pub fn has_tool_calls(&self) -> bool {
        self.tool_calls
            .as_ref()
            .map(|tc| !tc.is_empty())
            .unwrap_or(false)
    }

    /// Check if this is a system message.
    pub fn is_system(&self) -> bool {
        self.role == ChatRole::System
    }

    /// Check if this is a user message.
    pub fn is_user(&self) -> bool {
        self.role == ChatRole::User
    }

    /// Check if this is an assistant message.
    pub fn is_assistant(&self) -> bool {
        self.role == ChatRole::Assistant
    }

    /// Check if this is a tool message.
    pub fn is_tool(&self) -> bool {
        self.role == ChatRole::Tool
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "tools")]
    use crate::ToolCallFunction;
    use serde_json::json;

    #[test]
    fn test_chat_message_new() {
        let msg = ChatMessage::new(ChatRole::User, "Hello");
        assert_eq!(msg.role, ChatRole::User);
        assert_eq!(msg.content, "Hello");
        assert!(msg.images.is_none());
        #[cfg(feature = "tools")]
        assert!(msg.tool_calls.is_none());
    }

    #[test]
    fn test_chat_message_system() {
        let msg = ChatMessage::system("Be helpful");
        assert_eq!(msg.role, ChatRole::System);
        assert_eq!(msg.content, "Be helpful");
    }

    #[test]
    fn test_chat_message_user() {
        let msg = ChatMessage::user("Hi there");
        assert_eq!(msg.role, ChatRole::User);
        assert_eq!(msg.content, "Hi there");
    }

    #[test]
    fn test_chat_message_assistant() {
        let msg = ChatMessage::assistant("Hello!");
        assert_eq!(msg.role, ChatRole::Assistant);
        assert_eq!(msg.content, "Hello!");
    }

    #[test]
    fn test_chat_message_tool() {
        let msg = ChatMessage::tool(r#"{"result": 42}"#);
        assert_eq!(msg.role, ChatRole::Tool);
        assert_eq!(msg.content, r#"{"result": 42}"#);
    }

    #[test]
    fn test_chat_message_with_image() {
        let msg = ChatMessage::user("What's this?").with_image("base64data");
        assert!(msg.images.is_some());
        let images = msg.images.unwrap();
        assert_eq!(images.len(), 1);
        assert_eq!(images[0], "base64data");
    }

    #[test]
    fn test_chat_message_with_multiple_images() {
        let msg = ChatMessage::user("Compare")
            .with_image("img1")
            .with_image("img2");
        let images = msg.images.unwrap();
        assert_eq!(images.len(), 2);
        assert_eq!(images[0], "img1");
        assert_eq!(images[1], "img2");
    }

    #[test]
    fn test_chat_message_with_images() {
        let msg = ChatMessage::user("Compare").with_images(vec!["a", "b", "c"]);
        let images = msg.images.unwrap();
        assert_eq!(images.len(), 3);
    }

    #[cfg(feature = "tools")]
    #[test]
    fn test_chat_message_with_tool_calls() {
        let call = ToolCall::new(ToolCallFunction::new("test"));
        let msg = ChatMessage::assistant("").with_tool_calls(vec![call]);
        assert!(msg.tool_calls.is_some());
        assert_eq!(msg.tool_calls.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_chat_message_has_images() {
        let no_images = ChatMessage::user("Hello");
        assert!(!no_images.has_images());

        let empty_images = ChatMessage {
            role: ChatRole::User,
            content: "test".into(),
            images: Some(vec![]),
            #[cfg(feature = "tools")]
            tool_calls: None,
        };
        assert!(!empty_images.has_images());

        let with_images = ChatMessage::user("Look").with_image("data");
        assert!(with_images.has_images());
    }

    #[cfg(feature = "tools")]
    #[test]
    fn test_chat_message_has_tool_calls() {
        let no_tools = ChatMessage::assistant("Hi");
        assert!(!no_tools.has_tool_calls());

        let empty_tools = ChatMessage {
            role: ChatRole::Assistant,
            content: "".into(),
            images: None,
            tool_calls: Some(vec![]),
        };
        assert!(!empty_tools.has_tool_calls());

        let with_tools = ChatMessage::assistant("")
            .with_tool_calls(vec![ToolCall::new(ToolCallFunction::new("f"))]);
        assert!(with_tools.has_tool_calls());
    }

    #[test]
    fn test_chat_message_role_checks() {
        assert!(ChatMessage::system("").is_system());
        assert!(!ChatMessage::system("").is_user());

        assert!(ChatMessage::user("").is_user());
        assert!(!ChatMessage::user("").is_assistant());

        assert!(ChatMessage::assistant("").is_assistant());
        assert!(!ChatMessage::assistant("").is_tool());

        assert!(ChatMessage::tool("").is_tool());
        assert!(!ChatMessage::tool("").is_system());
    }

    #[test]
    fn test_chat_message_serialize() {
        let msg = ChatMessage::user("Hello");
        let json = serde_json::to_value(&msg).unwrap();

        assert_eq!(json["role"], "user");
        assert_eq!(json["content"], "Hello");
        assert!(json.get("images").is_none()); // Skipped when None
        #[cfg(feature = "tools")]
        assert!(json.get("tool_calls").is_none());
    }

    #[test]
    fn test_chat_message_serialize_with_images() {
        let msg = ChatMessage::user("Look").with_image("data");
        let json = serde_json::to_value(&msg).unwrap();

        assert_eq!(json["images"], json!(["data"]));
    }

    #[cfg(feature = "tools")]
    #[test]
    fn test_chat_message_serialize_with_tool_calls() {
        let call = ToolCall::new(ToolCallFunction::with_arguments("f", json!({"x": 1})));
        let msg = ChatMessage::assistant("").with_tool_calls(vec![call]);
        let json = serde_json::to_value(&msg).unwrap();

        assert!(json.get("tool_calls").is_some());
        assert_eq!(json["tool_calls"][0]["function"]["name"], "f");
    }

    #[test]
    fn test_chat_message_deserialize() {
        let json = r#"{
            "role": "user",
            "content": "Hello world"
        }"#;

        let msg: ChatMessage = serde_json::from_str(json).unwrap();
        assert_eq!(msg.role, ChatRole::User);
        assert_eq!(msg.content, "Hello world");
        assert!(msg.images.is_none());
        #[cfg(feature = "tools")]
        assert!(msg.tool_calls.is_none());
    }

    #[test]
    fn test_chat_message_deserialize_with_images() {
        let json = r#"{
            "role": "user",
            "content": "Look",
            "images": ["img1", "img2"]
        }"#;

        let msg: ChatMessage = serde_json::from_str(json).unwrap();
        assert_eq!(
            msg.images,
            Some(vec!["img1".to_string(), "img2".to_string()])
        );
    }

    #[cfg(feature = "tools")]
    #[test]
    fn test_chat_message_deserialize_with_tool_calls() {
        let json = r#"{
            "role": "assistant",
            "content": "",
            "tool_calls": [
                {"function": {"name": "test", "arguments": {"a": 1}}}
            ]
        }"#;

        let msg: ChatMessage = serde_json::from_str(json).unwrap();
        assert!(msg.tool_calls.is_some());
        assert_eq!(
            msg.tool_calls.as_ref().unwrap()[0].function_name(),
            Some("test")
        );
    }

    #[test]
    fn test_chat_message_clone() {
        let msg = ChatMessage::user("Hello").with_image("data");
        let cloned = msg.clone();
        assert_eq!(msg, cloned);
    }

    #[test]
    fn test_chat_message_equality() {
        let msg1 = ChatMessage::user("Hello");
        let msg2 = ChatMessage::user("Hello");
        let msg3 = ChatMessage::user("World");

        assert_eq!(msg1, msg2);
        assert_ne!(msg1, msg3);
    }

    #[test]
    fn test_chat_message_into_string() {
        let msg = ChatMessage::user(String::from("owned"));
        assert_eq!(msg.content, "owned");

        let msg = ChatMessage::user("borrowed");
        assert_eq!(msg.content, "borrowed");
    }
}
