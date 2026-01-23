//! Example: Generate embeddings (async)
//!
//! This example demonstrates how to generate text embeddings using
//! an embedding model.
//!
//! Run with: cargo run --example embed_async
//!
//! Note: Requires a running Ollama server with an embedding model
//! (e.g., nomic-embed-text, all-minilm, mxbai-embed-large)

use ollama_oxide::{EmbedInput, EmbedRequest, OllamaApiAsync, OllamaClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with default configuration
    let client = OllamaClient::default()?;

    // Model to use (change to your installed embedding model)
    let model = "nomic-embed-text";

    println!("Generating embeddings with model: {}", model);

    // Example 1: Single text embedding
    println!("\n--- Single Text Embedding ---");
    let request = EmbedRequest::new(model, "The quick brown fox jumps over the lazy dog.");
    let response = client.embed(&request).await?;

    println!("Model: {:?}", response.model);
    println!("Embedding dimensions: {:?}", response.dimensions());
    println!("Total duration: {:?} ms", response.total_duration_ms());
    println!("Tokens processed: {:?}", response.prompt_eval_count);

    if let Some(embedding) = response.first_embedding() {
        println!("First 5 values: {:?}", &embedding[..5.min(embedding.len())]);
    }

    // Example 2: Multiple text embeddings
    println!("\n--- Multiple Text Embeddings ---");
    let texts = vec![
        "Artificial intelligence is transforming industries.",
        "Machine learning models require training data.",
        "Neural networks can learn complex patterns.",
    ];

    let request = EmbedRequest::new(model, EmbedInput::multiple(texts.clone()));
    let response = client.embed(&request).await?;

    println!("Generated {} embeddings", response.len());
    for (i, embedding) in response.embeddings.iter().enumerate() {
        println!(
            "  Text {}: '{}...' -> {} dimensions",
            i + 1,
            &texts[i][..30.min(texts[i].len())],
            embedding.len()
        );
    }

    // Example 3: With options
    println!("\n--- With Options ---");
    let request = EmbedRequest::new(model, "Hello, world!")
        .with_truncate(true)
        .with_keep_alive("5m");

    let response = client.embed(&request).await?;
    println!(
        "Embedding with options: {:?} dimensions",
        response.dimensions()
    );

    println!("\nDone!");

    Ok(())
}
