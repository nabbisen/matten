//! # Companion example: batched CSV reading (matten-data, RFC-082)
//!
//! Run: `cargo run -p matten-data --example data_csv_batches --features streaming`
//!
//! ## What this shows
//! `CsvBatchReader` reads a CSV file `batch_rows` rows at a time, so a file
//! larger than available memory can still be processed as long as the *work* is
//! batch-friendly. A batch is an ordinary `Table` — everything that already
//! works on a `Table` works on a batch, unchanged.
//!
//! ## Teaching points
//! - `open` reads the header once; every batch shares it;
//! - `next_batch` returns `Ok(None)` once exhausted, and keeps returning it;
//! - concatenating every batch reproduces exactly what `Table::from_csv_path`
//!   would have loaded in one shot — batching is a memory strategy, not a
//!   different parser.

use matten_data::CsvBatchReader;

fn main() -> Result<(), matten_data::MattenDataError> {
    let path = std::env::temp_dir().join(format!(
        "matten_data_csv_batches_example_{}.csv",
        std::process::id()
    ));
    std::fs::write(&path, "sales,cost\n100,40\n150,45\n120,55\n90,30\n200,60").unwrap();

    let mut reader = CsvBatchReader::open(&path, 2)?;
    let mut total_rows = 0;

    while let Some(batch) = reader.next_batch()? {
        println!(
            "batch: {} rows, columns {:?}",
            batch.row_count(),
            batch.column_names()
        );
        total_rows += batch.row_count();
    }

    // Every subsequent call keeps returning None.
    assert!(reader.next_batch()?.is_none());

    println!("total rows read across all batches: {total_rows}");
    assert_eq!(total_rows, 5);

    std::fs::remove_file(&path).ok();
    println!("csv_batches: OK");
    Ok(())
}
