# Contributing to csvmd

Thank you for your interest in contributing to csvmd! 🎉 This document provides guidelines and information to help you get started.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Environment](#development-environment)
- [Project Architecture](#project-architecture)
- [Development Workflow](#development-workflow)
- [Code Style and Quality](#code-style-and-quality)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Release Process](#release-process)
- [Getting Help](#getting-help)

## Getting Started

csvmd is a Rust CLI tool that converts CSV files to Markdown tables. It's designed for speed and efficiency, supporting both standard and streaming modes for handling large files.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- Git
- A text editor or IDE with Rust support

### Quick Setup

1. **Fork and clone the repository:**
   ```bash
   git clone https://github.com/YOUR_USERNAME/csvmd.git
   cd csvmd
   ```

2. **Build the project:**
   ```bash
   cargo build
   ```

3. **Run tests to ensure everything works:**
   ```bash
   cargo test
   ```

4. **Try the CLI tool:**
   ```bash
   # Create a sample CSV
   echo "Name,Age\nJohn,25\nJane,30" > sample.csv
   
   # Convert it to Markdown
   cargo run -- sample.csv
   ```

## Development Environment

### Recommended Tools

- **Rust Analyzer**: For IDE support
- **cargo-watch**: For continuous building/testing
  ```bash
  cargo install cargo-watch
  cargo watch -x test
  ```

### Essential Commands

```bash
# Build the project
cargo build

# Build optimized release version
cargo build --release

# Run the CLI tool
cargo run -- [OPTIONS] [FILE]

# Format code (ALWAYS run before committing)
cargo fmt

# Check formatting (ensure code is properly formatted)
cargo fmt --check

# Lint code
cargo clippy

# Run all tests (unit + integration)
cargo test

# Run only unit tests (in src/lib.rs)
cargo test --lib

# Run only integration tests
cargo test --test integration_tests

# Run specific test
cargo test test_csv_with_pipes

# Run with output
cargo test -- --nocapture
```

**Important**: Always run `cargo fmt` before making commits to ensure consistent code formatting.

## Project Architecture

csvmd follows a standard Rust library + binary pattern:

```
src/
├── lib.rs          # Core library with conversion logic
├── main.rs         # CLI interface using clap
└── error.rs        # Custom error types with thiserror

tests/
├── integration_tests.rs  # Full CLI functionality tests
└── edge_cases.rs         # Edge case and error condition tests
```

### Key Components

- **Core Library** (`src/lib.rs`): Contains two main functions:
  - `csv_to_markdown()`: Loads entire CSV into memory, suitable for smaller files
  - `csv_to_markdown_streaming()`: Two-pass streaming approach for large files
  
- **CLI Interface** (`src/main.rs`): Uses clap for argument parsing and handles input/output

- **Error Handling** (`src/error.rs`): Custom error types for CSV parsing, IO, and formatting errors

### Key Design Decisions

- Uses csv crate with flexible parsing to handle uneven column counts
- Escapes Markdown special characters: `|` → `\|`, `\n` → `<br>`
- Pre-allocates string capacity based on estimated output size
- Streaming mode uses two-pass approach to ensure correct table formatting

## Development Workflow

### Making Changes

1. **Create a feature branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following the coding standards

3. **Test your changes:**
   ```bash
   # Run formatting
   cargo fmt
   
   # Run linting
   cargo clippy
   
   # Run all tests
   cargo test
   ```

4. **Test the CLI manually:**
   ```bash
   # Test basic functionality
   echo "Name,Age\nJohn,25" | cargo run
   
   # Test with files
   cargo run -- test_data.csv
   
   # Test streaming mode
   cargo run -- --stream large_file.csv
   
   # Test different alignments
   cargo run -- --align center data.csv
   ```

### Code Changes Guidelines

- Make minimal modifications - change as few lines as possible
- Don't delete/remove working code unless absolutely necessary
- Always validate that changes don't break existing behavior
- Update documentation if directly related to your changes

## Code Style and Quality

### Formatting

- **Always run `cargo fmt` before committing**
- Use the default rustfmt configuration
- Code must pass `cargo fmt --check` in CI

### Linting

- Code must pass `cargo clippy` without warnings
- Follow Rust naming conventions and idioms
- Use meaningful variable and function names

### Documentation

- Add docstrings for public functions and types
- Include examples in documentation where helpful
- Update README.md if adding new features or changing CLI interface

## Testing

csvmd has a comprehensive testing strategy:

### Unit Tests

Located in `src/lib.rs`, these test core conversion logic:

```bash
cargo test --lib
```

Coverage includes:
- Basic CSV conversion
- Edge cases (empty cells, special characters, Unicode)
- Header alignment options
- Custom delimiters
- Error conditions

### Integration Tests

Located in `tests/integration_tests.rs`, these test full CLI functionality:

```bash
cargo test --test integration_tests
```

Coverage includes:
- Command-line argument parsing
- File input/output
- Stdin/stdout handling
- Error handling and reporting
- Cross-platform compatibility

### Edge Case Tests

Located in `tests/edge_cases.rs`, these test unusual inputs:

```bash
cargo test --test edge_cases
```

### Adding New Tests

When adding features:

1. **Add unit tests** for core logic in `src/lib.rs`
2. **Add integration tests** for CLI behavior in `tests/integration_tests.rs`
3. **Consider edge cases** and add tests in `tests/edge_cases.rs`

Example unit test:
```rust
#[test]
fn test_your_feature() {
    let csv_data = "Name,Age\nJohn,25";
    let input = Cursor::new(csv_data);
    let config = Config::default();
    let result = csv_to_markdown(input, config).unwrap();
    
    let expected = "| Name | Age |\n| --- | --- |\n| John | 25 |\n";
    assert_eq!(result, expected);
}
```

## Submitting Changes

### Pull Request Guidelines

1. **Ensure all tests pass:**
   ```bash
   cargo test
   cargo fmt --check
   cargo clippy
   ```

2. **Write a clear PR description:**
   - Explain what changes you made and why
   - Reference any related issues
   - Include examples if adding new features

3. **Keep PRs focused:**
   - One feature or fix per PR
   - Avoid mixing unrelated changes

4. **Update documentation** if your changes affect:
   - CLI interface
   - Public API
   - Installation or usage instructions

### Commit Messages

- Use clear, descriptive commit messages
- Start with a verb in present tense ("Add", "Fix", "Update")
- Reference issues when applicable ("Fixes #123")

### CI Requirements

Your PR must pass all CI checks:

- ✅ Tests pass on all platforms (Linux, macOS, Windows)
- ✅ Code is properly formatted (`cargo fmt --check`)
- ✅ No linting warnings (`cargo clippy`)
- ✅ Builds successfully in release mode

## Release Process

csvmd uses an automated release process:

1. **Version Tagging**: Releases are triggered by pushing tags in the format `vX.Y.Z`
2. **Cross-platform Builds**: CI automatically builds for multiple platforms
3. **Code Signing**: macOS binaries are signed and notarized
4. **Publishing**: Releases are published to both GitHub releases and crates.io

Contributors don't need to worry about releases - maintainers handle this process.

## Getting Help

### Documentation

- [README.md](README.md) - Usage instructions and examples
- [Rust Book](https://doc.rust-lang.org/book/) - Learn Rust
- [Clap Documentation](https://docs.rs/clap/) - CLI argument parsing
- [CSV Crate Documentation](https://docs.rs/csv/) - CSV parsing

### Communication

- **Issues**: Use GitHub issues for bug reports and feature requests
- **Discussions**: Use GitHub discussions for questions and general discussion
- **Security**: For security issues, please follow responsible disclosure

### Common Issues

**Build Issues:**
```bash
# Clean and rebuild
cargo clean
cargo build
```

**Test Failures:**
```bash
# Run specific test with output
cargo test test_name -- --nocapture

# Run tests one at a time
cargo test -- --test-threads=1
```

**Formatting Issues:**
```bash
# Auto-fix formatting
cargo fmt

# Check what would be changed
cargo fmt -- --check
```

---

Thank you for contributing to csvmd! Your contributions help make CSV-to-Markdown conversion better for everyone. 🚀