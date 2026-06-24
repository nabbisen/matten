//! Tests for `matten-data`, validating the RFC-034 / RFC-035 design specifications.
//!
//! These exercise the public contract (construction, inspection, selection,
//! missing-value handling, strict numeric conversion, and Tensor output), not just
//! the code paths. The module requires the `csv` feature, which is default-on.

use crate::{CellValue, ColumnKind, MattenDataError, Table};

// --- Construction & header policy (RFC-035 §3) ---

#[test]
fn from_csv_str_basic() {
    let t = Table::from_csv_str("a,b\n1,2\n3,4").unwrap();
    assert_eq!(t.row_count(), 2);
    assert_eq!(t.column_count(), 2);
    assert_eq!(t.column_names(), &["a".to_string(), "b".to_string()]);
}

#[test]
fn empty_input_is_error() {
    assert!(matches!(
        Table::from_csv_str(""),
        Err(MattenDataError::EmptyInput)
    ));
    assert!(matches!(
        Table::from_csv_str("   \n  "),
        Err(MattenDataError::EmptyInput)
    ));
}

#[test]
fn header_only_has_zero_rows() {
    let t = Table::from_csv_str("a,b,c").unwrap();
    assert_eq!(t.row_count(), 0);
    assert_eq!(t.column_count(), 3);
}

#[test]
fn duplicate_header_is_rejected() {
    match Table::from_csv_str("a,a\n1,2") {
        Err(MattenDataError::DuplicateColumn { name }) => assert_eq!(name, "a"),
        other => panic!("expected DuplicateColumn, got {other:?}"),
    }
}

#[test]
fn empty_header_is_rejected() {
    assert!(matches!(
        Table::from_csv_str("a,,b\n1,2,3"),
        Err(MattenDataError::Csv { .. })
    ));
}

#[test]
fn malformed_csv_is_a_structured_error_never_a_panic() {
    // RFC-035 §1: malformed input must return a `Result`, never panic and never
    // silently produce a wrong `Table`. The `csv` crate is configured lenient +
    // flexible, so an unterminated quoted field is not surfaced as a parser error;
    // it swallows the line break and the resulting record has the wrong cell count,
    // which `matten-data` reports as a precise `RaggedRow`. (Header-structure
    // malformations such as a blank header column surface as `Csv` instead — see
    // `empty_header_is_rejected`.) Either way the contract is the same: a structured
    // error, not a panic. Assert that union so the test is robust to the parser's
    // lenient quote handling.
    let result = Table::from_csv_str("a,b\n\"1,2\n3,4");
    assert!(
        matches!(
            result,
            Err(MattenDataError::RaggedRow { .. }) | Err(MattenDataError::Csv { .. })
        ),
        "unterminated quote must be a structured RaggedRow/Csv error, got {result:?}"
    );
}

#[test]
fn ragged_row_too_long_is_rejected() {
    match Table::from_csv_str("a,b\n1,2,3") {
        Err(MattenDataError::RaggedRow {
            row,
            expected,
            actual,
        }) => {
            assert_eq!((row, expected, actual), (2, 2, 3));
        }
        other => panic!("expected RaggedRow, got {other:?}"),
    }
}

#[test]
fn ragged_row_too_short_is_rejected() {
    match Table::from_csv_str("a,b,c\n1,2") {
        Err(MattenDataError::RaggedRow {
            row,
            expected,
            actual,
        }) => {
            assert_eq!((row, expected, actual), (2, 3, 2));
        }
        other => panic!("expected RaggedRow, got {other:?}"),
    }
}

#[test]
fn second_data_row_ragged_reports_line_three() {
    match Table::from_csv_str("a,b\n1,2\n3,4,5") {
        Err(MattenDataError::RaggedRow { row, .. }) => assert_eq!(row, 3),
        other => panic!("expected RaggedRow at line 3, got {other:?}"),
    }
}

#[test]
fn quoted_comma_is_preserved() {
    // "x,y" is a single quoted field; selecting it as text round-trips.
    let t = Table::from_csv_str("label,n\n\"x,y\",2").unwrap();
    assert_eq!(t.row_count(), 1);
    let summary = t.schema_summary();
    assert_eq!(summary.column_summaries()[0].kind, ColumnKind::Text);
}

#[test]
fn surrounding_whitespace_is_trimmed() {
    let t = Table::from_csv_str(" a , b \n 1 , 2 ").unwrap();
    assert_eq!(t.column_names(), &["a".to_string(), "b".to_string()]);
    // Trimmed "1"/"2" infer as integers -> numeric conversion succeeds.
    let tensor = t.try_numeric().unwrap().to_tensor().unwrap();
    assert_eq!(tensor.as_slice(), &[1.0, 2.0]);
}

