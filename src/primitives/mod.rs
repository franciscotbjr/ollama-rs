//! Primitive types for Ollama API responses and requests
//!
//! This module contains all primitive data types used in the Ollama API,
//! including request and response structures.

mod copy_request;
mod delete_request;
mod list_response;
mod model_details;
mod model_summary;
mod ps_response;
mod running_model;
mod version;

pub use copy_request::CopyRequest;
pub use delete_request::DeleteRequest;
pub use list_response::ListResponse;
pub use model_details::ModelDetails;
pub use model_summary::ModelSummary;
pub use ps_response::PsResponse;
pub use running_model::RunningModel;
pub use version::VersionResponse;
