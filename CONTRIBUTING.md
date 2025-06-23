# Contributing to csvmd

Thank you for your interest in contributing to csvmd! This guide will help you get started with development and understand the project's workflow.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- Git

### Development Setup

1. **Fork and clone the repository**:
   ```bash
   git clone https://github.com/your-username/csvmd.git
   cd csvmd
   ```

2. **Build the project**:
   ```bash
   cargo build
   ```

3. **Run the tests to ensure everything works**:
   ```bash
   cargo test
   ```

4. **Try running the CLI tool**:
   ```bash
   cargo run -- --help
   ```

## Project Structure

csvmd follows a standard Rust library + binary pattern:

- **`src/lib.rs`**: Core library with conversion logic
  - `csv_to_markdown()`: Standard mode (loads entire CSV into memory)
  - `csv_to_markdown_streaming()`: Streaming mode for large files
- **`src/main.rs`**: CLI interface using clap for argument parsing
- **`src/error.rs`**: Custom error types using thiserror
- **`tests/`**: Integration and edge case tests
  - `tests/integration_tests.rs`: Full CLI functionality tests
  - `tests/edge_cases.rs`: Edge case and error condition tests

## Development Workflow

### Before Making Changes

1. **Create a new branch** for your feature or fix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Check the current state** of the project:
   ```bash
   cargo fmt --check  # Verify formatting
   cargo clippy       # Run linter
   cargo test --all   # Run all tests
   ```

### Making Changes

1. **Format your code** (REQUIRED before committing):
   ```bash
   cargo fmt
   ```

2. **Run clippy** to catch common issues:
   ```bash
   cargo clippy
   ```

3. **Run tests frequently** during development:
   ```bash
   cargo test
   ```

### Testing

The project has comprehensive test coverage:

```bash
# Run all tests
cargo test

# Run only unit tests (in src/lib.rs)
cargo test --lib

# Run only integration tests
cargo test --test integration_tests

# Run specific test
cargo test test_csv_with_pipes

# Run tests with output
cargo test -- --nocapture
```

**Test Types:**
- **Unit tests**: Located in `src/lib.rs`, test core conversion logic
- **Integration tests**: Test full CLI functionality using temporary files
- **Edge case tests**: Test error conditions and unusual inputs

### Adding Tests

When adding new features or fixing bugs:

1. **Add unit tests** for core library changes in `src/lib.rs`
2. **Add integration tests** for CLI changes in `tests/integration_tests.rs`
3. **Consider edge cases** and add tests in `tests/edge_cases.rs`

Example unit test:
```rust
#[test]
fn test_your_feature() {
    let csv_data = "Name,Age\nJohn,25";
    let input = Cursor::new(csv_data);
    let config = Config::default();
    let result = csv_to_markdown(input, config).unwrap();
    assert_eq!(result, "| Name | Age |\n| --- | --- |\n| John | 25 |\n");
}
```

## Code Style

### Formatting (Required)

- **Always run `cargo fmt`** before committing
- The CI will fail if code is not properly formatted
- Use `cargo fmt --check` to verify formatting without making changes

### Linting

- Run `cargo clippy` to catch common issues
- Address clippy warnings when reasonable
- The project follows standard Rust conventions

### Dependencies

- Keep dependencies minimal and well-justified
- Prefer standard library solutions when possible
- Update dependencies only when necessary

## Pull Request Process

### Before Submitting

1. **Ensure your code is formatted**:
   ```bash
   cargo fmt
   ```

2. **Run all tests**:
   ```bash
   cargo test --all
   ```

3. **Run clippy**:
   ```bash
   cargo clippy
   ```

4. **Test your changes manually**:
   ```bash
   cargo run -- test-data.csv
   echo "Name,Age\nJohn,25" | cargo run
   ```

### Pull Request Guidelines

- **Write clear commit messages** describing what changed and why
- **Include tests** for new features or bug fixes
- **Update documentation** if your changes affect usage
- **Keep changes focused** - one feature or fix per PR
- **Reference issues** if your PR addresses existing issues

### PR Description

Include in your PR description:
- **What** changed
- **Why** it was needed
- **How** to test the changes
- **Any breaking changes** or migration notes

## Reporting Issues

### Bug Reports

When reporting bugs, please include:

- **csvmd version**: `cargo run -- --version`
- **Operating system**: Windows/macOS/Linux
- **Rust version**: `rustc --version`
- **Input data**: Sample CSV that reproduces the issue (if possible)
- **Expected behavior**: What you expected to happen
- **Actual behavior**: What actually happened
- **Steps to reproduce**: Exact commands used

### Feature Requests

For feature requests:
- **Describe the use case**: Why is this feature needed?
- **Provide examples**: Show how it would be used
- **Consider alternatives**: Are there existing ways to achieve this?

## Building and Running

### Development Builds

```bash
# Debug build (faster compilation)
cargo build

# Run with arguments
cargo run -- input.csv
cargo run -- --delimiter ";" --no-headers data.csv
cargo run -- --stream large_file.csv
```

### Release Builds

```bash
# Optimized release build
cargo build --release

# Run release binary
./target/release/csvmd input.csv
```

### Testing Different Scenarios

```bash
# Test with file input
echo "Name,Age\nJohn,25\nJane,30" > test.csv
cargo run -- test.csv

# Test with stdin
echo "Product,Price\nLaptop,$999" | cargo run

# Test streaming mode
cargo run -- --stream large_file.csv

# Test different alignments
cargo run -- --align center test.csv
```

## Architecture Notes

### Key Design Decisions

- **Flexible CSV parsing**: Handles uneven column counts gracefully
- **Markdown escaping**: Properly escapes `|` characters and newlines
- **Memory efficiency**: Streaming mode for large files
- **Error handling**: Comprehensive error types for different failure modes

### Performance Considerations

- **Standard mode**: Suitable for files that fit in memory
- **Streaming mode**: Constant memory usage regardless of file size
- **Two-pass streaming**: First pass determines column count, second pass outputs data

## Getting Help

- **Check existing issues** before creating new ones
- **Look at tests** for usage examples
- **Review the README** for basic usage information
- **Examine the code** - it's well-documented and approachable

## License

By contributing, you agree that your contributions will be licensed under the same MIT License that covers the project.