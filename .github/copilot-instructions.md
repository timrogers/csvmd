## Project Overview

csvmd is a Rust CLI tool that converts CSV files to Markdown tables. The tool supports both standard and streaming modes, handles complex CSV features (quoted fields, embedded newlines, pipe characters), and provides comprehensive error handling.

## Architecture

The project follows a standard Rust library + binary pattern:

- **src/lib.rs**: Core library containing conversion logic with two main functions:
  - `csv_to_markdown()`: Loads entire CSV into memory, suitable for smaller files
  - `csv_to_markdown_streaming()`: Two-pass streaming approach for large files (determines max columns first, then streams output)
- **src/main.rs**: CLI interface using clap for argument parsing
- **src/error.rs**: Custom error types with thiserror for CSV parsing, IO, and formatting errors

Key design decisions:

- Uses csv crate with flexible parsing to handle uneven column counts
- Escapes Markdown special characters: `|` → `\|`, `\n` → `<br>`
- Pre-allocates string capacity based on estimated output size
- Streaming mode uses two-pass approach to ensure correct table formatting

## Common Commands

### Development

```bash
# Build the project
cargo build

# Build optimized release version
cargo build --release

# Run the CLI tool
cargo run -- [OPTIONS] [FILE]

# Format code
cargo fmt

# Lint code
cargo clippy

# Run all tests (unit + integration)
cargo test
```

### Testing

```bash
# Run only unit tests (in src/lib.rs)
cargo test --lib

# Run only integration tests
cargo test --test integration_tests

# Run specific test
cargo test test_csv_with_pipes

# Run with output
cargo test -- --nocapture
```

## CLI Usage Patterns

The tool accepts input from file or stdin:

```bash
# File input
./csvmd data.csv
./csvmd --delimiter ";" --no-headers data.csv
./csvmd --stream large_file.csv

# Stdin input
cat data.csv | ./csvmd
echo "Name,Age\nJohn,25" | ./csvmd
```

## Testing Strategy

- **Unit tests**: Located in src/lib.rs, cover core conversion logic, edge cases, and error conditions
- **Integration tests**: Located in tests/integration_tests.rs, test full CLI functionality using tempfiles and process spawning
- Tests cover: empty input, complex CSV features, Unicode, error conditions, different CLI options

## Key Dependencies

- **csv**: Primary CSV parsing library with flexible parsing enabled
- **clap**: CLI argument parsing with derive feature
- **thiserror**: Error handling with automatic trait implementations
- **tempfile**: Used in integration tests for temporary file creation
