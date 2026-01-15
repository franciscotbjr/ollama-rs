//! Example: List running models (sync/blocking)
//!
//! This example demonstrates how to retrieve the list of models
//! currently loaded into memory on the Ollama server using the
//! synchronous (blocking) API.
//!
//! Run with: cargo run --example list_running_models_sync

use ollama_oxide::{OllamaApiSync, OllamaClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with default configuration
    let client = OllamaClient::default()?;

    // List running models (blocking)
    let response = client.list_running_models_blocking()?;

    if response.models.is_empty() {
        println!("No models currently running.");
        println!("Tip: Load a model with 'ollama run <model>' to see it here.");
    } else {
        println!("Currently running models:");
        println!("{:-<60}", "");

        for model in &response.models {
            println!("Model: {}", model.model);

            if let Some(size) = model.size {
                println!("  Size: {:.2} GB", size as f64 / 1_073_741_824.0);
            }

            if let Some(vram) = model.size_vram {
                println!("  VRAM: {:.2} GB", vram as f64 / 1_073_741_824.0);
            }

            if let Some(ctx) = model.context_length {
                println!("  Context: {} tokens", ctx);
            }

            if let Some(expires) = &model.expires_at {
                println!("  Expires: {}", expires);
            }

            if let Some(details) = &model.details {
                if let Some(family) = &details.family {
                    println!("  Family: {}", family);
                }
                if let Some(quant) = &details.quantization_level {
                    println!("  Quantization: {}", quant);
                }
            }

            println!();
        }
    }

    Ok(())
}
