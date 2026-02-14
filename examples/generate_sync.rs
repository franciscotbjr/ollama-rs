//! Example: Generate text (sync/blocking)
//!
//! This example demonstrates how to generate text completions
//! using the synchronous (blocking) API.
//!
//! Run with: cargo run --example generate_sync
//!
//! Note: Requires a running Ollama server with a model installed
//! (e.g., qwen3:0.6b, llama3.2, etc.)

use ollama_oxide::{GenerateRequest, ModelOptions, OllamaApiSync, OllamaClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with default configuration
    let client = OllamaClient::default()?;

    // Model to use (change to your installed model)
    let model = "qwen3:0.6b";

    println!("Generating text with model: {}", model);

    // Example 1: Basic generation
    println!("\n--- Basic Generation ---");
    let request = GenerateRequest::new(model, "What is Rust programming language?");
    let response = client.generate_blocking(&request)?;

    println!("Response: {}", response.text().unwrap_or("No response"));

    // Example 2: With system prompt and options
    println!("\n--- With System Prompt and Options ---");
    let options = ModelOptions::new()
        .with_temperature(0.7)
        .with_num_predict(50);

    let request = GenerateRequest::new(model, "Explain async/await in one sentence.")
        .with_system("You are a programming tutor. Be concise.")
        .with_options(options);

    let response = client.generate_blocking(&request)?;
    println!("Explanation: {}", response.text().unwrap_or("No response"));

    // Example 3: Performance metrics
    println!("\n--- Performance Metrics ---");
    let request = GenerateRequest::new(model, "Hi!");
    let response = client.generate_blocking(&request)?;

    println!("Response: {}", response.text().unwrap_or("No response"));
    if let Some(tps) = response.tokens_per_second() {
        println!("Tokens/sec: {:.2}", tps);
    }
    if let Some(ms) = response.total_duration_ms() {
        println!("Total duration: {:.2} ms", ms);
    }

    println!("\nDone!");

    Ok(())
}
