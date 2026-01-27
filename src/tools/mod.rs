//! Ergonomic Tools API for type-safe function calling
//!
//! This module provides a high-level API for defining and dispatching tools
//! (function calling) with automatic JSON schema generation and type-safe
//! parameter handling.
//!
//! # Feature Flag
//!
//! This module requires the `tools` feature:
//!
//! ```toml
//! [dependencies]
//! ollama-oxide = { version = "0.1", features = ["tools"] }
//! ```
//!
//! # Overview
//!
//! The tools API provides three tiers of abstraction:
//!
//! 1. **Low-Level** (always available): `ToolDefinition`, `ToolCall` - manual JSON schemas
//! 2. **Type-Safe** (this module): `Tool` trait - auto-generated schemas from Rust types
//! 3. **Registry** (this module): `ToolRegistry` - automatic dispatch
//!
//! # Quick Start
//!
//! ```no_run
//! use ollama_oxide::tools::{Tool, ToolRegistry, ToolResult};
//! use schemars::JsonSchema;
//! use serde::{Deserialize, Serialize};
//!
//! // 1. Define parameter struct (JSON schema auto-generated!)
//! #[derive(Debug, Deserialize, JsonSchema)]
//! struct WeatherParams {
//!     location: String,
//!     #[serde(default)]
//!     unit: Option<String>,
//! }
//!
//! // 2. Define output struct
//! #[derive(Serialize)]
//! struct WeatherResult {
//!     temperature: f32,
//!     description: String,
//! }
//!
//! // 3. Implement Tool trait
//! struct GetWeather;
//!
//! impl Tool for GetWeather {
//!     type Params = WeatherParams;
//!     type Output = WeatherResult;
//!
//!     fn name(&self) -> &'static str { "get_weather" }
//!     fn description(&self) -> &'static str { "Get current weather" }
//!
//!     async fn execute(&self, params: Self::Params) -> ToolResult<Self::Output> {
//!         // Your implementation here
//!         Ok(WeatherResult {
//!             temperature: 22.0,
//!             description: format!("Sunny in {}", params.location),
//!         })
//!     }
//! }
//!
//! // 4. Register tools and use in chat
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut registry = ToolRegistry::new();
//!     registry.register(GetWeather);
//!
//!     // Use registry.definitions() in ChatRequest
//!     // Use registry.execute_all(&response) to handle tool calls
//!     Ok(())
//! }
//! ```
//!
//! # Comparison with Low-Level API
//!
//! **Low-Level (manual JSON schema):**
//! ```ignore
//! let tool = ToolDefinition::function("get_weather", json!({
//!     "type": "object",
//!     "properties": { "location": { "type": "string" } },
//!     "required": ["location"]
//! }));
//!
//! // Manual dispatch required
//! match call.function_name() {
//!     Some("get_weather") => { /* manual JSON parsing */ }
//!     _ => {}
//! }
//! ```
//!
//! **Ergonomic (auto-generated schema):**
//! ```ignore
//! // Schema derived from WeatherParams automatically!
//! let definitions = registry.definitions();
//!
//! // Automatic dispatch with type-safe parsing
//! let results = registry.execute_all(&response).await;
//! ```

mod erased_tool;
mod tool_error;
mod tool_registry;
mod tool_trait;

pub use tool_error::{ToolError, ToolResult};
pub use tool_registry::ToolRegistry;
pub use tool_trait::Tool;
