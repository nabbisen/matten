//! Index-returning reductions on numeric tensors (RFC-038).
//!
//! `argmin`/`argmax` return the **flat, row-major** index of the smallest/largest
//! element, with the **first occurrence** winning ties. Unlike value reductions
//! (`min`/`max`), an index is ill-defined when any element is `NaN`, so these follow
//! the selection branch of the RFC-038 NaN policy: the `try_*` forms return
//! [`MattenError::InvalidArgument`] and the convenience forms panic with the same
//! context. On a dynamic tensor the `try_*` forms return
//! [`MattenError::Unsupported`] and the convenience forms panic; call
//! [`Tensor::try_numeric`](crate::Tensor::try_numeric) first.

use crate::{MattenError, Tensor};

/// Flat index of the extreme element, first occurrence on ties.
///
/// Returns [`MattenError::InvalidArgument`] if any element is `NaN`. Callers
/// guarantee `data` is non-empty (core rejects zero-sized dimensions).
fn arg_extreme(
    data: &[f64],
    operation: &'static str,
    want_min: bool,
) -> Result<usize, MattenError> {
    if data.iter().any(|v| v.is_nan()) {
        return Err(MattenError::InvalidArgument {
            operation,
            argument: "self",
            message: format!("{operation} is undefined for tensors containing NaN"),
        });
    }
    let mut best = 0;
    let mut best_val = data[0];
    for (i, &v) in data.iter().enumerate().skip(1) {
        let better = if want_min { v < best_val } else { v > best_val };
        if better {
            best = i;
            best_val = v;
        }
    }
    Ok(best)
}

impl Tensor {
    /// Flat (row-major) index of the smallest element; first occurrence on ties.
    ///
    /// # Panics
    /// Panics if any element is `NaN` (the index would be ill-defined), or if called
    /// on a dynamic tensor. Use [`Tensor::try_argmin`] for the non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::new(vec![3.0, 1.0, 5.0, 1.0], &[2, 2]);
    /// assert_eq!(t.argmin(), 1); // first of the two 1.0s
    /// ```
    #[must_use]
    pub fn argmin(&self) -> usize {
        self.try_argmin().unwrap_or_else(|e| panic!("{e}"))
    }

    /// Flat (row-major) index of the largest element; first occurrence on ties.
    ///
    /// # Panics
    /// Panics if any element is `NaN`, or if called on a dynamic tensor. Use
    /// [`Tensor::try_argmax`] for the non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::from_vec(vec![3.0, 1.0, 5.0, 5.0]);
    /// assert_eq!(t.argmax(), 2); // first of the two 5.0s
    /// ```
    #[must_use]
    pub fn argmax(&self) -> usize {
        self.try_argmax().unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::argmin`].
    ///
    /// Returns [`MattenError::InvalidArgument`] if any element is `NaN`, or
    /// [`MattenError::Unsupported`] if called on a dynamic tensor.
    ///
    /// ```
    /// use matten::Tensor;
    /// assert_eq!(Tensor::from_vec(vec![3.0, 1.0, 2.0]).try_argmin().unwrap(), 1);
    /// assert!(Tensor::from_vec(vec![1.0, f64::NAN]).try_argmin().is_err());
    /// ```
    pub fn try_argmin(&self) -> Result<usize, MattenError> {
        #[cfg(feature = "dynamic")]
        if self.is_dynamic() {
            return Err(MattenError::Unsupported {
                operation: "argmin",
                message: "argmin is not supported on dynamic tensors; call try_numeric() first"
                    .to_string(),
            });
        }
        arg_extreme(&self.data, "argmin", true)
    }

    /// Non-panicking [`Tensor::argmax`].
    ///
    /// Returns [`MattenError::InvalidArgument`] if any element is `NaN`, or
    /// [`MattenError::Unsupported`] if called on a dynamic tensor.
    ///
    /// ```
    /// use matten::Tensor;
    /// assert_eq!(Tensor::from_vec(vec![3.0, 1.0, 2.0]).try_argmax().unwrap(), 0);
    /// assert!(Tensor::from_vec(vec![1.0, f64::NAN]).try_argmax().is_err());
    /// ```
    pub fn try_argmax(&self) -> Result<usize, MattenError> {
        #[cfg(feature = "dynamic")]
        if self.is_dynamic() {
            return Err(MattenError::Unsupported {
                operation: "argmax",
                message: "argmax is not supported on dynamic tensors; call try_numeric() first"
                    .to_string(),
            });
        }
        arg_extreme(&self.data, "argmax", false)
    }
}

#[cfg(test)]
mod tests;
