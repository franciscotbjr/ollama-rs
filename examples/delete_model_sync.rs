//! Example: Delete a model (sync)
//!
//! This example demonstrates how to delete a model from the Ollama server
//! using the blocking API.
//!
//! Run with: cargo run --example delete_model_sync
//!
//! Note: Requires a running Ollama server.
//! WARNING: This will permanently delete the specified model!
//!
//! Tip: First run `cargo run --example copy_model_sync` to create a backup model,
//! then use this example to delete it. This avoids deleting important models.

use ollama_oxide::{DeleteRequest, OllamaApiSync, OllamaClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with default configuration
    let client = OllamaClient::default()?;

    // Model to delete - using "llama3.1-backup" which can be created by copy_model example
    let model_name = "llama3.1-backup";

    println!("WARNING: This will permanently delete '{}'", model_name);
    println!("Deleting model...");

    // Create delete request
    let request = DeleteRequest::new(model_name);

    // Execute delete
    match client.delete_model_blocking(&request) {
        Ok(()) => {
            println!("Model '{}' deleted successfully!", model_name);
        }
        Err(e) => {
            eprintln!("Failed to delete model: {}", e);
            eprintln!("The model '{}' may not exist.", model_name);
        }
    }

    Ok(())
}
