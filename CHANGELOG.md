# Changelog

All notable changes to `matten` are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project aims to
follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html) once it reaches
a public API (`0.1.0`).

## [0.11.0] - 2026-06-20

**Post-audit release.** Systematic four-layer audit (RFC → implementation,
requirements → tests, tests → codebase, docs → codebase) with all findings
addressed.

### Audit findings and fixes

**GAP 1 (RFC-007 §10):** `Tensor::get_flat(index)` was specified but missing.  
→ Implemented `pub fn get_flat(&self, index: usize) -> Option<f64>` in `tensor.rs`.

**GAP 2 (RFC-011 §10):** RFC specified `is_none(&self) -> Tensor`; implementation used `none_mask()`.  
→ Added `is_none_mask()` as a documented alias; `none_mask()` remains the canonical name.

**GAP 3 (RFC-011 §14):** "default build does not expose Element" test was missing.  
→ Added compile-time isolation test in `src/tests/dynamic.rs`; the lean-core CI job is the definitive proof.

**GAP 5 (RFC-010 §14):** NumPy golden tests were a SHOULD requirement with no implementation.  
→ Added `tests/golden/numpy_broadcasting.json` and `tests/golden/numpy_matmul.json`; integration tests in `tests/smoke.rs` load and verify them.

**GAP 7 (RFC-009 §13):** Allocation/size-limit test for boundary parsers was missing.  
→ Added `from_json_oversized_array_is_err` and `from_json_slice_str_length_limit` tests in `src/tests/parse.rs`.

**Docs gaps fixed:**  
- `docs/src/reference/math.md`: added `min_axis`/`max_axis` section.  
- `docs/src/reference/dynamic.md`: added `none_mask`, `count_none`, `forward_fill_none`, `is_none_mask`, `sum_skip_none` section.  
- `docs/src/reference/public-api-snapshot.md`: added `get_flat` and `is_none_mask`.  
- `docs/src/reference/compatibility.md`: updated `get_flat` from "not implemented" to "implemented".

**Acceptable deferred items (documented):**
- GAP 4 (RFC-012): `materialize()`/`is_materialized()` public API — optional diagnostics, already in compatibility.md.
- GAP 6 (RFC-005/008): Fuzz target for `slice_str` — roadmap says "not required in every CI run".
- GAP 8 (RFC-013): Full NumPy golden suite beyond the two sets added above — SHOULD, not MUST.

## [0.10.0] - 2026-06-20

**Release Candidate — stabilisation milestone.**

### Added

- **Migration guide** (`docs/src/reference/migration.md`) — how and when to
  move data from `matten` to `ndarray`, `nalgebra`, or `candle`; code snippets
  for all three.
- **Compatibility and stability policy** (`docs/src/reference/compatibility.md`)
  — v0.x change policy, feature-flag stability, MSRV, deferred items table.
- **Public API snapshot** (`docs/src/reference/public-api-snapshot.md`) — full
  enumeration of every public item at v0.10.0, serving as the v1.0.0 baseline.

### Fixed

- Malformed whitespace in `axis_reduce` panic message (`"out of range
  for rank-{}"` had extra spaces; corrected to single space).

### Release Decision Checklist (roadmap §12)

1. **Family-car identity preserved?** Yes — no `Tensor<T>`, no BLAS, no GPU,
   no proc-macros. One primary type, two error zones.
2. **Beginner quick-start without lifetime annotations?** Yes — `00_quickstart`
   compiles with only `use matten::Tensor`.
3. **Panics actionable?** Yes — all start with `"matten <category> error in
   <operation>: …"` and include shape/axis context.
4. **I/O boundaries return `Result`?** Yes — `from_json`, `from_csv`,
   `load_json`, `load_csv`, `from_json_dynamic`, `from_csv_dynamic`,
   `try_new`, `try_reshape`, `try_arange`, `slice_str`, `build()`.
5. **Limitations stated honestly?** Yes — README, philosophy.md, migration.md,
   and every example that touches performance warns explicitly.
6. **Public APIs narrow enough?** Yes — 4 root exports, sealed slice traits,
   `SliceSpecRepr` is `#[doc(hidden)]`.
7. **Default compilation lightweight?** Yes — `cargo build --no-default-features`
   pulls in zero optional deps; default profile adds only `serde`/`serde_json`/`csv`.
8. **Examples realistic and runnable?** Yes — 35 examples, all `cargo run`-able,
   all smoke-tested in CI.
9. **Examples demonstrate accepted APIs?** Yes — no example creates new implicit
   scope; RFC-014 review complete.
10. **Feature flags understandable?** Yes — `default = ["serde","json","csv"]`;
    `dynamic` off by default; each feature is documented.
11. **Version appropriate?** `0.10.0` is the release candidate on the v0 line.
    v1.0.0 requires explicit maintainer confirmation.

## [0.9.0] - 2026-06-20

Milestones **M9, M10, M11** — Dynamic hardening, messy-data workflows, and
pattern examples. Closes all remaining open RFCs.

### Added

