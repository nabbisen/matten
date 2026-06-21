//! Matrix × matrix multiplication using `Tensor::matmul`.
//!
//! Run: cargo run --example 22_matrix_multiplication
//!
//! Shape rule: [m, n] × [n, p] -> [m, p]
//! `*` is element-wise — never a matrix product. Use `matmul` explicitly.

use matten::Tensor;

fn main() {
    // 2×2 example
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let b = Tensor::new(vec![5.0, 6.0, 7.0, 8.0], &[2, 2]);
    let c = a.matmul(&b);
    println!("A     = {a:?}");
    println!("B     = {b:?}");
    println!("A × B = {c:?}"); // [[19,22],[43,50]]
    assert_eq!(c.as_slice(), &[19.0, 22.0, 43.0, 50.0]);

    // Non-square: [2,3] × [3,4] -> [2,4]
    let x = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let y = Tensor::new((1..=12).map(|v| v as f64).collect(), &[3, 4]);
    let z = x.matmul(&y);
    println!("X × Y shape = {:?}", z.shape()); // [2, 4]

    println!("Matrix multiplication: OK");
}
