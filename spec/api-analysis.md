# Ollama API Specification Analysis

**Document Version:** 1.0
**Analysis Date:** 2025-01-11
**Total Endpoints:** 12

## Overview

This document provides a comprehensive analysis of all Ollama API endpoints based on the OpenAPI specifications located in [spec/primitives/](primitives/). Each endpoint has been analyzed for its purpose, complexity, and implementation requirements.

## API Endpoints Summary

| # | Endpoint | Method | Operation | Complexity | Streaming | Phase |
|---|----------|--------|-----------|------------|-----------|-------|
| 01 | `/api/generate` | POST | Text generation | High | Yes | 2 |
| 02 | `/api/chat` | POST | Chat conversation | High | Yes | 2 |
| 03 | `/api/embed` | POST | Generate embeddings | Medium | No | 2 |
| 04 | `/api/tags` | GET | List models | Low | No | 2 |
| 05 | `/api/ps` | GET | List running models | Low | No | 2 |
| 06 | `/api/show` | POST | Show model details | Medium | No | 2 |
| 07 | `/api/create` | POST | Create model | High | Yes | 2 |
| 08 | `/api/copy` | POST | Copy model | Low | No | 2 |
| 09 | `/api/pull` | POST | Pull model | High | Yes | 2 |
| 10 | `/api/push` | POST | Push model | High | Yes | 2 |
| 11 | `/api/delete` | DELETE | Delete model | Low | No | 2 |
| 12 | `/api/version` | GET | Get version | Low | No | 2 |

## Detailed Endpoint Analysis

### 01. Generate (`POST /api/generate`)

**Specification:** [01-ollama_api_generate.yaml](primitives/01-ollama_api_generate.yaml)

**Purpose:** Generates text responses from prompts using a specified model.

**Key Features:**
- Streaming and non-streaming modes
- Image input support (multimodal)
- Structured output format (JSON schema support)
- Thinking mode (reasoning traces)
- Log probabilities support

**Request Schema:**
```rust
struct GenerateRequest {
    model: String,
    prompt: Option<String>,
    suffix: Option<String>,
    images: Option<Vec<String>>,  // Base64-encoded
    format: Option<Format>,  // String or JSON schema
    system: Option<String>,
    stream: Option<bool>,  // Default: true
    think: Option<Think>,  // bool or "high"/"medium"/"low"
    raw: Option<bool>,
    keep_alive: Option<KeepAlive>,  // String or number
    options: Option<ModelOptions>,
    logprobs: Option<bool>,
    top_logprobs: Option<i32>,
}
```

**Response Schemas:**
- `GenerateResponse` - Complete response
- `GenerateStreamEvent` - Stream chunks

**Complexity Factors:**
- Multiple response formats
- Streaming support
- Complex nested types (ModelOptions, Logprob)
- Image handling

**Implementation Priority:** High (core functionality)

---

### 02. Chat (`POST /api/chat`)

**Specification:** [02-ollama_api_chat.yaml](primitives/02-ollama_api_chat.yaml)

**Purpose:** Generate chat messages in conversational context.

**Key Features:**
- Conversation history management
- Tool/function calling support
- Structured output format
- Thinking mode
- Image support in messages

**Request Schema:**
```rust
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    tools: Option<Vec<ToolDefinition>>,
    format: Option<Format>,
    options: Option<ModelOptions>,
    stream: Option<bool>,
    think: Option<Think>,
    keep_alive: Option<KeepAlive>,
    logprobs: Option<bool>,
    top_logprobs: Option<i32>,
}

struct ChatMessage {
    role: Role,  // system, user, assistant, tool
    content: String,
    images: Option<Vec<String>>,
    tool_calls: Option<Vec<ToolCall>>,
}
```

**Response Schemas:**
- `ChatResponse` - Complete response with assistant message
- `ChatStreamEvent` - Stream chunks

**Complexity Factors:**
- Message history tracking
- Tool/function calling integration
- Multiple role types
- Nested complex structures

**Implementation Priority:** High (core functionality)

---

### 03. Embed (`POST /api/embed`)

**Specification:** [03-ollama_api_embed.yaml](primitives/03-ollama_api_embed.yaml)

**Purpose:** Generate vector embeddings for text inputs.

**Key Features:**
- Single or batch text input
- Configurable dimensions
- Truncation control

**Request Schema:**
```rust
struct EmbedRequest {
    model: String,
    input: Input,  // String or Vec<String>
    truncate: Option<bool>,  // Default: true
    dimensions: Option<i32>,
    keep_alive: Option<String>,
    options: Option<ModelOptions>,
}
```

