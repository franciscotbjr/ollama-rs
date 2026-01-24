//! Example: Generate text (async)
//!
//! This example demonstrates how to generate text completions
//! using the async API.
//!
//! Run with: cargo run --example generate_async
//!
//! Note: Requires a running Ollama server with a model installed
//! (e.g., qwen3:0.6b, llama3.2, etc.)

use ollama_oxide::{FormatSetting, GenerateRequest, ModelOptions, OllamaApiAsync, OllamaClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with default configuration
    let client = OllamaClient::default()?;

    // Model to use (change to your installed model)
    let model = "qwen3:0.6b";

    println!("Generating text with model: {}", model);

    // Example 1: Basic generation
    println!("\n--- Basic Generation ---");
    let request = GenerateRequest::new(model, "Why is the sky blue?");
    let response = client.generate(&request).await?;

    println!("Response: {}", response.text().unwrap_or("No response"));
    println!("Done: {}", response.is_done());
    println!("Done reason: {:?}", response.done_reason);

    // Example 2: With system prompt
    println!("\n--- With System Prompt ---");
    let request = GenerateRequest::new(model, "Tell me a short joke.")
        .with_system("You are a comedian. Keep your responses brief and funny.");

    let response = client.generate(&request).await?;
    println!("Joke: {}", response.text().unwrap_or("No joke"));

    // Example 3: With model options
    println!("\n--- With Model Options ---");
    let options = ModelOptions::new()
        .with_temperature(0.9)
        .with_top_p(0.95)
        .with_num_predict(100);

    let request =
        GenerateRequest::new(model, "Write a creative one-line story.").with_options(options);

    let response = client.generate(&request).await?;
    println!("Story: {}", response.text().unwrap_or("No story"));

    // Example 4: JSON output format
    println!("\n--- JSON Output Format ---");
    let request = GenerateRequest::new(
        model,
        "List 3 programming languages with their primary use case. Output as JSON.",
    )
    .with_format(FormatSetting::json());

    let response = client.generate(&request).await?;
    println!("JSON: {}", response.text().unwrap_or("No JSON"));

    // Example 5: Performance metrics
    println!("\n--- Performance Metrics ---");
    let request = GenerateRequest::new(model, "Hello!");
    let response = client.generate(&request).await?;

    println!("Response: {}", response.text().unwrap_or("No response"));
    println!("Model: {:?}", response.model);
    println!("Tokens generated: {:?}", response.eval_count);
    println!("Prompt tokens: {:?}", response.prompt_eval_count);
    if let Some(tps) = response.tokens_per_second() {
        println!("Tokens/sec: {:.2}", tps);
    }
    if let Some(ms) = response.total_duration_ms() {
        println!("Total duration: {:.2} ms", ms);
    }

    println!("\nDone!");

    Ok(())
}
