//! Example: Get Ollama server version (sync)
//!
//! This example demonstrates how to retrieve the Ollama server version
//! using the blocking API.
//!
//! # Usage
//!
//! ```bash
//! cargo run --example get_version_sync
//! ```

use ollama_oxide::{OllamaApiSync, OllamaClient, Result};

fn main() -> Result<()> {
    // Create client with default configuration (http://localhost:11434)
    let client = OllamaClient::default()?;

    // Get Ollama server version (blocking)
    let response = client.version_blocking()?;

    println!("Ollama version: {}", response.version);

    Ok(())
}
