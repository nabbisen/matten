//! Tests validating the RFC-082 §4 design specifications for `CsvBatchReader`.
//!
//! All tests live in this single integration file. The equivalence tests are the
//! important ones: everything else can pass while an incrementally-parsed batch
//! silently diverges from what `Table::from_csv_path` would have produced.

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use matten_data::{CsvBatchReader, MattenDataError, Table};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Writes `content` to a unique temp file and returns its path. Unique per call
/// (process id + a counter) so parallel test execution cannot collide.
fn temp_csv(content: &str) -> PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "matten_data_streaming_test_{}_{n}.csv",
        std::process::id()
    ));
    std::fs::write(&path, content).unwrap();
    path
}

/// Flattens a `Table` to a numeric `Vec<f64>` via the same conversion path every
/// other test in the workspace uses (`try_numeric` then `to_tensor`).
fn numeric_flat(table: &Table) -> Vec<f64> {
    table
        .try_numeric()
        .expect("numeric conversion")
        .to_tensor()
        .expect("to_tensor")
        .as_slice()
        .to_vec()
}

fn column_names(table: &Table) -> Vec<String> {
    table.column_names().to_vec()
}

// ── batch lifecycle (RFC-082 §4.1) ──────────────────────────────────────────

#[test]
fn exact_batching_3_3_3_1() {
    let path = temp_csv("a,b\n1,10\n2,20\n3,30\n4,40\n5,50\n6,60\n7,70\n8,80\n9,90\n10,100");
    let mut reader = CsvBatchReader::open(&path, 3).unwrap();

    let sizes: Vec<usize> = std::iter::from_fn(|| reader.next_batch().unwrap())
        .map(|t| t.row_count())
        .collect();

    assert_eq!(sizes, vec![3, 3, 3, 1]);
    assert!(reader.next_batch().unwrap().is_none());
}

#[test]
fn file_smaller_than_one_batch_is_a_single_batch() {
    let path = temp_csv("a,b\n1,2\n3,4");
    let mut reader = CsvBatchReader::open(&path, 100).unwrap();

    let batch = reader.next_batch().unwrap().expect("one batch");
    assert_eq!(batch.row_count(), 2);
    assert!(reader.next_batch().unwrap().is_none());
}

#[test]
fn header_only_file_yields_no_batches() {
    let path = temp_csv("a,b,c");
    let mut reader = CsvBatchReader::open(&path, 10).unwrap();
    assert!(reader.next_batch().unwrap().is_none());
}

#[test]
fn repeated_calls_after_exhaustion_keep_returning_none() {
    let path = temp_csv("a\n1\n2");
    let mut reader = CsvBatchReader::open(&path, 10).unwrap();

    assert!(reader.next_batch().unwrap().is_some());
    assert!(reader.next_batch().unwrap().is_none());
    assert!(reader.next_batch().unwrap().is_none());
    assert!(reader.next_batch().unwrap().is_none());
}

#[test]
fn headers_are_applied_to_every_batch_not_just_the_first() {
    let path = temp_csv("x,y\n1,2\n3,4\n5,6\n7,8");
    let mut reader = CsvBatchReader::open(&path, 2).unwrap();

    let first = reader.next_batch().unwrap().unwrap();
    let second = reader.next_batch().unwrap().unwrap();

    assert_eq!(column_names(&first), vec!["x".to_string(), "y".to_string()]);
    assert_eq!(
        column_names(&second),
        vec!["x".to_string(), "y".to_string()]
    );
}

// ── equivalence with Table::from_csv_path (the important tests) ────────────

#[test]
fn equivalence_concatenated_batches_equal_from_csv_path() {
    let content = "a,b,c\n1,2,3\n4,5,6\n7,8,9\n10,11,12\n13,14,15\n16,17,18\n19,20,21";
    let path = temp_csv(content);

    let whole = Table::from_csv_path(&path).unwrap();
    let whole_flat = numeric_flat(&whole);

    let mut reader = CsvBatchReader::open(&path, 3).unwrap();
    let mut batched_flat = Vec::new();
    let mut batched_rows = 0;
    while let Some(batch) = reader.next_batch().unwrap() {
        assert_eq!(column_names(&batch), column_names(&whole));
        batched_rows += batch.row_count();
        batched_flat.extend(numeric_flat(&batch));
    }

    assert_eq!(batched_rows, whole.row_count());
    assert_eq!(batched_flat, whole_flat);
}

