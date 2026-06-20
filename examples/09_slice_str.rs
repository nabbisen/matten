//! Convenience string slicing — NumPy-like, always returns Result.
//!
//! Run: cargo run --example 09_slice_str
//!
//! `slice_str` is a secondary convenience API. The builder (example 08) is
//! canonical. Malformed specs always return Err, never panic.

use matten::Tensor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let t = Tensor::new((1..=12).map(|x| x as f64).collect(), &[3, 4]);

    println!("\"0, :\"    = {:?}", t.slice_str("0, :")?);
    println!("\"0:2, :\" = {:?}", t.slice_str("0:2, :")?);
    println!("\":, 1:3\" = {:?}", t.slice_str(":, 1:3")?);

    // Whitespace is ignored
    let a = t.slice_str("0:2, :")?;
    let b = t.slice_str(" 0:2 , : ")?;
    assert_eq!(a, b);
    println!("whitespace-insensitive: OK");

    // builder and slice_str agree
    let from_str = t.slice_str("0:2, :")?;
    let from_builder = t.slice().range(0..2).all().build()?;
    assert_eq!(from_str, from_builder);
    println!("builder == slice_str: OK");

    // Malformed specs never panic
    println!("bad spec: {:?}", t.slice_str("0::")); // Err

    Ok(())
}
