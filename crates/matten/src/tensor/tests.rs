use crate::{MattenError, Tensor};
use proptest::prelude::*;

// ---- construction & inspection (M1) -------------------------------------

#[test]
fn constructs_and_inspects() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    assert_eq!(t.shape(), &[2, 2]);
    assert_eq!(t.len(), 4);
    assert_eq!(t.ndim(), 2);
    assert_eq!(t.as_slice(), &[1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn scalar_constructor() {
    let s = Tensor::scalar(42.0);
    assert!(s.shape().is_empty());
    assert_eq!(s.len(), 1);
    assert_eq!(s.ndim(), 0);
    assert!(s.is_scalar());
    assert_eq!(s.as_slice(), &[42.0]);
}

#[test]
fn shape_predicates() {
    assert!(Tensor::scalar(1.0).is_scalar());
    assert!(Tensor::new(vec![1.0, 2.0], &[2]).is_vector());
    assert!(Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]).is_matrix());
    let t3 = Tensor::new(vec![0.0; 8], &[2, 2, 2]);
    assert!(!t3.is_scalar());
    assert!(!t3.is_vector());
    assert!(!t3.is_matrix());
}

// RFC-108: is_empty() is reachable via slicing even though no constructor
// accepts a zero-sized shape directly.
#[test]
fn is_empty_true_on_sliced_empty_tensor() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3])
        .slice()
        .range(0..0)
        .all()
        .build()
        .unwrap();
    assert_eq!(t.shape(), &[0, 3]);
    assert_eq!(t.len(), 0);
    assert!(t.is_empty());
}

#[test]
fn is_empty_false_on_scalar() {
    // A rank-0 scalar has len() == 1 and is therefore never empty (RFC §5 R3).
    assert!(!Tensor::scalar(0.0).is_empty());
}

#[test]
fn is_empty_false_on_ordinary_tensor() {
    assert!(!Tensor::new(vec![1.0, 2.0], &[2]).is_empty());
    assert!(!Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]).is_empty());
}

// RFC-031: is_dynamic() is unconditionally available.
// When the `dynamic` feature is off, it must always return false.
// When it is on, numeric tensors return false and dynamic tensors return true
// (covered by the dynamic sub-suite below).
#[test]
fn is_dynamic_false_for_numeric_tensor() {
    assert!(!Tensor::scalar(1.0).is_dynamic());
    assert!(!Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]).is_dynamic());
    assert!(!Tensor::zeros(&[3, 3]).is_dynamic());
}

#[test]
fn to_vec_returns_owned_copy() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);
    assert_eq!(t.to_vec(), vec![1.0, 2.0, 3.0]);
    assert_eq!(t.as_slice(), &[1.0, 2.0, 3.0]);
}

#[test]
fn clone_and_partial_eq() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let b = a.clone();
    assert_eq!(a, b);
    assert_ne!(a, Tensor::new(vec![1.0, 2.0, 3.0, 5.0], &[2, 2]));
    assert_ne!(a, Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[4]));
}

#[test]
fn debug_is_shape_first() {
    let t = Tensor::new(vec![1.0, 2.0], &[2]);
    assert_eq!(format!("{t:?}"), "Tensor(shape=[2], data=[1.0, 2.0])");
}

// ---- shape validation (M1) ---------------------------------------------

#[test]
fn try_new_rejects_length_mismatch() {
    let err = Tensor::try_new(vec![1.0, 2.0, 3.0], &[2, 2]).unwrap_err();
    assert!(matches!(err, MattenError::Shape { .. }));
}

#[test]
#[should_panic(expected = "matten shape error")]
fn new_panics_on_mismatch() {
    let _ = Tensor::new(vec![1.0], &[2, 2]);
}

#[test]
fn try_new_rejects_shape_overflow() {
    let err = Tensor::try_new(vec![], &[usize::MAX, usize::MAX]).unwrap_err();
    assert!(matches!(err, MattenError::Allocation { .. }));
}

#[test]
fn accepts_zero_dim() {
    // RFC-111 (T8): checked_shape_len no longer rejects a zero-sized dimension.
    let a = Tensor::try_new(vec![], &[0]).unwrap();
    assert_eq!(a.shape(), &[0]);
    assert_eq!(a.len(), 0);
    assert!(a.is_empty());

    let b = Tensor::try_new(vec![], &[2, 0]).unwrap();
    assert_eq!(b.shape(), &[2, 0]);
    assert_eq!(b.len(), 0);
    assert!(b.is_empty());
}

#[test]
fn new_accepts_zero_dim() {
    // RFC-111 (T8): the panicking form no longer panics on a zero-sized shape.
    let t = Tensor::new(vec![], &[0]);
    assert_eq!(t.shape(), &[0]);
    assert!(t.is_empty());
}

