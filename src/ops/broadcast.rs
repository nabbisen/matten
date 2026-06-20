//! Broadcasting shape computation and index mapping (RFC-006 §12.1-12.2).
//!
//! Rules (right-aligned NumPy-style):
//! - equal dimensions are compatible;
//! - one dimension equal to `1` broadcasts to the other;
//! - a missing leading dimension is treated as `1`.
//!
//! Incompatible pairs produce [`MattenError::Broadcast`], which operators
//! panic-format.

use crate::error::MattenError;
use crate::shape::strides_for_shape;

/// Computes the broadcast result shape for two shapes, or returns
/// [`MattenError::Broadcast`] if they are incompatible.
pub(crate) fn broadcast_shape(left: &[usize], right: &[usize]) -> Result<Vec<usize>, MattenError> {
    let out_rank = left.len().max(right.len());
    let mut result = vec![0usize; out_rank];

    for (i, slot) in result.iter_mut().enumerate() {
        // Map backwards: each operand contributes 1 if that axis is missing.
        let l = left
            .len()
            .checked_sub(out_rank - i)
            .map_or(1, |idx| left[idx]);
        let r = right
            .len()
            .checked_sub(out_rank - i)
            .map_or(1, |idx| right[idx]);
        *slot = match (l, r) {
            (a, b) if a == b => a,
            (1, b) => b,
            (a, 1) => a,
            _ => {
                return Err(MattenError::Broadcast {
                    left: left.to_vec(),
                    right: right.to_vec(),
                });
            }
        };
    }
    Ok(result)
}

/// Precomputed context for iterating a broadcast operation without allocating
/// per-element coordinate vectors.
pub(crate) struct BroadcastCtx {
    result_len: usize,
    result_strides: Vec<usize>,
    left_strides_bc: Vec<usize>, // 0 where operand dim was 1 (repeat that element)
    right_strides_bc: Vec<usize>,
}

impl BroadcastCtx {
    /// Build a context from the two operand shapes and the already-computed
    /// result shape.
    pub(crate) fn new(left_shape: &[usize], right_shape: &[usize], result_shape: &[usize]) -> Self {
        let rank = result_shape.len();

        // Pad a shape on the left with 1s to reach `rank`.
        let pad_left = |s: &[usize]| -> Vec<usize> {
            let mut v = vec![1usize; rank];
            v[rank - s.len()..].copy_from_slice(s);
            v
        };
        let lp = pad_left(left_shape);
        let rp = pad_left(right_shape);

        // Natural row-major strides, then zero out any axis whose padded dim is
        // 1 — that axis is broadcast, so the flat index doesn't advance.
        let bc_strides = |padded: &[usize]| -> Vec<usize> {
            let nat = strides_for_shape(padded);
            padded
                .iter()
                .zip(&nat)
                .map(|(&d, &s)| if d == 1 { 0 } else { s })
                .collect()
        };

        // Check for overflow and apply the default element budget before allocating.
        let result_len: usize = {
            let n = result_shape
                .iter()
                .try_fold(1usize, |acc, &d| acc.checked_mul(d))
                .unwrap_or_else(|| {
                    panic!(
                        "matten broadcast error: broadcast result shape {result_shape:?} \
                         overflows usize when computing element count"
                    )
                });
            crate::limits::MattenLimits::default()
                .check_elements(n, "broadcast")
                .unwrap_or_else(|e| panic!("{e}"));
            n
        };
        Self {
            result_len,
            result_strides: strides_for_shape(result_shape),
            left_strides_bc: bc_strides(&lp),
            right_strides_bc: bc_strides(&rp),
        }
    }

    pub(crate) fn result_len(&self) -> usize {
        self.result_len
    }

    /// Maps a flat result index to the flat index into the left operand.
    #[inline]
    pub(crate) fn left_flat(&self, result_flat: usize) -> usize {
        self.operand_flat(result_flat, &self.left_strides_bc)
    }

    /// Maps a flat result index to the flat index into the right operand.
    #[inline]
    pub(crate) fn right_flat(&self, result_flat: usize) -> usize {
        self.operand_flat(result_flat, &self.right_strides_bc)
    }

    #[inline]
    fn operand_flat(&self, result_flat: usize, op_strides: &[usize]) -> usize {
        let mut rem = result_flat;
        let mut flat = 0usize;
        for (&rs, &os) in self.result_strides.iter().zip(op_strides) {
            let coord = rem / rs;
            rem %= rs;
            flat += coord * os;
        }
        flat
    }
}

/// Applies a binary `f64 → f64 → f64` function element-wise with broadcasting.
/// Panics on incompatible shapes.
pub(crate) fn apply_binary<F>(
    lhs: &crate::Tensor,
    rhs: &crate::Tensor,
    operation: &'static str,
    f: F,
) -> crate::Tensor
where
    F: Fn(f64, f64) -> f64,
{
    #[cfg(feature = "dynamic")]
    if lhs.is_dynamic() || rhs.is_dynamic() {
        panic!(
            "matten unsupported error in {operation}: element-wise arithmetic is not supported on dynamic tensors; call try_numeric() on each operand first"
        );
    }
    let result_shape = broadcast_shape(lhs.shape(), rhs.shape()).unwrap_or_else(|_| {
        panic!(
            "matten broadcast error in {operation}: shapes {:?} and {:?} are not compatible",
            lhs.shape(),
            rhs.shape()
        )
    });
    let ctx = BroadcastCtx::new(lhs.shape(), rhs.shape(), &result_shape);
    let ldata = lhs.as_slice();
    let rdata = rhs.as_slice();
    let mut data = Vec::with_capacity(ctx.result_len());
    for i in 0..ctx.result_len() {
        data.push(f(ldata[ctx.left_flat(i)], rdata[ctx.right_flat(i)]));
    }
    // SAFETY: result_shape was computed from validated shapes; product == data.len().
    crate::Tensor {
        data,
        shape: result_shape,
        #[cfg(feature = "dynamic")]
        dynamic: None,
    }
}

#[cfg(test)]
mod tests;
