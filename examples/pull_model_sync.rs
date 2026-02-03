//! Example: Pull (download) a model synchronously
//!
//! This example demonstrates how to download a model from the Ollama registry
//! using the blocking API.
//!
//! # Prerequisites
//!
//! - Ollama server running at http://localhost:11434
//! - Internet connection to download from registry
//!
//! # Usage
//!
//! ```sh
//! cargo run --example pull_model_sync --features model
//! ```

use ollama_oxide::{OllamaApiSync, OllamaClient, PullRequest};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Pull Model Example (Sync) ===\n");

    // Create client with default configuration
    let client = OllamaClient::default()?;

    // Pull a small model (all-minilm:33m is very small ~67MB)
    let model_name = "all-minilm:33m";
    println!("Pulling model: {}", model_name);
    println!("This may take a few minutes depending on your connection...\n");

    let request = PullRequest::new(model_name);
    let response = client.pull_model_blocking(&request)?;

    if response.is_success() {
        println!("Model '{}' downloaded successfully!", model_name);
    } else {
        println!("Pull status: {:?}", response.status());
    }

    Ok(())
}
