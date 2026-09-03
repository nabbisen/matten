//! Shape operations, slicing, and boundary integration methods for `Tensor`
//! (RFC-007, RFC-008, RFC-009). Split from `tensor.rs` per the 300-ELOC guideline.

use crate::{MattenError, Tensor};

/// Reads `path` into a `String`, refusing to read more than
/// [`MattenLimits::max_parse_bytes`](crate::MattenLimits::max_parse_bytes)
/// bytes (RFC-132 §12.3). A metadata check rejects an obviously oversized
/// file before opening it; `Read::take` bounds the actual read too, since a
/// size check alone is a TOCTOU race — the file can grow between the check
/// and the read.
#[cfg(any(feature = "json", feature = "csv"))]
fn read_bounded_file(
    path: &std::path::Path,
    data_format: crate::error::DataFormat,
) -> Result<String, MattenError> {
    use std::io::Read;

    let max_bytes = crate::limits::MAX_PARSE_BYTES;
    let metadata = std::fs::metadata(path).map_err(|e| MattenError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;
    if metadata.len() > max_bytes as u64 {
        return Err(MattenError::Parse {
            format: data_format,
            message: format!(
                "file is {} bytes, exceeding the maximum parse size of {max_bytes} bytes \
                 (MattenLimits::max_parse_bytes)",
                metadata.len()
            ),
        });
    }

    let file = std::fs::File::open(path).map_err(|e| MattenError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;
    let mut content = String::new();
    file.take(max_bytes as u64 + 1)
        .read_to_string(&mut content)
        .map_err(|e| MattenError::Io {
            path: path.to_path_buf(),
            source: e,
        })?;
    if content.len() > max_bytes {
        return Err(MattenError::Parse {
            format: data_format,
            message: format!(
                "file exceeds the maximum parse size of {max_bytes} bytes \
                 (MattenLimits::max_parse_bytes)"
            ),
        });
    }
    Ok(content)
}

impl Tensor {
    // ---- Shape operations (M4 / RFC-007) ------------------------------------

    /// Reshapes the tensor to `new_shape`, returning a new owned tensor.
    ///
    /// The total element count must be unchanged. Data order is preserved
    /// (row-major flat order).
    ///
    /// # Panics
    ///
    /// Panics on element-count mismatch or invalid shape. Use
    /// [`try_reshape`](Tensor::try_reshape) for recoverable construction.
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    /// let flat = t.reshape(&[4]);
    /// assert_eq!(flat.shape(), &[4]);
    /// assert_eq!(flat.as_slice(), &[1.0, 2.0, 3.0, 4.0]);
    /// ```
    #[must_use]
    pub fn reshape(&self, new_shape: &[usize]) -> Tensor {
        crate::reshape::try_reshape_impl(self, new_shape).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Reshapes the tensor, returning an error instead of panicking.
    ///
    /// # Errors
    ///
    /// Returns [`MattenError::Shape`] on element-count mismatch or invalid shape.
    pub fn try_reshape(&self, new_shape: &[usize]) -> Result<Tensor, MattenError> {
        crate::reshape::try_reshape_impl(self, new_shape)
    }

    /// Flattens the tensor to a 1-D tensor, preserving row-major order.
    ///
    /// A scalar (shape `[]`) is returned as shape `[1]`.
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    /// let flat = t.flatten();
    /// assert_eq!(flat.shape(), &[4]);
    /// ```
    #[must_use]
    pub fn flatten(&self) -> Tensor {
        #[cfg(feature = "dynamic")]
        if self.is_dynamic() {
            panic!(
                "matten unsupported error in flatten: dynamic tensors do not support flatten; call try_numeric() first to convert to a numeric tensor"
            );
        }
        let len = self.data.len();
        Tensor::from_parts_checked(self.data.clone(), vec![len])
    }

    /// Transposes the tensor by reversing the axis order.
    ///
    /// For a rank-2 tensor this swaps rows and columns. For rank > 2 the axis
    /// order is reversed: `[d0, d1, d2] → [d2, d1, d0]`.
    ///
    /// # Panics
    ///
    /// Panics for a rank-0 scalar (no axes to transpose).
    ///
    /// ```
    /// use matten::Tensor;
    /// let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    /// let mt = m.transpose();
    /// assert_eq!(mt.shape(), &[3, 2]);
    /// assert_eq!(mt.as_slice(), &[1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
    /// ```
    #[must_use]
    pub fn transpose(&self) -> Tensor {
        let ndim = self.ndim();
        if ndim == 0 {
            panic!("matten shape error in transpose: cannot transpose a scalar (rank 0)");
        }
        let perm: Vec<usize> = (0..ndim).rev().collect();
        crate::reshape::permute_axes(self, &perm)
    }

    /// Alias for [`transpose`](Tensor::transpose).
    #[must_use]
    pub fn t(&self) -> Tensor {
        self.transpose()
    }

    /// Returns a new tensor with `axis1` and `axis2` swapped.
    ///
    /// # Panics
    ///
    /// Panics if either axis is out of range.
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::new((1..=24).map(|x| x as f64).collect(), &[2, 3, 4]);
    /// let s = t.swap_axes(0, 2);
    /// assert_eq!(s.shape(), &[4, 3, 2]);
    /// ```
    #[must_use]
    pub fn swap_axes(&self, axis1: usize, axis2: usize) -> Tensor {
        crate::reshape::validate_axes(axis1, axis2, self.ndim(), "swap_axes")
            .unwrap_or_else(|e| panic!("{e}"));
        let mut perm: Vec<usize> = (0..self.ndim()).collect();
        perm.swap(axis1, axis2);
        crate::reshape::permute_axes(self, &perm)
    }

    /// Removes all axes of length `1`, returning a new owned tensor.
    ///
    /// Data order is unchanged. A scalar stays a scalar, and a tensor whose every
    /// axis is `1` (e.g. `[1, 1]`) becomes a scalar (shape `[]`).
    ///
    /// # Panics
    ///
    /// Panics on a dynamic tensor; call `try_numeric()` first.
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::new(vec![1.0, 2.0, 3.0], &[1, 3, 1]);
    /// assert_eq!(t.squeeze().shape(), &[3]);
    /// ```
    #[must_use]
    pub fn squeeze(&self) -> Tensor {
        #[cfg(feature = "dynamic")]
        if self.is_dynamic() {
            panic!(
                "matten unsupported error in squeeze: dynamic tensors do not support squeeze; call try_numeric() first to convert to a numeric tensor"
            );
        }
        let shape: Vec<usize> = self.shape.iter().copied().filter(|&d| d != 1).collect();
        Tensor::from_parts_checked(self.data.clone(), shape)
    }

    /// Inserts a new axis of length `1` at `axis`, returning a new owned tensor.
    ///
    /// `axis` may be `0..=ndim` (inserting at `ndim` appends a trailing axis).
    /// Data order is unchanged.
    ///
    /// # Panics
    ///
    /// Panics if `axis > ndim`, or on a dynamic tensor. Use
    /// [`try_expand_dims`](Tensor::try_expand_dims) for the non-panicking form.
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    /// assert_eq!(t.expand_dims(0).shape(), &[1, 3]);
    /// assert_eq!(t.expand_dims(1).shape(), &[3, 1]);
    /// ```
    #[must_use]
    pub fn expand_dims(&self, axis: usize) -> Tensor {
        self.try_expand_dims(axis).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Non-panicking [`expand_dims`](Tensor::expand_dims).
    ///
    /// # Errors
    ///
    /// Returns [`MattenError::InvalidArgument`] if `axis > ndim`, or
    /// [`MattenError::Unsupported`] on a dynamic tensor.
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    /// assert!(t.try_expand_dims(5).is_err());
    /// ```
    pub fn try_expand_dims(&self, axis: usize) -> Result<Tensor, MattenError> {
        #[cfg(feature = "dynamic")]
        if self.is_dynamic() {
            return Err(MattenError::Unsupported {
                operation: "expand_dims",
                message: "dynamic tensors do not support expand_dims; call try_numeric() first"
                    .to_string(),
            });
        }
        let ndim = self.shape.len();
        if axis > ndim {
            return Err(MattenError::InvalidArgument {
                operation: "expand_dims",
                argument: "axis",
                message: format!(
                    "axis {axis} is out of range for a rank-{ndim} tensor (valid 0..={ndim})"
                ),
            });
        }
        let mut shape = self.shape.clone();
        shape.insert(axis, 1);
        Ok(Tensor::from_parts_checked(self.data.clone(), shape))
    }

    /// Returns the element at the multidimensional `coord`, or `None` if the
    /// coordinate rank doesn't match or any component is out of bounds.
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    /// assert_eq!(t.get(&[0, 1]), Some(2.0));
    /// assert_eq!(t.get(&[5, 0]), None);
    /// ```
    pub fn get(&self, coord: &[usize]) -> Option<f64> {
        #[cfg(feature = "dynamic")]
        self.panic_if_dynamic("get");
        let flat = crate::shape::coord_to_flat(coord, &self.shape)?;
        self.data.get(flat).copied()
    }

    /// Returns the element at flat row-major `index`, or `None` if out of bounds.
    ///
    /// This is the flat-index companion to [`get`](Tensor::get). The index
    /// follows the same row-major layout as [`as_slice`](Tensor::as_slice).
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    /// assert_eq!(t.get_flat(1), Some(2.0));
    /// assert_eq!(t.get_flat(10), None);
    /// ```
    pub fn get_flat(&self, index: usize) -> Option<f64> {
        #[cfg(feature = "dynamic")]
        self.panic_if_dynamic("get_flat");
        self.data.get(index).copied()
    }

    /// Returns a mutable reference to the element at the multidimensional
    /// `coord`, or `None` if the coordinate rank doesn't match or any
    /// component is out of bounds.
    ///
    /// # Panics
    ///
    /// Panics on a dynamic tensor, the same guard [`get`](Tensor::get) uses —
    /// use [`get_element_mut`](Tensor::get_element_mut) there instead.
    ///
    /// ```
    /// use matten::Tensor;
    /// let mut t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    /// *t.get_mut(&[0, 1]).unwrap() += 1.0;
    /// assert_eq!(t.get(&[0, 1]), Some(3.0));
    /// assert_eq!(t.get_mut(&[5, 0]), None);
    /// ```
    pub fn get_mut(&mut self, coord: &[usize]) -> Option<&mut f64> {
        #[cfg(feature = "dynamic")]
        self.panic_if_dynamic("get_mut");
        let flat = crate::shape::coord_to_flat(coord, &self.shape)?;
        self.data.get_mut(flat)
    }

    /// Returns a mutable reference to the element at flat row-major `index`,
    /// or `None` if out of bounds.
    ///
    /// The flat-index companion to [`get_mut`](Tensor::get_mut), following
    /// the same row-major layout as [`as_slice`](Tensor::as_slice).
    ///
    /// # Panics
    ///
    /// Panics on a dynamic tensor, the same guard [`get_flat`](Tensor::get_flat) uses.
    ///
    /// ```
    /// use matten::Tensor;
    /// let mut t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    /// *t.get_flat_mut(1).unwrap() += 1.0;
    /// assert_eq!(t.get_flat(1), Some(3.0));
    /// assert_eq!(t.get_flat_mut(10), None);
    /// ```
    pub fn get_flat_mut(&mut self, index: usize) -> Option<&mut f64> {
        #[cfg(feature = "dynamic")]
        self.panic_if_dynamic("get_flat_mut");
        self.data.get_mut(index)
    }

    // ---- Slicing (M4 / RFC-008) ---------------------------------------------

    /// Starts a slice builder for this tensor. The builder is the canonical
    /// slicing API; [`slice_str`](Tensor::slice_str) is a convenience wrapper.
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::new(vec![1.0,2.0,3.0,4.0,5.0,6.0], &[2, 3]);
    /// let row = t.slice().index(0).all().build().unwrap();
    /// assert_eq!(row.as_slice(), &[1.0, 2.0, 3.0]);
    /// ```
    pub fn slice(&self) -> crate::slice::SliceBuilder<'_> {
        crate::slice::SliceBuilder::new(self)
    }

    /// Slices this tensor using a NumPy-like string specification.
    ///
    /// This is a convenience wrapper over the builder API. It always returns
    /// `Result` and never panics on malformed input.
    ///
    /// `index`, `start`, and `end` accept an optional leading `-` (RFC-088):
    /// `-1` means the last element along that axis, resolved as `dim + i`
    /// before the usual bounds check. `step` does **not** accept a sign —
    /// `"::-1"` (reversal) is not implemented and remains a parse error.
    ///
    /// **Negative bounds do not clamp on out-of-range, unlike Python.**
    /// `"-10"` on an axis of size 3 is an error, naming both the written form
    /// and what it resolved to — the same policy `matten` already applies to
    /// positive out-of-range values (`"0:100"` on size 3 also errors, never
    /// silently truncates). The builder does not accept negative indices; a
    /// caller with `len` in hand writes `len - 1` directly.
    ///
    /// # Errors
    ///
    /// Returns [`MattenError::Slice`] for any parse or bounds error.
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::new(vec![1.0,2.0,3.0,4.0,5.0,6.0], &[2, 3]);
    /// let top = t.slice_str("0, :").unwrap();
    /// assert_eq!(top.as_slice(), &[1.0, 2.0, 3.0]);
    ///
    /// // Negative indices count from the end.
    /// let last_row = t.slice_str("-1, :").unwrap();
    /// assert_eq!(last_row.as_slice(), &[4.0, 5.0, 6.0]);
    /// let all_but_last_col = t.slice_str(":, 0:-1").unwrap();
    /// assert_eq!(all_but_last_col.as_slice(), &[1.0, 2.0, 4.0, 5.0]);
    /// ```
    pub fn slice_str(&self, spec: &str) -> Result<Tensor, MattenError> {
        let specs = crate::slice::parse_slice_str(spec)?;
        crate::slice::execute_slice(self, &specs, "slice_str")
    }
}

// ---- Boundary integration (M5 / RFC-009) --------------------------------

impl Tensor {
    /// Parses a JSON string into a `Tensor`.
    ///
    /// Accepts the canonical `{"shape":[…],"data":[…]}` object form and the
    /// convenience nested-array form (rank 1 and 2). Returns
    /// [`MattenError::Parse`] for any error; never panics.
    ///
    /// ```
    /// use matten::Tensor;
    ///
    /// // Canonical object form
    /// let t = Tensor::from_json(r#"{"shape":[2,2],"data":[1.0,2.0,3.0,4.0]}"#).unwrap();
    /// assert_eq!(t.shape(), &[2, 2]);
    ///
    /// // Nested-array convenience form
    /// let t = Tensor::from_json("[[1.0,2.0],[3.0,4.0]]").unwrap();
    /// assert_eq!(t.shape(), &[2, 2]);
    /// ```
    #[cfg(feature = "json")]
    pub fn from_json(input: &str) -> Result<Tensor, MattenError> {
        crate::parse::json::from_json_str(input)
    }

    /// Parses a CSV string into a `Tensor` with shape `[rows, cols]`.
    ///
    /// All fields must be valid `f64` values. Returns [`MattenError::Parse`]
    /// for ragged rows or non-numeric fields; never panics.
    ///
    /// ```
    /// use matten::Tensor;
    ///
    /// let t = Tensor::from_csv("1.0,2.0\n3.0,4.0\n").unwrap();
    /// assert_eq!(t.shape(), &[2, 2]);
    /// assert_eq!(t.as_slice(), &[1.0, 2.0, 3.0, 4.0]);
    /// ```
    #[cfg(feature = "csv")]
    pub fn from_csv(input: &str) -> Result<Tensor, MattenError> {
        crate::parse::csv::from_csv_str(input)
    }

    /// Loads and parses a JSON file into a `Tensor`.
    ///
    /// Returns [`MattenError::Io`] for file errors, [`MattenError::Parse`] for
    /// parse errors.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or the content is invalid,
    /// or if the file exceeds
    /// [`MattenLimits::max_parse_bytes`](crate::MattenLimits::max_parse_bytes)
    /// (RFC-132 §12.3) — checked via file metadata before the file is opened,
    /// and enforced again during the read itself.
    #[cfg(feature = "json")]
    pub fn load_json(path: impl AsRef<std::path::Path>) -> Result<Tensor, MattenError> {
        let path = path.as_ref();
        let content = read_bounded_file(path, crate::error::DataFormat::Json)?;
        crate::parse::json::from_json_str(&content)
    }

    /// Loads and parses a CSV file into a `Tensor` with shape `[rows, cols]`.
    ///
    /// Returns [`MattenError::Io`] for file errors, [`MattenError::Parse`] for
    /// parse errors.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or the content is invalid,
    /// or if the file exceeds
    /// [`MattenLimits::max_parse_bytes`](crate::MattenLimits::max_parse_bytes)
    /// (RFC-132 §12.3) — checked via file metadata before the file is opened,
    /// and enforced again during the read itself.
    #[cfg(feature = "csv")]
    pub fn load_csv(path: impl AsRef<std::path::Path>) -> Result<Tensor, MattenError> {
        let path = path.as_ref();
        let content = read_bounded_file(path, crate::error::DataFormat::Csv)?;
        crate::parse::csv::from_csv_str(&content)
    }
}

#[cfg(test)]
mod tests;
