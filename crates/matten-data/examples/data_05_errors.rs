//! # data_05 — Boundary errors are explicit and precise
//!
//! Run: `cargo run -p matten-data --example data_05_errors`
//!
//! ## What this shows
//! The common ways table input can be wrong, and the structured error each one
//! produces. Malformed input is always a returned `Result`, never a panic.
//!
//! ## Teaching points
//! - a duplicate header column is rejected up front;
//! - a row with the wrong number of cells is a `RaggedRow` with the CSV line;
//! - non-numeric text is never coerced — it is a `NonNumericValue` error;
//! - a missing cell at conversion time is a `MissingValue` error (fill first).
//!
//! Row numbers are 1-based CSV line numbers: the header is line 1, so the first
//! data row is line 2.

use matten_data::{MattenDataError, Table};

fn main() {
    // 1. Duplicate header column.
    match Table::from_csv_str("sales,sales\n1,2") {
        Err(MattenDataError::DuplicateColumn { name }) => {
            println!("duplicate header : {name}");
            assert_eq!(name, "sales");
        }
        other => panic!("expected DuplicateColumn, got {other:?}"),
    }

    // 2. Ragged row: the second data row (CSV line 3) has too few cells.
    match Table::from_csv_str("a,b,c\n1,2,3\n4,5") {
        Err(MattenDataError::RaggedRow {
            row,
            expected,
            actual,
        }) => {
            println!("ragged row       : line {row}, expected {expected}, got {actual}");
            assert_eq!((row, expected, actual), (3, 3, 2));
        }
        other => panic!("expected RaggedRow, got {other:?}"),
    }

    // 3. Non-numeric value during conversion.
    let with_text = Table::from_csv_str("label,value\nok,10\nbad,oops")
        .expect("parses fine; the text only fails at numeric conversion");
    match with_text
        .select_columns(["value"])
        .and_then(|t| t.try_numeric())
    {
        Err(MattenDataError::NonNumericValue { column, row, value }) => {
            println!("non-numeric value: column={column}, line={row}, value={value:?}");
            assert_eq!((column.as_str(), row, value.as_str()), ("value", 3, "oops"));
        }
        other => panic!("expected NonNumericValue, got {other:?}"),
    }

    // 4. Missing value during conversion (fill it first to proceed).
    let with_missing =
        Table::from_csv_str("a,b\n1,2\n3,").expect("missing cell is allowed in a Table");
    match with_missing.try_numeric() {
        Err(MattenDataError::MissingValue { column, row }) => {
            println!("missing value    : column={column}, line={row}");
            assert_eq!((column.as_str(), row), ("b", 3));
        }
        other => panic!("expected MissingValue, got {other:?}"),
    }

    println!("data_05_errors: OK");
}
