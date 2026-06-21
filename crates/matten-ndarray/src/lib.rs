//! `matten-ndarray` — an experimental conversion bridge between
//! [`matten::Tensor`] and [`ndarray::ArrayD<f64>`].
//!
//! This is a deliberately *boring* companion crate (RFC-025, RFC-027): it
//! converts between the two owned representations and does nothing else. It adds
//! no dependency to core `matten`, wraps none of the `ndarray` API, and exposes
//! no view or lifetime types.
//!
//! ```
//! use matten::Tensor;
//! use matten_ndarray::{from_arrayd, to_arrayd};
//!
//! let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
//! let arr = to_arrayd(&t).unwrap();
//! let back = from_arrayd(arr).unwrap();
//! assert_eq!(back.as_slice(), t.as_slice());
//! ```
//!
//! # Status
//!
//! **Experimental (0.1.x).** The API may change. Pin the version if you depend
//! on it. Supported `ndarray`: the `0.16` minor.
//!
//! # Behavior
//!
//! - Both conversions **copy**; no zero-copy is claimed.
//! - [`from_arrayd`] preserves *logical* element order even for non-standard
//!   (transposed / sliced) `ArrayD` inputs.
//! - [`from_arrayd`] rejects shapes with a zero-length axis ([`matten`] forbids
//!   zero-sized dimensions).
//! - With the `dynamic` feature, a dynamic tensor passed to [`to_arrayd`]
//!   returns [`MattenNdarrayError::DynamicTensor`] instead of panicking.

#![forbid(unsafe_code)]

mod convert;
mod error;

pub use crate::convert::{from_arrayd, to_arrayd};
pub use crate::error::MattenNdarrayError;
