use crate::Tensor;

// ---- broadcasting (M3) -------------------------------------------------

#[test]
fn broadcast_same_shape() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let b = Tensor::new(vec![10.0, 20.0, 30.0, 40.0], &[2, 2]);
    let c = &a + &b;
    assert_eq!(c.shape(), &[2, 2]);
    assert_eq!(c.as_slice(), &[11.0, 22.0, 33.0, 44.0]);
}

#[test]
fn broadcast_scalar_to_matrix() {
    let scalar = Tensor::scalar(2.0);
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let r = &scalar + &m;
    assert_eq!(r.shape(), &[2, 2]);
    assert_eq!(r.as_slice(), &[3.0, 4.0, 5.0, 6.0]);
}

#[test]
fn broadcast_vector_to_matrix() {
    // [4] + [3, 4] -> [3, 4]
    let row = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[4]);
    let mat = Tensor::new(
        vec![
            10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0, 110.0, 120.0,
        ],
        &[3, 4],
    );
    let r = &mat + &row;
    assert_eq!(r.shape(), &[3, 4]);
    assert_eq!(r.as_slice()[0], 11.0);
    assert_eq!(r.as_slice()[4], 51.0);
    assert_eq!(r.as_slice()[8], 91.0);
}

#[test]
fn broadcast_column_and_row() {
    // [3, 1] + [1, 4] -> [3, 4]
    let col = Tensor::new(vec![1.0, 2.0, 3.0], &[3, 1]);
    let row = Tensor::new(vec![10.0, 20.0, 30.0, 40.0], &[1, 4]);
    let r = &col + &row;
    assert_eq!(r.shape(), &[3, 4]);
    assert_eq!(
        r.as_slice(),
        &[
            11.0, 21.0, 31.0, 41.0, 12.0, 22.0, 32.0, 42.0, 13.0, 23.0, 33.0, 43.0
        ]
    );
}

#[test]
#[should_panic(expected = "matten broadcast error in add")]
fn broadcast_incompatible_panics() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let b = Tensor::new(vec![1.0, 2.0], &[2]);
    let _ = &a + &b;
}

#[test]
fn element_wise_sub_mul_div() {
    let a = Tensor::new(vec![10.0, 8.0, 6.0, 4.0], &[2, 2]);
    let b = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let sub = &a - &b;
    assert_eq!(sub.as_slice(), &[9.0, 6.0, 3.0, 0.0]);
    let mul = &a * &b;
    assert_eq!(mul.as_slice(), &[10.0, 16.0, 18.0, 16.0]);
    let div = &a / &b;
    assert_eq!(div.as_slice(), &[10.0, 4.0, 2.0, 1.0]);
}

#[test]
fn neg_unary() {
    let t = Tensor::new(vec![1.0, -2.0, 0.0], &[3]);
    let r = -&t;
    assert_eq!(r.as_slice(), &[-1.0, 2.0, 0.0]);
}

#[test]
fn division_by_zero_is_inf() {
    let a = Tensor::new(vec![1.0, 0.0], &[2]);
    let b = Tensor::new(vec![0.0, 0.0], &[2]);
    let r = &a / &b;
    assert!(r.as_slice()[0].is_infinite());
    assert!(r.as_slice()[1].is_nan());
}

// ---- scalar ops (M3) ---------------------------------------------------

#[test]
fn scalar_ops_tensor_on_left() {
    let t = Tensor::new(vec![2.0, 4.0, 6.0], &[3]);
    assert_eq!((&t + 1.0).as_slice(), &[3.0, 5.0, 7.0]);
    assert_eq!((&t - 1.0).as_slice(), &[1.0, 3.0, 5.0]);
    assert_eq!((&t * 2.0).as_slice(), &[4.0, 8.0, 12.0]);
    assert_eq!((&t / 2.0).as_slice(), &[1.0, 2.0, 3.0]);
}

#[test]
fn scalar_ops_scalar_on_left() {
    let t = Tensor::new(vec![1.0, 2.0, 4.0], &[3]);
    assert_eq!((10.0_f64 + &t).as_slice(), &[11.0, 12.0, 14.0]);
    assert_eq!((10.0_f64 - &t).as_slice(), &[9.0, 8.0, 6.0]);
    assert_eq!((3.0_f64 * &t).as_slice(), &[3.0, 6.0, 12.0]);
    assert_eq!((12.0_f64 / &t).as_slice(), &[12.0, 6.0, 3.0]);
}

#[test]
fn star_is_element_wise_not_matmul() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let b = Tensor::new(vec![5.0, 6.0, 7.0, 8.0], &[2, 2]);
    // element-wise, not matrix product
    let r = &a * &b;
    assert_eq!(r.as_slice(), &[5.0, 12.0, 21.0, 32.0]);
}

// ---- broadcast shape helper (internal) ---------------------------------

#[test]
fn broadcast_shape_cases() {
    use crate::ops::broadcast::broadcast_shape;
    assert_eq!(broadcast_shape(&[], &[2, 3]).unwrap(), vec![2, 3]);
    assert_eq!(broadcast_shape(&[4], &[3, 4]).unwrap(), vec![3, 4]);
    assert_eq!(broadcast_shape(&[3, 1], &[1, 4]).unwrap(), vec![3, 4]);
    assert_eq!(broadcast_shape(&[2, 3], &[2, 3]).unwrap(), vec![2, 3]);
    assert!(broadcast_shape(&[2, 3], &[2]).is_err()); // incompatible
    assert!(broadcast_shape(&[3], &[4]).is_err());
}
