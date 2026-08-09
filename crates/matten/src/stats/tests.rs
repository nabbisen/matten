//! Tests for the RFC-040 statistics reductions (`var`/`std`, `var_axis`/`std_axis`).
//!
//! Validates the design spec: population variance (`ddof = 0`, divide by `n`),
//! two-pass; `std = sqrt(var)`; singleton variance `0.0`; NaN propagation; the
//! empty-tensor policy (`var`/`std` error rather than compute over zero elements);
//! axis reductions that drop the reduced axis; invalid-axis and dynamic error policy.

use crate::{MattenError, Tensor};

const EPS: f64 = 1e-12;

// ----- scalar var / std: population variance -----

#[test]
fn var_simple_vector_population() {
    // [1,2,3,4]: mean 2.5; population variance = (2.25+0.25+0.25+2.25)/4 = 1.25.
    let t = Tensor::from_vec(vec![1.0, 2.0, 3.0, 4.0]);
    assert_eq!(t.var(), 1.25);
}

#[test]
fn std_simple_vector_population() {
    let t = Tensor::from_vec(vec![1.0, 2.0, 3.0, 4.0]);
    assert!((t.std() - 1.25_f64.sqrt()).abs() < EPS);
}

#[test]
fn var_divides_by_n_not_n_minus_1() {
    // Sample variance of [1,2,3,4] would be 5/3 ≈ 1.667; population is 5/4 = 1.25.
    let t = Tensor::from_vec(vec![1.0, 2.0, 3.0, 4.0]);
    assert!((t.var() - 1.25).abs() < EPS);
    assert!((t.var() - 5.0 / 3.0).abs() > 0.1); // definitely not the sample variance
}

#[test]
fn singleton_variance_is_zero() {
    assert_eq!(Tensor::from_vec(vec![42.0]).var(), 0.0);
    assert_eq!(Tensor::scalar(7.0).var(), 0.0);
    assert_eq!(Tensor::from_vec(vec![42.0]).std(), 0.0);
}

#[test]
fn var_matrix_uses_all_elements() {
    // Flattened [1..=6]: mean 3.5; variance = mean of (2.5²,1.5²,0.5²,0.5²,1.5²,2.5²)
    // = (6.25+2.25+0.25+0.25+2.25+6.25)/6 = 17.5/6.
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    assert!((m.var() - 17.5 / 6.0).abs() < EPS);
}

#[test]
fn var_and_std_nan_propagates() {
    let t = Tensor::from_vec(vec![1.0, f64::NAN, 3.0]);
    assert!(t.var().is_nan());
    assert!(t.std().is_nan());
}

// ----- empty-tensor policy -----

#[test]
fn empty_tensor_is_directly_constructible() {
    // RFC-111 (T8): `matten` used to forbid zero-sized dimensions outright, so
    // an empty tensor could only be reached via slicing (see the test below).
    // checked_shape_len no longer rejects them; try_new(vec![], &[0]) now
    // succeeds directly.
    let t = Tensor::try_new(vec![], &[0]).unwrap();
    assert_eq!(t.shape(), &[0]);
    assert!(t.is_empty());
}

// RFC-105: the empty-variance guard is reachable via slicing, independent of
// whether direct construction is also allowed (RFC-111 made it so). try_var
// and try_std were already correct on this path before RFC-105; this test
// exists to prove that RFC-105's changes elsewhere left them byte-identical.
#[test]
fn var_and_std_are_unchanged_on_a_reachable_empty_tensor() {
    let t = Tensor::new(vec![1., 2., 3., 4., 5., 6.], &[2, 3])
        .slice()
        .range(0..0)
        .all()
        .build()
        .unwrap();
    assert_eq!(t.len(), 0);

    let err = t.try_var().unwrap_err();
    assert!(matches!(
        err,
        MattenError::InvalidArgument {
            operation: "var",
            ..
        }
    ));
    assert_eq!(
        err.to_string(),
        "matten invalid argument error in var: self: variance is undefined for an empty tensor"
    );

    let err = t.try_std().unwrap_err();
    assert!(matches!(
        err,
        MattenError::InvalidArgument {
            operation: "std",
            ..
        }
    ));
    assert_eq!(
        err.to_string(),
        "matten invalid argument error in std: self: standard deviation is undefined for an empty tensor"
    );
}

// ----- axis reductions: drop the reduced axis -----

#[test]
fn var_axis_0() {
    // [[1,2,3],[4,5,6]] axis 0: each column [1,4],[2,5],[3,6] has variance 2.25.
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let v = m.var_axis(0);
    assert_eq!(v.shape(), &[3]);
    for &x in v.as_slice() {
        assert!((x - 2.25).abs() < EPS);
    }
}

#[test]
fn var_axis_1() {
    // axis 1: each row [1,2,3],[4,5,6] has mean 2/5 resp., variance 2/3.
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let v = m.var_axis(1);
    assert_eq!(v.shape(), &[2]);
    for &x in v.as_slice() {
        assert!((x - 2.0 / 3.0).abs() < EPS);
    }
}

#[test]
fn std_axis_0_and_1() {
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    // columns have variance 2.25 -> std 1.5
    let s0 = m.std_axis(0);
    assert_eq!(s0.shape(), &[3]);
    for &x in s0.as_slice() {
        assert!((x - 1.5).abs() < EPS);
    }
    // rows have variance 2/3 -> std sqrt(2/3)
    let s1 = m.std_axis(1);
    assert_eq!(s1.shape(), &[2]);
    for &x in s1.as_slice() {
        assert!((x - (2.0_f64 / 3.0).sqrt()).abs() < EPS);
    }
}

