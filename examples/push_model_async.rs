//! Example: Push (upload) a model to Ollama registry (async)
//!
//! This example demonstrates how to push a local model to a remote
//! Ollama registry using the async API.
//!
//! # Prerequisites
//!
//! - Ollama server running at http://localhost:11434
//! - A local model that you want to push
//! - Proper authentication configured for the registry
//!
//! # Running
//!
//! ```bash
//! cargo run --example push_model_async --features model
//! ```
//!
//! # Note
//!
//! Pushing models requires:
//! 1. The model to exist locally
//! 2. Proper namespace permissions (e.g., "myuser/mymodel")
//! 3. Registry authentication (typically via `ollama login`)

use ollama_oxide::{OllamaApiAsync, OllamaClient, PushRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with default configuration
    let client = OllamaClient::default()?;

    // Specify the model to push
    // Format: "namespace/model:tag" (e.g., "myuser/mymodel:latest")
    let model_name = "myuser/mymodel:latest";

    println!("Pushing model: {}", model_name);

    // Create the push request
    let request = PushRequest::new(model_name);

    // Push the model (non-streaming in v0.1.0)
    match client.push_model(&request).await {
        Ok(response) => {
            println!("Push completed!");
            println!("Status: {:?}", response.status());
            if response.is_success() {
                println!("Model successfully pushed to registry!");
            }
        }
        Err(e) => {
            eprintln!("Failed to push model: {}", e);
            eprintln!("Make sure:");
            eprintln!("  1. The model exists locally");
            eprintln!("  2. You have proper namespace permissions");
            eprintln!("  3. You are authenticated (run 'ollama login' if needed)");
        }
    }

    Ok(())
}
