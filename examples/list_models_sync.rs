//! Example: List locally available models (sync)
//!
//! This example demonstrates how to fetch and display all models
//! installed on the Ollama server using the blocking API.
//!
//! # Usage
//!
//! ```bash
//! cargo run --example list_models_sync
//! ```

use ollama_oxide::{OllamaApiSync, OllamaClient, Result};

fn main() -> Result<()> {
    // Create client with default configuration
    let client = OllamaClient::default()?;

    // List all available models (blocking)
    let response = client.list_models_blocking()?;

    println!("Available models ({}):", response.models.len());

    for model in &response.models {
        let size_str = model
            .size
            .map(|s| format!("{:.2} GB", s as f64 / 1_073_741_824.0))
            .unwrap_or_else(|| "unknown".to_string());

        println!("  - {} ({})", model.name, size_str);
    }

    if response.models.is_empty() {
        println!("\nNo models found. Try pulling a model first:");
        println!("  ollama pull llama3.2");
    }

    Ok(())
}
