# Contributing

Thank you for your interest in contributing to csvmd! This document provides guidelines for contributing to this Rust CLI tool that converts CSV files to Markdown tables.

## Development Setup

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- Git

### Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/csvmd.git
   cd csvmd
   ```
3. Build the project:
   ```bash
   cargo build
   ```
4. Run tests to ensure everything works:
   ```bash
   cargo test
   ```

## Project Structure

The project follows a standard Rust library + binary pattern:

- **src/lib.rs**: Core library containing conversion logic with two main functions:
  - `csv_to_markdown()`: Loads entire CSV into memory, suitable for smaller files
  - `csv_to_markdown_streaming()`: Two-pass streaming approach for large files
- **src/main.rs**: CLI interface using clap for argument parsing
- **src/error.rs**: Custom error types with thiserror for CSV parsing, IO, and formatting errors
- **tests/**: Integration and edge case tests

## Development Workflow

### Code Formatting

**ALWAYS** run `cargo fmt` before making commits to ensure consistent code formatting:

```bash
# Format code
cargo fmt

# Check formatting without making changes
cargo fmt --check
```

### Linting

Run clippy to catch common mistakes and improve code quality:

```bash
cargo clippy
```

### Testing

We have comprehensive test coverage including unit tests, integration tests, and edge case tests:

```bash
# Run all tests
cargo test

# Run only unit tests (in src/lib.rs)
cargo test --lib

# Run only integration tests
cargo test --test integration_tests

# Run specific test
cargo test test_csv_with_pipes

# Run with output for debugging
cargo test -- --nocapture
```

### Building

```bash
# Build in development mode
cargo build

# Build optimized release version
cargo build --release

# Check compilation without building
cargo check
```

## Testing Guidelines

### Test Categories

1. **Unit Tests** (in src/lib.rs): Test core conversion logic, edge cases, and error conditions
2. **Integration Tests** (tests/integration_tests.rs): Test full CLI functionality using tempfiles and process spawning
3. **Edge Case Tests** (tests/edge_cases.rs): Test boundary conditions and malformed input

### Writing Tests

- Follow existing test patterns and naming conventions
- Include tests for both standard and streaming modes when applicable
- Test error conditions and edge cases
- Use `insta` for snapshot testing of CLI output
- Ensure tests are deterministic and don't depend on external resources

### Snapshot Tests

Some tests use `insta` for snapshot testing. If you modify output format:

```bash
# Review and approve snapshot changes
cargo insta review
```

## Pull Request Process

1. **Create a feature branch** from the main branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following the coding guidelines

3. **Run the full test suite**:
   ```bash
   cargo fmt --check
   cargo clippy
   cargo test
   ```

4. **Commit your changes** with clear, descriptive commit messages

5. **Push to your fork** and create a pull request

6. **Ensure CI passes** and address any feedback

### Commit Messages

- Use clear, descriptive commit messages
- Start with a verb in the imperative mood (e.g., "Add", "Fix", "Update")
- Keep the first line under 50 characters
- Add detailed explanation if needed in the body

## Code Style Guidelines

- Follow standard Rust conventions and idioms
- Use meaningful variable and function names
- Add documentation comments for public APIs
- Keep functions focused and reasonably sized
- Handle errors appropriately using the `Result` type

## Key Design Principles

- **Memory Efficiency**: Support both in-memory and streaming modes for different file sizes
- **Robust Parsing**: Handle various CSV complexities (quoted fields, embedded newlines, pipe characters)
- **Proper Escaping**: Escape Markdown special characters correctly
- **Comprehensive Error Handling**: Provide meaningful error messages with context

## CLI Testing

Test CLI functionality manually:

```bash
# Test basic functionality
echo "Name,Age\nJohn,25" | cargo run

# Test different options
cargo run -- --help
cargo run -- --align center test.csv
cargo run -- --stream large_file.csv
```

## Documentation

- Update README.md if adding new features or changing behavior
- Add doc comments for new public APIs
- Include examples in doc comments where helpful
- Keep documentation in sync with implementation

## Getting Help

- Check existing issues and discussions on GitHub
- Feel free to open an issue for questions or suggestions
- Follow the project's code of conduct in all interactions

## License

By contributing to csvmd, you agree that your contributions will be licensed under the MIT License.