**Response Schema:**
```rust
struct EmbedResponse {
    model: String,
    embeddings: Vec<Vec<f64>>,  // 2D array of floats
    total_duration: i64,
    load_duration: i64,
    prompt_eval_count: i32,
}
```

**Complexity Factors:**
- Batch processing
- Large numeric arrays
- Performance considerations

**Implementation Priority:** Medium (specialized use case)

---

### 04. List Models (`GET /api/tags`)

**Specification:** [04-ollama_api_list_models.yaml](primitives/04-ollama_api_list_models.yaml)

**Purpose:** Retrieve list of available models.

**Key Features:**
- Simple GET request
- Model metadata
- Size and digest information

**Response Schema:**
```rust
struct ListResponse {
    models: Vec<ModelSummary>,
}

struct ModelSummary {
    name: String,
    modified_at: String,  // ISO 8601
    size: i64,
    digest: String,  // SHA256
    details: ModelDetails,
}
```

**Complexity Factors:**
- Simple structure
- Straightforward implementation

**Implementation Priority:** High (utility)

---

### 05. List Running Models (`GET /api/ps`)

**Specification:** [05-ollama_api_list_running_models.yaml](primitives/05-ollama_api_list_running_models.yaml)

**Purpose:** Get currently loaded models in memory.

**Key Features:**
- Runtime status information
- Memory usage (VRAM)
- Expiration times

**Response Schema:**
```rust
struct PsResponse {
    models: Vec<Ps>,
}

struct Ps {
    model: String,
    size: i64,
    digest: String,
    details: Object,  // Flexible structure
    expires_at: String,
    size_vram: i64,
    context_length: i32,
}
```

**Complexity Factors:**
- Simple GET request
- Runtime state tracking

**Implementation Priority:** Medium (monitoring)

---

### 06. Show Model (`POST /api/show`)

**Specification:** [06-ollama_api_show_running_models.yaml](primitives/06-ollama_api_show_running_models.yaml)

**Purpose:** Get detailed information about a specific model.

**Key Features:**
- Comprehensive model metadata
- Architecture details
- Tokenizer configuration
- License information

**Request Schema:**
```rust
struct ShowRequest {
    model: String,
    verbose: Option<bool>,
}
```

**Response Schema:**
```rust
struct ShowResponse {
    parameters: String,  // Text format
    license: String,
    modified_at: String,
    details: Object,
    template: Option<String>,
    capabilities: Vec<String>,
    model_info: HashMap<String, Value>,  // Flexible key-value
}
```

**Complexity Factors:**
- Large response payload
- Flexible schema (model_info)
- Verbose mode handling

**Implementation Priority:** Medium (introspection)

---

### 07. Create Model (`POST /api/create`)

**Specification:** [07-ollama_api_create_model.yaml](primitives/07-ollama_api_create_model.yaml)

**Purpose:** Create custom models from existing ones.

**Key Features:**
- Model derivation
- Custom system prompts
- Quantization options
- Streaming status updates

**Request Schema:**
```rust
struct CreateRequest {
    model: String,
    from: Option<String>,
    template: Option<String>,
    license: Option<License>,  // String or Vec<String>
    system: Option<String>,
    parameters: Option<Object>,
    messages: Option<Vec<ChatMessage>>,
    quantize: Option<String>,
    stream: Option<bool>,  // Default: true
}
```

**Response Schemas:**
- `StatusResponse` - Final status
- `StatusEvent` - Stream events with progress

**Complexity Factors:**
- Long-running operation
- Progress tracking
- Streaming status updates

**Implementation Priority:** Medium (advanced feature)

---

### 08. Copy Model (`POST /api/copy`)

**Specification:** [08-ollama_api_copy_model.yaml](primitives/08-ollama_api_copy_model.yaml)

**Purpose:** Duplicate an existing model with a new name.

**Key Features:**
- Simple copy operation
- Immediate completion

**Request Schema:**
```rust
struct CopyRequest {
    source: String,
    destination: String,
}
```

**Complexity Factors:**
- Simple request/response
- No streaming

**Implementation Priority:** Low (utility)

---

### 09. Pull Model (`POST /api/pull`)

**Specification:** [09-ollama__api_pull_model.yaml](primitives/09-ollama__api_pull_model.yaml)

**Purpose:** Download models from remote registry.

**Key Features:**
- Streaming progress updates
- Insecure connection option
- Transfer status tracking

**Request Schema:**
```rust
struct PullRequest {
    model: String,
    insecure: Option<bool>,
    stream: Option<bool>,  // Default: true
}
```

