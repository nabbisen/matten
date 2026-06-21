//! Standard conversion traits for [`Tensor`](crate::Tensor) (RFC-004).
//!
//! Panic zone:
//! - [`From<Vec<f64>>`] — creates a 1-D tensor.
//! - [`From<Vec<Vec<f64>>>`] — convenience wrapper for rectangular row data;
//!   panics on ragged input with an actionable message.
//! - [`From<Tensor> for Vec<f64>`] — consuming flat extraction.
//! - [`From<&Tensor> for Vec<f64>`] — borrowing flat extraction (clones data).
//!
//! Result zone:
//! - [`TryFrom<Tensor> for Vec<Vec<f64>>`] — fails for non-rank-2 tensors.
//! - [`Tensor::try_from_rows`] — boundary-safe rectangular row construction.

use crate::{MattenError, Tensor};

// ---- From<Vec<f64>> ---------------------------------------------------

impl From<Vec<f64>> for Tensor {
    /// Creates a 1-D tensor from a flat vector; shape is `[len]`.
    ///
    /// # Panics
    ///
    /// Panics only if the resulting `[len]` shape is somehow invalid (not
    /// possible for non-empty `Vec<f64>` unless `len` is 0, which triggers the
    /// zero-sized-dimension policy). An empty vector will panic.
    fn from(data: Vec<f64>) -> Self {
        let len = data.len();
        Tensor::new(data, &[len])
    }
}

// ---- From<Vec<Vec<f64>>> (panic wrapper) -------------------------------

impl From<Vec<Vec<f64>>> for Tensor {
    /// Creates a 2-D tensor from rectangular row data; shape is `[rows, cols]`.
    ///
    /// This is a convenience for trusted literals and examples. It **panics** on
    /// ragged rows, an empty outer vector, or any row with zero columns. Use
    /// [`Tensor::try_from_rows`] for recoverable, boundary-safe construction.
    ///
    /// # Panics
    ///
    /// Panics on ragged rows, an empty outer vector, or any zero-length row.
    fn from(rows: Vec<Vec<f64>>) -> Self {
        Tensor::try_from_rows(rows).unwrap_or_else(|e| panic!("{e}"))
    }
}

// ---- From<Tensor> for Vec<f64> ----------------------------------------

impl From<Tensor> for Vec<f64> {
    /// Consumes the tensor and returns the flat, row-major data. No copy.
    fn from(t: Tensor) -> Vec<f64> {
        #[cfg(feature = "dynamic")]
        t.panic_if_dynamic("From<Tensor> for Vec<f64>");
        t.data
    }
}

// ---- From<&Tensor> for Vec<f64> ---------------------------------------

impl From<&Tensor> for Vec<f64> {
    /// Returns an owned copy of the tensor's flat, row-major data.
    fn from(t: &Tensor) -> Vec<f64> {
        #[cfg(feature = "dynamic")]
        t.panic_if_dynamic("From<&Tensor> for Vec<f64>");
        t.data.clone()
    }
}

// ---- TryFrom<Tensor> for Vec<Vec<f64>> --------------------------------

impl TryFrom<Tensor> for Vec<Vec<f64>> {
    type Error = MattenError;

    /// Consumes a 2-D tensor and returns rows of owned data.
    ///
    /// # Errors
    ///
    /// Returns [`MattenError::Shape`] if the tensor is not rank 2.
    fn try_from(t: Tensor) -> Result<Vec<Vec<f64>>, MattenError> {
        #[cfg(feature = "dynamic")]
        if t.is_dynamic() {
            return Err(MattenError::Unsupported {
                operation: "TryFrom<Tensor> for Vec<Vec<f64>>",
                message: "dynamic tensors cannot be converted to Vec<Vec<f64>>; ".to_string()
                    + "call try_numeric() first",
            });
        }
        if t.ndim() != 2 {
            return Err(MattenError::Shape {
                operation: "TryFrom<Tensor> for Vec<Vec<f64>>",
                message: format!(
                    "expected a rank-2 tensor but found rank {} (shape {:?})",
                    t.ndim(),
                    t.shape()
                ),
            });
        }
        let rows = t.shape[0];
        let cols = t.shape[1];
        let mut data = t.data.into_iter();
        Ok((0..rows)
            .map(|_| (0..cols).map(|_| data.next().unwrap()).collect())
            .collect())
    }
}

// ---- helper reused by tensor.rs ----------------------------------------

/// Validates rectangular row data and returns `(flat_data, shape)`.
///
/// Rejects: empty outer vector, any row with zero columns, ragged rows.
pub(crate) fn flatten_rectangular(
    rows: Vec<Vec<f64>>,
    operation: &'static str,
) -> Result<(Vec<f64>, Vec<usize>), MattenError> {
    let row_count = rows.len();
    if row_count == 0 {
        return Err(MattenError::Shape {
            operation,
            message: "cannot create a tensor from an empty row list (zero-sized dimensions are not supported in the current matten shape model)".into(),
        });
    }
    let col_count = rows[0].len();
    if col_count == 0 {
        return Err(MattenError::Shape {
            operation,
            message: "rows must have at least one column (zero-sized dimensions are not supported in the current matten shape model)".into(),
        });
    }
    let capacity =
        row_count
            .checked_mul(col_count)
            .ok_or_else(|| crate::MattenError::Allocation {
                requested_elements: usize::MAX,
                message: format!(
                    "flatten_rectangular: {row_count} × {col_count} rows overflows usize"
                ),
            })?;
    let mut flat = Vec::with_capacity(capacity);
    for (i, row) in rows.into_iter().enumerate() {
        if row.len() != col_count {
            return Err(MattenError::Shape {
                operation,
                message: format!(
                    "ragged rows: row 0 has {col_count} columns but row {i} has {} columns",
                    row.len()
                ),
            });
        }
        flat.extend(row);
    }
    Ok((flat, vec![row_count, col_count]))
}
