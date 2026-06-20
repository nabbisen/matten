//! The canonical slicing API: builder with `.all()`, `.index()`, `.range()`.
//!
//! Run: cargo run --example 08_slicing_builder
//!
//! The builder is the canonical form; `slice_str` is a convenience wrapper.

use matten::Tensor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let t = Tensor::new((1..=12).map(|x| x as f64).collect(), &[3, 4]);
    println!("tensor   {t:?}");

    // First row (index 0, axis removed from output shape)
    let row0 = t.slice().index(0).all().build()?;
    println!("row 0    {row0:?}"); // shape [4]

    // First two rows, all columns
    let top2 = t.slice().range(0..2).all().build()?;
    println!("top 2    {top2:?}"); // shape [2,4]

    // All rows, columns 1..3
    let cols = t.slice().all().range(1..3).build()?;
    println!("cols 1:3 {cols:?}"); // shape [3,2]

    // Single element -> scalar
    let elem = t.slice().index(1).index(2).build()?;
    assert!(elem.is_scalar());
    println!("t[1,2]   {elem:?}");

    Ok(())
}