**Response Schemas:**
- `StatusResponse` - Final status
- `StatusEvent` - Stream events with download progress

**Complexity Factors:**
- Long-running operation
- Large downloads
- Progress tracking
- Network error handling

**Implementation Priority:** High (model acquisition)

---

### 10. Push Model (`POST /api/push`)

**Specification:** [10-ollama_api_push_model.yaml](primitives/10-ollama_api_push_model.yaml)

**Purpose:** Upload models to remote registry.

**Key Features:**
- Streaming progress updates
- Insecure connection option
- Upload status tracking

**Request Schema:**
```rust
struct PushRequest {
    model: String,
    insecure: Option<bool>,
    stream: Option<bool>,  // Default: true
}
```

**Response Schemas:**
- `StatusResponse` - Final status
- `StatusEvent` - Stream events with upload progress

**Complexity Factors:**
- Long-running operation
- Large uploads
- Progress tracking
- Network error handling

**Implementation Priority:** Medium (publishing)

---

### 11. Delete Model (`DELETE /api/delete`)

**Specification:** [11-ollama_api_delete_model.yaml](primitives/11-ollama_api_delete_model.yaml)

**Purpose:** Remove a model from local storage.

**Key Features:**
- Simple deletion
- Immediate completion

**Request Schema:**
```rust
struct DeleteRequest {
    model: String,
}
```

**Complexity Factors:**
- Simple DELETE request
- No streaming
- Error handling for missing models

**Implementation Priority:** Medium (cleanup)

---

### 12. Get Version (`GET /api/version`)

**Specification:** [12-ollama_api_get_version.yaml](primitives/12-ollama_api_get_version.yaml)

**Purpose:** Retrieve Ollama server version.

**Key Features:**
- Simple version check
- Health check capability

**Response Schema:**
```rust
struct VersionResponse {
    version: String,
}
```

**Complexity Factors:**
- Simplest endpoint
- No request body

**Implementation Priority:** High (compatibility check)

---

## Shared Components Analysis

### ModelOptions

Used across multiple endpoints (generate, chat, embed, create).

```rust
struct ModelOptions {
    seed: Option<i32>,
    temperature: Option<f32>,
    top_k: Option<i32>,
    top_p: Option<f32>,
    min_p: Option<f32>,
    stop: Option<Stop>,  // String or Vec<String>
    num_ctx: Option<i32>,
    num_predict: Option<i32>,
    // additionalProperties: true - allows custom fields
}
```

**Considerations:**
- Flexible schema (additionalProperties)
- Needs HashMap support for unknown fields

### Logprob & TokenLogprob

Used in generate and chat endpoints.

```rust
struct Logprob {
    token: String,
    logprob: f64,
    bytes: Vec<i32>,
    top_logprobs: Vec<TokenLogprob>,
}

struct TokenLogprob {
    token: String,
    logprob: f64,
    bytes: Vec<i32>,
}
```

### StatusEvent

Used in create, pull, push endpoints for progress tracking.

```rust
struct StatusEvent {
    status: String,
    digest: Option<String>,
    total: Option<i64>,
    completed: Option<i64>,
}
```

### ToolDefinition & ToolCall

Used in chat endpoint for function calling.

```rust
struct ToolDefinition {
    type: String,  // "function"
    function: FunctionDefinition,
}

struct FunctionDefinition {
    name: String,
    description: Option<String>,
    parameters: Object,  // JSON Schema
}

struct ToolCall {
    function: FunctionCall,
}

struct FunctionCall {
    name: String,
    description: Option<String>,
    arguments: Object,
}
```

## Implementation Groupings

### Group 1: Simple GET Requests
- `/api/tags` (List Models)
- `/api/ps` (List Running)
- `/api/version` (Get Version)

**Characteristics:**
- No request body
- Simple responses
- No streaming
- Good starting points

### Group 2: Simple POST Requests
- `/api/copy` (Copy Model)
- `/api/delete` (Delete Model)

**Characteristics:**
- Simple request bodies
- No streaming
- Quick operations

### Group 3: Complex POST Requests (Non-Streaming)
- `/api/embed` (Generate Embeddings)
- `/api/show` (Show Model)

**Characteristics:**
- Moderate complexity
- Larger responses
- No streaming

### Group 4: Complex POST Requests (With Streaming)
- `/api/generate` (Generate Text)
- `/api/chat` (Chat Completion)
- `/api/create` (Create Model)
- `/api/pull` (Pull Model)
- `/api/push` (Push Model)

