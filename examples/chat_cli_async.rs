//! Example: Interactive CLI Chat
//!
//! A simple command-line chat interface demonstrating conversational flow
//! with message history: User -> Assistant -> User -> Assistant...
//!
//! Run with: cargo run --example chat_cli_async
//!
//! Commands:
//!   /exit, /quit, /q - Exit the chat
//!   /clear           - Clear conversation history
//!   /history         - Show conversation history
//!   /help            - Show available commands
//!
//! Note: Requires a running Ollama server with a model installed

use ollama_oxide::{ChatMessage, ChatRequest, ModelOptions, OllamaApiAsync, OllamaClient};
use std::io::{self, Write};

const DEFAULT_MODEL: &str = "qwen3:0.6b";
const SYSTEM_PROMPT: &str = "You are a helpful assistant. Be concise and clear in your responses.";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OllamaClient::default()?;

    // Get model from command line args or use default
    let model = std::env::args().nth(1).unwrap_or_else(|| DEFAULT_MODEL.to_string());

    println!("╭─────────────────────────────────────────╮");
    println!("│       ollama-oxide CLI Chat             │");
    println!("╰─────────────────────────────────────────╯");
    println!();
    println!("Model: {}", model);
    println!("Type /help for commands, /exit to quit");
    println!();

    // Initialize conversation with system message
    let mut messages: Vec<ChatMessage> = vec![ChatMessage::system(SYSTEM_PROMPT)];

    // Model options for consistent responses
    let options = ModelOptions::new()
        .with_temperature(0.1)
        .with_top_p(0.9);

    loop {
        // Print prompt and read user input
        print!("You: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        // Skip empty input
        if input.is_empty() {
            continue;
        }

        // Handle commands
        if input.starts_with('/') {
            match input.to_lowercase().as_str() {
                "/exit" | "/quit" | "/q" => {
                    println!("\nGoodbye!");
                    break;
                }
                "/clear" => {
                    messages.clear();
                    messages.push(ChatMessage::system(SYSTEM_PROMPT));
                    println!("Conversation cleared.\n");
                    continue;
                }
                "/history" => {
                    print_history(&messages);
                    continue;
                }
                "/help" => {
                    print_help();
                    continue;
                }
                _ => {
                    println!("Unknown command: {}", input);
                    println!("Type /help for available commands.\n");
                    continue;
                }
            }
        }

        // Add user message to history
        messages.push(ChatMessage::user(input));

        // Build and send request
        let request = ChatRequest::new(&model, messages.clone()).with_options(options.clone());

        print!("Assistant: ");
        io::stdout().flush()?;

        match client.chat(&request).await {
            Ok(response) => {
                let content = response.content().unwrap_or("(no response)");
                println!("{}", content);

                // Add assistant response to history
                messages.push(ChatMessage::assistant(content));

                // Show token stats if available
                if let Some(tps) = response.tokens_per_second() {
                    println!("  [{:.1} tokens/sec]", tps);
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                // Remove the failed user message from history
                messages.pop();
            }
        }

        println!();
    }

    Ok(())
}

fn print_help() {
    println!();
    println!("Available commands:");
    println!("  /exit, /quit, /q  - Exit the chat");
    println!("  /clear            - Clear conversation history");
    println!("  /history          - Show conversation history");
    println!("  /help             - Show this help message");
    println!();
    println!("Usage: cargo run --example chat_cli_async [model_name]");
    println!("  Example: cargo run --example chat_cli_async llama3.2");
    println!();
}

fn print_history(messages: &[ChatMessage]) {
    println!();
    println!("─── Conversation History ───");

    if messages.len() <= 1 {
        println!("(empty)");
    } else {
        for msg in messages.iter().skip(1) {
            // Skip system message
            let prefix = if msg.is_user() {
                "You"
            } else if msg.is_assistant() {
                "Assistant"
            } else {
                "Tool"
            };
            // Truncate long messages for display
            let content = &msg.content;
            let display_content = if content.len() > 100 {
                format!("{}...", &content[..100])
            } else {
                content.clone()
            };
            println!("  {}: {}", prefix, display_content);
        }
    }

    println!("─── {} messages ───", messages.len().saturating_sub(1));
    println!();
}
