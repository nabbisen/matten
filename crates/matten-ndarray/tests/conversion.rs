//! Conversion tests validating RFC-027 §7 (design specifications, not code).

use matten::Tensor;
use matten_ndarray::{MattenNdarrayError, from_arrayd, to_arrayd};
use ndarray::{ArrayD, IxDyn};

fn arr(shape: &[usize], data: Vec<f64>) -> ArrayD<f64> {
    ArrayD::from_shape_vec(IxDyn(shape), data).unwrap()
}

#[test]
fn roundtrip_scalar() {
    let t = Tensor::scalar(42.0);
    let a = to_arrayd(&t).unwrap();
    assert_eq!(a.shape(), &[] as &[usize]);
    let back = from_arrayd(a).unwrap();
    assert!(back.is_scalar());
    assert_eq!(back.as_slice(), &[42.0]);
}

#[test]
fn roundtrip_vector() {
    let t = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    let back = from_arrayd(to_arrayd(&t).unwrap()).unwrap();
    assert_eq!(back.shape(), &[3]);
    assert_eq!(back.as_slice(), &[1.0, 2.0, 3.0]);
}

#[test]
fn roundtrip_matrix_preserves_row_major_order() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let a = to_arrayd(&t).unwrap();
    assert_eq!(a[[0, 0]], 1.0);
    assert_eq!(a[[1, 0]], 4.0);
    let back = from_arrayd(a).unwrap();
    assert_eq!(back.shape(), &[2, 3]);
    assert_eq!(back.as_slice(), &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
}

#[test]
fn roundtrip_nd() {
    let data: Vec<f64> = (0..24).map(|x| x as f64).collect();
    let t = Tensor::new(data.clone(), &[2, 3, 4]);
    let back = from_arrayd(to_arrayd(&t).unwrap()).unwrap();
    assert_eq!(back.shape(), &[2, 3, 4]);
    assert_eq!(back.as_slice(), &data[..]);
}

#[test]
fn from_arrayd_transposed_preserves_logical_order() {
    // Non-standard layout: transpose of [[1,2,3],[4,5,6]] is logically
    // [[1,4],[2,5],[3,6]] -> row-major [1,4,2,5,3,6].
    let a = arr(&[2, 3], vec![1., 2., 3., 4., 5., 6.]);
    let t = from_arrayd(a.t().to_owned()).unwrap();
    assert_eq!(t.shape(), &[3, 2]);
    assert_eq!(t.as_slice(), &[1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
}

#[test]
fn from_arrayd_sliced_noncontiguous_preserves_logical_order() {
    // Slice every other column of a 2x4 -> logical 2x2 with non-standard strides.
    let a = arr(&[2, 4], vec![1., 2., 3., 4., 5., 6., 7., 8.]);
    let sliced = a.slice(ndarray::s![.., ..;2]).to_owned().into_dyn(); // cols 0 and 2
    let t = from_arrayd(sliced).unwrap();
    assert_eq!(t.shape(), &[2, 2]);
    assert_eq!(t.as_slice(), &[1.0, 3.0, 5.0, 7.0]);
}

#[test]
fn from_arrayd_zero_axis_is_rejected() {
    let a = ArrayD::<f64>::zeros(IxDyn(&[2, 0, 3]));
    let err = from_arrayd(a).unwrap_err();
    assert!(matches!(err, MattenNdarrayError::ZeroSizedAxis(_)));
    assert!(err.to_string().contains("zero-length axis"));
}

#[test]
fn from_arrayd_rank_too_high_maps_to_matten_error() {
    // Core matten caps rank at MAX_NDIM = 8; a rank-9 array must be rejected.
    let a = ArrayD::<f64>::zeros(IxDyn(&[1, 1, 1, 1, 1, 1, 1, 1, 1]));
    let err = from_arrayd(a).unwrap_err();
    assert!(matches!(err, MattenNdarrayError::Matten(_)));
}

#[test]
fn error_is_std_error_with_source() {
    use std::error::Error;
    let a = ArrayD::<f64>::zeros(IxDyn(&[1; 9]));
    let err = from_arrayd(a).unwrap_err();
    // The Matten variant exposes a source.
    assert!(err.source().is_some());
}

#[cfg(feature = "dynamic")]
#[test]
fn dynamic_tensor_is_rejected_not_panicked() {
    use matten::Element;
    let t = Tensor::from_elements(
        vec![Element::Float(1.0), Element::None, Element::Int(3)],
        &[3],
    );
    let err = to_arrayd(&t).unwrap_err();
    assert!(matches!(err, MattenNdarrayError::DynamicTensor));
}

#[cfg(feature = "dynamic")]
#[test]
fn numeric_tensor_under_dynamic_feature_still_converts() {
    // With the dynamic feature on, a plain numeric tensor must still convert.
    let t = Tensor::new(vec![1.0, 2.0], &[2]);
    let a = to_arrayd(&t).unwrap();
    assert_eq!(a.shape(), &[2]);
}

// ── v0.19 hardening: reliability of roundtrips and edge values (RFC-029 §2.3) ──

#[test]
fn roundtrip_rank4() {
    let data: Vec<f64> = (0..16).map(|x| x as f64 * 0.5).collect();
    let t = Tensor::new(data.clone(), &[2, 2, 2, 2]);
    let back = from_arrayd(to_arrayd(&t).unwrap()).unwrap();
    assert_eq!(back.shape(), &[2, 2, 2, 2]);
    assert_eq!(back.as_slice(), &data[..]);
}

#[test]
fn from_arrayd_3d_permuted_axes_preserves_logical_order() {
    // [2,3,4] permuted to axis order (2,0,1) -> logical [4,2,3], non-standard layout.
    let data: Vec<f64> = (0..24).map(|x| x as f64).collect();
    let a = arr(&[2, 3, 4], data);
    let permuted = a.clone().permuted_axes(vec![2, 0, 1]); // view, non-standard layout
    let t = from_arrayd(permuted.to_owned()).unwrap();
    assert_eq!(t.shape(), &[4, 2, 3]);
    // Compare against ndarray's own logical iteration as ground truth.
    let expected: Vec<f64> = a.permuted_axes(vec![2, 0, 1]).iter().copied().collect();
    assert_eq!(t.as_slice(), &expected[..]);
}

#[test]
fn nan_and_inf_pass_through_both_directions() {
    let t = Tensor::new(vec![1.0, f64::NAN, f64::INFINITY, -2.0], &[2, 2]);
    let a = to_arrayd(&t).unwrap();
    assert!(a[[0, 1]].is_nan());
    assert!(a[[1, 0]].is_infinite());
    let back = from_arrayd(a).unwrap();
    assert!(back.as_slice()[1].is_nan());
    assert!(back.as_slice()[2].is_infinite());
    assert_eq!(back.as_slice()[0], 1.0);
}

#[test]
fn fractional_value_fidelity() {
    let data = vec![0.1, 0.2, 0.3, 0.123_456_789, -9.87654321, 1e-12];
    let t = Tensor::new(data.clone(), &[2, 3]);
    let back = from_arrayd(to_arrayd(&t).unwrap()).unwrap();
    assert_eq!(back.as_slice(), &data[..]); // exact bit-for-bit, no rounding
}

#[test]
fn to_arrayd_is_standard_layout() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let a = to_arrayd(&t).unwrap();
    assert!(a.is_standard_layout());
}