#[test]
fn rejects_rank_over_limit() {
    let shape = [1usize; 9]; // rank 9 > MAX_NDIM(8)
    let err = Tensor::try_new(vec![1.0], &shape).unwrap_err();
    assert!(matches!(err, MattenError::Shape { .. }));
}

#[test]
fn accepts_rank_8() {
    let shape = [1usize; 8];
    let t = Tensor::new(vec![1.0], &shape);
    assert_eq!(t.ndim(), 8);
    assert_eq!(t.len(), 1);
}

// ---- fill constructors (M2) -------------------------------------------

#[test]
fn zeros_fills_with_zero() {
    let t = Tensor::zeros(&[2, 3]);
    assert_eq!(t.shape(), &[2, 3]);
    assert!(t.as_slice().iter().all(|&v| v == 0.0));
}

#[test]
fn ones_fills_with_one() {
    let t = Tensor::ones(&[4]);
    assert_eq!(t.len(), 4);
    assert!(t.as_slice().iter().all(|&v| v == 1.0));
}

#[test]
fn full_fills_with_value() {
    let t = Tensor::full(&[3, 2], -5.0);
    assert_eq!(t.shape(), &[3, 2]);
    assert!(t.as_slice().iter().all(|&v| v == -5.0));
}

#[test]
fn from_vec_creates_1d() {
    let t = Tensor::from_vec(vec![10.0, 20.0, 30.0]);
    assert_eq!(t.shape(), &[3]);
    assert!(t.is_vector());
    assert_eq!(t.as_slice(), &[10.0, 20.0, 30.0]);
}

// ---- into_vec (M2) -----------------------------------------------------

#[test]
fn into_vec_consumes_tensor() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let v = t.into_vec();
    assert_eq!(v, vec![1.0, 2.0, 3.0, 4.0]);
}

// ---- arange (M2) -------------------------------------------------------

