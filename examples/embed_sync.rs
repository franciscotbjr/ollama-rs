//! Example: Generate embeddings (sync)
//!
//! This example demonstrates how to generate text embeddings using
//! the blocking API.
//!
//! Run with: cargo run --example embed_sync
//!
//! Note: Requires a running Ollama server with an embedding model

use ollama_oxide::{EmbedRequest, OllamaApiSync, OllamaClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with default configuration
    let client = OllamaClient::default()?;

    // Model to use (change to your installed embedding model)
    let model = "nomic-embed-text";

    println!("Generating embeddings with model: {}", model);

    // Generate embedding
    let request = EmbedRequest::new(model, "Hello, world!");
    let response = client.embed_blocking(&request)?;

    println!("Model: {:?}", response.model);
    println!("Embedding dimensions: {:?}", response.dimensions());
    println!("Total duration: {:?} ms", response.total_duration_ms());

    if let Some(embedding) = response.first_embedding() {
        println!("First 5 values: {:?}", &embedding[..5.min(embedding.len())]);
    }

    println!("\nDone!");

    Ok(())
}
