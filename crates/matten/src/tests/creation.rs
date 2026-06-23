//! Tests for the RFC-038 creation comfort constructors (`linspace`/`eye`).
//!
//! These validate the design spec: endpoint inclusion and spacing, the
//! `count == 1` / scalar cases, identity structure, and the zero-sized and
//! allocation-limit rejections through `MattenLimits`.

use crate::{MattenError, Tensor};

// ── linspace ───────────────────────────────────────────────────────────────

#[test]
fn linspace_includes_both_endpoints() {
    let t = Tensor::linspace(0.0, 1.0, 5);
    assert_eq!(t.shape(), &[5]);
    assert_eq!(t.as_slice(), &[0.0, 0.25, 0.5, 0.75, 1.0]);
    // Endpoints are exact.
    assert_eq!(t.as_slice()[0], 0.0);
    assert_eq!(t.as_slice()[4], 1.0);
}

#[test]
fn linspace_handles_descending_and_negative() {
    let t = Tensor::linspace(10.0, -10.0, 5);
    assert_eq!(t.as_slice(), &[10.0, 5.0, 0.0, -5.0, -10.0]);
}

#[test]
fn linspace_count_one_returns_start() {
    let t = Tensor::linspace(2.0, 9.0, 1);
    assert_eq!(t.shape(), &[1]);
    assert_eq!(t.as_slice(), &[2.0]);
}

#[test]
fn linspace_rejects_zero_count() {
    let err = Tensor::try_linspace(0.0, 1.0, 0).unwrap_err();
    assert!(matches!(
        err,
        MattenError::Shape {
            operation: "try_linspace",
            ..
        }
    ));
}

#[test]
fn linspace_rejects_oversize() {
    let err = Tensor::try_linspace(0.0, 1.0, usize::MAX).unwrap_err();
    assert!(matches!(err, MattenError::Allocation { .. }));
}

// ── eye ────────────────────────────────────────────────────────────────────

#[test]
fn eye_basic_identity() {
    let i = Tensor::eye(3);
    assert_eq!(i.shape(), &[3, 3]);
    assert_eq!(i.as_slice(), &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
}

#[test]
fn eye_one() {
    let i = Tensor::eye(1);
    assert_eq!(i.shape(), &[1, 1]);
    assert_eq!(i.as_slice(), &[1.0]);
}

#[test]
fn eye_rejects_zero() {
    let err = Tensor::try_eye(0).unwrap_err();
    assert!(matches!(
        err,
        MattenError::Shape {
            operation: "try_eye",
            ..
        }
    ));
}

#[test]
fn eye_rejects_oversize() {
    let err = Tensor::try_eye(usize::MAX).unwrap_err();
    assert!(matches!(err, MattenError::Allocation { .. }));
}
