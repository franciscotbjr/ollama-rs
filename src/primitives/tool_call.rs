//! Tool call type for function calling responses.

use serde::{Deserialize, Serialize};

use super::ToolCallFunction;

/// A tool call requested by the assistant.
///
/// When the model decides to use a tool, it returns one or more `ToolCall`
/// objects in the response. Each tool call contains the function details
/// (name and arguments) that your application should execute.
///
/// # Important
///
/// Tool calls are **executed on the client side** (your Rust application),
/// not on the Ollama server. The server only generates structured JSON
/// describing which tool to call and with what arguments. Your application
/// must:
///
/// 1. Parse the tool call
/// 2. Execute the actual function
/// 3. Send the result back in a follow-up request
///
/// # Examples
///
/// ```ignore
/// use ollama_oxide::ToolCall;
///
/// // Typically received from API response, not constructed manually
/// let json = r#"{
///     "function": {
///         "name": "get_weather",
///         "arguments": {"location": "Paris"}
///     }
/// }"#;
///
/// let call: ToolCall = serde_json::from_str(json).unwrap();
/// assert_eq!(call.function_name(), Some("get_weather"));
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ToolCall {
    /// Function call details.
    ///
    /// Contains the function name and arguments to pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<ToolCallFunction>,
}

impl ToolCall {
    /// Create a new tool call with function details.
    ///
    /// # Arguments
    ///
    /// * `function` - The function call details
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ollama_oxide::{ToolCall, ToolCallFunction};
    /// use serde_json::json;
    ///
    /// let func = ToolCallFunction::with_arguments("search", json!({"query": "rust"}));
    /// let call = ToolCall::new(func);
    /// assert_eq!(call.function_name(), Some("search"));
    /// ```
    pub fn new(function: ToolCallFunction) -> Self {
        Self {
            function: Some(function),
        }
    }

    /// Get the function name if available.
    ///
    /// # Returns
    ///
    /// Returns `Some(&str)` with the function name if the function field
    /// is present, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ollama_oxide::{ToolCall, ToolCallFunction};
    ///
    /// let call = ToolCall::new(ToolCallFunction::new("my_function"));
    /// assert_eq!(call.function_name(), Some("my_function"));
    ///
    /// let empty_call = ToolCall::default();
    /// assert_eq!(empty_call.function_name(), None);
    /// ```
    pub fn function_name(&self) -> Option<&str> {
        self.function.as_ref().map(|f| f.name.as_str())
    }

    /// Get the function arguments if available.
    ///
    /// # Returns
    ///
    /// Returns `Some(&Value)` with the arguments if present, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ollama_oxide::{ToolCall, ToolCallFunction};
    /// use serde_json::json;
    ///
    /// let func = ToolCallFunction::with_arguments("calc", json!({"x": 42}));
    /// let call = ToolCall::new(func);
    ///
    /// let args = call.arguments().unwrap();
    /// assert_eq!(args["x"], 42);
    /// ```
    pub fn arguments(&self) -> Option<&serde_json::Value> {
        self.function.as_ref().and_then(|f| f.arguments.as_ref())
    }

