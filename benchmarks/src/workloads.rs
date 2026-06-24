//! Benchmark workloads, grouped into a core micro set and scenario set.
//!
//! Each workload is a pure function over pre-built inputs that returns a value the
//! bench can hand to `black_box`. Workloads do not print, allocate inputs in the
//! timed body, or read the clock.

pub mod core;
pub mod scenarios;

// RFC-049 Phase 2: Rust peer-comparison workloads (ndarray / nalgebra). Compiled
// only under the opt-in `peers` feature, so the default build stays peer-free.
#[cfg(feature = "peers")]
pub mod peers;
