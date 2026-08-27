//! Matrix × vector multiplication using `Tensor::matmul`.
//!
//! Run: cargo run --example 21_matrix_vector_product
//!
//! Shape rule: [m, n] × [n] -> [m]

use matten::Tensor;

fn main() {
    // [[1,2,3],[4,5,6]] × [1,0,1] = [4, 10]
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let v = Tensor::from_vec(vec![1.0, 0.0, 1.0]);

    let r = m.matmul(&v);
    println!("m    = {m:?}");
    println!("v    = {v:?}");
    println!("m·v  = {r:?}"); // [4.0, 10.0]

    assert_eq!(r.shape(), &[2]);
    assert_eq!(r.as_slice(), &[4.0, 10.0]);

    // Vector × matrix: [n] × [n, p] -> [p]
    let w = Tensor::from_vec(vec![1.0, 2.0]);
    let r2 = w.matmul(&m);
    println!("w·m  = {r2:?}");

    assert_eq!(r2.shape(), &[3]);
    assert_eq!(r2.as_slice(), &[9.0, 12.0, 15.0]);

    println!("Shapes and values verified: OK");
}
