//! Loading numeric CSV data into a tensor.
//!
//! Run: cargo run --example 11_csv_numeric_loading
//!
//! Phase 1 accepts rectangular numeric-only CSV. Shape is inferred as
//! [rows, cols]. Each field must be a valid f64.

use matten::Tensor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ── from_csv: inline string ──────────────────────────────────────────
    let t = Tensor::from_csv("1.0,2.0,3.0\n4.0,5.0,6.0\n")?;
    println!(
        "inline CSV:   shape={:?}  data={:?}",
        t.shape(),
        t.as_slice()
    );

    // ── load_csv: from file ──────────────────────────────────────────────
    let t2 = Tensor::load_csv("examples/data/numeric_2x3.csv")?;
    println!(
        "from file:    shape={:?}  data={:?}",
        t2.shape(),
        t2.as_slice()
    );

    let t3 = Tensor::load_csv("examples/data/numeric_3x3.csv")?;
    println!("3×3 from file: shape={:?}", t3.shape());

    Ok(())
}
