//! The primary public container, [`Tensor`].
//!
//! M0 provides only the minimal surface needed for a compiling skeleton and the
//! smoke example: construction (`new` / `try_new`) and shape inspection. The
//! full Core Tensor Contract — scalar/vector/matrix predicates, `to_vec`,
//! reshape, transpose, arithmetic, broadcasting, slicing, and the data
//! boundaries — lands in M1 and later milestones (see the RFC pack).

use crate::error::MattenError;
use std::fmt;

/// A dense, row-major, owned multidimensional array of `f64`.
///
/// Fields are private: the storage layout is an implementation detail and is
/// never exposed across the public API.
///
/// # Examples
///
/// ```
/// use matten::Tensor;
///
/// let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
/// assert_eq!(t.shape(), &[2, 2]);
/// assert_eq!(t.len(), 4);
/// assert_eq!(t.ndim(), 2);
/// ```
pub struct Tensor {
    data: Vec<f64>,
    shape: Vec<usize>,
}

// `is_empty` is intentionally absent for 0.1.0 (zero-sized dims are rejected and
// a scalar has `len() == 1`, so it would always be false). It is deferred to a
// future zero-sized-tensor RFC; until then we silence clippy's paired-method lint.
#[allow(clippy::len_without_is_empty)]
impl Tensor {
    /// Creates a tensor from row-major `data` and `shape`.
    ///
    /// This is a panic-zone convenience constructor: it panics with an
    /// actionable message if `data.len()` does not equal the product of
    /// `shape`. Use [`Tensor::try_new`] for recoverable, boundary-safe
    /// construction.
    ///
    /// # Panics
    ///
    /// Panics if the data length does not match the shape product, or if the
    /// shape product overflows.
    #[must_use]
    pub fn new(data: Vec<f64>, shape: &[usize]) -> Tensor {
        Self::build(data, shape, "new").unwrap_or_else(|e| panic!("{e}"))
    }

    /// Creates a tensor from row-major `data` and `shape`, returning an error
    /// instead of panicking on mismatch or overflow.
    ///
    /// # Errors
    ///
    /// Returns [`MattenError::Shape`] if the data length does not match the
    /// shape product, or [`MattenError::Allocation`] if the shape product
    /// overflows `usize`.
    pub fn try_new(data: Vec<f64>, shape: &[usize]) -> Result<Tensor, MattenError> {
        Self::build(data, shape, "try_new")
    }

    fn build(
        data: Vec<f64>,
        shape: &[usize],
        operation: &'static str,
    ) -> Result<Tensor, MattenError> {
        let expected = checked_product(shape, operation)?;
        if data.len() != expected {
            return Err(MattenError::Shape {
                operation,
                message: format!(
                    "data length {} does not match shape {shape:?}, which requires {expected} elements",
                    data.len()
                ),
            });
        }
        Ok(Tensor {
            data,
            shape: shape.to_vec(),
        })
    }

    /// The shape as a slice of dimension lengths. Cheap and non-allocating.
    #[must_use]
    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    /// The rank (number of dimensions): `shape().len()`.
    #[must_use]
    pub fn ndim(&self) -> usize {
        self.shape.len()
    }

    /// The logical element count: the product of the shape.
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// The flat, row-major data as a slice. Valid because Phase 1 storage is
    /// contiguous and owned.
    #[must_use]
    pub fn as_slice(&self) -> &[f64] {
        &self.data
    }
}

impl fmt::Debug for Tensor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const MAX: usize = 8;
        write!(f, "Tensor(shape={:?}, data=[", self.shape)?;
        for (i, v) in self.data.iter().take(MAX).enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            write!(f, "{v:?}")?;
        }
        if self.data.len() > MAX {
            write!(f, ", ... ({} more)", self.data.len() - MAX)?;
        }
        f.write_str("])")
    }
}

/// Computes the product of a shape with checked arithmetic, mapping overflow to
/// [`MattenError::Allocation`]. Shared by all shape-validating constructors.
fn checked_product(shape: &[usize], operation: &'static str) -> Result<usize, MattenError> {
    let mut acc: usize = 1;
    for &dim in shape {
        acc = acc.checked_mul(dim).ok_or_else(|| MattenError::Allocation {
            requested_elements: usize::MAX,
            message: format!("shape {shape:?} overflows usize when computing the element count in {operation}"),
        })?;
    }
    Ok(acc)
}
