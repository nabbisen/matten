//! Resource safety limits (RFC-018, revised RFC-132).
//!
//! [`MattenLimits`] bounds allocations sized by a value the **caller supplied
//! as data** — a shape passed to a constructor, a count passed to
//! `repeat`/`tile`, a document handed to a parser (RFC-132 §12.1). It does
//! **not** bound ordinary operations on already-validated, already-in-memory
//! data (arithmetic, reductions, matmul, slicing, concatenation) — that data
//! is the caller's own, and `matten` does not second-guess it.
//!
//! **Three exceptions keep a fixed, non-configurable ceiling regardless of
//! this rule**: `matmul`, `outer`, and elementwise-arithmetic broadcasting.
//! Their output size is the *product* of two independent already-validated
//! dimensions, not their sum or a subset of them, so neither operand's own
//! validation bounds it (RFC-132 §12.0 — an implementer-caught correction to
//! this RFC's original, broader "do not apply" list). These three always
//! check against the *default* budget; there is no way for a caller to raise
//! it for them.
//!
//! The former scattered constants (`MAX_NDIM`, `ARANGE_MAX_ELEMENTS`, etc.)
//! live here as the default values and are re-exported for internal use.

use crate::{MattenError, Tensor};

// ── Default values (internal; kept as named constants for clarity) ─────────

/// Maximum number of dimensions (axes) a tensor may have.
pub(crate) const MAX_NDIM: usize = 8;

/// Maximum number of elements allowed by `arange` and by construction helpers.
pub(crate) const MAX_ELEMENTS: usize = 1 << 20; // ~1 M elements / ~8 MiB f64

/// The largest single-dimension size any shape check may ever enforce,
/// regardless of a caller-supplied [`MattenLimits::max_elements`] (RFC-127
/// §5 review). Every `Tensor` element is `f64` (8 bytes), and Rust's largest
/// single allocation is `isize::MAX` bytes — no larger element count is ever
/// satisfiable by any allocation, so this is a hard ceiling, not a policy
/// choice. It is also what makes `slice.rs`'s `usize_to_isize_saturating`
/// correct: saturating an out-of-range `usize` to `isize::MAX` is guaranteed
/// to exceed this ceiling, and therefore any dimension a shape check has
/// actually accepted, unconditionally — not merely "as long as
/// `max_elements` stays below `isize::MAX`", which nothing previously
/// enforced.
pub(crate) const MAX_REPRESENTABLE_DIMENSION: usize = isize::MAX as usize / 8;

/// Maximum number of elements the JSON parser will accept per array dimension.
#[cfg(feature = "json")]
pub(crate) const MAX_JSON_ELEMENTS: usize = 1 << 24; // 16 M — generous PoC bound

/// Maximum number of elements the dynamic JSON parser will accept.
#[cfg(all(feature = "dynamic", feature = "json"))]
pub(crate) const MAX_DYNAMIC_ELEMENTS: usize = 1 << 24;

/// Maximum byte length accepted by the `slice_str` parser.
pub(crate) const MAX_SLICE_STR_BYTES: usize = 512;

/// Maximum byte length accepted by JSON and CSV parsers.
pub(crate) const MAX_PARSE_BYTES: usize = 128 * 1024 * 1024; // 128 MiB

// ── Public struct ──────────────────────────────────────────────────────────

/// Resource safety limits for shape calculations and allocations (RFC-018,
/// revised RFC-132).
///
/// `MattenLimits` bounds allocations sized by a caller-supplied value — a
/// shape, a count, a parsed document. It does not bound ordinary operations
/// on data already in memory and already validated: that data is the
/// caller's own. See the module docs for the model and its one deliberate
/// exception (`matmul`/`outer`/broadcast). The default values are generous
/// for typical PoC workloads but prevent pathological resource exhaustion
/// from malformed or adversarial inputs at the points where limits apply.
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
    /// Maximum number of elements a caller-supplied shape may produce.
    /// Default: 1 048 576 (~1 M, ~8 MiB for f64).
    ///
    /// Read by the three `_with_limits` constructors (`try_zeros_with_limits`
    /// etc.) — the caller-supplied value here governs those. `matmul`,
    /// `outer`, and arithmetic broadcasting also check against a value of
    /// this same *kind*, but always the **default**, never this instance's
    /// value: their output size is a product of two independent operands
    /// (RFC-132 §12.0), not a caller-supplied shape, so there is no way to
    /// raise their ceiling.
    ///
    /// Note: a 2048×2048 matrix has ~4 M elements and exceeds this default.
    /// This is an intentionally conservative safety-first default for PoC use.
    /// Use `try_zeros_with_limits` with a custom `MattenLimits` for larger tensors,
    /// or use the panicking `zeros`/`ones`/`full` only when you know the shape is safe.
    pub max_elements: usize,
    /// Maximum number of bytes `load_json`/`load_csv` and the string parsers
    /// (`from_json`, `from_csv`, `from_json_dynamic`, `from_csv_dynamic`)
    /// accept — the boundary control for untrusted JSON/CSV (RFC-132 §12.3).
    /// Default: 128 MiB.
    ///
    /// **Not read by any parser in this crate.** Core `matten`'s own parsers
    /// enforce the same budget via the internal `MAX_PARSE_BYTES` constant
    /// directly, not this field — there is no `_with_limits` form for
    /// parsers, so setting this on a `MattenLimits` instance has no effect
    /// on `load_json`/`load_csv`/etc. It exists as a documented budget and a
    /// future extension point, should a caller-configurable parser variant
    /// be added later.
    ///
    /// The field itself **is** read from outside this crate: `matten-data`'s
    /// `Table::from_csv_path` uses `MattenLimits::default().max_parse_bytes`
    /// directly (RFC-132 §12.1), since `MAX_PARSE_BYTES` is `pub(crate)` and
    /// not reachable from a companion crate.
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
    ///
    /// The message says only "use smaller shapes", not "or increase the
    /// limit" (RFC-132 §12.5 review correction): most callers of this check
    /// have no way to raise `max_elements` at all — only the three
    /// `_with_limits` constructors do, and their own doc comments already
    /// name that path. A remedy the caller cannot act on is worse than no
    /// remedy, especially in an error message read exactly when someone is
    /// already stuck.
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
                     limit of {} (MattenLimits::max_elements); use smaller shapes",
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
        let len = crate::shape::checked_shape_len(shape, operation, self.max_elements)?;
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
        Ok(Tensor::from_parts_checked(
            vec![0.0f64; len],
            shape.to_vec(),
        ))
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
        Ok(Tensor::from_parts_checked(
            vec![1.0f64; len],
            shape.to_vec(),
        ))
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
        Ok(Tensor::from_parts_checked(vec![value; len], shape.to_vec()))
    }
}
