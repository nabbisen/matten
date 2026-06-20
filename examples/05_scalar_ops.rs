//! Scalar arithmetic: tensor on left and scalar on left (all eight forms).
//!
//! Run: cargo run --example 05_scalar_ops

use matten::Tensor;

fn main() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);

    // tensor op scalar
    println!("t + 10  = {:?}", &t + 10.0);
    println!("t - 1   = {:?}", &t - 1.0);
    println!("t * 2   = {:?}", &t * 2.0);
    println!("t / 2   = {:?}", &t / 2.0);

    // scalar op tensor
    println!("10 + t  = {:?}", 10.0_f64 + &t);
    println!("10 - t  = {:?}", 10.0_f64 - &t);
    println!("2 * t   = {:?}", 2.0_f64 * &t);
    println!("8 / t   = {:?}", 8.0_f64 / &t);
}
