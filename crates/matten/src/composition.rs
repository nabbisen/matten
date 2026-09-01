//! Shape composition: joining, repeating, and gridding tensors (RFC-039, RFC-087).
//!
//! [`Tensor::concatenate`] joins tensors along an **existing** axis — all inputs
//! must share the same rank and the same size on every non-concatenation axis, and
//! the output axis size is the sum of the input axis sizes. [`Tensor::stack`] joins
//! **identically shaped** tensors along a **new** axis, so the output rank is the
//! input rank plus one and the new axis size is the number of inputs.
//! [`Tensor::repeat`]/[`Tensor::repeat_axis`] and [`Tensor::tile`] repeat a single
//! tensor's own data — see their docs for the repeat-vs-tile contrast, the single
//! most confused pair in this area. [`Tensor::meshgrid`] builds the two coordinate
//! grids for evaluating a function of two variables over a rank-1 `x`/`y` pair.
//!
//! All of these enforce [`MattenLimits`] on the output allocation and reject dynamic
//! tensors — convert with [`Tensor::try_numeric`](crate::Tensor::try_numeric) first.
//! The `try_*` forms return [`MattenError`]; the convenience forms panic with the
//! same message.

use crate::limits::MattenLimits;
use crate::shape::{coord_to_flat, flat_to_coord};
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

        Ok(Tensor::from_parts_checked(data, out_shape))
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

        Ok(Tensor::from_parts_checked(data, out_shape))
    }

    /// Repeats each **element** `n` times, flattening the result to rank 1
    /// (the analogue of NumPy's `repeat` with no `axis`).
    ///
    /// **`repeat` repeats elements; [`Tensor::tile`] repeats the whole tensor** —
    /// the single most confused pair in this area:
    ///
    /// ```text
    /// [1, 2, 3].repeat(2)        -> [1, 1, 2, 2, 3, 3]   (each element, in place)
    /// [1, 2, 3].tile(&[2])       -> [1, 2, 3, 1, 2, 3]   (the whole tensor, twice)
    /// ```
    ///
    /// A rank-0 scalar repeats to a rank-1 tensor of length `n`. Repetition is
    /// explicit allocation, unlike broadcasting, which is implicit and materializes
    /// nothing — `[1,2,3] * 2` and `[1,2,3].repeat(2)` differ for exactly that reason.
    /// `n == 0` returns an empty tensor (RFC-111).
    ///
    /// # Panics
    /// Panics if the input is a dynamic tensor, or the result exceeds the
    /// allocation limit. Use [`Tensor::try_repeat`] for the non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    /// let r = a.repeat(2);
    /// assert_eq!(r.shape(), &[6]);
    /// assert_eq!(r.as_slice(), &[1.0, 1.0, 2.0, 2.0, 3.0, 3.0]);
    /// ```
    #[must_use]
    pub fn repeat(&self, n: usize) -> Tensor {
        self.try_repeat(n).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::repeat`].
    ///
    /// # Errors
    /// - [`MattenError::Unsupported`] if the input is a dynamic tensor.
    /// - [`MattenError::Allocation`] if the result exceeds the allocation limit.
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::from_vec(vec![1.0, 2.0]);
    /// let r = Tensor::try_repeat(&a, 0).unwrap(); // n = 0 -> empty, not an error
    /// assert!(r.is_empty());
    /// ```
    pub fn try_repeat(&self, n: usize) -> Result<Tensor, MattenError> {
        reject_dynamic(&[self], "repeat")?;

        let in_len = self.data.len();
        let out_len = in_len
            .checked_mul(n)
            .ok_or_else(|| MattenError::Allocation {
                requested_elements: usize::MAX,
                message: format!(
                    "repeat output length overflowed for {in_len} elements repeated {n} times"
                ),
            })?;
        let out_shape = vec![out_len];
        let total = MattenLimits::default().check_shape(&out_shape, "repeat")?;

        let mut data = Vec::with_capacity(total);
        for &x in &self.data {
            for _ in 0..n {
                data.push(x);
            }
        }

        Ok(Tensor::from_parts_checked(data, out_shape))
    }

    /// Repeats each element `n` times along `axis`, preserving rank (the analogue
    /// of NumPy's `repeat` with an `axis` argument).
    ///
    /// ```text
    /// [[1, 2], [3, 4]].repeat_axis(2, 0)  ->  [[1, 2], [1, 2], [3, 4], [3, 4]]
    /// ```
    ///
    /// See [`Tensor::repeat`] for the repeat-vs-[`Tensor::tile`] contrast.
    ///
    /// `n == 0` returns a zero-length result on `axis` (RFC-111).
    ///
    /// # Panics
    /// Panics if the input is a rank-0 scalar (there is no axis to repeat
    /// along), `axis` is out of range, or the input is a dynamic tensor, or the
    /// result exceeds the allocation limit. Use [`Tensor::try_repeat_axis`] for the
    /// non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    /// let r = a.repeat_axis(2, 0);
    /// assert_eq!(r.shape(), &[4, 2]);
    /// assert_eq!(r.as_slice(), &[1.0, 2.0, 1.0, 2.0, 3.0, 4.0, 3.0, 4.0]);
    /// ```
    #[must_use]
    pub fn repeat_axis(&self, n: usize, axis: usize) -> Tensor {
        self.try_repeat_axis(n, axis)
            .unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::repeat_axis`].
    ///
    /// # Errors
    /// - [`MattenError::Shape`] if the input is a rank-0 scalar, or `axis` is out
    ///   of range (`0..rank`).
    /// - [`MattenError::Unsupported`] if the input is a dynamic tensor.
    /// - [`MattenError::Allocation`] if the result exceeds the allocation limit.
    ///
    /// ```
    /// use matten::Tensor;
    /// let s = Tensor::scalar(3.0);
    /// assert!(Tensor::try_repeat_axis(&s, 2, 0).is_err()); // rank-0: no axis
    /// ```
    pub fn try_repeat_axis(&self, n: usize, axis: usize) -> Result<Tensor, MattenError> {
        reject_dynamic(&[self], "repeat_axis")?;
        let rank = self.shape.len();
        if rank == 0 {
            return Err(MattenError::Shape {
                operation: "repeat_axis",
                message: "repeat_axis requires at least one axis, but the input is a rank-0 \
                          scalar with no axis to repeat along"
                    .to_string(),
            });
        }
        if axis >= rank {
            return Err(MattenError::Shape {
                operation: "repeat_axis",
                message: format!("axis {axis} is out of range for a rank-{rank} tensor"),
            });
        }

        let axis_len = self.shape[axis];
        let new_axis_len = axis_len
            .checked_mul(n)
            .ok_or_else(|| MattenError::Allocation {
                requested_elements: usize::MAX,
                message: format!(
                    "repeat_axis output shape overflowed for axis length {axis_len} repeated {n} times"
                ),
            })?;
        let mut out_shape = self.shape.clone();
        out_shape[axis] = new_axis_len;
        let total = MattenLimits::default().check_shape(&out_shape, "repeat_axis")?;

        // Same inner/outer decomposition as concatenate/stack: for each outer
        // slab, walk the axis positions and repeat each one's inner block n times.
        let inner: usize = self.shape[axis + 1..].iter().product();
        let outer: usize = self.shape[..axis].iter().product();
        let mut data = Vec::with_capacity(total);
        for o in 0..outer {
            for k in 0..axis_len {
                let start = (o * axis_len + k) * inner;
                let block = &self.data[start..start + inner];
                for _ in 0..n {
                    data.extend_from_slice(block);
                }
            }
        }

        Ok(Tensor::from_parts_checked(data, out_shape))
    }

    /// Repeats the **whole tensor** according to `reps`, one repetition factor per
    /// axis (the analogue of NumPy's `tile`).
    ///
    /// **[`Tensor::repeat`] repeats elements; `tile` repeats the whole tensor**:
    ///
    /// ```text
    /// [1, 2, 3].repeat(2)        -> [1, 1, 2, 2, 3, 3]   (each element, in place)
    /// [1, 2, 3].tile(&[2])       -> [1, 2, 3, 1, 2, 3]   (the whole tensor, twice)
    /// ```
    ///
    /// If `reps` is shorter than the input's rank, it is padded with leading `1`s
    /// (NumPy-compatible — `tile(&[2])` on a matrix repeats only the last axis). If
    /// `reps` is longer than the rank, this is an **error**: NumPy would silently
    /// promote the tensor's rank, which `matten` treats as the surprising direction
    /// (the result would have more dimensions than the input, with no obvious place
    /// for a caller to look) — an explicit [`MattenError::Shape`] is preferred.
    ///
    /// A `0` entry in `reps` produces a zero-length result on that axis (RFC-111).
    ///
    /// # Panics
    /// Panics if `reps` is empty, `reps` is longer than the input's rank, the
    /// input is a dynamic tensor, or the result exceeds the allocation limit.
    /// Use [`Tensor::try_tile`] for the non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    /// let t = a.tile(&[2]);
    /// assert_eq!(t.shape(), &[6]);
    /// assert_eq!(t.as_slice(), &[1.0, 2.0, 3.0, 1.0, 2.0, 3.0]);
    /// ```
    #[must_use]
    pub fn tile(&self, reps: &[usize]) -> Tensor {
        self.try_tile(reps).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::tile`].
    ///
    /// # Errors
    /// - [`MattenError::Shape`] if `reps` is empty, or `reps` is longer than the
    ///   input's rank (rank promotion is rejected, not performed).
    /// - [`MattenError::Unsupported`] if the input is a dynamic tensor.
    /// - [`MattenError::Allocation`] if the result exceeds the allocation limit.
    ///
    /// ```
    /// use matten::Tensor;
    /// let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    /// assert!(Tensor::try_tile(&a, &[1, 1, 1]).is_err()); // reps longer than rank 2
    /// ```
    pub fn try_tile(&self, reps: &[usize]) -> Result<Tensor, MattenError> {
        reject_dynamic(&[self], "tile")?;
        let rank = self.shape.len();

        if reps.is_empty() {
            return Err(MattenError::Shape {
                operation: "tile",
                message: "tile requires at least one repetition factor; reps must not be empty"
                    .to_string(),
            });
        }
        if reps.len() > rank {
            return Err(MattenError::Shape {
                operation: "tile",
                message: format!(
                    "reps has length {} but the input has rank {rank}; tile does not promote \
                     rank (reps longer than rank is rejected rather than silently padding the \
                     input's shape)",
                    reps.len()
                ),
            });
        }

        // Pad reps with leading 1s so it lines up with the input rank exactly.
        let mut padded_reps = vec![1usize; rank - reps.len()];
        padded_reps.extend_from_slice(reps);

        let mut out_shape = Vec::with_capacity(rank);
        for (&dim, &rep) in self.shape.iter().zip(&padded_reps) {
            let scaled = dim
                .checked_mul(rep)
                .ok_or_else(|| MattenError::Allocation {
                    requested_elements: usize::MAX,
                    message: format!("tile output shape overflowed for dim {dim} x rep {rep}"),
                })?;
            out_shape.push(scaled);
        }
        let total = MattenLimits::default().check_shape(&out_shape, "tile")?;

        // Each output element's value is the input element at the same coordinate
        // modulo the input's own dimension on every axis (that "wrap" is what makes
        // this a repetition of the whole tensor rather than of each element).
        let mut data = Vec::with_capacity(total);
        for flat_out in 0..total {
            let coord = flat_to_coord(flat_out, &out_shape);
            let in_coord: Vec<usize> = coord
                .iter()
                .zip(&self.shape)
                .map(|(&c, &dim)| c % dim)
                .collect();
            let in_flat = coord_to_flat(&in_coord, &self.shape)
                .expect("in_coord is constructed component-wise in-bounds for self.shape");
            data.push(self.data[in_flat]);
        }

        Ok(Tensor::from_parts_checked(data, out_shape))
    }

    /// Builds the two coordinate grids for evaluating a function of two variables
    /// over rank-1 `x` (length `m`) and `y` (length `n`), NumPy's `xy` indexing:
    /// both outputs have shape `[n, m]`, `out_x[i][j] == x[j]`, `out_y[i][j] ==
    /// y[i]`.
    ///
    /// `xy` indexing is used deliberately, matching NumPy's default, even though the
    /// "row `i` reads as `y[i]`, column `j` reads as `x[j]`" `ij` convention can feel
    /// more natural for a matrix reading. When `x` and `y` have **equal** length the
    /// two conventions differ only by a transpose — an invisible mistake with no
    /// shape error to catch it — so this matches the ecosystem instead of diverging
    /// on an axis a caller cannot see. (A reader who specifically wants `ij` gets it
    /// by transposing both outputs.)
    ///
    /// # Panics
    /// Panics if either input is not rank-1, either input is a dynamic tensor, or
    /// the result exceeds the allocation limit. Use [`Tensor::try_meshgrid`] for the
    /// non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// let x = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    /// let y = Tensor::from_vec(vec![10.0, 20.0]);
    /// let (gx, gy) = Tensor::meshgrid(&x, &y);
    /// assert_eq!(gx.shape(), &[2, 3]);
    /// assert_eq!(gx.as_slice(), &[1.0, 2.0, 3.0, 1.0, 2.0, 3.0]);
    /// assert_eq!(gy.as_slice(), &[10.0, 10.0, 10.0, 20.0, 20.0, 20.0]);
    /// ```
    #[must_use]
    pub fn meshgrid(x: &Tensor, y: &Tensor) -> (Tensor, Tensor) {
        Tensor::try_meshgrid(x, y).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`Tensor::meshgrid`].
    ///
    /// # Errors
    /// - [`MattenError::Shape`] if either `x` or `y` is not rank-1 (a rank-2 input
    ///   is rejected, never silently flattened).
    /// - [`MattenError::Unsupported`] if either input is a dynamic tensor.
    /// - [`MattenError::Allocation`] if the result exceeds the allocation limit.
    ///
    /// ```
    /// use matten::Tensor;
    /// let matrix = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    /// let vector = Tensor::from_vec(vec![1.0, 2.0]);
    /// assert!(Tensor::try_meshgrid(&matrix, &vector).is_err()); // matrix is rank-2
    /// ```
    pub fn try_meshgrid(x: &Tensor, y: &Tensor) -> Result<(Tensor, Tensor), MattenError> {
        reject_dynamic(&[x, y], "meshgrid")?;
        if x.shape.len() != 1 {
            return Err(MattenError::Shape {
                operation: "meshgrid",
                message: format!("x must be rank-1, got rank {}", x.shape.len()),
            });
        }
        if y.shape.len() != 1 {
            return Err(MattenError::Shape {
                operation: "meshgrid",
                message: format!("y must be rank-1, got rank {}", y.shape.len()),
            });
        }

        let m = x.shape[0];
        let n = y.shape[0];
        let out_shape = vec![n, m];
        let total = MattenLimits::default().check_shape(&out_shape, "meshgrid")?;

        let mut out_x = Vec::with_capacity(total);
        let mut out_y = Vec::with_capacity(total);
        for i in 0..n {
            for j in 0..m {
                out_x.push(x.data[j]);
                out_y.push(y.data[i]);
            }
        }

        Ok((
            Tensor::from_parts_checked(out_x, out_shape.clone()),
            Tensor::from_parts_checked(out_y, out_shape),
        ))
    }
}

#[cfg(test)]
mod tests;
