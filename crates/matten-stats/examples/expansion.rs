//! # Companion example: population covariance, skewness, kurtosis (RFC-083)
//!
//! Run: cargo run -p matten-stats --example stats_expansion
//!
//! ## What this shows
//! - `covariance_population` is the `ddof = 0` counterpart to `covariance`'s
//!   `ddof = 1`, and (unlike `covariance`) accepts a single-element input;
//! - `skewness`/`kurtosis` are the **uncorrected** SciPy-default estimators,
//!   not pandas' bias-corrected ones;
//! - `kurtosis` reports **excess** kurtosis: a normal-ish symmetric input is
//!   near `0.0`, not `3.0`.

use matten::Tensor;
use matten_stats::{covariance, covariance_population, kurtosis, skewness};

fn main() {
    let x = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);
    let y = Tensor::new(vec![2.0, 4.0, 6.0], &[3]);

    // covariance_population vs covariance: cov_pop * n == cov_sample * (n - 1).
    let cov_pop = covariance_population(&x, &y).expect("valid inputs");
    let cov_sample = covariance(&x, &y).expect("valid inputs");
    println!("covariance_population(x, y) = {cov_pop}");
    println!("covariance(x, y)            = {cov_sample}");
    assert!((cov_pop * 3.0 - cov_sample * 2.0).abs() < 1e-9);

    // Unlike `covariance`, a single element is well-defined here.
    let one = Tensor::new(vec![5.0], &[1]);
    assert_eq!(covariance_population(&one, &one).unwrap(), 0.0);
    println!("covariance_population on a single element: 0.0 (well-defined)");

    // Symmetric input: skewness is exactly 0.0.
    let symmetric = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], &[5]);
    let skew = skewness(&symmetric).expect("valid input");
    println!("skewness([1..5]) = {skew}");
    assert_eq!(skew, 0.0);

    // Excess kurtosis: m2 = 2, m4 = 6.8, raw ratio 1.7, excess = 1.7 - 3 = -1.3.
    let kurt = kurtosis(&symmetric).expect("valid input");
    println!("kurtosis([1..5]) = {kurt} (excess; normal distribution scores 0.0, not 3.0)");
    assert_eq!(kurt, -1.3);

    println!("stats_expansion: OK");
}
