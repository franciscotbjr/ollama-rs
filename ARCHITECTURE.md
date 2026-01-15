# Architecture Guide

This document describes the architectural principles and module organization of the `ollama-oxide` library.

---

## Design Principles

### 1. Single Concern Per File

Each file contains one primary type (struct, enum, or trait) with its implementations and tightly coupled helpers.

### 2. Module as Facade

The `mod.rs` file serves exclusively as a re-export facade:
- Module declarations (`mod foo;`)
- Public re-exports (`pub use foo::Foo;`)
- Module-level documentation

### 3. Explicit Over Implicit

- File names match their primary type (in snake_case)
- Public API is explicitly re-exported
- Internal details use `pub(super)` or `pub(crate)` visibility

---

## Module Organization Rule

```
module/
├── mod.rs          # Module declarations + re-exports only
├── foo.rs          # struct Foo + impl Foo + related items
└── bar.rs          # struct Bar + impl Bar + related items
```

**Example:**
```
http/
├── mod.rs          # pub use config::ClientConfig; pub use client::OllamaClient;
├── config.rs       # struct ClientConfig + impl Default
├── client.rs       # struct OllamaClient + constructors
├── api_async.rs    # trait OllamaApiAsync + impl
└── api_sync.rs     # trait OllamaApiSync + impl
```

---

## Current Structure

```
src/
├── lib.rs                          # Module declarations + re-exports + prelude
├── error.rs                        # Error enum + impl From + Result alias
├── primitives/
│   ├── mod.rs                      # Re-exports: VersionResponse
│   └── version.rs                  # VersionResponse struct
├── http/
│   ├── mod.rs                      # Re-exports: ClientConfig, OllamaClient, traits
│   ├── config.rs                   # ClientConfig + impl Default
│   ├── client.rs                   # OllamaClient + constructors + validation
│   ├── api_async.rs                # OllamaApiAsync trait + impl
│   └── api_sync.rs                 # OllamaApiSync trait + impl
└── conveniences/
    └── mod.rs                      # (Future: convenience APIs)
```

---

## Adding New Components

### New Primitive Type

1. Create `src/primitives/model_info.rs`:
   ```rust
   use serde::{Deserialize, Serialize};

   #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
   pub struct ModelInfo {
       pub name: String,
       pub size: u64,
   }
   ```

2. Update `src/primitives/mod.rs`:
   ```rust
   mod version;
   mod model_info;

   pub use version::VersionResponse;
   pub use model_info::ModelInfo;
   ```

### New HTTP Endpoint

Update `src/http/api_async.rs` and `src/http/api_sync.rs` with new methods.

### New Configuration Option

Add field to `ClientConfig` in `src/http/config.rs` and update `impl Default`.

---

## Design Patterns

### Visibility Control

```rust
pub struct OllamaClient {
    pub(super) config: ClientConfig,  // Visible in http module only
    pub(super) client: Arc<Client>,
}
```

Allows trait implementations in sibling files to access internals while keeping them private externally.

### Trait per Concern

Separate async and sync traits for clear separation of concerns.

---

## Testing Architecture

### Unit Tests (`tests/` folder)
All files in `tests/*.rs` must be **unit tests only**:
- Use `mockito` for HTTP mocking
- No external service dependencies
- Must pass without Ollama running
- Run with `cargo test`

### Integration Tests (`examples/` folder)
Integration tests are implemented as **examples**:
- Require running Ollama server
- Demonstrate real API usage
- Run with `cargo run --example <name>`

**Rule:** Never add tests to `tests/` folder that require external services.

---

## Migration Checklist

- [ ] Identify distinct components/types
- [ ] Create one file per component (`component_name.rs`)
- [ ] Move type definition + implementations to new file
- [ ] Update visibility (`pub(super)` for internal sharing)
- [ ] Clear out `mod.rs`, add `mod` declarations and re-exports
- [ ] Run `cargo build`, `cargo test`, `cargo clippy`

---

## Version History

- **2026-01-13**: Initial architecture document

---

## References

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Cargo Book - Package Layout](https://doc.rust-lang.org/cargo/guide/project-layout.html)
