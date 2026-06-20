use crate::{DataFormat, MattenError, Tensor};

#[test]
fn constructs_and_inspects() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    assert_eq!(t.shape(), &[2, 2]);
    assert_eq!(t.len(), 4);
    assert_eq!(t.ndim(), 2);
    assert_eq!(t.as_slice(), &[1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn scalar_shape_has_len_one() {
    let s = Tensor::new(vec![42.0], &[]);
    assert!(s.shape().is_empty());
    assert_eq!(s.len(), 1);
    assert_eq!(s.ndim(), 0);
}

#[test]
fn try_new_rejects_length_mismatch() {
    let err = Tensor::try_new(vec![1.0, 2.0, 3.0], &[2, 2]).unwrap_err();
    assert!(matches!(err, MattenError::Shape { .. }));
}

#[test]
#[should_panic(expected = "matten shape error")]
fn new_panics_on_mismatch() {
    let _ = Tensor::new(vec![1.0], &[2, 2]);
}

#[test]
fn try_new_rejects_shape_overflow() {
    let err = Tensor::try_new(vec![], &[usize::MAX, usize::MAX]).unwrap_err();
    assert!(matches!(err, MattenError::Allocation { .. }));
}

#[test]
fn error_display_and_matching() {
    let e = MattenError::Parse {
        format: DataFormat::Csv,
        message: "row 3, column 2".into(),
    };
    assert!(matches!(
        e,
        MattenError::Parse {
            format: DataFormat::Csv,
            ..
        }
    ));
    assert_eq!(e.to_string(), "matten csv parse error: row 3, column 2");
}

#[test]
fn data_format_is_copy_eq_display() {
    assert_eq!(DataFormat::Json, DataFormat::Json);
    assert_ne!(DataFormat::Json, DataFormat::Csv);
    assert_eq!(DataFormat::Json.to_string(), "json");
}

#[test]
fn debug_is_shape_first() {
    let t = Tensor::new(vec![1.0, 2.0], &[2]);
    assert_eq!(format!("{t:?}"), "Tensor(shape=[2], data=[1.0, 2.0])");
}
