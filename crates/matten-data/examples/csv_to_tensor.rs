//! Canonical `matten-data` workflow: CSV string -> clean -> numeric `Tensor`.
//!
//! Run with: `cargo run -p matten-data --example csv_to_tensor`

use matten_data::Table;

fn main() -> Result<(), matten_data::MattenDataError> {
    // A small, messy table: mixed types and a missing cell.
    let csv = "\
region,sales,cost,quantity
north,100,40,5
south,150,,7
east,120,55,6";

    let table = Table::from_csv_str(csv)?;

    // Inspect what we have before converting anything.
    println!("{}", table.schema_summary());

    // Select only the numeric columns we want, fill the one missing cost with 0,
    // convert explicitly, and produce a [rows, columns] f64 tensor.
    let tensor = table
        .select_columns(["sales", "cost", "quantity"])?
        .fill_missing(0.0)?
        .try_numeric()?
        .to_tensor()?;

    println!("tensor shape: {:?}", tensor.shape());
    println!("tensor data : {:?}", tensor.as_slice());

    assert_eq!(tensor.shape(), &[3, 3]);
    Ok(())
}
