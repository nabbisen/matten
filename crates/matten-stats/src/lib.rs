//! `matten-stats` — small, explicit scalar statistics over [`matten::Tensor`].
//!
//! This companion crate (RFC-078) provides the three statistics APIs RFC-040
//! §8 deliberately kept out of core: [`covariance`], [`correlation`], and
//! [`quantile`]. It depends only on core `matten` (no default features) — no
//! third-party dependency of any kind.
//!
//! # The `matten-mlprep` boundary (RFC-078 §5)
//!
//! `matten-mlprep` transforms tensors for ML pipelines: `Tensor -> Tensor`.
//! `matten-stats` computes scalar statistical summaries: `Tensor -> f64`. No
//! function appears in both crates.
//!
//! # The `ddof = 1` divergence from core (RFC-078 §4.1)
//!
//! Core `matten`'s `var`/`std` are **population** statistics (`ddof = 0`).
//! [`covariance`] and [`correlation`] in this crate use the **sample**
//! estimator (`ddof = 1`, i.e. divide by `n - 1`), matching the near-universal
//! default in inferential statistics (NumPy, R, pandas). This is a deliberate
//! divergence, not an oversight — a reader must not have to discover it
//! empirically. `correlation` is unaffected by the choice (the `n-1` factors
//! cancel); only `covariance` is a genuine policy decision.
//!
//! # Quantile method (RFC-078 §4.2)
//!
//! [`quantile`] uses **linear interpolation** between the two nearest ranks of
//! the sorted sample (NumPy's `"linear"` method). No alternative method
//! (nearest, lower, higher, midpoint) is provided.
//!
//! # Status
//!
//! **Experimental** (RFC-040 §9). This is a new crate with no usage history;
//! its surface may still change. This RFC does not promote it further.
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
mod quantile;

pub use crate::covariance::{correlation, covariance};
pub use crate::error::MattenStatsError;
pub use crate::quantile::quantile;
