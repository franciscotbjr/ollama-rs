//! Example: Show model information (sync)
//!
//! This example demonstrates how to retrieve detailed information about a model
//! using the synchronous (blocking) API.
//!
//! Run with: cargo run --example show_model_sync
//!
//! Note: Requires a running Ollama server with at least one model installed.

use ollama_oxide::{OllamaApiSync, OllamaClient, ShowRequest};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with default configuration
    let client = OllamaClient::default()?;

    // Model to query
    let model_name = "llama3.1";

    println!("Fetching information for model '{}'...\n", model_name);

    // Create show request (verbose for more details)
    let request = ShowRequest::verbose(model_name);

    // Execute request
    match client.show_model_blocking(&request) {
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
                if let Some(parent) = &details.parent_model
                    && !parent.is_empty()
                {
                    println!("  Parent Model: {}", parent);
                }
            }

            // Display capabilities
            if let Some(caps) = &response.capabilities {
                println!("\nCapabilities:");
                for cap in caps {
                    println!("  - {}", cap);
                }
            }

            // Display template if available
            if let Some(template) = &response.template {
                println!("\nTemplate Preview:");
                let preview: String = template.chars().take(200).collect();
                println!("  {}", preview.replace('\n', "\n  "));
                if template.len() > 200 {
                    println!("  ... (truncated)");
                }
            }

            // Display parameters if available
            if let Some(params) = &response.parameters {
                println!("\nParameters:");
                for line in params.lines().take(10) {
                    println!("  {}", line);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to get model info: {}", e);
            eprintln!("Make sure the model '{}' exists.", model_name);
        }
    }

    Ok(())
}
