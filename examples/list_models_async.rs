//! Example: List locally available models (async)
//!
//! This example demonstrates how to fetch and display all models
//! installed on the Ollama server using the async API.
//!
//! # Usage
//!
//! ```bash
//! cargo run --example list_models_async
//! ```

use ollama_oxide::{OllamaApiAsync, OllamaClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create client with default configuration
    let client = OllamaClient::default()?;

    // List all available models
    let response = client.list_models().await?;

    println!("Available models ({}):", response.models.len());
    println!("{:-<60}", "");

    for model in &response.models {
        println!("Name: {}", model.name);

        if let Some(size) = model.size {
            let size_gb = size as f64 / 1_073_741_824.0;
            println!("  Size: {:.2} GB", size_gb);
        }

        if let Some(modified_at) = &model.modified_at {
            println!("  Modified: {}", modified_at);
        }

        if let Some(digest) = &model.digest {
            let short_digest = if digest.len() > 12 {
                &digest[..12]
            } else {
                digest
            };
            println!("  Digest: {}...", short_digest);
        }

        if let Some(details) = &model.details {
            if let Some(format) = &details.format {
                println!("  Format: {}", format);
            }
            if let Some(family) = &details.family {
                println!("  Family: {}", family);
            }
            if let Some(param_size) = &details.parameter_size {
                println!("  Parameters: {}", param_size);
            }
            if let Some(quant) = &details.quantization_level {
                println!("  Quantization: {}", quant);
            }
        }

        println!();
    }

    if response.models.is_empty() {
        println!("No models found. Try pulling a model first:");
        println!("  ollama pull llama3.2");
    }

    Ok(())
}
