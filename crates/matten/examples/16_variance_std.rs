//! Statistics core-lite: population variance and standard deviation (RFC-040).
//!
//! Run: cargo run --example 16_variance_std
//!
//! `var`/`std` are POPULATION statistics (ddof = 0): var = sum((x - mean)^2) / n,
//! std = sqrt(var). `var_axis`/`std_axis` do the same along one axis, dropping it
//! from the shape. Quantile, percentile, histogram, covariance, and correlation are
//! out of core scope (a possible future matten-stats companion).

use matten::Tensor;

fn main() {
    // Population variance and std over all elements.
    let v = Tensor::from_vec(vec![1.0, 2.0, 3.0, 4.0]);
    println!("data          = {:?}", v.as_slice());
    println!("mean          = {}", v.mean());
    println!("var (pop)     = {}", v.var()); // 1.25
    println!("std (pop)     = {}", v.std()); // sqrt(1.25)
    assert_eq!(v.var(), 1.25);

    // Per-axis statistics on a 2x3 matrix; the reduced axis is dropped.
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    println!("matrix        = {:?} shape {:?}", m.as_slice(), m.shape());

    let var0 = m.var_axis(0); // per column -> [3]
    println!(
        "var_axis(0)   = {:?} shape {:?}",
        var0.as_slice(),
        var0.shape()
    );
    assert_eq!(var0.shape(), &[3]);

    let std1 = m.std_axis(1); // per row -> [2]
    println!(
        "std_axis(1)   = {:?} shape {:?}",
        std1.as_slice(),
        std1.shape()
    );
    assert_eq!(std1.shape(), &[2]);

    // try_* forms return Result; e.g. an out-of-range axis is Err.
    assert!(m.try_var_axis(5).is_err());

    println!("Variance/std: OK");
}
