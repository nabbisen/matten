//! Cosine similarity between two vectors using element-wise ops.
//!
//! Run: cargo run --example 26_cosine_similarity
//!
//! cosine_sim(a, b) = (a · b) / (‖a‖ · ‖b‖)
//!
//! matten 0.5 has no built-in dot product; that arrives with RFC-010.
//! This example composes the existing API to implement it locally.

use matten::Tensor;

fn dot(a: &Tensor, b: &Tensor) -> f64 {
    a.as_slice()
        .iter()
        .zip(b.as_slice())
        .map(|(x, y)| x * y)
        .sum()
}

fn l2_norm(v: &Tensor) -> f64 {
    v.as_slice().iter().map(|x| x * x).sum::<f64>().sqrt()
}

fn cosine_similarity(a: &Tensor, b: &Tensor) -> f64 {
    dot(a, b) / (l2_norm(a) * l2_norm(b))
}

fn main() {
    let a = Tensor::from_vec(vec![1.0, 0.0, 0.0]);
    let b = Tensor::from_vec(vec![0.0, 1.0, 0.0]);
    let c = Tensor::from_vec(vec![1.0, 0.0, 0.0]);

    println!("cos_sim(a, b) = {:.4}", cosine_similarity(&a, &b)); // 0.0 (orthogonal)
    println!("cos_sim(a, c) = {:.4}", cosine_similarity(&a, &c)); // 1.0 (identical)

    let p = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    let q = Tensor::from_vec(vec![4.0, 5.0, 6.0]);
    let sim = cosine_similarity(&p, &q);
    println!("cos_sim(p, q) = {sim:.6}"); // ≈ 0.974632

    assert!((cosine_similarity(&a, &b)).abs() < 1e-10);
    assert!((cosine_similarity(&a, &c) - 1.0).abs() < 1e-10);
    println!("All assertions passed.");
}
