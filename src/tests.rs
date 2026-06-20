use crate::{MattenError, Tensor};

// ---- construction & inspection (M1) -------------------------------------

#[test]
fn constructs_and_inspects() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    assert_eq!(t.shape(), &[2, 2]);
    assert_eq!(t.len(), 4);
    assert_eq!(t.ndim(), 2);
    assert_eq!(t.as_slice(), &[1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn scalar_constructor() {
    let s = Tensor::scalar(42.0);
    assert!(s.shape().is_empty());
    assert_eq!(s.len(), 1);
    assert_eq!(s.ndim(), 0);
    assert!(s.is_scalar());
    assert_eq!(s.as_slice(), &[42.0]);
}

#[test]
fn shape_predicates() {
    assert!(Tensor::scalar(1.0).is_scalar());
    assert!(Tensor::new(vec![1.0, 2.0], &[2]).is_vector());
    assert!(Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]).is_matrix());
    let t3 = Tensor::new(vec![0.0; 8], &[2, 2, 2]);
    assert!(!t3.is_scalar());
    assert!(!t3.is_vector());
    assert!(!t3.is_matrix());
}

#[test]
fn to_vec_returns_owned_copy() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);
    assert_eq!(t.to_vec(), vec![1.0, 2.0, 3.0]);
    assert_eq!(t.as_slice(), &[1.0, 2.0, 3.0]);
}

#[test]
fn clone_and_partial_eq() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let b = a.clone();
    assert_eq!(a, b);
    assert_ne!(a, Tensor::new(vec![1.0, 2.0, 3.0, 5.0], &[2, 2]));
    assert_ne!(a, Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[4]));
}

#[test]
fn debug_is_shape_first() {
    let t = Tensor::new(vec![1.0, 2.0], &[2]);
    assert_eq!(format!("{t:?}"), "Tensor(shape=[2], data=[1.0, 2.0])");
}

// ---- shape validation (M1) ---------------------------------------------

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
fn rejects_zero_dim() {
    assert!(matches!(
        Tensor::try_new(vec![], &[0]),
        Err(MattenError::Shape { .. })
    ));
    assert!(matches!(
        Tensor::try_new(vec![], &[2, 0]),
        Err(MattenError::Shape { .. })
    ));
}

#[test]
#[should_panic(expected = "zero-sized dimensions")]
fn new_panics_on_zero_dim() {
    let _ = Tensor::new(vec![], &[0]);
}

#[test]
fn rejects_rank_over_limit() {
    let shape = [1usize; 9]; // rank 9 > MAX_NDIM(8)
    let err = Tensor::try_new(vec![1.0], &shape).unwrap_err();
    assert!(matches!(err, MattenError::Shape { .. }));
}

#[test]
fn accepts_rank_8() {
    let shape = [1usize; 8];
    let t = Tensor::new(vec![1.0], &shape);
    assert_eq!(t.ndim(), 8);
    assert_eq!(t.len(), 1);
}

