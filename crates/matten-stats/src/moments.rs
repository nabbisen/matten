//! Skewness and excess kurtosis (RFC-083 §4).
//!
//! Both are **uncorrected** (`g1`/`g2`) estimators — SciPy's `skew(bias=True)`
//! and `kurtosis(fisher=True, bias=True)` defaults, and NumPy's / R's `e1071`
//! type-1 defaults. This differs from pandas' `.skew()`/`.kurt()`, which are
//! bias-corrected — see the crate-level docs for the full estimator
//! convention table (RFC-083 §4.1). `kurtosis` reports the **excess** value
//! (a normal distribution scores `0.0`, not `3.0`), Fisher's definition.

use crate::error::MattenStatsError;
use matten::Tensor;

/// Validates a single-tensor input (at least `min_n` elements, finite, not
/// dynamic) and returns the slice plus the element count.
fn validate_single(x: &Tensor, min_n: usize) -> Result<(&[f64], usize), MattenStatsError> {
    if x.is_dynamic() {
        return Err(MattenStatsError::DynamicTensor);
    }

    let n = x.len();
    if n < min_n {
        return Err(MattenStatsError::Empty);
    }

    let xs = x.as_slice();
    if xs.iter().any(|v| !v.is_finite()) {
        return Err(MattenStatsError::NonFiniteValue);
    }

    Ok((xs, n))
}

/// Computes the second, third, and fourth central moments of `xs` in a
/// single pass over the mean-centered deviations: `m_k = Σ (xi - mean)^k / n`.
///
/// `skewness` and `kurtosis` both need `m2` plus one higher moment, so this is
/// shared rather than computed twice. This never calls core `matten`'s
/// `var()`/`std()` — those are population (`ddof = 0`), which happens to
/// match what `m2` needs here, but `covariance.rs` already records the
/// standing rule that this crate computes its own moments locally rather
/// than depending on core's estimator choice, so a future change to core
/// cannot silently alter a statistic here.
fn central_moments(xs: &[f64], n: usize) -> (f64, f64, f64) {
    let n_f = n as f64;
    let mean = xs.iter().sum::<f64>() / n_f;

    let (mut m2, mut m3, mut m4) = (0.0, 0.0, 0.0);
    for &xi in xs {
        let d = xi - mean;
        let d2 = d * d;
        m2 += d2;
        m3 += d2 * d;
        m4 += d2 * d2;
    }

    (m2 / n_f, m3 / n_f, m4 / n_f)
}

/// Skewness of `x`: `m3 / m2^(3/2)` (RFC-083 §4.1, §4.2) — the **uncorrected**
/// `g1` estimator (SciPy's `skew(bias=True)` default), not pandas' `.skew()`
/// (which bias-corrects and would return a different number for the same
/// input).
///
/// Values are read in row-major order; shape beyond the element count is not
/// constrained.
///
/// # Errors
///
/// - [`MattenStatsError::DynamicTensor`] if `x` is dynamic.
/// - [`MattenStatsError::Empty`] if `x` has fewer than 2 elements.
/// - [`MattenStatsError::NonFiniteValue`] if any value in `x` is `NaN` or infinite.
/// - [`MattenStatsError::ZeroVariance`] if `x` has zero variance — an
///   explicit error rather than a silent `NaN` from `0.0 / 0.0` (consistent
///   with [`crate::correlation`], RFC-078 §4.3).
///
/// ```
/// use matten::Tensor;
/// use matten_stats::skewness;
///
/// // Symmetric input: skewness is exactly 0.0.
/// let symmetric = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], &[5]);
/// assert_eq!(skewness(&symmetric).unwrap(), 0.0);
/// ```
pub fn skewness(x: &Tensor) -> Result<f64, MattenStatsError> {
    let (xs, n) = validate_single(x, 2)?;
    let (m2, m3, _m4) = central_moments(xs, n);

    if m2 == 0.0 {
        return Err(MattenStatsError::ZeroVariance);
    }

    Ok(m3 / m2.powf(1.5))
}

/// Excess kurtosis of `x`: `m4 / m2^2 - 3.0` (RFC-083 §4.1, §4.2) — the
/// **uncorrected** `g2` estimator (SciPy's `kurtosis(fisher=True, bias=True)`
/// default), not pandas' `.kurt()` (which bias-corrects and would return a
/// different number for the same input).
///
/// This is **excess** kurtosis (Fisher's definition): a normal distribution
/// scores `0.0`, not the raw (Pearson) `3.0`. The `- 3.0` is not optional and
/// must not be dropped.
///
/// Values are read in row-major order; shape beyond the element count is not
/// constrained.
///
/// # Errors
///
/// - [`MattenStatsError::DynamicTensor`] if `x` is dynamic.
/// - [`MattenStatsError::Empty`] if `x` has fewer than 2 elements.
/// - [`MattenStatsError::NonFiniteValue`] if any value in `x` is `NaN` or infinite.
/// - [`MattenStatsError::ZeroVariance`] if `x` has zero variance — an
///   explicit error rather than a silent `NaN` from `0.0 / 0.0` (consistent
///   with [`crate::correlation`], RFC-078 §4.3).
///
/// ```
/// use matten::Tensor;
/// use matten_stats::kurtosis;
///
/// // m2 = 2, m4 = 6.8; raw ratio 6.8 / 4 = 1.7; excess = 1.7 - 3.0 = -1.3.
/// let x = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], &[5]);
/// assert_eq!(kurtosis(&x).unwrap(), -1.3);
/// ```
pub fn kurtosis(x: &Tensor) -> Result<f64, MattenStatsError> {
    let (xs, n) = validate_single(x, 2)?;
    let (m2, _m3, m4) = central_moments(xs, n);

    if m2 == 0.0 {
        return Err(MattenStatsError::ZeroVariance);
    }

    Ok(m4 / (m2 * m2) - 3.0)
}
