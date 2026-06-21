//! Vector dot product using `Tensor::dot`.
//!
//! Run: cargo run --example 20_dot_product
//!
//! `dot` returns a scalar-shaped tensor `[]` for two vectors of equal length.
//! `*` is always element-wise — never a dot product.

use matten::Tensor;

fn main() {
    let a = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    let b = Tensor::from_vec(vec![4.0, 5.0, 6.0]);

    let d = a.dot(&b); // 1*4 + 2*5 + 3*6 = 32
    assert!(d.is_scalar());
    println!("a · b = {}", d.as_slice()[0]); // 32.0

    // Orthogonal vectors have zero dot product
    let x = Tensor::from_vec(vec![1.0, 0.0]);
    let y = Tensor::from_vec(vec![0.0, 1.0]);
    assert_eq!(x.dot(&y).as_slice(), &[0.0]);
    println!("orthogonal dot = 0: OK");
}
