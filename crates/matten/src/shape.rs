//! Shape model: validation and row-major index helpers (RFC-003).
//!
//! Shapes are runtime `&[usize]`. A scalar is shape `[]` with exactly one
//! element. Layout is row-major. Every constructor validates a shape before a
//! [`Tensor`](crate::Tensor) is created, so an invalid shape is never stored.

use crate::error::MattenError;

/// Maximum supported rank for the current shape model.
///
/// This is a DX / parser-abuse guard, not a mathematical limit: shapes are
/// stored as `Vec<usize>`, so the cap can be relaxed by a later RFC.
// MAX_NDIM is defined in crate::limits and re-exported from there.
use crate::limits::{MAX_ELEMENTS, MAX_NDIM, MAX_REPRESENTABLE_DIMENSION};

/// Validates a shape and returns its logical element count.
///
/// Enforces, in order: the rank limit ([`MAX_NDIM`]), rejection of zero-sized
/// dimensions, and checked multiplication of the dimension lengths. Returns
/// [`MattenError::Shape`] for rank/zero-dimension problems and
/// [`MattenError::Allocation`] for product overflow.
pub(crate) fn validate_shape(
    shape: &[usize],
    operation: &'static str,
) -> Result<usize, MattenError> {
    if shape.len() > MAX_NDIM {
        return Err(MattenError::Shape {
            operation,
            message: format!(
                "rank {} exceeds the maximum supported rank of {MAX_NDIM} (shape {shape:?})",
                shape.len()
            ),
        });
    }
    checked_shape_len(shape, operation, MAX_ELEMENTS)
}

/// Computes the logical element count of a shape with checked arithmetic.
/// Does not enforce the rank limit.
///
/// Zero-sized dimensions are accepted (RFC-111): a shape containing a `0`
/// yields a length of `0`, arithmetically what it means. The empty product
/// (rank 0, a scalar) is `1`, not `0` — a scalar is never empty.
///
/// `max_dimension` bounds each **individual** dimension, not the product
/// (RFC-127). A zero dimension makes the product `0`, so the `checked_mul`
/// loop below can never overflow once one is present — before RFC-111, a
/// zero dimension was rejected outright, which incidentally bounded every
/// dimension; RFC-111 removed that rejection deliberately, and nothing
/// downstream re-bounded the surviving dimensions. Without this loop, a
/// shape like `[400_000_000_000, 0]` validates here (product `0`) and later
/// aborts the process when some other operation allocates based on the
/// surviving `400_000_000_000`-sized axis.
///
/// The effective bound is always `min(max_dimension, MAX_REPRESENTABLE_DIMENSION)`
/// (RFC-127 §5 review): a caller-supplied [`MattenLimits`](crate::MattenLimits)
/// can set `max_elements` arbitrarily high, including above `isize::MAX`, and
/// nothing else enforced that it stayed representable. Clamping here — not
/// merely documenting the assumption elsewhere — is what makes
/// `slice.rs`'s `usize_to_isize_saturating` unconditionally correct, rather
/// than correct only as long as every caller happens to respect a limit
/// nothing checked.
pub(crate) fn checked_shape_len(
    shape: &[usize],
    operation: &'static str,
    max_dimension: usize,
) -> Result<usize, MattenError> {
    let max_dimension = max_dimension.min(MAX_REPRESENTABLE_DIMENSION);
    for &dim in shape {
        if dim > max_dimension {
            return Err(MattenError::Allocation {
                requested_elements: dim,
                message: format!(
                    "dimension {dim} in shape {shape:?} exceeds the maximum \
                     supported single-dimension size of {max_dimension} in {operation}"
                ),
            });
        }
    }

    let mut len: usize = 1;
    for &dim in shape {
        len = len.checked_mul(dim).ok_or_else(|| MattenError::Allocation {
            requested_elements: usize::MAX,
            message: format!("shape {shape:?} overflows usize when computing the element count in {operation}"),
        })?;
    }
    Ok(len)
}

// The row-major index helpers below are part of the shape foundation (RFC-003
// §12.3, handoff PR-003-B). They are exercised by round-trip tests now and are
// consumed by indexing, reshape, and slicing in M3-M5, hence `dead_code` is
// allowed until then.

/// Row-major strides for `shape`: `stride_j = product(dims[j + 1 ..])`.
///
/// Assumes `shape` has already been validated (no overflow). For a scalar
/// shape `[]` this returns an empty vector.
#[allow(dead_code)]
pub(crate) fn strides_for_shape(shape: &[usize]) -> Vec<usize> {
    let mut strides = vec![0usize; shape.len()];
    let mut acc: usize = 1;
    for j in (0..shape.len()).rev() {
        strides[j] = acc;
        acc *= shape[j];
    }
    strides
}

/// Maps a multidimensional coordinate to a flat row-major index.
///
/// Returns `None` if the coordinate rank does not match the shape or any
/// component is out of bounds. A scalar coordinate `[]` maps to `0`.
#[allow(dead_code)]
pub(crate) fn coord_to_flat(coord: &[usize], shape: &[usize]) -> Option<usize> {
    if coord.len() != shape.len() {
        return None;
    }
    let strides = strides_for_shape(shape);
    let mut flat = 0usize;
    for (i, (&c, &dim)) in coord.iter().zip(shape).enumerate() {
        if c >= dim {
            return None;
        }
        flat += c * strides[i];
    }
    Some(flat)
}

/// Maps a flat row-major index back to a multidimensional coordinate.
///
/// Assumes `flat` is in bounds for `shape`. A scalar shape `[]` yields `[]`.
#[allow(dead_code)]
pub(crate) fn flat_to_coord(flat: usize, shape: &[usize]) -> Vec<usize> {
    let strides = strides_for_shape(shape);
    let mut coord = vec![0usize; shape.len()];
    let mut rem = flat;
    for (i, &stride) in strides.iter().enumerate() {
        coord[i] = rem / stride;
        rem %= stride;
    }
    coord
}

#[cfg(test)]
mod tests;
