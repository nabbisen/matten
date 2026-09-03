use crate::{MattenError, Tensor};
use proptest::prelude::*;

// ── axis reductions ───────────────────────────────────────────────────────

#[test]
fn sum_axis_0_on_matrix() {
    // [[1,2,3],[4,5,6]] -> [5,7,9]
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let r = m.sum_axis(0);
    assert_eq!(r.shape(), &[3]);
    assert_eq!(r.as_slice(), &[5.0, 7.0, 9.0]);
}

#[test]
fn sum_axis_1_on_matrix() {
    // [[1,2,3],[4,5,6]] -> [6,15]
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let r = m.sum_axis(1);
    assert_eq!(r.shape(), &[2]);
    assert_eq!(r.as_slice(), &[6.0, 15.0]);
}

#[test]
fn sum_axis_on_vector_gives_scalar() {
    let v = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    let r = v.sum_axis(0);
    assert!(r.is_scalar());
    assert_eq!(r.as_slice(), &[6.0]);
}

#[test]
fn mean_axis_0_on_matrix() {
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let r = m.mean_axis(0);
    assert_eq!(r.shape(), &[3]);
    assert_eq!(r.as_slice(), &[2.5, 3.5, 4.5]);
}

#[test]
fn mean_axis_1_on_matrix() {
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let r = m.mean_axis(1);
    assert_eq!(r.shape(), &[2]);
    assert_eq!(r.as_slice(), &[2.0, 5.0]);
}

#[test]
fn sum_axis_rank3() {
    // shape [2,3,4] summed along axis 1 -> [2,4]
    let data: Vec<f64> = (0..24).map(|x| x as f64).collect();
    let t = Tensor::new(data, &[2, 3, 4]);
    let r = t.sum_axis(1);
    assert_eq!(r.shape(), &[2, 4]);
    // row 0: sum of rows 0..3 of first batch = [0+4+8, 1+5+9, 2+6+10, 3+7+11]
    assert_eq!(r.as_slice()[0], 12.0);
    assert_eq!(r.as_slice()[1], 15.0);
}

#[test]
#[should_panic(expected = "out of range")]
fn sum_axis_out_of_range_panics() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let _ = t.sum_axis(5);
}

// ── min_axis / max_axis ---------------------------------------------------

#[test]
fn min_axis_0_on_matrix() {
    let m = Tensor::new(vec![3.0, 1.0, 4.0, 1.0, 5.0, 9.0], &[2, 3]);
    let r = m.min_axis(0);
    assert_eq!(r.shape(), &[3]);
    assert_eq!(r.as_slice(), &[1.0, 1.0, 4.0]);
}

#[test]
fn max_axis_0_on_matrix() {
    let m = Tensor::new(vec![3.0, 1.0, 4.0, 1.0, 5.0, 9.0], &[2, 3]);
    let r = m.max_axis(0);
    assert_eq!(r.shape(), &[3]);
    assert_eq!(r.as_slice(), &[3.0, 5.0, 9.0]);
}

