//! Per-column scaling (RFC-028 §4.1, §4.2).
//!
//! Both functions follow the `rows = samples`, `columns = features` convention
//! and reject constant columns explicitly via [`MattenMlprepError::ZeroVariance`]
//! rather than silently emitting a zero column. Per-column statistics reuse core
//! `matten` axis reductions (RFC-019), so `NaN`/`Inf` propagate exactly as in
//! core (`mean_axis` / `min_axis` / `max_axis`).

use crate::error::MattenMlprepError;
use crate::util::matrix_dims;
use matten::Tensor;

#[inline]
fn at(data: &[f64], i: usize, j: usize, cols: usize) -> f64 {
    data[i * cols + j]
}

/// Standardizes each column to zero mean and unit (population) standard
/// deviation: `out[i,j] = (x[i,j] - mean_j) / std_j`.
///
/// `std_j` uses the population formula (divide by `n`), matching scikit-learn's
/// `StandardScaler`.
///
/// # Errors
///
/// - [`MattenMlprepError::ExpectedMatrix`] if `x` is not rank-2.
/// - [`MattenMlprepError::ZeroVariance`] if any column is constant.
/// - [`MattenMlprepError::DynamicTensor`] (with the `dynamic` feature) if `x` is dynamic.
///
/// ```
/// use matten::Tensor;
/// use matten_mlprep::standardize_columns;
///
/// // Column 0: [1, 3] -> mean 2, std 1 -> [-1, 1]; column 1: [10, 20] -> [-1, 1].
/// let x = Tensor::new(vec![1.0, 10.0, 3.0, 20.0], &[2, 2]);
/// let z = standardize_columns(&x).unwrap();
/// assert_eq!(z.as_slice(), &[-1.0, -1.0, 1.0, 1.0]);
/// ```
pub fn standardize_columns(x: &Tensor) -> Result<Tensor, MattenMlprepError> {
    let (rows, cols) = matrix_dims(x)?;
    let data = x.as_slice();

    // Per-column means via core axis reduction (NaN propagates as in core).
    let means = x.mean_axis(0);
    let means = means.as_slice();

    let mut out = vec![0.0f64; rows * cols];
    for j in 0..cols {
        let mean = means[j];
        let var = (0..rows)
            .map(|i| {
                let d = at(data, i, j, cols) - mean;
                d * d
            })
            .sum::<f64>()
            / rows as f64;
        let std = var.sqrt();
        if std == 0.0 {
            return Err(MattenMlprepError::ZeroVariance { column: j });
        }
        for i in 0..rows {
            out[i * cols + j] = (at(data, i, j, cols) - mean) / std;
        }
    }

    Tensor::try_new(out, &[rows, cols]).map_err(MattenMlprepError::Matten)
}

/// Scales each column to the `[0, 1]` range:
/// `out[i,j] = (x[i,j] - min_j) / (max_j - min_j)`.
///
/// # Errors
///
/// - [`MattenMlprepError::ExpectedMatrix`] if `x` is not rank-2.
/// - [`MattenMlprepError::ZeroVariance`] if any column is constant (zero range).
/// - [`MattenMlprepError::DynamicTensor`] (with the `dynamic` feature) if `x` is dynamic.
///
/// ```
/// use matten::Tensor;
/// use matten_mlprep::minmax_scale_columns;
///
/// // Column 0: [0, 5, 10] -> [0, 0.5, 1].
/// let x = Tensor::new(vec![0.0, 5.0, 10.0], &[3, 1]);
/// let s = minmax_scale_columns(&x).unwrap();
/// assert_eq!(s.as_slice(), &[0.0, 0.5, 1.0]);
/// ```
pub fn minmax_scale_columns(x: &Tensor) -> Result<Tensor, MattenMlprepError> {
    let (rows, cols) = matrix_dims(x)?;
    let data = x.as_slice();

    // Per-column min/max via core axis reductions (NaN propagates as in core).
    let mins = x.min_axis(0);
    let maxs = x.max_axis(0);
    let mins = mins.as_slice();
    let maxs = maxs.as_slice();

    let mut out = vec![0.0f64; rows * cols];
    for j in 0..cols {
        let range = maxs[j] - mins[j];
        if range == 0.0 {
            return Err(MattenMlprepError::ZeroVariance { column: j });
        }
        for i in 0..rows {
            out[i * cols + j] = (at(data, i, j, cols) - mins[j]) / range;
        }
    }

    Tensor::try_new(out, &[rows, cols]).map_err(MattenMlprepError::Matten)
}
