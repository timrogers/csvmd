//! A library for converting CSV data to Markdown tables.
//!
//! This crate provides functionality to convert CSV (Comma-Separated Values) data
//! into properly formatted Markdown tables. It handles various CSV complexities
//! including quoted fields, embedded newlines, and pipe characters.
//!
//! # Features
//!
//! - Stream processing for memory efficiency with large files
//! - Proper escaping of Markdown special characters
//! - Support for uneven column counts across rows
//! - Comprehensive error handling
//!
//! # Example
//!
//! ```rust
//! use csvmd::{csv_to_markdown, Config};
//! use std::io::Cursor;
//!
//! let csv_data = "Name,Age\nJohn,25\nJane,30";
//! let input = Cursor::new(csv_data);
//! let config = Config::default();
//! let result = csv_to_markdown(input, config).unwrap();
//! println!("{}", result);
//! ```

pub mod error;

use csv::{ReaderBuilder, StringRecord};
use error::Result;
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};

/// Header alignment options for Markdown tables.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderAlignment {
    /// Left-aligned headers (default): `| --- |`
    Left,
    /// Center-aligned headers: `| :---: |`
    Center,
    /// Right-aligned headers: `| ---: |`
    Right,
}

/// Configuration for CSV to Markdown conversion.
#[derive(Debug, Clone)]
pub struct Config {
    /// Whether the CSV has headers (affects separator line placement).
    pub has_headers: bool,
    /// Whether to allow flexible column counts.
    pub flexible: bool,
    /// CSV field delimiter character.
    pub delimiter: u8,
    /// Header alignment for Markdown table.
    pub header_alignment: HeaderAlignment,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            has_headers: true,
            flexible: true,
            delimiter: b',',
            header_alignment: HeaderAlignment::Left,
        }
    }
}

/// Convert CSV data to a Markdown table string.
///
/// This function reads CSV data from the provided reader and converts it to
/// a Markdown table format. It processes the data in a streaming fashion to
/// handle large files efficiently.
///
/// # Arguments
///
/// * `input` - A reader containing CSV data
/// * `config` - Configuration options for the conversion
///
/// # Returns
///
/// A string containing the formatted Markdown table.
///
/// # Errors
///
/// Returns `CsvMdError` if:
/// - The input cannot be read
/// - The CSV data is malformed
/// - Memory allocation fails during processing
///
/// # Example
///
/// ```rust
/// use csvmd::{csv_to_markdown, Config};
/// use std::io::Cursor;
///
/// let csv_data = "Name,Age\nJohn,25\nJane,30";
/// let input = Cursor::new(csv_data);
/// let config = Config::default();
/// let result = csv_to_markdown(input, config)?;
/// assert!(result.contains("| Name | Age |"));
/// # Ok::<(), csvmd::error::CsvMdError>(())
/// ```
pub fn csv_to_markdown<R: Read>(input: R, config: Config) -> Result<String> {
    let mut reader = new_reader(input, &config);

    let mut records: Vec<StringRecord> = Vec::new();
    let mut max_cols = 0;
    let mut total_bytes = 0;

    // First pass: collect all records and determine max column count
    let mut record = StringRecord::new();
    while reader.read_record(&mut record)? {
        max_cols = max_cols.max(record.len());
        total_bytes += record.as_slice().len();
        records.push(record.clone());
    }

    if records.is_empty() {
        return Ok(String::new());
    }

    // Estimate output size to reduce allocations
    let estimated_size = total_bytes + (max_cols * 4 + 2) * records.len() + max_cols * 6 + 100;
    let mut output = String::with_capacity(estimated_size);

    // Write the table
    for (i, record) in records.iter().enumerate() {
        write_table_row(&mut output, record, max_cols);

        // Add header separator after first row if configured
        if i == 0 && config.has_headers {
            write_header_separator(&mut output, max_cols, config.header_alignment);
        }
    }

    Ok(output)
}

