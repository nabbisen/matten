//! Per-column statistics using axis reductions: mean, min, max, std dev.
//!
//! Run: cargo run --example 28_column_statistics
//!
//! A common PoC pattern: treating each column of a [rows, cols] tensor as
//! a feature, computing basic stats across all rows.

use matten::Tensor;

/// Manually compute column standard deviation from column means.
/// std_dev_i = sqrt( mean( (col_i - mean_i)^2 ) )
fn column_std(data: &Tensor, col_means: &Tensor) -> Tensor {
    // Broadcast means over all rows, compute squared deviations
    let deviations = data - col_means; // [rows, cols] - [cols] → [rows, cols]
    let sq = &deviations * &deviations;
    sq.mean_axis(0) // → [cols] mean of squared deviations
        .as_slice()
        .iter()
        .map(|v| v.sqrt())
        .collect::<Vec<f64>>()
        .into()
}

fn main() {
    // 4 rows, 3 columns (features)
    let data = Tensor::new(
        vec![
            1.0, 10.0, 100.0, 2.0, 20.0, 200.0, 3.0, 30.0, 300.0, 4.0, 40.0, 400.0,
        ],
        &[4, 3],
    );

    let means = data.mean_axis(0);
    let mins = data.min_axis(0);
    let maxs = data.max_axis(0);
    let stds = column_std(&data, &means);

    println!("columns : 0      1      2");
    println!("mean    : {:?}", means.as_slice());
    println!("min     : {:?}", mins.as_slice());
    println!("max     : {:?}", maxs.as_slice());
    println!("std dev : {:?}", stds.as_slice());

    assert_eq!(means.as_slice(), &[2.5, 25.0, 250.0]);
    assert_eq!(mins.as_slice(), &[1.0, 10.0, 100.0]);
    assert_eq!(maxs.as_slice(), &[4.0, 40.0, 400.0]);

    // Std dev: sqrt of mean squared deviation from mean
    let expected_std0 = {
        // Column 0 values: 1,2,3,4. Mean=2.5. Deviations: -1.5,-0.5,0.5,1.5
        let sq_devs = [
            1.5f64.powi(2),
            0.5f64.powi(2),
            0.5f64.powi(2),
            1.5f64.powi(2),
        ];
        (sq_devs.iter().sum::<f64>() / 4.0).sqrt()
    };
    assert!((stds.as_slice()[0] - expected_std0).abs() < 1e-10);

    println!("done.");
}
