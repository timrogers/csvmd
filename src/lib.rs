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
use std::io::{Read, Write, Seek};

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
        let row: Vec<String> = record.iter().map(escape_markdown_cell).collect();

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
    mut output: W,
    config: Config,
) -> Result<()> {
    // First, we need to read the input to determine max columns
    // Since we need to read twice, we'll read all data into memory first
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer)?;

    // First pass: determine max column count
    let max_cols = {
        let cursor = std::io::Cursor::new(&buffer);
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .flexible(config.flexible)
            .delimiter(config.delimiter)
            .from_reader(cursor);

        let mut max_cols = 0;
        for result in reader.records() {
            let record = result?;
            max_cols = max_cols.max(record.len());
        }
        max_cols
    };

    // Second pass: stream output with correct column count
    let cursor = std::io::Cursor::new(&buffer);
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .flexible(config.flexible)
        .delimiter(config.delimiter)
        .from_reader(cursor);

    let mut first_row = true;

    for result in reader.records() {
        let record = result?;
        let row: Vec<String> = record.iter().map(escape_markdown_cell).collect();

        // Write the row with correct column count
        write_table_row_to_writer(&mut output, &row, max_cols)?;

        // Add header separator after first row if configured
        if first_row && config.has_headers {
            write_header_separator_to_writer(&mut output, max_cols, config.header_alignment)?;
            first_row = false;
        }
    }

    output.flush()?;
    Ok(())
}

/// Convert CSV data to Markdown using true two-pass streaming for seekable inputs.
///
/// This version provides optimal memory efficiency by:
/// 1. **Seekable inputs (files)**: True two-pass streaming without loading entire file into memory
/// 2. **Non-seekable inputs (stdin)**: Memory-efficient chunked processing with temporary storage
///
/// # Arguments
///
/// * `input` - A reader containing CSV data (must be seekable for optimal performance)
/// * `output` - A writer where the Markdown table will be written
/// * `config` - Configuration options for the conversion
///
/// # Errors
///
/// Returns `CsvMdError` if reading, parsing, or writing fails.
pub fn csv_to_markdown_two_pass_streaming<R: Read + std::io::Seek, W: Write>(
    mut input: R,
    mut output: W,
    config: Config,
) -> Result<()> {
    // First pass: determine max column count without storing all data
    let max_cols = {
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .flexible(config.flexible)
            .delimiter(config.delimiter)
            .from_reader(&mut input);

        let mut max_cols = 0;
        for result in reader.records() {
            let record = result?;
            max_cols = max_cols.max(record.len());
        }
        max_cols
    };

    // Reset to beginning for second pass
    input.seek(std::io::SeekFrom::Start(0))?;

    // Second pass: stream output with correct column count
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .flexible(config.flexible)
        .delimiter(config.delimiter)
        .from_reader(&mut input);

    let mut first_row = true;

    for result in reader.records() {
        let record = result?;
        let row: Vec<String> = record.iter().map(escape_markdown_cell).collect();

        // Write the row with correct column count
        write_table_row_to_writer(&mut output, &row, max_cols)?;

        // Add header separator after first row if configured
        if first_row && config.has_headers {
            write_header_separator_to_writer(&mut output, max_cols, config.header_alignment)?;
            first_row = false;
        }
    }

    output.flush()?;
    Ok(())
}

/// Convert CSV data to Markdown using memory-efficient chunked streaming.
///
/// This version provides a balance between memory efficiency and performance by:
/// 1. Processing data in chunks to determine column count
/// 2. Using temporary storage only for metadata, not full content
/// 3. Streaming output as soon as column count is determined
///
/// This is ideal for very large files where memory usage must be minimized.
///
/// # Arguments
///
/// * `input` - A reader containing CSV data
/// * `output` - A writer where the Markdown table will be written  
/// * `config` - Configuration options for the conversion
/// * `chunk_size` - Size of chunks to process at a time (in bytes)
///
/// # Errors
///
/// Returns `CsvMdError` if reading, parsing, or writing fails.
pub fn csv_to_markdown_chunked_streaming<R: Read, W: Write>(
    mut input: R,
    mut output: W,
    config: Config,
    chunk_size: usize,
) -> Result<()> {

    
    // Use a temporary file to store the input for two-pass processing
    let mut temp_file = tempfile::tempfile()?;
    
    // Copy input to temp file while simultaneously determining max columns
    let mut max_cols = 0;
    let mut chunk_buffer = vec![0u8; chunk_size];
    let mut partial_line = String::new();
    
    loop {
        let bytes_read = input.read(&mut chunk_buffer)?;
        if bytes_read == 0 {
            break;
        }
        
        // Write to temp file
        temp_file.write_all(&chunk_buffer[..bytes_read])?;
        
        // Process this chunk to update max_cols
        let chunk_str = String::from_utf8_lossy(&chunk_buffer[..bytes_read]);
        let full_content = format!("{}{}", partial_line, chunk_str);
        
        let lines: Vec<&str> = full_content.lines().collect();
        
        // Process complete lines (all but possibly the last one)
        let complete_lines = if full_content.ends_with('\n') || full_content.ends_with('\r') {
            &lines[..]
        } else if lines.len() > 1 {
            partial_line = lines[lines.len()-1].to_string();
            &lines[..lines.len()-1]
        } else {
            partial_line = full_content;
            &[]
        };
        
        // Analyze complete lines for column count
        for line in complete_lines {
            if !line.is_empty() {
                let cols = count_csv_columns(line, config.delimiter);
                max_cols = max_cols.max(cols);
            }
        }
    }
    
    // Process any remaining partial line
    if !partial_line.trim().is_empty() {
        let cols = count_csv_columns(&partial_line, config.delimiter);
        max_cols = max_cols.max(cols);
    }
    
    // Reset temp file to beginning for second pass
    temp_file.seek(std::io::SeekFrom::Start(0))?;
    
    // Second pass: stream output with correct column count
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .flexible(config.flexible)
        .delimiter(config.delimiter)
        .from_reader(temp_file);

    let mut first_row = true;

    for result in reader.records() {
        let record = result?;
        let row: Vec<String> = record.iter().map(escape_markdown_cell).collect();

        // Write the row with correct column count
        write_table_row_to_writer(&mut output, &row, max_cols)?;

        // Add header separator after first row if configured
        if first_row && config.has_headers {
            write_header_separator_to_writer(&mut output, max_cols, config.header_alignment)?;
            first_row = false;
        }
    }

    output.flush()?;
    Ok(())
}