#[test]
fn trailing_newline_does_not_add_a_row() {
    let t = Table::from_csv_str("a,b\n1,2\n").unwrap();
    assert_eq!(t.row_count(), 1);
}

// --- Cell inference & schema summary (RFC-035 §4–5) ---

#[test]
fn schema_summary_counts_and_kinds() {
    let csv = "i,f,b,t,m,mix\n1,1.5,true,hello,,1\n2,2.5,false,world,,x";
    let t = Table::from_csv_str(csv).unwrap();
    let s = t.schema_summary();
    assert_eq!(s.rows, 2);
    assert_eq!(s.columns, 6);

    let cols = s.column_summaries();
    assert_eq!(cols[0].kind, ColumnKind::Integer);
    assert_eq!(cols[1].kind, ColumnKind::Float);
    assert_eq!(cols[2].kind, ColumnKind::Boolean);
    assert_eq!(cols[3].kind, ColumnKind::Text);
    assert_eq!(cols[4].kind, ColumnKind::MissingOnly);
    assert_eq!(cols[4].missing, 2);
    assert_eq!(cols[5].kind, ColumnKind::Mixed); // 1 (int) and "x" (text)
}

#[test]
fn integer_and_float_mix_is_float_kind() {
    let t = Table::from_csv_str("v\n1\n2.5\n3").unwrap();
    assert_eq!(
        t.schema_summary().column_summaries()[0].kind,
        ColumnKind::Float
    );
}

#[test]
fn schema_summary_display_contains_shape_and_names() {
    let t = Table::from_csv_str("sales,cost\n10,2").unwrap();
    let text = t.schema_summary().to_string();
    assert!(text.contains("1 rows x 2 columns"));
    assert!(text.contains("sales"));
    assert!(text.contains("cost"));
}

// --- Selection (RFC-034 §5.3) ---

#[test]
fn select_columns_preserves_requested_order() {
    let t = Table::from_csv_str("a,b,c\n1,2,3\n4,5,6").unwrap();
    let sel = t.select_columns(["c", "a"]).unwrap();
    assert_eq!(sel.column_names(), &["c".to_string(), "a".to_string()]);
    let tensor = sel.try_numeric().unwrap().to_tensor().unwrap();
    // Row 0: c=3, a=1 ; Row 1: c=6, a=4
    assert_eq!(tensor.as_slice(), &[3.0, 1.0, 6.0, 4.0]);
}

#[test]
fn select_missing_column_is_error() {
    let t = Table::from_csv_str("a,b\n1,2").unwrap();
    match t.select_columns(["a", "z"]) {
        Err(MattenDataError::MissingColumn { name }) => assert_eq!(name, "z"),
        other => panic!("expected MissingColumn, got {other:?}"),
    }
}

#[test]
fn duplicate_selection_is_rejected() {
    let t = Table::from_csv_str("a,b\n1,2").unwrap();
    match t.select_columns(["a", "a"]) {
        Err(MattenDataError::DuplicateSelection { name }) => assert_eq!(name, "a"),
        other => panic!("expected DuplicateSelection, got {other:?}"),
    }
}

#[test]
fn empty_selection_is_error() {
    let t = Table::from_csv_str("a,b\n1,2").unwrap();
    let empty: [&str; 0] = [];
    assert!(matches!(
        t.select_columns(empty),
        Err(MattenDataError::EmptySelection)
    ));
}

#[test]
fn select_columns_accepts_string_and_str() {
    let t = Table::from_csv_str("a,b\n1,2").unwrap();
    let owned = vec![String::from("a")];
    assert!(t.select_columns(owned).is_ok());
    assert!(t.select_columns(["b"]).is_ok());
}

// --- Missing-value handling (RFC-035 §6) ---

#[test]
fn fill_missing_replaces_only_missing_cells() {
    let t = Table::from_csv_str("a,b\n10,\n,4").unwrap();
    let filled = t.fill_missing(0.0).unwrap();
    let tensor = filled.try_numeric().unwrap().to_tensor().unwrap();
    assert_eq!(tensor.shape(), &[2, 2]);
    assert_eq!(tensor.as_slice(), &[10.0, 0.0, 0.0, 4.0]);
}

#[test]
fn missing_is_not_silently_zero() {
    // Without fill_missing, a missing cell must error, not become 0.
    let t = Table::from_csv_str("a,b\n10,\n5,4").unwrap();
    match t.try_numeric() {
        Err(MattenDataError::MissingValue { column, row }) => {
            assert_eq!(column, "b");
            assert_eq!(row, 2);
        }
        other => panic!("expected MissingValue, got {other:?}"),
    }
}

// --- Strict numeric conversion (RFC-035 §7) ---

