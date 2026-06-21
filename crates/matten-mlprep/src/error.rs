//! Error type for `matten-mlprep` (RFC-024 §7, RFC-028 §5).
//!
//! The crate defines its own error type rather than growing core
//! [`matten::MattenError`] (RFC-022 §8). Every entry point returns `Result`; a
//! dynamic tensor returns [`MattenMlprepError::DynamicTensor`] rather than
//! panicking.

use std::fmt;

/// Errors produced by `matten-mlprep` preprocessing functions.
///
/// `#[non_exhaustive]` so future variants are not a breaking change.
#[derive(Debug)]
#[non_exhaustive]
pub enum MattenMlprepError {
    /// A dynamic (`Element`) tensor was passed. Convert it to a numeric tensor
    /// first with `Tensor::try_numeric()`.
    DynamicTensor,
    /// The input was not rank-2. `matten-mlprep` operates only on matrices with
    /// the convention `rows = samples`, `columns = features`.
    ExpectedMatrix {
        /// The shape that was provided.
        shape: Vec<usize>,
    },
    /// `train_ratio` was not a finite value in the open interval `(0.0, 1.0)`.
    InvalidRatio(f64),
    /// The requested split would leave the train set empty
    /// (`floor(rows * train_ratio) == 0`).
    EmptySplit {
        /// Number of rows (samples) in the input.
        rows: usize,
        /// The requested train ratio.
        train_ratio: f64,
    },
    /// A column has zero variance / range and therefore cannot be scaled.
    ZeroVariance {
        /// The index of the offending column (feature).
        column: usize,
    },
    /// Core `matten` rejected a constructed result.
    Matten(matten::MattenError),
}

impl fmt::Display for MattenMlprepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MattenMlprepError::DynamicTensor => write!(
                f,
                "matten-mlprep error: dynamic tensors are not supported; call \
                 try_numeric() to convert to a numeric tensor first"
            ),
            MattenMlprepError::ExpectedMatrix { shape } => write!(
                f,
                "matten-mlprep error: expected a rank-2 tensor (rows = samples, \
                 columns = features), got shape {shape:?}"
            ),
            MattenMlprepError::InvalidRatio(r) => write!(
                f,
                "matten-mlprep error: train_ratio must be a finite value in (0.0, 1.0), got {r}"
            ),
            MattenMlprepError::EmptySplit { rows, train_ratio } => write!(
                f,
                "matten-mlprep error: a split of {rows} row(s) at ratio {train_ratio} \
                 leaves the train set empty; use a larger ratio or more rows"
            ),
            MattenMlprepError::ZeroVariance { column } => write!(
                f,
                "matten-mlprep error: column {column} has zero variance/range and \
                 cannot be scaled; drop or handle the constant column first"
            ),
            MattenMlprepError::Matten(e) => {
                write!(f, "matten-mlprep error: matten rejected the result: {e}")
            }
        }
    }
}

impl std::error::Error for MattenMlprepError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MattenMlprepError::Matten(e) => Some(e),
            _ => None,
        }
    }
}