#[test]
fn arange_forward() {
    let t = Tensor::arange(0.0, 5.0, 1.0);
    assert_eq!(t.shape(), &[5]);
    assert_eq!(t.as_slice(), &[0.0, 1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn arange_backward() {
    let t = Tensor::arange(3.0, 0.0, -1.0);
    assert_eq!(t.as_slice(), &[3.0, 2.0, 1.0]);
}

#[test]
fn arange_fractional_step() {
    let t = Tensor::arange(0.0, 1.0, 0.5);
    assert_eq!(t.len(), 2);
    assert!((t.as_slice()[0] - 0.0).abs() < 1e-12);
    assert!((t.as_slice()[1] - 0.5).abs() < 1e-12);
}

#[test]
fn try_arange_zero_step_is_err() {
    assert!(matches!(
        Tensor::try_arange(0.0, 5.0, 0.0),
        Err(MattenError::Shape { .. })
    ));
}

#[test]
fn try_arange_nonfinite_step_is_err() {
    assert!(matches!(
        Tensor::try_arange(0.0, 5.0, f64::NAN),
        Err(MattenError::Shape { .. })
    ));
    assert!(matches!(
        Tensor::try_arange(0.0, 5.0, f64::INFINITY),
        Err(MattenError::Shape { .. })
    ));
}

#[test]
fn try_arange_nonfinite_bounds_is_err() {
    assert!(matches!(
        Tensor::try_arange(f64::NAN, 5.0, 1.0),
        Err(MattenError::Shape { .. })
    ));
    assert!(matches!(
        Tensor::try_arange(0.0, f64::INFINITY, 1.0),
        Err(MattenError::Shape { .. })
    ));
}

#[test]
fn try_arange_empty_range_is_err() {
    assert!(matches!(
        Tensor::try_arange(5.0, 3.0, 1.0),
        Err(MattenError::Shape { .. })
    ));
}

#[test]
#[should_panic(expected = "matten shape error")]
fn arange_panics_on_zero_step() {
    let _ = Tensor::arange(0.0, 5.0, 0.0);
}

// ---- get_flat (RFC-007 §10) --------------------------------------------

#[test]
fn get_flat_in_bounds() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    assert_eq!(t.get_flat(0), Some(1.0));
    assert_eq!(t.get_flat(1), Some(2.0));
    assert_eq!(t.get_flat(3), Some(4.0));
}

#[test]
fn get_flat_out_of_bounds_is_none() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    assert_eq!(t.get_flat(4), None);
    assert_eq!(t.get_flat(100), None);
}

#[test]
fn get_flat_matches_as_slice_order() {
    let t = Tensor::new(vec![10.0, 20.0, 30.0], &[3]);
    for (i, &v) in t.as_slice().iter().enumerate() {
        assert_eq!(t.get_flat(i), Some(v));
    }
}

// ---- get_mut / get_flat_mut (RFC-104) -----------------------------------

// T1 + T2: write lands, and read-modify-write works in one expression.
#[test]
fn get_mut_writes_land_in_place() {
    let mut t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    *t.get_mut(&[0, 1]).unwrap() += 1.0;
    assert_eq!(t.get(&[0, 1]), Some(3.0));
    assert_eq!(t.as_slice(), &[1.0, 3.0, 3.0, 4.0]);
}

#[test]
fn get_flat_mut_writes_land_in_place() {
    let mut t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    *t.get_flat_mut(1).unwrap() += 100.0;
    assert_eq!(t.get_flat(1), Some(102.0));
    assert_eq!(t.as_slice(), &[1.0, 102.0, 3.0, 4.0]);
}

// T3: out-of-range returns None and leaves the tensor unchanged.
#[test]
fn get_mut_out_of_range_is_none_and_leaves_tensor_unchanged() {
    let mut t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    assert_eq!(t.get_mut(&[5, 0]), None);
    assert_eq!(t.get_mut(&[0]), None); // wrong rank
    assert_eq!(t.as_slice(), &[1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn get_flat_mut_out_of_range_is_none_and_leaves_tensor_unchanged() {
    let mut t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    assert_eq!(t.get_flat_mut(99), None);
    assert_eq!(t.as_slice(), &[1.0, 2.0, 3.0, 4.0]);
}

// T5: non-square tensor -- get_mut(&[r,c]) and get_flat_mut must agree on the
// same element. A square tensor cannot distinguish a coord/flat transposition
// bug from correct code, since rows and columns have the same stride.
#[test]
fn get_mut_and_get_flat_mut_agree_on_a_non_square_tensor() {
    // [2, 3]: [[0,1,2],[3,4,5]] -- row-major flat index r*3+c
    let mut t = Tensor::new((0..6).map(|x| x as f64).collect(), &[2, 3]);
    *t.get_mut(&[1, 2]).unwrap() = 99.0; // flat index 1*3+2 = 5
    assert_eq!(t.get_flat(5), Some(99.0));

    *t.get_flat_mut(2).unwrap() = 77.0; // coord [0, 2]
    assert_eq!(t.get(&[0, 2]), Some(77.0));
}

// T6 (risk 2): numeric slices are independent owned copies (RFC-008 Phase 1),
// so mutating one must never reach the source. Proven, not assumed.
#[test]
fn mutating_a_numeric_slice_leaves_its_source_unchanged() {
    let source = Tensor::new((0..6).map(|x| x as f64).collect(), &[2, 3]);
    let mut slice = source.slice().index(0).all().build().unwrap();
    *slice.get_mut(&[1]).unwrap() = 999.0;
    assert_eq!(slice.as_slice(), &[0.0, 999.0, 2.0]);
    assert_eq!(source.as_slice(), &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
}

// ---- RFC-018: resource safety limit tests --------------------------------

mod limits_tests {
    use crate::limits::{MAX_ELEMENTS, MAX_NDIM, MattenLimits};
    use crate::{MattenError, Tensor};

    #[test]
    fn try_zeros_success() {
        let t = Tensor::try_zeros(&[3, 4]).unwrap();
        assert_eq!(t.shape(), &[3, 4]);
        assert_eq!(t.as_slice(), &[0.0f64; 12]);
    }

    #[test]
    fn try_ones_success() {
        let t = Tensor::try_ones(&[2, 3]).unwrap();
        assert_eq!(t.as_slice(), &[1.0f64; 6]);
    }

    #[test]
    fn try_full_success() {
        let t = Tensor::try_full(&[2, 2], 7.0).unwrap();
        assert_eq!(t.as_slice(), &[7.0f64; 4]);
    }

    #[test]
    fn try_zeros_accepts_zero_dim() {
        // RFC-111 (T8): a zero-sized dimension used to be a Shape error via
        // checked_shape_len; it now succeeds, empty.
        let t = Tensor::try_zeros(&[2, 0]).unwrap();
        assert_eq!(t.shape(), &[2, 0]);
        assert!(t.is_empty());
    }

    #[test]
    fn try_ones_and_try_full_accept_zero_dim() {
        // RFC-111 T4: zeros/ones/full all accept a zero-sized result.
        let ones = Tensor::try_ones(&[0, 3]).unwrap();
        assert_eq!(ones.shape(), &[0, 3]);
        assert!(ones.is_empty());

        let full = Tensor::try_full(&[3, 0], 7.0).unwrap();
        assert_eq!(full.shape(), &[3, 0]);
        assert!(full.is_empty());
    }

    #[test]
    fn try_ones_rank_too_high() {
        let shape = vec![2usize; MAX_NDIM + 1];
        let err = Tensor::try_ones(&shape).unwrap_err();
        assert!(matches!(err, MattenError::Shape { .. }));
    }

    #[test]
    fn try_full_element_budget_exceeded() {
        let limits = MattenLimits {
            max_dimensions: 8,
            max_elements: 10,
            max_parse_bytes: 1024,
        };
        let err = Tensor::try_full_with_limits(&[100], 0.0, &limits).unwrap_err();
        assert!(matches!(err, MattenError::Allocation { .. }));
    }

    #[test]
    fn mattan_limits_default_absorbs_constants() {
        let lim = MattenLimits::default();
        assert_eq!(lim.max_dimensions, MAX_NDIM);
        assert_eq!(lim.max_elements, MAX_ELEMENTS);
    }

    #[test]
    fn zeros_delegates_to_try_zeros() {
        // panicking zeros must respect the same limits as try_zeros
        let t = Tensor::zeros(&[2, 3]);
        assert_eq!(t.len(), 6);
    }

    #[test]
    fn broadcast_output_budget_checked() {
        // A shape whose product exceeds MAX_ELEMENTS must panic
        let a = Tensor::try_zeros(&[1024]).unwrap();
        let b = Tensor::try_zeros(&[1024]).unwrap();
        // [1024] + [1024] → [1024]: stays within budget, should succeed
        let _ = &a + &b;
    }
}

// ---- P0-1 fix: zeros/ones/full must route through MattenLimits ----------

#[test]
#[should_panic(expected = "matten allocation error")]
fn zeros_panics_when_default_limit_exceeded() {
    use crate::limits::MAX_ELEMENTS;
    let _ = Tensor::zeros(&[MAX_ELEMENTS + 1]);
}

#[test]
#[should_panic(expected = "matten allocation error")]
fn ones_panics_when_default_limit_exceeded() {
    use crate::limits::MAX_ELEMENTS;
    let _ = Tensor::ones(&[MAX_ELEMENTS + 1]);
}

#[test]
#[should_panic(expected = "matten allocation error")]
fn full_panics_when_default_limit_exceeded() {
    use crate::limits::MAX_ELEMENTS;
    let _ = Tensor::full(&[MAX_ELEMENTS + 1], 1.0);
}

// ---- P1: shape/data invariant (RFC-128) ------------------------------------
//
// for any tensor produced by ANY public constructor or operation:
//     shape.iter().product() == data.len()
//
// This is the property whose absence produced RFC-127's Critical: a shape
// like [400_000_000_000, 0] validated and a huge surviving axis later aborted
// the process. `try_zeros` validates the shape (rejecting anything the
// default MattenLimits budget disallows) before allocating, so generating
// shapes FREELY here and asserting only "if Ok, the invariant holds" is
// exactly RFC-128 §4.1's instruction: bound the DATA, not the shape — the
// Err branch below is itself the property's coverage of the rejected cases,
// not a skip (RFC-128 R1/R5).

proptest! {
    #[test]
    fn p1_try_zeros_invariant(shp in crate::proptest_support::shape()) {
        use crate::limits::{MAX_ELEMENTS, MAX_NDIM};

        match Tensor::try_zeros(&shp) {
            Ok(t) => {
                let expected = crate::proptest_support::checked_product(&shp);
                prop_assert_eq!(Some(t.as_slice().len()), expected, "shape {:?}", shp);
                prop_assert_eq!(t.shape(), shp.as_slice());
            }
            Err(_) => {
                // A rejection must have a real cause: too many axes, a
                // single dimension past `MAX_ELEMENTS` (the effective
                // per-dimension bound `try_zeros`'s DEFAULT limits enforce —
                // checked_shape_len bounds each dimension against
                // `min(self.max_elements, MAX_REPRESENTABLE_DIMENSION)`,
                // and MAX_ELEMENTS is far smaller than
                // MAX_REPRESENTABLE_DIMENSION here, so it is the binding
                // one), or the overall product exceeding that same budget
                // (or overflowing usize entirely). Anything else would mean
                // try_zeros spuriously rejected a reasonable shape.
                let rank_too_big = shp.len() > MAX_NDIM;
                let dim_too_big = shp.iter().any(|&d| d > MAX_ELEMENTS);
                let too_big_or_overflow = crate::proptest_support::checked_product(&shp)
                    .is_none_or(|p| p > MAX_ELEMENTS);
                prop_assert!(
                    rank_too_big || dim_too_big || too_big_or_overflow,
                    "try_zeros rejected a shape that should have been representable: {:?}",
                    shp
                );
            }
        }
    }
}
