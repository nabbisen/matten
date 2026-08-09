# Public API snapshot

This page lists every public item in `matten` at the current v0.45 release
family. It serves as the baseline for tracking breaking changes toward v1.0.0
and as the review gate required by RFC-015. Core `matten`'s public API most
recently changed in RFC-104, which added `get_mut`/`get_flat_mut` (numeric) and
`get_element_mut` (dynamic) — all three mirroring their existing getters — and
RFC-108, which added `is_empty()`; all four are additive, none changing an
existing signature or message. Before those, RFC-099 added `try_dot`/
`try_matmul`, and RFC-100 added `Display for Tensor` — both additive as well.
Earlier still, RFC-087 added `repeat`, `repeat_axis`, `tile`, and `meshgrid`
(see the shape composition section below). RFC-088 followed with negative
indices in `slice_str`, but changed no public item — a signature-level grammar
extension behind an existing method, not a new row here. RFC-102 is the same
shape: `slice()` and `slice_str()` now accept dynamic tensors and return one
instead of `MattenError::Unsupported` — a behavior change behind two existing
methods, with no signature change and no new row. RFC-105 and RFC-108's
`mm_mul` fix are likewise behavior changes with no new row: `mean`/`min`/`max`/
`argmin`/`argmax` now return `Err`/panic-with-message on an empty tensor
instead of panicking with a raw index error or returning `NaN`/`inf`/`-inf`,
and `dot`/`matmul`/`try_dot`/`try_matmul` no longer panic on a zero-column
product. The RFC-082 streaming feature, RFC-083's functions before it, and
RFC-090's `histogram` were companion-crate (`matten-data`/`matten-stats`)
additions, and the RFC-080/084/085 maturity promotions were label changes;
none of the three touched core `matten`'s root exports.

## Root exports

```rust
// Primary user-facing types
pub use matten::Tensor;
pub use matten::MattenError;
pub use matten::DataFormat;
pub use matten::MattenLimits;  // RFC-018: resource safety limits
pub use matten::SliceBuilder;

// Feature-gated
#[cfg(feature = "dynamic")]
pub use matten::Element;
#[cfg(feature = "dynamic")]
pub use matten::NumericPolicy; // RFC-017: numeric conversion policy

// Compiler-visibility plumbing — #[doc(hidden)], NOT user-facing extension points.
// IntoSliceRange and SliceConvert use a private sealed::Sealed supertrait;
// downstream crates cannot meaningfully implement either trait.
// Users never need to name them in imports.
#[doc(hidden)] pub use matten::IntoSliceRange;
#[doc(hidden)] pub use matten::SliceConvert;
#[doc(hidden)] pub use matten::SliceSpecRepr;
```

## Dynamic tensor behaviour

Methods marked numeric-only **panic** with a `matten unsupported error` message
when called on a dynamic tensor. Call `try_numeric()` to convert first.

| Numeric method group | Dynamic behaviour |
|---|---|
| `reshape`, `flatten`, `transpose`, `swap_axes`, `squeeze`, `expand_dims` | panic |
| `slice()` builder, `slice_str()` | returns `MattenError::Unsupported` |
| Arithmetic operators, scalar operators | panic |
| Reductions (`sum`, `mean`, `min`, `max`, `norm`, `*_axis`) | panic; non-panicking `try_*` forms return `Unsupported` (and `Shape` for axis) |
| `dot` / `matmul` | panic; non-panicking `try_dot` / `try_matmul` forms return `Unsupported` (bespoke `dot/matmul` message) and `Shape` |
| `as_slice`, `to_vec`, `into_vec`, `get`, `get_flat` | panic |
| `From<Tensor> for Vec<f64>`, `From<&Tensor>`, `TryFrom` | panic / `Err` |
| `Serialize` | returns serde error |
| `Display` | **renders**, not panic — the one group where dynamic is the intended use (RFC-100 §5.5); cells use `Element`'s own `Display`, except `Float`, which uses `{:?}` on the inner `f64` (review C1) so it stays distinct from `Int` |

## `Tensor` — formatting (RFC-100)

| Trait | Notes |
|---|---|
| `Debug` (`{:?}`) | single-line, truncated at 8 elements; unchanged by RFC-100 (RFC-020 owns it) |
| `Display` (`{}`) | rank 0/1/2 as a right-aligned grid, `{:?}` per cell; rank > 2 falls back to `shape=... values=...`; rank-1 truncates past 12 values, rank-2 past 12 columns; `{:#}` disables truncation; renders on dynamic tensors using `Element`'s own `Display`, except `Float` (`{:?}` on the inner `f64`, so it stays distinct from `Int` — review C1) |

