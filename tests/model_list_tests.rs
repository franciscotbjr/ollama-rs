//! Tests for list models primitive types

use ollama_oxide::{ListResponse, ModelDetails, ModelSummary};

// ============================================================================
// ModelDetails Tests
// ============================================================================

#[test]
fn test_model_details_deserialization() {
    let json = r#"{
        "format": "gguf",
        "family": "gemma",
        "families": ["gemma"],
        "parameter_size": "4.3B",
        "quantization_level": "Q4_K_M"
    }"#;

    let details: ModelDetails = serde_json::from_str(json).unwrap();
    assert_eq!(details.format, Some("gguf".to_string()));
    assert_eq!(details.family, Some("gemma".to_string()));
    assert_eq!(details.families, Some(vec!["gemma".to_string()]));
    assert_eq!(details.parameter_size, Some("4.3B".to_string()));
    assert_eq!(details.quantization_level, Some("Q4_K_M".to_string()));
}

#[test]
fn test_model_details_partial_deserialization() {
    let json = r#"{"format": "gguf"}"#;

    let details: ModelDetails = serde_json::from_str(json).unwrap();
    assert_eq!(details.format, Some("gguf".to_string()));
    assert_eq!(details.family, None);
    assert_eq!(details.families, None);
    assert_eq!(details.parameter_size, None);
    assert_eq!(details.quantization_level, None);
}

#[test]
fn test_model_details_empty_deserialization() {
    let json = r#"{}"#;

    let details: ModelDetails = serde_json::from_str(json).unwrap();
    assert_eq!(details.format, None);
    assert_eq!(details.family, None);
    assert_eq!(details.families, None);
    assert_eq!(details.parameter_size, None);
    assert_eq!(details.quantization_level, None);
}

#[test]
fn test_model_details_multiple_families() {
    let json = r#"{
        "families": ["llama", "codellama", "instruct"]
    }"#;

    let details: ModelDetails = serde_json::from_str(json).unwrap();
    assert_eq!(
        details.families,
        Some(vec![
            "llama".to_string(),
            "codellama".to_string(),
            "instruct".to_string()
        ])
    );
}

// ============================================================================
// ModelSummary Tests
// ============================================================================

#[test]
fn test_model_summary_full_deserialization() {
    let json = r#"{
        "name": "gemma3",
        "modified_at": "2025-10-03T23:34:03.409490317-07:00",
        "size": 3338801804,
        "digest": "a2af6cc3eb7fa8be8504abaf9b04e88f17a119ec3f04a3addf55f92841195f5a",
        "details": {
            "format": "gguf",
            "family": "gemma"
        }
    }"#;

    let summary: ModelSummary = serde_json::from_str(json).unwrap();
    assert_eq!(summary.name, "gemma3");
    assert_eq!(
        summary.modified_at,
        Some("2025-10-03T23:34:03.409490317-07:00".to_string())
    );
    assert_eq!(summary.size, Some(3338801804));
    assert_eq!(
        summary.digest,
        Some("a2af6cc3eb7fa8be8504abaf9b04e88f17a119ec3f04a3addf55f92841195f5a".to_string())
    );
    assert!(summary.details.is_some());

    let details = summary.details.unwrap();
    assert_eq!(details.format, Some("gguf".to_string()));
    assert_eq!(details.family, Some("gemma".to_string()));
}

#[test]
fn test_model_summary_minimal() {
    let json = r#"{"name": "llama3"}"#;

    let summary: ModelSummary = serde_json::from_str(json).unwrap();
    assert_eq!(summary.name, "llama3");
    assert_eq!(summary.modified_at, None);
    assert_eq!(summary.size, None);
    assert_eq!(summary.digest, None);
    assert_eq!(summary.details, None);
}

#[test]
fn test_model_summary_large_size() {
    // Test with very large model size (100GB+)
    let json = r#"{"name": "large-model", "size": 107374182400}"#;

    let summary: ModelSummary = serde_json::from_str(json).unwrap();
    assert_eq!(summary.name, "large-model");
    assert_eq!(summary.size, Some(107374182400)); // 100 GB
}

// ============================================================================
// ListResponse Tests
// ============================================================================

