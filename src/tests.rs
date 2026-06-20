use crate::{DataFormat, MattenError, Tensor};

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

// ---- fill constructors (M2) -------------------------------------------

#[test]
fn zeros_fills_with_zero() {
    let t = Tensor::zeros(&[2, 3]);
    assert_eq!(t.shape(), &[2, 3]);
    assert!(t.as_slice().iter().all(|&v| v == 0.0));
}

#[test]
fn ones_fills_with_one() {
    let t = Tensor::ones(&[4]);
    assert_eq!(t.len(), 4);
    assert!(t.as_slice().iter().all(|&v| v == 1.0));
}

#[test]
fn full_fills_with_value() {
    let t = Tensor::full(&[3, 2], -5.0);
    assert_eq!(t.shape(), &[3, 2]);
    assert!(t.as_slice().iter().all(|&v| v == -5.0));
}

#[test]
fn from_vec_creates_1d() {
    let t = Tensor::from_vec(vec![10.0, 20.0, 30.0]);
    assert_eq!(t.shape(), &[3]);
    assert!(t.is_vector());
    assert_eq!(t.as_slice(), &[10.0, 20.0, 30.0]);
}

// ---- into_vec (M2) -----------------------------------------------------

#[test]
fn into_vec_consumes_tensor() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let v = t.into_vec();
    assert_eq!(v, vec![1.0, 2.0, 3.0, 4.0]);
}

// ---- arange (M2) -------------------------------------------------------

#[test]
fn arange_forward() {
    let t = Tensor::arange(0.0, 5.0, 1.0);
    assert_eq!(t.shape(), &[5]);
    assert_eq!(t.as_slice(), &[0.0, 1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn arange_backward() {
    let t = Tensor::arange(3.0, 0.0, -1.0);
    assert_eq!(t.as_slice(), &[3.0, 2.0, 1.0]);
}

#[test]
fn arange_fractional_step() {
    let t = Tensor::arange(0.0, 1.0, 0.5);
    assert_eq!(t.len(), 2);
    assert!((t.as_slice()[0] - 0.0).abs() < 1e-12);
    assert!((t.as_slice()[1] - 0.5).abs() < 1e-12);
}

#[test]
fn try_arange_zero_step_is_err() {
    assert!(matches!(
        Tensor::try_arange(0.0, 5.0, 0.0),
        Err(MattenError::Shape { .. })
    ));
}

#[test]
fn try_arange_nonfinite_step_is_err() {
    assert!(matches!(
        Tensor::try_arange(0.0, 5.0, f64::NAN),
        Err(MattenError::Shape { .. })
    ));
    assert!(matches!(
        Tensor::try_arange(0.0, 5.0, f64::INFINITY),
        Err(MattenError::Shape { .. })
    ));
}

#[test]
fn try_arange_nonfinite_bounds_is_err() {
    assert!(matches!(
        Tensor::try_arange(f64::NAN, 5.0, 1.0),
        Err(MattenError::Shape { .. })
    ));
    assert!(matches!(
        Tensor::try_arange(0.0, f64::INFINITY, 1.0),
        Err(MattenError::Shape { .. })
    ));
}

#[test]
fn try_arange_empty_range_is_err() {
    // step > 0 but start >= end produces no elements
    assert!(matches!(
        Tensor::try_arange(5.0, 3.0, 1.0),
        Err(MattenError::Shape { .. })
    ));
}

#[test]
#[should_panic(expected = "matten shape error")]
fn arange_panics_on_zero_step() {
    let _ = Tensor::arange(0.0, 5.0, 0.0);
}

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

// ---- error model -------------------------------------------------------

#[test]
fn error_display_and_matching() {
    let e = MattenError::Parse {
        format: DataFormat::Csv,
        message: "row 3, column 2".into(),
    };
    assert!(matches!(
        e,
        MattenError::Parse {
            format: DataFormat::Csv,
            ..
        }
    ));
    assert_eq!(e.to_string(), "matten csv parse error: row 3, column 2");
}

#[test]
fn data_format_is_copy_eq_display() {
    assert_eq!(DataFormat::Json, DataFormat::Json);
    assert_ne!(DataFormat::Json, DataFormat::Csv);
    assert_eq!(DataFormat::Json.to_string(), "json");
}

// ---- row-major index helpers (M1) -------------------------------------

#[test]
fn strides_are_row_major() {
    use crate::shape::strides_for_shape;
    assert_eq!(strides_for_shape(&[2, 3, 4]), vec![12, 4, 1]);
    assert_eq!(strides_for_shape(&[5]), vec![1]);
    assert_eq!(strides_for_shape(&[]), Vec::<usize>::new());
}

#[test]
fn coord_out_of_bounds_is_none() {
    use crate::shape::coord_to_flat;
    assert_eq!(coord_to_flat(&[2, 0], &[2, 3]), None);
    assert_eq!(coord_to_flat(&[0], &[2, 3]), None); // rank mismatch
}

#[test]
fn index_round_trip() {
    use crate::shape::{coord_to_flat, flat_to_coord};
    let shapes: &[&[usize]] = &[&[], &[1], &[5], &[2, 3], &[3, 1, 4], &[2, 2, 2, 2]];
    for &shp in shapes {
        let len: usize = shp.iter().product();
        for flat in 0..len {
            let coord = flat_to_coord(flat, shp);
            assert_eq!(coord.len(), shp.len());
            assert_eq!(
                coord_to_flat(&coord, shp),
                Some(flat),
                "shape {shp:?} flat {flat}"
            );
        }
    }
}

