//! Slicing API: builder (canonical) and `slice_str` convenience (RFC-008).
//!
//! The builder is the canonical, Rust-native form. `slice_str` is a bounded
//! NumPy-like convenience that always returns `Result` and never panics on
//! malformed input.
//!
//! Phase 1 rule: every slice result is an independent owned [`Tensor`].

use crate::shape::coord_to_flat;
use crate::{MattenError, Tensor};

/// Maximum byte length accepted by the `slice_str` parser.
const MAX_SLICE_STR_BYTES: usize = 512;

// ---- Internal slice specification ---------------------------------------

/// One per-axis slice specification (internal).
#[derive(Debug, Clone)]
pub(crate) enum SliceSpec {
    All,
    Index(usize),
    Range {
        start: Option<usize>,
        end: Option<usize>,
        step: usize,
    },
}

// ---- Shared execution ---------------------------------------------------

fn resolve_spec(
    spec: &SliceSpec,
    dim: usize,
    axis: usize,
    operation: &'static str,
) -> Result<(Vec<usize>, bool), MattenError> {
    let err = |msg: String| MattenError::Slice {
        input: None,
        message: msg,
    };
    match spec {
        SliceSpec::All => Ok(((0..dim).collect(), true)),
        SliceSpec::Index(i) => {
            if *i >= dim {
                return Err(err(format!(
                    "index {i} is out of range for axis {axis} with size {dim}"
                )));
            }
            Ok((vec![*i], false))
        }
        SliceSpec::Range { start, end, step } => {
            let s = start.unwrap_or(0);
            let e = end.unwrap_or(dim);
            if s > dim || e > dim {
                return Err(err(format!(
                    "range {s}..{e} is out of range for axis {axis} with size {dim} \
                     in {operation}"
                )));
            }
            if s > e {
                return Err(err(format!(
                    "range start {s} > end {e} for axis {axis} in {operation}"
                )));
            }
            if *step == 0 {
                return Err(err(format!(
                    "step must be >= 1 for axis {axis} in {operation}"
                )));
            }
            Ok(((s..e).step_by(*step).collect(), true))
        }
    }
}

/// Shared slice executor used by both the builder and `slice_str`.
pub(crate) fn execute_slice(
    tensor: &Tensor,
    specs: &[SliceSpec],
    operation: &'static str,
) -> Result<Tensor, MattenError> {
    #[cfg(feature = "dynamic")]
    if tensor.is_dynamic() {
        return Err(MattenError::Unsupported {
            operation,
            message: "dynamic tensors do not support the slice builder or slice_str; \
                      use get_element(&[row, col]) for element access, or call \
                      try_numeric() first"
                .to_string(),
        });
    }
    let ndim = tensor.ndim();
    if specs.len() != ndim {
        return Err(MattenError::Slice {
            input: None,
            message: format!(
                "slice has {} specifications but tensor has rank {ndim}",
                specs.len()
            ),
        });
    }

    let mut per_axis: Vec<(Vec<usize>, bool)> = Vec::with_capacity(ndim);
    for (axis, (spec, &dim)) in specs.iter().zip(tensor.shape()).enumerate() {
        per_axis.push(resolve_spec(spec, dim, axis, operation)?);
    }

    let out_shape: Vec<usize> = per_axis
        .iter()
        .filter(|(_, keep)| *keep)
        .map(|(idxs, _)| idxs.len())
        .collect();

    let out_len = if out_shape.is_empty() {
        1
    } else {
        out_shape.iter().product()
    };
    let mut out_data = vec![0.0f64; out_len];

    let counts: Vec<usize> = per_axis.iter().map(|(v, _)| v.len()).collect();
    let total: usize = counts.iter().product();

    for out_flat in 0..total {
        let mut rem = out_flat;
        let mut src_coord = vec![0usize; ndim];
        let mut out_coord_kept = Vec::with_capacity(out_shape.len());
        let mut stride = total;
        for (ax, (sel, keep)) in per_axis.iter().enumerate() {
            stride /= counts[ax];
            let local = rem / stride;
            rem %= stride;
            src_coord[ax] = sel[local];
            if *keep {
                out_coord_kept.push(local);
            }
        }

        let src_flat = coord_to_flat(&src_coord, tensor.shape())
            .expect("constructed coordinate is always valid");
        let dst_flat = if out_shape.is_empty() {
            0
        } else {
            coord_to_flat(&out_coord_kept, &out_shape).expect("kept coordinate is always valid")
        };
        out_data[dst_flat] = tensor.data[src_flat];
    }

    Ok(Tensor {
        data: out_data,
        shape: out_shape,
        #[cfg(feature = "dynamic")]
        dynamic: None,
    })
}