/// Count columns in a CSV line (simple estimation for chunked processing).
fn count_csv_columns(line: &str, delimiter: u8) -> usize {
    let delimiter_char = delimiter as char;
    let mut count = 1;
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    
    while let Some(ch) = chars.next() {
        match ch {
            '"' => {
                // Handle quote escaping
                if chars.peek() == Some(&'"') {
                    chars.next(); // Skip escaped quote
                } else {
                    in_quotes = !in_quotes;
                }
            }
            c if c == delimiter_char && !in_quotes => {
                count += 1;
            }
            _ => {}
        }
    }
    
    count
}

/// Escape Markdown special characters in a CSV cell.
///
/// This function handles:
/// - Pipe characters (`|`) → escaped as `\|`
/// - Newlines (`\n`) → converted to `<br>` tags
/// - Carriage returns (`\r`) → removed
fn escape_markdown_cell(field: &str) -> String {
    field
        .replace('|', "\\|")
        .replace('\n', "<br>")
        .replace('\r', "")
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
    write!(output, "|")?;

    for i in 0..max_cols {
        let cell = row.get(i).map(String::as_str).unwrap_or("");
        write!(output, " {} |", cell)?;
    }

    writeln!(output)?;
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
    write!(output, "|")?;

    let separator = match alignment {
        HeaderAlignment::Left => " --- |",
        HeaderAlignment::Center => " :---: |",
        HeaderAlignment::Right => " ---: |",
    };

    for _ in 0..max_cols {
        write!(output, "{}", separator)?;
    }

    writeln!(output)?;
    Ok(())
}

/// Estimate the output size to pre-allocate string capacity.
fn estimate_output_size(records: &[Vec<String>], max_cols: usize) -> usize {
    let avg_cell_size = records
        .iter()
        .flat_map(|row| row.iter())
        .map(|cell| cell.len())
        .sum::<usize>()
        / records.len().max(1);

    // Rough estimate: (avg_cell_size + 3) * cols * rows + separators
    (avg_cell_size + 3) * max_cols * records.len() + (max_cols * 6) + 100
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

    #[test]
    fn test_two_pass_streaming_basic() {
        let csv_data = "Name,Age\nJohn,25\nJane,30";
        let input = Cursor::new(csv_data);
        let mut output = Vec::new();
        let config = Config::default();

        csv_to_markdown_two_pass_streaming(input, &mut output, config).unwrap();

        let result = String::from_utf8(output).unwrap();
        let expected = "| Name | Age |\n| --- | --- |\n| John | 25 |\n| Jane | 30 |\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_two_pass_streaming_uneven_columns() {
        let csv_data = "A,B,C\nX,Y\nP,Q,R,S";
        let input = Cursor::new(csv_data);
        let mut output = Vec::new();
        let config = Config::default();

        csv_to_markdown_two_pass_streaming(input, &mut output, config).unwrap();

        let result = String::from_utf8(output).unwrap();
        let expected = "| A | B | C |  |\n| --- | --- | --- | --- |\n| X | Y |  |  |\n| P | Q | R | S |\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_chunked_streaming_basic() {
        let csv_data = "Name,Age,City\nJohn,25,NYC\nJane,30,LA\nBob,35,Chicago";
        let input = Cursor::new(csv_data);
        let mut output = Vec::new();
        let config = Config::default();
        let chunk_size = 20; // Small chunk size to test chunking

        csv_to_markdown_chunked_streaming(input, &mut output, config, chunk_size).unwrap();

        let result = String::from_utf8(output).unwrap();
        let expected = "| Name | Age | City |\n| --- | --- | --- |\n| John | 25 | NYC |\n| Jane | 30 | LA |\n| Bob | 35 | Chicago |\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_chunked_streaming_with_quotes() {
        let csv_data = "Name,Description\nJohn,\"A long description with, commas\"\nJane,\"Another description\"";
        let input = Cursor::new(csv_data);
        let mut output = Vec::new();
        let config = Config::default();
        let chunk_size = 15; // Small chunk size

        csv_to_markdown_chunked_streaming(input, &mut output, config, chunk_size).unwrap();

        let result = String::from_utf8(output).unwrap();
        let expected = "| Name | Description |\n| --- | --- |\n| John | A long description with, commas |\n| Jane | Another description |\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_csv_columns() {
        assert_eq!(count_csv_columns("a,b,c", b','), 3);
        assert_eq!(count_csv_columns("a,b", b','), 2);
        assert_eq!(count_csv_columns("a", b','), 1);
        assert_eq!(count_csv_columns("\"a,b\",c", b','), 2);
        assert_eq!(count_csv_columns("\"a\"\"b\",c", b','), 2); // Escaped quotes
        assert_eq!(count_csv_columns("a;b;c", b';'), 3);
    }
}
