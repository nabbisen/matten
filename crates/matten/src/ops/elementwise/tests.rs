//! Tests for the RFC-038 elementwise comfort math (`abs`/`sqrt`/`exp`/`ln`/`clip`).
//!
//! These validate the design spec: shape preservation, `f64` NaN/Inf behavior,
//! and the panic-vs-`Result` boundary for invalid arguments and dynamic tensors.

use crate::Tensor;

// ── abs ────────────────────────────────────────────────────────────────────

#[test]
fn abs_basic_and_shape_preserved() {
    let t = Tensor::new(vec![-1.0, 2.0, -3.0, 4.0], &[2, 2]);
    let r = t.abs();
    assert_eq!(r.as_slice(), &[1.0, 2.0, 3.0, 4.0]);
    assert_eq!(r.shape(), &[2, 2]);
}

// ── sqrt ───────────────────────────────────────────────────────────────────

#[test]
fn sqrt_basic() {
    let t = Tensor::from_vec(vec![1.0, 4.0, 9.0]);
    assert_eq!(t.sqrt().as_slice(), &[1.0, 2.0, 3.0]);
}

#[test]
fn sqrt_of_negative_is_nan() {
    let t = Tensor::from_vec(vec![-1.0]);
    assert!(t.sqrt().as_slice()[0].is_nan());
}

// ── exp / ln ───────────────────────────────────────────────────────────────

#[test]
fn exp_basic() {
    let r = Tensor::from_vec(vec![0.0, 1.0]).exp();
    assert_eq!(r.as_slice()[0], 1.0);
    assert!((r.as_slice()[1] - std::f64::consts::E).abs() < 1e-12);
}

#[test]
fn ln_basic_and_edges() {
    let r = Tensor::from_vec(vec![1.0, std::f64::consts::E]).ln();
    assert_eq!(r.as_slice()[0], 0.0);
    assert!((r.as_slice()[1] - 1.0).abs() < 1e-12);

    let edges = Tensor::from_vec(vec![0.0, -1.0]).ln();
    assert_eq!(edges.as_slice()[0], f64::NEG_INFINITY);
    assert!(edges.as_slice()[1].is_nan());
}

// ── clip ───────────────────────────────────────────────────────────────────

#[test]
fn clip_clamps_and_preserves_shape() {
    let t = Tensor::new(vec![-5.0, 0.5, 9.0, 0.25], &[2, 2]);
    let r = t.clip(0.0, 1.0);
    assert_eq!(r.as_slice(), &[0.0, 0.5, 1.0, 0.25]);
    assert_eq!(r.shape(), &[2, 2]);
}

#[test]
#[should_panic(expected = "min must be <= max")]
fn clip_panics_when_min_gt_max() {
    let _ = Tensor::from_vec(vec![1.0]).clip(1.0, 0.0);
}

#[test]
fn try_clip_ok() {
    let t = Tensor::from_vec(vec![-5.0, 0.5, 9.0]);
    assert_eq!(t.try_clip(0.0, 1.0).unwrap().as_slice(), &[0.0, 0.5, 1.0]);
}

#[test]
fn try_clip_rejects_min_gt_max() {
    let err = Tensor::from_vec(vec![1.0]).try_clip(1.0, 0.0).unwrap_err();
    assert!(matches!(
        err,
        crate::MattenError::InvalidArgument {
            operation: "clip",
            ..
        }
    ));
}

#[test]
fn try_clip_rejects_nan_bound() {
    let err = Tensor::from_vec(vec![1.0])
        .try_clip(f64::NAN, 1.0)
        .unwrap_err();
    assert!(matches!(err, crate::MattenError::InvalidArgument { .. }));
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
    fn abs_panics_on_dynamic() {
        assert!(std::panic::catch_unwind(|| dyn1().abs()).is_err());
    }

    #[test]
    fn try_clip_unsupported_on_dynamic() {
        let err = dyn1().try_clip(0.0, 1.0).unwrap_err();
        assert!(matches!(
            err,
            crate::MattenError::Unsupported {
                operation: "clip",
                ..
            }
        ));
    }
}
