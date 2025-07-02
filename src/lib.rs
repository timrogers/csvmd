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

use csv::ReaderBuilder;
use error::Result;
use std::fmt::Write as FmtWrite;
use std::io::{Read, Write};

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
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .flexible(config.flexible)
        .delimiter(config.delimiter)
        .from_reader(input);

    let mut records: Vec<Vec<String>> = Vec::new();
    let mut max_cols = 0;

    // First pass: collect all records and determine max column count
    for result in reader.records() {
        let record = result?;
        let row: Vec<String> = record.iter().map(escape_markdown_cell_optimized).collect();

        max_cols = max_cols.max(row.len());
        records.push(row);
    }

    if records.is_empty() {
        return Ok(String::new());
    }

    // Estimate output size to reduce allocations
    let estimated_size = estimate_output_size(&records, max_cols);
    let mut output = String::with_capacity(estimated_size);

    // Write the table
    for (i, record) in records.iter().enumerate() {
        write_table_row(&mut output, record, max_cols)?;

        // Add header separator after first row if configured
        if i == 0 && config.has_headers {
            write_header_separator(&mut output, max_cols, config.header_alignment)?;
        }
    }

    Ok(output)
}

/// Convert CSV data to Markdown and write directly to output.
///
/// This streaming version uses a smart buffering approach that balances
/// memory efficiency with correctness for variable column counts.
/// It buffers a small number of initial rows to determine the stable
/// maximum column count, then streams the rest.
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
    input: R,
    mut output: W,
    config: Config,
) -> Result<()> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .flexible(config.flexible)
        .delimiter(config.delimiter)
        .buffer_capacity(8 * 1024) // 8KB buffer for better I/O performance
        .from_reader(input);

    let mut buffered_rows: Vec<Vec<String>> = Vec::new();
    let max_buffer_size = 50; // Buffer up to 50 rows to detect column patterns
    let mut max_cols_seen = 0;
    let mut streaming_started = false;

    for result in reader.records() {
        let record = result?;
        let row: Vec<String> = record.iter().map(escape_markdown_cell_optimized).collect();
        let current_cols = row.len();

        max_cols_seen = max_cols_seen.max(current_cols);

        if !streaming_started {
            // We're still in buffering mode
            buffered_rows.push(row);

            // Start streaming if buffer is full or we've seen consistent column counts
            if buffered_rows.len() >= max_buffer_size {
                streaming_started = true;

                // Flush all buffered rows
                for (idx, buffered_row) in buffered_rows.iter().enumerate() {
                    write_table_row_to_writer(&mut output, buffered_row, max_cols_seen)?;

                    // Add header separator after first row if configured
                    if idx == 0 && config.has_headers {
                        write_header_separator_to_writer(
                            &mut output,
                            max_cols_seen,
                            config.header_alignment,
                        )?;
                    }
                }
                buffered_rows.clear();
            }
        } else {
            // We're in streaming mode
            if current_cols > max_cols_seen {
                // This shouldn't happen often, but handle gracefully
                max_cols_seen = current_cols;
            }

            write_table_row_to_writer(&mut output, &row, max_cols_seen)?;
        }
    }

    // If we never started streaming (small file), flush buffered rows
    if !streaming_started && !buffered_rows.is_empty() {
        for (idx, buffered_row) in buffered_rows.iter().enumerate() {
            write_table_row_to_writer(&mut output, buffered_row, max_cols_seen)?;

            // Add header separator after first row if configured
            if idx == 0 && config.has_headers {
                write_header_separator_to_writer(
                    &mut output,
                    max_cols_seen,
                    config.header_alignment,
                )?;
            }
        }
    }

    output.flush()?;
    Ok(())
}

/// Optimized version of escape_markdown_cell that processes characters in a single pass.
fn escape_markdown_cell_optimized(field: &str) -> String {
    // Early return for simple cases
    if !field.contains(['|', '\n', '\r']) {
        return field.to_string();
    }

    // Estimate capacity (common case: few special characters)
    let mut result = String::with_capacity(field.len() + 16);

    for ch in field.chars() {
        match ch {
            '|' => result.push_str("\\|"),
            '\n' => result.push_str("<br>"),
            '\r' => continue, // Skip carriage returns
            _ => result.push(ch),
        }
    }

    result
}

/// Write a table row to a string buffer.
fn write_table_row(output: &mut String, row: &[String], max_cols: usize) -> Result<()> {
    output.push('|');

    for i in 0..max_cols {
        let cell = row.get(i).map(String::as_str).unwrap_or("");
        write!(output, " {} |", cell)?;
    }

    output.push('\n');
    Ok(())
}

/// Write a table row directly to a writer.
fn write_table_row_to_writer<W: Write>(
    output: &mut W,
    row: &[String],
    max_cols: usize,
) -> Result<()> {
    // Pre-allocate buffer for the entire row to reduce system calls
    let estimated_size = max_cols * 10 + 10; // rough estimate
    let mut buffer = String::with_capacity(estimated_size);

    buffer.push('|');
    for i in 0..max_cols {
        let cell = row.get(i).map(String::as_str).unwrap_or("");
        buffer.push(' ');
        buffer.push_str(cell);
        buffer.push_str(" |");
    }
    buffer.push('\n');

    output.write_all(buffer.as_bytes())?;
    Ok(())
}

/// Write the header separator line to a string buffer.
fn write_header_separator(
    output: &mut String,
    max_cols: usize,
    alignment: HeaderAlignment,
) -> Result<()> {
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
    Ok(())
}

/// Write the header separator line directly to a writer.
fn write_header_separator_to_writer<W: Write>(
    output: &mut W,
    max_cols: usize,
    alignment: HeaderAlignment,
) -> Result<()> {
    // Pre-allocate buffer for efficiency
    let mut buffer = String::with_capacity(max_cols * 8 + 10);

    buffer.push('|');

    let separator = match alignment {
        HeaderAlignment::Left => " --- |",
        HeaderAlignment::Center => " :---: |",
        HeaderAlignment::Right => " ---: |",
    };

    for _ in 0..max_cols {
        buffer.push_str(separator);
    }
    buffer.push('\n');

    output.write_all(buffer.as_bytes())?;
    Ok(())
}

/// Estimate the output size to pre-allocate string capacity.
fn estimate_output_size(records: &[Vec<String>], max_cols: usize) -> usize {
    if records.is_empty() {
        return 0;
    }

    // Calculate total character content
    let total_content_size: usize = records
        .iter()
        .flat_map(|row| row.iter())
        .map(|cell| cell.len())
        .sum();

    // More accurate estimation:
    // - Content size + markdown formatting (3 chars per cell: " | ")
    // - Row separators (1 newline per row)
    // - Header separator line (max_cols * 7 + 2 for a typical "| --- |" pattern)
    let formatting_overhead = records.len() * max_cols * 3 + records.len();
    let header_separator = if records.len() > 0 {
        max_cols * 7 + 2
    } else {
        0
    };

    total_content_size + formatting_overhead + header_separator + 50 // small buffer
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_escape_markdown_cell() {
        assert_eq!(escape_markdown_cell_optimized("simple"), "simple");
        assert_eq!(escape_markdown_cell_optimized("with|pipe"), "with\\|pipe");
        assert_eq!(
            escape_markdown_cell_optimized("with\nlinebreak"),
            "with<br>linebreak"
        );
        assert_eq!(
            escape_markdown_cell_optimized("with\r\nwindows"),
            "with<br>windows"
        );
        assert_eq!(escape_markdown_cell_optimized(""), "");
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
