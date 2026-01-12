# Development Notes

This document contains internal development notes, architectural decisions, and implementation details for ollama-oxide.

## Project Status

**Current Version:** 0.1.0
**Status:** Early development / Foundation phase
**Last Updated:** 2025-01-10

## Architecture Overview

### Single-Crate Structure

**Decision Date:** 2026-01-11

The project uses a **single-crate architecture** with modular organization:

```
ollama-oxide/
└── src/
    ├── lib.rs           # Main library entry point
    ├── primitives/      # API data structures (default feature)
    ├── http/            # HTTP client layer (default feature)
    └── conveniences/    # High-level API (optional feature)
```

**Rationale:**
- **Simpler build process**: No workspace complexity
- **Easier maintenance**: Single version, single release
- **Clearer dependencies**: Feature flags instead of crate dependencies
- **Better for library users**: One dependency instead of multiple
- **Faster compile times**: No cross-crate boundaries

**Changed From:** Previously planned as 5-crate workspace (ollama-oxide, primitives, http-core, conveniences, samples)

### Design Philosophy

1. **Modular Design**: Clear separation via modules and feature flags
2. **Type Safety**: Leverage Rust's type system for API correctness
3. **Async First**: Built on Tokio for async/await support
4. **Minimal Dependencies**: Keep dependency tree lean
5. **OpenAPI Driven**: Follow Ollama's official API specification
6. **Feature-Based**: Optional functionality via Cargo features

### Feature Flags

```toml
[features]
default = ["http", "primitives"]
conveniences = ["http", "primitives"]
http = []
primitives = []
```

## Current State

### Implemented
- Single-crate configuration with feature flags
- Module structure (primitives, http, conveniences)
- Dependency setup (tokio, serde, reqwest, async-trait)
- 12 OpenAPI specifications documented
- Comprehensive documentation foundation

### In Progress
- `http` module implementation
- `primitives` module type definitions
- Error handling strategy
- First endpoint: GET /api/version

### TODO
- [ ] Implement shared types in `primitives` module
- [ ] Build HTTP client in `http` module
- [ ] Implement GET /api/version endpoint
- [ ] Create `conveniences` module (Phase 3)
- [ ] Add comprehensive tests
- [ ] Implement streaming support
- [ ] Add examples in `/examples` directory
- [ ] Performance benchmarks

## Technical Decisions

### HTTP Client: reqwest

**Rationale:**
- Mature, well-maintained async HTTP client
- Excellent tokio integration
- Built-in JSON support via serde
- HTTP/2 support
- Cookie handling

**Configuration:**
- Features: blocking, cookies, http2, json, native-tls
- Version: 0.13.1

### Async Runtime: Tokio

**Rationale:**
- Industry standard for async Rust
- Rich ecosystem
- Excellent performance
- Well-documented

**Configuration:**
- Features: macros, rt-multi-thread
- Version: 1.49.0

### Serialization: Serde

**Rationale:**
- De facto standard for Rust serialization
- Excellent derive macro support
- Comprehensive JSON support

**Configuration:**
- serde: 1.0.228
- serde_json: 1.0.149

## API Implementation Strategy

### Phase 1 (v0.1.0): Foundation + HTTP Module (Current)
Set up `primitives` and `http` modules:
- Define shared types (ModelOptions, Logprob, enums)
- Implement HTTP client in `http` module
- Create error type hierarchy
- First endpoint: GET /api/version
- Feature flags working

### Phase 2 (v0.1.1): All Primitives
Complete all 12 endpoints in `primitives` module:
- 5 Simple endpoints
- 2 Medium complexity endpoints
- 5 Complex endpoints with streaming
- Full test coverage

### Phase 3 (v0.2.0): Conveniences Module
Build high-level APIs in `conveniences` module:
- Optional feature flag
- Simplified method signatures
- Builder patterns
- Stream helpers
- Common workflows

### Phase 4 (v0.3.0): Examples & Production
Polish and prepare for v1.0.0:
- Comprehensive examples in `/examples`
- API documentation complete
- Performance benchmarks
- Production guides

## OpenAPI Specification

Location: `spec/alloma_api.yaml`

Current endpoints to implement:
- `/api/generate` - Text generation (streaming & non-streaming)

Future endpoints:
- Model management
- Embeddings
- Additional completion modes

## Testing Strategy

### Unit Tests
- Individual function validation
- Data structure serialization
- Error handling paths

### Integration Tests
- Full API interactions (requires Ollama running)
- End-to-end workflows
- Error scenarios

### Mocking Strategy
- Mock HTTP responses for unit tests
- Real Ollama instance for integration tests
- Consider wiremock or similar for HTTP mocking

## Performance Considerations

### Current Focus
- Correctness and usability over premature optimization
- Establish baseline performance metrics

### Future Optimization Areas
- Connection pooling
- Request batching
- Response streaming efficiency
- Memory allocations

## Known Issues / Limitations

- Project in early development stage
- API not yet stable
- Limited endpoint coverage
- No production usage yet

## Development Environment

### Recommended Tools
- rust-analyzer (LSP)
- cargo-watch (auto-rebuild)
- cargo-expand (macro debugging)
- cargo-audit (security)

### Testing with Ollama
Ensure Ollama is running locally:
```bash
ollama serve
```

Default endpoint: `http://localhost:11434`

## Code Style Guidelines

### Naming Conventions
- Types: `PascalCase`
- Functions: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Modules: `snake_case`

### Module Organization
```rust
// Public API at module root
pub use self::client::Client;
pub use self::error::Error;

// Private implementation details in submodules
mod client;
mod error;
```

### Error Handling
Use `thiserror` for error types (to be added):
```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error: {message}")]
    Api { message: String },
}
```

## Future Considerations

### Potential Features
- CLI tool for Ollama interaction
- Server-side events for streaming
- Batch processing utilities
- Model management helpers
- Custom middleware/interceptors

### Breaking Changes Policy
- Document all breaking changes in CHANGELOG
- Follow semantic versioning strictly
- Provide migration guides for major versions

## Resources

- [Ollama GitHub](https://github.com/ollama)
- [Ollama API Docs](https://github.com/ollama/ollama/blob/main/docs/api.md)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Async Book](https://rust-lang.github.io/async-book/)

## Questions & Decisions Log

### Q: Should we support sync and async APIs?
**Decision:** Start with async-first. Add sync wrapper if there's demand.
**Rationale:** Ollama API is inherently I/O bound, async is more natural.

### Q: How to handle streaming responses?
**Decision:** TBD - evaluate tokio streams vs custom iterator
**Status:** Under investigation

### Q: Error handling strategy?
**Decision:** Use Result with custom Error enum
**Status:** To be implemented

---

**Note:** This document is for internal development reference. Update as the project evolves.
