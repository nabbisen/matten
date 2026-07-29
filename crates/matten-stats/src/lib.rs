//! `matten-stats` — small, explicit scalar statistics over [`matten::Tensor`].
//!
//! This companion crate (RFC-078, RFC-083) provides six statistics APIs
//! RFC-040 §8 deliberately kept out of core: [`covariance`],
//! [`covariance_population`], [`correlation`], [`quantile`], [`skewness`],
//! and [`kurtosis`]. It depends only on core `matten` (no default features) —
//! no third-party dependency of any kind.
//!
//! # The `matten-mlprep` boundary (RFC-078 §5)
//!
//! `matten-mlprep` transforms tensors for ML pipelines: `Tensor -> Tensor`.
//! `matten-stats` computes scalar statistical summaries: `Tensor -> f64`. No
//! function appears in both crates.
//!
//! # Estimator conventions (RFC-078 §4.1, RFC-083 §4.1)
//!
//! Core `matten`'s `var`/`std` are **population** statistics (`ddof = 0`).
//! Each function below matches the convention its ecosystem name is expected
//! to carry — the estimator **differs per function**, deliberately, because
//! the ecosystem's own defaults differ per function:
//!
//! ```text
//! covariance             sample,     ddof = 1        (NumPy/R/pandas `cov`/`corrcoef` default)
//! covariance_population  population, ddof = 0        (explicit in the name; no default to choose)
//! correlation            ddof-invariant                (the `n - 1` factors cancel; see below)
//! skewness               g1,  uncorrected               (SciPy `skew(bias=True)` default; NOT pandas' `.skew()`)
//! kurtosis               g2,  uncorrected, EXCESS        (SciPy `kurtosis(fisher=True, bias=True)` default; NOT pandas' `.kurt()`)
//! ```
//!
//! `correlation` is unaffected by the `ddof` choice (the `n - 1` factors
//! cancel algebraically); only [`covariance`] is a genuine policy decision.
//! pandas' `.skew()`/`.kurt()` bias-correct and so return a **different**
//! number than [`skewness`]/[`kurtosis`] for the same input — this must not
//! be assumed away. This is a deliberate divergence, not an oversight; a
//! reader must not have to discover it empirically.
//!
//! # Quantile method (RFC-078 §4.2)
//!
//! [`quantile`] uses **linear interpolation** between the two nearest ranks of
//! the sorted sample (NumPy's `"linear"` method). No alternative method
//! (nearest, lower, higher, midpoint) is provided.
//!
//! # Status
//!
//! **Production-ready candidate** (RFC-084), promoted once the six-function
//! surface settled (RFC-083). The candidate label denotes a settled surface
//! and a narrowed recommendation, not field-tested usage history — this
//! crate has none. Under lock-step family versioning (RFC-030) the crate
//! shares the workspace family version; maturity is the Status label, not the
//! version number.
//!
//! ```
//! use matten::Tensor;
//! use matten_stats::{correlation, covariance, quantile};
//!
//! let x = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[4]);
//! let y = Tensor::new(vec![2.0, 4.0, 6.0, 8.0], &[4]);
//!
//! let cov = covariance(&x, &y).unwrap();
//! let r = correlation(&x, &y).unwrap();
//! let median = quantile(&x, 0.5).unwrap();
//! assert!((r - 1.0).abs() < 1e-9);
//! assert_eq!(median, 2.5);
//! ```

#![forbid(unsafe_code)]

mod covariance;
mod error;
mod moments;
mod quantile;

pub use crate::covariance::{correlation, covariance, covariance_population};
pub use crate::error::MattenStatsError;
pub use crate::moments::{kurtosis, skewness};
pub use crate::quantile::quantile;
