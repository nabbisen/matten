//! Tests validating the RFC-028 §8 design specifications.

use matten::Tensor;
use matten_mlprep::{
    MattenMlprepError, add_bias_column, minmax_scale_columns, standardize_columns,
    train_test_split, train_test_split_seeded,
};

fn approx(a: &[f64], b: &[f64]) {
    assert_eq!(a.len(), b.len(), "length mismatch");
    for (x, y) in a.iter().zip(b) {
        assert!((x - y).abs() < 1e-9, "expected {b:?}, got {a:?}");
    }
}

// ── standardize_columns ───────────────────────────────────────────────────

#[test]
fn standardize_known_values() {
    // col0 [1,3] -> mean 2 std 1 -> [-1, 1]; col1 [10,20] -> mean 15 std 5 -> [-1, 1]
    let x = Tensor::new(vec![1.0, 10.0, 3.0, 20.0], &[2, 2]);
    let z = standardize_columns(&x).unwrap();
    approx(z.as_slice(), &[-1.0, -1.0, 1.0, 1.0]);
}

#[test]
fn standardize_produces_zero_mean_unit_std_per_column() {
    let x = Tensor::new(vec![1.0, 2.0, 4.0, 6.0, 9.0, 8.0], &[3, 2]);
    let z = standardize_columns(&x).unwrap();
    // Per column: mean ~ 0, population std ~ 1.
    for j in 0..2 {
        let col: Vec<f64> = (0..3).map(|i| z.as_slice()[i * 2 + j]).collect();
        let mean = col.iter().sum::<f64>() / 3.0;
        let var = col.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / 3.0;
        assert!(mean.abs() < 1e-9, "column {j} mean {mean}");
        assert!(
            (var.sqrt() - 1.0).abs() < 1e-9,
            "column {j} std {}",
            var.sqrt()
        );
    }
}

#[test]
fn standardize_constant_column_is_zero_variance_error() {
    // Column 1 is constant -> explicit error, not a silent zero column.
    let x = Tensor::new(vec![1.0, 5.0, 2.0, 5.0], &[2, 2]);
    let err = standardize_columns(&x).unwrap_err();
    assert!(matches!(err, MattenMlprepError::ZeroVariance { column: 1 }));
}

// ── minmax_scale_columns ──────────────────────────────────────────────────

#[test]
fn minmax_scales_to_unit_interval() {
    let x = Tensor::new(vec![0.0, 100.0, 5.0, 150.0, 10.0, 200.0], &[3, 2]);
    let s = minmax_scale_columns(&x).unwrap();
    // col0 [0,5,10] -> [0,0.5,1]; col1 [100,150,200] -> [0,0.5,1]
    approx(s.as_slice(), &[0.0, 0.0, 0.5, 0.5, 1.0, 1.0]);
}

#[test]
fn minmax_constant_column_is_zero_variance_error() {
    let x = Tensor::new(vec![7.0, 1.0, 7.0, 9.0], &[2, 2]); // col0 constant
    let err = minmax_scale_columns(&x).unwrap_err();
    assert!(matches!(err, MattenMlprepError::ZeroVariance { column: 0 }));
}

// ── add_bias_column ───────────────────────────────────────────────────────

#[test]
fn add_bias_prepends_ones_column() {
    let x = Tensor::new(vec![2.0, 3.0, 4.0, 5.0], &[2, 2]);
    let b = add_bias_column(&x).unwrap();
    assert_eq!(b.shape(), &[2, 3]);
    assert_eq!(b.as_slice(), &[1.0, 2.0, 3.0, 1.0, 4.0, 5.0]);
}

// ── train_test_split ──────────────────────────────────────────────────────

#[test]
fn split_ordered_partition_and_shapes() {
    let x = Tensor::new(vec![10.0, 20.0, 30.0, 40.0, 50.0], &[5, 1]);
    let (train, test) = train_test_split(&x, 0.6).unwrap(); // floor(3.0)=3
    assert_eq!(train.shape(), &[3, 1]);
    assert_eq!(test.shape(), &[2, 1]);
    assert_eq!(train.as_slice(), &[10.0, 20.0, 30.0]);
    assert_eq!(test.as_slice(), &[40.0, 50.0]);
}

#[test]
fn split_is_deterministic() {
    let x = Tensor::new((0..20).map(|v| v as f64).collect(), &[10, 2]);
    let a = train_test_split(&x, 0.8).unwrap();
    let b = train_test_split(&x, 0.8).unwrap();
    assert_eq!(a.0.as_slice(), b.0.as_slice());
    assert_eq!(a.1.as_slice(), b.1.as_slice());
    assert_eq!(a.0.shape(), &[8, 2]);
    assert_eq!(a.1.shape(), &[2, 2]);
}

