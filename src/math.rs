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

use crate::shape::{coord_to_flat, flat_to_coord};
use crate::{MattenError, Tensor};

// ── Whole-tensor reductions ───────────────────────────────────────────────

impl Tensor {
    /// Returns the sum of all elements.
    ///
    /// `NaN` propagates naturally (IEEE 754).
    ///
    /// ```
    /// use matten::Tensor;
    /// assert_eq!(Tensor::from_vec(vec![1.0, 2.0, 3.0]).sum(), 6.0);
    /// assert!(Tensor::from_vec(vec![1.0, f64::NAN]).sum().is_nan());
    /// ```
    #[must_use]
    pub fn sum(&self) -> f64 {
        self.data.iter().sum()
    }

    /// Returns the arithmetic mean of all elements (`sum / len`).
    ///
    /// `NaN` propagates. Behaviour on an empty tensor is unspecified
    /// (zero-sized dims are rejected by constructors in Phase 1).
    ///
    /// ```
    /// use matten::Tensor;
    /// assert_eq!(Tensor::from_vec(vec![1.0, 2.0, 3.0, 4.0]).mean(), 2.5);
    /// ```
    #[must_use]
    pub fn mean(&self) -> f64 {
        self.sum() / self.data.len() as f64
    }

    /// Returns the minimum element.
    ///
    /// Returns `NaN` if **any** element is `NaN` (explicit NaN-propagation;
    /// do not use `f64::min` which silently ignores NaN).
    ///
    /// ```
    /// use matten::Tensor;
    /// assert_eq!(Tensor::from_vec(vec![3.0, 1.0, 2.0]).min(), 1.0);
    /// assert!(Tensor::from_vec(vec![1.0, f64::NAN, 3.0]).min().is_nan());
    /// ```
    #[must_use]
    pub fn min(&self) -> f64 {
        nan_reduce(&self.data, f64::INFINITY, |acc, v| acc.min(v))
    }

    /// Returns the maximum element.
    ///
    /// Returns `NaN` if **any** element is `NaN`.
    ///
    /// ```
    /// use matten::Tensor;
    /// assert_eq!(Tensor::from_vec(vec![3.0, 1.0, 2.0]).max(), 3.0);
    /// assert!(Tensor::from_vec(vec![1.0, f64::NAN, 3.0]).max().is_nan());
    /// ```
    #[must_use]
    pub fn max(&self) -> f64 {
        nan_reduce(&self.data, f64::NEG_INFINITY, |acc, v| acc.max(v))
    }
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
    /// # Panics
    ///
    /// Panics if `axis >= self.ndim()`.
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
        axis_reduce(self, axis, "sum_axis", |acc, v| acc + v, 0.0)
    }

    /// Reduces along `axis` by computing the arithmetic mean.
    ///
    /// # Panics
    ///
    /// Panics if `axis >= self.ndim()`.
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
        let n = self.shape()[axis] as f64;
        let sums = axis_reduce(self, axis, "mean_axis", |acc, v| acc + v, 0.0);
        &sums / n
    }
}

// ── Min/max axis reductions ───────────────────────────────────────────────

impl Tensor {
    /// Reduces along `axis` by taking the minimum, removing that axis from the
    /// output shape. Returns `NaN` if any element along the axis is `NaN`.
    ///
    /// # Panics
    ///
    /// Panics if `axis >= self.ndim()`.
    ///
    /// ```
    /// use matten::Tensor;
    /// let m = Tensor::new(vec![3.0,1.0,4.0,1.0,5.0,9.0], &[2,3]);
    /// assert_eq!(m.min_axis(0).as_slice(), &[1.0, 1.0, 4.0]);
    /// ```
    #[must_use]
    pub fn min_axis(&self, axis: usize) -> Tensor {
        nan_axis_reduce(self, axis, "min_axis", f64::INFINITY, |a, b| a.min(b))
    }

    /// Reduces along `axis` by taking the maximum, removing that axis from the
    /// output shape. Returns `NaN` if any element along the axis is `NaN`.
    ///
    /// # Panics
    ///
    /// Panics if `axis >= self.ndim()`.
    ///
    /// ```
    /// use matten::Tensor;
    /// let m = Tensor::new(vec![3.0,1.0,4.0,1.0,5.0,9.0], &[2,3]);
    /// assert_eq!(m.max_axis(0).as_slice(), &[3.0, 5.0, 9.0]);
    /// ```
    #[must_use]
    pub fn max_axis(&self, axis: usize) -> Tensor {
        nan_axis_reduce(self, axis, "max_axis", f64::NEG_INFINITY, |a, b| a.max(b))
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
            "matten shape error in {operation}: axis {axis} is out of range \
             for rank-{} tensor",
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
    /// Panics on incompatible shapes or unsupported rank combinations.
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
        matmul_dispatch(self, rhs, "matmul")
    }
}

