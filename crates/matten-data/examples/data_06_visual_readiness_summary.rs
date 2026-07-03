//! # data_06 - Visual readiness summary
//!
//! Run: `cargo run -p matten-data --example data_06_visual_readiness_summary`
//!
//! ## What this shows
//! A compact terminal summary of table-to-Tensor readiness: source columns,
//! selected columns, left-out columns, missing counts, strict conversion, and
//! the final row-major tensor values after explicit cleanup.

use matten_data::{MattenDataError, Table};

fn print_schema(summary: &matten_data::SchemaSummary) {
    println!("rows             {}", summary.rows);
    println!("columns          {}", summary.columns);
    for col in summary.column_summaries() {
        println!(
            "column {:<8} kind={:<7} missing={}",
            col.name, col.kind, col.missing
        );
    }
}

fn main() -> Result<(), MattenDataError> {
    let csv = "\
region,sales,cost,note
north,100,40,ok
south,150,,review
east,120,55,ok";

    let table = Table::from_csv_str(csv)?;
    let source = table.schema_summary();

    println!("== Source table ==");
    println!("source columns   {:?}", table.column_names());
    print_schema(&source);
    assert_eq!(table.column_names(), &["region", "sales", "cost", "note"]);
    assert_eq!(source.rows, 3);
    assert_eq!(source.columns, 4);
    assert_eq!(
        source
            .column_summaries()
            .iter()
            .find(|col| col.name == "cost")
            .map(|col| col.missing),
        Some(1)
    );

    println!();
    println!("== Selection ==");
    let selected = table.select_columns(["sales", "cost"])?;
    println!("selected columns {:?}", selected.column_names());
    println!("left out         [\"region\", \"note\"]");
    assert_eq!(selected.column_names(), &["sales", "cost"]);

    match selected.try_numeric() {
        Err(MattenDataError::MissingValue { column, row }) => {
            println!("strict numeric   Err: missing column={column}, csv_line={row}");
            assert_eq!(column, "cost");
            assert_eq!(row, 3);
        }
        other => panic!("expected MissingValue, got {other:?}"),
    }

    println!();
    println!("== Explicit cleanup ==");
    let filled = selected.fill_missing(0.0)?;
    let numeric = filled.try_numeric()?;
    println!("numeric rows     {}", numeric.row_count());
    println!("numeric columns  {}", numeric.column_count());
    println!("numeric names    {:?}", numeric.column_names());
    assert_eq!(numeric.row_count(), 3);
    assert_eq!(numeric.column_count(), 2);
    assert_eq!(numeric.column_names(), &["sales", "cost"]);

    let tensor = numeric.to_tensor()?;
    println!("tensor shape     {:?}", tensor.shape());
    println!("row-major values {:?}", tensor.as_slice());
    assert_eq!(tensor.shape(), &[3, 2]);
    assert_eq!(tensor.as_slice(), &[100.0, 40.0, 150.0, 0.0, 120.0, 55.0]);

    println!();
    println!("data_06_visual_readiness_summary: OK");
    Ok(())
}
