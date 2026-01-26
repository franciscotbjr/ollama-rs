//! Tool call function details for function calling responses.

use serde::{Deserialize, Serialize};

/// Function call details in a tool call response.
///
/// Contains the name of the function to call and the arguments
/// to pass to it. This is returned by the model when it decides
/// to invoke a tool.
///
/// # Examples
///
/// ```ignore
/// use ollama_oxide::ToolCallFunction;
/// use serde_json::json;
///
/// // Deserialize from a typical API response
/// let json = r#"{
///     "name": "get_weather",
///     "arguments": {"location": "Paris", "unit": "celsius"}
/// }"#;
///
/// let func: ToolCallFunction = serde_json::from_str(json).unwrap();
/// assert_eq!(func.name, "get_weather");
/// assert!(func.arguments.is_some());
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolCallFunction {
    /// Name of the function to call.
    pub name: String,

    /// Human-readable description of what the function does.
    ///
    /// This field is optional and may not always be present in responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Arguments to pass to the function as a JSON object.
    ///
    /// The structure of arguments depends on the function's parameter schema.
    /// This may be `None` for functions that take no arguments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<serde_json::Value>,
}

impl ToolCallFunction {
    /// Create a new tool call function with just a name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ollama_oxide::ToolCallFunction;
    ///
    /// let func = ToolCallFunction::new("get_time");
    /// assert_eq!(func.name, "get_time");
    /// assert!(func.arguments.is_none());
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            arguments: None,
        }
    }

    /// Create a new tool call function with name and arguments.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function
    /// * `arguments` - The arguments to pass to the function
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ollama_oxide::ToolCallFunction;
    /// use serde_json::json;
    ///
    /// let func = ToolCallFunction::with_arguments(
    ///     "get_weather",
    ///     json!({"location": "Paris"})
    /// );
    /// assert_eq!(func.name, "get_weather");
    /// assert!(func.arguments.is_some());
    /// ```
    pub fn with_arguments(name: impl Into<String>, arguments: serde_json::Value) -> Self {
        Self {
            name: name.into(),
            description: None,
            arguments: Some(arguments),
        }
    }

    /// Add a description to this function call.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ollama_oxide::ToolCallFunction;
    ///
    /// let func = ToolCallFunction::new("search")
    ///     .with_description("Search the web for information");
    /// assert_eq!(func.description.unwrap(), "Search the web for information");
    /// ```
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Get the arguments as a typed value.
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
    /// use ollama_oxide::ToolCallFunction;
    /// use serde::Deserialize;
    /// use serde_json::json;
    ///
    /// #[derive(Deserialize, PartialEq, Debug)]
    /// struct WeatherArgs {
    ///     location: String,
    /// }
    ///
    /// let func = ToolCallFunction::with_arguments(
    ///     "get_weather",
    ///     json!({"location": "Paris"})
    /// );
    ///
    /// let args: Option<WeatherArgs> = func.arguments_as();
    /// assert_eq!(args.unwrap().location, "Paris");
    /// ```
    pub fn arguments_as<T>(&self) -> Option<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.arguments
            .as_ref()
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_tool_call_function_new() {
        let func = ToolCallFunction::new("test_function");
        assert_eq!(func.name, "test_function");
        assert!(func.description.is_none());
        assert!(func.arguments.is_none());
    }

    #[test]
    fn test_tool_call_function_with_arguments() {
        let args = json!({"key": "value"});
        let func = ToolCallFunction::with_arguments("test", args.clone());
        assert_eq!(func.name, "test");
        assert_eq!(func.arguments, Some(args));
    }

    #[test]
    fn test_tool_call_function_with_description() {
        let func = ToolCallFunction::new("test").with_description("A test function");
        assert_eq!(func.description, Some("A test function".to_string()));
    }

    #[test]
    fn test_tool_call_function_serialize() {
        let func = ToolCallFunction::with_arguments("get_weather", json!({"location": "Paris"}));

        let json = serde_json::to_value(&func).unwrap();
        assert_eq!(json["name"], "get_weather");
        assert_eq!(json["arguments"]["location"], "Paris");
        assert!(json.get("description").is_none()); // Skipped when None
    }

    #[test]
    fn test_tool_call_function_deserialize() {
        let json = r#"{
            "name": "calculate",
            "arguments": {"expression": "2 + 2"}
        }"#;

        let func: ToolCallFunction = serde_json::from_str(json).unwrap();
        assert_eq!(func.name, "calculate");
        assert_eq!(
            func.arguments.unwrap()["expression"],
            json!("2 + 2")
        );
    }

    #[test]
    fn test_tool_call_function_deserialize_with_description() {
        let json = r#"{
            "name": "search",
            "description": "Search the web",
            "arguments": {"query": "rust programming"}
        }"#;

        let func: ToolCallFunction = serde_json::from_str(json).unwrap();
        assert_eq!(func.name, "search");
        assert_eq!(func.description, Some("Search the web".to_string()));
    }

    #[test]
    fn test_tool_call_function_deserialize_minimal() {
        let json = r#"{"name": "no_args"}"#;

        let func: ToolCallFunction = serde_json::from_str(json).unwrap();
        assert_eq!(func.name, "no_args");
        assert!(func.description.is_none());
        assert!(func.arguments.is_none());
    }

    #[test]
    fn test_tool_call_function_arguments_as() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Args {
            x: i32,
            y: i32,
        }

        let func = ToolCallFunction::with_arguments("add", json!({"x": 1, "y": 2}));

        let args: Option<Args> = func.arguments_as();
        assert!(args.is_some());
        let args = args.unwrap();
        assert_eq!(args.x, 1);
        assert_eq!(args.y, 2);
    }

    #[test]
    fn test_tool_call_function_arguments_as_none() {
        let func = ToolCallFunction::new("no_args");
        let args: Option<serde_json::Value> = func.arguments_as();
        assert!(args.is_none());
    }

    #[test]
    fn test_tool_call_function_arguments_as_wrong_type() {
        let func = ToolCallFunction::with_arguments("test", json!({"x": "not a number"}));

        #[derive(Deserialize)]
        struct Args {
            x: i32,
        }

        let args: Option<Args> = func.arguments_as();
        assert!(args.is_none()); // Deserialization fails
    }

    #[test]
    fn test_tool_call_function_clone() {
        let func = ToolCallFunction::with_arguments("test", json!({"a": 1}))
            .with_description("desc");
        let cloned = func.clone();
        assert_eq!(func, cloned);
    }

    #[test]
    fn test_tool_call_function_equality() {
        let func1 = ToolCallFunction::with_arguments("test", json!({"a": 1}));
        let func2 = ToolCallFunction::with_arguments("test", json!({"a": 1}));
        let func3 = ToolCallFunction::with_arguments("test", json!({"a": 2}));

        assert_eq!(func1, func2);
        assert_ne!(func1, func3);
    }
}
