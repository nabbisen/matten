//! Rust peer-comparison workloads (RFC-049 Phase 2).
//!
//! **Comparable task only.** A task appears here only if `matten`, `ndarray`, and/or
//! `nalgebra` solve the *same small mathematical problem* with comparable data
//! representation and no hidden extra library capability. This is a peer comparison,
//! not a competitor ranking: the point is to show where `matten`'s approachable
//! `Tensor` API sits relative to established Rust numeric crates on small, equivalent
//! tasks — never to claim `matten` is "faster than X".
//!
//! Peer assignment (architect ruling, RFC-049 Phase 2 §6):
//! - `ndarray` covers all six tasks (its `Array1`/`Array2` map cleanly to `matten`'s
//!   row-major tensors).
//! - `nalgebra` covers the tasks that are naturally small dense matrix/vector
//!   operations. Each task below documents its representation and why it is comparable.
//!
//! Inputs are built once by the bench from identical logical data, then converted to
//! each library's native type, so the three implementations time the same problem at
//! the same sizes.

pub mod nalgebra_tasks;
pub mod ndarray_tasks;