#[test]
fn equivalence_trailing_newline() {
    let content = "a,b\n1,2\n3,4\n5,6\n";
    let path = temp_csv(content);

    let whole = Table::from_csv_path(&path).unwrap();
    let whole_flat = numeric_flat(&whole);

    let mut reader = CsvBatchReader::open(&path, 2).unwrap();
    let mut batched_flat = Vec::new();
    let mut batched_rows = 0;
    while let Some(batch) = reader.next_batch().unwrap() {
        batched_rows += batch.row_count();
        batched_flat.extend(numeric_flat(&batch));
    }

    assert_eq!(batched_rows, whole.row_count());
    assert_eq!(batched_flat, whole_flat);
}

#[test]
fn equivalence_blank_line_before_end() {
    // A stray blank line mid-file must be skipped identically by both paths
    // (Table::from_csv_str already skips a "stray fully-empty record").
    let content = "a,b\n1,2\n3,4\n\n5,6";
    let path = temp_csv(content);

    let whole = Table::from_csv_path(&path).unwrap();
    let whole_flat = numeric_flat(&whole);

    let mut reader = CsvBatchReader::open(&path, 2).unwrap();
    let mut batched_flat = Vec::new();
    let mut batched_rows = 0;
    while let Some(batch) = reader.next_batch().unwrap() {
        batched_rows += batch.row_count();
        batched_flat.extend(numeric_flat(&batch));
    }

    assert_eq!(batched_rows, whole.row_count());
    assert_eq!(batched_flat, whole_flat);
}

// ── malformed-row policy (RFC-082 §4.3) ─────────────────────────────────────

#[test]
fn malformed_row_same_variant_and_line_number_as_from_csv_path() {
    // Row 3 (CSV line 4) is ragged: two fields instead of three.
    let content = "a,b,c\n1,2,3\n4,5,6\n7,8\n10,11,12";
    let path = temp_csv(content);

    let whole_err = Table::from_csv_path(&path).unwrap_err();
    let mut reader = CsvBatchReader::open(&path, 10).unwrap();
    let batch_err = reader.next_batch().unwrap_err();

    match (&whole_err, &batch_err) {
        (
            MattenDataError::RaggedRow {
                row: r1,
                expected: e1,
                actual: a1,
            },
            MattenDataError::RaggedRow {
                row: r2,
                expected: e2,
                actual: a2,
            },
        ) => {
            assert_eq!(
                r1, r2,
                "line number must match Table::from_csv_path exactly"
            );
            assert_eq!(e1, e2);
            assert_eq!(a1, a2);
        }
        other => panic!("expected RaggedRow on both paths, got {other:?}"),
    }
}

#[test]
fn line_number_parity_at_a_batch_boundary() {
    // 6 good rows, then a ragged row 7 (CSV line 8), with batch_rows = 3 so the
    // bad row falls in the THIRD batch, not the first -- this is what actually
    // exercises the reader's running line counter across batch boundaries.
    let content = "a,b,c\n1,2,3\n4,5,6\n7,8,9\n10,11,12\n13,14,15\n16,17,18\n19,20\n22,23,24";
    let path = temp_csv(content);

    let whole_err = Table::from_csv_path(&path).unwrap_err();

    let mut reader = CsvBatchReader::open(&path, 3).unwrap();
    reader.next_batch().unwrap(); // rows 1-3, ok
    reader.next_batch().unwrap(); // rows 4-6, ok
    let batch_err = reader.next_batch().unwrap_err(); // row 7 is ragged

    match (&whole_err, &batch_err) {
        (
            MattenDataError::RaggedRow { row: r1, .. },
            MattenDataError::RaggedRow { row: r2, .. },
        ) => {
            assert_eq!(r1, r2, "line number must match across the batch boundary");
        }
        other => panic!("expected RaggedRow on both paths, got {other:?}"),
    }
}

#[test]
fn reader_is_unusable_after_an_error() {
    let content = "a,b\n1,2\n3\n5,6";
    let path = temp_csv(content);
    let mut reader = CsvBatchReader::open(&path, 10).unwrap();

    assert!(reader.next_batch().is_err());
    // Must not resume, repeat the error, or panic.
    assert!(reader.next_batch().unwrap().is_none());
    assert!(reader.next_batch().unwrap().is_none());
}

// ── open() validation ────────────────────────────────────────────────────────

#[test]
fn batch_rows_zero_is_rejected_at_open() {
    let path = temp_csv("a,b\n1,2");
    assert!(matches!(
        CsvBatchReader::open(&path, 0),
        Err(MattenDataError::InvalidBatchSize)
    ));
}

#[test]
fn missing_file_is_rejected_at_open() {
    let path = std::env::temp_dir().join("matten_data_streaming_test_does_not_exist.csv");
    assert!(matches!(
        CsvBatchReader::open(&path, 10),
        Err(MattenDataError::Io { .. })
    ));
}
