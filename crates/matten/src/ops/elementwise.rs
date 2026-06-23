//! Elementwise comfort math on numeric tensors (RFC-038).
//!
//! These are small, familiar `f64` transforms applied to every element. Each
//! preserves shape and follows ordinary `f64` NaN/Inf behavior (for example,
//! `sqrt` of a negative value is `NaN`, `ln(0.0)` is `-inf`). They are numeric-only:
//! on a dynamic tensor the convenience forms panic with an `Unsupported` message,
//! and the `try_*` form returns [`MattenError::Unsupported`]. Call
//! [`Tensor::try_numeric`](crate::Tensor::try_numeric) first to convert.

use crate::{MattenError, Tensor};

impl Tensor {
    /// Builds a new numeric tensor with this tensor's shape from a per-element map.
    ///
    /// Caller must ensure `self` is non-dynamic (the public methods guard first).
    fn map_unchecked(&self, f: impl Fn(f64) -> f64) -> Tensor {
        Tensor {
            data: self.data.iter().map(|&v| f(v)).collect(),
            shape: self.shape.clone(),
            #[cfg(feature = "dynamic")]
            dynamic: None,
        }
    }

    /// Elementwise absolute value. Shape is preserved.
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::new(vec![-1.0, 2.0, -3.0, 4.0], &[2, 2]);
    /// assert_eq!(t.abs().as_slice(), &[1.0, 2.0, 3.0, 4.0]);
    /// ```
    #[must_use]
    pub fn abs(&self) -> Tensor {
        #[cfg(feature = "dynamic")]
        self.panic_if_dynamic("abs");
        self.map_unchecked(f64::abs)
    }

    /// Elementwise square root. Shape is preserved; a negative element yields `NaN`.
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::from_vec(vec![1.0, 4.0, 9.0]);
    /// assert_eq!(t.sqrt().as_slice(), &[1.0, 2.0, 3.0]);
    /// ```
    #[must_use]
    pub fn sqrt(&self) -> Tensor {
        #[cfg(feature = "dynamic")]
        self.panic_if_dynamic("sqrt");
        self.map_unchecked(f64::sqrt)
    }

    /// Elementwise natural exponential `e^x`. Shape is preserved.
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::from_vec(vec![0.0, 1.0]);
    /// let r = t.exp();
    /// assert_eq!(r.as_slice()[0], 1.0);
    /// assert!((r.as_slice()[1] - std::f64::consts::E).abs() < 1e-12);
    /// ```
    #[must_use]
    pub fn exp(&self) -> Tensor {
        #[cfg(feature = "dynamic")]
        self.panic_if_dynamic("exp");
        self.map_unchecked(f64::exp)
    }

    /// Elementwise natural logarithm. Shape is preserved; `ln(0.0)` is `-inf` and a
    /// negative element yields `NaN`.
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::from_vec(vec![1.0, std::f64::consts::E]);
    /// let r = t.ln();
    /// assert_eq!(r.as_slice()[0], 0.0);
    /// assert!((r.as_slice()[1] - 1.0).abs() < 1e-12);
    /// ```
    #[must_use]
    pub fn ln(&self) -> Tensor {
        #[cfg(feature = "dynamic")]
        self.panic_if_dynamic("ln");
        self.map_unchecked(f64::ln)
    }

    /// Elementwise clamp into `[min, max]`. Shape is preserved.
    ///
    /// # Panics
    /// Panics if `min > max` (or either bound is `NaN`). Use [`Tensor::try_clip`]
    /// for the non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::from_vec(vec![-5.0, 0.5, 9.0]);
    /// assert_eq!(t.clip(0.0, 1.0).as_slice(), &[0.0, 0.5, 1.0]);
    /// ```
    #[must_use]
    pub fn clip(&self, min: f64, max: f64) -> Tensor {
        #[cfg(feature = "dynamic")]
        self.panic_if_dynamic("clip");
        assert!(
            min <= max,
            "matten invalid argument error in clip: min/max: min must be <= max (got min={min}, max={max})"
        );
        self.map_unchecked(|v| v.clamp(min, max))
    }

    /// Non-panicking [`Tensor::clip`].
    ///
    /// Returns [`MattenError::InvalidArgument`] if `min > max`, or
    /// [`MattenError::Unsupported`] if called on a dynamic tensor.
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::from_vec(vec![-5.0, 0.5, 9.0]);
    /// assert_eq!(t.try_clip(0.0, 1.0).unwrap().as_slice(), &[0.0, 0.5, 1.0]);
    /// assert!(t.try_clip(1.0, 0.0).is_err());
    /// ```
    pub fn try_clip(&self, min: f64, max: f64) -> Result<Tensor, MattenError> {
        #[cfg(feature = "dynamic")]
        if self.is_dynamic() {
            return Err(MattenError::Unsupported {
                operation: "clip",
                message: "clip is not supported on dynamic tensors; call try_numeric() first"
                    .to_string(),
            });
        }
        if min.is_nan() || max.is_nan() || min > max {
            return Err(MattenError::InvalidArgument {
                operation: "clip",
                argument: "min/max",
                message: format!("min must be <= max (got min={min}, max={max})"),
            });
        }
        Ok(self.map_unchecked(|v| v.clamp(min, max)))
    }
}
