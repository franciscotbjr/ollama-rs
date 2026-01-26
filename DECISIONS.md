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
| 2026-01-24 | Use `#[serde(untagged)]` for oneOf types | ThinkSetting, FormatSetting, KeepAliveSetting, StopSetting need flexible JSON representation | impl/10-post-generate-implementation-plan.md |
| 2026-01-24 | StopSetting integrated into ModelOptions | Stop sequences are model options, not request-level settings; consistent with Ollama API | impl/10-post-generate-implementation-plan.md |
| 2026-01-24 | GenerateRequest always sets `stream: false` | v0.1.0 is non-streaming only; streaming deferred to v0.2.0 | impl/10-post-generate-implementation-plan.md |
| 2026-01-24 | Helper methods on GenerateResponse | `text()`, `is_done()`, `tokens_per_second()` provide ergonomic access to common data | impl/10-post-generate-implementation-plan.md |
| 2026-01-24 | Ergonomic tools API behind `tools` feature flag | Keeps core library lightweight; `schemars`/`futures` deps are optional | impl/12-ergonomic-tools-api-proposal.md |
| 2026-01-25 | No `Chat` struct - use `Vec<ChatMessage>` | Library provides primitives, not app-level abstractions; users define their own entities | impl/11-post-chat-implementation-plan.md |
| 2026-01-25 | `ChatRequest::new()` accepts `IntoIterator<Item=ChatMessage>` | More flexible than `Into<Vec<...>>`; enables custom types via `IntoIterator` impl | impl/11-post-chat-implementation-plan.md |
| 2026-01-26 | Tool primitives stay in `src/primitives/`, not `src/tools/` | Semantic separation: `primitives/` = API data structures (ToolCall, ToolDefinition), `tools/` = ergonomic Rust abstractions (Tool trait, ToolRegistry). Consistent with other API types (ChatRequest, GenerateRequest). Feature flag gates correctly without relocation. | DECISIONS.md |
| 2026-01-26 | No doc tests - all tests in source files or `tests/` folder | Feature flag complexity makes doc tests hard to maintain; coverage already exists in unit tests; simpler workflow without doc test failures | DEV_NOTES.md, ARCHITECTURE.md |
| 2026-01-26 | Example naming: `<feature>_<variant>_<mode>.rs` | Clear feature identification; consistent async/sync suffix; grouped by prefix | DEV_NOTES.md |
| 2026-01-26 | `chat_with_tools_async` example with mock weather service | Demonstrates complete tool call flow; uses Open-Meteo API format as reference; shows type-safe Tool trait usage | examples/chat_with_tools_async.rs |
