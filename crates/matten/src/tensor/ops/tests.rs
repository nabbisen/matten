//! Tests for the RFC-038 shape band (`squeeze`/`expand_dims`).
//!
//! These validate the design spec: removing all length-1 axes (including the
//! all-ones → scalar case), inserting a length-1 axis across the valid
//! `0..=ndim` range, and the out-of-range and dynamic rejections.

use crate::{MattenError, Tensor};

// ── squeeze ────────────────────────────────────────────────────────────────

#[test]
fn squeeze_removes_all_unit_axes() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0], &[1, 3, 1]);
    let s = t.squeeze();
    assert_eq!(s.shape(), &[3]);
    assert_eq!(s.as_slice(), &[1.0, 2.0, 3.0]);
}

#[test]
fn squeeze_all_ones_becomes_scalar() {
    let t = Tensor::new(vec![5.0], &[1, 1]);
    let s = t.squeeze();
    assert_eq!(s.shape(), &[] as &[usize]);
    assert!(s.is_scalar());
}

#[test]
fn squeeze_scalar_stays_scalar() {
    assert!(Tensor::scalar(7.0).squeeze().is_scalar());
}

#[test]
fn squeeze_noop_when_no_unit_axes() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    assert_eq!(t.squeeze().shape(), &[2, 3]);
}

// ── expand_dims ────────────────────────────────────────────────────────────

#[test]
fn expand_dims_front_and_back() {
    let t = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    assert_eq!(t.expand_dims(0).shape(), &[1, 3]);
    assert_eq!(t.expand_dims(1).shape(), &[3, 1]);
}

#[test]
fn expand_dims_preserves_data() {
    let t = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    let e = t.expand_dims(1);
    assert_eq!(e.as_slice(), &[1.0, 2.0, 3.0]);
}

#[test]
fn expand_dims_at_ndim_appends_axis() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    // axis == ndim is valid (append).
    assert_eq!(t.expand_dims(2).shape(), &[2, 2, 1]);
}

#[test]
fn expand_dims_on_scalar() {
    assert_eq!(Tensor::scalar(9.0).expand_dims(0).shape(), &[1]);
}

#[test]
fn try_expand_dims_rejects_out_of_range_axis() {
    let t = Tensor::from_vec(vec![1.0, 2.0, 3.0]); // ndim 1, valid axes 0..=1
    let err = t.try_expand_dims(5).unwrap_err();
    assert!(matches!(
        err,
        MattenError::InvalidArgument {
            operation: "expand_dims",
            argument: "axis",
            ..
        }
    ));
}

#[test]
#[should_panic(expected = "out of range")]
fn expand_dims_panics_on_out_of_range_axis() {
    let _ = Tensor::from_vec(vec![1.0]).expand_dims(9);
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
    fn squeeze_panics_on_dynamic() {
        assert!(std::panic::catch_unwind(|| dyn1().squeeze()).is_err());
    }

    #[test]
    fn try_expand_dims_unsupported_on_dynamic() {
        assert!(matches!(
            dyn1().try_expand_dims(0).unwrap_err(),
            crate::MattenError::Unsupported {
                operation: "expand_dims",
                ..
            }
        ));
    }
}
