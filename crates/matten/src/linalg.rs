//! Linear-algebra-adjacent helpers (RFC-041).
//!
//! Core `matten` provides small linalg-adjacent helpers, not a linear algebra
//! backend. This module adds three:
//!
//! - [`Tensor::norm`] — the L2 / Frobenius norm over **all** elements.
//! - [`Tensor::trace`] — the diagonal sum of a rank-2 tensor (rectangular via
//!   `min(rows, cols)`).
//! - [`Tensor::outer`] — the rank-1 × rank-1 outer product.
//!
//! Serious linear algebra — `inverse`, `determinant`, `solve`, eigen-decomposition,
//! SVD, QR, LU, Cholesky, sparse formats, and BLAS/LAPACK backends — is
//! intentionally **out of scope** for core (RFC-041 §5). Use a specialized crate
//! such as `nalgebra` or `ndarray-linalg` for those. `matten` prioritizes PoC
//! ergonomics, not numerical linear algebra performance or stability leadership.

use crate::limits::MattenLimits;
use crate::{MattenError, Tensor};

impl Tensor {
    /// L2 / Frobenius norm over all elements: `sqrt(sum(x_i^2))`.
    ///
    /// Works at any rank — it reduces every element, so for a matrix this is the
    /// Frobenius norm. `NaN` propagates: any `NaN` element yields `NaN`. No special
    /// overflow-avoidance scaling is applied, so extreme magnitudes may overflow to
    /// infinity (use a specialized crate if that matters).
    ///
    /// For a non-panicking form, see [`Tensor::try_norm`].
    ///
    /// # Panics
    /// Panics if called on a dynamic tensor; call
    /// [`try_numeric`](crate::Tensor::try_numeric) first. Use [`Tensor::try_norm`]
    /// for the non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// // 3-4-5: sqrt(9 + 16) = 5
    /// assert_eq!(Tensor::from_vec(vec![3.0, 4.0]).norm(), 5.0);
    /// ```
    #[must_use]
    pub fn norm(&self) -> f64 {
        self.try_norm().unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::norm`].
    ///
    /// Use `try_norm` when handling a tensor that may be dynamic; `norm` is the
    /// panic-on-error convenience form.
    ///
    /// # Errors
    /// Returns [`MattenError::Unsupported`] on a dynamic tensor. `NaN` is treated
    /// as a value and propagates according to the underlying operation.
    ///
    /// ```
    /// use matten::Tensor;
    /// assert_eq!(Tensor::from_vec(vec![3.0, 4.0]).try_norm().unwrap(), 5.0);
    /// ```
    pub fn try_norm(&self) -> Result<f64, MattenError> {
        crate::math::reject_dynamic(self, "norm")?;
        Ok(self.data.iter().map(|x| x * x).sum::<f64>().sqrt())
    }

    /// Sum of the diagonal of a rank-2 tensor (the matrix trace).
    ///
    /// Rectangular matrices are allowed: the trace sums `self[i, i]` for
    /// `i in 0..min(rows, cols)`.
    ///
    /// # Panics
    /// Panics if the tensor is not rank-2, or if called on a dynamic tensor. Use
    /// [`Tensor::try_trace`] for the non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    /// assert_eq!(m.trace(), 5.0); // 1 + 4
    /// ```
    #[must_use]
    pub fn trace(&self) -> f64 {
        self.try_trace().unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::trace`].
    ///
    /// # Errors
    /// Returns [`MattenError::Shape`] if the tensor is not rank-2, or
    /// [`MattenError::Unsupported`] if called on a dynamic tensor.
    ///
    /// ```
    /// use matten::Tensor;
    /// // rectangular: min(2, 3) = 2 diagonal entries -> self[0,0] + self[1,1]
    /// let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    /// assert_eq!(m.try_trace().unwrap(), 6.0); // 1 + 5
    /// assert!(Tensor::from_vec(vec![1.0, 2.0]).try_trace().is_err()); // rank 1
    /// ```
    pub fn try_trace(&self) -> Result<f64, MattenError> {
        #[cfg(feature = "dynamic")]
        if self.is_dynamic() {
            return Err(MattenError::Unsupported {
                operation: "trace",
                message: "trace is not supported on dynamic tensors; call try_numeric() first"
                    .to_string(),
            });
        }
        if self.shape.len() != 2 {
            return Err(MattenError::Shape {
                operation: "trace",
                message: format!(
                    "trace requires a rank-2 tensor, got rank {}",
                    self.shape.len()
                ),
            });
        }
        let rows = self.shape[0];
        let cols = self.shape[1];
        let k = rows.min(cols);
        let mut acc = 0.0;
        for i in 0..k {
            acc += self.data[i * cols + i];
        }
        Ok(acc)
    }

    /// Outer product of two rank-1 tensors: `out[i, j] = self[i] * other[j]`.
    ///
    /// The output has shape `[self.len(), other.len()]`.
    ///
    /// # Panics
    /// Panics if either input is not rank-1, if either is a dynamic tensor, or if
    /// the result exceeds the allocation limit. Use [`Tensor::try_outer`] for the
    /// non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    /// let b = Tensor::from_vec(vec![4.0, 5.0]);
    /// let o = a.outer(&b);
    /// assert_eq!(o.shape(), &[3, 2]);
    /// assert_eq!(o.as_slice(), &[4.0, 5.0, 8.0, 10.0, 12.0, 15.0]);
    /// ```
    #[must_use]
    pub fn outer(&self, other: &Tensor) -> Tensor {
        self.try_outer(other).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::outer`].
    ///
    /// # Errors
    /// Returns [`MattenError::Shape`] if either input is not rank-1,
    /// [`MattenError::Unsupported`] if either is a dynamic tensor, or
    /// [`MattenError::Allocation`] if the result exceeds the allocation limit.
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::from_vec(vec![1.0, 2.0]);
    /// let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    /// assert!(a.try_outer(&m).is_err()); // rhs is rank 2
    /// ```
    pub fn try_outer(&self, other: &Tensor) -> Result<Tensor, MattenError> {
        #[cfg(feature = "dynamic")]
        if self.is_dynamic() || other.is_dynamic() {
            return Err(MattenError::Unsupported {
                operation: "outer",
                message:
                    "outer is not supported on dynamic tensors; call try_numeric() on each operand first"
                        .to_string(),
            });
        }
        if self.shape.len() != 1 || other.shape.len() != 1 {
            return Err(MattenError::Shape {
                operation: "outer",
                message: format!(
                    "outer requires two rank-1 tensors, got ranks {} and {}",
                    self.shape.len(),
                    other.shape.len()
                ),
            });
        }
        let m = self.shape[0];
        let n = other.shape[0];
        let out_shape = vec![m, n];
        let total = MattenLimits::default().check_shape(&out_shape, "outer")?;

        let mut data = Vec::with_capacity(total);
        for &a in &self.data {
            for &b in &other.data {
                data.push(a * b);
            }
        }
        Ok(Tensor {
            data,
            shape: out_shape,
            #[cfg(feature = "dynamic")]
            dynamic: None,
        })
    }
}
