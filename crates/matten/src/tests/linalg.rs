//! Tests for the RFC-041 linalg core-lite helpers (`norm`/`trace`/`outer`).
//!
//! Validates the design spec: L2/Frobenius norm over all elements with NaN
//! propagation; rank-2 trace with rectangular `min(rows, cols)` behavior; rank-1
//! outer product with allocation limits; and the rank/dynamic error policy.

use crate::{MattenError, Tensor};

// ----- norm: L2 / Frobenius over all elements -----

#[test]
fn norm_vector_3_4_is_5() {
    assert_eq!(Tensor::from_vec(vec![3.0, 4.0]).norm(), 5.0);
}

#[test]
fn norm_matrix_uses_all_elements() {
    // Frobenius norm of [[1,2],[2,4]] = sqrt(1 + 4 + 4 + 16) = sqrt(25) = 5.
    let m = Tensor::new(vec![1.0, 2.0, 2.0, 4.0], &[2, 2]);
    assert!((m.norm() - 5.0).abs() < 1e-12);
}

#[test]
fn norm_scalar() {
    // sqrt((-7)^2) = 7
    assert_eq!(Tensor::scalar(-7.0).norm(), 7.0);
}

#[test]
fn norm_nan_propagates() {
    assert!(Tensor::from_vec(vec![1.0, f64::NAN, 2.0]).norm().is_nan());
}

// ----- trace: rank-2, rectangular via min(rows, cols) -----

#[test]
fn trace_square() {
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    assert_eq!(m.trace(), 5.0); // 1 + 4
}

#[test]
fn trace_rectangular_rows_lt_cols() {
    // [2, 3]: min(2, 3) = 2; diagonal = self[0,0] + self[1,1] = 1 + 5 = 6.
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    assert_eq!(m.trace(), 6.0);
}

#[test]
fn trace_rectangular_rows_gt_cols() {
    // [3, 2]: min(3, 2) = 2; diagonal = self[0,0] + self[1,1] = 1 + 4 = 5.
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[3, 2]);
    assert_eq!(m.trace(), 5.0);
}

#[test]
fn trace_rank1_is_shape() {
    let v = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    assert!(matches!(
        v.try_trace().unwrap_err(),
        MattenError::Shape {
            operation: "trace",
            ..
        }
    ));
}

#[test]
fn trace_rank3_is_shape() {
    let t = Tensor::new(vec![1.0; 8], &[2, 2, 2]);
    assert!(matches!(
        t.try_trace().unwrap_err(),
        MattenError::Shape {
            operation: "trace",
            ..
        }
    ));
}

// ----- outer: rank-1 x rank-1 -> [m, n] -----

#[test]
fn outer_basic_values() {
    let a = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    let b = Tensor::from_vec(vec![4.0, 5.0]);
    let o = a.outer(&b);
    assert_eq!(o.shape(), &[3, 2]);
    // out[i,j] = a[i] * b[j]
    assert_eq!(o.as_slice(), &[4.0, 5.0, 8.0, 10.0, 12.0, 15.0]);
}

#[test]
fn outer_non_square_shape() {
    let a = Tensor::from_vec(vec![2.0, 3.0, 4.0, 5.0]); // len 4
    let b = Tensor::from_vec(vec![10.0, 20.0]); // len 2
    let o = a.outer(&b);
    assert_eq!(o.shape(), &[4, 2]);
    assert_eq!(
        o.as_slice(),
        &[20.0, 40.0, 30.0, 60.0, 40.0, 80.0, 50.0, 100.0]
    );
}

#[test]
fn outer_rejects_non_rank1_lhs() {
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let v = Tensor::from_vec(vec![1.0, 2.0]);
    assert!(matches!(
        m.try_outer(&v).unwrap_err(),
        MattenError::Shape {
            operation: "outer",
            ..
        }
    ));
}

#[test]
fn outer_rejects_non_rank1_rhs() {
    let v = Tensor::from_vec(vec![1.0, 2.0]);
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    assert!(matches!(
        v.try_outer(&m).unwrap_err(),
        MattenError::Shape {
            operation: "outer",
            ..
        }
    ));
}

#[test]
fn outer_respects_allocation_limit() {
    // MAX_ELEMENTS default is 1 << 20 (1_048_576). 1025 * 1025 = 1_050_625 > limit,
    // while each input vector (1025 elements) is well within bounds.
    let a = Tensor::from_vec(vec![1.0; 1025]);
    let b = Tensor::from_vec(vec![1.0; 1025]);
    assert!(matches!(
        a.try_outer(&b).unwrap_err(),
        MattenError::Allocation { .. }
    ));
}

// ----- dynamic rejection (try_* must Err; norm panics) -----

#[cfg(feature = "dynamic")]
#[test]
fn trace_and_outer_reject_dynamic() {
    use crate::dynamic::Element;
    let dynamic = Tensor::from_elements(
        vec![
            Element::Float(1.0),
            Element::Float(2.0),
            Element::Float(3.0),
            Element::Float(4.0),
        ],
        &[2, 2],
    );
    assert!(dynamic.is_dynamic());

    assert!(matches!(
        dynamic.try_trace().unwrap_err(),
        MattenError::Unsupported {
            operation: "trace",
            ..
        }
    ));

    let v = Tensor::from_elements(vec![Element::Float(1.0), Element::Float(2.0)], &[2]);
    let numeric = Tensor::from_vec(vec![3.0, 4.0]);
    assert!(matches!(
        numeric.try_outer(&v).unwrap_err(),
        MattenError::Unsupported {
            operation: "outer",
            ..
        }
    ));
}

#[cfg(feature = "dynamic")]
#[test]
#[should_panic(expected = "dynamic")]
fn norm_panics_on_dynamic() {
    use crate::dynamic::Element;
    let dynamic = Tensor::from_elements(vec![Element::Float(3.0), Element::Float(4.0)], &[2]);
    let _ = dynamic.norm();
}

// ── Result-form norm (RFC-055) ────────────────────────────────────────────

#[test]
fn try_norm_matches_panic_form() {
    let t = Tensor::from_vec(vec![3.0, 4.0]);
    assert_eq!(t.try_norm().unwrap(), t.norm());
    assert_eq!(t.try_norm().unwrap(), 5.0);
}

#[test]
fn try_norm_propagates_nan() {
    assert!(
        Tensor::from_vec(vec![1.0, f64::NAN])
            .try_norm()
            .unwrap()
            .is_nan()
    );
}

#[cfg(feature = "dynamic")]
#[test]
fn try_norm_rejects_dynamic() {
    use crate::dynamic::Element;
    let d = Tensor::from_elements(vec![Element::Float(3.0), Element::Float(4.0)], &[2]);
    assert!(matches!(
        d.try_norm().unwrap_err(),
        MattenError::Unsupported {
            operation: "norm",
            ..
        }
    ));
}
