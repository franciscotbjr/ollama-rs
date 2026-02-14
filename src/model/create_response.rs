//! Create response primitive type

use serde::{Deserialize, Serialize};

/// Response from POST /api/create endpoint (non-streaming)
///
/// Contains the status of the model creation operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateResponse {
    /// Status message (e.g., "success")
    #[serde(default)]
    pub status: Option<String>,
}

impl CreateResponse {
    /// Get the status message
    pub fn status(&self) -> Option<&str> {
        self.status.as_deref()
    }

    /// Check if the operation was successful
    pub fn is_success(&self) -> bool {
        self.status.as_deref() == Some("success")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_response_deserialization() {
        let json = r#"{"status": "success"}"#;
        let response: CreateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status(), Some("success"));
        assert!(response.is_success());
    }

    #[test]
    fn test_create_response_is_success() {
        let success: CreateResponse = serde_json::from_str(r#"{"status": "success"}"#).unwrap();
        assert!(success.is_success());

        let other: CreateResponse = serde_json::from_str(r#"{"status": "creating"}"#).unwrap();
        assert!(!other.is_success());
    }

    #[test]
    fn test_create_response_empty() {
        let response: CreateResponse = serde_json::from_str("{}").unwrap();
        assert!(response.status().is_none());
        assert!(!response.is_success());
    }

    #[test]
    fn test_create_response_default() {
        let response = CreateResponse::default();
        assert!(response.status().is_none());
        assert!(!response.is_success());
    }
}
