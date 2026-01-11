# ollama-rs

<div align="center">
  <img src="assets/logo.svg" alt="ollama-rs Logo" width="1200" height="300">
</div>
<br>

This is the Llama in the crate, a Rust library providing low-level primitives and high-level conveniences for integrating with [Ollama](https://github.com/ollama)'s native API.

<div align="center">
  <img src="assets/llama-in-the-crate.png" alt="Llama in the crate" width="300" height="300">
</div>

## Features

- Low-level primitives for direct Ollama API interaction
- High-level convenience methods for common use cases
- Async/await support with Tokio runtime
- Type-safe API bindings
- Comprehensive error handling
- HTTP/2 support

## Features

This project is organized as a Cargo workspace with the following features:

- **ollama-rs** - Main library integrating all components
- **primitives** - Low-level primitives for Ollama's native API
- **http-core** - Core HTTP implementation for API communication
- **conveniences** - Higher-level abstractions for common workflows
- **samples** - Example usage and integration patterns

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ollama-rs = "0.1.0"
```

## Quick Start

```rust
#[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    todo!("Working ");
}
```

## Requirements

- Rust 1.75+ (edition 2024)
- [Ollama](https://github.com/ollama) running locally or accessible via network

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running Examples

```bash
cargo run --package samples
```

## API Documentation

The library follows Ollama's OpenAPI specification (see [spec/alloma_api.yaml](spec/alloma_api.yaml)).

Supported endpoints:
- `/api/generate` - Generate text completions
- Additional endpoints coming soon

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

Based on [Ollama's](https://github.com/ollama) official libraries and API specifications.

## Links

- [Repository](https://github.com/franciscotbjr/ollama-rs)
- [Ollama Documentation](https://github.com/ollama)
- [Issue Tracker](https://github.com/franciscotbjr/ollama-rs/issues)

