//! Example: Function calling with tools (async)
//!
//! This example demonstrates how to use the tools/function calling API
//! to let the model request function executions.
//!
//! Run with: cargo run --example tools_async --features tools
//!
//! Note: Requires a running Ollama server with a model that supports
//! function calling (e.g., qwen3:0.6b, llama3.2, etc.)

use ollama_oxide::{
    ChatMessage, ChatRequest, OllamaApiAsync, OllamaClient, ToolDefinition,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with default configuration
    let client = OllamaClient::default()?;

    // Model to use (change to your installed model)
    // Note: Not all models support function calling
    let model = "qwen3:0.6b";

    println!("Tools/Function Calling examples with model: {}", model);

    // Example 1: Single tool definition
    println!("\n--- Single Tool (Weather) ---");
    let weather_tool = ToolDefinition::function(
        "get_weather",
        json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "The city name, e.g., 'Paris' or 'New York'"
                },
                "unit": {
                    "type": "string",
                    "enum": ["celsius", "fahrenheit"],
                    "description": "Temperature unit"
                }
            },
            "required": ["location"]
        }),
    )
    .with_description("Get the current weather for a location");

    let request = ChatRequest::new(
        model,
        [ChatMessage::user("What's the weather like in Paris?")],
    )
    .with_tools(vec![weather_tool]);

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

    // Example 2: Multiple tools
    println!("\n--- Multiple Tools ---");
    let calculator_tool = ToolDefinition::function(
        "calculate",
        json!({
            "type": "object",
            "properties": {
                "expression": {
                    "type": "string",
                    "description": "Mathematical expression to evaluate"
                }
            },
            "required": ["expression"]
        }),
    )
    .with_description("Evaluate a mathematical expression");

    let search_tool = ToolDefinition::function(
        "web_search",
        json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Search query"
                },
                "max_results": {
                    "type": "integer",
                    "description": "Maximum number of results to return"
                }
            },
            "required": ["query"]
        }),
    )
    .with_description("Search the web for information");

    let request = ChatRequest::new(
        model,
        [ChatMessage::user("What is 25 * 4 + 10?")],
    )
    .with_tools(vec![calculator_tool, search_tool]);

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

    // Example 3: Tool with no parameters
    println!("\n--- Tool Without Parameters ---");
    let time_tool = ToolDefinition::function_no_params("get_current_time")
        .with_description("Get the current date and time");

    let request = ChatRequest::new(
        model,
        [ChatMessage::user("What time is it right now?")],
    )
    .with_tools(vec![time_tool]);

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

    // Example 4: Using with_tool() to add tools incrementally
    println!("\n--- Adding Tools Incrementally ---");
    let request = ChatRequest::new(
        model,
        [ChatMessage::user("Search for Rust programming language")],
    )
    .with_tool(
        ToolDefinition::function(
            "search",
            json!({
                "type": "object",
                "properties": {
                    "query": {"type": "string"}
                },
                "required": ["query"]
            }),
        )
        .with_description("Search for information"),
    )
    .with_tool(
        ToolDefinition::function_no_params("get_popular_topics")
            .with_description("Get trending topics"),
    );

    let response = client.chat(&request).await?;

    if response.has_tool_calls() {
        println!("Model requested tool calls:");
        for call in response.tool_calls().unwrap() {
            println!("  Function: {:?}", call.function_name());
            if let Some(args) = call.arguments() {
                println!("  Arguments: {}", serde_json::to_string_pretty(args)?);
            }
        }
    } else {
        println!("Response: {}", response.content().unwrap_or("No response"));
    }

    // Example 5: Simulating a tool call response flow
    println!("\n--- Tool Call Response Flow ---");
    println!("Step 1: Send initial request with tools");

    let request = ChatRequest::new(
        model,
        [ChatMessage::user("What's 15 + 27?")],
    )
    .with_tool(
        ToolDefinition::function(
            "add",
            json!({
                "type": "object",
                "properties": {
                    "a": {"type": "number", "description": "First number"},
                    "b": {"type": "number", "description": "Second number"}
                },
                "required": ["a", "b"]
            }),
        )
        .with_description("Add two numbers"),
    );

    let response = client.chat(&request).await?;

    if response.has_tool_calls() {
        let calls = response.tool_calls().unwrap();
        println!("Step 2: Model requested function: {:?}", calls[0].function_name());
        println!("        With arguments: {:?}", calls[0].arguments());

        // In a real application, you would:
        // 1. Execute the function with the provided arguments
        // 2. Send a follow-up message with the tool result
        // For example:
        //
        // let result = 42; // Result of add(15, 27)
        // let follow_up = ChatRequest::new(model, [
        //     ChatMessage::user("What's 15 + 27?"),
        //     ChatMessage::assistant("").with_tool_calls(calls.to_vec()),
        //     ChatMessage::tool(&format!("{{\"result\": {}}}", result)),
        // ]).with_tools(vec![add_tool]);
        //
        // let final_response = client.chat(&follow_up).await?;

        println!("Step 3: (In production) Execute function and send result back");
    } else {
        println!("Model answered directly: {}", response.content().unwrap_or("No response"));
    }

    println!("\nDone!");

    Ok(())
}
