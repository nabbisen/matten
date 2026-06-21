//! `matten::Tensor` ↔ `ndarray::ArrayD<f64>` conversions (RFC-027 §4).
//!
//! Both directions copy: `matten::Tensor` owns a contiguous row-major
//! `Vec<f64>`, and these functions hand over / materialize owned buffers. No
//! zero-copy is claimed (RFC-025 §3).

use crate::error::MattenNdarrayError;
use matten::Tensor;
use ndarray::{ArrayD, IxDyn};

/// Converts a numeric [`Tensor`] into an [`ndarray::ArrayD<f64>`].
///
/// The result is standard (row-major) layout. A dynamic tensor (under the
/// `dynamic` feature) returns [`MattenNdarrayError::DynamicTensor`] rather than
/// panicking.
///
/// ```
/// use matten::Tensor;
/// use matten_ndarray::to_arrayd;
///
/// let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
/// let arr = to_arrayd(&t).unwrap();
/// assert_eq!(arr.shape(), &[2, 2]);
/// assert_eq!(arr[[1, 0]], 3.0);
/// ```
pub fn to_arrayd(tensor: &Tensor) -> Result<ArrayD<f64>, MattenNdarrayError> {
    #[cfg(feature = "dynamic")]
    if tensor.is_dynamic() {
        return Err(MattenNdarrayError::DynamicTensor);
    }

    let shape = tensor.shape().to_vec();
    // `to_vec` is row-major; safe here because dynamic tensors were rejected above.
    let data = tensor.to_vec();
    ArrayD::from_shape_vec(IxDyn(&shape), data).map_err(MattenNdarrayError::NdarrayShape)
}

/// Converts an [`ndarray::ArrayD<f64>`] into a [`Tensor`].
///
/// Conversion preserves **logical** element order: an `ArrayD` may be in
/// non-standard (transposed / sliced / non-standard-stride) layout, so the raw
/// backing buffer is not read directly — that would silently transpose the
/// data. A shape with any zero-length axis is rejected, because core `matten`
/// does not support zero-sized dimensions.
///
/// ```
/// use matten_ndarray::from_arrayd;
/// use ndarray::{ArrayD, IxDyn};
///
/// // A transposed (non-standard-layout) array still converts by logical order.
/// let a = ArrayD::from_shape_vec(IxDyn(&[2, 3]), vec![1., 2., 3., 4., 5., 6.]).unwrap();
/// let t = from_arrayd(a.t().to_owned()).unwrap(); // logical shape [3, 2]
/// assert_eq!(t.shape(), &[3, 2]);
/// assert_eq!(t.as_slice(), &[1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
/// ```
pub fn from_arrayd(array: ArrayD<f64>) -> Result<Tensor, MattenNdarrayError> {
    let shape: Vec<usize> = array.shape().to_vec();

    if shape.iter().any(|&dim| dim == 0) {
        return Err(MattenNdarrayError::ZeroSizedAxis(shape));
    }

    // `as_standard_layout` yields a row-major view (cloning only if the input
    // was non-standard layout); iterating it gives logical order.
    let data: Vec<f64> = array.as_standard_layout().iter().copied().collect();

    Tensor::try_new(data, &shape).map_err(MattenNdarrayError::Matten)
}
