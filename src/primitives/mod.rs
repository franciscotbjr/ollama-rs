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
mod show_model_details;
mod show_request;
mod show_response;
mod version;

pub use copy_request::CopyRequest;
pub use delete_request::DeleteRequest;
pub use list_response::ListResponse;
pub use model_details::ModelDetails;
pub use model_summary::ModelSummary;
pub use ps_response::PsResponse;
pub use running_model::RunningModel;
pub use show_model_details::ShowModelDetails;
pub use show_request::ShowRequest;
pub use show_response::ShowResponse;
pub use version::VersionResponse;
