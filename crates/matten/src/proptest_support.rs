//! Shared `proptest` strategies for RFC-128's properties (P1-P4).
//!
//! Test-only infrastructure: this module is `#[cfg(test)]`-gated in `lib.rs`
//! and compiles into no non-test build. It exists so the shape generator that
//! actually probes RFC-127's edge cases is written once, not duplicated across
//! every properties file that needs it.
//!
//! **A generator that only produces small friendly shapes proves nothing**
//! (RFC-128 §4.1, R1) — [`dim`] deliberately weights toward degenerate and
//! boundary values, not away from them.

use crate::limits::{MAX_ELEMENTS, MAX_NDIM, MAX_REPRESENTABLE_DIMENSION};
use proptest::prelude::*;

/// A single shape dimension. Mostly small (and often zero, legal since
/// RFC-111), but sometimes a value chosen specifically to sit at or past a
/// boundary the crate enforces:
///
/// - just past [`MAX_REPRESENTABLE_DIMENSION`] (the hard per-dimension ceiling)
/// - just past [`MAX_ELEMENTS`] (the default element budget)
/// - `usize::MAX` and `usize::MAX / 2` (product-overflow triggers, RFC-127's
///   own reproduction shape)
pub(crate) fn dim() -> impl Strategy<Value = usize> {
    prop_oneof![
        6 => 0usize..6,
        1 => Just(MAX_REPRESENTABLE_DIMENSION + 1),
        1 => Just(MAX_ELEMENTS + 1),
        1 => Just(usize::MAX),
        1 => Just(usize::MAX / 2),
    ]
}

/// A shape: rank 0 (the scalar) through one rank past [`MAX_NDIM`] (so the
/// rank-rejection path gets exercised too), each dimension from [`dim`].
pub(crate) fn shape() -> impl Strategy<Value = Vec<usize>> {
    prop::collection::vec(dim(), 0..=(MAX_NDIM + 1))
}

/// A shape bounded so its product is small enough to actually allocate and
/// iterate over in a test (distinct from [`shape`], which generates freely
/// and lets the constructor under test reject what it must). Used by
/// properties that need to construct real data and touch every element,
/// where generating an oversized shape would OOM the test itself rather
/// than the crate (RFC-128 §4.1, R2: bound the DATA, not the shape).
///
/// Rank 0-5, each dimension 0-4: the worst case (`4^5 = 1024`) already sits
/// comfortably under the 4096-element filter, so the filter is a safety net,
/// not the mechanism doing the bounding — it should reject close to nothing.
pub(crate) fn small_shape() -> impl Strategy<Value = Vec<usize>> {
    prop::collection::vec(0usize..5, 0..=5)
        .prop_filter("product must stay small enough to materialize", |s| {
            checked_product(s).is_some_and(|p| p <= 4096)
        })
}

/// The checked (overflow-safe) element count of a shape, mirroring the
/// crate's own computation. Used by properties to compute the expected
/// invariant value without risking a debug-mode multiplication-overflow
/// panic inside the property itself, which would report as a spurious
/// property failure rather than the crate behavior under test.
pub(crate) fn checked_product(shape: &[usize]) -> Option<usize> {
    shape.iter().try_fold(1usize, |acc, &d| acc.checked_mul(d))
}

/// Demonstrates — not merely asserts — that [`shape`] actually produces the
/// edge classes RFC-128 §4.1 requires. "It should generate them" is not
/// evidence (the handoff's own words); this samples the strategy directly
/// (bypassing the property-test loop, so a class this generator never hits
/// cannot hide behind an early return in some unrelated property) and fails
/// loudly if any required class never appears.
#[test]
fn shape_generator_covers_the_required_edge_classes() {
    use proptest::strategy::{Strategy, ValueTree};
    use proptest::test_runner::TestRunner;

    let mut runner = TestRunner::default();
    let mut saw_zero_dim = false;
    let mut saw_rank0 = false;
    let mut saw_max_rank_or_beyond = false;
    let mut saw_huge_dim = false;
    let mut saw_overflow_trigger = false;

    for _ in 0..2000 {
        let s = shape().new_tree(&mut runner).unwrap().current();
        saw_rank0 |= s.is_empty();
        saw_zero_dim |= s.contains(&0);
        saw_max_rank_or_beyond |= s.len() >= MAX_NDIM;
        saw_huge_dim |= s.iter().any(|&d| d > MAX_ELEMENTS);
        saw_overflow_trigger |= checked_product(&s).is_none();
    }

    assert!(
        saw_rank0,
        "shape() never produced rank 0 (the scalar) in 2000 samples"
    );
    assert!(
        saw_zero_dim,
        "shape() never produced a zero dimension in 2000 samples"
    );
    assert!(
        saw_max_rank_or_beyond,
        "shape() never produced rank >= MAX_NDIM in 2000 samples"
    );
    assert!(
        saw_huge_dim,
        "shape() never produced a dimension past MAX_ELEMENTS in 2000 samples"
    );
    assert!(
        saw_overflow_trigger,
        "shape() never produced a shape whose product overflows usize in 2000 samples"
    );
}
