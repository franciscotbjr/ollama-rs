//! Example: Chat with tools using external weather API
//!
//! Demonstrates a complete chat flow with tool calling where the model
//! requests weather information and we execute the tool against a real
//! (or mocked) weather service.
//!
//! The example shows:
//! - Defining a weather tool using the type-safe `Tool` trait
//! - Sending a chat request with tool definitions
//! - Handling tool call responses from the model
//! - Executing tool calls (with mock HTTP service)
//! - Sending tool results back to continue the conversation
//!
//! Run with: cargo run --example chat_with_tools_async --features tools
//!
//! ## Free Weather API Reference
//! This example uses Open-Meteo (https://open-meteo.com/) as reference.
//! Open-Meteo is a free, open-source weather API that requires no authentication.
//! Real API endpoint: https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}&current=temperature_2m,weather_code

use ollama_oxide::tools::{Tool, ToolError, ToolRegistry, ToolResult};
use ollama_oxide::{ChatMessage, ChatRequest, OllamaApiAsync, OllamaClient};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Weather Service (Mock HTTP Client)
// ---------------------------------------------------------------------------

/// Mock weather service that simulates HTTP calls to a weather API.
/// In production, you would use reqwest or similar to make real HTTP requests.
///
/// Real Open-Meteo API example:
/// GET https://api.open-meteo.com/v1/forecast?latitude=48.8566&longitude=2.3522&current=temperature_2m,relative_humidity_2m,weather_code
struct WeatherService;

impl WeatherService {
    /// Simulates an HTTP GET request to a weather API.
    /// Returns mock weather data based on the location.
    async fn fetch_weather(location: &str, format: &str) -> Result<WeatherData, WeatherApiError> {
        // Simulate network latency
        #[cfg(not(test))]
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Mock location coordinates (in real implementation, use geocoding API)
        let (lat, lon, city) = Self::geocode_location(location)?;

        // Simulate API response based on location
        // In production: reqwest::get(format!("https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}&current=temperature_2m,weather_code"))
        let (temp_celsius, weather_code) = Self::mock_weather_data(&city);

        let temperature = match format {
            "fahrenheit" => temp_celsius * 9.0 / 5.0 + 32.0,
            _ => temp_celsius,
        };

        let unit = match format {
            "fahrenheit" => "°F",
            _ => "°C",
        };

        Ok(WeatherData {
            location: city,
            latitude: lat,
            longitude: lon,
            temperature,
            unit: unit.to_string(),
            description: Self::weather_code_to_description(weather_code),
            weather_code,
        })
    }

    /// Mock geocoding - converts location name to coordinates.
    /// Real implementation would use a geocoding API.
    fn geocode_location(location: &str) -> Result<(f64, f64, String), WeatherApiError> {
        let location_lower = location.to_lowercase();

        // Mock database of locations
        let locations = [
            ("paris", 48.8566, 2.3522, "Paris, France"),
            ("london", 51.5074, -0.1278, "London, UK"),
            ("new york", 40.7128, -74.0060, "New York, USA"),
            ("tokyo", 35.6762, 139.6503, "Tokyo, Japan"),
            ("san francisco", 37.7749, -122.4194, "San Francisco, USA"),
            ("sydney", -33.8688, 151.2093, "Sydney, Australia"),
            ("berlin", 52.5200, 13.4050, "Berlin, Germany"),
            ("são paulo", -23.5505, -46.6333, "São Paulo, Brazil"),
            ("sao paulo", -23.5505, -46.6333, "São Paulo, Brazil"),
        ];

        for (name, lat, lon, full_name) in locations {
            if location_lower.contains(name) {
                return Ok((lat, lon, full_name.to_string()));
            }
        }

        // Default fallback with approximate coordinates
        Ok((0.0, 0.0, location.to_string()))
    }

    /// Mock weather data based on city (simulates API response).
    fn mock_weather_data(city: &str) -> (f64, u8) {
        // Return (temperature_celsius, weather_code)
        // Weather codes follow WMO standard used by Open-Meteo
        match city {
            "Paris, France" => (18.5, 2),     // Partly cloudy
            "London, UK" => (14.0, 61),       // Light rain
            "New York, USA" => (22.0, 1),     // Mainly clear
            "Tokyo, Japan" => (26.0, 3),      // Overcast
            "San Francisco, USA" => (16.0, 45), // Fog
            "Sydney, Australia" => (20.0, 0), // Clear sky
            "Berlin, Germany" => (15.0, 51),  // Light drizzle
            "São Paulo, Brazil" => (24.0, 80), // Light showers
            _ => (20.0, 0),                   // Default: Clear, 20°C
        }
    }

    /// Convert WMO weather code to human-readable description.
    /// See: https://open-meteo.com/en/docs (Weather code documentation)
    fn weather_code_to_description(code: u8) -> String {
        match code {
            0 => "Clear sky",
            1 => "Mainly clear",
            2 => "Partly cloudy",
            3 => "Overcast",
            45 | 48 => "Foggy",
            51 | 53 | 55 => "Drizzle",
            61 | 63 | 65 => "Rain",
            71 | 73 | 75 => "Snow",
            80 | 81 | 82 => "Rain showers",
            95 => "Thunderstorm",
            _ => "Unknown",
        }
        .to_string()
    }
}

/// Weather data returned by the weather service.
#[derive(Debug, Serialize)]
struct WeatherData {
    location: String,
    latitude: f64,
    longitude: f64,
    temperature: f64,
    unit: String,
    description: String,
    weather_code: u8,
}

