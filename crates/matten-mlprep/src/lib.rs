//! `matten-mlprep` — small, transparent, deterministic preprocessing helpers for
//! [`matten::Tensor`].
//!
//! This companion crate (RFC-024, RFC-028) prepares numeric tensors for use with
//! external tools. It is **not** an ML framework: there is no model training, no
//! autograd, no optimizer, and no hidden randomness. Every function is a pure,
//! deterministic transform you can reason about. It depends only on core
//! `matten` (no default features) — no `ndarray`, no `candle`, no `rand`.
//!
//! # Convention
//!
//! All functions operate on **rank-2** tensors with `rows = samples` and
//! `columns = features`. A non-2D tensor is rejected; there is no silent
//! transposition.
//!
//! # Functions
//!
//! - [`standardize_columns`] — per-column z-score (population std).
//! - [`minmax_scale_columns`] — per-column scaling to `[0, 1]`.
//! - [`add_bias_column`] — prepend a constant `1.0` intercept column.
//! - [`train_test_split`] — ordered, deterministic row split.
//!
//! ```
//! use matten::Tensor;
//! use matten_mlprep::{add_bias_column, standardize_columns, train_test_split};
//!
//! let x = Tensor::new(vec![1.0, 3.0, 5.0, 7.0], &[4, 1]);
//! let z = standardize_columns(&x).unwrap();        // zero mean, unit std
//! let z = add_bias_column(&z).unwrap();            // [4, 2], column 0 = 1.0
//! let (train, test) = train_test_split(&z, 0.75).unwrap();
//! assert_eq!(train.shape(), &[3, 2]);
//! assert_eq!(test.shape(), &[1, 2]);
//! ```
//!
//! # Status
//!
//! **Production-ready candidate.** The small surface is stable; usable seriously
//! if the documented limits are acceptable. Note that [`train_test_split`] is
//! ordered-only (no shuffle). Constant (zero-variance) columns are rejected
//! explicitly by the scalers rather than silently producing a zero column — see
//! [`MattenMlprepError::ZeroVariance`]. Dynamic tensors are rejected at every
//! public entry point unconditionally — the guard does not depend on the
//! companion `dynamic` feature (RFC-031).
//!
//! # Feature flags
//!
//! - `dynamic` — Compatibility forwarding feature. No longer required for
//!   dynamic rejection as of v0.19.1. Dynamic tensors are rejected at companion
//!   boundaries regardless of whether this feature is enabled. Reconsider
//!   removal no earlier than v0.20.0.

#![forbid(unsafe_code)]

mod bias;
mod error;
mod scale;
mod split;
mod util;

pub use crate::bias::add_bias_column;
pub use crate::error::MattenMlprepError;
pub use crate::scale::{minmax_scale_columns, standardize_columns};
pub use crate::split::train_test_split;
