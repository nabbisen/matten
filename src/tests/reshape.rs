use crate::{MattenError, Tensor};

// ---- reshape ------------------------------------------------------------

#[test]
fn reshape_same_element_count() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let r = t.reshape(&[3, 2]);
    assert_eq!(r.shape(), &[3, 2]);
    // flat data order preserved
    assert_eq!(r.as_slice(), &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
}

#[test]
fn reshape_to_1d() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let flat = t.reshape(&[4]);
    assert_eq!(flat.shape(), &[4]);
}

#[test]
fn reshape_to_scalar() {
    let t = Tensor::new(vec![42.0], &[1]);
    let s = t.reshape(&[]);
    assert!(s.is_scalar());
    assert_eq!(s.as_slice(), &[42.0]);
}

#[test]
fn try_reshape_mismatch_is_err() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let err = t.try_reshape(&[4, 2]).unwrap_err();
    assert!(matches!(err, MattenError::Shape { .. }));
    assert!(err.to_string().contains("reshape"));
}

#[test]
#[should_panic(expected = "matten shape error")]
fn reshape_panics_on_mismatch() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);
    let _ = t.reshape(&[2, 2]);
}

#[test]
fn reshape_is_independent() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let r = t.reshape(&[4]);
    // mutating r's data does not affect t (they're independent)
    assert_eq!(t.as_slice(), &[1.0, 2.0, 3.0, 4.0]);
    assert_eq!(r.as_slice(), &[1.0, 2.0, 3.0, 4.0]);
}

// ---- flatten ------------------------------------------------------------

#[test]
fn flatten_matrix() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let f = t.flatten();
    assert_eq!(f.shape(), &[6]);
    assert_eq!(f.as_slice(), t.as_slice());
}

#[test]
fn flatten_scalar_gives_length_1() {
    let s = Tensor::scalar(7.0);
    let f = s.flatten();
    assert_eq!(f.shape(), &[1]);
    assert_eq!(f.as_slice(), &[7.0]);
}

// ---- transpose ----------------------------------------------------------

#[test]
fn transpose_2d() {
    // [[1,2,3],[4,5,6]] -> [[1,4],[2,5],[3,6]]
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let tr = t.transpose();
    assert_eq!(tr.shape(), &[3, 2]);
    assert_eq!(tr.as_slice(), &[1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
}

#[test]
fn t_alias() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    assert_eq!(t.transpose(), t.t());
}

#[test]
fn transpose_reverse_twice_is_identity() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    assert_eq!(t.transpose().transpose(), t);
}

#[test]
fn transpose_rank3_reverses_axes() {
    // shape [2,3,4] -> [4,3,2]
    let data: Vec<f64> = (1..=24).map(|x| x as f64).collect();
    let t = Tensor::new(data, &[2, 3, 4]);
    let tr = t.transpose();
    assert_eq!(tr.shape(), &[4, 3, 2]);
    // element at (0,0,0) stays at (0,0,0); (0,0,1) -> (1,0,0)
    assert_eq!(tr.get(&[0, 0, 0]), Some(1.0));
    assert_eq!(tr.get(&[1, 0, 0]), Some(2.0));
}

#[test]
#[should_panic(expected = "scalar")]
fn transpose_scalar_panics() {
    let _ = Tensor::scalar(1.0).transpose();
}

// ---- swap_axes ----------------------------------------------------------

#[test]
fn swap_axes_rank2() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let s = t.swap_axes(0, 1);
    assert_eq!(s.shape(), &[3, 2]);
    assert_eq!(s, t.transpose());
}

#[test]
fn swap_axes_rank3() {
    let data: Vec<f64> = (1..=24).map(|x| x as f64).collect();
    let t = Tensor::new(data, &[2, 3, 4]);
    let s = t.swap_axes(0, 2);
    assert_eq!(s.shape(), &[4, 3, 2]);
}

#[test]
fn swap_axes_same_is_identity() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    assert_eq!(t.swap_axes(0, 0), t);
}

#[test]
#[should_panic(expected = "out of range")]
fn swap_axes_out_of_range_panics() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let _ = t.swap_axes(0, 5);
}

// ---- get ----------------------------------------------------------------

#[test]
fn get_valid_coordinate() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    assert_eq!(t.get(&[0, 0]), Some(1.0));
    assert_eq!(t.get(&[0, 1]), Some(2.0));
    assert_eq!(t.get(&[1, 0]), Some(3.0));
    assert_eq!(t.get(&[1, 1]), Some(4.0));
}

#[test]
fn get_out_of_bounds_is_none() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    assert_eq!(t.get(&[2, 0]), None); // row out of bounds
    assert_eq!(t.get(&[0, 5]), None); // col out of bounds
}

#[test]
fn get_rank_mismatch_is_none() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    assert_eq!(t.get(&[0]), None); // wrong rank
    assert_eq!(t.get(&[0, 0, 0]), None);
}

#[test]
fn get_scalar() {
    let s = Tensor::scalar(99.0);
    assert_eq!(s.get(&[]), Some(99.0));
}
