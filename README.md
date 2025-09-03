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

| Dataset | Rows | Input Size | Standard Mode | Streaming Mode (File) | Streaming Mode (Stdin) |
| --- | --- | --- | --- | --- | --- |
| Employee Records | 1,000 | 57KB | 4ms / 3MB | 4ms / 2.6MB | 4ms / 2.6MB |
| Employee Records | 10,000 | 576KB | 10ms / 6.2MB | 10ms / 2.7MB | 10ms / 2.7MB |
| Employee Records | 100,000 | 5.8MB | 90ms / 42MB | 130ms / 2.6MB | 140ms / 17MB |
| Employee Records | 200,000 | 12MB | 180ms / 80MB | 270ms / 2.7MB | 280ms / 35MB |

**Key Benefits of Streaming Mode:**
- **File input**: Constant ~2.7MB memory usage regardless of file size
- **Stdin input**: Reduced memory usage compared to standard mode  
- **Memory efficiency**: Up to 95% reduction in memory usage for large files

### Complex Data with Special Characters

csvmd handles complex CSV data efficiently, including:
- Quoted fields with embedded commas
- Multi-line content with newlines  
- Pipe characters (`|`) that need escaping
- Unicode characters

| Dataset | Rows | Input Size | Standard Mode | Streaming Mode |
| --- | --- | --- | --- | --- |
| Complex Data | 1,000 | 124KB | 4ms / 3MB | 4ms / 2.6MB |
| Complex Data | 10,000 | 1.3MB | 10ms / 8.3MB | 10ms / 2.6MB |

### Streaming Mode Performance

The `--stream` flag provides significant memory efficiency improvements, especially for large files. The implementation uses two different strategies based on input source:

- **File input (seekable)**: Uses a two-pass approach by rewinding the file between passes. Memory usage remains constant (~2.7MB) regardless of file size, as it never buffers the entire file in memory.
- **Stdin/pipe (non-seekable)**: Buffers the entire input to determine column count, then streams output. While this requires more memory than file input, it still provides substantial memory savings compared to standard mode.

#### Memory Usage Comparison (100,000 rows, 5.8MB file):

| Mode | Memory Usage | Reduction |
| --- | --- | --- |
| Standard | 42MB | - |
| Streaming (Stdin) | 17MB | 60% less |
| Streaming (File) | 2.6MB | 94% less |

Examples:

```bash
# File input: optimal memory efficiency
csvmd --stream data.csv > output.md

# Piped input: still provides memory savings
cat data.csv | csvmd --stream > output.md

# With custom alignment
csvmd --stream --align center data.csv > output.md
```
