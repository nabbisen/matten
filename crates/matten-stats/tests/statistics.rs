//! Tests validating the RFC-078 §4 and RFC-083 §4 policy decisions and their
//! handoffs' §6 test lists.
//!
//! All tests live in this single integration file (RFC-078 handoff §6): the
//! crate's entire purpose is a small set of public functions, so exercising
//! them through the public surface is exactly what should be verified.

use matten::{MattenLimits, Tensor};
use matten_stats::{
    MattenStatsError, correlation, covariance, covariance_population, histogram, kurtosis,
    quantile, skewness,
};

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

// ── covariance_population ────────────────────────────────────────────────────

#[test]
fn covariance_population_matches_sample_identity() {
    // x = [1,2,3,7], y = [2,4,5,11]; mean_x=3.25, mean_y=5.5.
    // deviations: (-2.25,-3.5), (-1.25,-1.5), (-0.25,-0.5), (3.75,5.5)
    // products: 7.875, 1.875, 0.125, 20.625 -> sum 30.5
    // cov_sample = 30.5 / (4-1) = 30.5/3; cov_pop = 30.5/4
    // identity: cov_pop * n == cov_sample * (n - 1), both sides == 30.5 exactly.
    let x = vec_tensor(vec![1.0, 2.0, 3.0, 7.0]);
    let y = vec_tensor(vec![2.0, 4.0, 5.0, 11.0]);
    let n = 4.0;

    let cov_pop = covariance_population(&x, &y).unwrap();
    let cov_sample = covariance(&x, &y).unwrap();

    approx(cov_pop * n, 30.5);
    approx(cov_sample * (n - 1.0), 30.5);
    approx(cov_pop * n, cov_sample * (n - 1.0));
}

#[test]
fn covariance_population_with_n_1_returns_zero() {
    // Unlike `covariance` (divisor n - 1), `covariance_population`'s divisor
    // is n, so a single-element pair is well-defined and returns 0.0 exactly.
    let x = vec_tensor(vec![5.0]);
    let y = vec_tensor(vec![9.0]);
    assert_eq!(covariance_population(&x, &y).unwrap(), 0.0);
}

#[test]
fn covariance_with_n_1_still_rejects_as_empty() {
    // The two minimums differ on purpose (RFC-083 §4.3): covariance needs
    // n >= 2 (its n - 1 divisor would be zero), covariance_population needs
    // only n >= 1.
    let x = vec_tensor(vec![5.0]);
    let y = vec_tensor(vec![9.0]);
    assert!(matches!(
        covariance(&x, &y).unwrap_err(),
        MattenStatsError::Empty
    ));
}

#[test]
fn covariance_population_length_mismatch_is_an_error() {
    let x = vec_tensor(vec![1.0, 2.0, 3.0]);
    let y = vec_tensor(vec![1.0, 2.0]);
    assert!(matches!(
        covariance_population(&x, &y).unwrap_err(),
        MattenStatsError::LengthMismatch { left: 3, right: 2 }
    ));
}

