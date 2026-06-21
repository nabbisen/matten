//! Dynamic boundary parsers: mixed JSON and CSV into `Vec<Element>` (RFC-011 §11).
//!
//! Each submodule is gated on the feature that provides the required dependency:
//! `json` for `serde_json` and `csv` for the `csv` crate.

#[cfg(feature = "json")]
pub(crate) mod json;

#[cfg(feature = "csv")]
pub(crate) mod csv;
