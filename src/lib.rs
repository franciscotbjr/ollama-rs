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
pub use primitives::{ListResponse, ModelDetails, ModelSummary, VersionResponse};

// ============================================================================
// HTTP Client Module
// ============================================================================

#[cfg(feature = "http")]
pub mod http;

#[cfg(feature = "http")]
pub use http::{ClientConfig, OllamaApiAsync, OllamaApiSync, OllamaClient};

// ============================================================================
// Prelude
// ============================================================================

pub mod prelude {
    pub use crate::{Error, Result};

    #[cfg(feature = "http")]
    pub use crate::{ClientConfig, OllamaApiAsync, OllamaApiSync, OllamaClient};

    #[cfg(feature = "primitives")]
    pub use crate::{ListResponse, ModelDetails, ModelSummary, VersionResponse};
}
