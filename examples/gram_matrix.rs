//! Gram matrix: G = X · Xᵀ, used in kernel methods and feature covariance.
//!
//! Run: cargo run --example gram_matrix

use matten::Tensor;

fn main() {
    // 4 data points × 3 features
    let x = Tensor::new(
        vec![1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        &[4, 3],
    );

    // G = X · Xᵀ: shape [4,4]
    let g = x.matmul(&x.transpose());
    println!("Gram matrix shape = {:?}", g.shape()); // [4,4]

    // Diagonal: ||x_i||²
    for i in 0..4 {
        let diag = g.get(&[i, i]).unwrap();
        println!("G[{i},{i}] = {diag:.1}");
    }

    // Symmetric check
    for i in 0..4 {
        for j in 0..4 {
            assert!((g.get(&[i, j]).unwrap() - g.get(&[j, i]).unwrap()).abs() < 1e-10);
        }
    }
    println!("Gram matrix is symmetric: OK");
}
