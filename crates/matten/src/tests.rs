//! Unit tests for `matten`, organised by the module they cover.
//!
//! Each submodule mirrors one `src/` module; the layout follows the project
//! guideline that a `foo.rs` and a `foo/` directory may coexist, and that test
//! code is separated from implementation when files grow large.

mod composition;
mod convert;
mod creation;
mod dynamic;
mod elementwise;
mod error;
mod linalg;
mod math;
mod ops;
mod parse;
mod reshape;
mod selection;
mod shape;
mod shape_ops;
mod slice;
mod stats;
mod tensor;
