//! External boundary parsers: JSON and CSV (RFC-009).
//!
//! Both submodules are Result-zone only; they never panic on malformed input.

#[cfg(feature = "json")]
pub(crate) mod json;

#[cfg(feature = "csv")]
pub(crate) mod csv;
