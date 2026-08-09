//! Reductions, basic statistics, and matrix multiplication (RFC-010).
//!
//! All operations are Phase 1 `f64`-only. `*` remains element-wise; matrix
//! multiplication is always explicit via [`Tensor::matmul`] or [`Tensor::dot`].
//!
//! # NaN / Inf policy
//!
//! Whole reductions (`sum`, `mean`) propagate `NaN` naturally via IEEE 754.
//!
//! `min` and `max` return `NaN` if **any** element is `NaN`.  
//! **Do not** use `fold(f64::INFINITY, f64::min)` — that silently ignores
//! `NaN`. The implementation short-circuits on the first `NaN` detected.

use crate::MattenError;
use crate::Tensor;
use crate::shape::{coord_to_flat, flat_to_coord};

// ── Whole-tensor reductions ───────────────────────────────────────────────

impl Tensor {
    /// Returns the sum of all elements.
    ///
    /// `NaN` propagates naturally (IEEE 754). For a non-panicking form, see
    /// [`Tensor::try_sum`].
    ///
    /// # Panics
    /// Panics on a dynamic tensor (call [`try_numeric`](crate::Tensor::try_numeric)
    /// first). Use [`Tensor::try_sum`] for the non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// assert_eq!(Tensor::from_vec(vec![1.0, 2.0, 3.0]).sum(), 6.0);
    /// assert!(Tensor::from_vec(vec![1.0, f64::NAN]).sum().is_nan());
    /// ```
    #[must_use]
    pub fn sum(&self) -> f64 {
        self.try_sum().unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::sum`].
    ///
    /// # Errors
    /// Returns [`MattenError::Unsupported`] on a dynamic tensor. `NaN` is treated
    /// as a value and propagates according to the underlying operation.
    ///
    /// ```
    /// use matten::Tensor;
    /// assert_eq!(Tensor::from_vec(vec![1.0, 2.0, 3.0]).try_sum().unwrap(), 6.0);
    /// ```
    pub fn try_sum(&self) -> Result<f64, MattenError> {
        reject_dynamic(self, "sum")?;
        Ok(self.data.iter().sum())
    }

    /// Returns the arithmetic mean of all elements (`sum / len`).
    ///
    /// `NaN` propagates. For a non-panicking form, see [`Tensor::try_mean`].
    ///
    /// # Panics
    /// Panics on a dynamic tensor, or if the tensor is empty (RFC-105) —
    /// reachable via slicing (`t.slice().range(0..0).all().build()`) and,
    /// since RFC-111, directly via any constructor as well. The mean of an
    /// empty set is undefined, not `NaN`. Use [`Tensor::try_mean`] for the
    /// non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// assert_eq!(Tensor::from_vec(vec![1.0, 2.0, 3.0, 4.0]).mean(), 2.5);
    /// ```
    #[must_use]
    pub fn mean(&self) -> f64 {
        self.try_mean().unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::mean`].
    ///
    /// # Errors
    /// Returns [`MattenError::Unsupported`] on a dynamic tensor, or
    /// [`MattenError::InvalidArgument`] if the tensor is empty (RFC-105).
    /// `NaN` is treated as a value and propagates according to the
    /// underlying operation.
    ///
    /// ```
    /// use matten::Tensor;
    /// assert_eq!(Tensor::from_vec(vec![1.0, 2.0, 3.0, 4.0]).try_mean().unwrap(), 2.5);
    /// ```
    pub fn try_mean(&self) -> Result<f64, MattenError> {
        reject_dynamic(self, "mean")?;
        if self.data.is_empty() {
            return Err(MattenError::InvalidArgument {
                operation: "mean",
                argument: "self",
                message: "mean is undefined for an empty tensor".to_string(),
            });
        }
        Ok(self.data.iter().sum::<f64>() / self.data.len() as f64)
    }

    /// Returns the minimum element.
    ///
    /// Returns `NaN` if **any** element is `NaN` (explicit NaN-propagation;
    /// do not use `f64::min` which silently ignores NaN). For a non-panicking
    /// form, see [`Tensor::try_min`].
    ///
    /// # Panics
    /// Panics on a dynamic tensor, or if the tensor is empty (RFC-105) — the
    /// minimum of an empty set is undefined, not `f64::INFINITY`. Use
    /// [`Tensor::try_min`] for the non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// assert_eq!(Tensor::from_vec(vec![3.0, 1.0, 2.0]).min(), 1.0);
    /// assert!(Tensor::from_vec(vec![1.0, f64::NAN, 3.0]).min().is_nan());
    /// ```
    #[must_use]
    pub fn min(&self) -> f64 {
        self.try_min().unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::min`].
    ///
    /// # Errors
    /// Returns [`MattenError::Unsupported`] on a dynamic tensor, or
    /// [`MattenError::InvalidArgument`] if the tensor is empty (RFC-105).
    /// `NaN` is treated as a value and propagates according to the
    /// underlying operation.
    ///
    /// ```
    /// use matten::Tensor;
    /// assert_eq!(Tensor::from_vec(vec![3.0, 1.0, 2.0]).try_min().unwrap(), 1.0);
    /// ```
    pub fn try_min(&self) -> Result<f64, MattenError> {
        reject_dynamic(self, "min")?;
        if self.data.is_empty() {
            return Err(MattenError::InvalidArgument {
                operation: "min",
                argument: "self",
                message: "minimum is undefined for an empty tensor".to_string(),
            });
        }
        Ok(nan_reduce(&self.data, f64::INFINITY, |acc, v| acc.min(v)))
    }

    /// Returns the maximum element.
    ///
    /// Returns `NaN` if **any** element is `NaN`. For a non-panicking form, see
    /// [`Tensor::try_max`].
    ///
    /// # Panics
    /// Panics on a dynamic tensor, or if the tensor is empty (RFC-105) — the
    /// maximum of an empty set is undefined, not `f64::NEG_INFINITY`. Use
    /// [`Tensor::try_max`] for the non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// assert_eq!(Tensor::from_vec(vec![3.0, 1.0, 2.0]).max(), 3.0);
    /// assert!(Tensor::from_vec(vec![1.0, f64::NAN, 3.0]).max().is_nan());
    /// ```
    #[must_use]
    pub fn max(&self) -> f64 {
        self.try_max().unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::max`].
    ///
    /// # Errors
    /// Returns [`MattenError::Unsupported`] on a dynamic tensor, or
    /// [`MattenError::InvalidArgument`] if the tensor is empty (RFC-105).
    /// `NaN` is treated as a value and propagates according to the
    /// underlying operation.
    ///
    /// ```
    /// use matten::Tensor;
    /// assert_eq!(Tensor::from_vec(vec![3.0, 1.0, 2.0]).try_max().unwrap(), 3.0);
    /// ```
    pub fn try_max(&self) -> Result<f64, MattenError> {
        reject_dynamic(self, "max")?;
        if self.data.is_empty() {
            return Err(MattenError::InvalidArgument {
                operation: "max",
                argument: "self",
                message: "maximum is undefined for an empty tensor".to_string(),
            });
        }
        Ok(nan_reduce(&self.data, f64::NEG_INFINITY, |acc, v| {
            acc.max(v)
        }))
    }
}

/// Rejects a dynamic tensor with [`MattenError::Unsupported`]. No-op when the
/// `dynamic` feature is disabled. Shared by the core value/axis reductions
/// (RFC-055/056) and the statistics reductions.
pub(crate) fn reject_dynamic(t: &Tensor, operation: &'static str) -> Result<(), MattenError> {
    #[cfg(feature = "dynamic")]
    if t.is_dynamic() {
        return Err(MattenError::Unsupported {
            operation,
            message: format!(
                "{operation} is not supported on dynamic tensors; call try_numeric() first"
            ),
        });
    }
    #[cfg(not(feature = "dynamic"))]
    let _ = (t, operation);
    Ok(())
}

/// Validates `axis` against the tensor rank, returning [`MattenError::Shape`]
/// when out of range. Shared by the axis reductions (RFC-056).
pub(crate) fn check_axis(
    t: &Tensor,
    axis: usize,
    operation: &'static str,
) -> Result<(), MattenError> {
    let rank = t.shape().len();
    if axis >= rank {
        return Err(MattenError::Shape {
            operation,
            message: format!("axis {axis} is out of range for a rank-{rank} tensor"),
        });
    }
    Ok(())
}

/// Reduces `data` with `f` starting from `init`, short-circuiting to `NaN`
/// on the first NaN encountered.  This avoids `f64::min`/`f64::max` which
/// silently ignore NaN.
fn nan_reduce(data: &[f64], init: f64, f: impl Fn(f64, f64) -> f64) -> f64 {
    let mut acc = init;
    for &v in data {
        if v.is_nan() {
            return f64::NAN;
        }
        acc = f(acc, v);
    }
    acc
}

// ── Axis reductions ───────────────────────────────────────────────────────

impl Tensor {
    /// Reduces along `axis` by summing, removing that axis from the output shape.
    ///
    /// For a non-panicking form, see [`Tensor::try_sum_axis`].
    ///
    /// # Panics
    ///
    /// Panics if `axis >= self.ndim()`, or on a dynamic tensor.
    ///
    /// ```
    /// use matten::Tensor;
    /// // [[1,2,3],[4,5,6]] summed along axis 0 -> [5,7,9]
    /// let m = Tensor::new(vec![1.0,2.0,3.0,4.0,5.0,6.0], &[2,3]);
    /// let r = m.sum_axis(0);
    /// assert_eq!(r.shape(), &[3]);
    /// assert_eq!(r.as_slice(), &[5.0, 7.0, 9.0]);
    /// ```
    #[must_use]
    pub fn sum_axis(&self, axis: usize) -> Tensor {
        self.try_sum_axis(axis).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::sum_axis`]. The reduced axis is removed from the
    /// output shape, matching the panic form.
    ///
    /// # Errors
    /// Returns [`MattenError::Shape`] if `axis >= rank`, or
    /// [`MattenError::Unsupported`] on a dynamic tensor.
    ///
    /// ```
    /// use matten::Tensor;
    /// let m = Tensor::new(vec![1.0,2.0,3.0,4.0,5.0,6.0], &[2,3]);
    /// assert_eq!(m.try_sum_axis(0).unwrap().as_slice(), &[5.0, 7.0, 9.0]);
    /// assert!(m.try_sum_axis(2).is_err()); // axis out of range
    /// ```
    pub fn try_sum_axis(&self, axis: usize) -> Result<Tensor, MattenError> {
        reject_dynamic(self, "sum_axis")?;
        check_axis(self, axis, "sum_axis")?;
        Ok(axis_reduce(self, axis, "sum_axis", |acc, v| acc + v, 0.0))
    }

    /// Reduces along `axis` by computing the arithmetic mean.
    ///
    /// For a non-panicking form, see [`Tensor::try_mean_axis`].
    ///
    /// # Panics
    ///
    /// Panics if `axis >= self.ndim()`, on a dynamic tensor, or if the
    /// **reduced** axis has length 0 (RFC-110) — the mean of nothing is
    /// undefined, not `NaN`. A zero-length axis that *survives* the reduction
    /// (a different axis than `axis`) is unaffected and yields an empty
    /// result, not a panic.
    ///
    /// ```
    /// use matten::Tensor;
    /// let m = Tensor::new(vec![1.0,2.0,3.0,4.0,5.0,6.0], &[2,3]);
    /// let r = m.mean_axis(0);
    /// assert_eq!(r.shape(), &[3]);
    /// assert_eq!(r.as_slice(), &[2.5, 3.5, 4.5]);
    /// ```
    #[must_use]
    pub fn mean_axis(&self, axis: usize) -> Tensor {
        self.try_mean_axis(axis).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::mean_axis`]. The reduced axis is removed from the
    /// output shape, matching the panic form.
    ///
    /// # Errors
    /// Returns [`MattenError::Shape`] if `axis >= rank`,
    /// [`MattenError::Unsupported`] on a dynamic tensor, or
    /// [`MattenError::InvalidArgument`] if the **reduced** axis has length 0
    /// (RFC-110) — reachable via slicing (`t.slice().range(0..0).all().build()`)
    /// and, since RFC-111, directly via any constructor as well. A zero-length
    /// axis that *survives* the reduction still returns `Ok` with an empty
    /// result.
    ///
    /// ```
    /// use matten::Tensor;
    /// let m = Tensor::new(vec![1.0,2.0,3.0,4.0,5.0,6.0], &[2,3]);
    /// assert_eq!(m.try_mean_axis(0).unwrap().as_slice(), &[2.5, 3.5, 4.5]);
    /// ```
    pub fn try_mean_axis(&self, axis: usize) -> Result<Tensor, MattenError> {
        reject_dynamic(self, "mean_axis")?;
        check_axis(self, axis, "mean_axis")?;
        let n = self.shape()[axis] as f64;
        if n == 0.0 {
            return Err(MattenError::InvalidArgument {
                operation: "mean_axis",
                argument: "axis",
                message: format!("mean is undefined for a reduced axis of length 0 (axis {axis})"),
            });
        }
        let sums = axis_reduce(self, axis, "mean_axis", |acc, v| acc + v, 0.0);
        Ok(&sums / n)
    }
}

// ── Min/max axis reductions ───────────────────────────────────────────────

impl Tensor {
    /// Reduces along `axis` by taking the minimum, removing that axis from the
    /// output shape. Returns `NaN` if any element along the axis is `NaN`.
    ///
    /// For a non-panicking form, see [`Tensor::try_min_axis`].
    ///
    /// # Panics
    ///
    /// Panics if `axis >= self.ndim()`, on a dynamic tensor, or if the
    /// **reduced** axis has length 0 (RFC-110) — the minimum of nothing is
    /// undefined, not `f64::INFINITY`. A zero-length axis that *survives* the
    /// reduction yields an empty result, not a panic.
    ///
    /// ```
    /// use matten::Tensor;
    /// let m = Tensor::new(vec![3.0,1.0,4.0,1.0,5.0,9.0], &[2,3]);
    /// assert_eq!(m.min_axis(0).as_slice(), &[1.0, 1.0, 4.0]);
    /// ```
    #[must_use]
    pub fn min_axis(&self, axis: usize) -> Tensor {
        self.try_min_axis(axis).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::min_axis`]. The reduced axis is removed from the
    /// output shape, matching the panic form.
    ///
    /// # Errors
    /// Returns [`MattenError::Shape`] if `axis >= rank`,
    /// [`MattenError::Unsupported`] on a dynamic tensor, or
    /// [`MattenError::InvalidArgument`] if the **reduced** axis has length 0
    /// (RFC-110). A zero-length axis that *survives* the reduction still
    /// returns `Ok` with an empty result.
    ///
    /// ```
    /// use matten::Tensor;
    /// let m = Tensor::new(vec![3.0,1.0,4.0,1.0,5.0,9.0], &[2,3]);
    /// assert_eq!(m.try_min_axis(0).unwrap().as_slice(), &[1.0, 1.0, 4.0]);
    /// assert!(m.try_min_axis(2).is_err()); // axis out of range
    /// ```
    pub fn try_min_axis(&self, axis: usize) -> Result<Tensor, MattenError> {
        reject_dynamic(self, "min_axis")?;
        check_axis(self, axis, "min_axis")?;
        if self.shape()[axis] == 0 {
            return Err(MattenError::InvalidArgument {
                operation: "min_axis",
                argument: "axis",
                message: format!(
                    "minimum is undefined for a reduced axis of length 0 (axis {axis})"
                ),
            });
        }
        Ok(nan_axis_reduce(
            self,
            axis,
            "min_axis",
            f64::INFINITY,
            |a, b| a.min(b),
        ))
    }

    /// Reduces along `axis` by taking the maximum, removing that axis from the
    /// output shape. Returns `NaN` if any element along the axis is `NaN`.
    ///
    /// For a non-panicking form, see [`Tensor::try_max_axis`].
    ///
    /// # Panics
    ///
    /// Panics if `axis >= self.ndim()`, on a dynamic tensor, or if the
    /// **reduced** axis has length 0 (RFC-110) — the maximum of nothing is
    /// undefined, not `f64::NEG_INFINITY`. A zero-length axis that *survives*
    /// the reduction yields an empty result, not a panic.
    ///
    /// ```
    /// use matten::Tensor;
    /// let m = Tensor::new(vec![3.0,1.0,4.0,1.0,5.0,9.0], &[2,3]);
    /// assert_eq!(m.max_axis(0).as_slice(), &[3.0, 5.0, 9.0]);
    /// ```
    #[must_use]
    pub fn max_axis(&self, axis: usize) -> Tensor {
        self.try_max_axis(axis).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::max_axis`]. The reduced axis is removed from the
    /// output shape, matching the panic form.
    ///
    /// # Errors
    /// Returns [`MattenError::Shape`] if `axis >= rank`,
    /// [`MattenError::Unsupported`] on a dynamic tensor, or
    /// [`MattenError::InvalidArgument`] if the **reduced** axis has length 0
    /// (RFC-110). A zero-length axis that *survives* the reduction still
    /// returns `Ok` with an empty result.
    ///
    /// ```
    /// use matten::Tensor;
    /// let m = Tensor::new(vec![3.0,1.0,4.0,1.0,5.0,9.0], &[2,3]);
    /// assert_eq!(m.try_max_axis(0).unwrap().as_slice(), &[3.0, 5.0, 9.0]);
    /// ```
    pub fn try_max_axis(&self, axis: usize) -> Result<Tensor, MattenError> {
        reject_dynamic(self, "max_axis")?;
        check_axis(self, axis, "max_axis")?;
        if self.shape()[axis] == 0 {
            return Err(MattenError::InvalidArgument {
                operation: "max_axis",
                argument: "axis",
                message: format!(
                    "maximum is undefined for a reduced axis of length 0 (axis {axis})"
                ),
            });
        }
        Ok(nan_axis_reduce(
            self,
            axis,
            "max_axis",
            f64::NEG_INFINITY,
            |a, b| a.max(b),
        ))
    }
}

/// Axis reduction with explicit NaN propagation (for min/max).
fn nan_axis_reduce(
    t: &Tensor,
    axis: usize,
    operation: &'static str,
    identity: f64,
    f: impl Fn(f64, f64) -> f64,
) -> Tensor {
    if axis >= t.ndim() {
        panic!(
            "matten shape error in {operation}: axis {axis} is out of range for rank-{} tensor",
            t.ndim()
        );
    }
    let src_shape = t.shape();
    let out_shape: Vec<usize> = src_shape
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != axis)
        .map(|(_, &d)| d)
        .collect();
    let out_len: usize = if out_shape.is_empty() {
        1
    } else {
        out_shape.iter().product()
    };
    let mut out_data = vec![identity; out_len];
    let mut has_nan = vec![false; out_len];

    for (src_flat, &val) in t.data.iter().enumerate() {
        let src_coord = flat_to_coord(src_flat, src_shape);
        let out_coord: Vec<usize> = src_coord
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != axis)
            .map(|(_, &c)| c)
            .collect();
        let dst_flat = if out_shape.is_empty() {
            0
        } else {
            coord_to_flat(&out_coord, &out_shape).expect("valid by construction")
        };
        if val.is_nan() {
            has_nan[dst_flat] = true;
        } else {
            out_data[dst_flat] = f(out_data[dst_flat], val);
        }
    }
    // Propagate NaN
    for (i, &nan) in has_nan.iter().enumerate() {
        if nan {
            out_data[i] = f64::NAN;
        }
    }
    Tensor {
        data: out_data,
        shape: out_shape,
        #[cfg(feature = "dynamic")]
        dynamic: None,
    }
}

/// Generic axis reduction.
fn axis_reduce(
    t: &Tensor,
    axis: usize,
    operation: &'static str,
    f: impl Fn(f64, f64) -> f64,
    identity: f64,
) -> Tensor {
    if axis >= t.ndim() {
        panic!(
            "matten shape error in {operation}: axis {axis} is out of range for rank-{} tensor",
            t.ndim()
        );
    }
    let src_shape = t.shape();
    // Output shape: remove the reduced axis.
    let out_shape: Vec<usize> = src_shape
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != axis)
        .map(|(_, &d)| d)
        .collect();
    let out_len: usize = if out_shape.is_empty() {
        1
    } else {
        out_shape.iter().product()
    };
    let mut out_data = vec![identity; out_len];

    for (src_flat, &val) in t.data.iter().enumerate() {
        let src_coord = flat_to_coord(src_flat, src_shape);
        // Drop the reduced axis coordinate.
        let out_coord: Vec<usize> = src_coord
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != axis)
            .map(|(_, &c)| c)
            .collect();
        let dst_flat = if out_shape.is_empty() {
            0
        } else {
            coord_to_flat(&out_coord, &out_shape).expect("valid by construction")
        };
        out_data[dst_flat] = f(out_data[dst_flat], val);
    }

    Tensor {
        data: out_data,
        shape: out_shape,
        #[cfg(feature = "dynamic")]
        dynamic: None,
    }
}

// ── dot and matmul ────────────────────────────────────────────────────────

impl Tensor {
    /// Vector/matrix multiplication.
    ///
    /// Supported forms:
    ///
    /// | `self` shape | `rhs` shape | result shape |
    /// |---|---|---|
    /// | `[n]` | `[n]` | `[]` scalar |
    /// | `[m, n]` | `[n]` | `[m]` |
    /// | `[n]` | `[n, p]` | `[p]` |
    /// | `[m, n]` | `[n, p]` | `[m, p]` |
    ///
    /// # Panics
    ///
    /// Panics on incompatible shapes or unsupported rank combinations. For a
    /// non-panicking form, see [`Tensor::try_dot`].
    ///
    /// ```
    /// use matten::Tensor;
    ///
    /// // vector · vector -> scalar
    /// let a = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    /// let b = Tensor::from_vec(vec![4.0, 5.0, 6.0]);
    /// let d = a.dot(&b);
    /// assert!(d.is_scalar());
    /// assert_eq!(d.as_slice(), &[32.0]); // 1*4 + 2*5 + 3*6
    ///
    /// // matrix × vector -> vector
    /// let m = Tensor::new(vec![1.0,2.0,3.0,4.0,5.0,6.0], &[2,3]);
    /// let v = Tensor::from_vec(vec![1.0, 0.0, 1.0]);
    /// let r = m.dot(&v);
    /// assert_eq!(r.shape(), &[2]);
    /// assert_eq!(r.as_slice(), &[4.0, 10.0]);
    /// ```
    #[must_use]
    pub fn dot(&self, rhs: &Tensor) -> Tensor {
        self.try_dot(rhs).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::dot`].
    ///
    /// # Errors
    /// Returns [`MattenError::Unsupported`] if either operand is a dynamic tensor
    /// (call [`try_numeric`](crate::Tensor::try_numeric) on each operand first), or
    /// [`MattenError::Shape`] on incompatible shapes or an unsupported rank
    /// combination.
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    /// let b = Tensor::from_vec(vec![4.0, 5.0, 6.0]);
    /// assert_eq!(a.try_dot(&b).unwrap().as_slice(), &[32.0]);
    /// ```
    pub fn try_dot(&self, rhs: &Tensor) -> Result<Tensor, MattenError> {
        #[cfg(feature = "dynamic")]
        if self.is_dynamic() || rhs.is_dynamic() {
            return Err(MattenError::Unsupported {
                operation: "dot/matmul",
                message:
                    "not supported on dynamic tensors; call try_numeric() on each operand first"
                        .to_string(),
            });
        }
        matmul_dispatch(self, rhs, "dot")
    }

    /// Alias for [`dot`](Tensor::dot) for familiarity.
    ///
    /// `*` is always element-wise multiplication; matrix multiplication requires
    /// this explicit method.
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::new(vec![1.0,2.0,3.0,4.0], &[2,2]);
    /// let b = Tensor::new(vec![5.0,6.0,7.0,8.0], &[2,2]);
    /// let c = a.matmul(&b);
    /// assert_eq!(c.shape(), &[2,2]);
    /// assert_eq!(c.as_slice(), &[19.0, 22.0, 43.0, 50.0]);
    /// ```
    #[must_use]
    pub fn matmul(&self, rhs: &Tensor) -> Tensor {
        self.try_matmul(rhs).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::matmul`]; delegates to [`Tensor::try_dot`], which
    /// contains the dynamic guard.
    ///
    /// # Errors
    /// See [`Tensor::try_dot`].
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::new(vec![1.0,2.0,3.0,4.0], &[2,2]);
    /// let b = Tensor::new(vec![5.0,6.0,7.0,8.0], &[2,2]);
    /// assert_eq!(a.try_matmul(&b).unwrap().as_slice(), &[19.0, 22.0, 43.0, 50.0]);
    /// ```
    pub fn try_matmul(&self, rhs: &Tensor) -> Result<Tensor, MattenError> {
        self.try_dot(rhs)
    }
}

fn matmul_dispatch(lhs: &Tensor, rhs: &Tensor, op: &'static str) -> Result<Tensor, MattenError> {
    match (lhs.ndim(), rhs.ndim()) {
        (1, 1) => vv_dot(lhs, rhs, op),
        (2, 1) => mv_mul(lhs, rhs, op),
        (1, 2) => vm_mul(lhs, rhs, op),
        (2, 2) => mm_mul(lhs, rhs, op),
        _ => Err(MattenError::Shape {
            operation: op,
            message: format!(
                "unsupported rank combination \
                 (left rank {}, right rank {}); supported: [n]×[n], [m,n]×[n], [n]×[n,p], [m,n]×[n,p]",
                lhs.ndim(),
                rhs.ndim()
            ),
        }),
    }
}

/// `[n] · [n] -> []` scalar tensor.
fn vv_dot(a: &Tensor, b: &Tensor, op: &'static str) -> Result<Tensor, MattenError> {
    let n = a.len();
    if b.len() != n {
        return Err(MattenError::Shape {
            operation: op,
            message: format!("vector lengths must match (left {n}, right {})", b.len()),
        });
    }
    let v: f64 = a.data.iter().zip(&b.data).map(|(x, y)| x * y).sum();
    Ok(Tensor::scalar(v))
}

/// `[m, n] × [n] -> [m]`.
fn mv_mul(a: &Tensor, b: &Tensor, op: &'static str) -> Result<Tensor, MattenError> {
    let [m, n] = shape2(a, op)?;
    dim_check(n, b.len(), "left columns", "right length", op)?;
    let mut out = vec![0.0f64; m];
    for (i, o) in out.iter_mut().enumerate() {
        for k in 0..n {
            *o += a.data[i * n + k] * b.data[k];
        }
    }
    Ok(Tensor {
        data: out,
        shape: vec![m],
        #[cfg(feature = "dynamic")]
        dynamic: None,
    })
}

/// `[n] × [n, p] -> [p]`.
fn vm_mul(a: &Tensor, b: &Tensor, op: &'static str) -> Result<Tensor, MattenError> {
    let [n, p] = shape2(b, op)?;
    dim_check(a.len(), n, "left length", "right rows", op)?;
    let mut out = vec![0.0f64; p];
    for k in 0..n {
        for (j, slot) in out.iter_mut().enumerate() {
            *slot += a.data[k] * b.data[k * p + j];
        }
    }
    Ok(Tensor {
        data: out,
        shape: vec![p],
        #[cfg(feature = "dynamic")]
        dynamic: None,
    })
}

/// `[m, n] × [n, p] -> [m, p]`.
fn mm_mul(a: &Tensor, b: &Tensor, op: &'static str) -> Result<Tensor, MattenError> {
    let [m, n] = shape2(a, op)?;
    let [nb, p] = shape2(b, op)?;
    dim_check(n, nb, "left columns", "right rows", op)?;
    let mut out = vec![0.0f64; m * p];
    // `chunks_mut(0)` panics regardless of slice length, so p == 0 must be
    // guarded before the loop; the result is simply the empty [m, p] matrix.
    if p == 0 {
        return Ok(Tensor {
            data: out,
            shape: vec![m, p],
            #[cfg(feature = "dynamic")]
            dynamic: None,
        });
    }
    for (i, row) in out.chunks_mut(p).enumerate() {
        for (j, slot) in row.iter_mut().enumerate() {
            let mut acc = 0.0f64;
            for k in 0..n {
                acc += a.data[i * n + k] * b.data[k * p + j];
            }
            *slot = acc;
        }
    }
    Ok(Tensor {
        data: out,
        shape: vec![m, p],
        #[cfg(feature = "dynamic")]
        dynamic: None,
    })
}

/// Extracts shape `[d0, d1]` from a rank-2 tensor; errors on wrong rank.
fn shape2(t: &Tensor, op: &'static str) -> Result<[usize; 2], MattenError> {
    match t.shape() {
        [a, b] => Ok([*a, *b]),
        s => Err(MattenError::Shape {
            operation: op,
            message: format!("expected rank-2 tensor, got shape {s:?}"),
        }),
    }
}

/// Errors with an actionable message when two dimensions that must be equal differ.
fn dim_check(
    left: usize,
    right: usize,
    left_name: &str,
    right_name: &str,
    op: &'static str,
) -> Result<(), MattenError> {
    if left != right {
        return Err(MattenError::Shape {
            operation: op,
            message: format!("{left_name} ({left}) must equal {right_name} ({right})"),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests;
