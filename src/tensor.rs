//! The primary public container, [`Tensor`].
//!
//! M1 implements the Core Tensor Contract (RFC-003): construction with full
//! shape validation, scalar/vector/matrix semantics, and the observational API.
//! Arithmetic, reshape, transpose, slicing, and the data boundaries land in
//! later milestones.

use crate::error::MattenError;
use crate::shape;
use std::fmt;

/// A dense, row-major, owned multidimensional array of `f64`.
///
/// Fields are private: the storage layout is an implementation detail and is
/// never exposed across the public API. A scalar is shape `[]` with one element.
///
/// # Examples
///
/// ```
/// use matten::Tensor;
///
/// let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
/// assert_eq!(t.shape(), &[2, 2]);
/// assert!(t.is_matrix());
///
/// let s = Tensor::scalar(42.0);
/// assert!(s.is_scalar());
/// assert_eq!(s.len(), 1);
/// ```
#[derive(Clone, PartialEq)]
pub struct Tensor {
    pub(crate) data: Vec<f64>,
    pub(crate) shape: Vec<usize>,
}

#[allow(clippy::len_without_is_empty)]
impl Tensor {
    /// Creates a tensor from row-major `data` and `shape`.
    ///
    /// Panics with an actionable message on any shape violation or data-length
    /// mismatch. Use [`try_new`](Tensor::try_new) for recoverable construction.
    ///
    /// # Panics
    ///
    /// Panics if the shape is invalid or `data.len()` does not equal the shape
    /// product.
    #[must_use]
    pub fn new(data: Vec<f64>, shape: &[usize]) -> Tensor {
        Self::try_new(data, shape).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Creates a tensor from row-major `data` and `shape`, returning an error
    /// instead of panicking.
    ///
    /// # Errors
    ///
    /// Returns [`MattenError::Shape`] for a length mismatch or invalid shape,
    /// or [`MattenError::Allocation`] on product overflow.
    pub fn try_new(data: Vec<f64>, shape: &[usize]) -> Result<Tensor, MattenError> {
        let expected = shape::validate_shape(shape, "try_new")?;
        if data.len() != expected {
            return Err(MattenError::Shape {
                operation: "try_new",
                message: format!(
                    "data length {} does not match shape {shape:?}, which requires {expected} elements",
                    data.len()
                ),
            });
        }
        Ok(Tensor { data, shape: shape.to_vec() })
    }

    /// Creates a rank-0 scalar tensor (shape `[]`, length `1`).
    ///
    /// ```
    /// use matten::Tensor;
    /// let s = Tensor::scalar(3.14);
    /// assert!(s.is_scalar());
    /// assert_eq!(s.len(), 1);
    /// ```
    #[must_use]
    pub fn scalar(value: f64) -> Tensor {
        Tensor { data: vec![value], shape: Vec::new() }
    }

    /// The shape as a slice of dimension lengths. Non-allocating.
    #[must_use]
    pub fn shape(&self) -> &[usize] { &self.shape }

    /// The rank (number of dimensions): `shape().len()`.
    #[must_use]
    pub fn ndim(&self) -> usize { self.shape.len() }

    /// The logical element count: the product of the shape.
    #[must_use]
    pub fn len(&self) -> usize { self.data.len() }

    /// Returns `true` for a rank-0 scalar tensor (shape `[]`).
    #[must_use]
    pub fn is_scalar(&self) -> bool { self.ndim() == 0 }

    /// Returns `true` for a rank-1 tensor.
    #[must_use]
    pub fn is_vector(&self) -> bool { self.ndim() == 1 }

    /// Returns `true` for a rank-2 tensor.
    #[must_use]
    pub fn is_matrix(&self) -> bool { self.ndim() == 2 }

    /// The flat, row-major data as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[f64] { &self.data }

    /// Returns an owned copy of the flat, row-major data.
    #[must_use]
    pub fn to_vec(&self) -> Vec<f64> { self.data.clone() }
}

impl fmt::Debug for Tensor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const MAX: usize = 8;
        write!(f, "Tensor(shape={:?}, data=[", self.shape)?;
        for (i, v) in self.data.iter().take(MAX).enumerate() {
            if i > 0 { f.write_str(", ")?; }
            write!(f, "{v:?}")?;
        }
        if self.data.len() > MAX {
            write!(f, ", ... ({} more)", self.data.len() - MAX)?;
        }
        f.write_str("])")
    }
}