#[test]
fn split_invalid_ratios_are_rejected() {
    let x = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[4, 1]);
    for r in [0.0, 1.0, -0.5, 1.5, f64::NAN, f64::INFINITY] {
        assert!(matches!(
            train_test_split(&x, r),
            Err(MattenMlprepError::InvalidRatio(_))
        ));
    }
}

#[test]
fn split_that_empties_train_is_rejected() {
    // 3 rows * 0.1 = 0.3 -> floor 0 train rows.
    let x = Tensor::new(vec![1.0, 2.0, 3.0], &[3, 1]);
    let err = train_test_split(&x, 0.1).unwrap_err();
    assert!(matches!(err, MattenMlprepError::EmptySplit { rows: 3, .. }));
}

// ── train_test_split_seeded (RFC-077) ─────────────────────────────────────

#[test]
fn seeded_split_is_reproducible() {
    let x = Tensor::new((0..16).map(|v| v as f64).collect(), &[8, 2]);
    let a = train_test_split_seeded(&x, 0.625, 42).unwrap();
    let b = train_test_split_seeded(&x, 0.625, 42).unwrap();
    assert_eq!(a.0.as_slice(), b.0.as_slice());
    assert_eq!(a.1.as_slice(), b.1.as_slice());
}

/// Locks the exact permutation SplitMix64 + Fisher-Yates produce for a fixed
/// input and seed. This is the test that makes the RFC-077 §6 reproducibility
/// contract real: it fails if the PRNG constants, the shuffle direction, or
/// the seed-to-state mapping ever change, rather than silently reshuffling
/// every user's data on the next release.
#[test]
fn seeded_split_locked_permutation() {
    let x = Tensor::new(vec![10.0, 20.0, 30.0, 40.0, 50.0], &[5, 1]);
    let (train, test) = train_test_split_seeded(&x, 0.6, 7).unwrap();
    assert_eq!(train.shape(), &[3, 1]);
    assert_eq!(test.shape(), &[2, 1]);
    assert_eq!(train.as_slice(), &[50.0, 20.0, 40.0]);
    assert_eq!(test.as_slice(), &[10.0, 30.0]);
}

#[test]
fn seeded_split_different_seeds_differ() {
    // 10 rows so a coincidental match across two seeds is implausible.
    let x = Tensor::new((0..10).map(|v| v as f64).collect(), &[10, 1]);
    let (a, _) = train_test_split_seeded(&x, 0.5, 1).unwrap();
    let (b, _) = train_test_split_seeded(&x, 0.5, 2).unwrap();
    assert_ne!(a.as_slice(), b.as_slice());
}

#[test]
fn seeded_split_size_parity_with_ordered_split() {
    let x = Tensor::new((0..20).map(|v| v as f64).collect(), &[10, 2]);
    let ordered = train_test_split(&x, 0.7).unwrap();
    let seeded = train_test_split_seeded(&x, 0.7, 123).unwrap();
    assert_eq!(ordered.0.shape(), seeded.0.shape());
    assert_eq!(ordered.1.shape(), seeded.1.shape());
}

#[test]
fn seeded_split_permutation_integrity() {
    // train ∪ test, as a sorted row multiset, must equal the input's rows
    // exactly: no row lost, duplicated, or corrupted across the boundary.
    let x = Tensor::new((0..16).map(|v| v as f64).collect(), &[8, 2]);
    let (train, test) = train_test_split_seeded(&x, 0.625, 42).unwrap();

    let mut rows: Vec<[u64; 2]> = train
        .as_slice()
        .chunks(2)
        .chain(test.as_slice().chunks(2))
        .map(|r| [r[0].to_bits(), r[1].to_bits()])
        .collect();
    let mut expected: Vec<[u64; 2]> = x
        .as_slice()
        .chunks(2)
        .map(|r| [r[0].to_bits(), r[1].to_bits()])
        .collect();
    rows.sort();
    expected.sort();
    assert_eq!(rows, expected);
}

#[test]
fn seeded_split_shuffles_rows_not_values() {
    // Every output row must match some *complete* input row — catches an
    // off-by-one or misaligned stride in the gather step, which would
    // otherwise produce right-shaped, silently-corrupt tensors.
    let x = Tensor::new((0..16).map(|v| v as f64).collect(), &[8, 2]);
    let input_rows: Vec<&[f64]> = x.as_slice().chunks(2).collect();
    let (train, test) = train_test_split_seeded(&x, 0.625, 42).unwrap();
    for row in train.as_slice().chunks(2).chain(test.as_slice().chunks(2)) {
        assert!(
            input_rows.contains(&row),
            "output row {row:?} does not match any complete input row"
        );
    }
}