#[test]
fn covariance_population_non_finite_input_is_an_error() {
    let x = vec_tensor(vec![1.0, f64::NAN, 3.0]);
    let y = vec_tensor(vec![1.0, 2.0, 3.0]);
    assert!(matches!(
        covariance_population(&x, &y).unwrap_err(),
        MattenStatsError::NonFiniteValue
    ));

    let x_inf = vec_tensor(vec![1.0, f64::INFINITY, 3.0]);
    assert!(matches!(
        covariance_population(&x_inf, &y).unwrap_err(),
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

// ── skewness / kurtosis ──────────────────────────────────────────────────────

#[test]
fn skewness_of_symmetric_input_is_exactly_zero() {
    let x = vec_tensor(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    assert_eq!(skewness(&x).unwrap(), 0.0);
}

#[test]
fn skewness_of_asymmetric_fixture_matches_hand_computed_value() {
    // x = [1,2,3,10]; mean = 16/4 = 4.
    // deviations: -3,-2,-1,6 -> d^2: 9,4,1,36 -> sum 50 -> m2 = 50/4 = 12.5
    //                          -> d^3: -27,-8,-1,216 -> sum 180 -> m3 = 180/4 = 45
    // skewness = m3 / m2^1.5 = 45 / 12.5^1.5
    let x = vec_tensor(vec![1.0, 2.0, 3.0, 10.0]);
    let expected = 45.0 / 12.5_f64.powf(1.5);
    approx(skewness(&x).unwrap(), expected);
}

#[test]
fn kurtosis_pins_the_excess_fisher_convention() {
    // x = [1,2,3,4,5]; m2 = 2, m4 = 6.8; raw (Pearson) ratio = 6.8/4 = 1.7;
    // excess (Fisher) = 1.7 - 3 = -1.3. This is the test that distinguishes
    // the two conventions unambiguously -- do NOT replace it with a
    // "near enough to 0" tolerance check on some other fixture: small
    // discrete samples do not sit near 0 (see the 9-point fixture in
    // kurtosis_small_symmetric_sample_is_not_near_zero below), so such a
    // test would either fail or get its tolerance widened until it stops
    // meaning anything.
    let x = vec_tensor(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    assert_eq!(kurtosis(&x).unwrap(), -1.3);
}

#[test]
fn kurtosis_small_symmetric_sample_is_not_near_zero() {
    // A 9-point symmetric fixture [1..9]: mean=5, deviations -4..4.
    // m2 = (16+9+4+1+0+1+4+9+16)/9 = 60/9; m4 = (256+81+16+1+0+1+16+81+256)/9 = 708/9.
    // excess = (708/9) / (60/9)^2 - 3 = 1.77 - 3 = -1.23 -- demonstrating why
    // "normal-ish small sample -> near 0" is not a valid test for the excess
    // convention (RFC-083 handoff SS6).
    let x = vec_tensor(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    approx(kurtosis(&x).unwrap(), -1.23);
}

#[test]
fn skewness_and_kurtosis_zero_variance_is_explicit_error_not_nan() {
    let constant = vec_tensor(vec![5.0, 5.0, 5.0]);
    assert!(matches!(
        skewness(&constant).unwrap_err(),
        MattenStatsError::ZeroVariance
    ));
    assert!(matches!(
        kurtosis(&constant).unwrap_err(),
        MattenStatsError::ZeroVariance
    ));
}

#[test]
fn skewness_and_kurtosis_single_element_is_empty() {
    // Both require n >= 2 (m2 > 0 is otherwise unreachable); the
    // zero-element case is untestable through the public API, same as
    // covariance/correlation -- see the NOTE below.
    let x = vec_tensor(vec![3.0]);
    assert!(matches!(skewness(&x).unwrap_err(), MattenStatsError::Empty));
    assert!(matches!(kurtosis(&x).unwrap_err(), MattenStatsError::Empty));
}

#[test]
fn skewness_and_kurtosis_non_finite_input_is_an_error() {
    let nan = vec_tensor(vec![1.0, f64::NAN, 3.0]);
    assert!(matches!(
        skewness(&nan).unwrap_err(),
        MattenStatsError::NonFiniteValue
    ));
    assert!(matches!(
        kurtosis(&nan).unwrap_err(),
        MattenStatsError::NonFiniteValue
    ));

    let inf = vec_tensor(vec![1.0, f64::INFINITY, 3.0]);
    assert!(matches!(
        skewness(&inf).unwrap_err(),
        MattenStatsError::NonFiniteValue
    ));
    assert!(matches!(
        kurtosis(&inf).unwrap_err(),
        MattenStatsError::NonFiniteValue
    ));
}

// ── histogram (RFC-090) ───────────────────────────────────────────────────

#[test]
fn histogram_shape_invariant() {
    let x = vec_tensor(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let h = histogram(&x, 4).unwrap();
    assert_eq!(h.counts.len(), 4);
    assert_eq!(h.edges.len(), 5);
}

#[test]
fn histogram_sum_invariant_including_max_on_final_edge() {
    // The maximum value (5.0) lands exactly on `edges[bins]`. Without the
    // closed-last-bin rule (RFC-090 §4.3) it would fall in no bin and the
    // sum would be x.len() - 1, not x.len().
    let x = vec_tensor(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let h = histogram(&x, 4).unwrap();
    assert_eq!(h.counts.iter().sum::<usize>(), x.len());

    // A second shape: non-round bin count, so no value lands on an interior
    // edge except by construction of this exact case's max.
    let y = vec_tensor(vec![0.0, 0.2, 0.5, 0.9, 1.0]);
    let hy = histogram(&y, 3).unwrap();
    assert_eq!(hy.counts.iter().sum::<usize>(), y.len());
}

#[test]
fn histogram_hand_computed_counts_and_edges() {
    // x in [0, 10], 5 bins -> width 2: edges [0, 2, 4, 6, 8, 10].
    // Values:      0   1   2   3   5   7   9   10
    // Bin index:   0   0   1   1   2   3   4   4 (10 clamped into the last bin)
    let x = vec_tensor(vec![0.0, 1.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]);
    let h = histogram(&x, 5).unwrap();
    assert_eq!(h.edges, vec![0.0, 2.0, 4.0, 6.0, 8.0, 10.0]);
    assert_eq!(h.counts, vec![2, 2, 1, 1, 2]);
}

#[test]
fn histogram_edges_are_evenly_spaced_from_min_to_max() {
    let x = vec_tensor(vec![3.0, 1.0, 4.0, 1.0, 5.0, 9.0, 2.0, 6.0]);
    let h = histogram(&x, 7).unwrap();

    assert_eq!(h.edges[0], 1.0); // min(x)
    assert_eq!(*h.edges.last().unwrap(), 9.0); // max(x), exact (§3)

    let widths: Vec<f64> = h.edges.windows(2).map(|w| w[1] - w[0]).collect();
    for w in &widths {
        approx(*w, widths[0]);
    }
}

#[test]
fn histogram_bins_one_puts_everything_in_one_bin() {
    let x = vec_tensor(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let h = histogram(&x, 1).unwrap();
    assert_eq!(h.counts, vec![5]);
    assert_eq!(h.edges, vec![1.0, 5.0]);
}

#[test]
fn histogram_bins_zero_is_invalid_bin_count() {
    let x = vec_tensor(vec![1.0, 2.0, 3.0]);
    assert!(matches!(
        histogram(&x, 0).unwrap_err(),
        MattenStatsError::InvalidBinCount
    ));
}

#[test]
fn histogram_constant_input_is_zero_variance_not_a_widened_range() {
    // NumPy would widen this to (5.0 - 0.5, 5.0 + 0.5); matten-stats errors
    // instead (RFC-090 §4.4).
    let x = vec_tensor(vec![5.0, 5.0, 5.0]);
    assert!(matches!(
        histogram(&x, 4).unwrap_err(),
        MattenStatsError::ZeroVariance
    ));
}

#[test]
fn histogram_non_finite_input_is_an_error() {
    let nan = vec_tensor(vec![1.0, f64::NAN, 3.0]);
    assert!(matches!(
        histogram(&nan, 2).unwrap_err(),
        MattenStatsError::NonFiniteValue
    ));

    let inf = vec_tensor(vec![1.0, f64::INFINITY, 3.0]);
    assert!(matches!(
        histogram(&inf, 2).unwrap_err(),
        MattenStatsError::NonFiniteValue
    ));
}

#[test]
fn histogram_overflowing_range_is_rejected_not_silently_corrupted() {
    // Every element is finite, but max(x) - min(x) overflows to infinity.
    // Before the C1 fix this returned Ok with edges [NaN, inf, inf, inf,
    // 1.7e308] and counts summing correctly to x.len() -- a corrupt result
    // the existing sum-invariant test could not detect.
    let x = vec_tensor(vec![-1.7e308, 0.0, 1.7e308]);
    let err = histogram(&x, 4).unwrap_err();
    assert!(matches!(err, MattenStatsError::NonFiniteValue));
}

#[test]
fn histogram_huge_bins_is_rejected_without_allocating() {
    // If this attempted the allocation, an over-budget `bins` would hang or
    // exhaust memory rather than fail fast; asserting the error variant is
    // itself the proof no allocation was attempted.
    let x = vec_tensor(vec![1.0, 2.0, 3.0]);
    let limit = MattenLimits::default().max_elements;
    let err = histogram(&x, limit + 1).unwrap_err();
    assert!(matches!(
        err,
        MattenStatsError::AllocationLimit {
            requested_bins,
            limit: reported_limit,
        } if requested_bins == limit + 1 && reported_limit == limit
    ));
}

// ── shared ─────────────────────────────────────────────────────────────────

// NOTE: RFC-078 handoff §6 (and RFC-083 handoff §6 for the three new
// functions) asks for "empty tensor -> Empty for all", but
// `matten::Tensor::try_new`/`new` unconditionally reject every zero-sized
// dimension (see compatibility.md's `is_empty()` entry) -- a genuinely empty
// Tensor cannot be constructed at all, so that exact scenario is untestable
// through the public API. `MattenStatsError::Empty`'s "empty tensor" wording
// is retained defensively (in case that invariant is ever relaxed upstream),
// but the only path to it that a caller can actually reach today is the
// `n < 2` case for covariance/correlation/skewness/kurtosis, covered above by
// `covariance_fewer_than_two_elements_is_an_error` and
// `skewness_and_kurtosis_single_element_is_empty`. `quantile`'s `x.len() ==
// 0` branch is unreachable dead code under matten's current shape model, and
// so is `covariance_population`'s (its minimum is 1, not 0) and
// `histogram`'s (RFC-090's minimum is also 1, not 0).

#[cfg(feature = "dynamic")]
#[test]
fn dynamic_tensor_is_rejected_for_all_seven() {
    use matten::Element;

    let dynamic = Tensor::from_elements(vec![Element::Float(1.0), Element::Float(2.0)], &[2]);
    assert!(matches!(
        covariance(&dynamic, &dynamic).unwrap_err(),
        MattenStatsError::DynamicTensor
    ));
    assert!(matches!(
        covariance_population(&dynamic, &dynamic).unwrap_err(),
        MattenStatsError::DynamicTensor
    ));
    assert!(matches!(
        correlation(&dynamic, &dynamic).unwrap_err(),
        MattenStatsError::DynamicTensor
    ));
    assert!(matches!(
        skewness(&dynamic).unwrap_err(),
        MattenStatsError::DynamicTensor
    ));
    assert!(matches!(
        kurtosis(&dynamic).unwrap_err(),
        MattenStatsError::DynamicTensor
    ));
    assert!(matches!(
        quantile(&dynamic, 0.5).unwrap_err(),
        MattenStatsError::DynamicTensor
    ));
    assert!(matches!(
        histogram(&dynamic, 4).unwrap_err(),
        MattenStatsError::DynamicTensor
    ));
}
