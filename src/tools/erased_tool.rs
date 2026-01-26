//! Type-erased tool trait for heterogeneous tool collections
//!
//! This module provides `ErasedTool` for storing different tool implementations
//! in the same registry.

use std::future::Future;
use std::pin::Pin;

use crate::ToolDefinition;

use super::tool_error::{ToolError, ToolResult};

/// Type-erased version of `Tool` for storing in collections
///
/// This trait allows different tool implementations to be stored
/// in the same `ToolRegistry` by erasing their specific parameter
/// and output types.
#[allow(dead_code)]
pub trait ErasedTool: Send + Sync {
    /// Returns the name of the tool
    fn name(&self) -> &'static str;

    /// Returns the tool definition
    fn definition(&self) -> ToolDefinition;

    /// Execute the tool with JSON arguments, returning JSON result
    fn execute_erased<'a>(
        &'a self,
        args: serde_json::Value,
    ) -> Pin<Box<dyn Future<Output = ToolResult<serde_json::Value>> + Send + 'a>>;

    /// Execute the tool synchronously (blocking)
    fn execute_erased_blocking(&self, args: serde_json::Value) -> ToolResult<serde_json::Value>;
}

/// Wrapper that implements `ErasedTool` for any `Tool`
pub(crate) struct ToolWrapper<T> {
    tool: T,
}

impl<T> ToolWrapper<T> {
    pub fn new(tool: T) -> Self {
        Self { tool }
    }
}

impl<T> ErasedTool for ToolWrapper<T>
where
    T: super::tool_trait::Tool + 'static,
{
    fn name(&self) -> &'static str {
        self.tool.name()
    }

    fn definition(&self) -> ToolDefinition {
        self.tool.to_definition()
    }

    fn execute_erased<'a>(
        &'a self,
        args: serde_json::Value,
    ) -> Pin<Box<dyn Future<Output = ToolResult<serde_json::Value>> + Send + 'a>> {
        Box::pin(async move {
            // Deserialize arguments
            let params: T::Params = serde_json::from_value(args).map_err(|e| {
                ToolError::DeserializationError(format!(
                    "failed to deserialize arguments for '{}': {}",
                    self.tool.name(),
                    e
                ))
            })?;

            // Execute the tool
            let output = self.tool.execute(params).await?;

            // Serialize the output
            serde_json::to_value(output).map_err(|e| {
                ToolError::SerializationError(format!(
                    "failed to serialize output for '{}': {}",
                    self.tool.name(),
                    e
                ))
            })
        })
    }

    fn execute_erased_blocking(&self, args: serde_json::Value) -> ToolResult<serde_json::Value> {
        // Use tokio's block_on for sync execution
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(self.execute_erased(args))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::tool_trait::Tool;
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, JsonSchema)]
    struct TestParams {
        x: i32,
    }

    #[derive(Serialize)]
    struct TestOutput {
        doubled: i32,
    }

    struct DoubleTool;

    impl Tool for DoubleTool {
        type Params = TestParams;
        type Output = TestOutput;

        fn name(&self) -> &'static str {
            "double"
        }

        fn description(&self) -> &'static str {
            "Doubles a number"
        }

        async fn execute(&self, params: Self::Params) -> ToolResult<Self::Output> {
            Ok(TestOutput {
                doubled: params.x * 2,
            })
        }
    }

    #[test]
    fn test_erased_tool_name() {
        let wrapper = ToolWrapper::new(DoubleTool);
        let erased: &dyn ErasedTool = &wrapper;
        assert_eq!(erased.name(), "double");
    }

    #[test]
    fn test_erased_tool_definition() {
        let wrapper = ToolWrapper::new(DoubleTool);
        let erased: &dyn ErasedTool = &wrapper;
        let def = erased.definition();
        assert_eq!(def.name(), "double");
        assert_eq!(def.description(), Some("Doubles a number"));
    }

    #[tokio::test]
    async fn test_erased_tool_execute() {
        let wrapper = ToolWrapper::new(DoubleTool);
        let erased: &dyn ErasedTool = &wrapper;

        let args = serde_json::json!({"x": 5});
        let result = erased.execute_erased(args).await.unwrap();

        assert_eq!(result["doubled"], 10);
    }

    #[tokio::test]
    async fn test_erased_tool_execute_invalid_args() {
        let wrapper = ToolWrapper::new(DoubleTool);
        let erased: &dyn ErasedTool = &wrapper;

        let args = serde_json::json!({"wrong": "field"});
        let result = erased.execute_erased(args).await;

        assert!(matches!(result, Err(ToolError::DeserializationError(_))));
    }
}