#[test]
fn seeded_split_error_parity_with_ordered_split() {
    let v = Tensor::from_vec(vec![1.0, 2.0, 3.0]); // rank-1
    assert!(matches!(
        train_test_split_seeded(&v, 0.5, 0),
        Err(MattenMlprepError::ExpectedMatrix { .. })
    ));

    let x = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[4, 1]);
    for r in [0.0, 1.0, -0.5, 1.5, f64::NAN, f64::INFINITY] {
        assert!(matches!(
            train_test_split_seeded(&x, r, 0),
            Err(MattenMlprepError::InvalidRatio(_))
        ));
    }

    // 3 rows * 0.1 = 0.3 -> floor 0 train rows, same as the ordered split.
    let small = Tensor::new(vec![1.0, 2.0, 3.0], &[3, 1]);
    let err = train_test_split_seeded(&small, 0.1, 0).unwrap_err();
    assert!(matches!(err, MattenMlprepError::EmptySplit { rows: 3, .. }));
}

#[cfg(feature = "dynamic")]
#[test]
fn seeded_split_dynamic_input_is_rejected_not_panicked() {
    use matten::Element;
    let t = Tensor::from_elements(
        vec![
            Element::Float(1.0),
            Element::None,
            Element::Int(3),
            Element::Float(4.0),
        ],
        &[2, 2],
    );
    assert!(matches!(
        train_test_split_seeded(&t, 0.5, 0),
        Err(MattenMlprepError::DynamicTensor)
    ));
}

#[test]
fn ordered_split_still_passes_unchanged() {
    // Existing train_test_split behaviour is untouched by the seeded addition.
    let x = Tensor::new(vec![10.0, 20.0, 30.0, 40.0, 50.0], &[5, 1]);
    let (train, test) = train_test_split(&x, 0.6).unwrap();
    assert_eq!(train.as_slice(), &[10.0, 20.0, 30.0]);
    assert_eq!(test.as_slice(), &[40.0, 50.0]);
}

// ── shape / dynamic guards ────────────────────────────────────────────────

#[test]
fn non_matrix_input_is_rejected_everywhere() {
    let v = Tensor::from_vec(vec![1.0, 2.0, 3.0]); // rank-1
    assert!(matches!(
        standardize_columns(&v),
        Err(MattenMlprepError::ExpectedMatrix { .. })
    ));
    assert!(matches!(
        minmax_scale_columns(&v),
        Err(MattenMlprepError::ExpectedMatrix { .. })
    ));
    assert!(matches!(
        add_bias_column(&v),
        Err(MattenMlprepError::ExpectedMatrix { .. })
    ));
    assert!(matches!(
        train_test_split(&v, 0.5),
        Err(MattenMlprepError::ExpectedMatrix { .. })
    ));
}

#[test]
fn error_display_is_actionable() {
    let v = Tensor::from_vec(vec![1.0]);
    let msg = standardize_columns(&v).unwrap_err().to_string();
    assert!(msg.contains("rank-2"));
}

#[cfg(feature = "dynamic")]
#[test]
fn dynamic_input_is_rejected_not_panicked() {
    use matten::Element;
    let t = Tensor::from_elements(
        vec![
            Element::Float(1.0),
            Element::None,
            Element::Int(3),
            Element::Float(4.0),
        ],
        &[2, 2],
    );
    assert!(matches!(
        standardize_columns(&t),
        Err(MattenMlprepError::DynamicTensor)
    ));
    assert!(matches!(
        train_test_split(&t, 0.5),
        Err(MattenMlprepError::DynamicTensor)
    ));
}

// ── v0.19 hardening: documented NaN propagation + degenerate input (RFC-029 §3.3) ──

#[test]
fn standardize_nan_column_propagates_not_zero_variance() {
    // A column containing NaN has NaN mean/std (not 0), so it is NOT reported as
    // ZeroVariance; the documented behavior is NaN propagation to the output.
    let x = Tensor::new(vec![1.0, 10.0, f64::NAN, 20.0], &[2, 2]);
    let z = standardize_columns(&x).unwrap();
    assert!(z.as_slice()[0].is_nan()); // col 0 row 0
    assert!(z.as_slice()[2].is_nan()); // col 0 row 1
    // col 1 ([10, 20]) standardizes normally.
    approx(&[z.as_slice()[1], z.as_slice()[3]], &[-1.0, 1.0]);
}

#[test]
fn minmax_nan_column_propagates() {
    let x = Tensor::new(vec![f64::NAN, 0.0, 5.0, 10.0], &[2, 2]);
    let s = minmax_scale_columns(&x).unwrap();
    assert!(s.as_slice()[0].is_nan());
    assert!(s.as_slice()[2].is_nan());
    // col 1 ([0, 10]) -> [0, 1]
    assert_eq!(s.as_slice()[1], 0.0);
    assert_eq!(s.as_slice()[3], 1.0);
}

