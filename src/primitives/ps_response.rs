//! List running models response primitive type

use serde::{Deserialize, Serialize};

use super::RunningModel;

/// Response from GET /api/ps endpoint
///
/// Contains a list of models currently loaded into memory.
///
/// # Example
///
/// ```json
/// {
///   "models": [
///     {
///       "model": "gemma3",
///       "size": 6591830464,
///       "expires_at": "2025-10-17T16:47:07.93355-07:00"
///     }
///   ]
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PsResponse {
    /// List of currently running models
    #[serde(default)]
    pub models: Vec<RunningModel>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ps_response_deserialization() {
        let json = r#"{
            "models": [
                {"model": "gemma3", "size": 6591830464},
                {"model": "llama3.2", "size": 3338801804}
            ]
        }"#;

        let response: PsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.models.len(), 2);
        assert_eq!(response.models[0].model, "gemma3");
        assert_eq!(response.models[1].model, "llama3.2");
    }

    #[test]
    fn test_ps_response_empty() {
        let json = r#"{"models": []}"#;
        let response: PsResponse = serde_json::from_str(json).unwrap();
        assert!(response.models.is_empty());
    }

    #[test]
    fn test_ps_response_missing_models_defaults_empty() {
        let json = r#"{}"#;
        let response: PsResponse = serde_json::from_str(json).unwrap();
        assert!(response.models.is_empty());
    }

    #[test]
    fn test_ps_response_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<PsResponse>();
    }

    #[test]
    fn test_ps_response_serialization_roundtrip() {
        let response = PsResponse {
            models: vec![RunningModel {
                model: "test-model".to_string(),
                size: Some(1000),
                digest: None,
                details: None,
                expires_at: None,
                size_vram: Some(500),
                context_length: Some(2048),
            }],
        };

        let json = serde_json::to_string(&response).unwrap();
        let deserialized: PsResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(response, deserialized);
    }
}
