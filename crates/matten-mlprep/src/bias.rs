//! Bias (intercept) column insertion (RFC-028 §4.3).

use crate::error::MattenMlprepError;
use crate::util::matrix_dims;
use matten::Tensor;

/// Prepends a constant `1.0` bias column: `[n, m] -> [n, m+1]`.
///
/// Column `0` of the result is all ones; the original features shift to columns
/// `1..=m`. Prepending (intercept at index 0) matches the common `w · [1, x]`
/// convention, so the first weight is the intercept.
///
/// # Errors
///
/// - [`MattenMlprepError::ExpectedMatrix`] if `x` is not rank-2.
/// - [`MattenMlprepError::DynamicTensor`] (with the `dynamic` feature) if `x` is dynamic.
///
/// ```
/// use matten::Tensor;
/// use matten_mlprep::add_bias_column;
///
/// let x = Tensor::new(vec![2.0, 3.0, 4.0, 5.0], &[2, 2]);
/// let b = add_bias_column(&x).unwrap();
/// assert_eq!(b.shape(), &[2, 3]);
/// // row 0 -> [1, 2, 3], row 1 -> [1, 4, 5]
/// assert_eq!(b.as_slice(), &[1.0, 2.0, 3.0, 1.0, 4.0, 5.0]);
/// ```
pub fn add_bias_column(x: &Tensor) -> Result<Tensor, MattenMlprepError> {
    let (rows, cols) = matrix_dims(x)?;
    let data = x.as_slice();

    let new_cols = cols + 1;
    let mut out = vec![0.0f64; rows * new_cols];
    for i in 0..rows {
        out[i * new_cols] = 1.0; // bias at column 0
        for j in 0..cols {
            out[i * new_cols + 1 + j] = data[i * cols + j];
        }
    }

    Tensor::try_new(out, &[rows, new_cols]).map_err(MattenMlprepError::Matten)
}
