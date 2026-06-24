//! Shape composition: joining tensors along an existing or a new axis (RFC-039).
//!
//! [`Tensor::concatenate`] joins tensors along an **existing** axis — all inputs
//! must share the same rank and the same size on every non-concatenation axis, and
//! the output axis size is the sum of the input axis sizes. [`Tensor::stack`] joins
//! **identically shaped** tensors along a **new** axis, so the output rank is the
//! input rank plus one and the new axis size is the number of inputs.
//!
//! Both take a borrowed slice `&[&Tensor]` (callers never have to clone just to pass
//! inputs), enforce [`MattenLimits`] on the output allocation, and reject dynamic
//! tensors — convert with [`Tensor::try_numeric`](crate::Tensor::try_numeric) first.
//! The `try_*` forms return [`MattenError`]; the convenience forms panic with the
//! same message.
//!
//! `repeat`, `tile`, and `meshgrid` are intentionally **deferred** (they need a
//! separate indexing/allocation policy; see RFC-039 §8) and are not provided here.

use crate::limits::MattenLimits;
use crate::{MattenError, Tensor};

/// Rejects an empty input list with [`MattenError::InvalidArgument`].
fn require_non_empty(tensors: &[&Tensor], operation: &'static str) -> Result<(), MattenError> {
    if tensors.is_empty() {
        return Err(MattenError::InvalidArgument {
            operation,
            argument: "tensors",
            message: "at least one tensor is required".to_string(),
        });
    }
    Ok(())
}

/// Rejects dynamic inputs with [`MattenError::Unsupported`]. A no-op when the
/// `dynamic` feature is disabled.
fn reject_dynamic(tensors: &[&Tensor], operation: &'static str) -> Result<(), MattenError> {
    #[cfg(feature = "dynamic")]
    {
        for t in tensors {
            if t.is_dynamic() {
                return Err(MattenError::Unsupported {
                    operation,
                    message:
                        "dynamic tensors must be converted with try_numeric() before shape composition"
                            .to_string(),
                });
            }
        }
    }
    #[cfg(not(feature = "dynamic"))]
    let _ = (tensors, operation);
    Ok(())
}

