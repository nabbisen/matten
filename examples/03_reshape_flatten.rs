//! Reshape and flatten: same flat data, different shape.
//!
//! Run: cargo run --example 03_reshape_flatten

use matten::Tensor;

fn main() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    println!("original  {t:?}");

    let r = t.reshape(&[3, 2]);
    println!("reshaped  {r:?}");

    let flat = t.flatten();
    println!("flat      {flat:?}");

    // try_reshape for user-provided shapes
    let bad = t.try_reshape(&[4, 2]);
    println!("bad reshape: {}", bad.unwrap_err());

    // Flat data order is preserved through reshape
    assert_eq!(t.as_slice(), r.as_slice());
    println!("Flat data order preserved: OK");
}
