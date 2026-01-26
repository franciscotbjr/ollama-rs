//! Example: Chat completion (sync/blocking)
//!
//! This example demonstrates how to use the chat API
//! for conversational interactions using the blocking API.
//!
//! Run with: cargo run --example chat_sync
//!
//! Note: Requires a running Ollama server with a model installed
//! (e.g., qwen3:0.6b, llama3.2, etc.)

use ollama_oxide::{ChatMessage, ChatRequest, OllamaApiSync, OllamaClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with default configuration
    let client = OllamaClient::default()?;

    // Model to use (change to your installed model)
    let model = "qwen3:0.6b";

    println!("Chat examples (sync) with model: {}", model);

    // Example 1: Basic chat
    println!("\n--- Basic Chat ---");
    let request = ChatRequest::new(model, [ChatMessage::user("Hello! What can you help me with?")]);

    let response = client.chat_blocking(&request)?;
    println!("Assistant: {}", response.content().unwrap_or("No response"));

    // Example 2: With system message
    println!("\n--- With System Message ---");
    let request = ChatRequest::new(
        model,
        [
            ChatMessage::system("You are a helpful coding assistant. Be concise."),
            ChatMessage::user("What is Rust?"),
        ],
    );

    let response = client.chat_blocking(&request)?;
    println!("Assistant: {}", response.content().unwrap_or("No response"));

    // Example 3: Multi-turn conversation
    println!("\n--- Multi-turn Conversation ---");
    let request = ChatRequest::new(
        model,
        [
            ChatMessage::system("You are a helpful assistant."),
            ChatMessage::user("What is Rust?"),
            ChatMessage::assistant("Rust is a systems programming language focused on safety, speed, and concurrency."),
            ChatMessage::user("What are its main features?"),
        ],
    );

    let response = client.chat_blocking(&request)?;
    println!("Assistant: {}", response.content().unwrap_or("No response"));

    // Example 4: Performance metrics
    println!("\n--- Performance Metrics ---");
    let request = ChatRequest::new(model, [ChatMessage::user("Hello!")]);
    let response = client.chat_blocking(&request)?;

    println!("Response: {}", response.content().unwrap_or("No response"));
    println!("Model: {:?}", response.model());
    println!("Prompt tokens: {:?}", response.prompt_tokens());
    println!("Completion tokens: {:?}", response.completion_tokens());
    if let Some(tps) = response.tokens_per_second() {
        println!("Tokens/sec: {:.2}", tps);
    }

    println!("\nDone!");

    Ok(())
}
