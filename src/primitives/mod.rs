//! Primitive types for Ollama API responses and requests
//!
//! This module contains all primitive data types used in the Ollama API,
//! including request and response structures.

// Chat types
mod chat_message;
mod chat_request;
mod chat_response;
mod chat_role;
mod response_message;

// Tool types (requires "tools" feature)
#[cfg(feature = "tools")]
mod tool_call;
#[cfg(feature = "tools")]
mod tool_call_function;
#[cfg(feature = "tools")]
mod tool_definition;
#[cfg(feature = "tools")]
mod tool_function;

// Create types
mod create_request;
mod create_response;
mod license_setting;

// Other primitives
mod copy_request;
mod delete_request;
mod embed_input;
mod embed_request;
mod embed_response;
mod format_setting;
mod generate_request;
mod generate_response;
mod keep_alive_setting;
mod list_response;
mod logprob;
mod model_details;
mod model_options;
mod model_summary;
mod ps_response;
mod running_model;
mod show_model_details;
mod show_request;
mod show_response;
mod stop_setting;
mod think_setting;
mod token_logprob;
mod version;

// Chat types re-exports
pub use chat_message::ChatMessage;
pub use chat_request::ChatRequest;
pub use chat_response::ChatResponse;
pub use chat_role::ChatRole;
pub use response_message::ResponseMessage;

// Tool types re-exports (requires "tools" feature)
#[cfg(feature = "tools")]
pub use tool_call::ToolCall;
#[cfg(feature = "tools")]
pub use tool_call_function::ToolCallFunction;
#[cfg(feature = "tools")]
pub use tool_definition::ToolDefinition;
#[cfg(feature = "tools")]
pub use tool_function::ToolFunction;

// Create types re-exports
pub use create_request::CreateRequest;
pub use create_response::CreateResponse;
pub use license_setting::LicenseSetting;

// Other primitives re-exports
pub use copy_request::CopyRequest;
pub use delete_request::DeleteRequest;
pub use embed_input::EmbedInput;
pub use embed_request::EmbedRequest;
pub use embed_response::EmbedResponse;
pub use format_setting::FormatSetting;
pub use generate_request::GenerateRequest;
pub use generate_response::GenerateResponse;
pub use keep_alive_setting::KeepAliveSetting;
pub use list_response::ListResponse;
pub use logprob::Logprob;
pub use model_details::ModelDetails;
pub use model_options::ModelOptions;
pub use model_summary::ModelSummary;
pub use ps_response::PsResponse;
pub use running_model::RunningModel;
pub use show_model_details::ShowModelDetails;
pub use show_request::ShowRequest;
pub use show_response::ShowResponse;
pub use stop_setting::StopSetting;
pub use think_setting::ThinkSetting;
pub use token_logprob::TokenLogprob;
pub use version::VersionResponse;
