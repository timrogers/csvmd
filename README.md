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

csvmd is built for speed and efficiency, delivering consistent throughput of **50-80 MB/s** across various dataset sizes.

### Key Performance Highlights

- **Linear Scaling**: Processing time scales linearly with input size
- **Memory Efficient**: Standard mode uses ~5-7x input size, streaming mode uses constant ~12MB
- **Fast Processing**: 4ms for 1K rows, 150ms for 100K rows, 1.2s for 1M rows
- **Complex Data Support**: Minimal overhead for quoted fields, unicode, and special characters

### Quick Comparison

| Dataset Size | Standard Mode | Streaming Mode | Memory Benefit |
| --- | --- | --- | --- |
| 10MB | 150ms, 66MB | 160ms, 12MB | 95% less memory |
| 50MB | 750ms, 280MB | 800ms, 12MB | 96% less memory |
| 100MB | 1.5s, 520MB | 1.6s, 12MB | 98% less memory |

### Streaming Mode Benefits

For large files, use `--stream` to process data with constant memory usage:

- **Memory Efficiency**: Constant ~12MB usage regardless of file size
- **Large File Support**: Handle files larger than available RAM  
- **Minimal Overhead**: Only 5-10% performance penalty vs massive memory savings

```bash
# Process large files with constant memory usage
csvmd --stream huge_dataset.csv > output.md

# Works seamlessly with all options
csvmd --stream --align center --delimiter ";" data.csv > output.md
```

**📊 [View Comprehensive Benchmarks](docs/benchmarks.md)** - Detailed performance analysis across datasets from 1KB to 1GB+