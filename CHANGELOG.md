# Changelog

All notable changes to `matten` are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project aims to
follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html) once it reaches
a public API (`0.1.0`).

## [0.4.0] - 2026-06-20

Milestone **M4 — Shape Operations and Slicing** (RFC-007 + RFC-008).

### Added

- **RFC-007 — Reshape, axis operations, and indexing:**
  - `reshape(&self, shape)` / `try_reshape` — element-count-preserving owned
    copy; panics / returns `Err` on mismatch.
  - `flatten(&self)` — collapses any shape to `[len]`; scalar becomes `[1]`.
  - `transpose(&self)` / `t(&self)` — reverses axis order (swap rows/cols for
    rank-2; reverses all axes for higher rank).
  - `swap_axes(&self, a, b)` — swaps two axes; both forms share an internal
    `permute_axes` helper that writes a fresh row-major result.
  - `get(&self, coord)` → `Option<f64>` — safe non-panicking element access.
  - New `src/reshape.rs` module for the above helpers.

- **RFC-008 — Slicing API:**
  - `tensor.slice()` returns a `SliceBuilder`; methods are `.all()`, `.index(n)`,
    and `.range(R)` (accepts `Range`, `RangeFrom`, `RangeTo`, `RangeFull`,
    `RangeInclusive`); `.build()` returns `Result<Tensor, MattenError>`.
  - `tensor.slice_str(spec)` — bounded NumPy-like convenience (max 512 bytes),
    always returns `Result`, never panics on malformed input. Grammar: `:`,
    `n`, `start:end`, `start:`, `:end`, `start:end:step`.
  - `IntoSliceRange` / `SliceConvert` — public sealed-trait pair (only std range
    types satisfy the bound; no external implementation possible).
  - New `src/slice.rs` module.

- **Test reorganisation (module-style fix):**
  - All `mod.rs` files replaced with the 2018+ `foo.rs` + `foo/` layout:
    `src/tests.rs`, `src/ops.rs`, `src/ops/broadcast.rs`.
  - `src/tests/reshape.rs` and `src/tests/slice.rs` added.

## [0.3.0] - 2026-06-20

Milestone **M3 — Broadcasting and Element-wise Operators** (RFC-006).

### Added

- `ops/` module directory with four sub-files:
  - `broadcast.rs` — `broadcast_shape` helper (right-aligned NumPy rules,
    returns `MattenError::Broadcast` on incompatible pairs) and
    `BroadcastCtx` (precomputed zero-stride index mapping, no expanded
    intermediates).
  - `tensor_ops.rs` — `Add`, `Sub`, `Mul`, `Div` for `&Tensor op &Tensor`.
  - `scalar_ops.rs` — all eight scalar forms: `&Tensor op f64` and
    `f64 op &Tensor` (the latter is legal; `&Tensor` is local in the trait
    parameter position and does not violate the orphan rule).
  - `unary_ops.rs` — `Neg` for `&Tensor`.
- Broadcasting handles scalar `[]`, missing leading axes, and `dim == 1`
  on either side without materialising expanded copies.
- `*` remains element-wise; matrix multiplication is RFC-010 / M6.
- Division by zero follows IEEE 754 (`inf`/`-inf`/`NaN`); no error produced.

## [0.2.0] - 2026-06-20

Milestone **M2 — Creation and Conversion APIs** (RFC-004): the full Phase 1
constructor and conversion surface.

### Added

- Fill constructors: `zeros`, `ones`, `full`, `from_vec`.
- `arange(start, end, step)` / `try_arange` — half-open range, checked
  allocation limit, rejection of zero/non-finite step and non-finite bounds.
- `into_vec(self)` — consuming flat extraction without a copy.
- `try_from_rows(rows)` — recoverable rectangular row construction.
- `From<Vec<f64>>` → 1-D tensor; `From<Vec<Vec<f64>>>` → 2-D (panic on ragged).
- `From<Tensor> for Vec<f64>` (consuming) and `From<&Tensor>` (borrowing copy).
- `TryFrom<Tensor> for Vec<Vec<f64>>` — fails for non-rank-2 tensors.
- New `convert.rs` module holding all trait impls and the `flatten_rectangular`
  helper.

### Notes

- Arithmetic, broadcasting, and reshape are M3 (v0.3.0); JSON/CSV I/O is M5.

## [0.1.0] - 2026-06-20

Milestone **M1 — Core Tensor Contract** (RFC-003): the validated shape model and
the observational `Tensor` surface. Still no arithmetic, reshape, or I/O.

### Added

- Full shape validation in a new `shape` module: rank cap (`MAX_NDIM = 8`),
  rejection of zero-sized dimensions, and checked element-count arithmetic.
  Constructors now route through it (`MattenError::Shape` / `Allocation`).
- Scalar semantics: `Tensor::scalar(value)` (shape `[]`, length `1`).
- Observational API: `is_scalar`, `is_vector`, `is_matrix`, and `to_vec`,
  alongside the existing `shape` / `ndim` / `len` / `as_slice`.
- `Tensor` now derives `Clone` and `PartialEq`.
- Internal row-major index helpers (`strides_for_shape`, `coord_to_flat`,
  `flat_to_coord`) with a flatten/unflatten round-trip property test.

### Notes

- `is_empty()` remains intentionally absent (deferred zero-sized-tensor RFC).
- Creation conveniences (`from_vec`, `zeros`, `ones`, `full`, `arange`,
  `From`/`TryFrom`) are M2; arithmetic and broadcasting are M3.

## [0.0.1] - 2026-06-20

The **M0 crate skeleton**: a compiling, lint-clean, CI-ready foundation aligned
with the v2 reconciled specification. No math is implemented yet.

### Added

- Crate manifest with the locked feature matrix
  (`default = ["serde", "json", "csv"]`), edition 2024, and MSRV `1.85`.
- `#![forbid(unsafe_code)]` crate-wide.
- Stable public error surface: the canonical `MattenError` enum (derives only
  `Debug`; matched by variant, never `==`) and the public `DataFormat` enum, with
  manual `Display` and `std::error::Error` implementations.
- Minimal public `Tensor`: `new`, `try_new`, `shape`, `ndim`, `len`, `as_slice`,
  and a shape-first `Debug`. Construction uses checked shape-product arithmetic.
- `examples/hello_tensor.rs` smoke example; unit and integration smoke tests.
- CI (fmt, clippy `-D warnings`, tests, doctests, feature-profile builds, MSRV).
- mdBook documentation scaffold under `docs/`.

### Notes

- `is_empty()` is intentionally absent (deferred to a future zero-sized-tensor RFC).
- The full Core Tensor Contract (scalar/vector/matrix predicates, `to_vec`,
  reshape, transpose, arithmetic, broadcasting, slicing, JSON/CSV) lands in M1
  and later milestones.