/// Convert CSV data to Markdown and write directly to output.
///
/// This streaming version uses a two-pass approach:
/// 1. First pass: determine the maximum column count
/// 2. Second pass: stream output with correct table formatting
///
/// This provides memory efficiency for large files while ensuring correct
/// Markdown table structure.
///
/// # Arguments
///
/// * `input` - A reader containing CSV data
/// * `output` - A writer where the Markdown table will be written
/// * `config` - Configuration options for the conversion
///
/// # Errors
///
/// Returns `CsvMdError` if reading, parsing, or writing fails.
pub fn csv_to_markdown_streaming<R: Read, W: Write>(
    mut input: R,
    output: W,
    config: Config,
) -> Result<()> {
    // First, we need to read the input to determine max columns
    // Since we need to read twice, we'll read all data into memory first
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer)?;

    // First pass: determine max column count
    let max_cols = count_max_cols(new_reader(&buffer[..], &config))?;

    // Second pass: stream output with correct column count
    write_streaming(new_reader(&buffer[..], &config), output, max_cols, &config)
}

/// Convert CSV data to Markdown and write directly to output using a seekable input.
///
/// For seekable inputs (e.g., files), this variant avoids buffering the entire
/// input into memory. It performs two passes by rewinding the reader between
/// passes to compute the maximum column count and then write the output.
pub fn csv_to_markdown_streaming_seekable<R: Read + Seek, W: Write>(
    mut input: R,
    output: W,
    config: Config,
) -> Result<()> {
    // First pass: determine max column count
    input.seek(SeekFrom::Start(0))?;
    let max_cols = count_max_cols(new_reader(&mut input, &config))?;

    // Second pass: rewind and stream output with correct column count
    input.seek(SeekFrom::Start(0))?;
    write_streaming(new_reader(&mut input, &config), output, max_cols, &config)
}

fn new_reader<R: Read>(input: R, config: &Config) -> csv::Reader<R> {
    ReaderBuilder::new()
        .has_headers(false)
        .flexible(config.flexible)
        .delimiter(config.delimiter)
        .from_reader(input)
}

fn count_max_cols<R: Read>(mut reader: csv::Reader<R>) -> Result<usize> {
    let mut record = StringRecord::new();
    let mut max_cols = 0;
    while reader.read_record(&mut record)? {
        max_cols = max_cols.max(record.len());
    }
    Ok(max_cols)
}

/// Stream rows to the writer, building each row in a reused buffer and writing
/// through a `BufWriter` to avoid many small writes.
fn write_streaming<R: Read, W: Write>(
    mut reader: csv::Reader<R>,
    output: W,
    max_cols: usize,
    config: &Config,
) -> Result<()> {
    let mut output = BufWriter::with_capacity(64 * 1024, output);
    let mut line = String::new();
    let mut record = StringRecord::new();
    let mut first_row = true;

    while reader.read_record(&mut record)? {
        line.clear();
        write_table_row(&mut line, &record, max_cols);
        output.write_all(line.as_bytes())?;

        if first_row && config.has_headers {
            line.clear();
            write_header_separator(&mut line, max_cols, config.header_alignment);
            output.write_all(line.as_bytes())?;
            first_row = false;
        }
    }

    output.flush()?;
    Ok(())
}

/// Append a Markdown-escaped CSV cell to `out`.
///
/// This function handles:
/// - Pipe characters (`|`) → escaped as `\|`
/// - Newlines (`\n`) → converted to `<br>` tags
/// - Carriage returns (`\r`) → removed
fn push_escaped_cell(out: &mut String, field: &str) {
    let mut start = 0;
    for (i, &b) in field.as_bytes().iter().enumerate() {
        let replacement = match b {
            b'|' => "\\|",
            b'\n' => "<br>",
            b'\r' => "",
            _ => continue,
        };
        out.push_str(&field[start..i]);
        out.push_str(replacement);
        start = i + 1;
    }
    out.push_str(&field[start..]);
}

#[cfg(test)]
fn escape_markdown_cell(field: &str) -> String {
    let mut out = String::new();
    push_escaped_cell(&mut out, field);
    out
}

/// Write a table row to a string buffer.
fn write_table_row(output: &mut String, row: &StringRecord, max_cols: usize) {
    output.push('|');

    for i in 0..max_cols {
        output.push(' ');
        push_escaped_cell(output, row.get(i).unwrap_or(""));
        output.push_str(" |");
    }

    output.push('\n');
}

