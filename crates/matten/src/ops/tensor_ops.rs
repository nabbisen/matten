//! Element-wise binary operators for borrowed tensor pairs (RFC-006).
//!
//! `*` is element-wise multiplication; matrix multiplication is explicit and
//! arrives in RFC-010 / M6.

use crate::Tensor;
use crate::ops::broadcast::apply_binary;
use std::ops::{Add, Div, Mul, Sub};

impl Add for &Tensor {
    type Output = Tensor;
    /// Element-wise addition with NumPy-style broadcasting.
    ///
    /// # Panics
    ///
    /// Panics with `"matten broadcast error in add: ..."` if the shapes are
    /// incompatible.
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    /// let b = Tensor::ones(&[2, 2]);
    /// let c = &a + &b;
    /// assert_eq!(c.as_slice(), &[2.0, 3.0, 4.0, 5.0]);
    /// ```
    fn add(self, rhs: &Tensor) -> Tensor {
        apply_binary(self, rhs, "add", |a, b| a + b)
    }
}

impl Sub for &Tensor {
    type Output = Tensor;
    /// Element-wise subtraction with broadcasting.
    ///
    /// # Panics
    ///
    /// Panics on incompatible shapes.
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::new(vec![5.0, 4.0, 3.0, 2.0], &[2, 2]);
    /// let b = Tensor::ones(&[2, 2]);
    /// let c = &a - &b;
    /// assert_eq!(c.as_slice(), &[4.0, 3.0, 2.0, 1.0]);
    /// ```
    fn sub(self, rhs: &Tensor) -> Tensor {
        apply_binary(self, rhs, "sub", |a, b| a - b)
    }
}

impl Mul for &Tensor {
    type Output = Tensor;
    /// Element-wise multiplication with broadcasting (`*` is **not** matrix
    /// multiply; use `matmul` for that).
    ///
    /// # Panics
    ///
    /// Panics on incompatible shapes.
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    /// let b = Tensor::full(&[2, 2], 2.0);
    /// let c = &a * &b;
    /// assert_eq!(c.as_slice(), &[2.0, 4.0, 6.0, 8.0]);
    /// ```
    fn mul(self, rhs: &Tensor) -> Tensor {
        apply_binary(self, rhs, "mul", |a, b| a * b)
    }
}

impl Div for &Tensor {
    type Output = Tensor;
    /// Element-wise division with broadcasting. Division by zero follows IEEE 754
    /// `f64` behavior (yields `inf`, `-inf`, or `NaN`); no error is produced.
    ///
    /// # Panics
    ///
    /// Panics on incompatible shapes.
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::new(vec![4.0, 9.0], &[2]);
    /// let b = Tensor::new(vec![2.0, 3.0], &[2]);
    /// let c = &a / &b;
    /// assert_eq!(c.as_slice(), &[2.0, 3.0]);
    /// ```
    fn div(self, rhs: &Tensor) -> Tensor {
        apply_binary(self, rhs, "div", |a, b| a / b)
    }
}
