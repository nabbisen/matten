//! Linear-interpolation quantile (RFC-078 §4.2).

use crate::error::MattenStatsError;
use matten::Tensor;

/// The `q`-quantile of `x`, using linear interpolation between the two
/// nearest ranks of the sorted sample — NumPy's `"linear"` method:
///
/// ```text
/// sort the values ascending
/// h = (n - 1) * q
/// lo = floor(h), hi = ceil(h)
/// result = v[lo] + (h - lo) * (v[hi] - v[lo])
/// ```
///
/// `q = 0.0` returns the minimum, `q = 1.0` the maximum, `q = 0.5` the
/// median. `x` is read into a sorted copy; the caller's tensor is never
/// mutated or reordered.
///
/// # Errors
///
/// - [`MattenStatsError::DynamicTensor`] if `x` is dynamic.
/// - [`MattenStatsError::Empty`] if `x` has no elements.
/// - [`MattenStatsError::InvalidQuantile`] if `q` is not finite or not in `[0.0, 1.0]`.
/// - [`MattenStatsError::NonFiniteValue`] if any value in `x` is `NaN` or infinite.
///
/// ```
/// use matten::Tensor;
/// use matten_stats::quantile;
///
/// let x = Tensor::new(vec![1.0, 3.0, 2.0, 4.0, 5.0], &[5]);
/// assert_eq!(quantile(&x, 0.0).unwrap(), 1.0); // min
/// assert_eq!(quantile(&x, 1.0).unwrap(), 5.0); // max
/// assert_eq!(quantile(&x, 0.5).unwrap(), 3.0); // median
/// ```
pub fn quantile(x: &Tensor, q: f64) -> Result<f64, MattenStatsError> {
    if x.is_dynamic() {
        return Err(MattenStatsError::DynamicTensor);
    }
    if x.is_empty() {
        return Err(MattenStatsError::Empty);
    }
    if !q.is_finite() || !(0.0..=1.0).contains(&q) {
        return Err(MattenStatsError::InvalidQuantile(q));
    }

    let mut values = x.to_vec();
    if values.iter().any(|v| !v.is_finite()) {
        return Err(MattenStatsError::NonFiniteValue);
    }
    values.sort_by(f64::total_cmp);

    let n = values.len();
    let h = (n - 1) as f64 * q;
    let lo = h.floor() as usize;
    let hi = h.ceil() as usize;

    if lo == hi {
        Ok(values[lo])
    } else {
        Ok(values[lo] + (h - lo as f64) * (values[hi] - values[lo]))
    }
}
