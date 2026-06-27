//! Tests for the RFC-038 selection reductions (`argmin`/`argmax`).
//!
//! These validate the design spec: flat row-major index, first-occurrence tie-break,
//! and the NaN/dynamic policy (selection branch → `InvalidArgument`/panic on NaN).

use crate::Tensor;

#[test]
fn argmin_argmax_basic() {
    let t = Tensor::from_vec(vec![3.0, 1.0, 5.0, 2.0]);
    assert_eq!(t.argmin(), 1);
    assert_eq!(t.argmax(), 2);
}

#[test]
fn ties_return_first_occurrence() {
    assert_eq!(Tensor::from_vec(vec![1.0, 1.0, 3.0]).argmin(), 0);
    assert_eq!(Tensor::from_vec(vec![5.0, 5.0, 2.0]).argmax(), 0);
}

#[test]
fn index_is_flat_row_major() {
    // 2x3; smallest (0.0) at flat index 4, largest (9.0) at flat index 1.
    let t = Tensor::new(vec![2.0, 9.0, 3.0, 1.0, 0.0, 4.0], &[2, 3]);
    assert_eq!(t.argmin(), 4);
    assert_eq!(t.argmax(), 1);
}

#[test]
fn scalar_returns_zero() {
    assert_eq!(Tensor::scalar(7.0).argmin(), 0);
    assert_eq!(Tensor::scalar(7.0).argmax(), 0);
}

#[test]
fn try_forms_reject_nan() {
    let t = Tensor::from_vec(vec![1.0, f64::NAN, 3.0]);
    assert!(matches!(
        t.try_argmin().unwrap_err(),
        crate::MattenError::InvalidArgument {
            operation: "argmin",
            ..
        }
    ));
    assert!(matches!(
        t.try_argmax().unwrap_err(),
        crate::MattenError::InvalidArgument {
            operation: "argmax",
            ..
        }
    ));
}

#[test]
#[should_panic(expected = "undefined for tensors containing NaN")]
fn argmin_panics_on_nan() {
    let _ = Tensor::from_vec(vec![1.0, f64::NAN]).argmin();
}

// ── dynamic rejection ──────────────────────────────────────────────────────

#[cfg(feature = "dynamic")]
mod dynamic_rejection {
    use crate::Tensor;
    use crate::dynamic::Element;

    fn dyn1() -> Tensor {
        Tensor::from_elements(vec![Element::Int(1), Element::Int(2)], &[2])
    }

    #[test]
    fn argmax_panics_on_dynamic() {
        assert!(std::panic::catch_unwind(|| dyn1().argmax()).is_err());
    }

    #[test]
    fn try_argmin_unsupported_on_dynamic() {
        assert!(matches!(
            dyn1().try_argmin().unwrap_err(),
            crate::MattenError::Unsupported {
                operation: "argmin",
                ..
            }
        ));
    }
}
