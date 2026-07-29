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

/// Same as [`temp_csv`], but for raw bytes that may not be valid UTF-8.
fn temp_csv_bytes(content: &[u8]) -> PathBuf {
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

#[test]
fn equivalence_non_numeric_columns_via_debug() {
    // The other equivalence tests compare via try_numeric().to_tensor(), which
    // only proves parity for all-numeric data. This test covers a text column,
    // a bool column, and a missing value, compared at full fidelity via Table's
    // Debug derive -- no new public API needed to see raw cell values.
    //
    // Per-batch technique (proves multi-batch x non-numeric, without adding a
    // public Table merge/slice accessor): for each batch CsvBatchReader
    // produces, write a separate temp file containing just the header plus
    // that batch's own data lines, load it via Table::from_csv_path, and
    // Debug-compare it against the batch. CSV quoting here is line-local (no
    // field spans multiple physical lines), so slicing by physical line and
    // reloading each slice independently is a valid comparison.
    let header = "name,active,note";
    let data_lines = [
        "alice,true,hello",
        "bob,false,",
        "café,true,\"a, b\"",
        "dan,false,world",
    ];
    let content = format!("{header}\n{}", data_lines.join("\n"));
    let path = temp_csv(&content);

    let whole = Table::from_csv_path(&path).unwrap();

    let mut reader = CsvBatchReader::open(&path, 2).unwrap();
    let mut start = 0;
    while let Some(batch) = reader.next_batch().unwrap() {
        let end = start + batch.row_count();
        // data_lines[start..end] indexes by data row, which only lines up with
        // CsvBatchReader's row count because this fixture has no blank lines
        // (a blank line is skipped as a stray empty record but still consumes
        // a slot in `data_lines`, which would desync the two indices).
        let sub_file = format!("{header}\n{}", data_lines[start..end].join("\n"));
        let sub_path = temp_csv(&sub_file);
        let sub_table = Table::from_csv_path(&sub_path).unwrap();

        assert_eq!(
            format!("{batch:?}"),
            format!("{sub_table:?}"),
            "batch covering rows {start}..{end} must be Debug-identical to \
             Table::from_csv_path loading just those rows"
        );

        start = end;
    }
    assert_eq!(start, whole.row_count());

    // Also keep the cheap single-batch-covers-everything compare.
    let mut single_batch_reader = CsvBatchReader::open(&path, 100).unwrap();
    let single_batch = single_batch_reader
        .next_batch()
        .unwrap()
        .expect("one batch covering the whole file");
    assert!(single_batch_reader.next_batch().unwrap().is_none());
    assert_eq!(format!("{single_batch:?}"), format!("{whole:?}"));
}

// ── documented divergences from Table::from_csv_path (RFC-082 §4.3) ────────

#[test]
fn only_line_terminators_is_empty_input_on_both_paths() {
    // The non-diverging side of the boundary: a file containing ONLY line
    // terminators trims to empty, so both paths agree on EmptyInput. This
    // must keep passing so the diverging case below cannot be misread as
    // "any blank-looking file diverges".
    let content = "\n\n";
    let path = temp_csv(content);

    assert!(matches!(
        Table::from_csv_path(&path),
        Err(MattenDataError::EmptyInput)
    ));
    assert!(matches!(
        CsvBatchReader::open(&path, 10),
        Err(MattenDataError::EmptyInput)
    ));
}

#[test]
fn blank_but_not_empty_file_diverges_from_from_csv_path_documented() {
    // Table::from_csv_path checks whether the WHOLE input trims to empty
    // before parsing; str::trim() strips spaces and tabs along with line
    // terminators, so a file with a stray space or tab still trims to empty
    // and from_csv_path still reports EmptyInput. CsvBatchReader has no such
    // upfront whole-file check (it would require buffering the file first)
    // and instead parses the first line as a header record with one
    // empty-named column, reporting Csv -- this is where the two paths
    // actually diverge. Documented, accepted divergence (stream.rs doc
    // comments); lock BOTH sides here so it cannot drift unnoticed. See the
    // test above for the boundary's non-diverging side (line terminators
    // alone, with no space or tab).
    let content = "   \n  \n";
    let path = temp_csv(content);

    assert!(matches!(
        Table::from_csv_path(&path),
        Err(MattenDataError::EmptyInput)
    ));
    assert!(matches!(
        CsvBatchReader::open(&path, 10),
        Err(MattenDataError::Csv { .. })
    ));
}

#[test]
fn invalid_utf8_diverges_in_variant_and_timing_documented() {
    // Table::from_csv_path validates UTF-8 for the whole file upfront
    // (read_to_string) and reports invalid UTF-8 as Io, before returning any
    // data. CsvBatchReader parses incrementally, so invalid UTF-8 is a
    // mid-stream Csv error, and a valid batch may already have been returned
    // before the bad bytes are reached. Lock both the variant difference and
    // the timing difference.
    let mut content = b"a,b\n1,2\n3,4\n".to_vec();
    content.extend_from_slice(b"\xff\xfe,6\n"); // invalid UTF-8 in a later row
    let path = temp_csv_bytes(&content);

    assert!(matches!(
        Table::from_csv_path(&path),
        Err(MattenDataError::Io { .. })
    ));

    let mut reader = CsvBatchReader::open(&path, 2).unwrap();
    // batch_rows = 2 covers rows 1-2 ("1,2" and "3,4"), both valid and BEFORE
    // the bad bytes -- this batch must be delivered successfully first.
    let first = reader
        .next_batch()
        .unwrap()
        .expect("valid batch delivered before the bad bytes");
    assert_eq!(first.row_count(), 2);

    // The next call reaches the invalid UTF-8 row and fails as Csv, not Io.
    assert!(matches!(
        reader.next_batch(),
        Err(MattenDataError::Csv { .. })
    ));
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