**Characteristics:**
- High complexity
- Streaming support required
- Long-running operations
- Progress tracking

## Streaming Implementation Notes

**Endpoints with Streaming:**
- `/api/generate` - Text generation chunks
- `/api/chat` - Chat message chunks
- `/api/create` - Creation progress
- `/api/pull` - Download progress
- `/api/push` - Upload progress

**Content-Type:**
- Non-streaming: `application/json`
- Streaming: `application/x-ndjson` (newline-delimited JSON)

**Implementation Requirements:**
- Async stream handling (tokio streams)
- Line-by-line JSON parsing
- Progress callback support
- Cancellation support

## Type System Considerations

### Enum Types

```rust
enum Role {
    System,
    User,
    Assistant,
    Tool,
}

enum Think {
    Bool(bool),
    Level(ThinkLevel),
}

enum ThinkLevel {
    High,
    Medium,
    Low,
}

enum Format {
    Json,
    Schema(Value),  // JSON Schema object
}

enum KeepAlive {
    Duration(String),  // "5m"
    Seconds(i64),
}
```

### Union Types (oneOf)

Several fields accept multiple types:
- `format`: String ("json") or Object (schema)
- `think`: Boolean or String (level)
- `keep_alive`: String or Number
- `stop`: String or Array
- `input`: String or Array
- `license`: String or Array

**Implementation Strategy:**
- Use Rust enums with serde's untagged feature
- Provide builder methods for ergonomics

## Error Handling Strategy

**HTTP Status Codes:**
- 200: Success
- 4xx: Client errors (invalid request)
- 5xx: Server errors (Ollama failures)

**Error Response Schema:**
```rust
struct ErrorResponse {
    error: String,
}
```

**Common Error Scenarios:**
- Model not found
- Invalid parameters
- Out of memory
- Network failures (pull/push)
- Streaming interruptions

## Testing Strategy per Endpoint

### Unit Tests
- Request/response serialization
- Enum variants
- Optional fields
- Error cases

### Integration Tests
- Actual API calls (requires Ollama)
- Streaming behavior
- Large payloads
- Error scenarios

### Mock Tests
- HTTP mocking for unit tests
- Predefined responses
- Error simulation

## Performance Considerations

### Memory
- Streaming for large responses
- Avoid loading entire response in memory
- Vector embedding arrays can be large

### Network
- Connection pooling (reqwest)
- Timeout configuration
- Retry logic for transient failures

### Concurrency
- Multiple simultaneous requests
- Stream cancellation
- Resource cleanup

## Recommended Implementation Order

### Phase 1 (v0.1.0): Foundation + HTTP Core
1. Set up primitives crate structure
2. Implement shared types (ModelOptions, Logprob, etc.)
3. Build HTTP client in http-core
4. Implement error types
5. Add basic integration tests

**Deliverable:** Working HTTP layer with type definitions

### Phase 2 (v0.2.0): All Primitives
Implement in this order:

**Week 1-2: Simple Endpoints**
1. GET `/api/version` (simplest)
2. GET `/api/tags`
3. GET `/api/ps`
4. POST `/api/copy`
5. DELETE `/api/delete`

**Week 3-4: Medium Complexity**
6. POST `/api/show`
7. POST `/api/embed`

**Week 5-7: High Complexity (Streaming)**
8. POST `/api/generate`
9. POST `/api/chat`

**Week 8-9: Model Management (Streaming)**
10. POST `/api/pull`
11. POST `/api/push`
12. POST `/api/create`

**Deliverable:** Complete primitive implementations

### Phase 3 (v0.3.0): Conveniences
Build high-level APIs on top of primitives:
- Client builder pattern
- Simplified method signatures
- Stream helpers
- Response iterators

**Deliverable:** Ergonomic convenience layer

### Phase 4 (v1.0.0): Examples & Polish
- Comprehensive examples
- Documentation
- Performance optimization
- Stability improvements

---

## Summary Statistics

- **Total Endpoints:** 12
- **GET Requests:** 3
- **POST Requests:** 8
- **DELETE Requests:** 1
- **Streaming Support:** 5 endpoints
- **Simple Endpoints:** 5
- **Medium Complexity:** 2
- **High Complexity:** 5

## Conclusion

The Ollama API provides a comprehensive interface for model interaction, management, and deployment. The implementation strategy focuses on:

1. Building a solid foundation (primitives + HTTP core)
2. Implementing endpoints in order of complexity
3. Ensuring streaming support for long-running operations
4. Providing ergonomic high-level APIs

This structured approach will result in a robust, maintainable Rust library for Ollama integration.
