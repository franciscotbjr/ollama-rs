//! Copy model request primitive type

use serde::{Deserialize, Serialize};

/// Request body for POST /api/copy endpoint
///
/// Creates a copy of an existing model with a new name.
///
/// # Example
///
/// ```no_run
/// use ollama_oxide::CopyRequest;
///
/// let request = CopyRequest {
///     source: "llama3.1".to_string(),
///     destination: "llama3.1-backup".to_string(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CopyRequest {
    /// Existing model name to copy from
    pub source: String,
    /// New model name to create
    pub destination: String,
}

impl CopyRequest {
    /// Create a new copy request
    ///
    /// # Arguments
    ///
    /// * `source` - Name of the existing model to copy
    /// * `destination` - Name for the new model copy
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ollama_oxide::CopyRequest;
    ///
    /// let request = CopyRequest::new("llama3.1", "llama3.1-backup");
    /// ```
    pub fn new(source: impl Into<String>, destination: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            destination: destination.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_request_serialization() {
        let request = CopyRequest {
            source: "gemma3".to_string(),
            destination: "gemma3-backup".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"source\":\"gemma3\""));
        assert!(json.contains("\"destination\":\"gemma3-backup\""));
    }

    #[test]
    fn test_copy_request_deserialization() {
        let json = r#"{"source": "llama3", "destination": "llama3-copy"}"#;
        let request: CopyRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.source, "llama3");
        assert_eq!(request.destination, "llama3-copy");
    }

    #[test]
    fn test_copy_request_new() {
        let request = CopyRequest::new("model-a", "model-b");
        assert_eq!(request.source, "model-a");
        assert_eq!(request.destination, "model-b");
    }

    #[test]
    fn test_copy_request_roundtrip() {
        let request = CopyRequest::new("source-model", "dest-model");
        let json = serde_json::to_string(&request).unwrap();
        let deserialized: CopyRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_copy_request_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<CopyRequest>();
    }
}
