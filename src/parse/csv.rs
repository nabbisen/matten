//! CSV boundary parser for [`Tensor`](crate::Tensor) (RFC-009, `csv` feature).
//!
//! Phase 1 accepts rectangular numeric-only CSV. Shape is inferred as
//! `[rows, cols]`. Empty fields and non-numeric values are errors.
//!
//! All APIs are Result-zone: they never panic on malformed input.

use crate::Tensor;
use crate::error::{DataFormat, MattenError};

fn parse_err(msg: impl Into<String>) -> MattenError {
    MattenError::Parse {
        format: DataFormat::Csv,
        message: msg.into(),
    }
}

/// Parses a CSV string into a [`Tensor`] with shape `[rows, cols]`.
///
/// All fields must be valid `f64` values. Rows must have equal column counts.
/// Returns [`MattenError::Parse`] for any structural or type error.
pub(crate) fn from_csv_str(input: &str) -> Result<Tensor, MattenError> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(false)
        .from_reader(input.as_bytes());

    let mut data: Vec<f64> = Vec::new();
    let mut cols: Option<usize> = None;
    let mut rows: usize = 0;

    for result in reader.records() {
        let record =
            result.map_err(|e| parse_err(format!("CSV parse error at row {rows}: {e}")))?;

        let row_cols = record.len();

        // Establish or verify column count.
        match cols {
            None => {
                if row_cols == 0 {
                    return Err(parse_err("CSV has no columns"));
                }
                cols = Some(row_cols);
            }
            Some(expected) => {
                if row_cols != expected {
                    return Err(parse_err(format!(
                        "ragged CSV: row 0 has {expected} columns but row {rows} has {row_cols}"
                    )));
                }
            }
        }

        // Parse each field as f64.
        for (col, field) in record.iter().enumerate() {
            let v = field.trim().parse::<f64>().map_err(|_| {
                parse_err(format!(
                    "at row {rows}, column {col}: expected f64, got {:?}",
                    field
                ))
            })?;
            data.push(v);
        }

        rows += 1;
    }

    if rows == 0 {
        return Err(parse_err("CSV input is empty"));
    }

    let c = cols.unwrap_or(0);
    Tensor::try_new(data, &[rows, c]).map_err(|e| parse_err(e.to_string()))
}
