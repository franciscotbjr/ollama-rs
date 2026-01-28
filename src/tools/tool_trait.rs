//! Tool trait for type-safe function calling
//!
//! This module provides the `Tool` trait for defining type-safe tools
//! with auto-generated JSON schemas.

use std::fmt::Debug;
use std::future::Future;

use schemars::JsonSchema;
use schemars::schema_for;
use serde::{Deserialize, Serialize};

use crate::ToolDefinition;

use super::tool_error::ToolResult;

/// Trait for defining type-safe tools with auto-generated JSON schemas
///
/// Implement this trait to create tools that can be used with
/// `ToolRegistry` for automatic dispatch and schema generation.
///
/// # Type Parameters
///
/// * `Params` - The input parameters type (must implement `Deserialize` and `JsonSchema`)
/// * `Output` - The output type (must implement `Serialize`)
///
/// # Examples
///
/// ```no_run
/// use ollama_oxide::tools::{Tool, ToolResult};
/// use schemars::JsonSchema;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Deserialize, JsonSchema)]
/// struct WeatherParams {
///     location: String,
///     #[serde(default)]
///     unit: Option<String>,
/// }
///
/// #[derive(Serialize)]
/// struct WeatherResult {
///     temperature: f32,
///     description: String,
/// }
///
/// struct GetWeather;
///
/// impl Tool for GetWeather {
///     type Params = WeatherParams;
///     type Output = WeatherResult;
///
///     fn name(&self) -> &'static str {
///         "get_weather"
///     }
///
///     fn description(&self) -> &'static str {
///         "Get the current weather for a location"
///     }
///
///     async fn execute(&self, params: Self::Params) -> ToolResult<Self::Output> {
///         Ok(WeatherResult {
///             temperature: 22.0,
///             description: format!("Sunny in {}", params.location),
///         })
///     }
/// }
/// ```
pub trait Tool: Send + Sync {
    /// The input parameters type
    type Params: for<'de> Deserialize<'de> + JsonSchema + Send + Debug;

    /// The output type
    type Output: Serialize + Send;

    /// Returns the name of the tool (used as function name)
    fn name(&self) -> &'static str;

    /// Returns a description of what the tool does
    fn description(&self) -> &'static str;

    /// Execute the tool with the given parameters
    ///
    /// This is an async function that performs the tool's action.
    fn execute(
        &self,
        params: Self::Params,
    ) -> impl Future<Output = ToolResult<Self::Output>> + Send;

    /// Generate the JSON schema for the tool's parameters
    ///
    /// This is auto-implemented using `schemars`.
    fn parameters_schema(&self) -> serde_json::Value {
        let schema = schema_for!(Self::Params);
        serde_json::to_value(schema).unwrap_or_else(|_| serde_json::json!({}))
    }

    /// Convert this tool to a `ToolDefinition` for use in chat requests
    ///
    /// This is auto-implemented using `name()`, `description()`, and `parameters_schema()`.
    fn to_definition(&self) -> ToolDefinition {
        ToolDefinition::function(self.name(), self.parameters_schema())
            .with_description(self.description())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, JsonSchema)]
    struct TestParams {
        value: String,
    }

    #[derive(Serialize)]
    struct TestOutput {
        result: String,
    }

    struct TestTool;

    impl Tool for TestTool {
        type Params = TestParams;
        type Output = TestOutput;

        fn name(&self) -> &'static str {
            "test_tool"
        }

        fn description(&self) -> &'static str {
            "A test tool"
        }

        async fn execute(&self, params: Self::Params) -> ToolResult<Self::Output> {
            Ok(TestOutput {
                result: format!("processed: {}", params.value),
            })
        }
    }

    #[test]
    fn test_tool_name() {
        let tool = TestTool;
        assert_eq!(tool.name(), "test_tool");
    }

    #[test]
    fn test_tool_description() {
        let tool = TestTool;
        assert_eq!(tool.description(), "A test tool");
    }

    #[test]
    fn test_tool_parameters_schema() {
        let tool = TestTool;
        let schema = tool.parameters_schema();
        assert!(schema.is_object());
    }

    #[test]
    fn test_tool_to_definition() {
        let tool = TestTool;
        let def = tool.to_definition();
        assert_eq!(def.name(), "test_tool");
        assert_eq!(def.description(), Some("A test tool"));
    }

    #[tokio::test]
    async fn test_tool_execute() {
        let tool = TestTool;
        let params = TestParams {
            value: "hello".to_string(),
        };
        let result = tool.execute(params).await.unwrap();
        assert_eq!(result.result, "processed: hello");
    }
}