See [Display / formatting](./math.md#display--formatting-rfc-100) for the full contract and examples.

## `Tensor` — construction

| Method | Returns | Notes |
|---|---|---|
| `new(data, shape)` | `Tensor` | panics on mismatch |
| `try_new(data, shape)` | `Result<Tensor, MattenError>` | |
| `scalar(value)` | `Tensor` | shape `[]` |
| `zeros(shape)` | `Tensor` | |
| `ones(shape)` | `Tensor` | |
| `full(shape, value)` | `Tensor` | |
| `from_vec(data)` | `Tensor` | shape `[n]` |
| `arange(start, end, step)` | `Tensor` | panics on invalid / too large |
| `try_arange(start, end, step)` | `Result<Tensor, MattenError>` | |
| `linspace(start, end, count)` | `Tensor` | RFC-038; `count` evenly spaced, both endpoints; panics if `count == 0` |
| `try_linspace(start, end, count)` | `Result<Tensor, MattenError>` | RFC-038; budget-checked |
| `eye(n)` | `Tensor` | RFC-038; `n × n` identity; panics if `n == 0` |
| `try_eye(n)` | `Result<Tensor, MattenError>` | RFC-038; budget-checked |
| `try_from_rows(rows)` | `Result<Tensor, MattenError>` | ragged → error |
| `try_zeros(shape)` | `Result<Tensor, MattenError>` | RFC-018; budget-checked |
| `try_ones(shape)` | `Result<Tensor, MattenError>` | RFC-018; budget-checked |
| `try_full(shape, value)` | `Result<Tensor, MattenError>` | RFC-018; budget-checked |
| `try_zeros_with_limits(shape, limits)` | `Result<Tensor, MattenError>` | custom budget |
| `try_ones_with_limits(shape, limits)` | `Result<Tensor, MattenError>` | custom budget |
| `try_full_with_limits(shape, value, limits)` | `Result<Tensor, MattenError>` | custom budget |

## `Tensor` — shape inspection

| Method | Returns | Notes |
|---|---|---|
| `shape()` | `&[usize]` | |
| `ndim()` | `usize` | |
| `len()` | `usize` | logical element count |
| `is_scalar()` | `bool` | ndim == 0 |
| `is_vector()` | `bool` | ndim == 1 |
| `is_matrix()` | `bool` | ndim == 2 |
| `is_empty()` | `bool` | RFC-108; `len() == 0`; reachable via slicing, never via a constructor |

## `Tensor` — data access (numeric Tensor)

| Method | Returns | Notes |
|---|---|---|
| `as_slice()` | `&[f64]` | panics on dynamic |
| `to_vec()` | `Vec<f64>` | clone; panics on dynamic |
| `into_vec(self)` | `Vec<f64>` | consuming; panics on dynamic |
| `get(coord)` | `Option<f64>` | panics on dynamic |
| `get_flat(index)` | `Option<f64>` | panics on dynamic |
| `get_mut(coord)` | `Option<&mut f64>` | RFC-104; mirrors `get`; panics on dynamic |
| `get_flat_mut(index)` | `Option<&mut f64>` | RFC-104; mirrors `get_flat`; panics on dynamic |

## `Tensor` — shape operations (numeric Tensor)

| Method | Returns | Notes |
|---|---|---|
| `reshape(shape)` | `Tensor` | panics on mismatch or dynamic |
| `try_reshape(shape)` | `Result<Tensor, MattenError>` | returns `Unsupported` on dynamic |
| `flatten()` | `Tensor` | panics on dynamic |
| `transpose()` | `Tensor` | reverses axes; panics on dynamic |
| `t()` | `Tensor` | alias for `transpose` |
| `swap_axes(a, b)` | `Tensor` | panics on dynamic |
| `squeeze()` | `Tensor` | RFC-038; removes length-1 axes; panics on dynamic |
| `expand_dims(axis)` | `Tensor` | RFC-038; inserts a length-1 axis; panics if `axis > ndim` or dynamic |
| `try_expand_dims(axis)` | `Result<Tensor, MattenError>` | RFC-038; `InvalidArgument` if `axis > ndim`; `Unsupported` on dynamic |

## `Tensor` — shape composition (numeric Tensor, RFC-039)

Associated functions (called as `Tensor::concatenate(...)`), not methods. Both take
a borrowed slice `&[&Tensor]` and reject dynamic inputs.

| Function | Returns | Notes |
|---|---|---|
| `concatenate(tensors, axis)` | `Tensor` | joins an existing axis; panics on empty/shape/axis error or dynamic |
| `try_concatenate(tensors, axis)` | `Result<Tensor, MattenError>` | `InvalidArgument` if empty; `Shape` on rank/dim/axis (`0..rank`); `Unsupported` on dynamic; `Allocation` if oversized |
| `stack(tensors, axis)` | `Tensor` | joins a new axis (rank + 1); panics on empty/shape/axis error or dynamic |
| `try_stack(tensors, axis)` | `Result<Tensor, MattenError>` | `InvalidArgument` if empty; `Shape` if shapes differ or `axis > rank`; `Unsupported` on dynamic; `Allocation` if oversized |
| `repeat(n)` | `Tensor` | repeats each element `n` times, flattens to rank 1; panics on `n = 0` or dynamic |
| `try_repeat(n)` | `Result<Tensor, MattenError>` | `Shape` if `n = 0`; `Unsupported` on dynamic; `Allocation` if oversized |
| `repeat_axis(n, axis)` | `Tensor` | repeats each element `n` times along `axis`, rank preserved; panics on rank-0 input, `axis` out of range, `n = 0`, or dynamic |
| `try_repeat_axis(n, axis)` | `Result<Tensor, MattenError>` | `Shape` on rank-0 input, `axis >= rank`, or `n = 0`; `Unsupported` on dynamic; `Allocation` if oversized |
| `tile(reps)` | `Tensor` | repeats the whole tensor per `reps` (padded with leading 1s if shorter than rank); panics on empty/zero `reps`, `reps` longer than rank, or dynamic |
| `try_tile(reps)` | `Result<Tensor, MattenError>` | `Shape` on empty/zero `reps` or `reps` longer than rank (no rank promotion); `Unsupported` on dynamic; `Allocation` if oversized |
| `meshgrid(x, y)` (associated fn) | `(Tensor, Tensor)` | builds `xy`-indexed coordinate grids from rank-1 `x`/`y`, both shape `[y.len(), x.len()]`; panics on non-rank-1 input or dynamic |
| `try_meshgrid(x, y)` (associated fn) | `Result<(Tensor, Tensor), MattenError>` | `Shape` if either input is not rank-1; `Unsupported` on dynamic; `Allocation` if oversized |

`repeat`, `tile`, and `meshgrid` were added in RFC-087, closing RFC-039 §8's three
deferred shape-composition APIs.

## `Tensor` — slicing (numeric Tensor)

| Method | Returns | Notes |
|---|---|---|
| `slice()` | `SliceBuilder<'_>` | returns `Unsupported` on dynamic |
| `slice_str(spec)` | `Result<Tensor, MattenError>` | returns `Unsupported` on dynamic |

## `SliceBuilder` methods

| Method | Returns |
|---|---|
| `all()` | `SliceBuilder` |
| `index(i)` | `SliceBuilder` |
| `range<R: IntoSliceRange>(r)` | `SliceBuilder` |
| `build()` | `Result<Tensor, MattenError>` |

## `Tensor` — arithmetic (numeric Tensor)

Operator traits implemented for `&Tensor`:
`Add`, `Sub`, `Mul`, `Div`, `Neg` — element-wise with broadcasting.

Scalar operators: `&Tensor + f64`, `&Tensor - f64`, `&Tensor * f64`, `&Tensor / f64`
(and reverse: `f64 + &Tensor`, `f64 - &Tensor`, `f64 * &Tensor`, `f64 / &Tensor`).

All panic on dynamic tensors.

## `Tensor` — elementwise comfort math (numeric Tensor, RFC-038)

| Method | Returns | Notes |
|---|---|---|
| `abs()` | `Tensor` | elementwise; shape preserved |
| `sqrt()` | `Tensor` | negative element → `NaN` |
| `exp()` | `Tensor` | natural exponential `e^x` |
| `ln()` | `Tensor` | `ln(0.0)` → `-inf`, negative → `NaN` |
| `clip(min, max)` | `Tensor` | clamp; panics if `min > max` |
| `try_clip(min, max)` | `Result<Tensor>` | `InvalidArgument` if `min > max`; `Unsupported` on dynamic |

All panic on dynamic tensors (except `try_clip`, which returns `Unsupported`).


| Method | Returns | Notes |
|---|---|---|
| `sum()` | `f64` | |
| `mean()` | `f64` | |
| `min()` | `f64` | NaN if any element is NaN |
| `max()` | `f64` | NaN if any element is NaN |
| `try_sum()` / `try_mean()` / `try_min()` / `try_max()` | `Result<f64, MattenError>` | `Unsupported` on dynamic; NaN propagates as a value (RFC-055) |
| `sum_axis(axis)` | `Tensor` | |
| `mean_axis(axis)` | `Tensor` | |
| `min_axis(axis)` | `Tensor` | NaN propagated per slice |
| `max_axis(axis)` | `Tensor` | NaN propagated per slice |
| `try_sum_axis(axis)` / `try_mean_axis(axis)` / `try_min_axis(axis)` / `try_max_axis(axis)` | `Result<Tensor, MattenError>` | `Shape` if `axis >= rank`; `Unsupported` on dynamic (RFC-056) |
| `argmin()` / `argmax()` | `usize` | flat row-major index; first tie; panics on NaN/dynamic |
| `try_argmin()` / `try_argmax()` | `Result<usize>` | `InvalidArgument` on NaN; `Unsupported` on dynamic |
| `dot(rhs)` | `Tensor` | 4 shape cases; panics on dynamic |
| `matmul(rhs)` | `Tensor` | alias for `dot`; panics on dynamic |
| `try_dot(rhs)` | `Result<Tensor, MattenError>` | `Shape` on the 4 shape cases; `Unsupported` on dynamic (bespoke `dot/matmul` message, RFC-099) |
| `try_matmul(rhs)` | `Result<Tensor, MattenError>` | delegates to `try_dot` (RFC-099) |

## `Tensor` — linalg core-lite (numeric Tensor, RFC-041)

Small linalg-adjacent helpers — not a linear algebra backend. `inverse`,
`determinant`, `solve`, eigen-decomposition, SVD, QR, LU, Cholesky, sparse, and
BLAS/LAPACK are out of scope for core (use `nalgebra` or `ndarray-linalg`).

| Method | Returns | Notes |
|---|---|---|
| `norm()` | `f64` | L2 / Frobenius over all elements; NaN propagates; panics on dynamic |
| `try_norm()` | `Result<f64, MattenError>` | `Unsupported` on dynamic; NaN propagates as a value (RFC-055) |
| `trace()` | `f64` | rank-2 only; rectangular via `min(rows, cols)`; panics on non-rank-2 or dynamic |
| `try_trace()` | `Result<f64, MattenError>` | `Shape` if not rank-2; `Unsupported` on dynamic |
| `outer(other)` | `Tensor` | rank-1 × rank-1 → `[m, n]`; panics on non-rank-1, dynamic, or oversized |
| `try_outer(other)` | `Result<Tensor, MattenError>` | `Shape` if not rank-1; `Unsupported` on dynamic; `Allocation` if oversized |

## `Tensor` — statistics (numeric Tensor, RFC-040)

Population variance only (`ddof = 0`): `var = sum((x_i - mean)^2) / n`,
`std = sqrt(var)`, two-pass, NaN-propagating. Sample variance, quantile,
percentile, histogram, covariance, correlation, and z-score are out of core scope.

| Method | Returns | Notes |
|---|---|---|
| `var()` / `std()` | `f64` | population (`ddof = 0`); NaN propagates; singleton → `0.0`; panics on dynamic |
| `try_var()` / `try_std()` | `Result<f64, MattenError>` | `Unsupported` on dynamic; `InvalidArgument` on empty (RFC-105) |
| `var_axis(axis)` / `std_axis(axis)` | `Tensor` | reduces and drops the axis; panics if `axis >= rank`, dynamic, or the reduced axis has length 0 (RFC-110) |
| `try_var_axis(axis)` / `try_std_axis(axis)` | `Result<Tensor, MattenError>` | `Shape` if `axis >= rank`; `Unsupported` on dynamic; `InvalidArgument` if the reduced axis has length 0 (RFC-110) |

## `Tensor` — boundary / serde

| Method | Returns | Notes |
|---|---|---|
| `from_json(input)` | `Result<Tensor, MattenError>` | |
| `load_json(path)` | `Result<Tensor, MattenError>` | |
| `from_csv(input)` | `Result<Tensor, MattenError>` | numeric only |
| `load_csv(path)` | `Result<Tensor, MattenError>` | |
| `Serialize` (serde) | via feature `serde` | returns serde error on dynamic |
| `Deserialize` (serde) | via feature `serde` | |

## `Tensor` — dynamic (`#[cfg(feature = "dynamic")]`)

| Method | Returns | Notes |
|---|---|---|
| `from_elements(data, shape)` | `Tensor` | |
| `try_from_elements(data, shape)` | `Result<Tensor, MattenError>` | |
| `get_element(coord)` | `Option<Element>` | |
| `get_element_mut(coord)` | `Option<&mut Element>` | RFC-104; mirrors `get_element`; materializes shared storage on first write, releasing the parent's allocation |
| `is_dynamic()` | `bool` | |
| `from_json_dynamic(input)` | `Result<Tensor, MattenError>` | needs `json` |
| `from_csv_dynamic(input)` | `Result<Tensor, MattenError>` | needs `csv` |
| `to_elements()` | `Vec<Element>` | |
| `fill_none(value: impl Into<Element>)` | `Tensor` | |
| `none_mask()` | `Tensor` | 1.0/0.0 mask |
| `is_none_mask()` | `Tensor` | alias for `none_mask` |
| `count_none()` | `usize` | |
| `forward_fill_none(fallback: impl Into<Element>)` | `Tensor` | |
| `sum_skip_none()` | `f64` | skips `None`; panics on non-numeric |
| `try_numeric()` | `Result<Tensor, MattenError>` | strict default |
| `try_numeric_with(policy)` | `Result<Tensor, MattenError>` | RFC-017; explicit policy |
| `numeric_mask()` | `Tensor` | RFC-016; 1.0/0.0 like `none_mask` |
| `is_numeric_convertible()` | `bool` | RFC-016; true if all Float/Int |
| `schema_summary()` | `String` | RFC-016; element-type counts |

## `MattenLimits` (RFC-018)

```rust
pub struct MattenLimits {
    pub max_dimensions: usize, // default: 8
    pub max_elements: usize,   // default: 1 048 576 (~1 M / ~8 MiB)
    pub max_parse_bytes: usize, // default: 128 MiB
}
```

Methods: `MattenLimits::default()`, `MattenLimits::strict()`.

## `NumericPolicy` (RFC-017, `#[cfg(feature = "dynamic")]`)

Controls how `Element` values coerce to `f64` in `try_numeric_with`.

Builder methods: `.strict()`, `.permissive()`, `.allow_bool()`,
`.allow_text_parse()`, `.none_as(value)`, `.none_as_nan()`.

## Conversion traits

| Trait | Notes |
|---|---|
| `From<Vec<f64>> for Tensor` | shape `[n]` |
| `From<Vec<Vec<f64>>> for Tensor` | panics if ragged |
| `From<Tensor> for Vec<f64>` | consuming; panics on dynamic |
| `From<&Tensor> for Vec<f64>` | clone; panics on dynamic |
| `TryFrom<Tensor> for Vec<Vec<f64>>` | requires rank-2; errors on dynamic |

## `MattenError` variants

```rust,ignore
#[non_exhaustive]
pub enum MattenError {
    Shape      { operation: &'static str, message: String },
    Broadcast  { left: Vec<usize>, right: Vec<usize> },
    Allocation { requested_elements: usize, message: String },
    Slice      { input: Option<String>, message: String },
    Parse      { format: DataFormat, message: String },
    Io         { path: PathBuf, source: std::io::Error },
    Unsupported { operation: &'static str, message: String },
    InvalidArgument { operation: &'static str, argument: &'static str, message: String },
}
```

## `DataFormat` variants

```rust
pub enum DataFormat { Json, Csv }
```

## `Element` variants (`#[cfg(feature = "dynamic")]`)

```rust,ignore
pub enum Element {
    Float(f64),
    Int(i64),
    Text(Arc<str>),
    Bool(bool),
    None,
}
```

Methods: `try_as_f64() -> Option<f64>`, `is_numeric() -> bool`,
`is_none() -> bool`, `as_text() -> Option<&str>`, `as_bool() -> Option<bool>`,
and the `text(s)` constructor.
