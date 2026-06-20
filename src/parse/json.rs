//! JSON boundary parser for [`Tensor`](crate::Tensor) (RFC-009, `json` feature).
//!
//! Accepts two forms:
//!
//! **Canonical object form** (preferred for serialisation round-trips):
//! ```json
//! { "shape": [2, 2], "data": [1.0, 2.0, 3.0, 4.0] }
//! ```
//!
//! **Convenience nested-array form** (rank 1 and rank 2):
//! ```json
//! [[1.0, 2.0], [3.0, 4.0]]
//! ```
//!
//! All APIs are Result-zone: they never panic on malformed input.

use crate::Tensor;
use crate::error::{DataFormat, MattenError};
use crate::shape::validate_shape;
use serde_json::Value;

/// Maximum nesting depth accepted in nested-array form.
const MAX_NESTING: usize = 8; // matches MAX_NDIM

/// Maximum total element count accepted from a JSON payload.
const MAX_JSON_ELEMENTS: usize = 1 << 24; // 16 million

fn parse_err(msg: impl Into<String>) -> MattenError {
    MattenError::Parse {
        format: DataFormat::Json,
        message: msg.into(),
    }
}

/// Parses a JSON string into a [`Tensor`].
///
/// Accepts the canonical `{"shape":[…],"data":[…]}` object form and the
/// convenience nested-array form (`[[1,2],[3,4]]`). Returns
/// [`MattenError::Parse`] for any structural or type error; never panics.
pub(crate) fn from_json_str(input: &str) -> Result<Tensor, MattenError> {
    let value: Value =
        serde_json::from_str(input).map_err(|e| parse_err(format!("invalid JSON: {e}")))?;

    match &value {
        Value::Object(_) => parse_object_form(&value),
        Value::Array(_) => parse_nested_form(&value),
        _ => Err(parse_err(
            "expected a JSON object or array at the top level",
        )),
    }
}

// ---- canonical object form ----------------------------------------------

fn parse_object_form(value: &Value) -> Result<Tensor, MattenError> {
    let obj = value.as_object().unwrap(); // guarded by caller

    let shape_val = obj
        .get("shape")
        .ok_or_else(|| parse_err("missing \"shape\" field"))?;
    let data_val = obj
        .get("data")
        .ok_or_else(|| parse_err("missing \"data\" field"))?;

    let shape: Vec<usize> = shape_val
        .as_array()
        .ok_or_else(|| parse_err("\"shape\" must be an array"))?
        .iter()
        .enumerate()
        .map(|(i, v)| {
            v.as_u64()
                .ok_or_else(|| parse_err(format!("\"shape\"[{i}] must be a non-negative integer")))
                .map(|n| n as usize)
        })
        .collect::<Result<_, _>>()?;

    let data: Vec<f64> = parse_numeric_array(
        data_val
            .as_array()
            .ok_or_else(|| parse_err("\"data\" must be an array"))?,
        "\"data\"",
    )?;

    Tensor::try_new(data, &shape).map_err(|e| parse_err(e.to_string()))
}

fn parse_numeric_array(arr: &[Value], context: &str) -> Result<Vec<f64>, MattenError> {
    if arr.len() > MAX_JSON_ELEMENTS {
        return Err(parse_err(format!(
            "{context} has {} elements, exceeding the limit of {MAX_JSON_ELEMENTS}",
            arr.len()
        )));
    }
    arr.iter()
        .enumerate()
        .map(|(i, v)| {
            v.as_f64().ok_or_else(|| {
                parse_err(format!(
                    "{context}[{i}] is not a number (got {})",
                    json_type_name(v)
                ))
            })
        })
        .collect()
}

// ---- nested-array form --------------------------------------------------

fn parse_nested_form(value: &Value) -> Result<Tensor, MattenError> {
    let (data, shape) = extract_nested(value, 0)?;
    // validate shape
    validate_shape(&shape, "from_json").map_err(|e| parse_err(e.to_string()))?;
    Tensor::try_new(data, &shape).map_err(|e| parse_err(e.to_string()))
}

/// Recursively extracts flat data and shape from a nested JSON array.
/// Returns `(flat_data, shape)`.
fn extract_nested(value: &Value, depth: usize) -> Result<(Vec<f64>, Vec<usize>), MattenError> {
    if depth > MAX_NESTING {
        return Err(parse_err(format!(
            "nested array exceeds maximum depth of {MAX_NESTING}"
        )));
    }

    match value {
        Value::Array(arr) => {
            if arr.is_empty() {
                return Err(parse_err("empty arrays are not supported in from_json"));
            }
            if arr.len() > MAX_JSON_ELEMENTS {
                return Err(parse_err(format!(
                    "array at depth {depth} has {} elements, exceeding the limit",
                    arr.len()
                )));
            }

            // Peek at the first element to decide leaf or nested.
            match &arr[0] {
                Value::Array(_) => {
                    // Nested: each element must also be an array of the same length.
                    let (first_data, mut inner_shape) = extract_nested(&arr[0], depth + 1)?;
                    let cols = first_data.len(); // product of inner_shape
                    let mut flat = first_data;

                    for (i, item) in arr.iter().enumerate().skip(1) {
                        let (row_data, row_shape) = extract_nested(item, depth + 1)?;
                        if row_shape != inner_shape {
                            return Err(parse_err(format!(
                                "ragged nested array: row 0 has shape {inner_shape:?} \
                                 but row {i} has shape {row_shape:?}"
                            )));
                        }
                        flat.extend(row_data);
                    }
                    // Prepend outer dimension
                    inner_shape.insert(0, arr.len());
                    let _ = cols; // suppressed
                    Ok((flat, inner_shape))
                }
                Value::Number(_) => {
                    // Leaf level: all elements must be numeric.
                    let data = parse_numeric_array(arr, &format!("array at depth {depth}"))?;
                    Ok((data, vec![arr.len()]))
                }
                other => Err(parse_err(format!(
                    "expected number or array at depth {depth}, got {}",
                    json_type_name(other)
                ))),
            }
        }
        Value::Number(_) => {
            // Scalar element reached unexpectedly at a level where we expected an array.
            let v = value.as_f64().unwrap();
            Ok((vec![v], vec![]))
        }
        other => Err(parse_err(format!(
            "expected array or number, got {}",
            json_type_name(other)
        ))),
    }
}

#[allow(unreachable_patterns)] // Value is #[non_exhaustive]
fn json_type_name(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
        _ => "unknown",
    }
}
