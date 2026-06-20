use crate::{MattenError, Tensor};

// ---- nested-row construction (M2) -------------------------------------

#[test]
fn try_from_rows_success() {
    let t = Tensor::try_from_rows(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
    assert_eq!(t.shape(), &[2, 2]);
    assert_eq!(t.as_slice(), &[1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn try_from_rows_rejects_ragged() {
    let err = Tensor::try_from_rows(vec![vec![1.0, 2.0], vec![3.0]]).unwrap_err();
    assert!(matches!(err, MattenError::Shape { .. }));
    assert!(err.to_string().contains("ragged"));
}

#[test]
fn try_from_rows_rejects_empty() {
    let err = Tensor::try_from_rows(vec![]).unwrap_err();
    assert!(matches!(err, MattenError::Shape { .. }));
}

#[test]
fn from_vec_vec_panics_on_ragged() {
    let result = std::panic::catch_unwind(|| {
        let _ = Tensor::from(vec![vec![1.0, 2.0], vec![3.0]]);
    });
    assert!(result.is_err());
}

// ---- From / TryFrom traits (M2) ---------------------------------------

#[test]
fn from_vec_f64_trait() {
    let t: Tensor = vec![1.0_f64, 2.0, 3.0].into();
    assert_eq!(t.shape(), &[3]);
}

#[test]
fn from_tensor_for_vec_f64() {
    let t = Tensor::new(vec![5.0, 6.0], &[2]);
    let v: Vec<f64> = t.into();
    assert_eq!(v, vec![5.0, 6.0]);
}

#[test]
fn from_ref_tensor_for_vec_f64() {
    let t = Tensor::new(vec![5.0, 6.0], &[2]);
    let v: Vec<f64> = Vec::from(&t);
    assert_eq!(v, vec![5.0, 6.0]);
    assert_eq!(t.len(), 2); // t still valid
}

#[test]
fn try_from_tensor_for_nested_vec() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let rows: Vec<Vec<f64>> = t.try_into().unwrap();
    assert_eq!(rows, vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
}

#[test]
fn try_from_tensor_non_2d_is_err() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);
    let result: Result<Vec<Vec<f64>>, _> = t.try_into();
    assert!(matches!(result, Err(MattenError::Shape { .. })));
}
