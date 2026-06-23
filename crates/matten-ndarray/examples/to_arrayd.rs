//! # Companion example: `Tensor` -> `ArrayD` (matten-ndarray)
//!
//! Run: cargo run -p matten-ndarray --example to_arrayd
//!
//! ## What this shows
//! Converting a numeric `matten::Tensor` into an `ndarray::ArrayD<f64>`.
//!
//! ## Teaching points
//! - the conversion **copies** data into the `ArrayD` (no zero-copy claim);
//! - shape is preserved (printed before and after);
//! - only numeric tensors convert — a dynamic/heterogeneous tensor must be made
//!   numeric first (hence the `expect` on a `Result`);
//! - core `matten` does not depend on `ndarray`; the bridge lives in this crate.

use matten::Tensor;
use matten_ndarray::to_arrayd;

fn main() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let arr = to_arrayd(&t).expect("numeric tensor converts");

    println!("matten shape: {:?}", t.shape());
    println!("ndarray shape: {:?}", arr.shape());
    println!("ndarray[[1, 2]] = {}", arr[[1, 2]]); // row 1, col 2 -> 6.0
    assert_eq!(t.shape(), arr.shape());
    assert_eq!(arr[[1, 2]], 6.0);
    println!("ok");
}
