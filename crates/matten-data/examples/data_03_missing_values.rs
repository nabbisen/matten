//! # data_03 — Missing values are explicit, never silent
//!
//! Run: `cargo run -p matten-data --example data_03_missing_values`
//!
//! ## What this shows
//! `matten-data` never turns a missing cell into `0` behind your back. A missing
//! value that reaches numeric conversion is an error; you must fill it first.
//!
//! ## Teaching points
//! - converting a table with a missing cell fails with a precise
//!   `MissingValue { column, row }` error (row is the 1-based CSV line);
//! - `fill_missing` replaces only the missing cells, with a value you choose;
//! - after an explicit fill, conversion succeeds.

use matten_data::{MattenDataError, Table};

fn main() -> Result<(), MattenDataError> {
    let csv = "\
region,sales,cost
north,100,40
south,150,
east,120,55";

    let table = Table::from_csv_str(csv)?;
    let numeric_cols = table.select_columns(["sales", "cost"])?;

    // Converting with a missing cell still present is rejected — no silent zero.
    match numeric_cols.try_numeric() {
        Err(MattenDataError::MissingValue { column, row }) => {
            println!("missing value blocked conversion: column={column}, csv_line={row}");
            assert_eq!(column, "cost");
            assert_eq!(row, 3); // header is line 1, so the south row is line 3
        }
        other => panic!("expected MissingValue, got {other:?}"),
    }

    // Decide explicitly what a missing cost means here, then convert.
    let tensor = numeric_cols.fill_missing(0.0)?.try_numeric()?.to_tensor()?;

    println!("filled shape: {:?}", tensor.shape());
    println!("filled data : {:?}", tensor.as_slice());

    assert_eq!(tensor.shape(), &[3, 2]);
    // Only the missing south/cost was filled; the other cells are untouched.
    assert_eq!(tensor.as_slice(), &[100.0, 40.0, 150.0, 0.0, 120.0, 55.0]);

    println!("data_03_missing_values: OK");
    Ok(())
}
