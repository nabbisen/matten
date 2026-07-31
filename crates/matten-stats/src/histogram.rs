//! Histogram bin-selection policy (RFC-090 §4): the caller chooses `bins`;
//! there is no automatic rule.

use crate::error::MattenStatsError;
use matten::{MattenLimits, Tensor};

/// A histogram's per-bin counts and the edges that define them.
///
/// `counts.len() == bins` and `edges.len() == bins + 1`. Bin `i` covers
/// `[edges[i], edges[i + 1])`, except the **last** bin, which is closed at
/// the top: `[edges[bins - 1], edges[bins]]` — see [`histogram`]'s docs for
/// why.
#[derive(Debug, Clone, PartialEq)]
pub struct Histogram {
    /// Per-bin element counts, one per bin.
    pub counts: Vec<usize>,
    /// Bin boundaries, `bins + 1` of them, evenly spaced from `min(x)` to
    /// `max(x)`.
    pub edges: Vec<f64>,
}

/// Bins `x` into `bins` equal-width intervals spanning `[min(x), max(x)]`.
///
/// **There is no automatic bin-count rule** — not Sturges, not
/// Freedman-Diaconis, not Scott, no `"auto"`. `bins` is a required argument:
/// bin count is a genuine analytical choice, and picking one for the caller
/// would teach the wrong lesson about what a histogram is (RFC-090 §4.1).
///
/// The **last bin is closed** at the top (`[edges[bins - 1], edges[bins]]`,
/// inclusive), unlike every other bin (`[edges[i], edges[i + 1])`,
/// half-open). Without this, `max(x)` would fall in no bin and silently
/// vanish from the counts — matching NumPy here, since the alternative is a
/// *silent* wrong answer (RFC-087 §6).
///
/// # Errors
///
/// - [`MattenStatsError::DynamicTensor`] if `x` is dynamic.
/// - [`MattenStatsError::InvalidBinCount`] if `bins == 0`.
/// - [`MattenStatsError::Empty`] if `x` has no elements.
/// - [`MattenStatsError::NonFiniteValue`] if any value in `x` is `NaN` or
///   infinite, **or** if `x` is entirely finite but `max(x) - min(x)`
///   overflows to infinity (extreme-magnitude inputs) — an overflowing range
///   would otherwise poison every edge with `NaN`/`inf` and return a
///   corrupt-but-`Ok` result.
/// - [`MattenStatsError::ZeroVariance`] if `min(x) == max(x)`. **`matten-stats`
///   errors rather than inventing a range**, unlike NumPy, which widens a
///   constant input to `(v - 0.5, v + 0.5)` — that `0.5` comes from nowhere
///   in the data (RFC-090 §4.4).
/// - [`MattenStatsError::AllocationLimit`] if `bins` exceeds
///   `matten::MattenLimits::default().max_elements`, checked before any
///   allocation.
///
/// ```
/// use matten::Tensor;
/// use matten_stats::histogram;
///
/// let x = Tensor::new(
///     vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
///     &[10],
/// );
/// let h = histogram(&x, 5).unwrap();
///
/// assert_eq!(h.edges, vec![0.0, 1.8, 3.6, 5.4, 7.2, 9.0]);
/// assert_eq!(h.counts.iter().sum::<usize>(), x.len()); // nothing is dropped
/// ```
pub fn histogram(x: &Tensor, bins: usize) -> Result<Histogram, MattenStatsError> {
    if x.is_dynamic() {
        return Err(MattenStatsError::DynamicTensor);
    }
    if bins == 0 {
        return Err(MattenStatsError::InvalidBinCount);
    }
    if x.len() == 0 {
        return Err(MattenStatsError::Empty);
    }

    let xs = x.as_slice();
    if xs.iter().any(|v| !v.is_finite()) {
        return Err(MattenStatsError::NonFiniteValue);
    }

    let (mut lo, mut hi) = (xs[0], xs[0]);
    for &v in &xs[1..] {
        if v < lo {
            lo = v;
        }
        if v > hi {
            hi = v;
        }
    }

    // Every element of `x` is finite (checked above), but `hi - lo` can still
    // overflow to infinity for extreme-magnitude inputs, which would then
    // poison every edge (`inf * 0 / bins` is `NaN`, `lo + inf` is `inf`) and
    // silently return a corrupt-but-`Ok` histogram. Reject it explicitly,
    // same reasoning as RFC-090 §4.4's constant-input rejection: an
    // uninterpretable range must error, not produce a plot no one can read.
    let range = hi - lo;
    if !range.is_finite() {
        return Err(MattenStatsError::NonFiniteValue);
    }

    if lo == hi {
        return Err(MattenStatsError::ZeroVariance);
    }

    let limit = MattenLimits::default().max_elements;
    if bins > limit {
        return Err(MattenStatsError::AllocationLimit {
            requested_bins: bins,
            limit,
        });
    }

    // Compute each interior edge directly from (lo, i, bins) rather than by
    // repeated addition of a width, then pin the final edge to `hi` exactly
    // rather than trusting `lo + range * bins / bins` to round back to `hi`
    // bit-for-bit (handoff §3).
    let bins_f = bins as f64;
    let mut edges: Vec<f64> = (0..bins)
        .map(|i| lo + range * (i as f64) / bins_f)
        .collect();
    edges.push(hi);

    let mut counts = vec![0usize; bins];
    for &v in xs {
        let idx = (((v - lo) / range) * bins_f) as usize;
        counts[idx.min(bins - 1)] += 1; // v == hi lands in the last bin (§4.3)
    }

    Ok(Histogram { counts, edges })
}
