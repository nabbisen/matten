//! # data_00 — Quickstart: CSV string -> numeric `Tensor` (matten-data)
//!
//! Run: `cargo run -p matten-data --example data_00_quickstart`
//!
//! ## What this shows
//! The whole `matten-data` happy path in one place:
//!
//! ```text
//! CSV string -> Table -> select columns -> fill missing -> try_numeric -> Tensor
//! ```
//!
//! ## Teaching points
//! - every step is explicit: you choose the columns, the fill value, and the
//!   moment of numeric conversion;
//! - the output is a plain `[rows, columns]` f64 [`matten::Tensor`];
//! - `matten-data` is a small conversion helper, **not** a dataframe library.

use matten_data::Table;

fn main() -> Result<(), matten_data::MattenDataError> {
    // A small table with one text column and one missing numeric cell.
    let csv = "\
region,sales,cost
north,100,40
south,150,
east,120,55";

    let tensor = Table::from_csv_str(csv)?
        .select_columns(["sales", "cost"])? // keep only the numeric columns
        .fill_missing(0.0)? // the missing south/cost becomes 0.0, explicitly
        .try_numeric()? // strict, explicit conversion to f64
        .to_tensor()?; // a normal [rows, columns] Tensor

    println!("shape: {:?}", tensor.shape());
    println!("data : {:?}", tensor.as_slice());

    assert_eq!(tensor.shape(), &[3, 2]);
    assert_eq!(tensor.as_slice(), &[100.0, 40.0, 150.0, 0.0, 120.0, 55.0]);

    println!("data_00_quickstart: OK");
    Ok(())
}
