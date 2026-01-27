//! Create request primitive type

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{LicenseSetting};
use crate::{ChatMessage};

/// Request body for POST /api/create endpoint
///
/// Creates a custom model from an existing model with custom configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateRequest {
    /// Name for the model to create (required)
    pub model: String,

    /// Existing model to create from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    /// Prompt template to use for the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,

    /// License string or list of licenses for the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<LicenseSetting>,

    /// System prompt to embed in the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    /// Key-value parameters for the model (JSON object)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Value>,

    /// Message history to use for few-shot examples
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<ChatMessage>>,

    /// Quantization level to apply (e.g., `q4_K_M`, `q8_0`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantize: Option<String>,

    /// Whether to stream status updates (always false for v0.1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

impl CreateRequest {
    /// Create a new create request with just the model name
    ///
    /// # Arguments
    ///
    /// * `model` - Name for the new model to create
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            from: None,
            template: None,
            license: None,
            system: None,
            parameters: None,
            messages: None,
            quantize: None,
            stream: Some(false), // Non-streaming for v0.1.0
        }
    }

    /// Create a new model based on an existing model
    ///
    /// # Arguments
    ///
    /// * `model` - Name for the new model to create
    /// * `from` - Name of the base model to create from
    pub fn from_model(model: impl Into<String>, from: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            from: Some(from.into()),
            template: None,
            license: None,
            system: None,
            parameters: None,
            messages: None,
            quantize: None,
            stream: Some(false), // Non-streaming for v0.1.0
        }
    }

    /// Set the base model to create from
    pub fn with_from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    /// Set the prompt template
    pub fn with_template(mut self, template: impl Into<String>) -> Self {
        self.template = Some(template.into());
        self
    }

    /// Set the license
    pub fn with_license(mut self, license: impl Into<LicenseSetting>) -> Self {
        self.license = Some(license.into());
        self
    }

    /// Set the system prompt
    pub fn with_system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(system.into());
        self
    }

    /// Set model parameters as JSON value
    pub fn with_parameters(mut self, parameters: Value) -> Self {
        self.parameters = Some(parameters);
        self
    }

    /// Set few-shot example messages
    pub fn with_messages<I>(mut self, messages: I) -> Self
    where
        I: IntoIterator<Item = ChatMessage>,
    {
        self.messages = Some(messages.into_iter().collect());
        self
    }

    /// Add a single message to examples
    pub fn with_message(mut self, message: ChatMessage) -> Self {
        self.messages.get_or_insert_with(Vec::new).push(message);
        self
    }

    /// Set quantization level
    pub fn with_quantize(mut self, quantize: impl Into<String>) -> Self {
        self.quantize = Some(quantize.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_create_request_new() {
        let request = CreateRequest::new("mario");
        assert_eq!(request.model, "mario");
        assert!(request.from.is_none());
        assert!(request.system.is_none());
        assert_eq!(request.stream, Some(false));
    }

    #[test]
    fn test_create_request_from_model() {
        let request = CreateRequest::from_model("mario", "qwen3:0.6b");
        assert_eq!(request.model, "mario");
        assert_eq!(request.from, Some("qwen3:0.6b".to_string()));
        assert_eq!(request.stream, Some(false));
    }

    #[test]
    fn test_create_request_builder_pattern() {
        let request = CreateRequest::from_model("mario", "qwen3:0.6b")
            .with_system("You are Mario from Super Mario Bros.")
            .with_template("{{ .System }}\n\n{{ .Prompt }}")
            .with_license("MIT")
            .with_quantize("q4_K_M");

        assert_eq!(request.model, "mario");
        assert_eq!(request.from, Some("qwen3:0.6b".to_string()));
        assert_eq!(
            request.system,
            Some("You are Mario from Super Mario Bros.".to_string())
        );
        assert_eq!(
            request.template,
            Some("{{ .System }}\n\n{{ .Prompt }}".to_string())
        );
        assert_eq!(
            request.license,
            Some(LicenseSetting::Single("MIT".to_string()))
        );
        assert_eq!(request.quantize, Some("q4_K_M".to_string()));
    }

    #[test]
    fn test_create_request_with_messages() {
        let request = CreateRequest::from_model("mario", "qwen3:0.6b").with_messages([
            ChatMessage::user("Who are you?"),
            ChatMessage::assistant("It's-a me, Mario!"),
        ]);

        assert!(request.messages.is_some());
        let messages = request.messages.unwrap();
        assert_eq!(messages.len(), 2);
    }

    #[test]
    fn test_create_request_with_single_message() {
        let request = CreateRequest::from_model("mario", "qwen3:0.6b")
            .with_message(ChatMessage::user("Who are you?"))
            .with_message(ChatMessage::assistant("It's-a me, Mario!"));

        assert!(request.messages.is_some());
        let messages = request.messages.unwrap();
        assert_eq!(messages.len(), 2);
    }

    #[test]
    fn test_create_request_with_parameters() {
        let request = CreateRequest::from_model("mario", "qwen3:0.6b").with_parameters(json!({
            "temperature": 0.8,
            "top_k": 40
        }));

        assert!(request.parameters.is_some());
        let params = request.parameters.unwrap();
        assert_eq!(params["temperature"], 0.8);
        assert_eq!(params["top_k"], 40);
    }

    #[test]
    fn test_create_request_serialization() {
        let request = CreateRequest::from_model("mario", "qwen3:0.6b").with_system("You are Mario");

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"model\":\"mario\""));
        assert!(json.contains("\"from\":\"qwen3:0.6b\""));
        assert!(json.contains("\"system\":\"You are Mario\""));
        assert!(json.contains("\"stream\":false"));
    }

    #[test]
    fn test_create_request_serialization_skips_none() {
        let request = CreateRequest::new("mario");

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"model\":\"mario\""));
        assert!(!json.contains("\"from\""));
        assert!(!json.contains("\"system\""));
        assert!(!json.contains("\"template\""));
        assert!(!json.contains("\"license\""));
        assert!(!json.contains("\"parameters\""));
        assert!(!json.contains("\"messages\""));
        assert!(!json.contains("\"quantize\""));
    }
}
