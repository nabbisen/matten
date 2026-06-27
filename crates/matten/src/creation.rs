//! Creation comfort constructors for numeric tensors (RFC-038).
//!
//! `linspace` and `eye` are small, familiar builders. Like the other
//! limit-enforcing constructors they validate through [`MattenLimits`]: a
//! zero-sized result (`count == 0`, `n == 0`) is rejected with
//! [`MattenError::Shape`], and an oversized result with
//! [`MattenError::Allocation`]. The convenience forms panic on those errors; the
//! `try_*` forms return them.

use crate::limits::MattenLimits;
use crate::{MattenError, Tensor};

impl Tensor {
    /// Creates a 1-D tensor of `count` evenly spaced values from `start` to `end`
    /// (inclusive of both endpoints when `count >= 2`).
    ///
    /// `count == 1` returns `[start]`. Element values follow ordinary `f64`
    /// behavior, so non-finite bounds propagate.
    ///
    /// # Panics
    /// Panics if `count == 0` (a zero-sized dimension) or the result exceeds the
    /// allocation limit. Use [`Tensor::try_linspace`] for the non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// assert_eq!(Tensor::linspace(0.0, 1.0, 5).as_slice(), &[0.0, 0.25, 0.5, 0.75, 1.0]);
    /// assert_eq!(Tensor::linspace(2.0, 9.0, 1).as_slice(), &[2.0]);
    /// ```
    #[must_use]
    pub fn linspace(start: f64, end: f64, count: usize) -> Tensor {
        Tensor::try_linspace(start, end, count).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::linspace`].
    ///
    /// # Errors
    /// Returns [`MattenError::Shape`] if `count == 0`, or
    /// [`MattenError::Allocation`] if the result exceeds the element limit.
    ///
    /// ```
    /// use matten::Tensor;
    /// assert!(Tensor::try_linspace(0.0, 1.0, 0).is_err());
    /// ```
    pub fn try_linspace(start: f64, end: f64, count: usize) -> Result<Tensor, MattenError> {
        // Rejects count == 0 (zero-sized dim) and enforces the element limit.
        let len = MattenLimits::default().check_shape(&[count], "try_linspace")?;
        let data: Vec<f64> = if len == 1 {
            vec![start]
        } else {
            let step = (end - start) / (len - 1) as f64;
            let mut v: Vec<f64> = (0..len).map(|i| start + step * i as f64).collect();
            // Pin the final value exactly to `end` (avoid floating-point drift).
            v[len - 1] = end;
            v
        };
        Ok(Tensor {
            data,
            shape: vec![len],
            #[cfg(feature = "dynamic")]
            dynamic: None,
        })
    }

    /// Creates an `n × n` identity matrix (1.0 on the diagonal, 0.0 elsewhere).
    ///
    /// # Panics
    /// Panics if `n == 0` (a zero-sized dimension) or the result exceeds the
    /// allocation limit. Use [`Tensor::try_eye`] for the non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// let i = Tensor::eye(3);
    /// assert_eq!(i.shape(), &[3, 3]);
    /// assert_eq!(i.as_slice(), &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
    /// ```
    #[must_use]
    pub fn eye(n: usize) -> Tensor {
        Tensor::try_eye(n).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::eye`].
    ///
    /// # Errors
    /// Returns [`MattenError::Shape`] if `n == 0`, or [`MattenError::Allocation`]
    /// if the result exceeds the element limit.
    ///
    /// ```
    /// use matten::Tensor;
    /// assert!(Tensor::try_eye(0).is_err());
    /// ```
    pub fn try_eye(n: usize) -> Result<Tensor, MattenError> {
        // Rejects n == 0 (zero-sized dim) and enforces the element limit on n*n.
        let len = MattenLimits::default().check_shape(&[n, n], "try_eye")?;
        let mut data = vec![0.0f64; len];
        for i in 0..n {
            data[i * n + i] = 1.0;
        }
        Ok(Tensor {
            data,
            shape: vec![n, n],
            #[cfg(feature = "dynamic")]
            dynamic: None,
        })
    }
}

#[cfg(test)]
mod tests;