#[test]
fn test_list_response_multiple_models() {
    let json = r#"{
        "models": [
            {
                "name": "gemma3",
                "size": 3338801804
            },
            {
                "name": "llama3",
                "size": 4000000000
            },
            {
                "name": "mistral",
                "size": 7000000000
            }
        ]
    }"#;

    let response: ListResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.models.len(), 3);
    assert_eq!(response.models[0].name, "gemma3");
    assert_eq!(response.models[1].name, "llama3");
    assert_eq!(response.models[2].name, "mistral");
}

#[test]
fn test_list_response_empty() {
    let json = r#"{"models": []}"#;

    let response: ListResponse = serde_json::from_str(json).unwrap();
    assert!(response.models.is_empty());
}

#[test]
fn test_list_response_missing_models() {
    let json = r#"{}"#;

    let response: ListResponse = serde_json::from_str(json).unwrap();
    assert!(response.models.is_empty());
}

#[test]
fn test_list_response_single_model() {
    let json = r#"{
        "models": [
            {"name": "only-one"}
        ]
    }"#;

    let response: ListResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.models.len(), 1);
    assert_eq!(response.models[0].name, "only-one");
}

// ============================================================================
// Thread Safety Tests
// ============================================================================

#[test]
fn test_types_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<ModelDetails>();
    assert_send_sync::<ModelSummary>();
    assert_send_sync::<ListResponse>();
}

// ============================================================================
// Serialization Round-Trip Tests
// ============================================================================

#[test]
fn test_model_details_round_trip() {
    let original = ModelDetails {
        format: Some("gguf".to_string()),
        family: Some("llama".to_string()),
        families: Some(vec!["llama".to_string(), "instruct".to_string()]),
        parameter_size: Some("7B".to_string()),
        quantization_level: Some("Q4_K_M".to_string()),
    };

    let json = serde_json::to_string(&original).unwrap();
    let deserialized: ModelDetails = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_model_summary_round_trip() {
    let original = ModelSummary {
        name: "test-model".to_string(),
        modified_at: Some("2025-01-01T00:00:00Z".to_string()),
        size: Some(5000000000),
        digest: Some("abc123def456".to_string()),
        details: Some(ModelDetails {
            format: Some("gguf".to_string()),
            family: Some("llama".to_string()),
            families: None,
            parameter_size: Some("7B".to_string()),
            quantization_level: None,
        }),
    };

    let json = serde_json::to_string(&original).unwrap();
    let deserialized: ModelSummary = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_list_response_round_trip() {
    let original = ListResponse {
        models: vec![
            ModelSummary {
                name: "model-a".to_string(),
                modified_at: Some("2025-01-01T00:00:00Z".to_string()),
                size: Some(1000000),
                digest: Some("abc123".to_string()),
                details: Some(ModelDetails {
                    format: Some("gguf".to_string()),
                    family: Some("llama".to_string()),
                    families: Some(vec!["llama".to_string()]),
                    parameter_size: Some("7B".to_string()),
                    quantization_level: Some("Q4_0".to_string()),
                }),
            },
            ModelSummary {
                name: "model-b".to_string(),
                modified_at: None,
                size: Some(2000000),
                digest: None,
                details: None,
            },
        ],
    };

    let json = serde_json::to_string(&original).unwrap();
    let deserialized: ListResponse = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

// ============================================================================
// Clone and Debug Tests
// ============================================================================

#[test]
fn test_model_details_clone() {
    let original = ModelDetails {
        format: Some("gguf".to_string()),
        family: Some("llama".to_string()),
        families: None,
        parameter_size: None,
        quantization_level: None,
    };

    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_model_summary_clone() {
    let original = ModelSummary {
        name: "test".to_string(),
        modified_at: None,
        size: Some(1000),
        digest: None,
        details: None,
    };

    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_list_response_clone() {
    let original = ListResponse {
        models: vec![ModelSummary {
            name: "test".to_string(),
            modified_at: None,
            size: None,
            digest: None,
            details: None,
        }],
    };

    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_model_details_debug() {
    let details = ModelDetails {
        format: Some("gguf".to_string()),
        family: None,
        families: None,
        parameter_size: None,
        quantization_level: None,
    };

    let debug_str = format!("{:?}", details);
    assert!(debug_str.contains("ModelDetails"));
    assert!(debug_str.contains("gguf"));
}

#[test]
fn test_list_response_debug() {
    let response = ListResponse { models: vec![] };

    let debug_str = format!("{:?}", response);
    assert!(debug_str.contains("ListResponse"));
}
