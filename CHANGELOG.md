# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
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
