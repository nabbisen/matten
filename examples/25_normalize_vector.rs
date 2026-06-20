//! L2 normalisation of a vector using simple tensor operations.
//!
//! Run: cargo run --example 25_normalize_vector
//!
//! This example intentionally computes the norm via as_slice() so you can
//! see how ordinary Rust and Tensor arithmetic compose for small PoC math.

use matten::Tensor;

/// Returns the L2 norm (Euclidean length) of a 1-D tensor.
fn l2_norm(v: &Tensor) -> f64 {
    v.as_slice().iter().map(|x| x * x).sum::<f64>().sqrt()
}

/// Returns the L2-normalised version of `v` (unit vector).
fn normalize(v: &Tensor) -> Tensor {
    let norm = l2_norm(v);
    v / norm
}

fn main() {
    let v = Tensor::from_vec(vec![3.0, 4.0]);
    println!("v        = {:?}", v.as_slice());
    println!("‖v‖      = {}", l2_norm(&v)); // 5.0

    let u = normalize(&v);
    println!("norm(v)  = {:?}", u.as_slice()); // [0.6, 0.8]

    let norm_u = l2_norm(&u);
    println!("‖norm(v)‖ = {norm_u:.6}"); // ≈ 1.0
    assert!((norm_u - 1.0).abs() < 1e-10, "unit vector check failed");
    println!("Unit vector check: OK");
}