#[test]
fn min_axis_nan_propagates() {
    let m = Tensor::new(vec![1.0, f64::NAN, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let r = m.min_axis(0);
    assert!(r.as_slice()[1].is_nan()); // NaN in column 1
    assert_eq!(r.as_slice()[0], 1.0);
    assert_eq!(r.as_slice()[2], 3.0);
}

#[test]
fn max_axis_on_vector_gives_scalar() {
    let v = Tensor::from_vec(vec![2.0, 7.0, 4.0]);
    let r = v.max_axis(0);
    assert!(r.is_scalar());
    assert_eq!(r.as_slice(), &[7.0]);
}

#[test]
#[should_panic(expected = "out of range")]
fn min_axis_out_of_range_panics() {
    let t = Tensor::ones(&[3]);
    let _ = t.min_axis(5);
}

// ── Result-form axis reductions (RFC-056) ─────────────────────────────────

#[test]
fn try_axis_reductions_match_panic_forms() {
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    for axis in 0..2 {
        let s = m.try_sum_axis(axis).unwrap();
        assert_eq!(s.shape(), m.sum_axis(axis).shape());
        assert_eq!(s.as_slice(), m.sum_axis(axis).as_slice());
        assert_eq!(
            m.try_mean_axis(axis).unwrap().as_slice(),
            m.mean_axis(axis).as_slice()
        );
        assert_eq!(
            m.try_min_axis(axis).unwrap().as_slice(),
            m.min_axis(axis).as_slice()
        );
        assert_eq!(
            m.try_max_axis(axis).unwrap().as_slice(),
            m.max_axis(axis).as_slice()
        );
    }
}

#[test]
fn try_axis_reductions_reject_out_of_range_axis() {
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    // axis == rank and axis > rank both error; operation is the conceptual op.
    assert!(matches!(
        m.try_sum_axis(2).unwrap_err(),
        MattenError::Shape {
            operation: "sum_axis",
            ..
        }
    ));
    assert!(matches!(
        m.try_mean_axis(9).unwrap_err(),
        MattenError::Shape {
            operation: "mean_axis",
            ..
        }
    ));
    assert!(matches!(
        m.try_min_axis(2).unwrap_err(),
        MattenError::Shape {
            operation: "min_axis",
            ..
        }
    ));
    assert!(matches!(
        m.try_max_axis(5).unwrap_err(),
        MattenError::Shape {
            operation: "max_axis",
            ..
        }
    ));
}

#[test]
#[should_panic(expected = "out of range")]
fn sum_axis_out_of_range_still_panics() {
    let _ = Tensor::ones(&[3]).sum_axis(5);
}

// ── empty reduced-axis semantics (RFC-110) ────────────────────────────────
//
// No constructor accepts a zero-sized shape; every fixture below is reached
// via slice().range(0..0), never a direct constructor (handoff R4).

fn empty_0x3() -> Tensor {
    // shape [0, 3]
    Tensor::new(vec![1., 2., 3., 4., 5., 6.], &[2, 3])
        .slice()
        .range(0..0)
        .all()
        .build()
        .unwrap()
}

fn empty_3x0() -> Tensor {
    // shape [3, 0]
    Tensor::new(vec![1., 2., 3.], &[3, 1])
        .slice()
        .all()
        .range(0..0)
        .build()
        .unwrap()
}

#[test]
fn empty_axis_fixtures_are_actually_empty() {
    // Guards against handoff R4: a fixture whose reduced axis is non-zero
    // would make every test below pass vacuously.
    let a = empty_0x3();
    assert_eq!(a.shape(), &[0, 3]);
    assert_eq!(a.len(), 0);
    let b = empty_3x0();
    assert_eq!(b.shape(), &[3, 0]);
    assert_eq!(b.len(), 0);
}

#[test]
fn mean_min_max_axis_error_on_zero_length_reduced_axis() {
    // T1/T2: reducing axis 0 of [0,3] (length 0) and axis 1 of [3,0] (length 0).
    let a = empty_0x3(); // reduce axis 0 -> length 0
    let b = empty_3x0(); // reduce axis 1 -> length 0

    let err = a.try_mean_axis(0).unwrap_err();
    assert!(matches!(
        err,
        MattenError::InvalidArgument {
            operation: "mean_axis",
            ..
        }
    ));
    assert_eq!(
        err.to_string(),
        "matten invalid argument error in mean_axis: axis: mean is undefined for a reduced axis of length 0 (axis 0)"
    );

    let err = b.try_mean_axis(1).unwrap_err();
    assert!(matches!(
        err,
        MattenError::InvalidArgument {
            operation: "mean_axis",
            ..
        }
    ));
    assert_eq!(
        err.to_string(),
        "matten invalid argument error in mean_axis: axis: mean is undefined for a reduced axis of length 0 (axis 1)"
    );

    let err = a.try_min_axis(0).unwrap_err();
    assert!(matches!(
        err,
        MattenError::InvalidArgument {
            operation: "min_axis",
            ..
        }
    ));
    assert_eq!(
        err.to_string(),
        "matten invalid argument error in min_axis: axis: minimum is undefined for a reduced axis of length 0 (axis 0)"
    );

    let err = a.try_max_axis(0).unwrap_err();
    assert!(matches!(
        err,
        MattenError::InvalidArgument {
            operation: "max_axis",
            ..
        }
    ));
    assert_eq!(
        err.to_string(),
        "matten invalid argument error in max_axis: axis: maximum is undefined for a reduced axis of length 0 (axis 0)"
    );
}

#[test]
#[should_panic(expected = "mean is undefined for a reduced axis of length 0 (axis 0)")]
fn mean_axis_panicking_form_carries_the_message() {
    // T3: the panicking form must carry the captured message, not merely panic.
    let _ = empty_0x3().mean_axis(0);
}

#[test]
#[should_panic(expected = "minimum is undefined for a reduced axis of length 0 (axis 0)")]
fn min_axis_panicking_form_carries_the_message() {
    let _ = empty_0x3().min_axis(0);
}

#[test]
#[should_panic(expected = "maximum is undefined for a reduced axis of length 0 (axis 0)")]
fn max_axis_panicking_form_carries_the_message() {
    let _ = empty_0x3().max_axis(0);
}

#[test]
fn mean_min_max_axis_surviving_empty_axis_is_still_ok_both_orientations() {
    // T4: the SURVIVING axis is length 0 here, not the reduced one -- must stay Ok.
    let a = empty_0x3(); // reduce axis 1 (length 3); axis 0 (length 0) survives
    let r = a.try_mean_axis(1).unwrap();
    assert_eq!(r.shape(), &[0]);
    assert!(r.as_slice().is_empty());
    assert!(a.try_min_axis(1).unwrap().as_slice().is_empty());
    assert!(a.try_max_axis(1).unwrap().as_slice().is_empty());

    let b = empty_3x0(); // reduce axis 0 (length 3); axis 1 (length 0) survives
    let r = b.try_mean_axis(0).unwrap();
    assert_eq!(r.shape(), &[0]);
    assert!(r.as_slice().is_empty());
    assert!(b.try_min_axis(0).unwrap().as_slice().is_empty());
    assert!(b.try_max_axis(0).unwrap().as_slice().is_empty());
}

#[test]
fn sum_axis_unchanged_on_every_empty_axis_case() {
    // T5: sum_axis must be untouched by this RFC on every case above --
    // the zero-length reduced axis returns the additive identity per slot,
    // and the surviving-empty-axis case returns an empty result, exactly as
    // before RFC-110.
    let a = empty_0x3();
    let b = empty_3x0();

    // reduced axis length 0 -> identity 0.0 per output slot (RFC-105 territory,
    // deliberately unchanged by RFC-110).
    assert_eq!(a.try_sum_axis(0).unwrap().as_slice(), &[0.0, 0.0, 0.0]);
    assert_eq!(b.try_sum_axis(1).unwrap().as_slice(), &[0.0, 0.0, 0.0]);

    // surviving axis length 0 -> empty result, no computation.
    assert!(a.try_sum_axis(1).unwrap().as_slice().is_empty());
    assert!(b.try_sum_axis(0).unwrap().as_slice().is_empty());
}

#[test]
fn axis_out_of_range_keeps_existing_message_not_an_index_panic() {
    // T7: the length-zero guard must not turn an out-of-range axis into a raw
    // index panic; the existing Shape error must survive.
    let a = empty_0x3();
    assert!(matches!(
        a.try_mean_axis(5).unwrap_err(),
        MattenError::Shape {
            operation: "mean_axis",
            ..
        }
    ));
    assert!(matches!(
        a.try_min_axis(5).unwrap_err(),
        MattenError::Shape {
            operation: "min_axis",
            ..
        }
    ));
    assert!(matches!(
        a.try_max_axis(5).unwrap_err(),
        MattenError::Shape {
            operation: "max_axis",
            ..
        }
    ));
}

// ── rank-1 axis reductions collapse to a scalar (RFC-056, deep-review P3) ──

#[test]
fn try_axis_reductions_on_vector_give_scalar() {
    // A rank-1 reduce along axis 0 collapses to a scalar output, matching the
    // panic form (both go through the same reduction path).
    let v = Tensor::from_vec(vec![2.0, 7.0, 4.0]);
    let cases = [
        (v.try_sum_axis(0).unwrap(), v.sum_axis(0)),
        (v.try_mean_axis(0).unwrap(), v.mean_axis(0)),
        (v.try_min_axis(0).unwrap(), v.min_axis(0)),
        (v.try_max_axis(0).unwrap(), v.max_axis(0)),
    ];
    for (got, want) in cases {
        assert!(got.is_scalar());
        assert_eq!(got.shape(), want.shape());
        assert_eq!(got.as_slice(), want.as_slice());
    }
}

// ── P1: shape/data invariant for axis reduction (RFC-128) ──────────────────
//
// for any tensor produced by ANY public constructor or operation:
//     shape.iter().product() == data.len()
//
// Generalizes t3_sum_axis_on_zero_reduced_axis_does_not_abort
// (tests/rfc127_unbounded_dimension.rs): sum_axis uniquely permits a
// zero-length REDUCED axis (RFC-110), so the surviving axes' product is not
// otherwise bounded by the input's own size (a zero anywhere makes the whole
// shape's product 0, so construction always succeeds regardless of how huge
// the other axes are) -- reducing over that zero axis is where the risk
// lives, and this is one of the two sites RFC-127 actually fixed (math.rs's
// axis_reduce).

fn surviving_axis_dim() -> impl Strategy<Value = usize> {
    prop_oneof![
        3 => 0usize..4,
        1 => Just(crate::limits::MAX_REPRESENTABLE_DIMENSION),
        1 => Just(2_000_000usize),
    ]
}

proptest! {
    #[test]
    fn p1_sum_axis_zero_reduced_axis_invariant(
        rank in 1usize..=crate::limits::MAX_NDIM,
        zero_axis_raw in 0usize..crate::limits::MAX_NDIM,
        dims in prop::collection::vec(surviving_axis_dim(), crate::limits::MAX_NDIM),
    ) {
        let zero_axis = zero_axis_raw % rank;
        let mut shape: Vec<usize> = dims.into_iter().take(rank).collect();
        shape[zero_axis] = 0;

        // A generous per-operand budget, matching RFC-127's own
        // t3_sum_axis_on_zero_reduced_axis_does_not_abort: try_zeros's
        // DEFAULT limits bound each dimension individually to MAX_ELEMENTS
        // (not just the product), so a huge surviving axis needs a raised
        // max_elements to construct at all -- the product being 0 does not
        // exempt it from the per-dimension check. Only sum_axis's own
        // hardcoded DEFAULT budget (not this raised one) should be able to
        // reject the surviving-axes' product below.
        let generous = crate::MattenLimits {
            max_elements: usize::MAX / 16,
            ..crate::MattenLimits::default()
        };
        let t = match Tensor::try_zeros_with_limits(&shape, &generous) {
            Ok(t) => t,
            Err(_) => {
                // A genuinely-zero-product shape can still overflow the
                // CHECKED (left-to-right) product computation if a huge
                // dimension is encountered before the zero -- e.g.
                // [huge, 1, huge, 0]: checked_mul overflows multiplying the
                // two huge values together, before ever reaching the
                // trailing zero that would mathematically bring the product
                // back to 0. This is order-dependent, safe (an Err, never a
                // corrupt Tensor or an abort), and orthogonal to what this
                // property targets (sum_axis's own guard, not construction)
                // -- nothing to test for this generated shape.
                return Ok(());
            }
        };

        match t.try_sum_axis(zero_axis) {
            Ok(r) => {
                let expected: usize = r.shape().iter().product();
                prop_assert_eq!(r.as_slice().len(), expected);
            }
            Err(e) => {
                prop_assert!(
                    matches!(e, MattenError::Allocation { .. }),
                    "unexpected error variant: {:?}",
                    e
                );
            }
        }
    }
}
