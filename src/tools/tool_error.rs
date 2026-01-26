//! Tool error types for the ergonomic tools API
//!
//! This module provides error handling for tool execution.

use thiserror::Error;

/// Result type for tool operations
pub type ToolResult<T> = std::result::Result<T, ToolError>;

/// Errors that can occur during tool execution
#[derive(Debug, Error)]
pub enum ToolError {
    /// Tool with the given name was not found in the registry
    #[error("tool not found: {0}")]
    NotFound(String),

    /// Failed to deserialize tool arguments
    #[error("failed to deserialize arguments: {0}")]
    DeserializationError(String),

    /// Failed to serialize tool output
    #[error("failed to serialize output: {0}")]
    SerializationError(String),

    /// Tool execution failed
    #[error("execution error: {0}")]
    ExecutionError(String),

    /// Tool call is missing required function information
    #[error("invalid tool call: missing function")]
    InvalidToolCall,

    /// Custom error from tool implementation
    #[error("{0}")]
    Custom(String),
}

impl ToolError {
    /// Create a custom error with the given message
    pub fn custom(message: impl Into<String>) -> Self {
        Self::Custom(message.into())
    }

    /// Create a deserialization error from a serde_json error
    pub fn from_json_error(err: serde_json::Error) -> Self {
        Self::DeserializationError(err.to_string())
    }
}

impl From<serde_json::Error> for ToolError {
    fn from(err: serde_json::Error) -> Self {
        Self::DeserializationError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_error_not_found() {
        let err = ToolError::NotFound("get_weather".to_string());
        assert_eq!(err.to_string(), "tool not found: get_weather");
    }

    #[test]
    fn test_tool_error_deserialization() {
        let err = ToolError::DeserializationError("missing field".to_string());
        assert_eq!(err.to_string(), "failed to deserialize arguments: missing field");
    }

    #[test]
    fn test_tool_error_serialization() {
        let err = ToolError::SerializationError("invalid type".to_string());
        assert_eq!(err.to_string(), "failed to serialize output: invalid type");
    }

    #[test]
    fn test_tool_error_execution() {
        let err = ToolError::ExecutionError("network timeout".to_string());
        assert_eq!(err.to_string(), "execution error: network timeout");
    }

    #[test]
    fn test_tool_error_invalid_tool_call() {
        let err = ToolError::InvalidToolCall;
        assert_eq!(err.to_string(), "invalid tool call: missing function");
    }

    #[test]
    fn test_tool_error_custom() {
        let err = ToolError::custom("something went wrong");
        assert_eq!(err.to_string(), "something went wrong");
    }

    #[test]
    fn test_tool_error_from_json_error() {
        let json_err = serde_json::from_str::<i32>("not a number").unwrap_err();
        let err = ToolError::from(json_err);
        assert!(matches!(err, ToolError::DeserializationError(_)));
    }
}
