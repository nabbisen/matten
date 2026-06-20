//! Scalar arithmetic operators: `&Tensor op f64` and `f64 op &Tensor` (RFC-006).
//!
//! The scalar-on-left forms (`f64 op &Tensor`) are legal under Rust's coherence
//! rules: `&Tensor` is a local type in the trait's parameter position, so a
//! concrete `impl Add<&Tensor> for f64` does not violate the orphan rule. Only a
//! generic blanket `impl<T> Add<&Tensor> for T` would.

use crate::Tensor;
use std::ops::{Add, Div, Mul, Sub};

// ---- &Tensor op f64 ---------------------------------------------------

impl Add<f64> for &Tensor {
    type Output = Tensor;
    /// Adds a scalar to every element.
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);
    /// let r = &t + 10.0;
    /// assert_eq!(r.as_slice(), &[11.0, 12.0, 13.0]);
    /// ```
    fn add(self, rhs: f64) -> Tensor {
        Tensor {
            data: self.data.iter().map(|&v| v + rhs).collect(),
            shape: self.shape.clone(),
            #[cfg(feature = "dynamic")]
            dynamic: None,
        }
    }
}

impl Sub<f64> for &Tensor {
    type Output = Tensor;
    /// Subtracts a scalar from every element.
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::new(vec![5.0, 3.0, 1.0], &[3]);
    /// let r = &t - 1.0;
    /// assert_eq!(r.as_slice(), &[4.0, 2.0, 0.0]);
    /// ```
    fn sub(self, rhs: f64) -> Tensor {
        Tensor {
            data: self.data.iter().map(|&v| v - rhs).collect(),
            shape: self.shape.clone(),
            #[cfg(feature = "dynamic")]
            dynamic: None,
        }
    }
}

impl Mul<f64> for &Tensor {
    type Output = Tensor;
    /// Multiplies every element by a scalar.
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);
    /// let r = &t * 3.0;
    /// assert_eq!(r.as_slice(), &[3.0, 6.0, 9.0]);
    /// ```
    fn mul(self, rhs: f64) -> Tensor {
        Tensor {
            data: self.data.iter().map(|&v| v * rhs).collect(),
            shape: self.shape.clone(),
            #[cfg(feature = "dynamic")]
            dynamic: None,
        }
    }
}

impl Div<f64> for &Tensor {
    type Output = Tensor;
    /// Divides every element by a scalar. Division by zero follows IEEE 754.
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::new(vec![6.0, 4.0, 2.0], &[3]);
    /// let r = &t / 2.0;
    /// assert_eq!(r.as_slice(), &[3.0, 2.0, 1.0]);
    /// ```
    fn div(self, rhs: f64) -> Tensor {
        Tensor {
            data: self.data.iter().map(|&v| v / rhs).collect(),
            shape: self.shape.clone(),
            #[cfg(feature = "dynamic")]
            dynamic: None,
        }
    }
}

// ---- f64 op &Tensor ---------------------------------------------------

impl Add<&Tensor> for f64 {
    type Output = Tensor;
    /// Adds every element of the tensor to a scalar (`scalar + &tensor`).
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);
    /// let r = 10.0 + &t;
    /// assert_eq!(r.as_slice(), &[11.0, 12.0, 13.0]);
    /// ```
    fn add(self, rhs: &Tensor) -> Tensor {
        Tensor {
            data: rhs.data.iter().map(|&v| self + v).collect(),
            shape: rhs.shape.clone(),
            #[cfg(feature = "dynamic")]
            dynamic: None,
        }
    }
}

impl Sub<&Tensor> for f64 {
    type Output = Tensor;
    /// Subtracts each tensor element from a scalar (`scalar - &tensor`).
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);
    /// let r = 10.0 - &t;
    /// assert_eq!(r.as_slice(), &[9.0, 8.0, 7.0]);
    /// ```
    fn sub(self, rhs: &Tensor) -> Tensor {
        Tensor {
            data: rhs.data.iter().map(|&v| self - v).collect(),
            shape: rhs.shape.clone(),
            #[cfg(feature = "dynamic")]
            dynamic: None,
        }
    }
}

impl Mul<&Tensor> for f64 {
    type Output = Tensor;
    /// Multiplies every element of the tensor by a scalar (`scalar * &tensor`).
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);
    /// let r = 3.0 * &t;
    /// assert_eq!(r.as_slice(), &[3.0, 6.0, 9.0]);
    /// ```
    fn mul(self, rhs: &Tensor) -> Tensor {
        Tensor {
            data: rhs.data.iter().map(|&v| self * v).collect(),
            shape: rhs.shape.clone(),
            #[cfg(feature = "dynamic")]
            dynamic: None,
        }
    }
}

impl Div<&Tensor> for f64 {
    type Output = Tensor;
    /// Divides a scalar by each tensor element (`scalar / &tensor`). Division by
    /// zero follows IEEE 754.
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::new(vec![2.0, 4.0, 8.0], &[3]);
    /// let r = 16.0 / &t;
    /// assert_eq!(r.as_slice(), &[8.0, 4.0, 2.0]);
    /// ```
    fn div(self, rhs: &Tensor) -> Tensor {
        Tensor {
            data: rhs.data.iter().map(|&v| self / v).collect(),
            shape: rhs.shape.clone(),
            #[cfg(feature = "dynamic")]
            dynamic: None,
        }
    }
}
