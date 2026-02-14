//! HTTP client configuration

use std::time::Duration;

/// Configuration for Ollama HTTP client
///
/// This struct allows customization of the HTTP client behavior including
/// base URL, timeout, and retry settings.
///
/// # Examples
///
/// ```no_run
/// use ollama_oxide::ClientConfig;
/// use std::time::Duration;
///
/// // Use default configuration
/// let config = ClientConfig::default();
///
/// // Custom configuration
/// let config = ClientConfig {
///     base_url: "http://example.com:8080".to_string(),
///     timeout: Duration::from_secs(60),
///     max_retries: 5,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Base URL for Ollama API
    ///
    /// Must include the scheme (http:// or https://)
    pub base_url: String,

    /// Request timeout duration
    ///
    /// How long to wait for a response before timing out
    pub timeout: Duration,

    /// Maximum retry attempts on failure
    ///
    /// Number of times to retry a failed request (0 = no retries)
    pub max_retries: u32,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:11434".to_string(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
        }
    }
}

impl ClientConfig {
    /// Build full URL from base URL and endpoint path
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::ClientConfig;
    ///
    /// let config = ClientConfig::default();
    /// let url = config.url("/api/version");
    /// assert_eq!(url, "http://localhost:11434/api/version");
    /// ```
    #[inline]
    pub fn url(&self, endpoint: &str) -> String {
        format!("{}{}", self.base_url, endpoint)
    }
}
