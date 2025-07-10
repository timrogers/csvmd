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

### Streaming Mode Performance

For large files, use `--stream` to process data with constant memory usage. Here are benchmarks comparing streaming vs standard mode:

| Dataset | Rows | File Size | Standard Mode | Streaming Mode | Memory Usage (Streaming) |
| --- | --- | --- | --- | --- | --- |
| Employee Data | 1,000 | 71KB | 4ms | 4ms | ~10MB |
| Employee Data | 10,000 | 737KB | 16ms | 20ms | ~10MB |
| Employee Data | 50,000 | 3MB | 67ms | 91ms | ~10MB |
| Employee Data | 100,000 | 7MB | 129ms | 181ms | ~10MB |
| Employee Data | 200,000 | 15MB | ~270ms | 337ms | ~10MB |

**Key Benefits of Streaming Mode:**
- **Memory Efficiency**: Constant memory usage (~10MB) regardless of file size
- **Immediate Output**: Results appear as soon as processing begins
- **Large File Support**: Handle files larger than available RAM
- **Header Alignment**: The `--align` option works seamlessly with streaming mode

```bash
# Process a 100MB file with constant ~10MB memory usage
csvmd --stream huge_dataset.csv > output.md

# With custom alignment - no additional memory overhead
csvmd --stream --align center huge_dataset.csv > output.md
```