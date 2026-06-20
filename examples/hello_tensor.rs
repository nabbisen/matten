//! Smoke example for the M0 skeleton.
//!
//! Run: `cargo run --example hello_tensor`

use matten::Tensor;

fn main() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    println!("{t:?}");
    println!(
        "shape = {:?}, len = {}, ndim = {}",
        t.shape(),
        t.len(),
        t.ndim()
    );
}
