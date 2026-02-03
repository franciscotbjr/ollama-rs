use serde::{Deserialize, Serialize};

/// Response from POST /api/pull endpoint.
///
/// Contains the status of the pull operation.
///
/// # JSON Example
///
/// ```json
/// {
///   "status": "success"
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullResponse {
    /// Status message indicating the result of the operation
    #[serde(default)]
    pub status: Option<String>,
}

impl PullResponse {
    /// Get the status message.
    ///
    /// # Returns
    ///
    /// The status string if present, or None.
    pub fn status(&self) -> Option<&str> {
        self.status.as_deref()
    }

    /// Check if the pull operation was successful.
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
        let response = PullResponse {
            status: Some("success".to_string()),
        };
        assert_eq!(response.status(), Some("success"));
    }

    #[test]
    fn test_status_returns_none_when_missing() {
        let response = PullResponse { status: None };
        assert_eq!(response.status(), None);
    }

    #[test]
    fn test_is_success_true() {
        let response = PullResponse {
            status: Some("success".to_string()),
        };
        assert!(response.is_success());
    }

    #[test]
    fn test_is_success_false_on_other_status() {
        let response = PullResponse {
            status: Some("downloading".to_string()),
        };
        assert!(!response.is_success());
    }

    #[test]
    fn test_is_success_false_on_none() {
        let response = PullResponse { status: None };
        assert!(!response.is_success());
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{"status": "success"}"#;
        let response: PullResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.status(), Some("success"));
        assert!(response.is_success());
    }

    #[test]
    fn test_deserialization_empty_object() {
        let json = r#"{}"#;
        let response: PullResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.status(), None);
        assert!(!response.is_success());
    }
}
