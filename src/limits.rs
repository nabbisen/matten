//! Resource safety limits (RFC-018).
//!
//! [`MattenLimits`] is the single source of truth for all allocation and shape
//! bounds in `matten`. The former scattered constants (`MAX_NDIM`,
//! `ARANGE_MAX_ELEMENTS`, etc.) now live here as the default values and are
//! re-exported for internal use.

use crate::{MattenError, Tensor};

// ── Default values (internal; kept as named constants for clarity) ─────────

/// Maximum number of dimensions (axes) a tensor may have.
pub(crate) const MAX_NDIM: usize = 8;

/// Maximum number of elements allowed by `arange` and by construction helpers.
pub(crate) const MAX_ELEMENTS: usize = 1 << 20; // ~1 M elements / ~8 MiB f64

/// Maximum number of elements the JSON parser will accept per array dimension.
#[cfg(feature = "json")]
pub(crate) const MAX_JSON_ELEMENTS: usize = 1 << 24; // 16 M — generous PoC bound

/// Maximum number of elements the dynamic JSON parser will accept.
#[cfg(feature = "dynamic")]
pub(crate) const MAX_DYNAMIC_ELEMENTS: usize = 1 << 24;

/// Maximum byte length accepted by the `slice_str` parser.
pub(crate) const MAX_SLICE_STR_BYTES: usize = 512;

/// Maximum byte length accepted by JSON and CSV parsers.
pub(crate) const MAX_PARSE_BYTES: usize = 128 * 1024 * 1024; // 128 MiB

// ── Public struct ──────────────────────────────────────────────────────────

/// Resource safety limits for shape calculations and allocations (RFC-018).
///
/// `MattenLimits` is the single source of truth for all allocation budgets in
/// `matten`. The default values are generous for typical PoC workloads but
/// prevent pathological resource exhaustion from malformed or adversarial
/// inputs.
///
/// # Examples
///
/// ```
/// use matten::MattenLimits;
///
/// let limits = MattenLimits::default();
/// assert_eq!(limits.max_dimensions, 8);
/// ```
///
/// Boundary-safe constructors (`try_zeros`, `try_ones`, `try_full`) use the
/// default limits automatically. Pass a custom `MattenLimits` to
/// [`Tensor::try_zeros_with_limits`] etc. if you need a different budget.
#[derive(Debug, Clone, PartialEq)]
pub struct MattenLimits {
    /// Maximum number of axes (rank). Default: 8.
    pub max_dimensions: usize,
    /// Maximum number of elements any constructor or parser may allocate.
    /// Default: 1 048 576 (~1 M, ~8 MiB for f64).
    pub max_elements: usize,
    /// Maximum number of bytes a JSON or CSV parser will accept before
    /// rejecting the input. Default: 128 MiB.
    pub max_parse_bytes: usize,
}

impl Default for MattenLimits {
    fn default() -> Self {
        Self {
            max_dimensions: MAX_NDIM,
            max_elements: MAX_ELEMENTS,
            max_parse_bytes: MAX_PARSE_BYTES,
        }
    }
}

impl MattenLimits {
    /// A very restrictive limit set useful for fuzz / threat-model tests.
    pub fn strict() -> Self {
        Self {
            max_dimensions: 4,
            max_elements: 1024,
            max_parse_bytes: 64 * 1024,
        }
    }

    /// Checks that `requested` does not exceed `max_elements`, returning a
    /// clear `MattenError::Allocation` on failure.
    pub(crate) fn check_elements(
        &self,
        requested: usize,
        operation: &'static str,
    ) -> Result<(), MattenError> {
        if requested > self.max_elements {
            Err(MattenError::Allocation {
                requested_elements: requested,
                message: format!(
                    "{operation} requested {requested} elements, exceeding the \
                     limit of {} (MattenLimits::max_elements); use smaller shapes \
                     or increase the limit",
                    self.max_elements
                ),
            })
        } else {
            Ok(())
        }
    }

