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
  -h, --help                   Print help
  -V, --version                Print version
```