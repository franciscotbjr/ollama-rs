//! Delete model request primitive type

use serde::{Deserialize, Serialize};

/// Request body for DELETE /api/delete endpoint
///
/// Deletes an existing model from the Ollama server.
///
/// # Example
///
/// ```
/// use ollama_oxide::DeleteRequest;
///
/// let request = DeleteRequest {
///     model: "llama3.1-backup".to_string(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteRequest {
    /// Name of the model to delete
    pub model: String,
}

impl DeleteRequest {
    /// Create a new delete request
    ///
    /// # Arguments
    ///
    /// * `model` - Name of the model to delete
    ///
    /// # Example
    ///
    /// ```
    /// use ollama_oxide::DeleteRequest;
    ///
    /// let request = DeleteRequest::new("llama3.1-backup");
    /// ```
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_request_serialization() {
        let request = DeleteRequest {
            model: "gemma3".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"model\":\"gemma3\""));
    }

    #[test]
    fn test_delete_request_deserialization() {
        let json = r#"{"model": "llama3"}"#;
        let request: DeleteRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.model, "llama3");
    }

    #[test]
    fn test_delete_request_new() {
        let request = DeleteRequest::new("model-a");
        assert_eq!(request.model, "model-a");
    }

    #[test]
    fn test_delete_request_roundtrip() {
        let request = DeleteRequest::new("source-model");
        let json = serde_json::to_string(&request).unwrap();
        let deserialized: DeleteRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_delete_request_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<DeleteRequest>();
    }
}
