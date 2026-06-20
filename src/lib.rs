//! `matten` — a developer-experience-first multidimensional array (tensor)
//! library for Rust.
//!
//! `matten` is the *family car* of Rust tensor libraries: easy to start,
//! predictable, and friendly for non-expert Rust developers doing small
//! numerical, data-exploration, or business proof-of-concept work. It
//! deliberately prioritizes **developer experience over peak performance**, and
//! is not a replacement for `ndarray`, `nalgebra`, or `candle` on hot paths.
//!
//! # Status
//!
//! This is the **M0 skeleton** (`0.0.1`). It establishes the crate structure,
//! the stable public error surface, and a minimal [`Tensor`] so the project
//! compiles, lints, and ships a smoke example. Math, reshaping, slicing,
//! broadcasting, and the JSON/CSV boundaries arrive in later milestones; see the
//! roadmap and RFC pack.
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
// Planned internal module map (added with their owning milestones):
//   ops/        element-wise + scalar operators and broadcasting (M3)
//   shape/      shape validation, strides, index helpers (M1)
//   parse/      JSON/CSV boundary parsers (M5)
//   dynamic/    feature-gated `Element` engine (Phase 2)
mod error;
mod tensor;

pub use crate::error::{DataFormat, MattenError};
pub use crate::tensor::Tensor;

// `Element` (the Phase 2 dynamic value type) becomes a public export under the
// `dynamic` feature once that engine lands. It is intentionally not exported in
// M0; enabling `dynamic` today simply compiles the Phase 1 surface unchanged.

#[cfg(test)]
mod tests;