/// Error type for weather API calls.
#[derive(Debug)]
#[allow(dead_code)]
enum WeatherApiError {
    LocationNotFound(String),
    NetworkError(String),
}

impl std::fmt::Display for WeatherApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LocationNotFound(loc) => write!(f, "Location not found: {}", loc),
            Self::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

// ---------------------------------------------------------------------------
// Tool Definition: GetCurrentWeather
// ---------------------------------------------------------------------------

/// Parameters for the get_current_weather tool.
/// These match the JSON schema that will be sent to the model.
#[derive(Debug, Deserialize, JsonSchema)]
struct GetCurrentWeatherParams {
    /// The location to get the weather for, e.g. "San Francisco, CA" or "Paris"
    location: String,

    /// The format to return the weather in: "celsius" or "fahrenheit"
    format: WeatherFormat,
}

/// Temperature format enum with schema generation.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
enum WeatherFormat {
    Celsius,
    Fahrenheit,
}

impl WeatherFormat {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Celsius => "celsius",
            Self::Fahrenheit => "fahrenheit",
        }
    }
}

/// Output from the weather tool.
#[derive(Debug, Serialize)]
struct GetCurrentWeatherOutput {
    location: String,
    temperature: f64,
    unit: String,
    conditions: String,
    coordinates: Coordinates,
}

#[derive(Debug, Serialize)]
struct Coordinates {
    latitude: f64,
    longitude: f64,
}

/// The weather tool implementation using the `Tool` trait.
struct GetCurrentWeatherTool;

impl Tool for GetCurrentWeatherTool {
    type Params = GetCurrentWeatherParams;
    type Output = GetCurrentWeatherOutput;

    fn name(&self) -> &'static str {
        "get_current_weather"
    }

    fn description(&self) -> &'static str {
        "Get the current weather for a location"
    }

    async fn execute(&self, params: Self::Params) -> ToolResult<Self::Output> {
        println!("  [Tool] Fetching weather for: {}", params.location);

        // Call the mock weather service (would be real HTTP in production)
        let weather = WeatherService::fetch_weather(&params.location, params.format.as_str())
            .await
            .map_err(|e| ToolError::ExecutionError(e.to_string()))?;

        Ok(GetCurrentWeatherOutput {
            location: weather.location,
            temperature: weather.temperature,
            unit: weather.unit,
            conditions: weather.description,
            coordinates: Coordinates {
                latitude: weather.latitude,
                longitude: weather.longitude,
            },
        })
    }
}

// ---------------------------------------------------------------------------
// Main: Chat with Tools Flow
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OllamaClient::default()?;
    let model = "qwen3:0.6b";

    println!("=== Chat with Tools Example ===");
    println!("Model: {}", model);
    println!();

    // Create tool registry and register the weather tool
    let mut registry = ToolRegistry::new();
    registry.register(GetCurrentWeatherTool);

    println!("Registered tools: {:?}", registry);
    println!();

    // ---------------------------------------------------------------------------
    // Step 1: Send user message with tool definitions
    // ---------------------------------------------------------------------------
    println!("--- Step 1: User asks about weather ---");

    let user_message = "What is the weather today in Paris?";
    println!("User: {}", user_message);

    let request = ChatRequest::new(model, [ChatMessage::user(user_message)])
        .with_tools(registry.definitions());

    println!("\nSending request to model...");
    let response = client.chat(&request).await?;

    // ---------------------------------------------------------------------------
    // Step 2: Handle tool calls from the model
    // ---------------------------------------------------------------------------
    if response.has_tool_calls() {
        println!("\n--- Step 2: Model requests tool execution ---");

        let tool_calls = response.tool_calls().unwrap();

        for call in tool_calls {
            println!("Tool requested: {:?}", call.function_name());
            println!("Arguments: {}", serde_json::to_string_pretty(call.arguments().unwrap_or(&serde_json::json!({})))?);
        }

        // Execute all tool calls via the registry
        println!("\n--- Step 3: Executing tool calls ---");
        let results = registry.execute_all(&response).await;

        // Collect results for the follow-up message
        let mut tool_results = Vec::new();

        for (call, result) in tool_calls.iter().zip(results.iter()) {
            match result {
                Ok(output) => {
                    println!("  Result: {}", serde_json::to_string_pretty(output)?);
                    tool_results.push((call.clone(), output.clone()));
                }
                Err(e) => {
                    println!("  Error: {}", e);
                    tool_results.push((call.clone(), serde_json::json!({"error": e.to_string()})));
                }
            }
        }

        // ---------------------------------------------------------------------------
        // Step 4: Send tool results back to the model for final response
        // ---------------------------------------------------------------------------
        println!("\n--- Step 4: Sending tool results to model ---");

        // Build the conversation with tool call and response
        let mut messages = vec![
            ChatMessage::user(user_message),
            ChatMessage::assistant("").with_tool_calls(tool_calls.to_vec()),
        ];

        // Add tool response messages
        for (_call, result) in &tool_results {
            messages.push(ChatMessage::tool(&serde_json::to_string(result)?));
        }

        let follow_up_request =
            ChatRequest::new(model, messages).with_tools(registry.definitions());

        let final_response = client.chat(&follow_up_request).await?;

        println!("\n--- Step 5: Final response from model ---");
        if let Some(content) = final_response.content() {
            println!("Assistant: {}", content);
        } else {
            println!("(No text response)");
        }
    } else {
        // Model answered directly without using tools
        println!("\nAssistant (direct response): {}", response.content().unwrap_or("No response"));
    }

    println!("\nDone!");

    Ok(())
}
