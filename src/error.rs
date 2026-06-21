//! Public error model for `matten`.
//!
//! `matten` has two error zones (see the crate docs and RFC-005):
//!
//! - **Panic zone**: local, developer-authored convenience APIs may panic with
//!   an actionable `matten <category> error in <operation>: ...` message.
//! - **Result zone**: every external boundary (parsing, file I/O, user-driven
//!   construction) returns [`Result`](std::result::Result) carrying a [`MattenError`].
//!
//! [`MattenError`] is the single public error type. It derives **only `Debug`**,
//! because [`MattenError::Io`] embeds [`std::io::Error`], which is neither `Clone`
//! nor `PartialEq`. Match errors by variant (`matches!`), never by `==`.

use std::fmt;

/// The single public error type for `matten`.
///
/// The variant set is canonical (RFC-005). It derives only `Debug`; it is not
/// `Clone`, `PartialEq`, or `Eq`. Tests should match by variant, e.g.
/// `matches!(err, MattenError::Shape { .. })`.
#[derive(Debug)]
#[non_exhaustive]
pub enum MattenError {
    /// Invalid shape, length mismatch, or a constructor argument (such as an
    /// `arange` step / finite bound) that determines the produced tensor shape.
    Shape {
        /// The operation that failed, e.g. `"new"`, `"reshape"`, `"arange"`.
        operation: &'static str,
        /// Human-readable, actionable detail.
        message: String,
    },
    /// Two shapes could not be broadcast together (never folded into `Shape`).
    Broadcast {
        /// Left operand shape.
        left: Vec<usize>,
        /// Right operand shape.
        right: Vec<usize>,
    },
    /// A shape product overflowed or an allocation exceeded a safety limit.
    Allocation {
        /// The requested element count (or the value that overflowed).
        requested_elements: usize,
        /// Human-readable detail.
        message: String,
    },
    /// A slice specification (builder or `slice_str`) was invalid.
    Slice {
        /// The original slice string, when the error came from `slice_str`.
        input: Option<String>,
        /// Human-readable detail.
        message: String,
    },
    /// A data-format parse error (JSON or CSV). Row/column context, when
    /// available, is carried inside `message`.
    Parse {
        /// Which data format produced the error.
        format: DataFormat,
        /// Human-readable detail.
        message: String,
    },
    /// A file I/O error while loading data.
    Io {
        /// The path that was being read.
        path: std::path::PathBuf,
        /// The underlying I/O error.
        source: std::io::Error,
    },
    /// An operation or input is not supported in this build or phase (for
    /// example, a feature-gated capability that is disabled).
    Unsupported {
        /// The operation that is unsupported.
        operation: &'static str,
        /// Human-readable detail.
        message: String,
    },
}

impl fmt::Display for MattenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MattenError::Shape { operation, message } => {
                write!(f, "matten shape error in {operation}: {message}")
            }
            MattenError::Broadcast { left, right } => write!(
                f,
                "matten broadcast error: shapes {left:?} and {right:?} are not compatible"
            ),
            MattenError::Allocation {
                requested_elements,
                message,
            } => write!(
                f,
                "matten allocation error: {message} (requested {requested_elements} elements)"
            ),
            MattenError::Slice { input, message } => match input {
                Some(spec) => write!(f, "matten slice error for {spec:?}: {message}"),
                None => write!(f, "matten slice error: {message}"),
            },
            MattenError::Parse { format, message } => {
                write!(f, "matten {format} parse error: {message}")
            }
            MattenError::Io { path, source } => {
                write!(f, "matten io error for {}: {source}", path.display())
            }
            MattenError::Unsupported { operation, message } => {
                write!(f, "matten unsupported error in {operation}: {message}")
            }
        }
    }
}

impl std::error::Error for MattenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MattenError::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// The external data format associated with a [`MattenError::Parse`].
///
/// This is a sanctioned public root export because it appears in the public
/// error type. It is `#[non_exhaustive]` so that future formats can be added
/// without a breaking change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DataFormat {
    /// JSON input (`from_json` / `load_json`).
    Json,
    /// CSV input (`from_csv` / `load_csv`).
    Csv,
}

impl fmt::Display for DataFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataFormat::Json => f.write_str("json"),
            DataFormat::Csv => f.write_str("csv"),
        }
    }
}
