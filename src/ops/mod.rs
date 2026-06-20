//! Element-wise operators and broadcasting (RFC-006).
//!
//! Module layout (handoff PR-006-A):
//! - `broadcast` — shape computation and index mapping
//! - `tensor_ops` — binary `&Tensor op &Tensor` impls
//! - `scalar_ops` — scalar `&Tensor op f64` and `f64 op &Tensor` impls
//! - `unary_ops` — `Neg`

pub(crate) mod broadcast;
mod scalar_ops;
mod tensor_ops;
mod unary_ops;
