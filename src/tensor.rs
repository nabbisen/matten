//! The primary public container, [`Tensor`].
//!
//! M2 adds the full creation and conversion surface from RFC-004: fill
//! constructors, `from_vec`, `into_vec`, `arange`/`try_arange`, and
//! `try_from_rows`. Arithmetic, reshape, and I/O arrive in later milestones.

use crate::convert::flatten_rectangular;
use crate::error::MattenError;
use crate::shape;
use std::fmt;

/// Maximum element count accepted by `arange` / `try_arange` before the
/// allocation limit fires. Set conservatively for Phase 1.
use crate::limits::MattenLimits;

/// A dense, row-major, owned multidimensional array of `f64`.
///
/// Fields are private. The storage layout is never exposed across the public
/// API. A scalar is shape `[]` with one element.
///
/// # Examples
///
/// ```
/// use matten::Tensor;
///
/// // 2×2 matrix
/// let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
/// assert!(m.is_matrix());
///
/// // Fill constructors
/// let z = Tensor::zeros(&[3]);
/// assert_eq!(z.as_slice(), &[0.0, 0.0, 0.0]);
///
/// // Range constructor
/// let r = Tensor::arange(0.0, 5.0, 1.0);
/// assert_eq!(r.len(), 5);
/// ```
#[derive(Clone)]
pub struct Tensor {
    pub(crate) data: Vec<f64>,
    pub(crate) shape: Vec<usize>,
    /// Phase 2 dynamic storage; `None` for Phase 1 numeric tensors.
    #[cfg(feature = "dynamic")]
    pub(crate) dynamic: Option<Box<crate::dynamic::storage::DynamicTensor>>,
}

impl PartialEq for Tensor {
    fn eq(&self, other: &Self) -> bool {
        // Compare Phase 1 fields. Dynamic tensors with same shape and logical
        // data are equal; both dynamic fields must be None or produce the same
        // element sequence.
        if self.shape != other.shape {
            return false;
        }
        #[cfg(feature = "dynamic")]
        {
            match (&self.dynamic, &other.dynamic) {
                (Some(a), Some(b)) => return a.to_vec() == b.to_vec(),
                (None, None) => {}
                _ => return false,
            }
        }
        self.data == other.data
    }
}

// `is_empty` is absent while zero-sized dimensions are rejected: zero-sized dims are rejected and a scalar
// has `len() == 1`, so it would always be false. Deferred to a future
// zero-sized-tensor RFC.
#[allow(clippy::len_without_is_empty)]
impl Tensor {
    // ------------------------------------------------------------------ //
    // Core constructors (also implemented in M1; repeated here for clarity)
    // ------------------------------------------------------------------ //

    /// Creates a tensor from row-major `data` and `shape`.
    ///
    /// Panics with an actionable message on any shape violation or data-length
    /// mismatch. Use [`try_new`](Tensor::try_new) for recoverable construction.
    ///
    /// # Panics
    ///
    /// Panics if the shape is invalid or `data.len()` does not equal the shape
    /// product.
    /// Panics with a clear `matten unsupported error` message if this tensor
    /// uses dynamic (`Element`) storage. Used to guard all Phase 1 numeric
    /// accessors and conversions.
    #[cfg(feature = "dynamic")]
    #[inline(always)]
    pub(crate) fn panic_if_dynamic(&self, operation: &'static str) {
        if self.is_dynamic() {
            panic!(
                "matten unsupported error in {operation}: this numeric API is not supported on dynamic tensors; use to_elements() or try_numeric() first"
            );
        }
    }

