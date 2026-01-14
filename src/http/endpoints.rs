//! API endpoint path constants
//!
//! This module defines all Ollama API endpoint paths as constants,
//! providing a single source of truth for API routes.

/// API endpoint paths relative to base URL
pub struct Endpoints;

#[allow(dead_code)]
impl Endpoints {
    /// GET /api/version - Get Ollama server version
    pub const VERSION: &'static str = "/api/version";

    /// POST /api/generate - Generate text from prompt
    pub const GENERATE: &'static str = "/api/generate";

    /// POST /api/chat - Generate chat messages
    pub const CHAT: &'static str = "/api/chat";

    /// POST /api/embed - Generate embeddings
    pub const EMBED: &'static str = "/api/embed";

    /// GET /api/tags - List available models
    pub const TAGS: &'static str = "/api/tags";

    /// GET /api/ps - List running models
    pub const PS: &'static str = "/api/ps";

    /// POST /api/show - Show model information
    pub const SHOW: &'static str = "/api/show";

    /// POST /api/create - Create a model from Modelfile
    pub const CREATE: &'static str = "/api/create";

    /// POST /api/copy - Copy a model
    pub const COPY: &'static str = "/api/copy";

    /// POST /api/pull - Pull a model from registry
    pub const PULL: &'static str = "/api/pull";

    /// POST /api/push - Push a model to registry
    pub const PUSH: &'static str = "/api/push";

    /// DELETE /api/delete - Delete a model
    pub const DELETE: &'static str = "/api/delete";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_endpoints_start_with_api() {
        assert!(Endpoints::VERSION.starts_with("/api/"));
        assert!(Endpoints::GENERATE.starts_with("/api/"));
        assert!(Endpoints::CHAT.starts_with("/api/"));
        assert!(Endpoints::EMBED.starts_with("/api/"));
        assert!(Endpoints::TAGS.starts_with("/api/"));
        assert!(Endpoints::PS.starts_with("/api/"));
        assert!(Endpoints::SHOW.starts_with("/api/"));
        assert!(Endpoints::CREATE.starts_with("/api/"));
        assert!(Endpoints::COPY.starts_with("/api/"));
        assert!(Endpoints::PULL.starts_with("/api/"));
        assert!(Endpoints::PUSH.starts_with("/api/"));
        assert!(Endpoints::DELETE.starts_with("/api/"));
    }

    #[test]
    fn test_endpoint_values() {
        assert_eq!(Endpoints::VERSION, "/api/version");
        assert_eq!(Endpoints::GENERATE, "/api/generate");
        assert_eq!(Endpoints::CHAT, "/api/chat");
        assert_eq!(Endpoints::EMBED, "/api/embed");
        assert_eq!(Endpoints::TAGS, "/api/tags");
        assert_eq!(Endpoints::PS, "/api/ps");
        assert_eq!(Endpoints::SHOW, "/api/show");
        assert_eq!(Endpoints::CREATE, "/api/create");
        assert_eq!(Endpoints::COPY, "/api/copy");
        assert_eq!(Endpoints::PULL, "/api/pull");
        assert_eq!(Endpoints::PUSH, "/api/push");
        assert_eq!(Endpoints::DELETE, "/api/delete");
    }
}
