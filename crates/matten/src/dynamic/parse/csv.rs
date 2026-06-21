//! Dynamic CSV parser — infers `Element` per field (RFC-011 §8).
//!
//! Policy per field:
//! - parseable as `i64` → `Int`
//! - parseable as `f64` → `Float`
//! - empty field → `None`
//! - "true"/"false" (case-insensitive) → `Bool`
//! - everything else → `Text`

use crate::Tensor;
use crate::dynamic::element::Element;
use crate::error::{DataFormat, MattenError};

fn parse_err(msg: impl Into<String>) -> MattenError {
    MattenError::Parse {
        format: DataFormat::Csv,
        message: msg.into(),
    }
}

/// Parses a CSV string into a dynamic `Tensor` with shape `[rows, cols]`.
///
/// Each field is independently inferred to the most specific `Element` type.
/// Ragged rows are rejected.
pub(crate) fn from_csv_dynamic(input: &str) -> Result<Tensor, MattenError> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(false)
        .from_reader(input.as_bytes());

    let mut data: Vec<Element> = Vec::new();
    let mut cols: Option<usize> = None;
    let mut rows: usize = 0;

    for result in reader.records() {
        let record =
            result.map_err(|e| parse_err(format!("CSV parse error at row {rows}: {e}")))?;
        let row_cols = record.len();

        match cols {
            None => {
                if row_cols == 0 {
                    return Err(parse_err("CSV has no columns"));
                }
                cols = Some(row_cols);
            }
            Some(expected) if row_cols != expected => {
                return Err(parse_err(format!(
                    "ragged CSV: row 0 has {expected} columns but row {rows} has {row_cols}"
                )));
            }
            _ => {}
        }

        for field in record.iter() {
            data.push(infer_element(field.trim()));
        }
        rows += 1;
    }

    if rows == 0 {
        return Err(parse_err("CSV input is empty"));
    }

    let c = cols.unwrap_or(0);
    Tensor::try_from_elements(data, &[rows, c]).map_err(|e| parse_err(e.to_string()))
}

/// Infers the most specific `Element` for a CSV field string.
fn infer_element(field: &str) -> Element {
    if field.is_empty() {
        return Element::None;
    }
    // Boolean (case-insensitive)
    if field.eq_ignore_ascii_case("true") {
        return Element::Bool(true);
    }
    if field.eq_ignore_ascii_case("false") {
        return Element::Bool(false);
    }
    // Integer
    if let Ok(i) = field.parse::<i64>() {
        return Element::Int(i);
    }
    // Float
    if let Ok(f) = field.parse::<f64>() {
        return Element::Float(f);
    }
    // Text fallback
    Element::text(field)
}