    #[must_use]
    pub fn new(data: Vec<f64>, shape: &[usize]) -> Tensor {
        Self::try_new(data, shape).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Creates a tensor from row-major `data` and `shape`, returning an error
    /// instead of panicking.
    ///
    /// # Errors
    ///
    /// Returns [`MattenError::Shape`] for a length mismatch or invalid shape,
    /// or [`MattenError::Allocation`] on product overflow.
    pub fn try_new(data: Vec<f64>, shape: &[usize]) -> Result<Tensor, MattenError> {
        let expected = shape::validate_shape(shape, "try_new")?;
        if data.len() != expected {
            return Err(MattenError::Shape {
                operation: "try_new",
                message: format!(
                    "data length {} does not match shape {shape:?}, which requires {expected} elements",
                    data.len()
                ),
            });
        }
        Ok(Tensor {
            data,
            shape: shape.to_vec(),
            #[cfg(feature = "dynamic")]
            dynamic: None,
        })
    }

    /// Creates a rank-0 scalar tensor (shape `[]`, length `1`).
    ///
    /// ```
    /// use matten::Tensor;
    /// let s = Tensor::scalar(3.14);
    /// assert!(s.is_scalar());
    /// assert_eq!(s.len(), 1);
    /// ```
    #[must_use]
    pub fn scalar(value: f64) -> Tensor {
        // Shape `[]` is always valid: rank 0, no dims, product 1.
        Tensor {
            data: vec![value],
            shape: Vec::new(),
            #[cfg(feature = "dynamic")]
            dynamic: None,
        }
    }

    // ------------------------------------------------------------------ //
    // Fill constructors
    // ------------------------------------------------------------------ //

    /// Creates a tensor filled with `0.0`.
    ///
    /// # Panics
    ///
    /// Panics on an invalid shape (same rules as [`new`](Tensor::new)).
    ///
    /// ```
    /// use matten::Tensor;
    /// let z = Tensor::zeros(&[2, 3]);
    /// assert_eq!(z.shape(), &[2, 3]);
    /// assert_eq!(z.len(), 6);
    /// assert!(z.as_slice().iter().all(|&v| v == 0.0));
    /// ```
    #[must_use]
    pub fn zeros(shape: &[usize]) -> Tensor {
        Self::try_zeros(shape).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Creates a tensor filled with `1.0`.
    ///
    /// # Panics
    ///
    /// Panics on an invalid shape.
    ///
    /// ```
    /// use matten::Tensor;
    /// let o = Tensor::ones(&[3]);
    /// assert!(o.as_slice().iter().all(|&v| v == 1.0));
    /// ```
    #[must_use]
    pub fn ones(shape: &[usize]) -> Tensor {
        Self::try_ones(shape).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Creates a tensor filled with `value`.
    ///
    /// # Panics
    ///
    /// Panics on an invalid shape.
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::full(&[2, 2], 7.0);
    /// assert!(t.as_slice().iter().all(|&v| v == 7.0));
    /// ```
    #[must_use]
    pub fn full(shape: &[usize], value: f64) -> Tensor {
        Self::try_full(shape, value).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Creates a 1-D tensor from a flat vector; shape is `[data.len()]`.
    ///
    /// # Panics
    ///
    /// Panics if `data` is empty (zero-sized dimension).
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    /// assert_eq!(t.shape(), &[3]);
    /// ```
    #[must_use]
    pub fn from_vec(data: Vec<f64>) -> Tensor {
        let len = data.len();
        Tensor::new(data, &[len])
    }

    // ------------------------------------------------------------------ //
    // Range constructor
    // ------------------------------------------------------------------ //

    /// Creates a 1-D tensor with values `start, start + step, ...` (half-open,
    /// so `end` is excluded).
    ///
    /// This is a panic-zone convenience. Use [`try_arange`](Tensor::try_arange)
    /// when `start`/`end`/`step` come from user input.
    ///
    /// # Panics
    ///
    /// Panics if `step == 0.0`, any argument is non-finite, or the computed
    /// element count would exceed the allocation limit.
    ///
    /// ```
    /// use matten::Tensor;
    /// let r = Tensor::arange(0.0, 5.0, 1.0);
    /// assert_eq!(r.as_slice(), &[0.0, 1.0, 2.0, 3.0, 4.0]);
    ///
    /// let r2 = Tensor::arange(1.0, 0.0, -0.5);
    /// assert_eq!(r2.as_slice(), &[1.0, 0.5]);
    /// ```
    #[must_use]
    pub fn arange(start: f64, end: f64, step: f64) -> Tensor {
        arange_impl(start, end, step, "arange").unwrap_or_else(|e| panic!("{e}"))
    }

    /// Creates a 1-D tensor with values `start, start + step, ...` (half-open),
    /// returning an error instead of panicking.
    ///
    /// # Errors
    ///
    /// Returns [`MattenError::Shape`] for a zero or non-finite step/bound, or
    /// [`MattenError::Allocation`] if the computed element count exceeds the
    /// allocation limit.
    pub fn try_arange(start: f64, end: f64, step: f64) -> Result<Tensor, MattenError> {
        arange_impl(start, end, step, "try_arange")
    }

    // ------------------------------------------------------------------ //
    // Nested-row construction
    // ------------------------------------------------------------------ //

    /// Creates a 2-D tensor from rectangular row data; shape is `[rows, cols]`.
    ///
    /// This is the recoverable version of [`From<Vec<Vec<f64>>>`].
    ///
    /// # Errors
    ///
    /// Returns [`MattenError::Shape`] on an empty outer vector, any zero-length
    /// row, or ragged rows.
    pub fn try_from_rows(rows: Vec<Vec<f64>>) -> Result<Tensor, MattenError> {
        let (data, shape) = flatten_rectangular(rows, "try_from_rows")?;
        Ok(Tensor {
            data,
            shape,
            #[cfg(feature = "dynamic")]
            dynamic: None,
        })
    }

    // ------------------------------------------------------------------ //
    // Observational API (also M1; listed here for doc completeness)
    // ------------------------------------------------------------------ //

    /// The shape as a slice of dimension lengths. Non-allocating.
    #[must_use]
    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    /// The rank (number of dimensions): `shape().len()`.
    #[must_use]
    pub fn ndim(&self) -> usize {
        self.shape.len()
    }

    /// The logical element count: the product of the shape.
    #[must_use]
    pub fn len(&self) -> usize {
        #[cfg(feature = "dynamic")]
        if let Some(dyn_t) = &self.dynamic {
            return dyn_t.len;
        }
        self.data.len()
    }

    /// Returns `true` for a rank-0 scalar tensor (shape `[]`).
    #[must_use]
    pub fn is_scalar(&self) -> bool {
        self.ndim() == 0
    }

    /// Returns `true` for a rank-1 tensor.
    #[must_use]
    pub fn is_vector(&self) -> bool {
        self.ndim() == 1
    }

    /// Returns `true` for a rank-2 tensor.
    #[must_use]
    pub fn is_matrix(&self) -> bool {
        self.ndim() == 2
    }

    /// The flat, row-major data as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[f64] {
        #[cfg(feature = "dynamic")]
        self.panic_if_dynamic("as_slice");
        &self.data
    }

    /// Returns an owned copy of the flat, row-major data.
    #[must_use]
    pub fn to_vec(&self) -> Vec<f64> {
        #[cfg(feature = "dynamic")]
        self.panic_if_dynamic("to_vec");
        self.data.clone()
    }

    /// Consumes the tensor and returns the flat, row-major data. No copy.
    #[must_use]
    pub fn into_vec(self) -> Vec<f64> {
        #[cfg(feature = "dynamic")]
        self.panic_if_dynamic("into_vec");
        self.data
    }
}

impl fmt::Debug for Tensor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const MAX: usize = 8;
        #[cfg(feature = "dynamic")]
        if let Some(dyn_t) = &self.dynamic {
            write!(f, "Tensor(shape={:?}, dynamic=[", self.shape)?;
            for i in 0..dyn_t.len.min(MAX) {
                if i > 0 {
                    f.write_str(", ")?;
                }
                if let Some(e) = dyn_t.get_flat(i) {
                    write!(f, "{e:?}")?;
                }
            }
            if dyn_t.len > MAX {
                write!(f, ", ... ({} more)", dyn_t.len - MAX)?;
            }
            return f.write_str("])");
        }
        write!(f, "Tensor(shape={:?}, data=[", self.shape)?;
        for (i, v) in self.data.iter().take(MAX).enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            write!(f, "{v:?}")?;
        }
        if self.data.len() > MAX {
            write!(f, ", ... ({} more)", self.data.len() - MAX)?;
        }
        f.write_str("])")
    }
}

