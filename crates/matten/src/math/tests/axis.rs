use crate::{MattenError, Tensor};

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

// ── Result-form axis reductions (RFC-056) ─────────────────────────────────

#[test]
fn try_axis_reductions_match_panic_forms() {
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    for axis in 0..2 {
        let s = m.try_sum_axis(axis).unwrap();
        assert_eq!(s.shape(), m.sum_axis(axis).shape());
        assert_eq!(s.as_slice(), m.sum_axis(axis).as_slice());
        assert_eq!(
            m.try_mean_axis(axis).unwrap().as_slice(),
            m.mean_axis(axis).as_slice()
        );
        assert_eq!(
            m.try_min_axis(axis).unwrap().as_slice(),
            m.min_axis(axis).as_slice()
        );
        assert_eq!(
            m.try_max_axis(axis).unwrap().as_slice(),
            m.max_axis(axis).as_slice()
        );
    }
}

#[test]
fn try_axis_reductions_reject_out_of_range_axis() {
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    // axis == rank and axis > rank both error; operation is the conceptual op.
    assert!(matches!(
        m.try_sum_axis(2).unwrap_err(),
        MattenError::Shape {
            operation: "sum_axis",
            ..
        }
    ));
    assert!(matches!(
        m.try_mean_axis(9).unwrap_err(),
        MattenError::Shape {
            operation: "mean_axis",
            ..
        }
    ));
    assert!(matches!(
        m.try_min_axis(2).unwrap_err(),
        MattenError::Shape {
            operation: "min_axis",
            ..
        }
    ));
    assert!(matches!(
        m.try_max_axis(5).unwrap_err(),
        MattenError::Shape {
            operation: "max_axis",
            ..
        }
    ));
}

#[test]
#[should_panic(expected = "out of range")]
fn sum_axis_out_of_range_still_panics() {
    let _ = Tensor::ones(&[3]).sum_axis(5);
}

// ── rank-1 axis reductions collapse to a scalar (RFC-056, deep-review P3) ──

#[test]
fn try_axis_reductions_on_vector_give_scalar() {
    // A rank-1 reduce along axis 0 collapses to a scalar output, matching the
    // panic form (both go through the same reduction path).
    let v = Tensor::from_vec(vec![2.0, 7.0, 4.0]);
    let cases = [
        (v.try_sum_axis(0).unwrap(), v.sum_axis(0)),
        (v.try_mean_axis(0).unwrap(), v.mean_axis(0)),
        (v.try_min_axis(0).unwrap(), v.min_axis(0)),
        (v.try_max_axis(0).unwrap(), v.max_axis(0)),
    ];
    for (got, want) in cases {
        assert!(got.is_scalar());
        assert_eq!(got.shape(), want.shape());
        assert_eq!(got.as_slice(), want.as_slice());
    }
}
