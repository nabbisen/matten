//! # data_01 — Inspecting a table with `schema_summary`
//!
//! Run: `cargo run -p matten-data --example data_01_schema_summary`
//!
//! ## What this shows
//! How to look at a table *before* converting it: row count, column count,
//! column names, the number of missing cells per column, and the simple inferred
//! kind of each column.
//!
//! ## Teaching points
//! - inspection is cheap and non-destructive — nothing is converted yet;
//! - the schema summary is a small, printable description, not a query engine;
//! - inferred kinds (`integer`, `float`, `boolean`, `text`, `mixed`,
//!   `missing-only`) are a hint for which columns can become numeric.

use matten_data::Table;

fn main() -> Result<(), matten_data::MattenDataError> {
    let csv = "\
region,sales,cost,active
north,100,40.5,true
south,150,,true
east,120,55.0,false";

    let table = Table::from_csv_str(csv)?;

    // Top-level shape of the table.
    println!("rows    : {}", table.row_count());
    println!("columns : {}", table.column_count());
    println!("names   : {:?}", table.column_names());

    // A printable, one-glance summary (Table: R rows x C columns, then a line
    // per column with its inferred kind and missing count).
    let summary = table.schema_summary();
    print!("{summary}");

    // The same information, per column, if you want to act on it in code.
    println!("--- per-column ---");
    for col in summary.column_summaries() {
        println!(
            "{:<8} kind={:<7} missing={}",
            col.name, col.kind, col.missing
        );
    }

    // The "cost" column has exactly one missing cell (south).
    let cost = summary
        .column_summaries()
        .iter()
        .find(|c| c.name == "cost")
        .expect("cost column exists");
    assert_eq!(cost.missing, 1);
    assert_eq!(table.row_count(), 3);
    assert_eq!(table.column_count(), 4);

    println!("data_01_schema_summary: OK");
    Ok(())
}
