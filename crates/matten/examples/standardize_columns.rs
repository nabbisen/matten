//! Standardise (z-score normalise) each column of a matrix.
//!
//! Run: cargo run --example standardize_columns
//!
//! Each column is transformed to zero mean, unit variance using only
//! existing matten ops: mean_axis, broadcasting, and element-wise arithmetic.

use matten::Tensor;

fn main() {
    // 4 samples × 3 features
    let data = Tensor::new(
        vec![2.0, 4.0, 1.0, 4.0, 6.0, 3.0, 6.0, 8.0, 5.0, 8.0, 10.0, 7.0],
        &[4, 3],
    );

    // Column means: shape [3]
    let means = data.mean_axis(0);
    // Centre: broadcast [3] across [4, 3]
    let centred = &data - &means;

    // Column std dev: sqrt(mean of squared deviations)
    let sq = &centred * &centred;
    let variance = sq.mean_axis(0);
    let std_dev_vals: Vec<f64> = variance.as_slice().iter().map(|v| v.sqrt()).collect();
    let std_dev = Tensor::new(std_dev_vals, &[3]);

    // Standardise
    let standardised = &centred / &std_dev;

    println!("means    = {:?}", means.as_slice());
    println!("std devs = {:?}", std_dev.as_slice());
    println!("result shape = {:?}", standardised.shape());

    // Each column should have mean ≈ 0 and std ≈ 1
    let col_means = standardised.mean_axis(0);
    for &m in col_means.as_slice() {
        assert!(m.abs() < 1e-10, "column mean not zero: {m}");
    }
    println!("Column means ≈ 0: OK");
}
