//! # ollama-oxide
//!
//! A Rust library for integrating with Ollama's native API.
//!
//! ## Quick Start
//!
//! ### Async Example
//! ```no_run
//! use ollama_oxide::{OllamaClient, OllamaApiAsync, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let client = OllamaClient::default()?;
//!     let version = client.version().await?;
//!     println!("Ollama version: {}", version.version);
//!     Ok(())
//! }
//! ```
//!
//! ### Sync Example
//! ```no_run
//! use ollama_oxide::{OllamaClient, OllamaApiSync, Result};
//!
//! fn main() -> Result<()> {
//!     let client = OllamaClient::default()?;
//!     let version = client.version_blocking()?;
//!     println!("Ollama version: {}", version.version);
//!     Ok(())
//! }
//! ```

// ============================================================================
// Error Handling
// ============================================================================

mod error;

pub use error::{Error, Result};

// ============================================================================
// Primitives Module
// ============================================================================

#[cfg(feature = "primitives")]
pub mod primitives;

#[cfg(feature = "primitives")]
pub use primitives::{
    // Chat types
    ChatMessage,
    ChatRequest,
    ChatResponse,
    ChatRole,
    // Other primitives
    EmbedInput,
    EmbedRequest,
    EmbedResponse,
    FormatSetting,
    GenerateRequest,
    GenerateResponse,
    KeepAliveSetting,
    Logprob,
    ModelOptions,
    ResponseMessage,
    StopSetting,
    ThinkSetting,
    TokenLogprob,
    VersionResponse,
};

// Tool types re-exports (requires both "primitives" and "tools" features)
#[cfg(all(feature = "primitives", feature = "tools"))]
pub use primitives::{ToolCall, ToolCallFunction, ToolDefinition, ToolFunction};

// ============================================================================
// HTTP Client Module
// ============================================================================

#[cfg(feature = "http")]
pub mod http;

#[cfg(feature = "http")]
pub use http::{ClientConfig, OllamaApiAsync, OllamaApiSync, OllamaClient};

// ============================================================================
// Model Module
// ============================================================================

#[cfg(feature = "model")]
pub mod model;

#[cfg(feature = "model")]
pub use model::{
    CopyRequest,
    CreateRequest,
    CreateResponse,
    DeleteRequest,
    LicenseSetting,
    ListResponse,
    ModelDetails,
    ModelSummary,
    PsResponse,
    RunningModel,
    ShowModelDetails,
    ShowRequest,
    ShowResponse,
};

// ============================================================================
// Ergonomic Tools Module (requires "tools" feature)
// ============================================================================

#[cfg(feature = "tools")]
pub mod tools;

// ============================================================================
// Prelude
// ============================================================================

pub mod prelude {
    pub use crate::{Error, Result};

    #[cfg(feature = "http")]
    pub use crate::{ClientConfig, OllamaApiAsync, OllamaApiSync, OllamaClient};

    #[cfg(feature = "primitives")]
    pub use crate::{
        // Chat types
        ChatMessage,
        ChatRequest,
        ChatResponse,
        ChatRole,
        // Other primitives
        EmbedInput,
        EmbedRequest,
        EmbedResponse,
        FormatSetting,
        GenerateRequest,
        GenerateResponse,
        KeepAliveSetting,
        Logprob,
        ModelOptions,
        ResponseMessage,
        StopSetting,
        ThinkSetting,
        TokenLogprob,
        VersionResponse,
    };

    // Tool types (requires both "primitives" and "tools" features)
    #[cfg(all(feature = "primitives", feature = "tools"))]
    pub use crate::{ToolCall, ToolCallFunction, ToolDefinition, ToolFunction};

    // Model types (requires "model" feature)
    #[cfg(feature = "model")]
    pub use crate::{
        CopyRequest,
        CreateRequest,
        CreateResponse,
        DeleteRequest,
        LicenseSetting,
        ListResponse,
        ModelDetails,
        ModelSummary,
        PsResponse,
        RunningModel,
        ShowModelDetails,
        ShowRequest,
        ShowResponse,
    };
}
