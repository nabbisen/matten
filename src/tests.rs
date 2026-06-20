//! Unit tests for `matten`, organised by the module they cover.
//!
//! Each submodule mirrors one `src/` module; the layout follows the project
//! guideline that a `foo.rs` and a `foo/` directory may coexist, and that test
//! code is separated from implementation when files grow large.

mod convert;
mod error;
mod ops;
mod reshape;
mod shape;
mod slice;
mod tensor;
