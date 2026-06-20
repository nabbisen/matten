//! Phase 2 dynamic feature: heterogeneous `Element` values, shared-storage
//! tensors, and CoW view semantics (RFC-011 + RFC-012).
//!
//! Everything in this module is `#[cfg(feature = "dynamic")]`.
//!
//! # Memory decision (RFC-011 §9)
//!
//! All text representations (`String`, `Box<str>`, `Arc<str>`) produce
//! `size_of::<Element>() == 24` on 64-bit targets. `Arc<str>` was chosen
//! because:
//! - same footprint as the alternatives;
//! - cheap clone — important for CoW shared-storage slices;
//! - immutable — no text mutation races;
//! - no extra dependency.

pub(crate) mod element;
#[cfg(any(feature = "json", feature = "csv"))]
pub(crate) mod parse;
pub(crate) mod policy;
pub(crate) mod storage;
pub(crate) mod tensor_ext;

pub use element::Element;

pub use policy::NumericPolicy;
