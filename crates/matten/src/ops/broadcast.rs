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

/// Element count of an already-validated shape, or `None` on overflow. Used
/// only to establish the "combined input size" a genuine broadcast expansion
/// is measured against (RFC-132 §12.0.1) — the shape itself already exists on
/// an in-memory tensor, so overflow here is not expected in practice.
fn shape_len(shape: &[usize]) -> Option<usize> {
    shape.iter().try_fold(1usize, |acc, &d| acc.checked_mul(d))
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
    ///
    /// # Errors
    ///
    /// Returns [`MattenError::Allocation`] if computing the result's element
    /// count overflows `usize`. Also returns it if the result exceeds the
    /// default element budget, but **only when this broadcast is a genuine
    /// multiplicative expansion** — the result element count exceeds the
    /// combined (summed) element count of the two operands (RFC-132 §12.0.1,
    /// §13). A same-shape (or otherwise non-expanding) broadcast, such as
    /// `&big + &big`, is a copy of already-validated data, not a product, and
    /// must succeed regardless of size — that is the RFC's own worked
    /// example. A shape like `[1048576,1]` broadcast against `[1,1048576]`
    /// is the genuine-expansion case this budget check exists to catch
    /// before it reaches an uncatchable allocator abort.
    pub(crate) fn new(
        left_shape: &[usize],
        right_shape: &[usize],
        result_shape: &[usize],
    ) -> Result<Self, MattenError> {
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

        // Check for overflow unconditionally, then apply the default element budget
        // only for a genuine multiplicative expansion (see the doc comment above).
        let result_len: usize = {
            let n = result_shape
                .iter()
                .try_fold(1usize, |acc, &d| acc.checked_mul(d))
                .ok_or_else(|| MattenError::Allocation {
                    requested_elements: usize::MAX,
                    message: format!(
                        "broadcast result shape {result_shape:?} overflows usize when \
                         computing element count"
                    ),
                })?;
            let combined_input_len = shape_len(left_shape)
                .and_then(|l| shape_len(right_shape).and_then(|r| l.checked_add(r)));
            let is_genuine_expansion = match combined_input_len {
                Some(combined) => n > combined,
                None => true, // couldn't establish a safe combined size; guard defensively
            };
            if is_genuine_expansion {
                crate::limits::MattenLimits::default().check_elements(n, "broadcast")?;
            }
            n
        };
        Ok(Self {
            result_len,
            result_strides: strides_for_shape(result_shape),
            left_strides_bc: bc_strides(&lp),
            right_strides_bc: bc_strides(&rp),
        })
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
///
/// # Errors
///
/// Returns [`MattenError::Unsupported`] if either operand is a dynamic
/// tensor, [`MattenError::Broadcast`] if the shapes are incompatible, or
/// [`MattenError::Allocation`] if computing the result overflows or exceeds
/// the default element budget (RFC-132 §12.0; see [`BroadcastCtx::new`]).
pub(crate) fn try_apply_binary<F>(
    lhs: &crate::Tensor,
    rhs: &crate::Tensor,
    operation: &'static str,
    f: F,
) -> Result<crate::Tensor, MattenError>
where
    F: Fn(f64, f64) -> f64,
{
    #[cfg(feature = "dynamic")]
    if lhs.is_dynamic() || rhs.is_dynamic() {
        return Err(MattenError::Unsupported {
            operation,
            message: "element-wise arithmetic is not supported on dynamic tensors; call \
                      try_numeric() on each operand first"
                .to_string(),
        });
    }
    #[cfg(not(feature = "dynamic"))]
    let _ = operation; // only read inside the dynamic-only branch above

    let result_shape = broadcast_shape(lhs.shape(), rhs.shape())?;
    let ctx = BroadcastCtx::new(lhs.shape(), rhs.shape(), &result_shape)?;
    let ldata = lhs.as_slice();
    let rdata = rhs.as_slice();
    let mut data = Vec::with_capacity(ctx.result_len());
    for i in 0..ctx.result_len() {
        data.push(f(ldata[ctx.left_flat(i)], rdata[ctx.right_flat(i)]));
    }
    Ok(crate::Tensor::from_parts_checked(data, result_shape))
}

/// Converts a [`try_apply_binary`] failure into the historical panic text for
/// the `Add`/`Sub`/`Mul`/`Div` operators (RFC-129). Each operator delegates to
/// its `try_*` twin and calls this on `Err`, rather than duplicating the
/// reconstruction four times.
///
/// The panic text is preserved byte-for-byte from before RFC-129: the dynamic
/// and allocation-budget cases already display identically via their
/// `MattenError` variants (both carry the `operation`/message content that
/// produces the historical text), but [`MattenError::Broadcast`] carries no
/// `operation` field — its own `Display` is shared crate-wide and does not
/// say which operator failed — so that one case is reconstructed explicitly
/// rather than delegated to `panic!("{e}")`. This is the same panic-vs-Result
/// text asymmetry already documented for `dot`/`matmul` (RFC-099) and
/// mirrored deliberately, not fixed, in the shape playground (RFC-093/095).
pub(crate) fn panic_for_arithmetic(operation: &'static str, e: MattenError) -> ! {
    match e {
        MattenError::Broadcast { left, right } => panic!(
            "matten broadcast error in {operation}: shapes {left:?} and {right:?} are not compatible"
        ),
        other => panic!("{other}"),
    }
}

#[cfg(test)]
mod tests;
