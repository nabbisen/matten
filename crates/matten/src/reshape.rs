//! Shape-transformation operations: reshape, flatten, transpose, swap axes
//! (RFC-007).
//!
//! Phase 1 rule: every returned [`Tensor`](crate::Tensor) is an independent
//! owned value. Operations copy/materialise data internally; no view lifetime
//! is ever exposed to the caller.

use crate::shape::{coord_to_flat, flat_to_coord, validate_shape};
use crate::{MattenError, Tensor};

// ---- reshape ------------------------------------------------------------

/// Validates that `new_shape` is compatible with `len` and returns the new
/// element count on success.
fn validate_reshape(
    len: usize,
    new_shape: &[usize],
    operation: &'static str,
) -> Result<usize, MattenError> {
    let new_len = validate_shape(new_shape, operation)?;
    if new_len != len {
        return Err(MattenError::Shape {
            operation,
            message: format!(
                "cannot reshape tensor with {len} elements into shape {new_shape:?} \
                 requiring {new_len} elements"
            ),
        });
    }
    Ok(new_len)
}

/// Reshape implementation shared by the panic and Result forms.
pub(crate) fn try_reshape_impl(t: &Tensor, new_shape: &[usize]) -> Result<Tensor, MattenError> {
    #[cfg(feature = "dynamic")]
    if t.is_dynamic() {
        return Err(MattenError::Unsupported {
            operation: "reshape",
            message: "dynamic tensors do not support reshape; \
                      call try_numeric() first to convert to a numeric tensor"
                .to_string(),
        });
    }
    validate_reshape(t.len(), new_shape, "reshape")?;
    Ok(Tensor {
        data: t.data.clone(),
        shape: new_shape.to_vec(),
        #[cfg(feature = "dynamic")]
        dynamic: None,
    })
}

// ---- axis permutation ---------------------------------------------------

/// Applies an axis permutation to `t`, producing a new owned row-major tensor.
///
/// `permutation[i]` gives the source axis for result axis `i`. The permutation
/// must be a valid bijection over `0..ndim`; the caller is responsible for
/// validation.
pub(crate) fn permute_axes(t: &Tensor, permutation: &[usize]) -> Tensor {
    #[cfg(feature = "dynamic")]
    if t.is_dynamic() {
        panic!(
            "matten unsupported error in transpose/swap_axes: \
             dynamic tensors do not support axis permutation; call try_numeric() first"
        );
    }
    let src_shape = t.shape();
    // Build result shape: result_shape[i] = src_shape[permutation[i]]
    let result_shape: Vec<usize> = permutation.iter().map(|&p| src_shape[p]).collect();
    let len = t.len();
    let mut result_data = vec![0.0f64; len];

    for src_flat in 0..len {
        let src_coord = flat_to_coord(src_flat, src_shape);
        // Permute: result_coord[i] = src_coord[permutation[i]]
        let result_coord: Vec<usize> = permutation.iter().map(|&p| src_coord[p]).collect();
        let dst_flat = coord_to_flat(&result_coord, &result_shape)
            .expect("permuted coordinate is always valid by construction");
        result_data[dst_flat] = t.data[src_flat];
    }
    Tensor {
        data: result_data,
        shape: result_shape,
        #[cfg(feature = "dynamic")]
        dynamic: None,
    }
}

/// Validates that `axis1` and `axis2` are both in-bounds for `ndim`, returning
/// a canonical `MattenError::Shape` on failure.
pub(crate) fn validate_axes(
    axis1: usize,
    axis2: usize,
    ndim: usize,
    operation: &'static str,
) -> Result<(), MattenError> {
    for ax in [axis1, axis2] {
        if ax >= ndim {
            return Err(MattenError::Shape {
                operation,
                message: format!("axis {ax} is out of range for rank-{ndim} tensor"),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
