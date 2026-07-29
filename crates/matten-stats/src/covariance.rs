//! Sample and population covariance, and correlation (RFC-078 §4.1, §4.3;
//! RFC-083 §4.1).
//!
//! [`covariance`] and [`correlation`] use the **sample** estimator
//! (`ddof = 1`), diverging deliberately from core `matten`'s population
//! `var`/`std` (`ddof = 0`) — see the crate-level docs for why.
//! [`covariance_population`] uses the **population** estimator (`ddof = 0`)
//! instead, for callers who want that convention explicitly (RFC-083 §2.1).
//! `correlation` computes its own sample standard deviations locally; it must
//! never call core's `std()`, which is population and would silently produce
//! a wrong result.

use crate::error::MattenStatsError;
use matten::Tensor;

/// Validates a two-tensor input pair (equal length, at least `min_n`
/// elements, all finite, neither dynamic) and returns the two slices plus the
/// shared element count.
///
/// `min_n` is the one difference between [`covariance`]/[`correlation`]
/// (`min_n = 2`, since their `n - 1` divisor would be zero) and
/// [`covariance_population`] (`min_n = 1`, since its `n` divisor never
/// vanishes) — every other check is shared, so the two validation paths
/// cannot silently diverge.
fn validate_pair<'a>(
    x: &'a Tensor,
    y: &'a Tensor,
    min_n: usize,
) -> Result<(&'a [f64], &'a [f64], usize), MattenStatsError> {
    if x.is_dynamic() || y.is_dynamic() {
        return Err(MattenStatsError::DynamicTensor);
    }

    let (left, right) = (x.len(), y.len());
    if left != right {
        return Err(MattenStatsError::LengthMismatch { left, right });
    }
    if left < min_n {
        return Err(MattenStatsError::Empty);
    }

    let xs = x.as_slice();
    let ys = y.as_slice();
    if xs.iter().chain(ys.iter()).any(|v| !v.is_finite()) {
        return Err(MattenStatsError::NonFiniteValue);
    }

    Ok((xs, ys, left))
}

/// Sample covariance of `x` and `y`: `Σ (xi - mean_x)(yi - mean_y) / (n - 1)`.
///
/// Values are read in row-major order; shape beyond the element count is not
/// constrained (RFC-078 §4.3) — `x` and `y` need not share a shape, only an
/// element count.
///
/// # Errors
///
/// - [`MattenStatsError::DynamicTensor`] if either input is dynamic.
/// - [`MattenStatsError::LengthMismatch`] if `x` and `y` have different element counts.
/// - [`MattenStatsError::Empty`] if either input has fewer than 2 elements.
/// - [`MattenStatsError::NonFiniteValue`] if any input value is `NaN` or infinite.
///
/// ```
/// use matten::Tensor;
/// use matten_stats::covariance;
///
/// let x = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);
/// let y = Tensor::new(vec![2.0, 4.0, 6.0], &[3]);
/// let cov = covariance(&x, &y).unwrap();
/// assert!((cov - 2.0).abs() < 1e-9); // sample (n-1) covariance
/// ```
pub fn covariance(x: &Tensor, y: &Tensor) -> Result<f64, MattenStatsError> {
    let (xs, ys, n) = validate_pair(x, y, 2)?;

    let mean_x = xs.iter().sum::<f64>() / n as f64;
    let mean_y = ys.iter().sum::<f64>() / n as f64;

    let sum: f64 = xs
        .iter()
        .zip(ys.iter())
        .map(|(&xi, &yi)| (xi - mean_x) * (yi - mean_y))
        .sum();

    Ok(sum / (n as f64 - 1.0))
}

