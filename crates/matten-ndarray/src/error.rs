//! Error type for `matten-ndarray` (RFC-025 §7, RFC-027 §5).
//!
//! The bridge defines its own error type rather than growing core
//! [`matten::MattenError`] (RFC-022 §8). Conversions return `Result`; a dynamic
//! tensor returns [`MattenNdarrayError::DynamicTensor`] rather than panicking.

use std::fmt;

/// Errors produced by the `matten` ↔ `ndarray` conversions.
///
/// `#[non_exhaustive]` so future variants are not a breaking change.
#[derive(Debug)]
#[non_exhaustive]
pub enum MattenNdarrayError {
    /// A dynamic (`Element`) tensor was passed to a conversion. Convert it to a
    /// numeric tensor first with `Tensor::try_numeric()`.
    DynamicTensor,
    /// The input `ndarray` shape contained a zero-length axis, which core
    /// `matten` does not support.
    ZeroSizedAxis(Vec<usize>),
    /// `ndarray` could not construct the target array (e.g. a shape/length
    /// mismatch).
    NdarrayShape(ndarray::ShapeError),
    /// Core `matten` rejected the conversion (e.g. the rank exceeds `MAX_NDIM`).
    Matten(matten::MattenError),
}

impl fmt::Display for MattenNdarrayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MattenNdarrayError::DynamicTensor => write!(
                f,
                "matten-ndarray error: dynamic tensors cannot be converted; call \
                 try_numeric() to convert to a numeric tensor first"
            ),
            MattenNdarrayError::ZeroSizedAxis(shape) => write!(
                f,
                "matten-ndarray error: ndarray shape {shape:?} contains a zero-length \
                 axis, which matten does not support"
            ),
            MattenNdarrayError::NdarrayShape(e) => {
                write!(
                    f,
                    "matten-ndarray error: ndarray could not build the array: {e}"
                )
            }
            MattenNdarrayError::Matten(e) => {
                write!(
                    f,
                    "matten-ndarray error: matten rejected the conversion: {e}"
                )
            }
        }
    }
}

impl std::error::Error for MattenNdarrayError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MattenNdarrayError::NdarrayShape(e) => Some(e),
            MattenNdarrayError::Matten(e) => Some(e),
            _ => None,
        }
    }
}
