//! # Companion example: sample covariance (matten-stats, RFC-078)
//!
//! Run: cargo run -p matten-stats --example stats_covariance
//!
//! ## What this shows
//! `covariance(x, y)` computes the **sample** covariance (`ddof = 1`),
//! deliberately diverging from core `matten`'s population `var`/`std`.
//!
//! ## Teaching points
//! - divides by `n - 1`, not `n`;
//! - `cov(x, x)` equals the sample variance of `x`;
//! - `cov(x, y) == cov(y, x)` (symmetric).

use matten::Tensor;
use matten_stats::covariance;

fn main() {
    let x = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[4]);
    let y = Tensor::new(vec![2.0, 4.0, 6.0, 8.0], &[4]); // y = 2x

    let cov_xy = covariance(&x, &y).expect("valid inputs");
    println!("cov(x, y) = {cov_xy}");
    assert!((cov_xy - 10.0 / 3.0).abs() < 1e-9);

    // Symmetric.
    let cov_yx = covariance(&y, &x).expect("valid inputs");
    assert!((cov_xy - cov_yx).abs() < 1e-12);
    println!("cov(x, y) == cov(y, x): OK");

    // cov(x, x) is the sample variance of x.
    let var_x = covariance(&x, &x).expect("valid inputs");
    println!("cov(x, x) (sample variance) = {var_x}");
    assert!((var_x - 5.0 / 3.0).abs() < 1e-9);

    println!("covariance: OK");
}
