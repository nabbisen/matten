//! Ordered, deterministic train/test split (RFC-028 §4.4).

use crate::error::MattenMlprepError;
use crate::util::matrix_dims;
use matten::Tensor;

/// Splits the rows of a 2D tensor into `(train, test)` by an ordered,
/// deterministic partition — **no shuffling**.
///
/// ```text
/// n_train = floor(n_rows * train_ratio)
/// train   = rows[0 .. n_train]
/// test    = rows[n_train .. n_rows]
/// ```
///
/// The split is fully deterministic and reproducible. If you need a randomized
/// split, shuffle the rows yourself first (a seeded variant is planned but not
/// in this release; see RFC-024 §6).
///
/// # Errors
///
/// - [`MattenMlprepError::ExpectedMatrix`] if `x` is not rank-2.
/// - [`MattenMlprepError::InvalidRatio`] if `train_ratio` is not finite or not in `(0.0, 1.0)`.
/// - [`MattenMlprepError::EmptySplit`] if `floor(rows * train_ratio) == 0`.
/// - [`MattenMlprepError::DynamicTensor`] (with the `dynamic` feature) if `x` is dynamic.
///
/// ```
/// use matten::Tensor;
/// use matten_mlprep::train_test_split;
///
/// // 4 rows, 1 feature; 0.75 -> 3 train rows, 1 test row.
/// let x = Tensor::new(vec![10.0, 20.0, 30.0, 40.0], &[4, 1]);
/// let (train, test) = train_test_split(&x, 0.75).unwrap();
/// assert_eq!(train.shape(), &[3, 1]);
/// assert_eq!(test.shape(), &[1, 1]);
/// assert_eq!(train.as_slice(), &[10.0, 20.0, 30.0]);
/// assert_eq!(test.as_slice(), &[40.0]);
/// ```
pub fn train_test_split(
    x: &Tensor,
    train_ratio: f64,
) -> Result<(Tensor, Tensor), MattenMlprepError> {
    let (rows, cols) = matrix_dims(x)?;

    if !train_ratio.is_finite() || train_ratio <= 0.0 || train_ratio >= 1.0 {
        return Err(MattenMlprepError::InvalidRatio(train_ratio));
    }

    let n_train = (rows as f64 * train_ratio).floor() as usize;
    // For any ratio < 1.0, n_train <= rows - 1, so the test set is never empty.
    // The only failure is an empty train set.
    if n_train == 0 {
        return Err(MattenMlprepError::EmptySplit { rows, train_ratio });
    }
    let n_test = rows - n_train;

    let data = x.as_slice();
    let split = n_train * cols;

    let train = Tensor::try_new(data[..split].to_vec(), &[n_train, cols])
        .map_err(MattenMlprepError::Matten)?;
    let test = Tensor::try_new(data[split..].to_vec(), &[n_test, cols])
        .map_err(MattenMlprepError::Matten)?;

    Ok((train, test))
}