#[test]
fn var_axis_on_vector_reduces_to_scalar() {
    // A rank-1 tensor reduced along axis 0 yields a scalar (shape []).
    let v = Tensor::from_vec(vec![1.0, 2.0, 3.0, 4.0]);
    let r = v.var_axis(0);
    assert_eq!(r.shape(), &[] as &[usize]);
    assert!((r.as_slice()[0] - 1.25).abs() < EPS);
}

#[test]
fn var_axis_nan_propagates_within_slice() {
    // Only column 0 contains NaN; other columns stay finite.
    let m = Tensor::new(vec![1.0, 2.0, 3.0, f64::NAN, 5.0, 6.0], &[2, 3]);
    let v = m.var_axis(0);
    assert!(v.as_slice()[0].is_nan());
    assert!(v.as_slice()[1].is_finite());
    assert!(v.as_slice()[2].is_finite());
}

// ----- empty reduced-axis semantics (RFC-110) -----
//
// No constructor accepts a zero-sized shape; every fixture below is reached
// via slice().range(0..0), never a direct constructor.

fn empty_0x3() -> Tensor {
    Tensor::new(vec![1., 2., 3., 4., 5., 6.], &[2, 3])
        .slice()
        .range(0..0)
        .all()
        .build()
        .unwrap()
}

fn empty_3x0() -> Tensor {
    Tensor::new(vec![1., 2., 3.], &[3, 1])
        .slice()
        .all()
        .range(0..0)
        .build()
        .unwrap()
}

#[test]
fn var_std_axis_error_on_zero_length_reduced_axis() {
    let a = empty_0x3();
    let err = a.try_var_axis(0).unwrap_err();
    assert!(matches!(
        err,
        MattenError::InvalidArgument {
            operation: "var_axis",
            ..
        }
    ));
    assert_eq!(
        err.to_string(),
        "matten invalid argument error in var_axis: axis: variance is undefined for a reduced axis of length 0 (axis 0)"
    );

    let err = a.try_std_axis(0).unwrap_err();
    assert!(matches!(
        err,
        MattenError::InvalidArgument {
            operation: "std_axis",
            ..
        }
    ));
    assert_eq!(
        err.to_string(),
        "matten invalid argument error in std_axis: axis: standard deviation is undefined for a reduced axis of length 0 (axis 0)"
    );

    let b = empty_3x0();
    assert!(matches!(
        b.try_var_axis(1).unwrap_err(),
        MattenError::InvalidArgument {
            operation: "var_axis",
            ..
        }
    ));
    assert!(matches!(
        b.try_std_axis(1).unwrap_err(),
        MattenError::InvalidArgument {
            operation: "std_axis",
            ..
        }
    ));
}

#[test]
#[should_panic(expected = "variance is undefined for a reduced axis of length 0 (axis 0)")]
fn var_axis_panicking_form_carries_the_message() {
    let _ = empty_0x3().var_axis(0);
}

#[test]
#[should_panic(
    expected = "standard deviation is undefined for a reduced axis of length 0 (axis 0)"
)]
fn std_axis_panicking_form_carries_the_message() {
    let _ = empty_0x3().std_axis(0);
}

#[test]
fn var_std_axis_surviving_empty_axis_is_still_ok_both_orientations() {
    let a = empty_0x3(); // reduce axis 1 (length 3); axis 0 (length 0) survives
    let r = a.try_var_axis(1).unwrap();
    assert_eq!(r.shape(), &[0]);
    assert!(r.as_slice().is_empty());
    assert!(a.try_std_axis(1).unwrap().as_slice().is_empty());

    let b = empty_3x0(); // reduce axis 0 (length 3); axis 1 (length 0) survives
    let r = b.try_var_axis(0).unwrap();
    assert_eq!(r.shape(), &[0]);
    assert!(r.as_slice().is_empty());
    assert!(b.try_std_axis(0).unwrap().as_slice().is_empty());
}

#[test]
fn var_axis_out_of_range_still_shape_not_index_panic() {
    let a = empty_0x3();
    assert!(matches!(
        a.try_var_axis(5).unwrap_err(),
        MattenError::Shape {
            operation: "var_axis",
            ..
        }
    ));
    assert!(matches!(
        a.try_std_axis(5).unwrap_err(),
        MattenError::Shape {
            operation: "std_axis",
            ..
        }
    ));
}

// ----- invalid axis -> Shape -----

#[test]
fn var_axis_invalid_is_shape() {
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    assert!(matches!(
        m.try_var_axis(2).unwrap_err(),
        MattenError::Shape {
            operation: "var_axis",
            ..
        }
    ));
    assert!(matches!(
        m.try_std_axis(9).unwrap_err(),
        MattenError::Shape {
            operation: "std_axis",
            ..
        }
    ));
}

// ----- dynamic rejection -----

#[cfg(feature = "dynamic")]
#[test]
fn stats_reject_dynamic() {
    use crate::dynamic::Element;
    let d = Tensor::from_elements(
        vec![
            Element::Float(1.0),
            Element::Float(2.0),
            Element::Float(3.0),
            Element::Float(4.0),
        ],
        &[2, 2],
    );
    assert!(d.is_dynamic());
    assert!(matches!(
        d.try_var().unwrap_err(),
        MattenError::Unsupported {
            operation: "var",
            ..
        }
    ));
    assert!(matches!(
        d.try_std().unwrap_err(),
        MattenError::Unsupported {
            operation: "std",
            ..
        }
    ));
    assert!(matches!(
        d.try_var_axis(0).unwrap_err(),
        MattenError::Unsupported {
            operation: "var_axis",
            ..
        }
    ));
    assert!(matches!(
        d.try_std_axis(0).unwrap_err(),
        MattenError::Unsupported {
            operation: "std_axis",
            ..
        }
    ));
}