    /// Checks rank and element count for a shape, using this limit object.
    pub(crate) fn check_shape(
        &self,
        shape: &[usize],
        operation: &'static str,
    ) -> Result<usize, MattenError> {
        if shape.len() > self.max_dimensions {
            return Err(MattenError::Shape {
                operation,
                message: format!(
                    "rank {} exceeds the maximum supported rank of {} (shape {shape:?})",
                    shape.len(),
                    self.max_dimensions
                ),
            });
        }
        let len = crate::shape::checked_shape_len(shape, operation)?;
        self.check_elements(len, operation)?;
        Ok(len)
    }
}

// ── Boundary-safe constructors ─────────────────────────────────────────────

impl Tensor {
    /// Creates a zero tensor, returning an error instead of panicking.
    ///
    /// Uses the default [`MattenLimits`]. For a custom budget use
    /// [`try_zeros_with_limits`](Tensor::try_zeros_with_limits).
    ///
    /// # Errors
    ///
    /// Returns [`MattenError`] for invalid shape, overflow, or exceeding the
    /// default element budget.
    ///
    /// # Examples
    ///
    /// ```
    /// use matten::Tensor;
    ///
    /// let t = Tensor::try_zeros(&[3, 4]).unwrap();
    /// assert_eq!(t.shape(), &[3, 4]);
    /// assert_eq!(t.as_slice(), &[0.0f64; 12]);
    /// ```
    pub fn try_zeros(shape: &[usize]) -> Result<Tensor, MattenError> {
        Tensor::try_zeros_with_limits(shape, &MattenLimits::default())
    }

    /// Creates a zero tensor with explicit limits.
    pub fn try_zeros_with_limits(
        shape: &[usize],
        limits: &MattenLimits,
    ) -> Result<Tensor, MattenError> {
        let len = limits.check_shape(shape, "try_zeros")?;
        Ok(Tensor {
            data: vec![0.0f64; len],
            shape: shape.to_vec(),
            #[cfg(feature = "dynamic")]
            dynamic: None,
        })
    }

    /// Creates a ones tensor, returning an error instead of panicking.
    ///
    /// Uses the default [`MattenLimits`].
    ///
    /// # Examples
    ///
    /// ```
    /// use matten::Tensor;
    ///
    /// let t = Tensor::try_ones(&[2, 3]).unwrap();
    /// assert_eq!(t.as_slice(), &[1.0f64; 6]);
    /// ```
    pub fn try_ones(shape: &[usize]) -> Result<Tensor, MattenError> {
        Tensor::try_ones_with_limits(shape, &MattenLimits::default())
    }

    /// Creates a ones tensor with explicit limits.
    pub fn try_ones_with_limits(
        shape: &[usize],
        limits: &MattenLimits,
    ) -> Result<Tensor, MattenError> {
        let len = limits.check_shape(shape, "try_ones")?;
        Ok(Tensor {
            data: vec![1.0f64; len],
            shape: shape.to_vec(),
            #[cfg(feature = "dynamic")]
            dynamic: None,
        })
    }

    /// Creates a tensor filled with `value`, returning an error instead of panicking.
    ///
    /// Uses the default [`MattenLimits`].
    ///
    /// # Examples
    ///
    /// ```
    /// use matten::Tensor;
    ///
    /// let t = Tensor::try_full(&[2, 2], 7.0).unwrap();
    /// assert_eq!(t.as_slice(), &[7.0f64; 4]);
    /// ```
    pub fn try_full(shape: &[usize], value: f64) -> Result<Tensor, MattenError> {
        Tensor::try_full_with_limits(shape, value, &MattenLimits::default())
    }

    /// Creates a filled tensor with explicit limits.
    pub fn try_full_with_limits(
        shape: &[usize],
        value: f64,
        limits: &MattenLimits,
    ) -> Result<Tensor, MattenError> {
        let len = limits.check_shape(shape, "try_full")?;
        Ok(Tensor {
            data: vec![value; len],
            shape: shape.to_vec(),
            #[cfg(feature = "dynamic")]
            dynamic: None,
        })
    }
}
