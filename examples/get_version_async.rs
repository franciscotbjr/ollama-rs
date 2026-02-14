//! Example: Get Ollama server version (async)
//!
//! This example demonstrates how to retrieve the Ollama server version
//! using the async API.
//!
//! # Usage
//!
//! ```bash
//! cargo run --example get_version_async
//! ```

use ollama_oxide::{OllamaApiAsync, OllamaClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create client with default configuration (http://localhost:11434)
    let client = OllamaClient::default()?;

    // Get Ollama server version
    let response = client.version().await?;

    println!("Ollama version: {}", response.version);

    Ok(())
}
