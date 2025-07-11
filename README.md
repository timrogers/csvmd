# csvmd

Convert a CSV file to a Markdown table 📊

```bash
# Convert a file on disk
csvmd input.csv > output.md

# Convert a file passed to stdin
cat input.csv | csvmd > output.md
```

## Performance

csvmd is designed for high performance with both small and large CSV files. Here are benchmark results from real-world testing:

### Processing Speed

| File Size | Standard Mode | Streaming Mode | Throughput |
|-----------|---------------|----------------|------------|
| 100 rows × 5 cols | 113 μs | 150 μs | ~42 MB/s |
| 1,000 rows × 5 cols | 1.0 ms | 1.2 ms | ~51 MB/s |
| 10,000 rows × 5 cols | 10.5 ms | 12.3 ms | ~54 MB/s |

### Memory Usage

csvmd has excellent memory efficiency:

- **Memory overhead**: Only ~19% increase from input to output size
- **1,000 rows**: 53 KB → 65 KB (1.22x)
- **100,000 rows**: 6.3 MB → 7.5 MB (1.19x)

### Column Scaling Performance

Performance scales well with increasing column counts:

| Columns | Processing Time | Throughput |
|---------|----------------|------------|
| 5 columns | 1.0 ms | ~52 MB/s |
| 10 columns | 1.8 ms | ~59 MB/s |
| 20 columns | 3.5 ms | ~63 MB/s |
| 50 columns | 8.1 ms | ~68 MB/s |

### Complex Data Handling

Even with complex CSV features (escaped pipes, embedded newlines, quotes), performance remains excellent:

- **100 rows**: 110 μs (~54 MB/s)
- **1,000 rows**: 965 μs (~63 MB/s)  
- **5,000 rows**: 4.8 ms (~65 MB/s)

### Mode Comparison

- **Standard mode**: Best for smaller files (< 10MB), loads entire CSV into memory
- **Streaming mode**: Better for very large files, uses two-pass approach with minimal memory usage
- **Trade-off**: Streaming mode is ~15-20% slower but uses constant memory regardless of file size

All benchmarks performed on a modern Apple Silicon Mac. Performance will vary based on hardware and specific CSV characteristics.

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
  -h, --help                   Print help
  -V, --version                Print version
https://github.com/github/marketing-data-requests/issues/125