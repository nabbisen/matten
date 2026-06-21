//! Tests validating the RFC-028 §8 design specifications.

use matten::Tensor;
use matten_mlprep::{
    MattenMlprepError, add_bias_column, minmax_scale_columns, standardize_columns, train_test_split,
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
