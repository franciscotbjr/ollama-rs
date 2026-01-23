# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **POST /api/embed endpoint**: Generate text embeddings
  - `EmbedRequest` primitive type with builder pattern support
  - `EmbedResponse` primitive type with embeddings vector
  - `EmbedInput` enum for single text or batch input
  - `ModelOptions` primitive type for model parameters (temperature, top_k, top_p, etc.)
  - `embed()` async method
  - `embed_blocking()` sync method
  - 28 new unit tests with mocking
  - Examples: `embed_async.rs`, `embed_sync.rs`
- **POST /api/show endpoint**: Show detailed model information
  - `ShowRequest` primitive type with `new()` and `verbose()` constructors
  - `ShowResponse` primitive type with `has_capability()` helper method
  - `ShowModelDetails` primitive type for model metadata
  - `show_model()` async method
  - `show_model_blocking()` sync method
  - POST helper methods: `post_with_retry()`, `post_blocking_with_retry()`
  - 26 new unit tests with mocking
  - Examples: `show_model_async.rs`, `show_model_sync.rs`
- **DELETE /api/delete endpoint**: Delete a model from the server
  - `DeleteRequest` primitive type with `model` field
  - `delete_model()` async method
  - `delete_model_blocking()` sync method
  - DELETE helper methods: `delete_empty_with_retry()`, `delete_empty_blocking_with_retry()`
  - 10 new unit tests with mocking
  - Examples: `delete_model_async.rs`, `delete_model_sync.rs`
- **POST /api/copy endpoint**: Copy/duplicate a model
  - `CopyRequest` primitive type with `source` and `destination` fields
  - `copy_model()` async method
  - `copy_model_blocking()` sync method
  - `HttpStatusError(u16)` error variant for HTTP status code errors
  - POST helper methods: `post_empty_with_retry()`, `post_empty_blocking_with_retry()`
  - 16 new unit tests with mocking
  - Examples: `copy_model_async.rs`, `copy_model_sync.rs`
- **GET /api/ps endpoint**: List currently running models
  - `PsResponse`, `RunningModel` primitive types
  - `list_running_models()` async method
  - `list_running_models_blocking()` sync method
  - 13 new unit tests with mocking
  - Examples: `list_running_models_async.rs`, `list_running_models_sync.rs`
- **GET /api/tags endpoint**: List locally available models
  - `ListResponse`, `ModelSummary`, `ModelDetails` primitive types
  - `list_models()` async method
  - `list_models_blocking()` sync method
  - Comprehensive test coverage (33 new tests)
- **Examples for GET /api/version**:
  - `get_version_async.rs` - Async usage example
  - `get_version_sync.rs` - Sync/blocking usage example
  - `get_version_custom.rs` - Custom configuration example
- **Examples for GET /api/tags**:
  - `list_models_async.rs` - Async list models example
  - `list_models_sync.rs` - Sync list models example

### Changed
- **Versioning strategy update**: All 12 endpoints will be implemented in v0.1.0 (non-streaming mode)
  - Streaming features moved to v0.2.0
  - v0.3.0 will focus on conveniences module
  - v0.4.0 will focus on examples and production readiness

### Planned for v0.1.0
- POST /api/generate - Text generation (non-streaming only)
- POST /api/chat - Chat completions (non-streaming only)
- POST /api/create - Model creation (non-streaming only)
- POST /api/pull - Model download (non-streaming only)
- POST /api/push - Model upload (non-streaming only)

### Planned for v0.2.0
- Streaming support for generate, chat, create, pull, push endpoints

## [0.1.0] - 2025-01-10

### Added
- Initial project structure and Cargo workspace setup
- Core crate structure (primitives, http-core, conveniences, samples)
- Basic project documentation (README, LICENSE, CONTRIBUTING)
- OpenAPI specification for Ollama API (v0.1.0)
- Project build configuration and CI/CD workflows

### Notes
- This is the initial release establishing the project foundation
- API is not yet stable and may change significantly

[Unreleased]: https://github.com/franciscotbjr/ollama-oxide/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/franciscotbjr/ollama-oxide/releases/tag/v0.1.0
