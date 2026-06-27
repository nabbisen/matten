//! Small statistics reductions (RFC-040).
//!
//! [`Tensor::var`] / [`Tensor::std`] compute the **population** variance and
//! standard deviation over all elements; [`Tensor::var_axis`] / [`Tensor::std_axis`]
//! do the same along one axis (removing it from the output shape). These are the
//! only statistics in core — quantile, percentile, histogram, covariance,
//! correlation, and z-score are deferred to a possible future `matten-stats`
//! companion (RFC-040 §6/§8), and there is no sample-variance (`ddof = 1`) variant
//! in the first cut.
//!
//! **Variance is population variance, not sample variance:**
//! `var = sum((x_i - mean)^2) / n` and `std = sqrt(var)`. A two-pass algorithm is
//! used (mean first, then squared deviations) to avoid the avoidable cancellation
//! of the naive one-pass `E[x^2] - E[x]^2`. `NaN` propagates (any `NaN` element
//! yields `NaN`), consistent with the other `f64` reductions.

use crate::{MattenError, Tensor};

/// Population variance of a non-empty slice, two-pass: `sum((x - mean)^2) / n`.
/// Callers guarantee `data` is non-empty.
fn population_variance(data: &[f64]) -> f64 {
    let n = data.len() as f64;
    let mean = data.iter().sum::<f64>() / n;
    data.iter()
        .map(|x| {
            let d = x - mean;
            d * d
        })
        .sum::<f64>()
        / n
}

/// Population variance reduced along `axis`, removing that axis (two-pass per
/// slice). Shared by [`Tensor::try_var_axis`] and [`Tensor::try_std_axis`].
fn variance_axis_impl(
    t: &Tensor,
    axis: usize,
    operation: &'static str,
) -> Result<Tensor, MattenError> {
    crate::math::reject_dynamic(t, operation)?;
    let rank = t.shape.len();
    if axis >= rank {
        return Err(MattenError::Shape {
            operation,
            message: format!("axis {axis} is out of range for a rank-{rank} tensor"),
        });
    }

    let axis_len = t.shape[axis];
    let outer: usize = t.shape[..axis].iter().product();
    let inner: usize = t.shape[axis + 1..].iter().product();

    let mut data = Vec::with_capacity(outer * inner);
    for o in 0..outer {
        let base = o * axis_len * inner;
        for i in 0..inner {
            // Two-pass over the `axis_len` values at stride `inner`.
            let mut sum = 0.0;
            for a in 0..axis_len {
                sum += t.data[base + a * inner + i];
            }
            let mean = sum / axis_len as f64;
            let mut acc = 0.0;
            for a in 0..axis_len {
                let d = t.data[base + a * inner + i] - mean;
                acc += d * d;
            }
            data.push(acc / axis_len as f64);
        }
    }

    let out_shape: Vec<usize> = t.shape[..axis]
        .iter()
        .chain(&t.shape[axis + 1..])
        .copied()
        .collect();

    Ok(Tensor {
        data,
        shape: out_shape,
        #[cfg(feature = "dynamic")]
        dynamic: None,
    })
}