    /// Get the function arguments as a typed value.
    ///
    /// Attempts to deserialize the arguments JSON into the specified type.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to deserialize into
    ///
    /// # Returns
    ///
    /// Returns `Some(T)` if arguments exist and can be deserialized,
    /// `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ollama_oxide::{ToolCall, ToolCallFunction};
    /// use serde::Deserialize;
    /// use serde_json::json;
    ///
    /// #[derive(Deserialize, Debug)]
    /// struct CalcArgs {
    ///     expression: String,
    /// }
    ///
    /// let func = ToolCallFunction::with_arguments(
    ///     "calculate",
    ///     json!({"expression": "2 + 2"})
    /// );
    /// let call = ToolCall::new(func);
    ///
    /// let args: Option<CalcArgs> = call.arguments_as();
    /// assert_eq!(args.unwrap().expression, "2 + 2");
    /// ```
    pub fn arguments_as<T>(&self) -> Option<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.function.as_ref().and_then(|f| f.arguments_as())
    }

    /// Check if this tool call has valid function details.
    ///
    /// # Returns
    ///
    /// Returns `true` if the function field is present and has a name.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ollama_oxide::{ToolCall, ToolCallFunction};
    ///
    /// let valid_call = ToolCall::new(ToolCallFunction::new("test"));
    /// assert!(valid_call.is_valid());
    ///
    /// let empty_call = ToolCall::default();
    /// assert!(!empty_call.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        self.function.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_tool_call_new() {
        let func = ToolCallFunction::new("test");
        let call = ToolCall::new(func);
        assert!(call.function.is_some());
        assert_eq!(call.function_name(), Some("test"));
    }

    #[test]
    fn test_tool_call_default() {
        let call = ToolCall::default();
        assert!(call.function.is_none());
        assert_eq!(call.function_name(), None);
    }

    #[test]
    fn test_tool_call_function_name() {
        let call = ToolCall::new(ToolCallFunction::new("my_func"));
        assert_eq!(call.function_name(), Some("my_func"));
    }

    #[test]
    fn test_tool_call_arguments() {
        let func = ToolCallFunction::with_arguments("test", json!({"a": 1, "b": "two"}));
        let call = ToolCall::new(func);

        let args = call.arguments().unwrap();
        assert_eq!(args["a"], 1);
        assert_eq!(args["b"], "two");
    }

    #[test]
    fn test_tool_call_arguments_none() {
        let call = ToolCall::new(ToolCallFunction::new("no_args"));
        assert!(call.arguments().is_none());
    }

    #[test]
    fn test_tool_call_arguments_as() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct Args {
            x: i32,
        }

        let func = ToolCallFunction::with_arguments("test", json!({"x": 42}));
        let call = ToolCall::new(func);

        let args: Option<Args> = call.arguments_as();
        assert_eq!(args, Some(Args { x: 42 }));
    }

    #[test]
    fn test_tool_call_is_valid() {
        let valid = ToolCall::new(ToolCallFunction::new("test"));
        assert!(valid.is_valid());

        let invalid = ToolCall::default();
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_tool_call_serialize() {
        let func = ToolCallFunction::with_arguments("weather", json!({"city": "NYC"}));
        let call = ToolCall::new(func);

        let json = serde_json::to_value(&call).unwrap();
        assert_eq!(json["function"]["name"], "weather");
        assert_eq!(json["function"]["arguments"]["city"], "NYC");
    }

    #[test]
    fn test_tool_call_serialize_empty() {
        let call = ToolCall::default();
        let json = serde_json::to_string(&call).unwrap();
        assert_eq!(json, "{}"); // function is skipped when None
    }

    #[test]
    fn test_tool_call_deserialize() {
        let json = r#"{
            "function": {
                "name": "calculate",
                "arguments": {"expression": "1 + 1"}
            }
        }"#;

        let call: ToolCall = serde_json::from_str(json).unwrap();
        assert_eq!(call.function_name(), Some("calculate"));
        assert_eq!(call.arguments().unwrap()["expression"], "1 + 1");
    }

    #[test]
    fn test_tool_call_deserialize_empty() {
        let json = "{}";
        let call: ToolCall = serde_json::from_str(json).unwrap();
        assert!(call.function.is_none());
    }

    #[test]
    fn test_tool_call_deserialize_null_function() {
        let json = r#"{"function": null}"#;
        let call: ToolCall = serde_json::from_str(json).unwrap();
        assert!(call.function.is_none());
    }

    #[test]
    fn test_tool_call_clone() {
        let call = ToolCall::new(ToolCallFunction::with_arguments("test", json!({"x": 1})));
        let cloned = call.clone();
        assert_eq!(call, cloned);
    }

    #[test]
    fn test_tool_call_equality() {
        let call1 = ToolCall::new(ToolCallFunction::new("a"));
        let call2 = ToolCall::new(ToolCallFunction::new("a"));
        let call3 = ToolCall::new(ToolCallFunction::new("b"));

        assert_eq!(call1, call2);
        assert_ne!(call1, call3);
    }
}
