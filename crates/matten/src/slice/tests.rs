use crate::{MattenError, Tensor};

// ---- SliceBuilder -------------------------------------------------------

#[test]
fn builder_all_all() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let s = t.slice().all().all().build().unwrap();
    assert_eq!(s, t);
}

#[test]
fn builder_index_first_row() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let row = t.slice().index(0).all().build().unwrap();
    assert_eq!(row.shape(), &[3]);
    assert_eq!(row.as_slice(), &[1.0, 2.0, 3.0]);
}

#[test]
fn builder_index_second_row() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let row = t.slice().index(1).all().build().unwrap();
    assert_eq!(row.shape(), &[3]);
    assert_eq!(row.as_slice(), &[4.0, 5.0, 6.0]);
}

#[test]
fn builder_range_rows() {
    // [3, 4] tensor; take rows 0..2 (all rows), all cols
    let t = Tensor::new((1..=12).map(|x| x as f64).collect(), &[3, 4]);
    let s = t.slice().range(0..2).all().build().unwrap();
    assert_eq!(s.shape(), &[2, 4]);
    assert_eq!(s.as_slice(), &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
}

#[test]
fn builder_range_cols() {
    let t = Tensor::new((1..=6).map(|x| x as f64).collect(), &[2, 3]);
    let s = t.slice().all().range(1..3).build().unwrap();
    assert_eq!(s.shape(), &[2, 2]);
    assert_eq!(s.as_slice(), &[2.0, 3.0, 5.0, 6.0]);
}

#[test]
fn builder_range_from() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[4]);
    let s = t.slice().range(2..).build().unwrap();
    assert_eq!(s.as_slice(), &[3.0, 4.0]);
}

#[test]
fn builder_range_to() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[4]);
    let s = t.slice().range(..2).build().unwrap();
    assert_eq!(s.as_slice(), &[1.0, 2.0]);
}

#[test]
fn builder_range_full() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[4]);
    let s = t.slice().range(..).build().unwrap();
    assert_eq!(s, t);
}

#[test]
fn builder_inclusive_range() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[4]);
    let s = t.slice().range(1..=2).build().unwrap();
    assert_eq!(s.as_slice(), &[2.0, 3.0]);
}

#[test]
fn builder_index_all_axes_gives_scalar() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let s = t.slice().index(1).index(0).build().unwrap();
    assert!(s.is_scalar());
    assert_eq!(s.as_slice(), &[3.0]);
}

#[test]
fn builder_rank_mismatch_is_err() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    // Only one spec for a rank-2 tensor
    let err = t.slice().all().build().unwrap_err();
    assert!(matches!(err, MattenError::Slice { .. }));
}

#[test]
fn builder_out_of_bounds_index_is_err() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let err = t.slice().index(5).all().build().unwrap_err();
    assert!(matches!(err, MattenError::Slice { .. }));
}

#[test]
fn builder_result_is_independent() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let s = t.slice().index(0).all().build().unwrap();
    // The original tensor is unchanged
    assert_eq!(t.len(), 6);
    assert_eq!(s.len(), 3);
}

// ---- slice_str ----------------------------------------------------------

#[test]
fn slice_str_all() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let s = t.slice_str(":, :").unwrap();
    assert_eq!(s, t);
}

#[test]
fn slice_str_first_row() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let s = t.slice_str("0, :").unwrap();
    assert_eq!(s.shape(), &[3]);
    assert_eq!(s.as_slice(), &[1.0, 2.0, 3.0]);
}

#[test]
fn slice_str_range() {
    let t = Tensor::new((1..=12).map(|x| x as f64).collect(), &[3, 4]);
    let s = t.slice_str("0:2, :").unwrap();
    assert_eq!(s.shape(), &[2, 4]);
}

#[test]
fn slice_str_range_from() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[4]);
    let s = t.slice_str("2:").unwrap();
    assert_eq!(s.as_slice(), &[3.0, 4.0]);
}

#[test]
fn slice_str_range_to() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[4]);
    let s = t.slice_str(":2").unwrap();
    assert_eq!(s.as_slice(), &[1.0, 2.0]);
}

#[test]
fn slice_str_step() {
    let t = Tensor::new((0..=9).map(|x| x as f64).collect(), &[10]);
    let s = t.slice_str("0:10:2").unwrap();
    assert_eq!(s.as_slice(), &[0.0, 2.0, 4.0, 6.0, 8.0]);
}

#[test]
fn slice_str_whitespace_ignored() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let a = t.slice_str("0,:").unwrap();
    let b = t.slice_str(" 0 , : ").unwrap();
    assert_eq!(a, b);
}

#[test]
fn slice_str_matches_builder() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let from_str = t.slice_str("0:2, :").unwrap();
    let from_builder = t.slice().range(0..2).all().build().unwrap();
    assert_eq!(from_str, from_builder);
}

#[test]
fn slice_str_malformed_is_err() {
    // All of these must return Err (never panic, never silently accept)
    // "0::" was previously accepted as "0:" — now rejected (trailing colon)
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    for bad in &["0::", "a:b", ":::", "", "x"] {
        assert!(
            t.slice_str(bad).is_err(),
            "expected Err for {:?} but got Ok",
            bad
        );
    }
}

#[test]
fn slice_str_too_many_dims_is_err() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let err = t.slice_str("0, 0, 0").unwrap_err();
    assert!(matches!(err, MattenError::Slice { .. }));
}

#[test]
fn slice_str_oversized_is_err() {
    let t = Tensor::new(vec![1.0, 2.0], &[2]);
    let long = "0:1, ".repeat(200);
    let err = t.slice_str(&long).unwrap_err();
    assert!(matches!(err, MattenError::Slice { .. }));
    assert!(err.to_string().contains("maximum length"));
}
