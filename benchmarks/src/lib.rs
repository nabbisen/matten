//! Internal benchmark harness for the `matten` workspace (RFC-049 Phase 1).
//!
//! This library holds the *workloads* (what is measured) separately from the
//! *benches* (how they are timed, in `benches/`). Keeping the workloads in a
//! plain library means they have no dependency on `criterion`; only the bench
//! targets do. Phase 1 covers a small core micro set and five scenario workloads
//! drawn from the existing examples — no peer (`ndarray`/`nalgebra`) or
//! cross-language (NumPy/Pandas) comparisons, which remain deferred (RFC-049
//! Phases 2–4).

#![forbid(unsafe_code)]

pub mod common;
pub mod workloads;
