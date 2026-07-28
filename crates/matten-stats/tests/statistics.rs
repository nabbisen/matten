//! Tests validating the RFC-078 §4 policy decisions and §6 test list.
//!
//! All tests live in this single integration file (RFC-078 handoff §6): the
//! crate's entire purpose is three public functions, so exercising them
//! through the public surface is exactly what should be verified.

use matten::Tensor;
use matten_stats::{MattenStatsError, correlation, covariance, quantile};

fn approx(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-9, "expected {b}, got {a}");
}

fn vec_tensor(v: Vec<f64>) -> Tensor {
    let n = v.len();
    Tensor::new(v, &[n])
}

// ── covariance ─────────────────────────────────────────────────────────────

#[test]
fn covariance_known_value() {
    // x = [1,2,3,4], y = [2,4,6,8] (y = 2x); mean_x=2.5, mean_y=5.
    // deviations: (-1.5,-3), (-0.5,-1), (0.5,1), (1.5,3) -> products 4.5,0.5,0.5,4.5 -> sum 10
    // sample covariance = 10 / (4-1) = 10/3
    let x = vec_tensor(vec![1.0, 2.0, 3.0, 4.0]);
    let y = vec_tensor(vec![2.0, 4.0, 6.0, 8.0]);
    approx(covariance(&x, &y).unwrap(), 10.0 / 3.0);
}

#[test]
fn covariance_uses_ddof_1_not_ddof_0() {
    // Same data, but explicitly show the n-1 result differs from the n result
    // -- this locks the RFC-078 SS4.1 policy.
    let x = vec_tensor(vec![1.0, 2.0, 3.0]);
    let y = vec_tensor(vec![1.0, 3.0, 2.0]);
    let n = 3.0;

    let mean_x = 2.0;
    let mean_y = 2.0;
    let sum: f64 = [(1.0, 1.0), (2.0, 3.0), (3.0, 2.0)]
        .iter()
        .map(|&(xi, yi)| (xi - mean_x) * (yi - mean_y))
        .sum();
    let ddof1 = sum / (n - 1.0);
    let ddof0 = sum / n;

    assert_ne!(ddof1, ddof0);
    approx(covariance(&x, &y).unwrap(), ddof1);
}

#[test]
fn covariance_is_symmetric() {
    let x = vec_tensor(vec![1.0, 5.0, 3.0, 9.0]);
    let y = vec_tensor(vec![2.0, 4.0, 6.0, 1.0]);
    approx(covariance(&x, &y).unwrap(), covariance(&y, &x).unwrap());
}

#[test]
fn covariance_of_x_with_itself_is_sample_variance() {
    let x = vec_tensor(vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]);
    let data = x.as_slice();
    let n = data.len() as f64;
    let mean = data.iter().sum::<f64>() / n;
    let expected = data.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1.0);
    approx(covariance(&x, &x).unwrap(), expected);
}

#[test]
fn covariance_length_mismatch_is_an_error() {
    let x = vec_tensor(vec![1.0, 2.0, 3.0]);
    let y = vec_tensor(vec![1.0, 2.0]);
    let err = covariance(&x, &y).unwrap_err();
    assert!(matches!(
        err,
        MattenStatsError::LengthMismatch { left: 3, right: 2 }
    ));
}

#[test]
fn covariance_fewer_than_two_elements_is_an_error() {
    // matten::Tensor cannot represent a zero-element tensor at all --
    // `try_new`/`new` reject every zero-sized dimension unconditionally
    // (see compatibility.md's "is_empty()" entry). So the only reachable
    // "too few elements" case for covariance/correlation is exactly one
    // element per input, where the `n - 1` divisor would be zero.
    let x = vec_tensor(vec![1.0]);
    let y = vec_tensor(vec![2.0]);
    assert!(matches!(
        covariance(&x, &y).unwrap_err(),
        MattenStatsError::Empty
    ));
}

#[test]
fn covariance_non_finite_input_is_an_error() {
    let x = vec_tensor(vec![1.0, f64::NAN, 3.0]);
    let y = vec_tensor(vec![1.0, 2.0, 3.0]);
    assert!(matches!(
        covariance(&x, &y).unwrap_err(),
        MattenStatsError::NonFiniteValue
    ));

    let x_inf = vec_tensor(vec![1.0, f64::INFINITY, 3.0]);
    assert!(matches!(
        covariance(&x_inf, &y).unwrap_err(),
        MattenStatsError::NonFiniteValue
    ));
}

