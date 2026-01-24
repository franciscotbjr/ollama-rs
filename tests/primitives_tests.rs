// Primitives Tests - Phase 0 TDD
// These tests validate the VersionResponse type

use ollama_oxide::VersionResponse;

#[test]
fn test_version_response_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<VersionResponse>();
}

#[test]
fn test_version_response_deserialization_valid() {
    let json = r#"{"version":"0.12.6"}"#;
    let response: VersionResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.version, "0.12.6");
}

#[test]
fn test_version_response_deserialization_different_version() {
    let json = r#"{"version":"1.0.0"}"#;
    let response: VersionResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.version, "1.0.0");
}

#[test]
fn test_version_response_serialization() {
    let response = VersionResponse {
        version: "0.12.6".to_string(),
    };
    let json = serde_json::to_string(&response).unwrap();
    assert!(json.contains("0.12.6"));
    assert!(json.contains("version"));
}

#[test]
fn test_version_response_serialization_round_trip() {
    let original = VersionResponse {
        version: "1.2.3".to_string(),
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: VersionResponse = serde_json::from_str(&json).unwrap();
    assert_eq!(original.version, deserialized.version);
}

#[test]
fn test_version_response_with_empty_string() {
    let json = r#"{"version":""}"#;
    let response: VersionResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.version, "");
}

#[test]
fn test_version_response_deserialization_invalid_missing_field() {
    let json = r#"{}"#;
    let result: Result<VersionResponse, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_version_response_deserialization_invalid_type() {
    let json = r#"{"version":123}"#;
    let result: Result<VersionResponse, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_version_response_clone() {
    let response = VersionResponse {
        version: "0.12.6".to_string(),
    };
    let cloned = response.clone();
    assert_eq!(response.version, cloned.version);
}

#[test]
fn test_version_response_debug() {
    let response = VersionResponse {
        version: "0.12.6".to_string(),
    };
    let debug = format!("{:?}", response);
    assert!(debug.contains("VersionResponse"));
    assert!(debug.contains("0.12.6"));
}

#[test]
fn test_version_response_partial_eq() {
    let response1 = VersionResponse {
        version: "0.12.6".to_string(),
    };
    let response2 = VersionResponse {
        version: "0.12.6".to_string(),
    };
    let response3 = VersionResponse {
        version: "1.0.0".to_string(),
    };

    assert_eq!(response1, response2);
    assert_ne!(response1, response3);
}