#[test]
fn single_row_matrix_is_zero_variance() {
    // One sample -> every column is constant -> ZeroVariance for the first column.
    let x = Tensor::new(vec![3.0, 7.0], &[1, 2]);
    assert!(matches!(
        standardize_columns(&x),
        Err(MattenMlprepError::ZeroVariance { column: 0 })
    ));
    assert!(matches!(
        minmax_scale_columns(&x),
        Err(MattenMlprepError::ZeroVariance { column: 0 })
    ));
}

// RFC-031: numeric tensors must never be flagged as dynamic, regardless of
// whether the companion `dynamic` feature is enabled.
#[test]
fn numeric_tensor_is_not_dynamic() {
    let x = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    assert!(!x.is_dynamic());
    // Guard must pass and preprocessing must succeed.
    assert!(standardize_columns(&x).is_ok());
}

// ── RFC-112: zero rows must error, never panic ──────────────────────────────
//
// RFC-110 changed core's mean_axis/min_axis/max_axis to error on a zero-length
// REDUCED axis instead of leaking NaN/inf. standardize_columns and
// minmax_scale_columns called the panicking forms directly, so a zero-row
// input (reachable from an ordinary header-only CSV via matten-data, no
// slicing required by the caller) started panicking. Fixtures here are built
// by slicing -- the reachable path -- matching every other empty-tensor test
// in this project.

fn empty_rows(cols: usize) -> Tensor {
    // shape [0, cols]
    Tensor::new(vec![0.0; cols], &[1, cols])
        .slice()
        .range(0..0)
        .all()
        .build()
        .unwrap()
}

#[test]
fn standardize_columns_on_zero_rows_errors_not_panics() {
    // T1. Before this fix: panicked at "mean is undefined for a reduced axis
    // of length 0 (axis 0)". Now: a clean Err via MattenMlprepError::Matten.
    let x = empty_rows(2);
    assert_eq!(x.shape(), &[0, 2]);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| standardize_columns(&x)));
    let err = result.expect("must not panic").expect_err("must be Err");
    assert!(matches!(err, MattenMlprepError::Matten(_)));
    assert!(err.to_string().contains("mean"));
}

#[test]
fn minmax_scale_columns_on_zero_rows_errors_not_panics() {
    // T2. Both call sites matter (R1): min_axis(0) is converted first in
    // source order, so a fixture alone cannot prove max_axis(0) was also
    // converted -- min's Err short-circuits before max is ever reached. This
    // is asserted at the source level too (see the review request); the
    // no-panic assertion here is still required and is what actually shipped
    // as the regression.
    let x = empty_rows(2);
    let result =
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| minmax_scale_columns(&x)));
    let err = result.expect("must not panic").expect_err("must be Err");
    assert!(matches!(err, MattenMlprepError::Matten(_)));
    // T3: the error carries core's message through MattenMlprepError::Matten.
    assert!(err.to_string().contains("minimum"));
}

#[test]
fn add_bias_column_on_zero_rows_is_unaffected() {
    // T5: add_bias_column performs no axis reduction, so RFC-112 did not touch
    // it. Made order-independent of RFC-111 (review correction): whether
    // Tensor::try_new accepts a zero-sized shape depends on RFC-111's
    // constructor change, which this RFC does not require and must not
    // depend on as a release blocker. Assert what RFC-112 actually
    // guarantees -- no panic, and if it errors, the error is not one of
    // RFC-110's axis-reduction messages -- and accept either constructor
    // policy for the shape/success question itself.
    let x = empty_rows(3);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| add_bias_column(&x)));
    match result.expect("must not panic") {
        Ok(b) => {
            assert_eq!(b.shape(), &[0, 4]);
            assert!(b.is_empty());
        }
        Err(MattenMlprepError::Matten(e)) => {
            let msg = e.to_string();
            assert!(
                !msg.contains("mean") && !msg.contains("minimum") && !msg.contains("maximum"),
                "add_bias_column performs no axis reduction; got an axis-reduction \
                 message instead of a constructor rejection: {msg}"
            );
        }
        Err(other) => panic!("expected Ok or Matten(..), got {other:?}"),
    }
}

#[test]
fn train_test_split_on_zero_rows_is_unaffected() {
    // T6: train_test_split rejects empty early and deliberately
    // (EmptySplit), unrelated to this fix.
    let x = empty_rows(2);
    assert!(matches!(
        train_test_split(&x, 0.5),
        Err(MattenMlprepError::EmptySplit { .. })
    ));
}
