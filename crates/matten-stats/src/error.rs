//! Error type for `matten-stats` (RFC-078 §4, §6).
//!
//! The crate defines its own error type rather than reusing core
//! [`matten::MattenError`] — RFC-032 forbids companions re-exporting core
//! error types. Every public function returns `Result`; none of the three
//! functions construct a `Tensor`, so there is no `Matten(..)` wrapping
//! variant to carry a core error through.

use std::fmt;

/// Errors produced by `matten-stats` functions.
///
/// `#[non_exhaustive]` so future variants are not a breaking change.
#[derive(Debug)]
#[non_exhaustive]
pub enum MattenStatsError {
    /// A dynamic (`Element`) tensor was passed. Convert it to a numeric
    /// tensor first with `Tensor::try_numeric()`.
    DynamicTensor,
    /// The input has too few elements for the requested operation: an empty
    /// tensor for any function; fewer than 2 elements for
    /// `covariance`/`correlation`/`skewness`/`kurtosis` (their `n - 1`
    /// divisor, or their ratio's `m2 > 0` precondition, would be undefined).
    /// `covariance_population` is the one exception — its divisor is `n`, so
    /// a single element is well-defined and returns `0.0`.
    Empty,
    /// `covariance`/`correlation` require both inputs to have the same
    /// element count.
    LengthMismatch {
        /// Element count of the left (`x`) input.
        left: usize,
        /// Element count of the right (`y`) input.
        right: usize,
    },
    /// An input value was not finite (`NaN` or infinite), or a value derived
    /// from otherwise-finite input was not finite — `histogram`'s `max(x) -
    /// min(x)` can overflow to infinity even though every element of `x` is
    /// finite (RFC-090 §4.4's follow-up: NumPy's invented `±0.5` range on
    /// constant input is rejected for producing an uninterpretable plot;
    /// `NaN`/`inf` edges from an overflowing range are strictly worse and
    /// must not be returned silently either).
    NonFiniteValue,
    /// `correlation` was asked to divide by a zero standard deviation in
    /// either input.
    ZeroVariance,
    /// `quantile`'s `q` was not finite or not in `[0.0, 1.0]`.
    InvalidQuantile(f64),
    /// `histogram`'s `bins` was `0`. Follows `matten-data`'s
    /// `InvalidBatchSize` precedent (RFC-082): a required count argument
    /// that cannot be zero earns its own variant.
    InvalidBinCount,
    /// `histogram`'s `bins` would allocate more elements than
    /// `matten::MattenLimits::default().max_elements` permits. Checked
    /// before allocating `Histogram::counts`/`edges` — unlike [`Empty`],
    /// which means too few elements, this means too many were requested.
    ///
    /// [`Empty`]: MattenStatsError::Empty
    AllocationLimit {
        /// The `bins` value that was requested.
        requested_bins: usize,
        /// `matten::MattenLimits::default().max_elements`, the ceiling that was exceeded.
        limit: usize,
    },
}

impl fmt::Display for MattenStatsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MattenStatsError::DynamicTensor => write!(
                f,
                "matten-stats error: dynamic tensors are not supported; call \
                 try_numeric() to convert to a numeric tensor first"
            ),
            MattenStatsError::Empty => write!(
                f,
                "matten-stats error: input has too few elements for this operation \
                 (must be non-empty; covariance/correlation/skewness/kurtosis require \
                 at least 2; covariance_population accepts 1)"
            ),
            MattenStatsError::LengthMismatch { left, right } => write!(
                f,
                "matten-stats error: inputs must have the same element count, \
                 got {left} and {right}"
            ),
            MattenStatsError::NonFiniteValue => write!(
                f,
                "matten-stats error: a non-finite value (NaN or infinite) was found in \
                 the input, or produced by a computation over it"
            ),
            MattenStatsError::ZeroVariance => write!(
                f,
                "matten-stats error: this operation is undefined when an input \
                 has zero variance"
            ),
            MattenStatsError::InvalidQuantile(q) => write!(
                f,
                "matten-stats error: q must be finite and in [0.0, 1.0], got {q}"
            ),
            MattenStatsError::InvalidBinCount => {
                write!(f, "matten-stats error: bins must be greater than 0")
            }
            MattenStatsError::AllocationLimit {
                requested_bins,
                limit,
            } => write!(
                f,
                "matten-stats error: bins ({requested_bins}) would exceed the allocation \
                 limit of {limit} (matten::MattenLimits::max_elements); use fewer bins or \
                 a larger limit"
            ),
        }
    }
}

impl std::error::Error for MattenStatsError {}
