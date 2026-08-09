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

// ── empty-tensor reduction semantics (RFC-105) ───────────────────────────
//
// An empty tensor is reachable via slicing -- that reachability was the
// point RFC-105 needed, so every fixture below goes through
// slice().range(0..0). A constructor reaches one too since RFC-111, but
// that is not what these particular tests are exercising.

use crate::MattenError;

fn empty_2x3() -> Tensor {
    Tensor::new(vec![1., 2., 3., 4., 5., 6.], &[2, 3])
        .slice()
        .range(0..0)
        .all()
        .build()
        .unwrap()
}

#[test]
fn empty_fixture_is_actually_empty() {
    // Guards against risk R3 (RFC-105 handoff SS9): a fixture with len > 0
    // would make every test below pass vacuously.
    let t = empty_2x3();
    assert_eq!(t.shape(), &[0, 3]);
    assert_eq!(t.len(), 0);
    assert!(t.as_slice().is_empty());
}

// T1 + T2: each try_ form returns Err on empty, naming the operation and
// "undefined for an empty tensor".
#[test]
fn try_mean_min_max_are_err_on_empty() {
    let t = empty_2x3();

    let err = t.try_mean().unwrap_err();
    assert!(matches!(
        err,
        MattenError::InvalidArgument {
            operation: "mean",
            ..
        }
    ));
    assert_eq!(
        err.to_string(),
        "matten invalid argument error in mean: self: mean is undefined for an empty tensor"
    );

    let err = t.try_min().unwrap_err();
    assert!(matches!(
        err,
        MattenError::InvalidArgument {
            operation: "min",
            ..
        }
    ));
    assert_eq!(
        err.to_string(),
        "matten invalid argument error in min: self: minimum is undefined for an empty tensor"
    );

    let err = t.try_max().unwrap_err();
    assert!(matches!(
        err,
        MattenError::InvalidArgument {
            operation: "max",
            ..
        }
    ));
    assert_eq!(
        err.to_string(),
        "matten invalid argument error in max: self: maximum is undefined for an empty tensor"
    );
}

// T3: the panicking forms panic with THAT SENTENCE, not a raw index panic.
// The captured message is asserted, not merely that a panic occurred --
// the entire defect was WHICH panic (RFC-105 handoff R2).
#[test]
#[should_panic(expected = "mean is undefined for an empty tensor")]
fn mean_panics_with_the_sentence_on_empty() {
    let _ = empty_2x3().mean();
}

#[test]
#[should_panic(expected = "minimum is undefined for an empty tensor")]
fn min_panics_with_the_sentence_on_empty() {
    let _ = empty_2x3().min();
}

#[test]
#[should_panic(expected = "maximum is undefined for an empty tensor")]
fn max_panics_with_the_sentence_on_empty() {
    let _ = empty_2x3().max();
}

// T5: try_sum still returns a zero on empty (sum of the empty set is the
// additive identity -- explicitly NOT part of this RFC's fix, RFC-105 §4).
#[test]
fn try_sum_still_returns_a_zero_on_empty() {
    let t = empty_2x3();
    assert_eq!(t.try_sum().unwrap(), 0.0);
    assert_eq!(t.sum(), 0.0);
}
