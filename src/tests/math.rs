use crate::Tensor;

// ── whole reductions ──────────────────────────────────────────────────────

#[test]
fn sum_basic() {
    assert_eq!(Tensor::from_vec(vec![1.0, 2.0, 3.0]).sum(), 6.0);
    assert_eq!(Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]).sum(), 10.0);
    assert_eq!(Tensor::scalar(7.0).sum(), 7.0);
}

#[test]
fn sum_nan_propagates() {
    assert!(Tensor::from_vec(vec![1.0, f64::NAN, 3.0]).sum().is_nan());
}

#[test]
fn mean_basic() {
    assert_eq!(Tensor::from_vec(vec![1.0, 2.0, 3.0, 4.0]).mean(), 2.5);
    assert_eq!(Tensor::scalar(5.0).mean(), 5.0);
}

#[test]
fn mean_nan_propagates() {
    assert!(Tensor::from_vec(vec![1.0, f64::NAN]).mean().is_nan());
}

#[test]
fn min_basic() {
    assert_eq!(Tensor::from_vec(vec![3.0, 1.0, 2.0]).min(), 1.0);
    assert_eq!(Tensor::new(vec![5.0, -2.0, 3.0, 0.0], &[2, 2]).min(), -2.0);
}

#[test]
fn max_basic() {
    assert_eq!(Tensor::from_vec(vec![3.0, 1.0, 2.0]).max(), 3.0);
    assert_eq!(Tensor::new(vec![5.0, -2.0, 3.0, 0.0], &[2, 2]).max(), 5.0);
}

#[test]
fn min_nan_returns_nan() {
    // Must return NaN when any element is NaN (not silently ignore it)
    assert!(Tensor::from_vec(vec![1.0, f64::NAN, 3.0]).min().is_nan());
    assert!(Tensor::from_vec(vec![f64::NAN, 99.0]).min().is_nan());
}

#[test]
fn max_nan_returns_nan() {
    assert!(Tensor::from_vec(vec![1.0, f64::NAN, 3.0]).max().is_nan());
}

#[test]
fn min_with_inf() {
    assert_eq!(
        Tensor::from_vec(vec![1.0, f64::NEG_INFINITY, 3.0]).min(),
        f64::NEG_INFINITY
    );
}

#[test]
fn max_with_inf() {
    assert_eq!(
        Tensor::from_vec(vec![1.0, f64::INFINITY, 3.0]).max(),
        f64::INFINITY
    );
}

// ── axis reductions ───────────────────────────────────────────────────────

#[test]
fn sum_axis_0_on_matrix() {
    // [[1,2,3],[4,5,6]] -> [5,7,9]
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let r = m.sum_axis(0);
    assert_eq!(r.shape(), &[3]);
    assert_eq!(r.as_slice(), &[5.0, 7.0, 9.0]);
}

#[test]
fn sum_axis_1_on_matrix() {
    // [[1,2,3],[4,5,6]] -> [6,15]
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let r = m.sum_axis(1);
    assert_eq!(r.shape(), &[2]);
    assert_eq!(r.as_slice(), &[6.0, 15.0]);
}

#[test]
fn sum_axis_on_vector_gives_scalar() {
    let v = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    let r = v.sum_axis(0);
    assert!(r.is_scalar());
    assert_eq!(r.as_slice(), &[6.0]);
}

#[test]
fn mean_axis_0_on_matrix() {
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let r = m.mean_axis(0);
    assert_eq!(r.shape(), &[3]);
    assert_eq!(r.as_slice(), &[2.5, 3.5, 4.5]);
}

#[test]
fn mean_axis_1_on_matrix() {
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let r = m.mean_axis(1);
    assert_eq!(r.shape(), &[2]);
    assert_eq!(r.as_slice(), &[2.0, 5.0]);
}

#[test]
fn sum_axis_rank3() {
    // shape [2,3,4] summed along axis 1 -> [2,4]
    let data: Vec<f64> = (0..24).map(|x| x as f64).collect();
    let t = Tensor::new(data, &[2, 3, 4]);
    let r = t.sum_axis(1);
    assert_eq!(r.shape(), &[2, 4]);
    // row 0: sum of rows 0..3 of first batch = [0+4+8, 1+5+9, 2+6+10, 3+7+11]
    assert_eq!(r.as_slice()[0], 12.0);
    assert_eq!(r.as_slice()[1], 15.0);
}

#[test]
#[should_panic(expected = "out of range")]
fn sum_axis_out_of_range_panics() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let _ = t.sum_axis(5);
}

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

// ── min_axis / max_axis ---------------------------------------------------

#[test]
fn min_axis_0_on_matrix() {
    let m = Tensor::new(vec![3.0, 1.0, 4.0, 1.0, 5.0, 9.0], &[2, 3]);
    let r = m.min_axis(0);
    assert_eq!(r.shape(), &[3]);
    assert_eq!(r.as_slice(), &[1.0, 1.0, 4.0]);
}

#[test]
fn max_axis_0_on_matrix() {
    let m = Tensor::new(vec![3.0, 1.0, 4.0, 1.0, 5.0, 9.0], &[2, 3]);
    let r = m.max_axis(0);
    assert_eq!(r.shape(), &[3]);
    assert_eq!(r.as_slice(), &[3.0, 5.0, 9.0]);
}

#[test]
fn min_axis_nan_propagates() {
    let m = Tensor::new(vec![1.0, f64::NAN, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let r = m.min_axis(0);
    assert!(r.as_slice()[1].is_nan()); // NaN in column 1
    assert_eq!(r.as_slice()[0], 1.0);
    assert_eq!(r.as_slice()[2], 3.0);
}

#[test]
fn max_axis_on_vector_gives_scalar() {
    let v = Tensor::from_vec(vec![2.0, 7.0, 4.0]);
    let r = v.max_axis(0);
    assert!(r.is_scalar());
    assert_eq!(r.as_slice(), &[7.0]);
}

#[test]
#[should_panic(expected = "out of range")]
fn min_axis_out_of_range_panics() {
    let t = Tensor::ones(&[3]);
    let _ = t.min_axis(5);
}
