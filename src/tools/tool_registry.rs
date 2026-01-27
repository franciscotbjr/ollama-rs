//! Tool registry for automatic dispatch
//!
//! This module provides `ToolRegistry` for registering tools and
//! automatically dispatching tool calls from chat responses.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::{ChatResponse, ToolCall, ToolDefinition};

use super::erased_tool::{ErasedTool, ToolWrapper};
use super::tool_error::{ToolError, ToolResult};
use super::tool_trait::Tool;

/// Registry for managing and dispatching tools
///
/// `ToolRegistry` provides:
/// - Tool registration with automatic schema generation
/// - Generation of `ToolDefinition` list for chat requests
/// - Automatic dispatch of tool calls from responses
///
/// # Thread Safety
///
/// The registry uses `Arc<RwLock<...>>` internally and is safe to clone
/// and share across threads.
///
/// # Examples
///
/// ```no_run
/// use ollama_oxide::tools::{Tool, ToolRegistry, ToolResult};
/// use schemars::JsonSchema;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Deserialize, JsonSchema)]
/// struct AddParams { a: i32, b: i32 }
///
/// #[derive(Serialize)]
/// struct AddResult { sum: i32 }
///
/// struct AddTool;
///
/// impl Tool for AddTool {
///     type Params = AddParams;
///     type Output = AddResult;
///     fn name(&self) -> &'static str { "add" }
///     fn description(&self) -> &'static str { "Add two numbers" }
///     async fn execute(&self, p: Self::Params) -> ToolResult<Self::Output> {
///         Ok(AddResult { sum: p.a + p.b })
///     }
/// }
///
/// let mut registry = ToolRegistry::new();
/// registry.register(AddTool);
///
/// // Get definitions for chat request
/// let definitions = registry.definitions();
///
/// // After receiving a response with tool calls
/// // let results = registry.execute_all(&response).await;
/// ```
#[derive(Clone)]
pub struct ToolRegistry {
    tools: Arc<RwLock<HashMap<String, Arc<dyn ErasedTool>>>>,
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolRegistry {
    /// Create a new empty tool registry
    pub fn new() -> Self {
        Self {
            tools: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a tool with the registry
    ///
    /// The tool's name is used as the key for dispatch.
    pub fn register<T: Tool + 'static>(&mut self, tool: T) {
        let name = tool.name().to_string();
        let wrapper = Arc::new(ToolWrapper::new(tool));
        self.tools.write().unwrap().insert(name, wrapper);
    }

    /// Get the number of registered tools
    pub fn len(&self) -> usize {
        self.tools.read().unwrap().len()
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.tools.read().unwrap().is_empty()
    }

    /// Check if a tool with the given name is registered
    pub fn contains(&self, name: &str) -> bool {
        self.tools.read().unwrap().contains_key(name)
    }

    /// Get tool definitions for all registered tools
    ///
    /// Use this when building a `ChatRequest`:
    /// ```ignore
    /// let request = ChatRequest::new(model, messages)
    ///     .with_tools(registry.definitions());
    /// ```
    pub fn definitions(&self) -> Vec<ToolDefinition> {
        self.tools
            .read()
            .unwrap()
            .values()
            .map(|t| t.definition())
            .collect()
    }

    /// Execute a single tool call asynchronously
    ///
    /// # Arguments
    ///
    /// * `call` - The tool call from a chat response
    ///
    /// # Returns
    ///
    /// Returns the tool's output as JSON, or an error if:
    /// - The tool is not found
    /// - The arguments are invalid
    /// - The tool execution fails
    pub async fn execute(&self, call: &ToolCall) -> ToolResult<serde_json::Value> {
        let func_name = call.function_name().ok_or(ToolError::InvalidToolCall)?;

        let args = call
            .arguments()
            .cloned()
            .unwrap_or_else(|| serde_json::json!({}));

        let tool = {
            let tools = self.tools.read().unwrap();
            tools
                .get(func_name)
                .cloned()
                .ok_or_else(|| ToolError::NotFound(func_name.to_string()))?
        };

        tool.execute_erased(args).await
    }

    /// Execute all tool calls from a chat response asynchronously
    ///
    /// # Arguments
    ///
    /// * `response` - The chat response that may contain tool calls
    ///
    /// # Returns
    ///
    /// Returns a vector of results, one for each tool call.
    /// If the response has no tool calls, returns an empty vector.
    pub async fn execute_all(&self, response: &ChatResponse) -> Vec<ToolResult<serde_json::Value>> {
        let Some(calls) = response.tool_calls() else {
            return Vec::new();
        };

        let mut results = Vec::with_capacity(calls.len());
        for call in calls {
            results.push(self.execute(call).await);
        }
        results
    }

    /// Execute a single tool call synchronously (blocking)
    ///
    /// This is a convenience method for sync code.
    pub fn execute_blocking(&self, call: &ToolCall) -> ToolResult<serde_json::Value> {
        let func_name = call.function_name().ok_or(ToolError::InvalidToolCall)?;

        let args = call
            .arguments()
            .cloned()
            .unwrap_or_else(|| serde_json::json!({}));

        let tool = {
            let tools = self.tools.read().unwrap();
            tools
                .get(func_name)
                .cloned()
                .ok_or_else(|| ToolError::NotFound(func_name.to_string()))?
        };

        tool.execute_erased_blocking(args)
    }

    /// Execute all tool calls from a chat response synchronously (blocking)
    ///
    /// This is a convenience method for sync code.
    pub fn execute_all_blocking(
        &self,
        response: &ChatResponse,
    ) -> Vec<ToolResult<serde_json::Value>> {
        let Some(calls) = response.tool_calls() else {
            return Vec::new();
        };

        calls
            .iter()
            .map(|call| self.execute_blocking(call))
            .collect()
    }
}

impl std::fmt::Debug for ToolRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tools = self.tools.read().unwrap();
        let names: Vec<_> = tools.keys().collect();
        f.debug_struct("ToolRegistry")
            .field("tools", &names)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::tool_trait::Tool;
    use crate::{ResponseMessage, ToolCallFunction};
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, JsonSchema)]
    struct MultiplyParams {
        a: i32,
        b: i32,
    }

    #[derive(Serialize)]
    struct MultiplyResult {
        product: i32,
    }

    struct MultiplyTool;

    impl Tool for MultiplyTool {
        type Params = MultiplyParams;
        type Output = MultiplyResult;

        fn name(&self) -> &'static str {
            "multiply"
        }

        fn description(&self) -> &'static str {
            "Multiply two numbers"
        }

        async fn execute(&self, params: Self::Params) -> ToolResult<Self::Output> {
            Ok(MultiplyResult {
                product: params.a * params.b,
            })
        }
    }

    #[derive(Debug, Deserialize, JsonSchema)]
    struct GreetParams {
        name: String,
    }

    #[derive(Serialize)]
    struct GreetResult {
        greeting: String,
    }

    struct GreetTool;

    impl Tool for GreetTool {
        type Params = GreetParams;
        type Output = GreetResult;

        fn name(&self) -> &'static str {
            "greet"
        }

        fn description(&self) -> &'static str {
            "Greet someone"
        }

        async fn execute(&self, params: Self::Params) -> ToolResult<Self::Output> {
            Ok(GreetResult {
                greeting: format!("Hello, {}!", params.name),
            })
        }
    }

    #[test]
    fn test_registry_new() {
        let registry = ToolRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn test_registry_register() {
        let mut registry = ToolRegistry::new();
        registry.register(MultiplyTool);
        assert_eq!(registry.len(), 1);
        assert!(registry.contains("multiply"));
    }

    #[test]
    fn test_registry_register_multiple() {
        let mut registry = ToolRegistry::new();
        registry.register(MultiplyTool);
        registry.register(GreetTool);
        assert_eq!(registry.len(), 2);
        assert!(registry.contains("multiply"));
        assert!(registry.contains("greet"));
    }

    #[test]
    fn test_registry_definitions() {
        let mut registry = ToolRegistry::new();
        registry.register(MultiplyTool);
        registry.register(GreetTool);

        let defs = registry.definitions();
        assert_eq!(defs.len(), 2);

        let names: Vec<_> = defs.iter().map(|d| d.name()).collect();
        assert!(names.contains(&"multiply"));
        assert!(names.contains(&"greet"));
    }

    #[tokio::test]
    async fn test_registry_execute() {
        let mut registry = ToolRegistry::new();
        registry.register(MultiplyTool);

        let call = ToolCall::new(ToolCallFunction::with_arguments(
            "multiply",
            serde_json::json!({"a": 3, "b": 4}),
        ));

        let result = registry.execute(&call).await.unwrap();
        assert_eq!(result["product"], 12);
    }

    #[tokio::test]
    async fn test_registry_execute_not_found() {
        let registry = ToolRegistry::new();

        let call = ToolCall::new(ToolCallFunction::new("unknown"));
        let result = registry.execute(&call).await;

        assert!(matches!(result, Err(ToolError::NotFound(_))));
    }

    #[tokio::test]
    async fn test_registry_execute_all() {
        let mut registry = ToolRegistry::new();
        registry.register(MultiplyTool);
        registry.register(GreetTool);

        let response = ChatResponse {
            message: Some(ResponseMessage {
                tool_calls: Some(vec![
                    ToolCall::new(ToolCallFunction::with_arguments(
                        "multiply",
                        serde_json::json!({"a": 2, "b": 5}),
                    )),
                    ToolCall::new(ToolCallFunction::with_arguments(
                        "greet",
                        serde_json::json!({"name": "Alice"}),
                    )),
                ]),
                ..Default::default()
            }),
            ..Default::default()
        };

        let results = registry.execute_all(&response).await;
        assert_eq!(results.len(), 2);

        let multiply_result = results[0].as_ref().unwrap();
        assert_eq!(multiply_result["product"], 10);

        let greet_result = results[1].as_ref().unwrap();
        assert_eq!(greet_result["greeting"], "Hello, Alice!");
    }

    #[tokio::test]
    async fn test_registry_execute_all_no_tool_calls() {
        let registry = ToolRegistry::new();

        let response = ChatResponse {
            message: Some(ResponseMessage {
                content: Some("Hello!".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };

        let results = registry.execute_all(&response).await;
        assert!(results.is_empty());
    }

    #[test]
    fn test_registry_clone() {
        let mut registry = ToolRegistry::new();
        registry.register(MultiplyTool);

        let cloned = registry.clone();
        assert_eq!(cloned.len(), 1);
        assert!(cloned.contains("multiply"));
    }

    #[test]
    fn test_registry_debug() {
        let mut registry = ToolRegistry::new();
        registry.register(MultiplyTool);

        let debug = format!("{:?}", registry);
        assert!(debug.contains("ToolRegistry"));
        assert!(debug.contains("multiply"));
    }
}
