//! # data_02 — Selecting columns by name
//!
//! Run: `cargo run -p matten-data --example data_02_select_columns`
//!
//! ## What this shows
//! Choosing a subset of columns by name, in the order you ask for them, and what
//! happens when you ask for a column that does not exist.
//!
//! ## Teaching points
//! - selection is **by name**, and the output column order matches your request
//!   (not the original CSV order);
//! - selecting an unknown column is a clean, structured error, never a panic;
//! - selection does not convert anything — it returns another `Table`.

use matten_data::{MattenDataError, Table};

fn main() -> Result<(), MattenDataError> {
    let csv = "\
region,sales,cost,quantity
north,100,40,5
south,150,45,7";

    let table = Table::from_csv_str(csv)?;

    // Ask for a subset in a deliberately different order than the CSV.
    let reordered = table.select_columns(["quantity", "sales"])?;
    println!("selected names: {:?}", reordered.column_names());

    // The output order is exactly what was requested.
    assert_eq!(
        reordered.column_names(),
        &["quantity".to_string(), "sales".to_string()]
    );
    assert_eq!(reordered.column_count(), 2);
    assert_eq!(reordered.row_count(), 2);

    // Asking for a column that does not exist is a structured error.
    match table.select_columns(["sales", "profit"]) {
        Err(MattenDataError::MissingColumn { name }) => {
            println!("missing column reported: {name}");
            assert_eq!(name, "profit");
        }
        other => panic!("expected MissingColumn, got {other:?}"),
    }

    println!("data_02_select_columns: OK");
    Ok(())
}
