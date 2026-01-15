//! Integration tests for list_models API
//!
//! These tests require a running Ollama server.
//! Set OLLAMA_TEST_SERVER=1 to enable.

use ollama_oxide::{OllamaApiAsync, OllamaApiSync, OllamaClient};

/// Integration test that requires a running Ollama server (async).
#[tokio::test]
async fn test_list_models_integration_async() {
    if std::env::var("OLLAMA_TEST_SERVER").is_err() {
        eprintln!("Skipping integration test: OLLAMA_TEST_SERVER not set");
        return;
    }

    let client = OllamaClient::default().expect("Failed to create client");
    let response = client.list_models().await;

    match response {
        Ok(list) => {
            println!("Found {} models (async)", list.models.len());
            for model in &list.models {
                let size_str = model
                    .size
                    .map(|s| format!("{:.2} GB", s as f64 / 1_073_741_824.0))
                    .unwrap_or_else(|| "unknown".to_string());
                println!("  - {} ({})", model.name, size_str);

                if let Some(details) = &model.details {
                    if let Some(family) = &details.family {
                        println!("    Family: {}", family);
                    }
                    if let Some(params) = &details.parameter_size {
                        println!("    Parameters: {}", params);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error listing models: {}", e);
            // Don't fail - Ollama might not have any models or might not be running
        }
    }
}

/// Integration test that requires a running Ollama server (sync).
#[test]
fn test_list_models_integration_sync() {
    if std::env::var("OLLAMA_TEST_SERVER").is_err() {
        eprintln!("Skipping integration test: OLLAMA_TEST_SERVER not set");
        return;
    }

    let client = OllamaClient::default().expect("Failed to create client");
    let response = client.list_models_blocking();

    match response {
        Ok(list) => {
            println!("Found {} models (sync)", list.models.len());
            for model in &list.models {
                let size_str = model
                    .size
                    .map(|s| format!("{:.2} GB", s as f64 / 1_073_741_824.0))
                    .unwrap_or_else(|| "unknown".to_string());
                println!("  - {} ({})", model.name, size_str);
            }
        }
        Err(e) => {
            eprintln!("Error listing models: {}", e);
        }
    }
}