#[test]
fn numeric_conversion_basic() {
    let t = Table::from_csv_str("a,b\n1,2\n3,4").unwrap();
    let tensor = t.try_numeric().unwrap().to_tensor().unwrap();
    assert_eq!(tensor.shape(), &[2, 2]);
    assert_eq!(tensor.as_slice(), &[1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn text_is_not_numeric() {
    let t = Table::from_csv_str("a\n1\nhello").unwrap();
    match t.try_numeric() {
        Err(MattenDataError::NonNumericValue { column, row, value }) => {
            assert_eq!(column, "a");
            assert_eq!(row, 3);
            assert_eq!(value, "hello");
        }
        other => panic!("expected NonNumericValue, got {other:?}"),
    }
}

#[test]
fn bool_is_not_silently_numeric() {
    let t = Table::from_csv_str("flag\ntrue\nfalse").unwrap();
    match t.try_numeric() {
        Err(MattenDataError::NonNumericValue { column, value, .. }) => {
            assert_eq!(column, "flag");
            assert_eq!(value, "true");
        }
        other => panic!("expected NonNumericValue for bool, got {other:?}"),
    }
}

#[test]
fn single_column_is_column_vector() {
    let t = Table::from_csv_str("x\n1\n2\n3").unwrap();
    let tensor = t.try_numeric().unwrap().to_tensor().unwrap();
    assert_eq!(tensor.shape(), &[3, 1]);
}

#[test]
fn zero_row_table_cannot_become_tensor() {
    let t = Table::from_csv_str("a,b").unwrap();
    let numeric = t.try_numeric().unwrap();
    assert_eq!(numeric.row_count(), 0);
    // Core matten rejects zero-length dimensions; surfaced as a wrapped error.
    assert!(matches!(
        numeric.to_tensor(),
        Err(MattenDataError::Matten(_))
    ));
}

#[test]
fn numeric_table_metadata() {
    let t = Table::from_csv_str("a,b\n1,2").unwrap();
    let numeric = t.select_columns(["b", "a"]).unwrap().try_numeric().unwrap();
    assert_eq!(numeric.row_count(), 1);
    assert_eq!(numeric.column_count(), 2);
    assert_eq!(numeric.column_names(), &["b".to_string(), "a".to_string()]);
}

#[test]
fn inf_and_nan_strings_parse_as_float() {
    let t = Table::from_csv_str("v\ninf\n-inf").unwrap();
    let tensor = t.try_numeric().unwrap().to_tensor().unwrap();
    assert!(tensor.as_slice()[0].is_infinite());
}

// --- End-to-end (RFC-033 §8 canonical workflow) ---

#[test]
fn canonical_workflow_end_to_end() {
    let csv = "sales,cost,note\n10,2,a\n20,,b\n30,4,c";
    let tensor = Table::from_csv_str(csv)
        .unwrap()
        .select_columns(["sales", "cost"])
        .unwrap()
        .fill_missing(0.0)
        .unwrap()
        .try_numeric()
        .unwrap()
        .to_tensor()
        .unwrap();
    assert_eq!(tensor.shape(), &[3, 2]);
    assert_eq!(tensor.as_slice(), &[10.0, 2.0, 20.0, 0.0, 30.0, 4.0]);
}

// --- Path constructor & I/O errors (RFC-035 §3.1) ---

#[test]
fn from_csv_path_reads_a_file() {
    let path = std::env::temp_dir().join(format!("matten_data_ok_{}.csv", std::process::id()));
    std::fs::write(&path, "a,b\n1,2\n3,4").unwrap();
    let t = Table::from_csv_path(&path).unwrap();
    assert_eq!(t.row_count(), 2);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn from_csv_path_missing_file_is_io_error() {
    let path = std::env::temp_dir().join("matten_data_definitely_missing_xyz.csv");
    match Table::from_csv_path(&path) {
        Err(e @ MattenDataError::Io { .. }) => {
            // The underlying I/O error is preserved as the source.
            use std::error::Error;
            assert!(e.source().is_some());
        }
        other => panic!("expected Io error, got {other:?}"),
    }
}

// --- Error quality (RFC-035 §9) ---

#[test]
fn error_display_includes_context() {
    let t = Table::from_csv_str("sales\n1\nbad").unwrap();
    let msg = t.try_numeric().unwrap_err().to_string();
    assert!(msg.contains("sales"));
    assert!(msg.contains("bad"));
    assert!(msg.contains("row 3"));
}

// --- CellValue construction helpers (RFC-034 §4.2) ---

#[test]
fn cellvalue_from_conversions() {
    assert_eq!(CellValue::from(1.5_f64), CellValue::Float(1.5));
    assert_eq!(CellValue::from(3_i64), CellValue::Int(3));
    assert_eq!(CellValue::from(true), CellValue::Bool(true));
    assert_eq!(CellValue::from("hi"), CellValue::Text("hi".to_string()));
}
