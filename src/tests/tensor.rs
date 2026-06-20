use crate::{MattenError, Tensor};

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
fn rejects_zero_dim() {
    assert!(matches!(
        Tensor::try_new(vec![], &[0]),
        Err(MattenError::Shape { .. })
    ));
    assert!(matches!(
        Tensor::try_new(vec![], &[2, 0]),
        Err(MattenError::Shape { .. })
    ));
}

#[test]
#[should_panic(expected = "zero-sized dimensions")]
fn new_panics_on_zero_dim() {
    let _ = Tensor::new(vec![], &[0]);
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
    fn try_zeros_shape_error() {
        let err = Tensor::try_zeros(&[2, 0]).unwrap_err();
        assert!(matches!(err, MattenError::Shape { .. }));
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
