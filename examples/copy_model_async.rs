//! Example: Copy a model (async)
//!
//! This example demonstrates how to create a copy of an existing model.
//!
//! Run with: cargo run --example copy_model_async
//!
//! Note: Requires a running Ollama server with at least one model installed.

use ollama_oxide::{CopyRequest, OllamaApiAsync, OllamaClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with default configuration
    let client = OllamaClient::default()?;

    // Define source and destination
    let source = "llama3.1";
    let destination = "llama3.1-backup";

    println!("Copying model '{}' to '{}'...", source, destination);

    // Create copy request
    let request = CopyRequest::new(source, destination);

    // Execute copy
    match client.copy_model(&request).await {
        Ok(()) => {
            println!("Model copied successfully!");
            println!("You can now use '{}' as a separate model.", destination);
        }
        Err(e) => {
            eprintln!("Failed to copy model: {}", e);
            eprintln!("Make sure the source model '{}' exists.", source);
        }
    }

    Ok(())
}
