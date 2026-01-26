//! Tool definition for function calling.

use serde::{Deserialize, Serialize};

use super::ToolFunction;

/// Tool definition for function calling.
///
/// Wraps a function definition with a type field. Currently, the only
/// supported type is "function".
///
/// # Examples
///
/// ## Using the Factory Method
///
/// ```ignore
/// use ollama_oxide::ToolDefinition;
/// use serde_json::json;
///
/// let tool = ToolDefinition::function("get_weather", json!({
///     "type": "object",
///     "properties": {
///         "location": {"type": "string"}
///     },
///     "required": ["location"]
/// })).with_description("Get current weather for a location");
/// ```
///
/// ## Multiple Tools
///
/// ```ignore
/// use ollama_oxide::ToolDefinition;
/// use serde_json::json;
///
/// let tools = vec![
///     ToolDefinition::function("search", json!({
///         "type": "object",
///         "properties": {"query": {"type": "string"}}
///     })).with_description("Search the web"),
///
///     ToolDefinition::function("calculate", json!({
///         "type": "object",
///         "properties": {"expression": {"type": "string"}}
///     })).with_description("Evaluate a math expression"),
/// ];
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Type of tool (always "function" currently).
    #[serde(rename = "type")]
    pub type_field: String,

    /// Function definition with name, description, and parameters.
    pub function: ToolFunction,
}

impl ToolDefinition {
    /// Create a function tool definition.
    ///
    /// This is the recommended way to create tool definitions.
    ///
    /// # Arguments
    ///
    /// * `name` - The function name
    /// * `parameters` - JSON Schema for the function parameters
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ollama_oxide::ToolDefinition;
    /// use serde_json::json;
    ///
    /// let tool = ToolDefinition::function("greet", json!({
    ///     "type": "object",
    ///     "properties": {
    ///         "name": {"type": "string"}
    ///     },
    ///     "required": ["name"]
    /// }));
    ///
    /// assert_eq!(tool.type_field, "function");
    /// assert_eq!(tool.function.name, "greet");
    /// ```
    pub fn function(name: impl Into<String>, parameters: serde_json::Value) -> Self {
        Self {
            type_field: "function".to_string(),
            function: ToolFunction::new(name, parameters),
        }
    }

    /// Create a function tool with no parameters.
    ///
    /// Convenience method for tools that don't take any arguments.
    ///
    /// # Arguments
    ///
    /// * `name` - The function name
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ollama_oxide::ToolDefinition;
    ///
    /// let tool = ToolDefinition::function_no_params("get_time")
    ///     .with_description("Get the current time");
    /// ```
    pub fn function_no_params(name: impl Into<String>) -> Self {
        Self {
            type_field: "function".to_string(),
            function: ToolFunction::no_params(name),
        }
    }

    /// Create a tool definition from an existing function.
    ///
    /// # Arguments
    ///
    /// * `function` - The function definition
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ollama_oxide::{ToolDefinition, ToolFunction};
    /// use serde_json::json;
    ///
    /// let func = ToolFunction::new("test", json!({}))
    ///     .with_description("A test function");
    /// let tool = ToolDefinition::from_function(func);
    /// ```
    pub fn from_function(function: ToolFunction) -> Self {
        Self {
            type_field: "function".to_string(),
            function,
        }
    }

