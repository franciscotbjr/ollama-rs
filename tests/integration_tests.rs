// Integration Tests - Phase 0 TDD
// These tests validate against a real Ollama server (when available)

use ollama_oxide::{ClientConfig, OllamaApiAsync, OllamaApiSync, OllamaClient};
use std::time::Duration;

// These tests are conditional - they only run if OLLAMA_TEST_SERVER env var is set
// This allows CI/CD to skip them if no Ollama server is available

fn should_run_integration_tests() -> bool {
    std::env::var("OLLAMA_TEST_SERVER").is_ok()
}

fn get_test_server_url() -> String {
    std::env::var("OLLAMA_TEST_SERVER")
        .unwrap_or_else(|_| "http://localhost:11434".to_string())
}

#[tokio::test]
async fn test_integration_async_version_with_real_server() {
    if !should_run_integration_tests() {
        println!("Skipping integration test - set OLLAMA_TEST_SERVER to enable");
        return;
    }

    let config = ClientConfig {
        base_url: get_test_server_url(),
        timeout: Duration::from_secs(30),
        max_retries: 3,
    };

    let client = OllamaClient::new(config).expect("Failed to create client");
    let result = client.version().await;

    assert!(result.is_ok(), "Failed to get version from real server");
    let version = result.unwrap();
    assert!(!version.version.is_empty(), "Version should not be empty");
    println!("Real Ollama version: {}", version.version);
}

#[test]
fn test_integration_sync_version_with_real_server() {
    if !should_run_integration_tests() {
        println!("Skipping integration test - set OLLAMA_TEST_SERVER to enable");
        return;
    }

    let config = ClientConfig {
        base_url: get_test_server_url(),
        timeout: Duration::from_secs(30),
        max_retries: 3,
    };

    let client = OllamaClient::new(config).expect("Failed to create client");
    let result = client.version_blocking();

    assert!(result.is_ok(), "Failed to get version from real server");
    let version = result.unwrap();
    assert!(!version.version.is_empty(), "Version should not be empty");
    println!("Real Ollama version: {}", version.version);
}

#[tokio::test]
async fn test_integration_async_with_default_client() {
    if !should_run_integration_tests() {
        println!("Skipping integration test - set OLLAMA_TEST_SERVER to enable");
        return;
    }

    let client = OllamaClient::default().expect("Failed to create default client");
    let result = client.version().await;

    // May fail if server is not on localhost:11434, but shouldn't panic
    if let Ok(version) = result {
        assert!(!version.version.is_empty());
        println!("Version with default client: {}", version.version);
    }
}

#[test]
fn test_integration_sync_with_default_client() {
    if !should_run_integration_tests() {
        println!("Skipping integration test - set OLLAMA_TEST_SERVER to enable");
        return;
    }

    let client = OllamaClient::default().expect("Failed to create default client");
    let result = client.version_blocking();

    // May fail if server is not on localhost:11434, but shouldn't panic
    if let Ok(version) = result {
        assert!(!version.version.is_empty());
        println!("Version with default client: {}", version.version);
    }
}

#[tokio::test]
async fn test_integration_async_multiple_calls() {
    if !should_run_integration_tests() {
        println!("Skipping integration test - set OLLAMA_TEST_SERVER to enable");
        return;
    }

    let config = ClientConfig {
        base_url: get_test_server_url(),
        timeout: Duration::from_secs(30),
        max_retries: 3,
    };

    let client = OllamaClient::new(config).expect("Failed to create client");

    // Make multiple calls to verify consistency
    for i in 0..5 {
        let result = client.version().await;
        assert!(
            result.is_ok(),
            "Call {} failed: {:?}",
            i,
            result.err()
        );
        let version = result.unwrap();
        assert!(!version.version.is_empty());
        println!("Call {}: version = {}", i, version.version);
    }
}

#[test]
fn test_integration_sync_multiple_calls() {
    if !should_run_integration_tests() {
        println!("Skipping integration test - set OLLAMA_TEST_SERVER to enable");
        return;
    }

    let config = ClientConfig {
        base_url: get_test_server_url(),
        timeout: Duration::from_secs(30),
        max_retries: 3,
    };

    let client = OllamaClient::new(config).expect("Failed to create client");

    // Make multiple calls to verify consistency
    for i in 0..5 {
        let result = client.version_blocking();
        assert!(
            result.is_ok(),
            "Call {} failed: {:?}",
            i,
            result.err()
        );
        let version = result.unwrap();
        assert!(!version.version.is_empty());
        println!("Call {}: version = {}", i, version.version);
    }
}

#[tokio::test]
async fn test_integration_async_with_custom_timeout() {
    if !should_run_integration_tests() {
        println!("Skipping integration test - set OLLAMA_TEST_SERVER to enable");
        return;
    }

    let config = ClientConfig {
        base_url: get_test_server_url(),
        timeout: Duration::from_secs(5), // Short timeout
        max_retries: 1,
    };

    let client = OllamaClient::new(config).expect("Failed to create client");
    let result = client.version().await;

    assert!(result.is_ok(), "Version call with custom timeout failed");
}

#[test]
fn test_integration_sync_with_custom_timeout() {
    if !should_run_integration_tests() {
        println!("Skipping integration test - set OLLAMA_TEST_SERVER to enable");
        return;
    }

    let config = ClientConfig {
        base_url: get_test_server_url(),
        timeout: Duration::from_secs(5), // Short timeout
        max_retries: 1,
    };

    let client = OllamaClient::new(config).expect("Failed to create client");
    let result = client.version_blocking();

    assert!(result.is_ok(), "Version call with custom timeout failed");
}
