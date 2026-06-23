//! Crate-local error type for `matten-data` (RFC-034 §6).
//!
//! `MattenDataError` is the single error type returned by every fallible
//! `matten-data` API. It is crate-local: core `matten::MattenError` is wrapped
//! (see [`MattenDataError::Matten`]) but never extended for table-specific
//! failures (RFC-014, RFC-033 §10).

use std::fmt;
use std::path::PathBuf;

/// Errors produced by `matten-data` table ingestion and conversion.
///
/// All external-input APIs return this type; malformed input never panics
/// (RFC-035 §1). Error messages include row/column context where practical.
/// Row numbers are **one-based CSV line numbers** (the header is line 1, so the
/// first data row is line 2).
#[derive(Debug)]
#[non_exhaustive]
pub enum MattenDataError {
    /// A CSV structural problem reported by the parser or by header validation
    /// (for example an empty header name).
    Csv {
        /// Human-readable description of the problem.
        message: String,
    },
    /// An I/O error while reading a CSV path.
    Io {
        /// The path that failed to read.
        path: PathBuf,
        /// The underlying I/O error.
        source: std::io::Error,
    },
    /// The input was empty (no header row).
    EmptyInput,
    /// A requested column name does not exist in the table.
    MissingColumn {
        /// The requested column name.
        name: String,
    },
    /// The CSV header contains a duplicate column name.
    DuplicateColumn {
        /// The duplicated column name.
        name: String,
    },
    /// The same column name was requested more than once in a selection.
    DuplicateSelection {
        /// The duplicated selection name.
        name: String,
    },
    /// A data row has a different number of cells than the header.
    RaggedRow {
        /// One-based CSV line number of the offending row.
        row: usize,
        /// Number of columns expected (from the header).
        expected: usize,
        /// Number of cells actually found.
        actual: usize,
    },
    /// A cell could not be converted to `f64` during numeric conversion.
    NonNumericValue {
        /// Column name.
        column: String,
        /// One-based CSV line number.
        row: usize,
        /// The offending cell value, rendered as text.
        value: String,
    },
    /// A missing cell remained during numeric conversion (fill it first).
    MissingValue {
        /// Column name.
        column: String,
        /// One-based CSV line number.
        row: usize,
    },
    /// A column selection or conversion was attempted with no columns.
    EmptySelection,
    /// A wrapped core `matten` error (for example from `Tensor` construction).
    Matten(matten::MattenError),
}

impl fmt::Display for MattenDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MattenDataError::Csv { message } => {
                write!(f, "matten-data CSV error: {message}")
            }
            MattenDataError::Io { path, source } => {
                write!(
                    f,
                    "matten-data I/O error reading {}: {source}",
                    path.display()
                )
            }
            MattenDataError::EmptyInput => {
                write!(
                    f,
                    "matten-data error: input is empty (a header row is required)"
                )
            }
            MattenDataError::MissingColumn { name } => {
                write!(f, "matten-data error: column \"{name}\" does not exist")
            }
            MattenDataError::DuplicateColumn { name } => {
                write!(f, "matten-data error: duplicate header column \"{name}\"")
            }
            MattenDataError::DuplicateSelection { name } => {
                write!(
                    f,
                    "matten-data error: column \"{name}\" was selected more than once"
                )
            }
            MattenDataError::RaggedRow {
                row,
                expected,
                actual,
            } => write!(
                f,
                "matten-data error: row {row} has {actual} cells but the header has {expected} columns"
            ),
            MattenDataError::NonNumericValue { column, row, value } => write!(
                f,
                "matten-data numeric conversion error: column \"{column}\", row {row} contains \"{value}\", \
                 which cannot be converted to f64. Fill or clean the column before calling try_numeric()."
            ),
            MattenDataError::MissingValue { column, row } => write!(
                f,
                "matten-data numeric conversion error: column \"{column}\", row {row} is missing. \
                 Fill missing values (e.g. with fill_missing) before calling try_numeric()."
            ),
            MattenDataError::EmptySelection => {
                write!(f, "matten-data error: no columns were selected")
            }
            MattenDataError::Matten(e) => write!(f, "matten-data error: {e}"),
        }
    }
}

impl std::error::Error for MattenDataError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MattenDataError::Io { source, .. } => Some(source),
            MattenDataError::Matten(e) => Some(e),
            _ => None,
        }
    }
}

impl From<matten::MattenError> for MattenDataError {
    fn from(e: matten::MattenError) -> Self {
        MattenDataError::Matten(e)
    }
}
