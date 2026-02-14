//! Example: Show model information (async)
//!
//! This example demonstrates how to retrieve detailed information about a model.
//!
//! Run with: cargo run --example show_model_async
//!
//! Note: Requires a running Ollama server with at least one model installed.

use ollama_oxide::{OllamaApiAsync, OllamaClient, ShowRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with default configuration
    let client = OllamaClient::default()?;

    // Model to query
    let model_name = "llama3.1";

    println!("Fetching information for model '{}'...\n", model_name);

    // Create show request
    let request = ShowRequest::new(model_name);

    // Execute request
    match client.show_model(&request).await {
        Ok(response) => {
            // Display model details
            if let Some(details) = &response.details {
                println!("Model Details:");
                if let Some(family) = &details.family {
                    println!("  Family: {}", family);
                }
                if let Some(param_size) = &details.parameter_size {
                    println!("  Parameter Size: {}", param_size);
                }
                if let Some(format) = &details.format {
                    println!("  Format: {}", format);
                }
                if let Some(quant) = &details.quantization_level {
                    println!("  Quantization: {}", quant);
                }
            }

            // Display capabilities
            if let Some(caps) = &response.capabilities {
                println!("\nCapabilities:");
                for cap in caps {
                    println!("  - {}", cap);
                }
            }

            // Check specific capabilities
            println!("\nCapability Checks:");
            println!(
                "  Supports completion: {}",
                response.has_capability("completion")
            );
            println!("  Supports vision: {}", response.has_capability("vision"));
            println!("  Supports tools: {}", response.has_capability("tools"));

            // Display license if available
            if let Some(license) = &response.license {
                println!("\nLicense: {}", &license[..license.len().min(100)]);
                if license.len() > 100 {
                    println!("  ... (truncated)");
                }
            }

            // Display modified date
            if let Some(modified) = &response.modified_at {
                println!("\nLast Modified: {}", modified);
            }
        }
        Err(e) => {
            eprintln!("Failed to get model info: {}", e);
            eprintln!("Make sure the model '{}' exists.", model_name);
        }
    }

    Ok(())
}
