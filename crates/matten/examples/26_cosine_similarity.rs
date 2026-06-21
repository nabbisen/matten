//! Cosine similarity between two vectors using `Tensor::dot`.
//!
//! Run: cargo run --example 26_cosine_similarity
//!
//! cosine_sim(a, b) = (a · b) / (‖a‖ · ‖b‖)

use matten::Tensor;

fn l2_norm(v: &Tensor) -> f64 {
    v.as_slice().iter().map(|x| x * x).sum::<f64>().sqrt()
}

fn cosine_similarity(a: &Tensor, b: &Tensor) -> f64 {
    // a · b is the scalar dot product
    let dot = a.dot(b).as_slice()[0];
    dot / (l2_norm(a) * l2_norm(b))
}

fn main() {
    let a = Tensor::from_vec(vec![1.0, 0.0, 0.0]);
    let b = Tensor::from_vec(vec![0.0, 1.0, 0.0]);
    let c = Tensor::from_vec(vec![1.0, 0.0, 0.0]);

    println!("cos_sim(a, b) = {:.4}", cosine_similarity(&a, &b)); // 0.0 orthogonal
    println!("cos_sim(a, c) = {:.4}", cosine_similarity(&a, &c)); // 1.0 identical

    let p = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    let q = Tensor::from_vec(vec![4.0, 5.0, 6.0]);
    println!("cos_sim(p, q) = {:.6}", cosine_similarity(&p, &q)); // ≈ 0.974632

    assert!((cosine_similarity(&a, &b)).abs() < 1e-10);
    assert!((cosine_similarity(&a, &c) - 1.0).abs() < 1e-10);
    println!("Assertions passed: OK");
}
