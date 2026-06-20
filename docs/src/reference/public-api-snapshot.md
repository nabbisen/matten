# Public API snapshot — v0.10.0

This page lists every public item in `matten` at v0.10.0. It serves as the
baseline for tracking breaking changes toward v1.0.0.

## Crate root exports

```rust
pub use matten::Tensor;
pub use matten::MattenError;
pub use matten::DataFormat;
#[cfg(feature = "dynamic")]
pub use matten::Element;
pub use matten::SliceBuilder; // returned by Tensor::slice(); rarely named
pub use matten::IntoSliceRange; // sealed trait; not for external impl
pub use matten::SliceConvert;   // sealed supertrait; not for external impl
// SliceSpecRepr: #[doc(hidden)], internal visibility artefact
```

## `Tensor` — construction

| Method | Zone | Notes |
|---|---|---|
| `new(data, shape)` | Panic | convenience literal constructor |
| `try_new(data, shape)` | Result | recoverable |
| `scalar(value)` | Panic | shape `[]` |
| `zeros(shape)` | Panic | |
| `ones(shape)` | Panic | |
| `full(shape, value)` | Panic | |
| `from_vec(data)` | Panic | shape `[len]` |
| `arange(start, end, step)` | Panic | |
| `try_arange(start, end, step)` | Result | |
| `try_from_rows(rows)` | Result | ragged → `Err` |
| `From<Vec<f64>>` | Panic | shape `[len]` |
| `From<Vec<Vec<f64>>>` | Panic | panics on ragged |
| `From<Tensor> for Vec<f64>` | — | consuming |
| `From<&Tensor> for Vec<f64>` | — | borrowing clone |
| `TryFrom<Tensor> for Vec<Vec<f64>>` | Result | fails for non-rank-2 |

## `Tensor` — inspection

| Method | Returns |
|---|---|
| `shape()` | `&[usize]` |
| `ndim()` | `usize` |
| `len()` | `usize` |
| `is_scalar()` | `bool` |
| `is_vector()` | `bool` |
| `is_matrix()` | `bool` |
| `as_slice()` | `&[f64]` |
| `to_vec()` | `Vec<f64>` |
| `into_vec(self)` | `Vec<f64>` |
| `get(coord)` | `Option<f64>` |
| `get_flat(index)` | `Option<f64>` | flat row-major index |

## `Tensor` — shape operations

| Method | Zone |
|---|---|
| `reshape(shape)` | Panic |
| `try_reshape(shape)` | Result |
| `flatten()` | Panic |
| `transpose()` / `t()` | Panic |
| `swap_axes(a, b)` | Panic |

## `Tensor` — slicing

| Method | Zone |
|---|---|
| `slice()` → `SliceBuilder` | — |
| `slice_str(spec)` | Result |
| `SliceBuilder::all()` | — |
| `SliceBuilder::index(n)` | — |
| `SliceBuilder::range(R)` | — accepts `Range`, `RangeFrom`, `RangeTo`, `RangeFull`, `RangeInclusive` |
| `SliceBuilder::build()` | Result |

## `Tensor` — arithmetic operators

| Operator | Notes |
|---|---|
| `&Tensor + &Tensor` | element-wise, broadcasting |
| `&Tensor - &Tensor` | |
| `&Tensor * &Tensor` | element-wise (**not** matmul) |
| `&Tensor / &Tensor` | |
| `-&Tensor` | unary negation |
| `&Tensor ± * / f64` | scalar right |
| `f64 ± * / &Tensor` | scalar left |

## `Tensor` — reductions and linear algebra

| Method | Returns | Notes |
|---|---|---|
| `sum()` | `f64` | NaN propagates |
| `mean()` | `f64` | |
| `min()` | `f64` | NaN if any NaN |
| `max()` | `f64` | NaN if any NaN |
| `sum_axis(axis)` | `Tensor` | axis removed |
| `mean_axis(axis)` | `Tensor` | |
| `min_axis(axis)` | `Tensor` | NaN if any NaN in axis |
| `max_axis(axis)` | `Tensor` | NaN if any NaN in axis |
| `dot(rhs)` | `Tensor` | 4 shape cases |
| `matmul(rhs)` | `Tensor` | alias for `dot` |

## `Tensor` — boundary I/O

| Method | Feature | Zone |
|---|---|---|
| `from_json(input)` | `json` | Result |
| `load_json(path)` | `json` | Result |
| `from_csv(input)` | `csv` | Result |
| `load_csv(path)` | `csv` | Result |
| `Serialize` / `Deserialize` | `serde` | via `serde_json` |

## `Tensor` — dynamic (`#[cfg(feature = "dynamic")]`)

| Method | Zone |
|---|---|
| `from_elements(data, shape)` | Panic |
| `try_from_elements(data, shape)` | Result |
| `get_element(coord)` | `Option<Element>` |
| `to_elements()` | `Vec<Element>` |
| `is_dynamic()` | `bool` |
| `fill_none(value)` | — |
| `none_mask()` | Phase 1 `Tensor` |
| `is_none_mask()` | Phase 1 `Tensor` | RFC-011 alias for `none_mask` |
| `count_none()` | `usize` |
| `forward_fill_none(fallback)` | — |
| `sum_skip_none()` | `f64` |
| `try_numeric()` | Result |
| `from_json_dynamic(input)` | `json` + `dynamic`, Result |
| `from_csv_dynamic(input)` | `csv` + `dynamic`, Result |

## `Element` (`#[cfg(feature = "dynamic")]`)

```rust
pub enum Element { Float(f64), Int(i64), Text(Arc<str>), Bool(bool), None }
```

| Method | Returns |
|---|---|
| `text(s)` | `Element::Text` constructor |
| `is_none()` | `bool` |
| `is_numeric()` | `bool` |
| `try_as_f64()` | `Option<f64>` |
| `as_text()` | `Option<&str>` |
| `as_bool()` | `Option<bool>` |

`From` impls: `f64`, `i64`, `i32`, `bool`, `String`, `&str`, `Arc<str>`.

## `MattenError`

```rust
#[non_exhaustive]
pub enum MattenError {
    Shape { operation: &'static str, message: String },
    Broadcast { left: Vec<usize>, right: Vec<usize> },
    Allocation { requested_elements: usize, message: String },
    Slice { input: Option<String>, message: String },
    Parse { format: DataFormat, message: String },
    Io { path: PathBuf, source: std::io::Error },
    Unsupported { operation: &'static str, message: String },
}
```

`#[non_exhaustive]` — match with a wildcard arm.
Derives only `Debug` (embeds `std::io::Error`). Not `Clone` or `PartialEq`.

## `DataFormat`

```rust
#[non_exhaustive]
pub enum DataFormat { Json, Csv }
```

Derives `Debug, Clone, Copy, PartialEq, Eq`.
