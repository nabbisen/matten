//! Element-wise arithmetic: `+`, `-`, `*` (NOT matmul), `/`, and `-` (unary).
//!
//! Run: cargo run --example 04_elementwise_ops

use matten::Tensor;

fn main() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let b = Tensor::full(&[2, 2], 2.0);

    println!("a        = {a:?}");
    println!("b        = {b:?}");
    println!("a + b    = {:?}", &a + &b);
    println!("a - b    = {:?}", &a - &b);
    println!("a * b    = {:?}", &a * &b); // element-wise, NOT matrix product
    println!("a / b    = {:?}", &a / &b);
    println!("-a       = {:?}", -&a);
}
