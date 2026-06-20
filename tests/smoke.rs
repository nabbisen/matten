//! Integration smoke tests: exercise the public crate surface as a user would.

use matten::{MattenError, Tensor};

#[test]
fn public_construction_and_inspection() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    assert_eq!(t.shape(), &[2, 3]);
    assert_eq!(t.len(), 6);
    assert_eq!(t.ndim(), 2);
}

#[test]
fn boundary_construction_is_recoverable() {
    let result = Tensor::try_new(vec![1.0, 2.0], &[3]);
    assert!(matches!(result, Err(MattenError::Shape { .. })));
}
