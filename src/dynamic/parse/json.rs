//! Dynamic JSON parser — maps JSON values to `Element` (RFC-011 §11.3).
//!
//! JSON integer-looking numbers map to `Element::Int(i64)` when exactly
//! representable; otherwise to `Element::Float(f64)`. Null → `None`,
//! string → `Text`, boolean → `Bool`.

use crate::Tensor;
use crate::dynamic::element::Element;
use crate::error::{DataFormat, MattenError};
use crate::shape::validate_shape;
use serde_json::Value;

fn parse_err(msg: impl Into<String>) -> MattenError {
    MattenError::Parse {
        format: DataFormat::Json,
        message: msg.into(),
    }
}

/// Parses a JSON string into a dynamic `Tensor`.
///
/// Accepts:
/// - canonical object form `{"shape":[…],"data":[…]}` where data elements
///   may be numbers, strings, booleans, or nulls;
/// - nested array form (rank 1 and 2) with mixed value types.
pub(crate) fn from_json_dynamic(input: &str) -> Result<Tensor, MattenError> {
    let value: Value =
        serde_json::from_str(input).map_err(|e| parse_err(format!("invalid JSON: {e}")))?;
    match &value {
        Value::Object(_) => parse_object_dynamic(&value),
        Value::Array(_) => parse_nested_dynamic(&value),
        _ => Err(parse_err(
            "expected a JSON object or array at the top level",
        )),
    }
}

// ── canonical object form ─────────────────────────────────────────────────

fn parse_object_dynamic(value: &Value) -> Result<Tensor, MattenError> {
    let obj = value.as_object().unwrap();
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
                .and_then(|n| {
                    usize::try_from(n)
                        .map_err(|_| parse_err("shape dimension overflows usize".to_string()))
                })
        })
        .collect::<Result<_, _>>()?;

    let elements: Vec<Element> = data_val
        .as_array()
        .ok_or_else(|| parse_err("\"data\" must be an array"))?
        .iter()
        .enumerate()
        .map(|(i, v)| {
            json_value_to_element(v)
                .ok_or_else(|| parse_err(format!("\"data\"[{i}]: unsupported value type")))
        })
        .collect::<Result<_, _>>()?;

    validate_shape(&shape, "from_json").map_err(|e| parse_err(e.to_string()))?;
    Tensor::try_from_elements(elements, &shape).map_err(|e| parse_err(e.to_string()))
}

// ── nested array form ─────────────────────────────────────────────────────

const MAX_DEPTH: usize = 8;
const MAX_ELEMENTS: usize = 1 << 24;

fn parse_nested_dynamic(value: &Value) -> Result<Tensor, MattenError> {
    let (data, shape) = extract_nested_dynamic(value, 0)?;
    validate_shape(&shape, "from_json").map_err(|e| parse_err(e.to_string()))?;
    Tensor::try_from_elements(data, &shape).map_err(|e| parse_err(e.to_string()))
}

fn extract_nested_dynamic(
    value: &Value,
    depth: usize,
) -> Result<(Vec<Element>, Vec<usize>), MattenError> {
    if depth > MAX_DEPTH {
        return Err(parse_err(format!(
            "nested array exceeds maximum depth of {MAX_DEPTH}"
        )));
    }
    match value {
        Value::Array(arr) => {
            if arr.is_empty() {
                return Err(parse_err("empty arrays are not supported in from_json"));
            }
            if arr.len() > MAX_ELEMENTS {
                return Err(parse_err(format!(
                    "array at depth {depth} exceeds element limit"
                )));
            }
            // Determine if this is a leaf (values) or nested (arrays)
            match &arr[0] {
                Value::Array(_) => {
                    let (first_data, mut inner_shape) = extract_nested_dynamic(&arr[0], depth + 1)?;
                    let mut flat = first_data;
                    for (i, item) in arr.iter().enumerate().skip(1) {
                        let (row_data, row_shape) = extract_nested_dynamic(item, depth + 1)?;
                        if row_shape != inner_shape {
                            return Err(parse_err(format!(
                                "ragged nested array: row 0 has shape {inner_shape:?} \
                                 but row {i} has shape {row_shape:?}"
                            )));
                        }
                        flat.extend(row_data);
                    }
                    inner_shape.insert(0, arr.len());
                    Ok((flat, inner_shape))
                }
                _ => {
                    // Leaf level: convert each value to Element
                    let elements = arr
                        .iter()
                        .enumerate()
                        .map(|(i, v)| {
                            json_value_to_element(v)
                                .ok_or_else(|| parse_err(format!("unsupported value at [{i}]")))
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok((elements, vec![arr.len()]))
                }
            }
        }
        other => {
            // Scalar at unexpected nesting level
            json_value_to_element(other)
                .map(|e| (vec![e], vec![]))
                .ok_or_else(|| parse_err("unsupported top-level JSON value type"))
        }
    }
}

/// Maps a single `serde_json::Value` to `Element`, returning `None` for
/// unsupported types (objects, etc.).
fn json_value_to_element(v: &Value) -> Option<Element> {
    match v {
        Value::Null => Some(Element::None),
        Value::Bool(b) => Some(Element::Bool(*b)),
        Value::String(s) => Some(Element::Text(std::sync::Arc::from(s.as_str()))),
        Value::Number(n) => {
            // Integer-looking number: try i64 first, fall back to f64.
            if let Some(i) = n.as_i64() {
                Some(Element::Int(i))
            } else {
                n.as_f64().map(Element::Float)
            }
        }
        _ => None, // objects, arrays — not leaf values
    }
}
