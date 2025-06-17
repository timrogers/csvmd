# csvmd

A simple command-line tool to convert CSV files to Markdown tables.

## Features

- Read CSV from file or stdin
- Handle line breaks in CSV cells (converted to `<br>` tags)
- Escape pipe characters in cells
- Configurable header handling
- Custom CSV delimiters (comma, semicolon, tab, etc.)
- Streaming mode for memory-efficient processing of large files
- Comprehensive error handling with detailed error messages
- Cross-platform support (Linux, Windows, macOS)

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/csvmd`.

## Usage

### Basic usage
```bash
# From file
csvmd input.csv

# From stdin
cat input.csv | csvmd
echo "Name,Age\nJohn,25\nJane,30" | csvmd
```

### Advanced options
```bash
# Custom delimiter (semicolon)
csvmd --delimiter ";" data.csv

# Treat first row as data, not headers
csvmd --no-headers data.csv

# Streaming mode for large files (memory efficient)
csvmd --stream large_file.csv

# Combine options
csvmd --delimiter ";" --no-headers --stream data.csv
```

### Help
```bash
csvmd --help
```

## Examples

### Input CSV:
```csv
Name,Age,Description
John,25,"A person with
multiple lines
in description"
Jane,30,Simple description
Bob,35,"Has | pipe character"
```

### Output Markdown:
```markdown
| Name | Age | Description |
| --- | --- | --- |
| John | 25 | A person with<br>multiple lines<br>in description |
| Jane | 30 | Simple description |
| Bob | 35 | Has \| pipe character |
```

## Development

### Running tests
```bash
cargo test
```

### Building
```bash
cargo build
```

### Formatting and linting
```bash
cargo fmt
cargo clippy
```

## License

This project is open source and available under the [MIT License](LICENSE).