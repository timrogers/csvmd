# csvmd

Convert a CSV file to a Markdown table 📊

```bash
# Convert a file on disk
csvmd input.csv > output.md

# Convert a file passed to stdin
cat input.csv | csvmd > output.md
```

---

## Installation

### macOS or Linux via [Homebrew](https://brew.sh/)

1. Install the latest version by running `brew tap timrogers/tap && brew install csvmd`.
1. Run `csvmd --help` to check that everything is working and see the available commands.

### macOS, Linux or Windows via [Cargo](https://doc.rust-lang.org/cargo/), Rust's package manager

1. Install [Rust](https://www.rust-lang.org/tools/install) on your machine, if it isn't already installed.
1. Install the `csvmd` crate by running `cargo install csvmd`.
1. Run `csvmd --help` to check that everything is working and see the available commands.

### macOS, Linux or Windows via direct binary download

1. Download the [latest release](https://github.com/timrogers/csvmd/releases/latest) for your platform. macOS, Linux and Windows devices are supported.
2. Add the binary to `$PATH`, so you can execute it from your shell. For the best experience, call it `csvmd` on macOS and Linux, and `csvmd.exe` on Windows.
3. Run `csvmd --help` to check that everything is working and see the available commands.

## Usage

```
Convert CSV to Markdown table

Usage: csvmd [OPTIONS] [FILE]

Arguments:
  [FILE]  Input CSV file (if not provided, reads from stdin)

Options:
  -d, --delimiter <DELIMITER>  CSV delimiter character [default: ,]
      --no-headers             Treat first row as data, not headers
      --stream                 Use streaming mode for large files (writes output immediately)
      --align <ALIGN>          Header alignment: left, center, or right [default: left]
  -h, --help                   Print help
  -V, --version                Print version
```

### Basic Examples

```bash
# Basic usage with left-aligned headers (default)
csvmd data.csv

# Center-aligned headers for better readability
csvmd --align center data.csv

# Right-aligned headers for numeric data
csvmd --align right financial_data.csv

# Combined with other options
csvmd --delimiter ";" --align center --no-headers data.csv

# Streaming mode with alignment for large files
csvmd --stream --align center large_dataset.csv
```

## Performance ⚡

csvmd is built for speed and efficiency. Here are some benchmarks showing processing times for various scenarios:

### Real-world Data Processing

| Dataset | Rows | Input Size | Processing Time | Memory Usage |
| --- | --- | --- | --- | --- |
| Employee Records | 1,000 | 71KB | 4ms | 3MB |
| Employee Records | 10,000 | 731KB | 10ms | 8MB |
| Employee Records | 100,000 | 7MB | 150ms | 66MB |
| Employee Records | 200,000 | 13MB | 180ms | ~100MB |

### Complex Data with Special Characters

csvmd handles complex CSV data efficiently, including:
- Quoted fields with embedded commas
- Multi-line content with newlines  
- Pipe characters (`|`) that need escaping
- Unicode characters

| Dataset | Rows | Input Size | Processing Time |
| --- | --- | --- | --- |
| Complex Data | 1,000 | 147KB | 4ms |
| Complex Data | 10,000 | 1MB | 12ms |

### Streaming Mode Benefits

For large files, use `--stream` to process data with constant memory usage:

- **Memory Efficiency**: Streaming mode uses constant memory regardless of file size
- **Immediate Output**: Results appear as soon as processing begins
- **Large File Support**: Handle files larger than available RAM
- **Header Alignment**: The `--align` option works seamlessly with streaming mode, affecting only the header separator line without impacting memory usage or performance

```bash
# Process a 100MB file with constant ~10MB memory usage
csvmd --stream huge_dataset.csv > output.md

# With custom alignment - no additional memory overhead
csvmd --stream --align center huge_dataset.csv > output.md
```

## Development 🛠️

### Building and Testing

```bash
# Build the project
cargo build

# Run all tests
cargo test

# Run only integration tests
cargo test --test integration_tests

# Run only unit tests
cargo test --lib
```

### Snapshot Testing

This project uses [insta](https://insta.rs/) for snapshot testing of CLI behavior. Snapshot tests capture stdout, stderr, and exit codes to ensure CLI behavior remains consistent.

#### Reviewing and Updating Snapshots

When CLI output changes intentionally, you'll need to review and update snapshots:

```bash
# Review all pending snapshot changes
cargo insta review

# Accept all pending snapshots (use with caution)
cargo insta accept

# Update snapshots while running tests
INSTA_UPDATE=always cargo test

# Review snapshots for a specific test
cargo test test_cli_help_flag
cargo insta review
```

#### Snapshot Test Structure

CLI tests capture comprehensive output:
- **Exit code**: Whether the command succeeded (0) or failed (non-zero)
- **Stdout**: Primary program output (Markdown tables, help text, etc.)
- **Stderr**: Error messages and diagnostics

Example snapshot format:
```
exit_code: 1
stdout: 
stderr: Error: Io(Os { code: 2, kind: NotFound, message: "No such file or directory" })
```

#### Best Practices

- **Review carefully**: Always review snapshot changes to ensure they're expected
- **Cross-platform**: Tests normalize platform differences (like `.exe` extensions)
- **Clean stderr**: Build output is filtered out to focus on application errors
- **Comprehensive coverage**: Tests cover success cases, error cases, and edge cases

For more details on snapshot testing, see the [insta documentation](https://insta.rs/).