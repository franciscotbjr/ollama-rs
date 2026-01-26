//! Tool function definition for function calling.

use serde::{Deserialize, Serialize};

/// Function definition for a tool.
///
/// Describes a function that the model can call, including its name,
/// description, and parameter schema.
///
/// # Examples
///
/// ## Basic Function
///
/// ```ignore
/// use ollama_oxide::ToolFunction;
/// use serde_json::json;
///
/// let func = ToolFunction::new("get_time", json!({
///     "type": "object",
///     "properties": {}
/// }));
/// ```
///
/// ## Function with Parameters
///
/// ```ignore
/// use ollama_oxide::ToolFunction;
/// use serde_json::json;
///
/// let func = ToolFunction::new("get_weather", json!({
///     "type": "object",
///     "properties": {
///         "location": {
///             "type": "string",
///             "description": "City name"
///         },
///         "unit": {
///             "type": "string",
///             "enum": ["celsius", "fahrenheit"]
///         }
///     },
///     "required": ["location"]
/// })).with_description("Get current weather for a location");
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolFunction {
    /// Name of the function.
    ///
    /// Should be a valid identifier (lowercase, underscores, no spaces).
    pub name: String,

    /// Human-readable description of what the function does.
    ///
    /// This helps the model understand when to use the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// JSON Schema describing the function parameters.
    ///
    /// Should be a valid JSON Schema object describing the expected
    /// arguments. Use `{"type": "object", "properties": {}}` for
    /// functions with no parameters.
    pub parameters: serde_json::Value,
}

impl ToolFunction {
    /// Create a new function definition.
    ///
    /// # Arguments
    ///
    /// * `name` - The function name (should be a valid identifier)
    /// * `parameters` - JSON Schema for the function parameters
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ollama_oxide::ToolFunction;
    /// use serde_json::json;
    ///
    /// let func = ToolFunction::new("add_numbers", json!({
    ///     "type": "object",
    ///     "properties": {
    ///         "a": {"type": "number"},
    ///         "b": {"type": "number"}
    ///     },
    ///     "required": ["a", "b"]
    /// }));
    /// ```
    pub fn new(name: impl Into<String>, parameters: serde_json::Value) -> Self {
        Self {
            name: name.into(),
            description: None,
            parameters,
        }
    }

    /// Add a description to this function.
    ///
    /// Descriptions help the model understand when to use the function.
    /// Be clear and specific about what the function does.
    ///
    /// # Arguments
    ///
    /// * `description` - Human-readable description
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ollama_oxide::ToolFunction;
    /// use serde_json::json;
    ///
    /// let func = ToolFunction::new("search", json!({"type": "object", "properties": {}}))
    ///     .with_description("Search the web for information");
    /// assert_eq!(func.description.unwrap(), "Search the web for information");
    /// ```
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Create a function with no parameters.
    ///
    /// Convenience method for functions that don't take any arguments.
    ///
    /// # Arguments
    ///
    /// * `name` - The function name
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ollama_oxide::ToolFunction;
    ///
    /// let func = ToolFunction::no_params("get_current_time")
    ///     .with_description("Get the current time");
    /// ```
    pub fn no_params(name: impl Into<String>) -> Self {
        Self::new(
            name,
            serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_tool_function_new() {
        let params = json!({
            "type": "object",
            "properties": {
                "x": {"type": "number"}
            }
        });

        let func = ToolFunction::new("test", params.clone());
        assert_eq!(func.name, "test");
        assert!(func.description.is_none());
        assert_eq!(func.parameters, params);
    }

    #[test]
    fn test_tool_function_with_description() {
        let func = ToolFunction::new("test", json!({})).with_description("A test function");
        assert_eq!(func.description, Some("A test function".to_string()));
    }

    #[test]
    fn test_tool_function_no_params() {
        let func = ToolFunction::no_params("get_time");
        assert_eq!(func.name, "get_time");
        assert_eq!(func.parameters["type"], "object");
        assert!(func.parameters["properties"].is_object());
    }

    #[test]
    fn test_tool_function_serialize() {
        let func = ToolFunction::new(
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

        let json = serde_json::to_value(&func).unwrap();
        assert_eq!(json["name"], "get_weather");
        assert_eq!(json["description"], "Get weather");
        assert_eq!(json["parameters"]["type"], "object");
        assert_eq!(
            json["parameters"]["properties"]["location"]["type"],
            "string"
        );
    }

    #[test]
    fn test_tool_function_serialize_no_description() {
        let func = ToolFunction::new("test", json!({}));
        let json = serde_json::to_value(&func).unwrap();

        assert!(json.get("description").is_none()); // Skipped when None
    }

    #[test]
    fn test_tool_function_deserialize() {
        let json = r#"{
            "name": "calculate",
            "description": "Evaluate math",
            "parameters": {
                "type": "object",
                "properties": {
                    "expression": {"type": "string"}
                }
            }
        }"#;

        let func: ToolFunction = serde_json::from_str(json).unwrap();
        assert_eq!(func.name, "calculate");
        assert_eq!(func.description, Some("Evaluate math".to_string()));
    }

    #[test]
    fn test_tool_function_deserialize_minimal() {
        let json = r#"{
            "name": "simple",
            "parameters": {}
        }"#;

        let func: ToolFunction = serde_json::from_str(json).unwrap();
        assert_eq!(func.name, "simple");
        assert!(func.description.is_none());
    }

    #[test]
    fn test_tool_function_clone() {
        let func = ToolFunction::new("test", json!({"a": 1})).with_description("desc");
        let cloned = func.clone();
        assert_eq!(func, cloned);
    }

    #[test]
    fn test_tool_function_equality() {
        let func1 = ToolFunction::new("a", json!({}));
        let func2 = ToolFunction::new("a", json!({}));
        let func3 = ToolFunction::new("b", json!({}));

        assert_eq!(func1, func2);
        assert_ne!(func1, func3);
    }

    #[test]
    fn test_tool_function_complex_params() {
        let func = ToolFunction::new(
            "create_user",
            json!({
                "type": "object",
                "properties": {
                    "name": {"type": "string", "minLength": 1},
                    "age": {"type": "integer", "minimum": 0, "maximum": 150},
                    "email": {"type": "string", "format": "email"},
                    "roles": {
                        "type": "array",
                        "items": {"type": "string", "enum": ["admin", "user", "guest"]}
                    }
                },
                "required": ["name", "email"]
            }),
        );

        assert_eq!(func.parameters["properties"]["name"]["type"], "string");
        assert_eq!(func.parameters["properties"]["age"]["minimum"], 0);
        assert_eq!(func.parameters["required"][0], "name");
    }

    #[test]
    fn test_tool_function_into_string() {
        let func = ToolFunction::new(String::from("owned"), json!({}));
        assert_eq!(func.name, "owned");

        let func = ToolFunction::new("borrowed", json!({}));
        assert_eq!(func.name, "borrowed");
    }
}
