//! Error types for the csvmd crate.

use std::fmt;

/// Errors that can occur during CSV to Markdown conversion.
#[derive(Debug, thiserror::Error)]
pub enum CsvMdError {
    /// IO error when reading input or writing output.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// CSV parsing error.
    #[error("CSV parsing error at {location}: {message}")]
    Csv {
        /// The underlying CSV error message.
        message: String,
        /// Location information if available.
        location: String,
    },

    /// Error during string formatting operations.
    #[error("Formatting error: {0}")]
    Fmt(#[from] fmt::Error),
}

impl From<csv::Error> for CsvMdError {
    fn from(err: csv::Error) -> Self {
        let location = match err.position() {
            Some(pos) => format!("line {}, record {}", pos.line(), pos.record()),
            None => "unknown location".to_string(),
        };

        CsvMdError::Csv {
            message: err.to_string(),
            location,
        }
    }
}

/// Result type alias for operations that can fail with CsvMdError.
pub type Result<T> = std::result::Result<T, CsvMdError>;
