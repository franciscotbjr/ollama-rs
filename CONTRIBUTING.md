# Contributing to ollama-rs

Thank you for your interest in contributing to ollama-rs! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Reporting Issues](#reporting-issues)

## Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please be respectful and professional in all interactions.

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally
3. Set up the development environment
4. Create a feature branch
5. Make your changes
6. Submit a pull request

## Development Setup

### Prerequisites

- Rust 1.75 or later (edition 2024)
- [Ollama](https://github.com/ollama) installed and running locally
- Git

### Initial Setup

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/ollama-rs.git
cd ollama-rs

# Build the project
cargo build

# Run tests
cargo test

# Run examples
cargo run --example basic_generation
```

## Project Structure

This is a single-crate project with modular organization:

```
ollama-rs/
├── src/
│   ├── lib.rs           # Main library entry point
│   ├── primitives/      # Low-level API primitives (default)
│   ├── http/            # HTTP client implementation (default)
│   └── conveniences/    # High-level APIs (optional feature)
├── examples/            # Usage examples
├── spec/                # OpenAPI specifications
└── .github/             # CI/CD workflows
```

### Module Responsibilities

- **primitives** (module): Data structures and types matching Ollama's API specification
- **http** (module): HTTP communication, request/response handling
- **conveniences** (module): Ergonomic, high-level APIs built on primitives (optional feature)

### Feature Flags

```toml
default = ["http", "primitives"]     # Core functionality
conveniences = ["http", "primitives"] # Optional high-level APIs
```

## Development Workflow

### Creating a Feature Branch

```bash
git checkout -b feature/your-feature-name
```

Use descriptive branch names:
- `feature/` for new features
- `fix/` for bug fixes
- `docs/` for documentation changes
- `refactor/` for code refactoring

### Making Changes

1. Write clear, concise code following Rust conventions
2. Add tests for new functionality
3. Update documentation as needed
4. Run tests and formatting checks

### Before Committing

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Run tests
cargo test

# Check documentation
cargo doc --no-deps
```

## Coding Standards

### Rust Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for consistent formatting
- Address all `cargo clippy` warnings
- Write idiomatic Rust code

### Code Organization

- Keep functions focused and small
- Use meaningful variable and function names
- Add doc comments for public APIs
- Group related functionality in modules

### Documentation

All public APIs must have documentation:

```rust
/// Brief description of what this does.
///
/// More detailed explanation if needed.
///
/// # Examples
///
/// ```
/// use ollama_rs::SomeType;
/// let example = SomeType::new();
/// ```
///
/// # Errors
///
/// Returns an error if...
pub fn example_function() -> Result<(), Error> {
    // Implementation
}
```

### Error Handling

- Use `Result<T, E>` for fallible operations
- Provide descriptive error messages
- Create custom error types when appropriate
- Document error conditions

## Testing

### Writing Tests

- Write unit tests for individual functions
- Write integration tests for API interactions
- Test error conditions and edge cases
- Mock external dependencies when appropriate

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Test implementation
    }

    #[tokio::test]
    async fn test_async_example() {
        // Async test implementation
    }
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific package
cargo test --package primitives

# With output
cargo test -- --nocapture

# Single test
cargo test test_name
```

## Submitting Changes

### Pull Request Process

1. Update documentation for any changed functionality
2. Add entries to CHANGELOG.md
3. Ensure all tests pass
4. Update README.md if needed
5. Push to your fork
6. Create a pull request

### Pull Request Guidelines

- Provide a clear description of changes
- Reference related issues (e.g., "Fixes #123")
- Keep changes focused and atomic
- Respond to review feedback promptly

### Commit Messages

Write clear, descriptive commit messages:

```
Brief summary (50 chars or less)

More detailed explanation if needed. Explain what
and why, not how. Wrap at 72 characters.

- Use bullet points for multiple items
- Reference issues: Fixes #123
```

## Reporting Issues

### Bug Reports

Include:
- Rust version (`rustc --version`)
- Ollama version
- Operating system
- Steps to reproduce
- Expected behavior
- Actual behavior
- Error messages/stack traces

### Feature Requests

Include:
- Use case description
- Proposed API/behavior
- Alternative solutions considered
- Willingness to implement

## Development Guidelines

### API Design Principles

1. **Consistency**: Follow existing patterns
2. **Ergonomics**: Make common tasks easy
3. **Safety**: Leverage Rust's type system
4. **Performance**: Avoid unnecessary allocations
5. **Clarity**: Prefer explicit over implicit

### Versioning

We follow [Semantic Versioning](https://semver.org/):
- MAJOR: Breaking changes
- MINOR: New features, backward compatible
- PATCH: Bug fixes, backward compatible

### Dependencies

- Minimize external dependencies
- Use well-maintained crates
- Justify new dependencies in PR description
- Keep workspace dependencies synchronized

## Questions?

If you have questions:
- Check existing documentation
- Search issues for similar questions
- Open a new issue with the "question" label
- Reach out to maintainers

Thank you for contributing to ollama-rs!
