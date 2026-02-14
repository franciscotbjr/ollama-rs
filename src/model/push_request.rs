use serde::{Deserialize, Serialize};

/// Request body for POST /api/push endpoint.
///
/// Uploads a model to a remote Ollama registry.
///
/// # JSON Examples
///
/// Minimal request:
/// ```json
/// {
///   "model": "namespace/model:tag"
/// }
/// ```
///
/// Full request with options:
/// ```json
/// {
///   "model": "namespace/model:tag",
///   "insecure": false,
///   "stream": false
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushRequest {
    /// Name of the model to push (e.g., "namespace/model:tag")
    pub model: String,

    /// Allow uploading over insecure connections (without TLS verification)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,

    /// Stream progress updates. Default: true in Ollama, but we set false for v0.1.0
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

impl PushRequest {
    /// Create a new push request for the specified model.
    ///
    /// The request is configured with `stream: false` for non-streaming mode.
    ///
    /// # Arguments
    ///
    /// * `model` - Name of the model to push (e.g., "namespace/model:tag")
    ///
    /// # Example
    ///
    /// ```
    /// use ollama_oxide::PushRequest;
    ///
    /// let request = PushRequest::new("myuser/mymodel:latest");
    /// ```
    pub fn new<M: Into<String>>(model: M) -> Self {
        Self {
            model: model.into(),
            insecure: None,
            stream: Some(false), // v0.1.0: non-streaming only
        }
    }

    /// Allow uploading over insecure connections.
    ///
    /// When set to `true`, the upload will proceed without TLS verification.
    /// Use with caution, only in trusted network environments.
    ///
    /// # Arguments
    ///
    /// * `insecure` - Whether to allow insecure connections
    ///
    /// # Example
    ///
    /// ```
    /// use ollama_oxide::PushRequest;
    ///
    /// let request = PushRequest::new("myuser/mymodel:latest")
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
        let request = PushRequest::new("myuser/mymodel:latest");
        assert_eq!(request.model, "myuser/mymodel:latest");
        assert_eq!(request.insecure, None);
    }

    #[test]
    fn test_with_insecure_sets_flag() {
        let request = PushRequest::new("myuser/mymodel:latest").with_insecure(true);
        assert_eq!(request.insecure, Some(true));
    }

    #[test]
    fn test_serialization_minimal() {
        let request = PushRequest::new("myuser/mymodel:latest");
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["model"], "myuser/mymodel:latest");
        assert_eq!(json["stream"], false);
        assert!(json.get("insecure").is_none());
    }

    #[test]
    fn test_serialization_full() {
        let request = PushRequest::new("myuser/mymodel:latest").with_insecure(true);
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["model"], "myuser/mymodel:latest");
        assert_eq!(json["insecure"], true);
        assert_eq!(json["stream"], false);
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{"model": "myuser/mymodel:latest", "insecure": false}"#;
        let request: PushRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.model, "myuser/mymodel:latest");
        assert_eq!(request.insecure, Some(false));
    }
}
