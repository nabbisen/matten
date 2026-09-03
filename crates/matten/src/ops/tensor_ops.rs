//! Element-wise binary operators for borrowed tensor pairs (RFC-006), and
//! their Result-zone twins (RFC-129).
//!
//! `*` is element-wise multiplication; matrix multiplication is explicit and
//! arrives in RFC-010 / M6.

use crate::MattenError;
use crate::Tensor;
use crate::ops::broadcast::{panic_for_arithmetic, try_apply_binary};
use std::ops::{Add, Div, Mul, Sub};

impl Tensor {
    /// Element-wise addition with NumPy-style broadcasting, returning
    /// [`MattenError`] instead of panicking.
    ///
    /// # Errors
    ///
    /// - [`MattenError::Unsupported`] if either operand is a dynamic tensor;
    ///   call `try_numeric()` on each first.
    /// - [`MattenError::Broadcast`] if the shapes are incompatible.
    /// - [`MattenError::Allocation`] if computing the result overflows or
    ///   exceeds the default element budget — only reachable with two
    ///   individually valid but very large operands (RFC-132 §12.0).
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    /// let b = Tensor::ones(&[2, 2]);
    /// let c = a.try_add(&b).unwrap();
    /// assert_eq!(c.as_slice(), &[2.0, 3.0, 4.0, 5.0]);
    ///
    /// let bad = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);
    /// assert!(a.try_add(&bad).is_err()); // incompatible shapes
    /// ```
    pub fn try_add(&self, other: &Tensor) -> Result<Tensor, MattenError> {
        try_apply_binary(self, other, "add", |a, b| a + b)
    }

    /// Element-wise subtraction with broadcasting, returning [`MattenError`]
    /// instead of panicking. See [`Tensor::try_add`] for the error conditions.
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::new(vec![5.0, 4.0, 3.0, 2.0], &[2, 2]);
    /// let b = Tensor::ones(&[2, 2]);
    /// let c = a.try_sub(&b).unwrap();
    /// assert_eq!(c.as_slice(), &[4.0, 3.0, 2.0, 1.0]);
    /// ```
    pub fn try_sub(&self, other: &Tensor) -> Result<Tensor, MattenError> {
        try_apply_binary(self, other, "sub", |a, b| a - b)
    }

    /// Element-wise multiplication with broadcasting (**not** matrix
    /// multiply; use [`Tensor::try_matmul`] for that), returning
    /// [`MattenError`] instead of panicking. See [`Tensor::try_add`] for the
    /// error conditions.
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    /// let b = Tensor::full(&[2, 2], 2.0);
    /// let c = a.try_mul(&b).unwrap();
    /// assert_eq!(c.as_slice(), &[2.0, 4.0, 6.0, 8.0]);
    /// ```
    pub fn try_mul(&self, other: &Tensor) -> Result<Tensor, MattenError> {
        try_apply_binary(self, other, "mul", |a, b| a * b)
    }

    /// Element-wise division with broadcasting, returning [`MattenError`]
    /// instead of panicking. Division by zero follows IEEE 754 `f64`
    /// behavior (yields `inf`, `-inf`, or `NaN`); it is not itself an error.
    /// See [`Tensor::try_add`] for the error conditions.
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::new(vec![4.0, 9.0], &[2]);
    /// let b = Tensor::new(vec![2.0, 3.0], &[2]);
    /// let c = a.try_div(&b).unwrap();
    /// assert_eq!(c.as_slice(), &[2.0, 3.0]);
    /// ```
    pub fn try_div(&self, other: &Tensor) -> Result<Tensor, MattenError> {
        try_apply_binary(self, other, "div", |a, b| a / b)
    }
}

impl Add for &Tensor {
    type Output = Tensor;
    /// Element-wise addition with NumPy-style broadcasting.
    ///
    /// # Panics
    ///
    /// Panics with `"matten broadcast error in add: ..."` if the shapes are
    /// incompatible. Use [`Tensor::try_add`] for the non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    /// let b = Tensor::ones(&[2, 2]);
    /// let c = &a + &b;
    /// assert_eq!(c.as_slice(), &[2.0, 3.0, 4.0, 5.0]);
    /// ```
    fn add(self, rhs: &Tensor) -> Tensor {
        self.try_add(rhs)
            .unwrap_or_else(|e| panic_for_arithmetic("add", e))
    }
}

impl Sub for &Tensor {
    type Output = Tensor;
    /// Element-wise subtraction with broadcasting.
    ///
    /// # Panics
    ///
    /// Panics on incompatible shapes. Use [`Tensor::try_sub`] for the
    /// non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::new(vec![5.0, 4.0, 3.0, 2.0], &[2, 2]);
    /// let b = Tensor::ones(&[2, 2]);
    /// let c = &a - &b;
    /// assert_eq!(c.as_slice(), &[4.0, 3.0, 2.0, 1.0]);
    /// ```
    fn sub(self, rhs: &Tensor) -> Tensor {
        self.try_sub(rhs)
            .unwrap_or_else(|e| panic_for_arithmetic("sub", e))
    }
}

impl Mul for &Tensor {
    type Output = Tensor;
    /// Element-wise multiplication with broadcasting (`*` is **not** matrix
    /// multiply; use `matmul` for that).
    ///
    /// # Panics
    ///
    /// Panics on incompatible shapes. Use [`Tensor::try_mul`] for the
    /// non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    /// let b = Tensor::full(&[2, 2], 2.0);
    /// let c = &a * &b;
    /// assert_eq!(c.as_slice(), &[2.0, 4.0, 6.0, 8.0]);
    /// ```
    fn mul(self, rhs: &Tensor) -> Tensor {
        self.try_mul(rhs)
            .unwrap_or_else(|e| panic_for_arithmetic("mul", e))
    }
}

impl Div for &Tensor {
    type Output = Tensor;
    /// Element-wise division with broadcasting. Division by zero follows IEEE 754
    /// `f64` behavior (yields `inf`, `-inf`, or `NaN`); no error is produced.
    ///
    /// # Panics
    ///
    /// Panics on incompatible shapes. Use [`Tensor::try_div`] for the
    /// non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::new(vec![4.0, 9.0], &[2]);
    /// let b = Tensor::new(vec![2.0, 3.0], &[2]);
    /// let c = &a / &b;
    /// assert_eq!(c.as_slice(), &[2.0, 3.0]);
    /// ```
    fn div(self, rhs: &Tensor) -> Tensor {
        self.try_div(rhs)
            .unwrap_or_else(|e| panic_for_arithmetic("div", e))
    }
}
