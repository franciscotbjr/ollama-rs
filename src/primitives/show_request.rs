//! Show model request primitive type

use serde::{Deserialize, Serialize};

/// Request body for POST /api/show endpoint
///
/// Retrieves detailed information about a specific model.
///
/// # Example
///
/// ```no_run
/// use ollama_oxide::ShowRequest;
///
/// // Basic request
/// let request = ShowRequest {
///     model: "llama3.1".to_string(),
///     verbose: None,
/// };
///
/// // Verbose request for more details
/// let verbose_request = ShowRequest {
///     model: "llama3.1".to_string(),
///     verbose: Some(true),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShowRequest {
    /// Name of the model to show information for
    pub model: String,

    /// If true, includes large verbose fields in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbose: Option<bool>,
}

impl ShowRequest {
    /// Create a new ShowRequest for the specified model
    ///
    /// # Arguments
    ///
    /// * `model` - Name of the model to query
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ollama_oxide::ShowRequest;
    ///
    /// let request = ShowRequest::new("llama3.1");
    /// ```
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            verbose: None,
        }
    }

    /// Create a verbose ShowRequest for detailed model information
    ///
    /// # Arguments
    ///
    /// * `model` - Name of the model to query
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ollama_oxide::ShowRequest;
    ///
    /// let request = ShowRequest::verbose("llama3.1");
    /// assert_eq!(request.verbose, Some(true));
    /// ```
    pub fn verbose(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            verbose: Some(true),
        }
    }
}
