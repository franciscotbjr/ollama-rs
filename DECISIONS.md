# Decisions Log

This document tracks architectural and design decisions made during development.

| Date | Decision | Rationale | Reference |
|------|----------|-----------|-----------|
| 2026-01-11 | Single-crate architecture | Simpler build, easier maintenance, clearer dependencies | DEV_NOTES.md |
| 2026-01-13 | Use `thiserror` for error handling | Ergonomic error handling with derive macros | impl/01-get-version-implementation-plan.md |
| 2026-01-13 | Use `url` crate for URL validation | Proper URL parsing and validation at client creation | impl/01-get-version-implementation-plan.md |
| 2026-01-13 | Client uses `Arc<Client>` internally | Thread-safe cloning without overhead | impl/01-get-version-implementation-plan.md |
| 2026-01-13 | All error variants use `Error` suffix | Naming consistency (HttpError, TimeoutError, etc.) | impl/01-get-version-implementation-plan.md |
| 2026-01-13 | TDD approach for all features | Tests written before implementation | impl/01-get-version-implementation-plan.md |
| 2026-01-14 | Single-concern file structure | One primary type per file, mod.rs as facade | ARCHITECTURE.md |
| 2026-01-14 | Generic retry helpers | `get_with_retry<T>()` reduces 90% code duplication | impl/03-http-retry-abstraction-analysis.md |
| 2026-01-14 | `ClientConfig.url()` method | More ergonomic than standalone function | impl/02-api-endpoint-abstraction-analysis.md |
| 2026-01-14 | Unit tests in `tests/`, integration in `examples/` | `cargo test` always succeeds without external dependencies | ARCHITECTURE.md |
| 2026-01-15 | No retry on 4xx errors | Client errors should fail immediately | DEV_NOTES.md |
| 2026-01-16 | Exponential backoff: 100ms × (attempt + 1) | Balance between retry speed and server load | DEV_NOTES.md |
