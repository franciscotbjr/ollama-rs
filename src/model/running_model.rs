//! Running model primitive type

use serde::{Deserialize, Serialize};

use super::ModelDetails;

/// Information about a model currently loaded in memory
///
/// Contains runtime information about a model including VRAM usage,
/// context length, and expiration time.
///
/// # Example
///
/// ```json
/// {
///   "model": "gemma3",
///   "size": 6591830464,
///   "digest": "a2af6cc3...",
///   "details": { "format": "gguf", "family": "gemma3" },
///   "expires_at": "2025-10-17T16:47:07.93355-07:00",
///   "size_vram": 5333539264,
///   "context_length": 4096
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunningModel {
    /// Model name (e.g., "llama3.2", "gemma3")
    pub model: String,

    /// Total size of the model in bytes
    #[serde(default)]
    pub size: Option<u64>,

    /// SHA256 digest identifier of the model contents
    #[serde(default)]
    pub digest: Option<String>,

    /// Additional information about the model
    #[serde(default)]
    pub details: Option<ModelDetails>,

    /// Time when the model will be unloaded from memory (ISO 8601)
    #[serde(default)]
    pub expires_at: Option<String>,

    /// VRAM (GPU memory) usage in bytes
    #[serde(default)]
    pub size_vram: Option<u64>,

    /// Context length for the running model
    #[serde(default)]
    pub context_length: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_model_deserialization() {
        let json = r#"{
            "model": "gemma3",
            "size": 6591830464,
            "digest": "a2af6cc3eb7fa8be8504abaf9b04e88f17a119ec3f04a3addf55f92841195f5a",
            "expires_at": "2025-10-17T16:47:07.93355-07:00",
            "size_vram": 5333539264,
            "context_length": 4096
        }"#;

        let model: RunningModel = serde_json::from_str(json).unwrap();
        assert_eq!(model.model, "gemma3");
        assert_eq!(model.size, Some(6591830464));
        assert_eq!(model.size_vram, Some(5333539264));
        assert_eq!(model.context_length, Some(4096));
    }

    #[test]
    fn test_running_model_minimal() {
        let json = r#"{"model": "llama3.2"}"#;
        let model: RunningModel = serde_json::from_str(json).unwrap();
        assert_eq!(model.model, "llama3.2");
        assert!(model.size.is_none());
        assert!(model.expires_at.is_none());
    }

    #[test]
    fn test_running_model_serialization_roundtrip() {
        let model = RunningModel {
            model: "test-model".to_string(),
            size: Some(1000),
            digest: Some("abc123".to_string()),
            details: None,
            expires_at: Some("2025-01-01T00:00:00Z".to_string()),
            size_vram: Some(500),
            context_length: Some(2048),
        };

        let json = serde_json::to_string(&model).unwrap();
        let deserialized: RunningModel = serde_json::from_str(&json).unwrap();
        assert_eq!(model, deserialized);
    }

    #[test]
    fn test_running_model_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<RunningModel>();
    }

    #[test]
    fn test_running_model_with_details() {
        let json = r#"{
            "model": "gemma3",
            "size": 6591830464,
            "details": {
                "parent_model": "",
                "format": "gguf",
                "family": "gemma3",
                "families": ["gemma3"],
                "parameter_size": "4.3B",
                "quantization_level": "Q4_K_M"
            }
        }"#;

        let model: RunningModel = serde_json::from_str(json).unwrap();
        assert_eq!(model.model, "gemma3");
        assert!(model.details.is_some());
        let details = model.details.unwrap();
        assert_eq!(details.family, Some("gemma3".to_string()));
        assert_eq!(details.format, Some("gguf".to_string()));
    }
}
