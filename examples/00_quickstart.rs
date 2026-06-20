//! A first look at `matten`: create, add, reshape, and inspect a tensor.
//!
//! Run: cargo run --example 00_quickstart
//!
//! `matten` is the family car of Rust tensor libraries — easy to start,
//! no type puzzles, and honest about performance. For heavy numerical work,
//! migrate your flat data to `ndarray`, `nalgebra`, or `candle`.

use matten::Tensor;

fn main() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let b = Tensor::ones(&[2, 2]);

    let c = &a + &b;
    println!("a      = {a:?}");
    println!("b      = {b:?}");
    println!("a + b  = {c:?}");

    let flat = c.flatten();
    println!("flat   = {flat:?}");

    assert_eq!(flat.shape(), &[4]);
    assert_eq!(flat.as_slice(), &[2.0, 3.0, 4.0, 5.0]);
    println!("All assertions passed.");
}
