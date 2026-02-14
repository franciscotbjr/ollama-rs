use serde::{Deserialize, Serialize};

/// Response from POST /api/push endpoint.
///
/// Contains the status of the push operation.
///
/// # JSON Example
///
/// ```json
/// {
///   "status": "success"
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushResponse {
    /// Status message indicating the result of the operation
    #[serde(default)]
    pub status: Option<String>,
}

impl PushResponse {
    /// Get the status message.
    ///
    /// # Returns
    ///
    /// The status string if present, or None.
    pub fn status(&self) -> Option<&str> {
        self.status.as_deref()
    }

    /// Check if the push operation was successful.
    ///
    /// # Returns
    ///
    /// `true` if status is "success", `false` otherwise.
    pub fn is_success(&self) -> bool {
        self.status.as_deref() == Some("success")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_returns_value() {
        let response = PushResponse {
            status: Some("success".to_string()),
        };
        assert_eq!(response.status(), Some("success"));
    }

    #[test]
    fn test_status_returns_none_when_missing() {
        let response = PushResponse { status: None };
        assert_eq!(response.status(), None);
    }

    #[test]
    fn test_is_success_true() {
        let response = PushResponse {
            status: Some("success".to_string()),
        };
        assert!(response.is_success());
    }

    #[test]
    fn test_is_success_false_on_other_status() {
        let response = PushResponse {
            status: Some("uploading".to_string()),
        };
        assert!(!response.is_success());
    }

    #[test]
    fn test_is_success_false_on_none() {
        let response = PushResponse { status: None };
        assert!(!response.is_success());
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{"status": "success"}"#;
        let response: PushResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.status(), Some("success"));
        assert!(response.is_success());
    }

    #[test]
    fn test_deserialization_empty_object() {
        let json = r#"{}"#;
        let response: PushResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.status(), None);
        assert!(!response.is_success());
    }
}