impl Tensor {
    /// Joins tensors along an existing `axis` (the analogue of NumPy's
    /// `concatenate`). All inputs must share the same rank and the same size on
    /// every axis except `axis`; the output `axis` size is the sum of the inputs'.
    ///
    /// A single-element list returns a clone of that tensor (after validation).
    ///
    /// # Panics
    /// Panics if the input list is empty, the ranks or non-axis dimensions
    /// disagree, `axis` is out of range (`0..rank`), any input is a dynamic tensor,
    /// or the result exceeds the allocation limit. Use [`Tensor::try_concatenate`]
    /// for the non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    /// let b = Tensor::new(vec![5.0, 6.0], &[1, 2]);
    /// let c = Tensor::concatenate(&[&a, &b], 0);
    /// assert_eq!(c.shape(), &[3, 2]);
    /// assert_eq!(c.as_slice(), &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    /// ```
    #[must_use]
    pub fn concatenate(tensors: &[&Tensor], axis: usize) -> Tensor {
        Tensor::try_concatenate(tensors, axis).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::concatenate`].
    ///
    /// # Errors
    /// - [`MattenError::InvalidArgument`] if `tensors` is empty.
    /// - [`MattenError::Shape`] on rank mismatch, a non-axis dimension mismatch, or
    ///   `axis >= rank`.
    /// - [`MattenError::Unsupported`] if any input is a dynamic tensor.
    /// - [`MattenError::Allocation`] if the result exceeds the allocation limit.
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::from_vec(vec![1.0, 2.0]);
    /// let b = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    /// assert!(Tensor::try_concatenate(&[&a, &b], 0).is_err()); // rank mismatch
    /// assert!(Tensor::try_concatenate(&[], 0).is_err()); // empty input
    /// ```
    pub fn try_concatenate(tensors: &[&Tensor], axis: usize) -> Result<Tensor, MattenError> {
        require_non_empty(tensors, "concatenate")?;
        reject_dynamic(tensors, "concatenate")?;

        let first = tensors[0];
        let rank = first.shape.len();
        if axis >= rank {
            return Err(MattenError::Shape {
                operation: "concatenate",
                message: format!(
                    "axis {axis} is out of range for concatenate on rank-{rank} tensors (valid 0..{rank})"
                ),
            });
        }

        // All inputs: same rank, and same size on every non-concatenation axis.
        for (i, t) in tensors.iter().enumerate() {
            if t.shape.len() != rank {
                return Err(MattenError::Shape {
                    operation: "concatenate",
                    message: format!(
                        "tensor {i} has rank {} but tensor 0 has rank {rank}; \
                         concatenate requires equal ranks",
                        t.shape.len()
                    ),
                });
            }
            for (ax, (&d, &d0)) in t.shape.iter().zip(&first.shape).enumerate() {
                if ax != axis && d != d0 {
                    return Err(MattenError::Shape {
                        operation: "concatenate",
                        message: format!(
                            "tensor {i} has size {d} at axis {ax} but tensor 0 has {d0}; \
                             all non-concatenation axes must match"
                        ),
                    });
                }
            }
        }

        // Output axis size is the (checked) sum of input axis sizes.
        let mut axis_total: usize = 0;
        for t in tensors {
            axis_total =
                axis_total
                    .checked_add(t.shape[axis])
                    .ok_or_else(|| MattenError::Allocation {
                        requested_elements: usize::MAX,
                        message: "concatenated axis size overflowed".to_string(),
                    })?;
        }
        let mut out_shape = first.shape.clone();
        out_shape[axis] = axis_total;
        let total = MattenLimits::default().check_shape(&out_shape, "concatenate")?;

        // Row-major copy: for each outer slab, append each input's contiguous block.
        let inner: usize = first.shape[axis + 1..].iter().product();
        let outer: usize = first.shape[..axis].iter().product();
        let mut data = Vec::with_capacity(total);
        for o in 0..outer {
            for t in tensors {
                let block = t.shape[axis] * inner;
                let start = o * block;
                data.extend_from_slice(&t.data[start..start + block]);
            }
        }

        Ok(Tensor {
            data,
            shape: out_shape,
            #[cfg(feature = "dynamic")]
            dynamic: None,
        })
    }

    /// Joins identically shaped tensors along a **new** `axis` (the analogue of
    /// NumPy's `stack`). The output rank is the input rank plus one, and the new
    /// axis (size = number of inputs) is inserted at position `axis`.
    ///
    /// `axis` may be `0..=rank`. A single-element list returns that tensor with a
    /// new length-1 axis inserted.
    ///
    /// # Panics
    /// Panics if the input list is empty, the input shapes are not all identical,
    /// `axis` is out of range (`0..=rank`), any input is a dynamic tensor, or the
    /// result exceeds the allocation limit. Use [`Tensor::try_stack`] for the
    /// non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    /// let b = Tensor::from_vec(vec![4.0, 5.0, 6.0]);
    /// assert_eq!(Tensor::stack(&[&a, &b], 0).shape(), &[2, 3]);
    /// let s1 = Tensor::stack(&[&a, &b], 1);
    /// assert_eq!(s1.shape(), &[3, 2]);
    /// assert_eq!(s1.as_slice(), &[1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
    /// ```
    #[must_use]
    pub fn stack(tensors: &[&Tensor], axis: usize) -> Tensor {
        Tensor::try_stack(tensors, axis).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::stack`].
    ///
    /// # Errors
    /// - [`MattenError::InvalidArgument`] if `tensors` is empty.
    /// - [`MattenError::Shape`] if the input shapes differ or `axis > rank`.
    /// - [`MattenError::Unsupported`] if any input is a dynamic tensor.
    /// - [`MattenError::Allocation`] if the result exceeds the allocation limit.
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::from_vec(vec![1.0, 2.0]);
    /// assert!(Tensor::try_stack(&[&a], 5).is_err()); // axis out of range (valid 0..=1)
    /// assert!(Tensor::try_stack(&[], 0).is_err()); // empty input
    /// ```
    pub fn try_stack(tensors: &[&Tensor], axis: usize) -> Result<Tensor, MattenError> {
        require_non_empty(tensors, "stack")?;
        reject_dynamic(tensors, "stack")?;

        let first = tensors[0];
        let rank = first.shape.len();
        if axis > rank {
            return Err(MattenError::Shape {
                operation: "stack",
                message: format!(
                    "axis {axis} is out of range for stack on rank-{rank} tensors (valid 0..={rank})"
                ),
            });
        }

        // All inputs must have identical shape.
        for (i, t) in tensors.iter().enumerate() {
            if t.shape != first.shape {
                return Err(MattenError::Shape {
                    operation: "stack",
                    message: format!(
                        "tensor {i} has shape {:?} but tensor 0 has shape {:?}; \
                         stack requires identical shapes",
                        t.shape, first.shape
                    ),
                });
            }
        }

        let n = tensors.len();
        let mut out_shape = Vec::with_capacity(rank + 1);
        out_shape.extend_from_slice(&first.shape[..axis]);
        out_shape.push(n);
        out_shape.extend_from_slice(&first.shape[axis..]);
        let total = MattenLimits::default().check_shape(&out_shape, "stack")?;

        // Row-major copy: for each outer slab, append each input's inner block in
        // turn, placing the new axis (size n) at position `axis`.
        let inner: usize = first.shape[axis..].iter().product();
        let outer: usize = first.shape[..axis].iter().product();
        let mut data = Vec::with_capacity(total);
        for o in 0..outer {
            for t in tensors {
                let start = o * inner;
                data.extend_from_slice(&t.data[start..start + inner]);
            }
        }

        Ok(Tensor {
            data,
            shape: out_shape,
            #[cfg(feature = "dynamic")]
            dynamic: None,
        })
    }
}
