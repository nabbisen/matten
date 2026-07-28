//! Ordered and seeded, deterministic train/test splits (RFC-028 §4.4, RFC-077).

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
/// split, see [`train_test_split_seeded`], which reproduces the same rows
/// in a seed-determined shuffled order rather than the first-N/last-M split
/// this function performs.
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

/// SplitMix64 — a tiny, dependency-free deterministic PRNG (RFC-024 §6).
///
/// The constants and advance order are part of the reproducibility contract
/// (RFC-077 §6): changing them changes every user's split for a given seed.
struct SplitMix64(u64);

impl SplitMix64 {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform in `[0, bound)`. `bound` must be non-zero.
    ///
    /// Uses modulo, which carries a negligible bias for `bound` far below
    /// `u64::MAX` — acceptable here because row counts are tiny relative to
    /// `u64`, and rejection sampling would complicate the reproducibility
    /// contract for no practical gain at this scale.
    fn next_below(&mut self, bound: usize) -> usize {
        (self.next_u64() % bound as u64) as usize
    }
}

/// Splits the rows of a 2D tensor into `(train, test)` by a seeded, shuffled
/// partition.
///
/// ```text
/// n_train = floor(n_rows * train_ratio)   // identical to train_test_split
/// ```
///
/// Row order is determined by a Fisher-Yates shuffle of the row *indices*
/// (never the data itself), driven by a [`SplitMix64`] stream seeded from
/// `seed`. The first `n_train` shuffled indices become `train`; the rest
/// become `test`. Only row selection and order differ from
/// [`train_test_split`]; the output sizes match exactly for the same
/// `(x, train_ratio)`.
///
/// # Reproducibility
///
/// The same `(x, train_ratio, seed)` always produces byte-identical output,
/// on every platform and every future release of this crate. The PRNG
/// constants, the shuffle direction, and the seed-to-state mapping are part
/// of this function's observable, contract-bearing behavior (RFC-077 §6) and
/// will not change without a documented breaking change.
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
/// use matten_mlprep::train_test_split_seeded;
///
/// let x = Tensor::new(vec![10.0, 20.0, 30.0, 40.0, 50.0], &[5, 1]);
/// let (train, test) = train_test_split_seeded(&x, 0.6, 42).unwrap();
/// assert_eq!(train.shape(), &[3, 1]);
/// assert_eq!(test.shape(), &[2, 1]);
///
/// // Same seed -> byte-identical output.
/// let (train2, test2) = train_test_split_seeded(&x, 0.6, 42).unwrap();
/// assert_eq!(train.as_slice(), train2.as_slice());
/// assert_eq!(test.as_slice(), test2.as_slice());
/// ```
pub fn train_test_split_seeded(
    x: &Tensor,
    train_ratio: f64,
    seed: u64,
) -> Result<(Tensor, Tensor), MattenMlprepError> {
    let (rows, cols) = matrix_dims(x)?;

    if !train_ratio.is_finite() || train_ratio <= 0.0 || train_ratio >= 1.0 {
        return Err(MattenMlprepError::InvalidRatio(train_ratio));
    }

    let n_train = (rows as f64 * train_ratio).floor() as usize;
    if n_train == 0 {
        return Err(MattenMlprepError::EmptySplit { rows, train_ratio });
    }

    // Fisher-Yates over row indices, descending. Direction is contract-bearing.
    let mut order: Vec<usize> = (0..rows).collect();
    let mut rng = SplitMix64::new(seed);
    for i in (1..rows).rev() {
        let j = rng.next_below(i + 1);
        order.swap(i, j);
    }

    let data = x.as_slice();
    let gather = |idx: &[usize]| -> Vec<f64> {
        let mut out = Vec::with_capacity(idx.len() * cols);
        for &r in idx {
            out.extend_from_slice(&data[r * cols..(r + 1) * cols]);
        }
        out
    };

    let train = Tensor::try_new(gather(&order[..n_train]), &[n_train, cols])
        .map_err(MattenMlprepError::Matten)?;
    let test = Tensor::try_new(gather(&order[n_train..]), &[rows - n_train, cols])
        .map_err(MattenMlprepError::Matten)?;

    Ok((train, test))
}