- **`min_axis(axis)` and `max_axis(axis)`** (`src/math.rs`) — axis reductions
  returning `NaN` when any element along the axis is `NaN` (explicit
  short-circuit, consistent with `min`/`max`).

- **Dynamic utility methods** (`src/dynamic/tensor_ext.rs`, RFC-011 §10):
  - `none_mask()` → Phase 1 tensor of `1.0`/`0.0` indicating `None` positions.
  - `count_none() → usize` — count of `None` elements.
  - `forward_fill_none(fallback)` — carry last non-None value forward in
    row-major order; leading `None` values take `fallback`.
  - `sum_skip_none()` — sum skipping `None`, panicking on non-numeric elements.

- **Dynamic examples** (RFC-014 §6.5, now complete):
  - `dynamic_01_mixed_elements.rs` — building and inspecting mixed tensors.
  - `dynamic_02_missing_values.rs` — detecting and counting `None` values.
  - `dynamic_03_fill_none.rs` — constant fill and forward-fill demonstration.
  - `dynamic_04_numeric_coercion.rs` — coercion policy demonstration.
  - `dynamic_05_dirty_csv_cleanup.rs` — end-to-end messy CSV workflow.

- **Recommended `0.1.x` pattern examples** (RFC-014 §6.4, now complete):
  - `standardize_columns.rs` — z-score normalisation per column.
  - `minmax_scaling.rs` — 0–1 scaling per column.
  - `rowwise_scoring.rs` — weighted row scoring.
  - `column_summary.rs` — per-column mean/min/max/std.
  - `moving_average.rs` — sliding-window mean.
  - `rolling_windows_basic.rs` — rolling sum and max.
  - `pairwise_distance.rs` — Euclidean distance matrix via matmul.
  - `gram_matrix.rs` — Gram matrix (X·Xᵀ).

- **Tests** — 5 new `min_axis`/`max_axis` tests in `src/tests/math.rs`;
  6 new dynamic utility tests in `src/tests/dynamic.rs` (`utility_tests`).

### Changed

- CI `.github/workflows/ci.yml`: added `dynamic_00_quickstart` and all
  pattern examples to the smoke job; added `dynamic+json+csv` feature profile.

### Closed

- **RFC-001 (Threat Model and Boundary Safety Policy)** moved to `rfcs/done/`.
  All controls in RFC-001 are now in place in the codebase:
  - `#![forbid(unsafe_code)]` throughout.
  - Checked arithmetic on all shape/product paths.
  - Parser size limits (`MAX_SLICE_STR_BYTES`, `MAX_JSON_ELEMENTS`, `MAX_DEPTH`).
  - Result-zone at all external boundaries (`try_new`, `from_json`, `from_csv`,
    `load_json`, `load_csv`, `slice_str`, `build()`).
  - Panic-zone clearly labelled and documented.
  - `MattenError::Allocation` for overflow before allocation.

  All 15 RFCs are now in `rfcs/done/`. No open RFCs remain.

## [0.8.0] - 2026-06-20

Milestone **M8 — Dynamic Feature, Phase 2** (RFC-011 + RFC-012).

### Added

- **`dynamic` Cargo feature** — enables Phase 2 heterogeneous tensor support.

- **`Element` enum** (`src/dynamic/element.rs`, RFC-011):
  - Variants: `Float(f64)`, `Int(i64)`, `Text(Arc<str>)`, `Bool(bool)`, `None`.
  - Text representation selected as `Arc<str>` after memory measurement:
    all candidates give `size_of::<Element>() == 24` bytes on 64-bit targets;
    `Arc<str>` chosen for cheap clone in CoW slices.
  - Coercion policy: `Float`/`Int` → `f64` allowed; `Bool`/`Text`/`None` → f64
    explicitly blocked (no silent coercion).
  - `From` conversions for `f64`, `i64`, `i32`, `bool`, `String`, `&str`, `Arc<str>`.
  - Exported as `matten::Element` under `#[cfg(feature = "dynamic")]`.

- **CoW storage** (`src/dynamic/storage.rs`, RFC-012):
  - `DynamicTensor` with `Arc<Vec<Element>>` shared backing storage.
  - `ViewKind::Contiguous` for construction/reshape; `ViewKind::Indexed` for
    non-contiguous slices (shared storage, no element copy).
  - `materialize()` produces fresh contiguous copy; `is_unique()` for CoW check.
  - No reference cycles; drops cleanly.

- **Dynamic `impl Tensor`** (`src/dynamic/tensor_ext.rs`):
  - `from_elements` / `try_from_elements` — construction from `Vec<Element>`.
  - `get_element(&[usize]) → Option<Element>` — safe element access.
  - `to_elements() → Vec<Element>` — extract in row-major order.
  - `is_dynamic() → bool`.
  - `fill_none(value) → Tensor` — replace all `None` values.
  - `try_numeric() → Result<Tensor, MattenError>` — convert to Phase 1 f64
    tensor; fails with `MattenError::Unsupported` on any non-numeric element.
  - `from_json_dynamic(input)` (requires `json` feature) — mixed JSON parser.
  - `from_csv_dynamic(input)` (requires `csv` feature) — mixed CSV parser.

