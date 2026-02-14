//! Example: Type-erased tool registry with automatic dispatch
//!
//! Demonstrates how to use `ToolRegistry` to store heterogeneous tools
//! and automatically dispatch tool calls from chat responses.
//!
//! Run with: cargo run --example tool_registry_async --features tools

use ollama_oxide::tools::{Tool, ToolRegistry, ToolResult};
use ollama_oxide::{ChatMessage, ChatRequest, OllamaApiAsync, OllamaClient};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// --- Tool 1: Calculator ---

#[derive(Debug, Deserialize, JsonSchema)]
struct CalculatorParams {
    a: f64,
    b: f64,
    operation: String,
}

#[derive(Serialize)]
struct CalculatorOutput {
    result: f64,
}

struct CalculatorTool;

impl Tool for CalculatorTool {
    type Params = CalculatorParams;
    type Output = CalculatorOutput;

    fn name(&self) -> &'static str {
        "calculator"
    }

    fn description(&self) -> &'static str {
        "Perform basic math operations (add, subtract, multiply, divide)"
    }

    async fn execute(&self, params: Self::Params) -> ToolResult<Self::Output> {
        let result = match params.operation.as_str() {
            "add" => params.a + params.b,
            "subtract" => params.a - params.b,
            "multiply" => params.a * params.b,
            "divide" => params.a / params.b,
            _ => {
                return Err(ollama_oxide::tools::ToolError::ExecutionError(format!(
                    "Unknown operation: {}",
                    params.operation
                )));
            }
        };
        Ok(CalculatorOutput { result })
    }
}

// --- Tool 2: String Formatter ---

#[derive(Debug, Deserialize, JsonSchema)]
struct FormatParams {
    text: String,
    style: String,
}

#[derive(Serialize)]
struct FormatOutput {
    formatted: String,
}

struct FormatTool;

impl Tool for FormatTool {
    type Params = FormatParams;
    type Output = FormatOutput;

    fn name(&self) -> &'static str {
        "format_text"
    }

    fn description(&self) -> &'static str {
        "Format text (uppercase, lowercase, reverse)"
    }

    async fn execute(&self, params: Self::Params) -> ToolResult<Self::Output> {
        let formatted = match params.style.as_str() {
            "uppercase" => params.text.to_uppercase(),
            "lowercase" => params.text.to_lowercase(),
            "reverse" => params.text.chars().rev().collect(),
            _ => params.text,
        };
        Ok(FormatOutput { formatted })
    }
}

// --- Tool 3: Random Number Generator ---

#[derive(Debug, Deserialize, JsonSchema)]
struct RandomParams {
    min: i32,
    max: i32,
}

#[derive(Serialize)]
struct RandomOutput {
    value: i32,
}

struct RandomTool;

impl Tool for RandomTool {
    type Params = RandomParams;
    type Output = RandomOutput;

    fn name(&self) -> &'static str {
        "random_number"
    }

    fn description(&self) -> &'static str {
        "Generate a random number between min and max"
    }

    async fn execute(&self, params: Self::Params) -> ToolResult<Self::Output> {
        // Simple pseudo-random for demo (not cryptographically secure)
        let value = params.min
            + (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as i32
                % (params.max - params.min + 1));
        Ok(RandomOutput { value })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OllamaClient::default()?;
    let model = "qwen3:0.6b";

    // Create registry and register all tools
    let mut registry = ToolRegistry::new();
    registry.register(CalculatorTool);
    registry.register(FormatTool);
    registry.register(RandomTool);

    println!("Registered {} tools: {:?}", registry.len(), registry);

    // Build request with all tool definitions from registry
    let request = ChatRequest::new(model, [ChatMessage::user("What is 15 multiplied by 7?")])
        .with_tools(registry.definitions());

    println!("Request: {:?}", &request);

    println!("\nSending request to model...");
    let response = client.chat(&request).await?;

    println!("Model response:");
    println!("Response: {:?}", &response);

    if response.has_tool_calls() {
        println!("Model requested tool calls:");

        // Execute all tool calls automatically via registry
        let results = registry.execute_all(&response).await;

        for (call, result) in response.tool_calls().unwrap().iter().zip(results.iter()) {
            println!("  Tool: {:?}", call.function_name());
            println!("  Args: {:?}", call.arguments());
            match result {
                Ok(output) => println!("  Result: {}", output),
                Err(e) => println!("  Error: {}", e),
            }
        }
    } else {
        println!("Response: {}", response.content().unwrap_or("No response"));
    }

    Ok(())
}