fn matmul_dispatch(lhs: &Tensor, rhs: &Tensor, op: &'static str) -> Tensor {
    match (lhs.ndim(), rhs.ndim()) {
        (1, 1) => vv_dot(lhs, rhs, op),
        (2, 1) => mv_mul(lhs, rhs, op),
        (1, 2) => vm_mul(lhs, rhs, op),
        (2, 2) => mm_mul(lhs, rhs, op),
        _ => panic!(
            "matten shape error in {op}: unsupported rank combination \
             (left rank {}, right rank {}); supported: [n]×[n], [m,n]×[n], \
             [n]×[n,p], [m,n]×[n,p]",
            lhs.ndim(),
            rhs.ndim()
        ),
    }
}

/// `[n] · [n] -> []` scalar tensor.
fn vv_dot(a: &Tensor, b: &Tensor, op: &'static str) -> Tensor {
    let n = a.len();
    if b.len() != n {
        panic!(
            "matten shape error in {op}: vector lengths must match \
             (left {n}, right {})",
            b.len()
        );
    }
    let v: f64 = a.data.iter().zip(&b.data).map(|(x, y)| x * y).sum();
    Tensor::scalar(v)
}

/// `[m, n] × [n] -> [m]`.
fn mv_mul(a: &Tensor, b: &Tensor, op: &'static str) -> Tensor {
    let [m, n] = shape2(a, op);
    dim_check(n, b.len(), "left columns", "right length", op);
    let mut out = vec![0.0f64; m];
    for (i, o) in out.iter_mut().enumerate() {
        for k in 0..n {
            *o += a.data[i * n + k] * b.data[k];
        }
    }
    Tensor {
        data: out,
        shape: vec![m],
        #[cfg(feature = "dynamic")]
        dynamic: None,
    }
}

/// `[n] × [n, p] -> [p]`.
fn vm_mul(a: &Tensor, b: &Tensor, op: &'static str) -> Tensor {
    let [n, p] = shape2(b, op);
    dim_check(a.len(), n, "left length", "right rows", op);
    let mut out = vec![0.0f64; p];
    for k in 0..n {
        for (j, slot) in out.iter_mut().enumerate() {
            *slot += a.data[k] * b.data[k * p + j];
        }
    }
    Tensor {
        data: out,
        shape: vec![p],
        #[cfg(feature = "dynamic")]
        dynamic: None,
    }
}

/// `[m, n] × [n, p] -> [m, p]`.
fn mm_mul(a: &Tensor, b: &Tensor, op: &'static str) -> Tensor {
    let [m, n] = shape2(a, op);
    let [nb, p] = shape2(b, op);
    dim_check(n, nb, "left columns", "right rows", op);
    let mut out = vec![0.0f64; m * p];
    for (i, row) in out.chunks_mut(p).enumerate() {
        for (j, slot) in row.iter_mut().enumerate() {
            let mut acc = 0.0f64;
            for k in 0..n {
                acc += a.data[i * n + k] * b.data[k * p + j];
            }
            *slot = acc;
        }
    }
    Tensor {
        data: out,
        shape: vec![m, p],
        #[cfg(feature = "dynamic")]
        dynamic: None,
    }
}

/// Extracts shape `[d0, d1]` from a rank-2 tensor; panics on wrong rank.
fn shape2(t: &Tensor, op: &'static str) -> [usize; 2] {
    match t.shape() {
        [a, b] => [*a, *b],
        s => panic!("matten shape error in {op}: expected rank-2 tensor, got shape {s:?}"),
    }
}

/// Panics with an actionable message when two dimensions that must be equal differ.
fn dim_check(left: usize, right: usize, left_name: &str, right_name: &str, op: &'static str) {
    if left != right {
        panic!(
            "matten shape error in {op}: {left_name} ({left}) \
             must equal {right_name} ({right})"
        );
    }
}

/// Public Result-zone wrapper for matmul used by future boundary APIs.
#[allow(dead_code)]
pub(crate) fn try_matmul(
    lhs: &Tensor,
    rhs: &Tensor,
    op: &'static str,
) -> Result<Tensor, MattenError> {
    // Catch panics from matmul_dispatch via a thin closure so callers that want
    // Result (e.g. future RFC-009 extensions) can use it without catch_unwind.
    // For now this just validates the common rank cases and delegates.
    match (lhs.ndim(), rhs.ndim()) {
        (1, 1) | (2, 1) | (1, 2) | (2, 2) => Ok(matmul_dispatch(lhs, rhs, op)),
        _ => Err(MattenError::Shape {
            operation: op,
            message: format!(
                "unsupported rank combination (left rank {}, right rank {})",
                lhs.ndim(),
                rhs.ndim()
            ),
        }),
    }
}
