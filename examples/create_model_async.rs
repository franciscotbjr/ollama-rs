//! Example: Create a custom model from an existing model (async)
//!
//! This example demonstrates creating a custom model with a custom system prompt.
//!
//! Prerequisites:
//! - Ollama server running on localhost:11434
//! - Base model pulled (e.g., `ollama pull qwen3:0.6b`)
//!
//! Run with: cargo run --example create_model_async

use ollama_oxide::{ChatMessage, CreateRequest, OllamaApiAsync, OllamaClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Create Model Example (Async) ===\n");

    let client = OllamaClient::default()?;

    // Create a Mario-style model
    let request = CreateRequest::from_model("mario-test", "qwen3:0.6b")
        .with_system(
            "You are Mario from Super Mario Bros. Always respond with enthusiasm \
             and use Mario's catchphrases like 'It's-a me!' and 'Let's-a go!'",
        )
        .with_messages([
            ChatMessage::user("Who are you?"),
            ChatMessage::assistant(
                "It's-a me, Mario! I'm here to help you with anything you need. Let's-a go!",
            ),
        ]);

    println!("Creating model: mario-test");
    println!("Base model: qwen3:0.6b");
    println!("System prompt: You are Mario from Super Mario Bros...\n");

    match client.create_model(&request).await {
        Ok(response) => {
            println!("Status: {:?}", response.status());
            if response.is_success() {
                println!("\nModel created successfully!");
                println!("You can now use: ollama run mario-test");
            }
        }
        Err(e) => {
            eprintln!("Error creating model: {}", e);
            eprintln!("\nMake sure:");
            eprintln!("  1. Ollama server is running");
            eprintln!("  2. Base model is available: ollama pull qwen3:0.6b");
        }
    }

    Ok(())
}