impl Tensor {
    /// Population variance over all elements: `sum((x_i - mean)^2) / n`.
    ///
    /// This is **population** variance (`ddof = 0`), not sample variance — it
    /// divides by `n`, not `n - 1`. A single-element tensor has variance `0.0`.
    /// `NaN` propagates.
    ///
    /// # Panics
    /// Panics on a dynamic tensor (call [`try_numeric`](crate::Tensor::try_numeric)
    /// first). Use [`Tensor::try_var`] for the non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// // [1,2,3,4]: mean 2.5, population variance 1.25
    /// assert_eq!(Tensor::from_vec(vec![1.0, 2.0, 3.0, 4.0]).var(), 1.25);
    /// ```
    #[must_use]
    pub fn var(&self) -> f64 {
        self.try_var().unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::var`].
    ///
    /// # Errors
    /// Returns [`MattenError::Unsupported`] on a dynamic tensor. The empty-tensor
    /// guard returns [`MattenError::InvalidArgument`], but `matten` already forbids
    /// zero-sized dimensions, so an empty tensor is not constructible in practice.
    ///
    /// ```
    /// use matten::Tensor;
    /// assert_eq!(Tensor::from_vec(vec![1.0, 2.0, 3.0, 4.0]).try_var().unwrap(), 1.25);
    /// ```
    pub fn try_var(&self) -> Result<f64, MattenError> {
        crate::math::reject_dynamic(self, "var")?;
        if self.data.is_empty() {
            return Err(MattenError::InvalidArgument {
                operation: "var",
                argument: "self",
                message: "variance is undefined for an empty tensor".to_string(),
            });
        }
        Ok(population_variance(&self.data))
    }

    /// Population standard deviation over all elements: `sqrt(var)`.
    ///
    /// Population (`ddof = 0`), not sample. A single-element tensor has std `0.0`.
    /// `NaN` propagates.
    ///
    /// # Panics
    /// Panics on a dynamic tensor. Use [`Tensor::try_std`] for the non-panicking
    /// form.
    ///
    /// ```
    /// use matten::Tensor;
    /// let s = Tensor::from_vec(vec![1.0, 2.0, 3.0, 4.0]).std();
    /// assert!((s - 1.25_f64.sqrt()).abs() < 1e-12);
    /// ```
    #[must_use]
    pub fn std(&self) -> f64 {
        self.try_std().unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::std`].
    ///
    /// # Errors
    /// Returns [`MattenError::Unsupported`] on a dynamic tensor (and
    /// [`MattenError::InvalidArgument`] on the unreachable empty-tensor case).
    ///
    /// ```
    /// use matten::Tensor;
    /// assert!(Tensor::from_vec(vec![5.0]).try_std().unwrap() == 0.0);
    /// ```
    pub fn try_std(&self) -> Result<f64, MattenError> {
        crate::math::reject_dynamic(self, "std")?;
        if self.data.is_empty() {
            return Err(MattenError::InvalidArgument {
                operation: "std",
                argument: "self",
                message: "standard deviation is undefined for an empty tensor".to_string(),
            });
        }
        Ok(population_variance(&self.data).sqrt())
    }

    /// Population variance along `axis`, removing that axis from the output shape.
    ///
    /// Population (`ddof = 0`). `NaN` propagates within each reduced slice. No
    /// `keepdims` (e.g. `[2, 3]` axis 0 → `[3]`, axis 1 → `[2]`).
    ///
    /// # Panics
    /// Panics if `axis >= rank`, or on a dynamic tensor. Use
    /// [`Tensor::try_var_axis`] for the non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    /// assert_eq!(m.var_axis(0).as_slice(), &[2.25, 2.25, 2.25]);
    /// ```
    #[must_use]
    pub fn var_axis(&self, axis: usize) -> Tensor {
        self.try_var_axis(axis).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::var_axis`].
    ///
    /// # Errors
    /// Returns [`MattenError::Shape`] if `axis >= rank`, or
    /// [`MattenError::Unsupported`] on a dynamic tensor.
    ///
    /// ```
    /// use matten::Tensor;
    /// let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    /// assert!(m.try_var_axis(2).is_err()); // axis out of range
    /// ```
    pub fn try_var_axis(&self, axis: usize) -> Result<Tensor, MattenError> {
        variance_axis_impl(self, axis, "var_axis")
    }

    /// Population standard deviation along `axis`, removing that axis.
    ///
    /// Population (`ddof = 0`). `NaN` propagates within each reduced slice.
    ///
    /// # Panics
    /// Panics if `axis >= rank`, or on a dynamic tensor. Use
    /// [`Tensor::try_std_axis`] for the non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    /// // each column [1,4],[2,5],[3,6] has variance 2.25, std 1.5
    /// assert_eq!(m.std_axis(0).as_slice(), &[1.5, 1.5, 1.5]);
    /// ```
    #[must_use]
    pub fn std_axis(&self, axis: usize) -> Tensor {
        self.try_std_axis(axis).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::std_axis`].
    ///
    /// # Errors
    /// Returns [`MattenError::Shape`] if `axis >= rank`, or
    /// [`MattenError::Unsupported`] on a dynamic tensor.
    ///
    /// ```
    /// use matten::Tensor;
    /// let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    /// assert!(m.try_std_axis(5).is_err());
    /// ```
    pub fn try_std_axis(&self, axis: usize) -> Result<Tensor, MattenError> {
        let mut v = variance_axis_impl(self, axis, "std_axis")?;
        for x in &mut v.data {
            *x = x.sqrt();
        }
        Ok(v)
    }
}