// ---- IntoSliceRange sealed trait ----------------------------------------

/// Public type alias used only to satisfy the `pub` visibility chain in the
/// sealed-trait bound. Equivalent to `SliceSpec`; kept `pub` to prevent lint
/// `private_interfaces`.
///
/// Not part of the stable public API; may change without notice.
#[doc(hidden)]
pub struct SliceSpecRepr(pub(crate) SliceSpec);

/// Private sealing module — only types with an impl here can satisfy
/// [`IntoSliceRange`].
mod sealed {
    pub trait Sealed {}
    impl Sealed for std::ops::Range<usize> {}
    impl Sealed for std::ops::RangeFrom<usize> {}
    impl Sealed for std::ops::RangeTo<usize> {}
    impl Sealed for std::ops::RangeFull {}
    impl Sealed for std::ops::RangeInclusive<usize> {}
}

/// Supertrait that does the actual range-to-spec conversion. Hidden plumbing
/// for [`IntoSliceRange`]; not intended for downstream implementation.
#[doc(hidden)]
pub trait SliceConvert: sealed::Sealed {
    fn into_repr(self) -> SliceSpecRepr;
}

/// Accepts any standard Rust range type as a slice axis specification.
///
/// This trait is **sealed**: it can only be satisfied by the five standard
/// library range types (`Range`, `RangeFrom`, `RangeTo`, `RangeFull`,
/// `RangeInclusive`). It is not intended for external implementation.
pub trait IntoSliceRange: SliceConvert {}

impl IntoSliceRange for std::ops::Range<usize> {}
impl IntoSliceRange for std::ops::RangeFrom<usize> {}
impl IntoSliceRange for std::ops::RangeTo<usize> {}
impl IntoSliceRange for std::ops::RangeFull {}
impl IntoSliceRange for std::ops::RangeInclusive<usize> {}

impl SliceConvert for std::ops::Range<usize> {
    fn into_repr(self) -> SliceSpecRepr {
        SliceSpecRepr(SliceSpec::Range {
            start: Some(self.start),
            end: Some(self.end),
            step: 1,
        })
    }
}
impl SliceConvert for std::ops::RangeFrom<usize> {
    fn into_repr(self) -> SliceSpecRepr {
        SliceSpecRepr(SliceSpec::Range {
            start: Some(self.start),
            end: None,
            step: 1,
        })
    }
}
impl SliceConvert for std::ops::RangeTo<usize> {
    fn into_repr(self) -> SliceSpecRepr {
        SliceSpecRepr(SliceSpec::Range {
            start: None,
            end: Some(self.end),
            step: 1,
        })
    }
}
impl SliceConvert for std::ops::RangeFull {
    fn into_repr(self) -> SliceSpecRepr {
        SliceSpecRepr(SliceSpec::All)
    }
}
impl SliceConvert for std::ops::RangeInclusive<usize> {
    fn into_repr(self) -> SliceSpecRepr {
        // Use saturating_add to avoid overflow on usize::MAX..=usize::MAX;
        // the resulting spec will fail bounds-checking at build() time.
        SliceSpecRepr(SliceSpec::Range {
            start: Some(*self.start()),
            end: Some(self.end().saturating_add(1)),
            step: 1,
        })
    }
}

// ---- Builder ------------------------------------------------------------

