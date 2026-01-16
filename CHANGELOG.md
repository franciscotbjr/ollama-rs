# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
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

### Planned
- Complete implementation of remaining API endpoints
- Implement streaming support for generation endpoints

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
