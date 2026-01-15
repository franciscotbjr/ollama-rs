//! Primitive types for Ollama API responses and requests
//!
//! This module contains all primitive data types used in the Ollama API,
//! including request and response structures.

mod version;
mod model_details;
mod model_summary;
mod list_response;

pub use version::VersionResponse;
pub use model_details::ModelDetails;
pub use model_summary::ModelSummary;
pub use list_response::ListResponse;
