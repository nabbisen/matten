//! Regression tests for RFC-127 (the unbounded-dimension audit fix).
//!
//! Each test is named for the evidence row it pins (E2-E5 in the RFC, T1-T5 in
//! the handoff). Before this RFC, all of these either aborted the process
//! (uncatchably) or returned a `Tensor` whose `shape` and `data` silently
//! disagreed. RFC-111's own zero-sized-dimension tests are untouched by this
//! RFC (T2) — the defect was the missing per-dimension bound, not the
//! decision to accept a zero-sized one.

use matten::{MattenError, MattenLimits, Tensor};

/// T1 / E2: a degenerate shape (one huge dimension paired with a zero one,
/// so the product is 0 and never overflows `checked_mul`) is rejected at
/// construction, from both `from_json` and `try_new` — not merely from one
/// entry point.
#[test]
#[cfg(feature = "json")]
fn t1_degenerate_shape_rejected_from_json() {
    let result = Tensor::from_json(r#"{"shape":[400000000000,0],"data":[]}"#);
    assert!(
        matches!(result, Err(MattenError::Parse { .. })),
        "expected a Parse error wrapping the allocation rejection, got {result:?}"
    );
}

#[test]
fn t1_degenerate_shape_rejected_from_try_new() {
    let result = Tensor::try_new(vec![], &[400_000_000_000, 0]);
    assert!(
        matches!(result, Err(MattenError::Allocation { .. })),
        "expected an Allocation error, got {result:?}"
    );
}

/// T3 / E3: `sum_axis` is the one reduction that legitimately proceeds when
/// the REDUCED axis is zero-length (RFC-110 left `sum`/`sum_axis` unchanged).
/// Change A alone does not close this: seven axes each individually within
/// the per-dimension bound, plus one zero axis, still construct — and their
/// product as SURVIVING axes is not otherwise bounded. This must return an
/// `Err`, never abort the process.
#[test]
fn t3_sum_axis_on_zero_reduced_axis_does_not_abort() {
    let big = MattenLimits {
        max_elements: 2_000_000,
        max_dimensions: 8,
        ..MattenLimits::default()
    };
    let shape = [1_048_576_usize, 1_048_576, 1, 1, 1, 1, 1, 0];
    let t = Tensor::try_zeros_with_limits(&shape, &big).expect("construction is within budget");
    assert_eq!(t.as_slice().len(), 0);

    let result = t.try_sum_axis(7);
    assert!(
        matches!(result, Err(MattenError::Allocation { .. })),
        "expected an Allocation error for the oversized surviving-axis product, got {result:?}"
    );
}

/// T4 / E4: `try_matmul` cannot return a `Tensor` whose `shape.product()`
/// disagrees with `data.len()`. Two individually valid operands (each within
/// the default element budget) can still multiply out to an unreasonable
/// output — this must error, not silently corrupt.
#[test]
fn t4_try_matmul_cannot_produce_a_shape_data_mismatch() {
    let big = MattenLimits {
        max_elements: 2_000_000,
        ..MattenLimits::default()
    };
    let a = Tensor::try_zeros_with_limits(&[1_048_576, 1], &big).expect("a is within budget");
    let b = Tensor::try_zeros_with_limits(&[1, 1_048_576], &big).expect("b is within budget");

    match a.try_matmul(&b) {
        Ok(r) => {
            let product: usize = r.shape().iter().product();
            assert_eq!(
                product,
                r.as_slice().len(),
                "invariant violated: shape {:?} implies {product} elements but data has {}",
                r.shape(),
                r.as_slice().len()
            );
        }
        Err(e) => assert!(matches!(e, MattenError::Allocation { .. })),
    }
}

/// T5 / E5: `usize::MAX` cast to `isize` wraps negative, and a negative index
/// means "from the end" (RFC-088) — silently returning the last row instead
/// of erroring. `index(9)` (a plain out-of-range index) must keep erroring
/// exactly as before, and an ordinary in-range index must keep working.
#[test]
fn t5_slice_index_usize_max_errors_instead_of_wrapping() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);

    let wrapped = t.slice().index(usize::MAX).all().build();
    assert!(
        matches!(wrapped, Err(MattenError::Slice { .. })),
        "expected an out-of-range Slice error, got {wrapped:?}"
    );

    let out_of_range = t.slice().index(9).all().build();
    assert!(matches!(out_of_range, Err(MattenError::Slice { .. })));

    let ordinary = t.slice().index(1).all().build().unwrap();
    assert_eq!(ordinary.shape(), &[3]);
    assert_eq!(ordinary.as_slice(), &[4.0, 5.0, 6.0]);
}

/// T7 (review correction, RFC-127 §5): the `usize` -> `isize` saturating cast
/// in `slice.rs` is only sound if no dimension a shape check accepts can ever
/// reach `isize::MAX`. Nothing previously enforced that a caller-supplied
/// `MattenLimits::max_elements` stayed below `isize::MAX` — a deliberately
/// huge budget let a `[2^63, 0]` shape construct, at which point
/// `index(usize::MAX)` (saturating to `isize::MAX`, which was LESS than the
/// dimension `2^63`) passed the bounds check it should have failed.
/// `checked_shape_len` now clamps every dimension to at most
/// `MAX_REPRESENTABLE_DIMENSION` (`isize::MAX / 8`) unconditionally, so an
/// oversized budget is rejected at construction rather than surviving to
/// make the saturating cast wrong.
#[test]
fn t7_an_oversized_max_elements_budget_cannot_defeat_the_saturating_cast() {
    let reckless = MattenLimits {
        max_elements: usize::MAX,
        ..MattenLimits::default()
    };
    let result = Tensor::try_zeros_with_limits(&[1usize << 63, 0], &reckless);
    assert!(
        matches!(result, Err(MattenError::Allocation { .. })),
        "expected construction to be rejected regardless of the caller's budget, got {result:?}"
    );
}
