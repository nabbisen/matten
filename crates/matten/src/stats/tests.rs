//! Tests for the RFC-040 statistics reductions (`var`/`std`, `var_axis`/`std_axis`).
//!
//! Validates the design spec: population variance (`ddof = 0`, divide by `n`),
//! two-pass; `std = sqrt(var)`; singleton variance `0.0`; NaN propagation; the
//! empty-tensor policy (zero-sized dims are not constructible); axis reductions that
//! drop the reduced axis; invalid-axis and dynamic error policy.

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
fn empty_tensor_is_not_constructible() {
    // The empty-variance guard exists for completeness, but `matten` forbids
    // zero-sized dimensions, so an empty tensor cannot be built to reach it.
    assert!(matches!(
        Tensor::try_new(vec![], &[0]).unwrap_err(),
        MattenError::Shape { .. }
    ));
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
