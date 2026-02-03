use serde::{Deserialize, Serialize};

/// Request body for POST /api/pull endpoint.
///
/// Downloads a model from the Ollama registry.
///
/// # JSON Examples
///
/// Minimal request:
/// ```json
/// {
///   "model": "llama3.2:latest"
/// }
/// ```
///
/// Full request with options:
/// ```json
/// {
///   "model": "llama3.2:latest",
///   "insecure": false,
///   "stream": false
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequest {
    /// Name of the model to download (e.g., "llama3.2:latest", "gemma:7b")
    pub model: String,

    /// Allow downloading over insecure connections (without TLS verification)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,

    /// Stream progress updates. Default: true in Ollama, but we set false for v0.1.0
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

impl PullRequest {
    /// Create a new pull request for the specified model.
    ///
    /// The request is configured with `stream: false` for non-streaming mode.
    ///
    /// # Arguments
    ///
    /// * `model` - Name of the model to download (e.g., "llama3.2:latest")
    ///
    /// # Example
    ///
    /// ```
    /// use ollama_oxide::PullRequest;
    ///
    /// let request = PullRequest::new("llama3.2:latest");
    /// ```
    pub fn new<M: Into<String>>(model: M) -> Self {
        Self {
            model: model.into(),
            insecure: None,
            stream: Some(false), // v0.1.0: non-streaming only
        }
    }

    /// Allow downloading over insecure connections.
    ///
    /// When set to `true`, the download will proceed without TLS verification.
    /// Use with caution, only in trusted network environments.
    ///
    /// # Arguments
    ///
    /// * `insecure` - Whether to allow insecure connections
    ///
    /// # Example
    ///
    /// ```
    /// use ollama_oxide::PullRequest;
    ///
    /// let request = PullRequest::new("llama3.2:latest")
    ///     .with_insecure(true);
    /// ```
    pub fn with_insecure(mut self, insecure: bool) -> Self {
        self.insecure = Some(insecure);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_request_with_model() {
        let request = PullRequest::new("llama3.2:latest");
        assert_eq!(request.model, "llama3.2:latest");
        assert_eq!(request.insecure, None);
    }

    #[test]
    fn test_with_insecure_sets_flag() {
        let request = PullRequest::new("llama3.2:latest").with_insecure(true);
        assert_eq!(request.insecure, Some(true));
    }

    #[test]
    fn test_serialization_minimal() {
        let request = PullRequest::new("llama3.2:latest");
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["model"], "llama3.2:latest");
        assert_eq!(json["stream"], false);
        assert!(json.get("insecure").is_none());
    }

    #[test]
    fn test_serialization_full() {
        let request = PullRequest::new("llama3.2:latest").with_insecure(true);
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["model"], "llama3.2:latest");
        assert_eq!(json["insecure"], true);
        assert_eq!(json["stream"], false);
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{"model": "llama3.2:latest", "insecure": false}"#;
        let request: PullRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.model, "llama3.2:latest");
        assert_eq!(request.insecure, Some(false));
    }
}
