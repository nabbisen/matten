use crate::{MattenError, Tensor};

// ---- serde round-trip ---------------------------------------------------

#[cfg(feature = "json")]
#[test]
fn serde_canonical_roundtrip() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let json = serde_json::to_string(&t).unwrap();
    assert!(json.contains("\"shape\""));
    assert!(json.contains("\"data\""));
    let t2: Tensor = serde_json::from_str(&json).unwrap();
    assert_eq!(t, t2);
}

#[cfg(feature = "json")]
#[test]
fn serde_canonical_scalar() {
    let s = Tensor::scalar(42.0);
    let json = serde_json::to_string(&s).unwrap();
    let s2: Tensor = serde_json::from_str(&json).unwrap();
    assert_eq!(s, s2);
}

#[cfg(feature = "json")]
#[test]
fn serde_deserialize_rejects_shape_mismatch() {
    let bad = r#"{"shape":[2,2],"data":[1.0,2.0,3.0]}"#;
    let err: Result<Tensor, _> = serde_json::from_str(bad);
    assert!(err.is_err());
}

// ---- from_json object form ----------------------------------------------

#[cfg(feature = "json")]
#[test]
fn from_json_object_form() {
    let t = Tensor::from_json(r#"{"shape":[2,2],"data":[1.0,2.0,3.0,4.0]}"#).unwrap();
    assert_eq!(t.shape(), &[2, 2]);
    assert_eq!(t.as_slice(), &[1.0, 2.0, 3.0, 4.0]);
}

#[cfg(feature = "json")]
#[test]
fn from_json_object_1d() {
    let t = Tensor::from_json(r#"{"shape":[3],"data":[1.0,2.0,3.0]}"#).unwrap();
    assert!(t.is_vector());
    assert_eq!(t.len(), 3);
}

#[cfg(feature = "json")]
#[test]
fn from_json_object_scalar() {
    let t = Tensor::from_json(r#"{"shape":[],"data":[99.0]}"#).unwrap();
    assert!(t.is_scalar());
    assert_eq!(t.as_slice(), &[99.0]);
}

#[cfg(feature = "json")]
#[test]
fn from_json_object_missing_field_is_err() {
    assert!(matches!(
        Tensor::from_json(r#"{"shape":[2]}"#),
        Err(MattenError::Parse { .. })
    ));
    assert!(matches!(
        Tensor::from_json(r#"{"data":[1.0]}"#),
        Err(MattenError::Parse { .. })
    ));
}

#[cfg(feature = "json")]
#[test]
fn from_json_object_shape_mismatch_is_err() {
    let err = Tensor::from_json(r#"{"shape":[2,2],"data":[1.0,2.0,3.0]}"#).unwrap_err();
    assert!(matches!(err, MattenError::Parse { .. }));
}

// ---- from_json nested-array form ----------------------------------------

#[cfg(feature = "json")]
#[test]
fn from_json_nested_2d() {
    let t = Tensor::from_json("[[1.0,2.0],[3.0,4.0]]").unwrap();
    assert_eq!(t.shape(), &[2, 2]);
    assert_eq!(t.as_slice(), &[1.0, 2.0, 3.0, 4.0]);
}

#[cfg(feature = "json")]
#[test]
fn from_json_nested_1d() {
    let t = Tensor::from_json("[1.0,2.0,3.0]").unwrap();
    assert_eq!(t.shape(), &[3]);
}

#[cfg(feature = "json")]
#[test]
fn from_json_ragged_is_err() {
    let err = Tensor::from_json("[[1.0,2.0],[3.0]]").unwrap_err();
    assert!(matches!(err, MattenError::Parse { .. }));
    assert!(err.to_string().contains("ragged"));
}

#[cfg(feature = "json")]
#[test]
fn from_json_non_numeric_is_err() {
    assert!(matches!(
        Tensor::from_json(r#"[[1.0,"hello"]]"#),
        Err(MattenError::Parse { .. })
    ));
    assert!(matches!(
        Tensor::from_json(r#"[[1.0,null]]"#),
        Err(MattenError::Parse { .. })
    ));
    assert!(matches!(
        Tensor::from_json(r#"[[1.0,true]]"#),
        Err(MattenError::Parse { .. })
    ));
}

#[cfg(feature = "json")]
#[test]
fn from_json_malformed_never_panics() {
    for bad in &["{", "null", "\"string\"", "", "[][]", "[[[[]]]]]"] {
        let _ = Tensor::from_json(bad); // must not panic
    }
}

// ---- from_csv -----------------------------------------------------------

#[cfg(feature = "csv")]
#[test]
fn from_csv_basic() {
    let t = Tensor::from_csv("1.0,2.0\n3.0,4.0\n").unwrap();
    assert_eq!(t.shape(), &[2, 2]);
    assert_eq!(t.as_slice(), &[1.0, 2.0, 3.0, 4.0]);
}

#[cfg(feature = "csv")]
#[test]
fn from_csv_1_row() {
    let t = Tensor::from_csv("1.0,2.0,3.0\n").unwrap();
    assert_eq!(t.shape(), &[1, 3]);
}

#[cfg(feature = "csv")]
#[test]
fn from_csv_whitespace_fields() {
    let t = Tensor::from_csv(" 1.0 , 2.0 \n 3.0 , 4.0 \n").unwrap();
    assert_eq!(t.as_slice(), &[1.0, 2.0, 3.0, 4.0]);
}

#[cfg(feature = "csv")]
#[test]
fn from_csv_ragged_is_err() {
    let err = Tensor::from_csv("1.0,2.0\n3.0\n").unwrap_err();
    assert!(matches!(err, MattenError::Parse { .. }));
}

#[cfg(feature = "csv")]
#[test]
fn from_csv_non_numeric_is_err() {
    let err = Tensor::from_csv("1.0,active\n3.0,4.0\n").unwrap_err();
    assert!(matches!(err, MattenError::Parse { .. }));
    assert!(err.to_string().contains("column"));
}

#[cfg(feature = "csv")]
#[test]
fn from_csv_empty_is_err() {
    assert!(matches!(
        Tensor::from_csv(""),
        Err(MattenError::Parse { .. })
    ));
}

// ---- load_json / load_csv via fixture files -----------------------------

#[cfg(feature = "json")]
#[test]
fn load_json_missing_file_is_io_err() {
    let err = Tensor::load_json("/nonexistent/path/tensor.json").unwrap_err();
    assert!(matches!(err, MattenError::Io { .. }));
}

#[cfg(feature = "csv")]
#[test]
fn load_csv_missing_file_is_io_err() {
    let err = Tensor::load_csv("/nonexistent/path/data.csv").unwrap_err();
    assert!(matches!(err, MattenError::Io { .. }));
}

#[cfg(feature = "json")]
#[test]
fn load_json_fixture() {
    let t = Tensor::load_json("examples/data/tensor_2x2.json").unwrap();
    assert_eq!(t.shape(), &[2, 2]);
    assert_eq!(t.as_slice(), &[1.0, 2.0, 3.0, 4.0]);
}

#[cfg(feature = "csv")]
#[test]
fn load_csv_fixture() {
    let t = Tensor::load_csv("examples/data/numeric_2x3.csv").unwrap();
    assert_eq!(t.shape(), &[2, 3]);
    assert_eq!(t.as_slice(), &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
}