// ── correlation ────────────────────────────────────────────────────────────

#[test]
fn correlation_perfect_positive() {
    let x = vec_tensor(vec![1.0, 2.0, 3.0, 4.0]);
    let y = vec_tensor(vec![2.0, 4.0, 6.0, 8.0]); // y = 2x
    approx(correlation(&x, &y).unwrap(), 1.0);
}

#[test]
fn correlation_perfect_negative() {
    let x = vec_tensor(vec![1.0, 2.0, 3.0, 4.0]);
    let y = vec_tensor(vec![8.0, 6.0, 4.0, 2.0]); // y = -2x + 10
    approx(correlation(&x, &y).unwrap(), -1.0);
}

#[test]
fn correlation_known_intermediate_value() {
    // x = [1,2,3,4,5], y = [2,1,4,3,5]
    let x = vec_tensor(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let y = vec_tensor(vec![2.0, 1.0, 4.0, 3.0, 5.0]);
    let xs = [1.0, 2.0, 3.0, 4.0, 5.0];
    let ys = [2.0, 1.0, 4.0, 3.0, 5.0];
    let n = xs.len() as f64;
    let mean_x = xs.iter().sum::<f64>() / n;
    let mean_y = ys.iter().sum::<f64>() / n;
    let mut cov_sum = 0.0;
    let mut var_x = 0.0;
    let mut var_y = 0.0;
    for (&xi, &yi) in xs.iter().zip(ys.iter()) {
        let dx = xi - mean_x;
        let dy = yi - mean_y;
        cov_sum += dx * dy;
        var_x += dx * dx;
        var_y += dy * dy;
    }
    let expected = cov_sum / (var_x * var_y).sqrt();
    approx(correlation(&x, &y).unwrap(), expected);
}

#[test]
fn correlation_is_bounded_in_unit_interval() {
    let cases: Vec<(Vec<f64>, Vec<f64>)> = vec![
        (vec![1.0, 2.0, 3.0], vec![3.0, 1.0, 2.0]),
        (vec![5.0, 1.0, 4.0, 2.0], vec![2.0, 2.0, 9.0, 1.0]),
        (
            vec![10.0, 20.0, 30.0, 40.0, 50.0],
            vec![1.0, 1.0, 1.0, 1.0, 2.0],
        ),
    ];
    for (xs, ys) in cases {
        let r = correlation(&vec_tensor(xs), &vec_tensor(ys)).unwrap();
        assert!((-1.0..=1.0).contains(&r), "r = {r} out of bounds");
    }
}

#[test]
fn correlation_zero_variance_is_explicit_error_not_nan() {
    let x = vec_tensor(vec![1.0, 2.0, 3.0]);
    let constant = vec_tensor(vec![5.0, 5.0, 5.0]);
    let err = correlation(&x, &constant).unwrap_err();
    assert!(matches!(err, MattenStatsError::ZeroVariance));
}

#[test]
fn correlation_is_invariant_to_ddof_choice() {
    // Compute correlation both the ddof=1 way (via `correlation`) and manually
    // the ddof=0 way, to document why RFC-078 SS4.1 only affects covariance.
    let xs = [1.0, 2.0, 3.0, 4.0, 5.0];
    let ys = [5.0, 3.0, 4.0, 2.0, 1.0];
    let n = xs.len() as f64;
    let mean_x = xs.iter().sum::<f64>() / n;
    let mean_y = ys.iter().sum::<f64>() / n;
    let mut cov_sum = 0.0;
    let mut var_x = 0.0;
    let mut var_y = 0.0;
    for (&xi, &yi) in xs.iter().zip(ys.iter()) {
        let dx = xi - mean_x;
        let dy = yi - mean_y;
        cov_sum += dx * dy;
        var_x += dx * dx;
        var_y += dy * dy;
    }
    let ddof0 = (cov_sum / n) / ((var_x / n).sqrt() * (var_y / n).sqrt());
    let ddof1 = (cov_sum / (n - 1.0)) / ((var_x / (n - 1.0)).sqrt() * (var_y / (n - 1.0)).sqrt());
    approx(ddof0, ddof1);

    let x = vec_tensor(xs.to_vec());
    let y = vec_tensor(ys.to_vec());
    approx(correlation(&x, &y).unwrap(), ddof0);
}

// ── quantile ───────────────────────────────────────────────────────────────

#[test]
fn quantile_boundaries_and_odd_length_median() {
    let x = vec_tensor(vec![5.0, 1.0, 3.0]); // sorted: [1,3,5]
    approx(quantile(&x, 0.0).unwrap(), 1.0);
    approx(quantile(&x, 1.0).unwrap(), 5.0);
    approx(quantile(&x, 0.5).unwrap(), 3.0); // exact middle
}

#[test]
fn quantile_even_length_median_is_interpolated() {
    let x = vec_tensor(vec![10.0, 20.0, 30.0, 40.0]);
    approx(quantile(&x, 0.5).unwrap(), 25.0);
}

#[test]
fn quantile_known_non_midpoint_case() {
    // sorted [10,20,30,40]; q=0.25 -> h = 3*0.25 = 0.75 -> lo=0, hi=1
    // result = 10 + 0.75*(20-10) = 17.5
    let x = vec_tensor(vec![40.0, 10.0, 30.0, 20.0]);
    approx(quantile(&x, 0.25).unwrap(), 17.5);
}

#[test]
fn quantile_is_independent_of_input_order() {
    let sorted = vec_tensor(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let shuffled = vec_tensor(vec![3.0, 1.0, 5.0, 2.0, 4.0]);
    for q in [0.0, 0.1, 0.5, 0.9, 1.0] {
        approx(
            quantile(&sorted, q).unwrap(),
            quantile(&shuffled, q).unwrap(),
        );
    }
}

#[test]
fn quantile_does_not_mutate_input() {
    let x = vec_tensor(vec![3.0, 1.0, 2.0]);
    let before = x.as_slice().to_vec();
    let _ = quantile(&x, 0.5).unwrap();
    assert_eq!(x.as_slice(), before.as_slice());
}

#[test]
fn quantile_invalid_q_is_an_error() {
    let x = vec_tensor(vec![1.0, 2.0, 3.0]);
    assert!(matches!(
        quantile(&x, -0.1).unwrap_err(),
        MattenStatsError::InvalidQuantile(_)
    ));
    assert!(matches!(
        quantile(&x, 1.1).unwrap_err(),
        MattenStatsError::InvalidQuantile(_)
    ));
    assert!(matches!(
        quantile(&x, f64::NAN).unwrap_err(),
        MattenStatsError::InvalidQuantile(_)
    ));
    assert!(matches!(
        quantile(&x, f64::INFINITY).unwrap_err(),
        MattenStatsError::InvalidQuantile(_)
    ));
}

#[test]
fn quantile_non_finite_input_value_is_an_error() {
    let x = vec_tensor(vec![1.0, f64::NAN, 3.0]);
    assert!(matches!(
        quantile(&x, 0.5).unwrap_err(),
        MattenStatsError::NonFiniteValue
    ));
}

// ── shared ─────────────────────────────────────────────────────────────────

// NOTE: RFC-078 handoff §6 asks for "empty tensor -> Empty for all three",
// but `matten::Tensor::try_new`/`new` unconditionally reject every zero-sized
// dimension (see compatibility.md's `is_empty()` entry) -- a genuinely empty
// Tensor cannot be constructed at all, so that exact scenario is untestable
// through the public API. `MattenStatsError::Empty`'s "empty tensor" wording
// is retained defensively (in case that invariant is ever relaxed upstream),
// but the only path to it that a caller can actually reach today is
// covariance/correlation's `n < 2` case, covered above by
// `covariance_fewer_than_two_elements_is_an_error`. `quantile`'s `x.len() ==
// 0` branch is unreachable dead code under matten's current shape model.

#[cfg(feature = "dynamic")]
#[test]
fn dynamic_tensor_is_rejected_for_all_three() {
    use matten::Element;

    let dynamic = Tensor::from_elements(vec![Element::Float(1.0), Element::Float(2.0)], &[2]);
    assert!(matches!(
        covariance(&dynamic, &dynamic).unwrap_err(),
        MattenStatsError::DynamicTensor
    ));
    assert!(matches!(
        correlation(&dynamic, &dynamic).unwrap_err(),
        MattenStatsError::DynamicTensor
    ));
    assert!(matches!(
        quantile(&dynamic, 0.5).unwrap_err(),
        MattenStatsError::DynamicTensor
    ));
}