/// Write the header separator line to a string buffer.
fn write_header_separator(output: &mut String, max_cols: usize, alignment: HeaderAlignment) {
    output.push('|');

    let separator = match alignment {
        HeaderAlignment::Left => " --- |",
        HeaderAlignment::Center => " :---: |",
        HeaderAlignment::Right => " ---: |",
    };

    for _ in 0..max_cols {
        output.push_str(separator);
    }

    output.push('\n');
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_escape_markdown_cell() {
        assert_eq!(escape_markdown_cell("simple"), "simple");
        assert_eq!(escape_markdown_cell("with|pipe"), "with\\|pipe");
        assert_eq!(escape_markdown_cell("with\nlinebreak"), "with<br>linebreak");
        assert_eq!(escape_markdown_cell("with\r\nwindows"), "with<br>windows");
        assert_eq!(escape_markdown_cell(""), "");
    }

    #[test]
    fn test_simple_csv() {
        let csv_data = "Name,Age\nJohn,25\nJane,30";
        let input = Cursor::new(csv_data);
        let config = Config::default();
        let result = csv_to_markdown(input, config).unwrap();

        let expected = "| Name | Age |\n| --- | --- |\n| John | 25 |\n| Jane | 30 |\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_csv() {
        let csv_data = "";
        let input = Cursor::new(csv_data);
        let config = Config::default();
        let result = csv_to_markdown(input, config).unwrap();

        assert_eq!(result, "");
    }

    #[test]
    fn test_csv_with_line_breaks() {
        let csv_data = "Name,Description\nJohn,\"Line 1\nLine 2\"";
        let input = Cursor::new(csv_data);
        let config = Config::default();
        let result = csv_to_markdown(input, config).unwrap();

        let expected = "| Name | Description |\n| --- | --- |\n| John | Line 1<br>Line 2 |\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_csv_with_pipes() {
        let csv_data = "Name,Description\nJohn,\"Has | pipe\"";
        let input = Cursor::new(csv_data);
        let config = Config::default();
        let result = csv_to_markdown(input, config).unwrap();

        let expected = "| Name | Description |\n| --- | --- |\n| John | Has \\| pipe |\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_csv_with_uneven_columns() {
        let csv_data = "A,B,C\nX,Y\nP,Q,R,S";
        let input = Cursor::new(csv_data);
        let config = Config::default();
        let result = csv_to_markdown(input, config).unwrap();

        let expected =
            "| A | B | C |  |\n| --- | --- | --- | --- |\n| X | Y |  |  |\n| P | Q | R | S |\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_row_csv() {
        let csv_data = "Name,Age,City";
        let input = Cursor::new(csv_data);
        let config = Config::default();
        let result = csv_to_markdown(input, config).unwrap();

        let expected = "| Name | Age | City |\n| --- | --- | --- |\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_csv_with_empty_cells() {
        let csv_data = "Name,Age,City\nJohn,,NYC\n,25,";
        let input = Cursor::new(csv_data);
        let config = Config::default();
        let result = csv_to_markdown(input, config).unwrap();

        let expected =
            "| Name | Age | City |\n| --- | --- | --- |\n| John |  | NYC |\n|  | 25 |  |\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_csv_with_quotes_and_commas() {
        let csv_data = "Name,Description\nJohn,\"Smith, Jr.\"\nJane,\"O'Connor\"";
        let input = Cursor::new(csv_data);
        let config = Config::default();
        let result = csv_to_markdown(input, config).unwrap();

        let expected =
            "| Name | Description |\n| --- | --- |\n| John | Smith, Jr. |\n| Jane | O'Connor |\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_csv_with_special_characters() {
        let csv_data = "Symbol,Unicode\n★,\"U+2605\"\n♠,\"U+2660\"";
        let input = Cursor::new(csv_data);
        let config = Config::default();
        let result = csv_to_markdown(input, config).unwrap();

        let expected = "| Symbol | Unicode |\n| --- | --- |\n| ★ | U+2605 |\n| ♠ | U+2660 |\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_csv_with_whitespace() {
        let csv_data = " Name , Age \n John , 25 \n Jane , 30 ";
        let input = Cursor::new(csv_data);
        let config = Config::default();
        let result = csv_to_markdown(input, config).unwrap();

        let expected = "|  Name  |  Age  |\n| --- | --- |\n|  John  |  25  |\n|  Jane  |  30  |\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_no_headers_config() {
        let csv_data = "Data1,Data2\nValue1,Value2";
        let input = Cursor::new(csv_data);
        let config = Config {
            has_headers: false,
            ..Config::default()
        };
        let result = csv_to_markdown(input, config).unwrap();

        // Should not have separator line when no headers
        let expected = "| Data1 | Data2 |\n| Value1 | Value2 |\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_custom_delimiter() {
        let csv_data = "Name;Age\nJohn;25\nJane;30";
        let input = Cursor::new(csv_data);
        let config = Config {
            delimiter: b';',
            ..Config::default()
        };
        let result = csv_to_markdown(input, config).unwrap();

        let expected = "| Name | Age |\n| --- | --- |\n| John | 25 |\n| Jane | 30 |\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_streaming_mode() {
        let csv_data = "Name,Age\nJohn,25\nJane,30";
        let input = Cursor::new(csv_data);
        let mut output = Vec::new();
        let config = Config::default();

        csv_to_markdown_streaming(input, &mut output, config).unwrap();

        let result = String::from_utf8(output).unwrap();
        let expected = "| Name | Age |\n| --- | --- |\n| John | 25 |\n| Jane | 30 |\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_streaming_mode_no_headers() {
        let csv_data = "Data1,Data2\nValue1,Value2";
        let input = Cursor::new(csv_data);
        let mut output = Vec::new();
        let config = Config {
            has_headers: false,
            ..Config::default()
        };

        csv_to_markdown_streaming(input, &mut output, config).unwrap();

        let result = String::from_utf8(output).unwrap();
        let expected = "| Data1 | Data2 |\n| Value1 | Value2 |\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_streaming_mode_uneven_columns() {
        // This test exposes the current streaming bug: early rows are malformed
        // when later rows have more columns
        let csv_data = "A,B\nX,Y,Z\nP,Q,R,S";
        let input = Cursor::new(csv_data);
        let mut output = Vec::new();
        let config = Config::default();

        csv_to_markdown_streaming(input, &mut output, config).unwrap();

        let result = String::from_utf8(output).unwrap();

        // Expected: all rows should have 4 columns (max from any row)
        let expected =
            "| A | B |  |  |\n| --- | --- | --- | --- |\n| X | Y | Z |  |\n| P | Q | R | S |\n";

        // Fixed: streaming now uses two-pass approach to determine max_cols correctly
        assert_eq!(result, expected);
    }

    #[test]
    fn test_header_alignment_left() {
        let csv_data = "Name,Age\nJohn,25\nJane,30";
        let input = Cursor::new(csv_data);
        let config = Config {
            header_alignment: HeaderAlignment::Left,
            ..Config::default()
        };
        let result = csv_to_markdown(input, config).unwrap();

        let expected = "| Name | Age |\n| --- | --- |\n| John | 25 |\n| Jane | 30 |\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_header_alignment_center() {
        let csv_data = "Name,Age\nJohn,25\nJane,30";
        let input = Cursor::new(csv_data);
        let config = Config {
            header_alignment: HeaderAlignment::Center,
            ..Config::default()
        };
        let result = csv_to_markdown(input, config).unwrap();

        let expected = "| Name | Age |\n| :---: | :---: |\n| John | 25 |\n| Jane | 30 |\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_header_alignment_right() {
        let csv_data = "Name,Age\nJohn,25\nJane,30";
        let input = Cursor::new(csv_data);
        let config = Config {
            header_alignment: HeaderAlignment::Right,
            ..Config::default()
        };
        let result = csv_to_markdown(input, config).unwrap();

        let expected = "| Name | Age |\n| ---: | ---: |\n| John | 25 |\n| Jane | 30 |\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_streaming_header_alignment_center() {
        let csv_data = "Name,Age\nJohn,25\nJane,30";
        let input = Cursor::new(csv_data);
        let mut output = Vec::new();
        let config = Config {
            header_alignment: HeaderAlignment::Center,
            ..Config::default()
        };

        csv_to_markdown_streaming(input, &mut output, config).unwrap();

        let result = String::from_utf8(output).unwrap();
        let expected = "| Name | Age |\n| :---: | :---: |\n| John | 25 |\n| Jane | 30 |\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_header_alignment_no_headers() {
        let csv_data = "Data1,Data2\nValue1,Value2";
        let input = Cursor::new(csv_data);
        let config = Config {
            has_headers: false,
            header_alignment: HeaderAlignment::Center, // Should be ignored
            ..Config::default()
        };
        let result = csv_to_markdown(input, config).unwrap();

        // Should not have separator line when no headers, regardless of alignment
        let expected = "| Data1 | Data2 |\n| Value1 | Value2 |\n";
        assert_eq!(result, expected);
    }
}
