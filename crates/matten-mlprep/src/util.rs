//! Shared input guard for `matten-mlprep` (RFC-028 §3, §6).

use crate::error::MattenMlprepError;
use matten::Tensor;

/// Validates that `x` is a numeric rank-2 tensor and returns `(rows, cols)`.
///
/// Enforces the `rows = samples`, `columns = features` convention for every
/// public entry point, and (under the `dynamic` feature) rejects dynamic tensors
/// with an error rather than letting core panic.
pub(crate) fn matrix_dims(x: &Tensor) -> Result<(usize, usize), MattenMlprepError> {
    #[cfg(feature = "dynamic")]
    if x.is_dynamic() {
        return Err(MattenMlprepError::DynamicTensor);
    }

    let shape = x.shape();
    if shape.len() != 2 {
        return Err(MattenMlprepError::ExpectedMatrix {
            shape: shape.to_vec(),
        });
    }
    Ok((shape[0], shape[1]))
}
