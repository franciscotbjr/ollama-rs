//! Example: Chat completion (async)
//!
//! This example demonstrates how to use the chat API
//! for conversational interactions.
//!
//! Run with: cargo run --example chat_async
//!
//! Note: Requires a running Ollama server with a model installed
//! (e.g., qwen3:0.6b, llama3.2, etc.)

use ollama_oxide::{
    ChatMessage, ChatRequest, FormatSetting, ModelOptions, OllamaApiAsync, OllamaClient,
};
#[cfg(feature = "tools")]
use ollama_oxide::ToolDefinition;
#[cfg(feature = "tools")]
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with default configuration
    let client = OllamaClient::default()?;

    // Model to use (change to your installed model)
    let model = "qwen3:0.6b";

    println!("Chat examples with model: {}", model);

    // Example 1: Basic chat
    println!("\n--- Basic Chat ---");
    let request = ChatRequest::new(model, [ChatMessage::user("Hello! What can you help me with?")]);

    let response = client.chat(&request).await?;
    println!("Assistant: {}", response.content().unwrap_or("No response"));
    println!("Done: {}", response.is_done());

    // Example 2: With system message
    println!("\n--- With System Message ---");
    let request = ChatRequest::new(
        model,
        [
            ChatMessage::system("You are a helpful coding assistant. Be concise."),
            ChatMessage::user("What is Rust?"),
        ],
    );

    let response = client.chat(&request).await?;
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

    let response = client.chat(&request).await?;
    println!("Assistant: {}", response.content().unwrap_or("No response"));

    // Example 4: With model options
    println!("\n--- With Model Options ---");
    let options = ModelOptions::new()
        .with_temperature(0.7)
        .with_top_p(0.9)
        .with_num_predict(150);

    let request = ChatRequest::new(
        model,
        [ChatMessage::user("Tell me a creative short story in 2 sentences.")],
    )
    .with_options(options);

    let response = client.chat(&request).await?;
    println!("Story: {}", response.content().unwrap_or("No story"));

    // Example 5: JSON output format
    println!("\n--- JSON Output Format ---");
    let request = ChatRequest::new(
        model,
        [ChatMessage::user(
            "List 3 colors with their hex codes. Output as JSON array.",
        )],
    )
    .with_format(FormatSetting::json());

    let response = client.chat(&request).await?;
    println!("JSON: {}", response.content().unwrap_or("No JSON"));

    // Example 6: With tools (function calling)
    // Note: Requires the "tools" feature to be enabled
    #[cfg(feature = "tools")]
    {
        println!("\n--- With Tools (Function Calling) ---");
        let request = ChatRequest::new(
            model,
            [ChatMessage::user("What's the weather like in Paris?")],
        )
        .with_tools(vec![ToolDefinition::function(
            "get_weather",
            json!({
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city name"
                    },
                    "unit": {
                        "type": "string",
                        "enum": ["celsius", "fahrenheit"]
                    }
                },
                "required": ["location"]
            }),
        )
        .with_description("Get the current weather for a location")]);

        let response = client.chat(&request).await?;

        if response.has_tool_calls() {
            println!("Model requested tool calls:");
            for call in response.tool_calls().unwrap() {
                println!("  Function: {:?}", call.function_name());
                println!("  Arguments: {:?}", call.arguments());
            }
        } else {
            println!("Response: {}", response.content().unwrap_or("No response"));
        }
    }
    #[cfg(not(feature = "tools"))]
    {
        println!("\n--- With Tools (Function Calling) ---");
        println!("(Skipped: requires 'tools' feature)");
    }

    // Example 7: Performance metrics
    println!("\n--- Performance Metrics ---");
    let request = ChatRequest::new(model, [ChatMessage::user("Hello!")]);
    let response = client.chat(&request).await?;

    println!("Response: {}", response.content().unwrap_or("No response"));
    println!("Model: {:?}", response.model());
    println!("Prompt tokens: {:?}", response.prompt_tokens());
    println!("Completion tokens: {:?}", response.completion_tokens());
    println!("Total tokens: {:?}", response.total_tokens());
    if let Some(tps) = response.tokens_per_second() {
        println!("Tokens/sec: {:.2}", tps);
    }
    if let Some(ms) = response.total_duration_ms() {
        println!("Total duration: {:.2} ms", ms);
    }

    println!("\nDone!");

    Ok(())
}
