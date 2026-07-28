//! # Companion example: Pearson correlation (matten-stats, RFC-078)
//!
//! Run: cargo run -p matten-stats --example stats_correlation
//!
//! ## What this shows
//! `correlation(x, y)` is bounded in `[-1, 1]` and is identical regardless of
//! the `ddof` convention — only `covariance` is affected by that choice.
//!
//! ## Teaching points
//! - a perfect linear relationship gives `+1.0` or `-1.0`;
//! - zero variance in either input is an explicit error, never `NaN`.

use matten::Tensor;
use matten_stats::correlation;

fn main() {
    let x = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[4]);

    let y_pos = Tensor::new(vec![2.0, 4.0, 6.0, 8.0], &[4]); // y = 2x
    let r_pos = correlation(&x, &y_pos).expect("valid inputs");
    println!("perfect positive: r = {r_pos}");
    assert!((r_pos - 1.0).abs() < 1e-9);

    let y_neg = Tensor::new(vec![8.0, 6.0, 4.0, 2.0], &[4]); // y = -2x + 10
    let r_neg = correlation(&x, &y_neg).expect("valid inputs");
    println!("perfect negative: r = {r_neg}");
    assert!((r_neg - (-1.0)).abs() < 1e-9);

    // Zero variance is an explicit error, not NaN.
    let constant = Tensor::new(vec![5.0, 5.0, 5.0, 5.0], &[4]);
    match correlation(&x, &constant) {
        Err(e) => println!("zero variance -> explicit error: {e}"),
        Ok(_) => panic!("expected ZeroVariance"),
    }

    println!("correlation: OK");
}
