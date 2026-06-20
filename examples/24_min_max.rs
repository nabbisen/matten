//! `min` and `max` reductions with explicit NaN policy.
//!
//! Run: cargo run --example 24_min_max
//!
//! `matten` returns NaN from min/max if any element is NaN.
//! This is deliberate: it is more predictable than silently ignoring NaN.

use matten::Tensor;

fn main() {
    let v = Tensor::from_vec(vec![3.0, 1.0, 4.0, 1.0, 5.0, 9.0, 2.0]);
    println!("min = {}", v.min()); // 1.0
    println!("max = {}", v.max()); // 9.0

    // NaN policy: any NaN -> result is NaN
    let with_nan = Tensor::from_vec(vec![1.0, f64::NAN, 3.0]);
    println!("min with NaN = {}", with_nan.min()); // NaN
    println!("max with NaN = {}", with_nan.max()); // NaN
    assert!(with_nan.min().is_nan());
    assert!(with_nan.max().is_nan());

    // Inf is handled normally
    let with_inf = Tensor::from_vec(vec![1.0, f64::INFINITY, -1.0]);
    println!("min with +inf = {}", with_inf.min()); // -1.0
    println!("max with +inf = {}", with_inf.max()); // inf

    println!("NaN policy verified: OK");
}
