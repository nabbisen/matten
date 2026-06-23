//! # Companion example: CSV string -> clean -> numeric `Tensor` (matten-data)
//!
//! Run: cargo run -p matten-data --example csv_to_tensor
//!
//! ## What this shows
//! The canonical `matten-data` workflow: parse a small messy CSV, inspect it,
//! select numeric columns, fill a missing cell, and convert explicitly to a
//! `[rows, columns]` f64 `Tensor`.
//!
//! ## Teaching points
//! - `matten-data` is **Experimental** and intentionally small;
//! - it is **not** a dataframe: no group-by, join, merge, pivot, or query;
//! - missing values and numeric conversion are **explicit** (`fill_missing` then
//!   `try_numeric`), never silent;
//! - the output is a plain numeric `Tensor` of shape `[rows, columns]`.

use matten_data::Table;

fn main() -> Result<(), matten_data::MattenDataError> {
    // A small, messy table: a text column and one missing numeric cell.
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

    // 3 rows x 3 columns; the missing south/cost became 0.0.
    assert_eq!(tensor.shape(), &[3, 3]);
    assert_eq!(
        tensor.as_slice(),
        &[100.0, 40.0, 5.0, 150.0, 0.0, 7.0, 120.0, 55.0, 6.0]
    );
    println!("csv_to_tensor: OK");
    Ok(())
}