    /// Add a description to the function.
    ///
    /// # Arguments
    ///
    /// * `description` - Human-readable description
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ollama_oxide::ToolDefinition;
    /// use serde_json::json;
    ///
    /// let tool = ToolDefinition::function("search", json!({}))
    ///     .with_description("Search for information");
    ///
    /// assert_eq!(tool.function.description.unwrap(), "Search for information");
    /// ```
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.function.description = Some(description.into());
        self
    }

    /// Get the function name.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ollama_oxide::ToolDefinition;
    /// use serde_json::json;
    ///
    /// let tool = ToolDefinition::function("my_tool", json!({}));
    /// assert_eq!(tool.name(), "my_tool");
    /// ```
    pub fn name(&self) -> &str {
        &self.function.name
    }

    /// Get the function description if available.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ollama_oxide::ToolDefinition;
    /// use serde_json::json;
    ///
    /// let tool = ToolDefinition::function("test", json!({}))
    ///     .with_description("Does testing");
    /// assert_eq!(tool.description(), Some("Does testing"));
    /// ```
    pub fn description(&self) -> Option<&str> {
        self.function.description.as_deref()
    }

    /// Get the parameter schema.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ollama_oxide::ToolDefinition;
    /// use serde_json::json;
    ///
    /// let tool = ToolDefinition::function("calc", json!({
    ///     "type": "object",
    ///     "properties": {"x": {"type": "number"}}
    /// }));
    /// assert_eq!(tool.parameters()["type"], "object");
    /// ```
    pub fn parameters(&self) -> &serde_json::Value {
        &self.function.parameters
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_tool_definition_function() {
        let tool = ToolDefinition::function(
            "test",
            json!({
                "type": "object",
                "properties": {}
            }),
        );

        assert_eq!(tool.type_field, "function");
        assert_eq!(tool.function.name, "test");
        assert!(tool.function.description.is_none());
    }

    #[test]
    fn test_tool_definition_function_no_params() {
        let tool = ToolDefinition::function_no_params("get_time");
        assert_eq!(tool.type_field, "function");
        assert_eq!(tool.function.name, "get_time");
        assert_eq!(tool.function.parameters["type"], "object");
    }

    #[test]
    fn test_tool_definition_from_function() {
        let func = ToolFunction::new("custom", json!({})).with_description("Custom func");
        let tool = ToolDefinition::from_function(func);

        assert_eq!(tool.type_field, "function");
        assert_eq!(tool.function.name, "custom");
        assert_eq!(tool.function.description, Some("Custom func".to_string()));
    }

    #[test]
    fn test_tool_definition_with_description() {
        let tool = ToolDefinition::function("test", json!({})).with_description("A test tool");
        assert_eq!(tool.function.description, Some("A test tool".to_string()));
    }

    #[test]
    fn test_tool_definition_name() {
        let tool = ToolDefinition::function("my_func", json!({}));
        assert_eq!(tool.name(), "my_func");
    }

    #[test]
    fn test_tool_definition_description() {
        let tool = ToolDefinition::function("test", json!({}));
        assert!(tool.description().is_none());

        let tool = tool.with_description("Does stuff");
        assert_eq!(tool.description(), Some("Does stuff"));
    }

    #[test]
    fn test_tool_definition_parameters() {
        let params = json!({
            "type": "object",
            "properties": {
                "x": {"type": "number"}
            }
        });

        let tool = ToolDefinition::function("calc", params.clone());
        assert_eq!(tool.parameters(), &params);
    }

    #[test]
    fn test_tool_definition_serialize() {
        let tool = ToolDefinition::function(
            "get_weather",
            json!({
                "type": "object",
                "properties": {
                    "location": {"type": "string"}
                },
                "required": ["location"]
            }),
        )
        .with_description("Get weather");

        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(json["type"], "function");
        assert_eq!(json["function"]["name"], "get_weather");
        assert_eq!(json["function"]["description"], "Get weather");
        assert_eq!(json["function"]["parameters"]["type"], "object");
    }

    #[test]
    fn test_tool_definition_deserialize() {
        let json = r#"{
            "type": "function",
            "function": {
                "name": "search",
                "description": "Search the web",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "query": {"type": "string"}
                    }
                }
            }
        }"#;

        let tool: ToolDefinition = serde_json::from_str(json).unwrap();
        assert_eq!(tool.type_field, "function");
        assert_eq!(tool.function.name, "search");
        assert_eq!(tool.function.description, Some("Search the web".to_string()));
    }

    #[test]
    fn test_tool_definition_serialize_matches_api_format() {
        // Test that our serialization matches the expected Ollama API format
        let tool = ToolDefinition::function(
            "get_weather",
            json!({
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "City name"
                    },
                    "unit": {
                        "type": "string",
                        "enum": ["celsius", "fahrenheit"]
                    }
                },
                "required": ["location"]
            }),
        )
        .with_description("Get the current weather for a location");

        let json_value = serde_json::to_value(&tool).unwrap();
        let json_string = serde_json::to_string_pretty(&json_value).unwrap();

        // Verify structure matches API docs
        assert!(json_string.contains("\"type\": \"function\""));
        assert!(json_string.contains("\"name\": \"get_weather\""));
        assert!(json_string.contains("\"description\":"));
        assert!(json_string.contains("\"parameters\":"));
    }

    #[test]
    fn test_tool_definition_clone() {
        let tool = ToolDefinition::function("test", json!({})).with_description("desc");
        let cloned = tool.clone();
        assert_eq!(tool, cloned);
    }

    #[test]
    fn test_tool_definition_equality() {
        let tool1 = ToolDefinition::function("a", json!({}));
        let tool2 = ToolDefinition::function("a", json!({}));
        let tool3 = ToolDefinition::function("b", json!({}));

        assert_eq!(tool1, tool2);
        assert_ne!(tool1, tool3);
    }
}
