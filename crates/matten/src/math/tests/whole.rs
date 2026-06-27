use crate::Tensor;

// ── whole reductions ──────────────────────────────────────────────────────

#[test]
fn sum_basic() {
    assert_eq!(Tensor::from_vec(vec![1.0, 2.0, 3.0]).sum(), 6.0);
    assert_eq!(Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]).sum(), 10.0);
    assert_eq!(Tensor::scalar(7.0).sum(), 7.0);
}

#[test]
fn sum_nan_propagates() {
    assert!(Tensor::from_vec(vec![1.0, f64::NAN, 3.0]).sum().is_nan());
}

#[test]
fn mean_basic() {
    assert_eq!(Tensor::from_vec(vec![1.0, 2.0, 3.0, 4.0]).mean(), 2.5);
    assert_eq!(Tensor::scalar(5.0).mean(), 5.0);
}

#[test]
fn mean_nan_propagates() {
    assert!(Tensor::from_vec(vec![1.0, f64::NAN]).mean().is_nan());
}

#[test]
fn min_basic() {
    assert_eq!(Tensor::from_vec(vec![3.0, 1.0, 2.0]).min(), 1.0);
    assert_eq!(Tensor::new(vec![5.0, -2.0, 3.0, 0.0], &[2, 2]).min(), -2.0);
}

#[test]
fn max_basic() {
    assert_eq!(Tensor::from_vec(vec![3.0, 1.0, 2.0]).max(), 3.0);
    assert_eq!(Tensor::new(vec![5.0, -2.0, 3.0, 0.0], &[2, 2]).max(), 5.0);
}

#[test]
fn min_nan_returns_nan() {
    // Must return NaN when any element is NaN (not silently ignore it)
    assert!(Tensor::from_vec(vec![1.0, f64::NAN, 3.0]).min().is_nan());
    assert!(Tensor::from_vec(vec![f64::NAN, 99.0]).min().is_nan());
}

#[test]
fn max_nan_returns_nan() {
    assert!(Tensor::from_vec(vec![1.0, f64::NAN, 3.0]).max().is_nan());
}

#[test]
fn min_with_inf() {
    assert_eq!(
        Tensor::from_vec(vec![1.0, f64::NEG_INFINITY, 3.0]).min(),
        f64::NEG_INFINITY
    );
}

#[test]
fn max_with_inf() {
    assert_eq!(
        Tensor::from_vec(vec![1.0, f64::INFINITY, 3.0]).max(),
        f64::INFINITY
    );
}

// ── Result-form scalar reductions (RFC-055) ───────────────────────────────

#[test]
fn try_scalar_reductions_match_panic_forms() {
    let t = Tensor::new(vec![3.0, 1.0, 2.0, 4.0], &[2, 2]);
    assert_eq!(t.try_sum().unwrap(), t.sum());
    assert_eq!(t.try_mean().unwrap(), t.mean());
    assert_eq!(t.try_min().unwrap(), t.min());
    assert_eq!(t.try_max().unwrap(), t.max());
}

#[test]
fn try_scalar_reductions_propagate_nan() {
    let t = Tensor::from_vec(vec![1.0, f64::NAN, 3.0]);
    assert!(t.try_sum().unwrap().is_nan());
    assert!(t.try_mean().unwrap().is_nan());
    assert!(t.try_min().unwrap().is_nan());
    assert!(t.try_max().unwrap().is_nan());
}