/// Population covariance of `x` and `y`: `Σ (xi - mean_x)(yi - mean_y) / n`
/// (RFC-083 §4.1, §4.2).
///
/// This is the `ddof = 0` counterpart to [`covariance`]'s `ddof = 1`
/// (sample) estimator — matching NumPy's `cov(..., ddof=0)`. Unlike
/// `covariance`, a single-element pair is well-defined here (its divisor is
/// `n`, not `n - 1`) and returns `0.0`.
///
/// Values are read in row-major order; shape beyond the element count is not
/// constrained (RFC-078 §4.3) — `x` and `y` need not share a shape, only an
/// element count.
///
/// # Errors
///
/// - [`MattenStatsError::DynamicTensor`] if either input is dynamic.
/// - [`MattenStatsError::LengthMismatch`] if `x` and `y` have different element counts.
/// - [`MattenStatsError::Empty`] if either input has no elements.
/// - [`MattenStatsError::NonFiniteValue`] if any input value is `NaN` or infinite.
///
/// ```
/// use matten::Tensor;
/// use matten_stats::covariance_population;
///
/// let x = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);
/// let y = Tensor::new(vec![2.0, 4.0, 6.0], &[3]);
/// let cov = covariance_population(&x, &y).unwrap();
/// assert!((cov - 4.0 / 3.0).abs() < 1e-9); // population (n) covariance
///
/// // A single element is well-defined here, unlike `covariance`.
/// let one = Tensor::new(vec![5.0], &[1]);
/// assert_eq!(covariance_population(&one, &one).unwrap(), 0.0);
/// ```
pub fn covariance_population(x: &Tensor, y: &Tensor) -> Result<f64, MattenStatsError> {
    let (xs, ys, n) = validate_pair(x, y, 1)?;

    let mean_x = xs.iter().sum::<f64>() / n as f64;
    let mean_y = ys.iter().sum::<f64>() / n as f64;

    let sum: f64 = xs
        .iter()
        .zip(ys.iter())
        .map(|(&xi, &yi)| (xi - mean_x) * (yi - mean_y))
        .sum();

    Ok(sum / n as f64)
}

/// Pearson correlation of `x` and `y`: `cov(x, y) / (std_sample(x) * std_sample(y))`.
///
/// The `n - 1` factors in the sample covariance and the two sample standard
/// deviations cancel algebraically, so `correlation` is identical whether
/// computed with `ddof = 0` or `ddof = 1` — only [`covariance`] is a genuine
/// policy decision (RFC-078 §4.1). Both standard deviations are computed
/// locally in this function; it never calls core `matten`'s `std()`, which is
/// population (`ddof = 0`) and would silently produce a wrong result if mixed
/// in here.
///
/// # Errors
///
/// - [`MattenStatsError::DynamicTensor`] if either input is dynamic.
/// - [`MattenStatsError::LengthMismatch`] if `x` and `y` have different element counts.
/// - [`MattenStatsError::Empty`] if either input has fewer than 2 elements.
/// - [`MattenStatsError::NonFiniteValue`] if any input value is `NaN` or infinite.
/// - [`MattenStatsError::ZeroVariance`] if either input has zero variance — an
///   explicit error rather than a silent `NaN` (RFC-078 §4.3).
///
/// ```
/// use matten::Tensor;
/// use matten_stats::correlation;
///
/// let x = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[4]);
/// let y = Tensor::new(vec![2.0, 4.0, 6.0, 8.0], &[4]); // y = 2x
/// let r = correlation(&x, &y).unwrap();
/// assert!((r - 1.0).abs() < 1e-9);
/// ```
pub fn correlation(x: &Tensor, y: &Tensor) -> Result<f64, MattenStatsError> {
    let (xs, ys, n) = validate_pair(x, y, 2)?;

    let mean_x = xs.iter().sum::<f64>() / n as f64;
    let mean_y = ys.iter().sum::<f64>() / n as f64;

    let mut cov_sum = 0.0;
    let mut var_x_sum = 0.0;
    let mut var_y_sum = 0.0;
    for (&xi, &yi) in xs.iter().zip(ys.iter()) {
        let dx = xi - mean_x;
        let dy = yi - mean_y;
        cov_sum += dx * dy;
        var_x_sum += dx * dx;
        var_y_sum += dy * dy;
    }

    if var_x_sum == 0.0 || var_y_sum == 0.0 {
        return Err(MattenStatsError::ZeroVariance);
    }

    let denom = n as f64 - 1.0;
    let std_x = (var_x_sum / denom).sqrt();
    let std_y = (var_y_sum / denom).sqrt();

    Ok((cov_sum / denom) / (std_x * std_y))
}