- **Dynamic parsers** (`src/dynamic/parse/`):
  - JSON: null→`None`, bool→`Bool`, string→`Text`, integer→`Int`, float→`Float`.
  - CSV: empty field→`None`, `true`/`false`→`Bool`, integer→`Int`, float→`Float`,
    other→`Text`.

- **Tests** (`src/tests/dynamic.rs`) — 26 new tests across 5 submodules:
  element model, tensor construction, CoW storage, JSON parser, CSV parser.

- **Example** `examples/dynamic_00_quickstart.rs` — runs with
  `--features dynamic,json,csv`.

- **Doc page** `docs/src/reference/dynamic.md`.

### Notes

- Default `f64` Phase 1 API is completely unaffected; the `dynamic` field in
  `Tensor` is `#[cfg(feature = "dynamic")]` and zero-cost when the feature is
  off.
- RFC-011 and RFC-012 moved to `rfcs/done/`.
- Only RFC-001 (threat model) remains in `rfcs/proposed/`.

## [0.7.0] - 2026-06-20

Milestone **M7 — Reductions and Matrix Multiplication** (RFC-010).

### Added

- Whole-tensor reductions: `sum`, `mean`, `min`, `max` — all returning `f64`.
  `NaN` propagates through `sum`/`mean`; `min`/`max` return `NaN` if any
  element is `NaN` (explicit short-circuit, not `f64::min`/`max` which would
  silently ignore it).
- Axis reductions: `sum_axis(axis)`, `mean_axis(axis)` — both returning a new
  `Tensor` with the reduced axis removed.
- `dot(&rhs)` / `matmul(&rhs)` — four supported shapes:
  `[n]×[n]→[]`, `[m,n]×[n]→[m]`, `[n]×[n,p]→[p]`, `[m,n]×[n,p]→[m,p]`.
  `*` remains element-wise.
- New `src/math.rs` module with all of the above.
- `src/tests/math.rs` — 28 tests covering reductions, NaN policy, axis ops,
  all four matmul cases, dimension-mismatch panics, and `*`-vs-`matmul`
  regression.
- Examples 20–24 and 26 completed (were deferred stubs):
  `20_dot_product`, `21_matrix_vector_product`, `22_matrix_multiplication`,
  `23_sum_mean`, `24_min_max`, `26_cosine_similarity`.
- `docs/src/reference/math.md` — reference page for all new operations.

## [0.6.0] - 2026-06-20

Milestone **M6 — Example Suite and CI Hardening** (RFC-013 + RFC-014).

### Added

- **Example suite** — 21 runnable examples covering every implemented API
  surface (RFC-014 §6.3 required set):
  - `00_quickstart` through `12_boundary_error_handling` — creation, shape,
    reshape, operators, scalar ops, broadcasting, transpose, slicing (builder
    and `slice_str`), JSON round-trip, CSV loading, boundary error handling.
  - `20`–`24` — deferred placeholders pending RFC-010 (reductions / matmul).
  - `25_normalize_vector`, `26_cosine_similarity` — L2 normalisation and
    cosine similarity implemented using existing element-wise ops.
- **Fixture files** added: `examples/data/numeric_3x3.csv`,
  `examples/data/malformed_numeric.csv`.
- **CI gates** (`.github/workflows/ci.yml`) extended with:
  - `cargo check --examples` and `cargo test --examples` in every PR job.
  - Full feature-profile test matrix (lean, serde, json, csv, dynamic,
    all-features).
  - MSRV 1.85.0 full test run (was build-only).
  - Separate `smoke` job running the six RFC-014 required smoke examples.

### Notes

- RFC-013 and RFC-014 moved to `rfcs/done/`.
- Examples 20–24 are placeholder stubs clearly marked as deferred pending
  RFC-010 (reductions / matmul).

## [0.5.0] - 2026-06-20

Milestone **M5 — Boundary Integration** (RFC-009).

### Added

- `Tensor::from_json(input: &str)` — accepts the canonical
  `{"shape":[…],"data":[…]}` object form and the convenience nested-array
  form (rank 1 and 2); rejects non-numeric values, ragged arrays, and
  malformed JSON. Gated by the `json` feature (default).
- `Tensor::load_json(path)` — reads a file then delegates to `from_json`.
  File errors map to `MattenError::Io`; parse errors to `MattenError::Parse`.
- `Tensor::from_csv(input: &str)` — rectangular numeric CSV; infers shape
  `[rows, cols]`; includes row/column context in error messages. Gated by
  the `csv` feature (default).
- `Tensor::load_csv(path)` — reads a file then delegates to `from_csv`.
- `Serialize`/`Deserialize` for `Tensor` via the canonical object form
  (gated by the `serde` feature, default).
- New `src/ser.rs` (serde impls) and `src/parse/` module (`json.rs`,
  `csv.rs`).
- Fixture files `examples/data/tensor_2x2.json` and
  `examples/data/numeric_2x3.csv`.
- `src/tests/parse.rs` — 23 boundary tests covering both parsers, both
  file loaders, serde round-trips, error variants, and malformed-input
  safety.

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