// ---- arange shared implementation ------------------------------------

/// Shared implementation for both `arange` and `try_arange`.
fn arange_impl(
    start: f64,
    end: f64,
    step: f64,
    operation: &'static str,
) -> Result<Tensor, MattenError> {
    // Argument validation (Result zone, so return Err rather than panic).
    if !start.is_finite() || !end.is_finite() {
        return Err(MattenError::Shape {
            operation,
            message: format!("start and end must be finite (got start={start}, end={end})"),
        });
    }
    if step == 0.0 || !step.is_finite() {
        return Err(MattenError::Shape {
            operation,
            message: format!("step must be a non-zero finite value (got {step})"),
        });
    }

    // Compute element count without allocating. The estimate is conservative:
    // we use integer ceiling so floating-point accumulation never drops a value
    // that should be included.
    let raw_count = ((end - start) / step).ceil();
    let count: usize = if raw_count <= 0.0 {
        0
    } else if raw_count > MattenLimits::default().max_elements as f64 {
        return Err(MattenError::Allocation {
            requested_elements: raw_count as usize,
            message: format!(
                "arange would produce ~{} elements, exceeding the limit of {}",
                raw_count as usize,
                MattenLimits::default().max_elements
            ),
        });
    } else {
        raw_count as usize
    };

    // Build the data vector by accumulation. Recompute to avoid drift near
    // `end`, but exclude values that have genuinely crossed `end`.
    let mut data = Vec::with_capacity(count);
    let mut i: usize = 0;
    loop {
        let v = start + step * i as f64;
        // Half-open: stop when we cross `end` in the direction of `step`.
        if (step > 0.0 && v >= end) || (step < 0.0 && v <= end) {
            break;
        }
        data.push(v);
        i += 1;
        // Guard against the estimate being too small (floating-point edge).
        if i > MattenLimits::default().max_elements {
            return Err(MattenError::Allocation {
                requested_elements: i,
                message: format!(
                    "arange exceeded the element limit of {}",
                    MattenLimits::default().max_elements
                ),
            });
        }
    }

    let len = data.len();
    // A zero-element range is an edge case we permit as an empty 1-D tensor
    // shape would be [0], which the shape module rejects. Return an error
    // so the user knows to adjust their range arguments.
    if len == 0 {
        return Err(MattenError::Shape {
            operation,
            message: format!("arange(start={start}, end={end}, step={step}) produces no elements"),
        });
    }

    Ok(Tensor {
        data,
        shape: vec![len],
        #[cfg(feature = "dynamic")]
        dynamic: None,
    })
}

// Shape operations, slicing, and boundary integration (split per 300-ELOC rule).
mod ops;
