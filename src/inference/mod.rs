//! Inference types for Ollama API responses and requests
//!
//! This module contains all data types used for inference operations:
//! chat completions, text generation, and embeddings.

// Chat types
mod chat_message;
mod chat_request;
mod chat_response;
mod chat_role;
mod response_message;

// Inference types
mod embed_input;
mod embed_request;
mod embed_response;
mod format_setting;
mod generate_request;
mod generate_response;
mod keep_alive_setting;
mod logprob;
mod model_options;
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

// Inference types re-exports
pub use embed_input::EmbedInput;
pub use embed_request::EmbedRequest;
pub use embed_response::EmbedResponse;
pub use format_setting::FormatSetting;
pub use generate_request::GenerateRequest;
pub use generate_response::GenerateResponse;
pub use keep_alive_setting::KeepAliveSetting;
pub use logprob::Logprob;
pub use model_options::ModelOptions;
pub use stop_setting::StopSetting;
pub use think_setting::ThinkSetting;
pub use token_logprob::TokenLogprob;
pub use version::VersionResponse;
