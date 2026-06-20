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

#[test]
fn fill_constructors_are_public() {
    let z = Tensor::zeros(&[3, 3]);
    let o = Tensor::ones(&[3, 3]);
    let f = Tensor::full(&[3, 3], 2.0);
    assert_eq!(z.len(), 9);
    assert_eq!(o.len(), 9);
    assert_eq!(f.len(), 9);
    assert!(o.as_slice().iter().all(|&v| v == 1.0));
}

#[test]
fn from_and_into_roundtrip() {
    let original = vec![1.0_f64, 2.0, 3.0, 4.0];
    let t: Tensor = original.clone().into();
    let back: Vec<f64> = t.into();
    assert_eq!(back, original);
}

#[test]
fn nested_row_construction() {
    let t: Tensor = vec![vec![1.0, 2.0], vec![3.0, 4.0]].into();
    assert_eq!(t.shape(), &[2, 2]);
    let rows: Vec<Vec<f64>> = t.try_into().unwrap();
    assert_eq!(rows[0], &[1.0, 2.0]);
    assert_eq!(rows[1], &[3.0, 4.0]);
}

#[test]
fn arange_basic() {
    let t = Tensor::arange(0.0, 3.0, 1.0);
    assert_eq!(t.as_slice(), &[0.0, 1.0, 2.0]);
}

#[test]
fn try_arange_bad_step() {
    assert!(Tensor::try_arange(0.0, 5.0, 0.0).is_err());
}

// ---- M3: operators and broadcasting ------------------------------------

#[test]
fn element_wise_ops_public() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let b = Tensor::ones(&[2, 2]);
    let c = &a + &b;
    assert_eq!(c.as_slice(), &[2.0, 3.0, 4.0, 5.0]);
    let d = &a * 2.0;
    assert_eq!(d.as_slice(), &[2.0, 4.0, 6.0, 8.0]);
    let e = 0.0 - &a;
    assert_eq!(e.as_slice(), &[-1.0, -2.0, -3.0, -4.0]);
}

#[test]
fn broadcasting_feels_like_numpy() {
    // bias addition: [2, 3] + [3] -> [2, 3]
    let matrix = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let bias = Tensor::new(vec![10.0, 20.0, 30.0], &[3]);
    let result = &matrix + &bias;
    assert_eq!(result.shape(), &[2, 3]);
    assert_eq!(result.as_slice(), &[11.0, 22.0, 33.0, 14.0, 25.0, 36.0]);
}

// ---- M5: boundary integration ------------------------------------------

#[cfg(feature = "json")]
#[test]
fn json_roundtrip_smoke() {
    #[allow(unused_imports)]
    use serde_json;
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let json = serde_json::to_string(&t).unwrap();
    let t2: Tensor = serde_json::from_str(&json).unwrap();
    assert_eq!(t, t2);
}

#[cfg(feature = "json")]
#[test]
fn from_json_nested_smoke() {
    let t = Tensor::from_json("[[1.0,2.0],[3.0,4.0]]").unwrap();
    assert_eq!(t.shape(), &[2, 2]);
}

#[cfg(feature = "csv")]
#[test]
fn from_csv_smoke() {
    let t = Tensor::from_csv("1.0,2.0,3.0\n4.0,5.0,6.0\n").unwrap();
    assert_eq!(t.shape(), &[2, 3]);
}
