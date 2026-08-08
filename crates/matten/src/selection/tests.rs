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

// ── empty-tensor semantics (RFC-105) ─────────────────────────────────────
//
// Before this RFC, argmin/argmax on an empty tensor panicked with a raw
// Rust slice index error ("index out of bounds: the len is 0 but the index
// is 0"), not a matten error -- defeating the try_ form entirely. The
// fixture MUST be reached via slicing; no constructor accepts a zero-sized
// dimension.

fn empty_2x3() -> Tensor {
    Tensor::new(vec![1., 2., 3., 4., 5., 6.], &[2, 3])
        .slice()
        .range(0..0)
        .all()
        .build()
        .unwrap()
}

#[test]
fn try_argmin_argmax_are_err_on_empty() {
    let t = empty_2x3();

    let err = t.try_argmin().unwrap_err();
    assert!(matches!(
        err,
        crate::MattenError::InvalidArgument {
            operation: "argmin",
            ..
        }
    ));
    assert_eq!(
        err.to_string(),
        "matten invalid argument error in argmin: self: argmin is undefined for an empty tensor"
    );

    let err = t.try_argmax().unwrap_err();
    assert!(matches!(
        err,
        crate::MattenError::InvalidArgument {
            operation: "argmax",
            ..
        }
    ));
    assert_eq!(
        err.to_string(),
        "matten invalid argument error in argmax: self: argmax is undefined for an empty tensor"
    );
}

// The captured message is asserted, not merely that a panic occurred --
// the defect was WHICH panic: a raw index panic versus a diagnosis.
#[test]
#[should_panic(expected = "argmin is undefined for an empty tensor")]
fn argmin_panics_with_the_sentence_on_empty_not_an_index_panic() {
    let _ = empty_2x3().argmin();
}

#[test]
#[should_panic(expected = "argmax is undefined for an empty tensor")]
fn argmax_panics_with_the_sentence_on_empty_not_an_index_panic() {
    let _ = empty_2x3().argmax();
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
