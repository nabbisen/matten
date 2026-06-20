//! `matten` — a developer-experience-first multidimensional array (tensor)
//! library for Rust.
//!
//! `matten` is a *family car* of Rust tensor library: easy to start,
//! predictable, and friendly for non-expert Rust developers doing small
//! numerical, data-exploration, or business proof-of-concept work. It
//! deliberately prioritizes **developer experience over peak performance**, and
//! is not a replacement for `ndarray`, `nalgebra`, or `candle` on hot paths.
//!
//! # Status
//!
//! This is **`0.1.0-alpha.3`** (milestone M3, Broadcasting and Element-wise
//! Operators). The full construction surface is in place: fill constructors, `from_vec`,
//! `arange`/`try_arange`, `into_vec`, `try_from_rows`, and `From`/`TryFrom`
//! impls. Arithmetic, reshape, slicing, and I/O arrive in later milestones.
//!
//! # Quick start
//!
//! ```
//! use matten::Tensor;
//!
//! let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
//! assert_eq!(a.shape(), &[2, 2]);
//! assert_eq!(a.ndim(), 2);
//! println!("{a:?}");
//! ```
//!
//! Boundary-style construction returns a [`Result`](std::result::Result) instead
//! of panicking:
//!
//! ```
//! use matten::{MattenError, Tensor};
//!
//! let bad = Tensor::try_new(vec![1.0, 2.0, 3.0], &[2, 2]);
//! assert!(matches!(bad, Err(MattenError::Shape { .. })));
//! ```
//!
//! # Panic zone vs Result zone
//!
//! `matten` splits its API into two error zones:
//!
//! - **Panic zone** — local, developer-authored convenience APIs (such as
//!   [`Tensor::new`]) panic with an actionable message for fast PoC feedback.
//! - **Result zone** — every external boundary (parsing, file I/O, user-driven
//!   construction such as [`Tensor::try_new`]) returns [`MattenError`] and never
//!   panics on ordinary invalid input.
//!
//! Errors are matched by variant (`matches!`), never by `==`: [`MattenError`]
//! embeds [`std::io::Error`] and so derives only `Debug`.
//!
//! # Cargo features
//!
//! The default profile is convenient for PoC users:
//!
//! - `serde` *(default)* — `Serialize` / `Deserialize` for [`Tensor`].
//! - `json` *(default, implies `serde`)* — `from_json` / `load_json`.
//! - `csv` *(default)* — `from_csv` / `load_csv`.
//! - `dynamic` — the Phase 2 heterogeneous `Element` engine (off by default).
//!
//! For the smallest dependency footprint, disable defaults and opt in:
//!
//! ```toml
//! matten = { version = "0.1", default-features = false }
//! ```

#![forbid(unsafe_code)]

// Public modules. The public surface is intentionally centered on `Tensor`,
// `MattenError`, and `DataFormat`; storage, layout, and (future) `ops` modules
// stay internal.
//
// Internal module map (each added with its owning milestone):
//   shape       shape validation, strides, row-major index helpers (M1)
//   convert     From/TryFrom impls and nested-row helpers (M2)
//   ops/        element-wise + scalar operators and broadcasting (M3)
//   parse/      JSON/CSV boundary parsers (M5)
//   dynamic/    feature-gated `Element` engine (Phase 2)
mod convert;
mod error;
mod shape;
mod tensor;

pub use crate::error::{DataFormat, MattenError};
pub use crate::tensor::Tensor;

// `Element` (the Phase 2 dynamic value type) becomes a public export under the
// `dynamic` feature once that engine lands. It is intentionally not exported in
// M0; enabling `dynamic` today simply compiles the Phase 1 surface unchanged.

#[cfg(test)]
mod tests;
