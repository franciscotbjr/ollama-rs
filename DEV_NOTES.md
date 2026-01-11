# Development Notes

This document contains internal development notes, architectural decisions, and implementation details for ollama-rs.

## Project Status

**Current Version:** 0.1.0
**Status:** Early development / Foundation phase
**Last Updated:** 2025-01-10

## Architecture Overview

### Workspace Structure

The project uses a Cargo workspace to separate concerns and maintain modularity:

```
ollama-rs/
├── ollama-rs (root)   # Main integration crate
├── primitives/        # API data structures
├── http-core/         # HTTP client layer
├── conveniences/      # High-level API
└── samples/           # Examples
```

### Design Philosophy

1. **Layered Architecture**: Separation between low-level primitives and high-level conveniences
2. **Type Safety**: Leverage Rust's type system for API correctness
3. **Async First**: Built on Tokio for async/await support
4. **Minimal Dependencies**: Keep dependency tree lean
5. **OpenAPI Driven**: Follow Ollama's official API specification

## Current State

### Implemented
- Cargo workspace configuration
- Basic crate structure
- Dependency setup (tokio, serde, reqwest)
- OpenAPI specification integration
- Documentation foundation

### In Progress
- Core HTTP client implementation
- API primitive definitions
- Error handling strategy

### TODO
- [ ] Implement primitives based on OpenAPI spec
- [ ] Build HTTP client in http-core
- [ ] Create convenience layer APIs
- [ ] Add comprehensive tests
- [ ] Implement streaming support
- [ ] Add usage examples
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
- Features: rt (runtime)
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

### Phase 1: Primitives (Current)
Define all data structures matching Ollama API:
- Request types
- Response types
- Configuration options
- Error types

### Phase 2: HTTP Core
Implement HTTP client layer:
- Connection management
- Request/response handling
- Error mapping
- Retry logic (if needed)

### Phase 3: Conveniences
Build high-level APIs:
- Simplified method signatures
- Common workflows
- Builder patterns
- Streaming helpers

### Phase 4: Examples & Docs
Complete the developer experience:
- Comprehensive examples
- API documentation
- Integration guides
- Best practices

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
