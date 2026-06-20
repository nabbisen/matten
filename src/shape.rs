//! Shape model: validation and row-major index helpers (RFC-003).
//!
//! Shapes are runtime `&[usize]`. A scalar is shape `[]` with exactly one
//! element. Layout is row-major. Every constructor validates a shape before a
//! [`Tensor`](crate::Tensor) is created, so an invalid shape is never stored.

use crate::error::MattenError;

/// Maximum supported rank for `0.1.x`.
///
/// This is a DX / parser-abuse guard, not a mathematical limit: shapes are
/// stored as `Vec<usize>`, so the cap can be relaxed by a later RFC.
pub(crate) const MAX_NDIM: usize = 8;

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
    checked_shape_len(shape, operation)
}

/// Computes the logical element count of a shape with checked arithmetic,
/// rejecting zero-sized dimensions. Does not enforce the rank limit.
pub(crate) fn checked_shape_len(
    shape: &[usize],
    operation: &'static str,
) -> Result<usize, MattenError> {
    let mut len: usize = 1;
    for &dim in shape {
        if dim == 0 {
            return Err(MattenError::Shape {
                operation,
                message: format!(
                    "zero-sized dimensions are not supported in matten 0.1 (shape {shape:?})"
                ),
            });
        }
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