/// Builder for tensor slicing (RFC-008 §10). This is the canonical API.
///
/// Create via [`Tensor::slice`]; finish with [`.build()`](SliceBuilder::build).
///
/// # Examples
///
/// ```
/// use matten::Tensor;
///
/// let t = Tensor::new(vec![1.0,2.0,3.0,4.0,5.0,6.0], &[2, 3]);
///
/// // First row: index axis 0, keep all of axis 1
/// let row = t.slice().index(0).all().build().unwrap();
/// assert_eq!(row.shape(), &[3]);
/// assert_eq!(row.as_slice(), &[1.0, 2.0, 3.0]);
///
/// // First two rows, all columns
/// let top = t.slice().range(0..2).all().build().unwrap();
/// assert_eq!(top.shape(), &[2, 3]);
/// ```
pub struct SliceBuilder<'a> {
    tensor: &'a Tensor,
    specs: Vec<SliceSpec>,
}

impl<'a> SliceBuilder<'a> {
    pub(crate) fn new(tensor: &'a Tensor) -> Self {
        Self {
            tensor,
            specs: Vec::with_capacity(tensor.ndim()),
        }
    }

    /// Selects all elements along the next axis.
    pub fn all(mut self) -> Self {
        self.specs.push(SliceSpec::All);
        self
    }

    /// Selects a single element along the next axis. That axis is removed from
    /// the output shape.
    pub fn index(mut self, index: usize) -> Self {
        self.specs.push(SliceSpec::Index(index));
        self
    }

    /// Selects a range of elements along the next axis.
    ///
    /// Accepts [`Range`](std::ops::Range), [`RangeFrom`](std::ops::RangeFrom),
    /// [`RangeTo`](std::ops::RangeTo), [`RangeFull`](std::ops::RangeFull), or
    /// [`RangeInclusive`](std::ops::RangeInclusive).
    pub fn range<R: IntoSliceRange>(mut self, range: R) -> Self {
        self.specs.push(range.into_repr().0);
        self
    }

    /// Materialises the slice as an owned [`Tensor`].
    ///
    /// # Errors
    ///
    /// Returns [`MattenError::Slice`] on rank mismatch or out-of-bounds specs.
    pub fn build(self) -> Result<Tensor, MattenError> {
        execute_slice(self.tensor, &self.specs, "slice_builder")
    }
}

// ---- slice_str parser ---------------------------------------------------

/// Parses a NumPy-like slice string into a list of [`SliceSpec`]s.
pub(crate) fn parse_slice_str(spec: &str) -> Result<Vec<SliceSpec>, MattenError> {
    let err = |msg: String| MattenError::Slice {
        input: Some(spec.to_string()),
        message: msg,
    };

    if spec.len() > MAX_SLICE_STR_BYTES {
        return Err(err(format!(
            "slice spec exceeds the maximum length of {MAX_SLICE_STR_BYTES} bytes"
        )));
    }

    spec.split(',')
        .map(|part| parse_axis_spec(part.trim(), spec))
        .collect()
}

fn parse_axis_spec(part: &str, full: &str) -> Result<SliceSpec, MattenError> {
    let err = |msg: String| MattenError::Slice {
        input: Some(full.to_string()),
        message: msg,
    };

    if let Ok(n) = part.parse::<usize>() {
        return Ok(SliceSpec::Index(n));
    }
    if !part.contains(':') {
        return Err(err(format!("unrecognised slice component {part:?}")));
    }

    let segments: Vec<&str> = part.splitn(3, ':').collect();
    let parse_opt = |s: &str| -> Result<Option<usize>, MattenError> {
        let s = s.trim();
        if s.is_empty() {
            Ok(None)
        } else {
            s.parse::<usize>()
                .map(Some)
                .map_err(|_| err(format!("expected integer, got {s:?}")))
        }
    };

    let start = parse_opt(segments[0])?;
    let end = parse_opt(segments[1])?;
    let step = if segments.len() == 3 {
        let s = segments[2].trim();
        if s.is_empty() {
            return Err(err(format!(
                "trailing colon in {part:?} is not valid; write without the trailing \
                 colon, or supply a step value (e.g. \"0:10:2\")"
            )));
        } else {
            s.parse::<usize>()
                .map_err(|_| err(format!("step must be a positive integer, got {s:?}")))?
        }
    } else {
        1
    };

    if start.is_none() && end.is_none() && step == 1 {
        return Ok(SliceSpec::All);
    }
    Ok(SliceSpec::Range { start, end, step })
}
