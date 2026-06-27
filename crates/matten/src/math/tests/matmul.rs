use crate::Tensor;

// ── dot (vector) ──────────────────────────────────────────────────────────

#[test]
fn vv_dot_basic() {
    let a = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    let b = Tensor::from_vec(vec![4.0, 5.0, 6.0]);
    let d = a.dot(&b);
    assert!(d.is_scalar());
    assert_eq!(d.as_slice(), &[32.0]); // 1*4 + 2*5 + 3*6
}

#[test]
fn vv_dot_orthogonal() {
    let a = Tensor::from_vec(vec![1.0, 0.0, 0.0]);
    let b = Tensor::from_vec(vec![0.0, 1.0, 0.0]);
    assert_eq!(a.dot(&b).as_slice(), &[0.0]);
}

#[test]
#[should_panic(expected = "lengths must match")]
fn vv_dot_length_mismatch_panics() {
    let a = Tensor::from_vec(vec![1.0, 2.0]);
    let b = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    let _ = a.dot(&b);
}

// ── matmul ────────────────────────────────────────────────────────────────

#[test]
fn matrix_vector_mul() {
    // [[1,2,3],[4,5,6]] × [1,0,1] = [4,10]
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let v = Tensor::from_vec(vec![1.0, 0.0, 1.0]);
    let r = m.matmul(&v);
    assert_eq!(r.shape(), &[2]);
    assert_eq!(r.as_slice(), &[4.0, 10.0]);
}

#[test]
fn vector_matrix_mul() {
    // [1,2] × [[1,2,3],[4,5,6]] = [9,12,15]
    let v = Tensor::from_vec(vec![1.0, 2.0]);
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let r = v.matmul(&m);
    assert_eq!(r.shape(), &[3]);
    assert_eq!(r.as_slice(), &[9.0, 12.0, 15.0]);
}

#[test]
fn matrix_matrix_mul() {
    // [[1,2],[3,4]] × [[5,6],[7,8]] = [[19,22],[43,50]]
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let b = Tensor::new(vec![5.0, 6.0, 7.0, 8.0], &[2, 2]);
    let c = a.matmul(&b);
    assert_eq!(c.shape(), &[2, 2]);
    assert_eq!(c.as_slice(), &[19.0, 22.0, 43.0, 50.0]);
}

#[test]
fn matmul_non_square() {
    // [2,3] × [3,4] -> [2,4]
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let b = Tensor::new((1..=12).map(|x| x as f64).collect(), &[3, 4]);
    let c = a.matmul(&b);
    assert_eq!(c.shape(), &[2, 4]);
    // row 0: [1,2,3] × cols = [1*1+2*5+3*9, 1*2+2*6+3*10, …]
    assert_eq!(c.as_slice()[0], 38.0); // 1+10+27
    assert_eq!(c.as_slice()[1], 44.0); // 2+12+30
}

#[test]
fn dot_and_matmul_are_aliases() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let b = Tensor::new(vec![5.0, 6.0, 7.0, 8.0], &[2, 2]);
    assert_eq!(a.dot(&b), a.matmul(&b));
}

#[test]
#[should_panic(expected = "left columns")]
fn matmul_dimension_mismatch_panics() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let b = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0], &[4, 2]);
    let _ = a.matmul(&b);
}

#[test]
#[should_panic(expected = "unsupported rank")]
fn matmul_rank3_panics() {
    let a = Tensor::zeros(&[2, 2, 2]);
    let b = Tensor::zeros(&[2, 2, 2]);
    let _ = a.matmul(&b);
}

#[test]
fn star_is_still_element_wise_not_matmul() {
    // Regression: * must never become matmul
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let b = Tensor::new(vec![5.0, 6.0, 7.0, 8.0], &[2, 2]);
    assert_eq!((&a * &b).as_slice(), &[5.0, 12.0, 21.0, 32.0]); // element-wise
    assert_eq!(a.matmul(&b).as_slice(), &[19.0, 22.0, 43.0, 50.0]); // matrix product
}
