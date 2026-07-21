# Architecture

## Source layout

```
src/
  lib.rs          crate root: public re-exports, #![forbid(unsafe_code)]
  error.rs        MattenError + DataFormat (RFC-005)
  shape.rs        validate_shape, strides, coord↔flat helpers (RFC-003)
  tensor.rs       Tensor struct, constructors, accessors, arange
  tensor/
    ops.rs        shape ops, slicing, boundary APIs (split per 300-ELOC rule)
  limits.rs       MattenLimits — single source of truth for allocation budgets
  convert.rs      From/TryFrom trait impls (RFC-004)
  reshape.rs      permute_axes, reshape helpers (RFC-007)
  slice.rs        SliceSpec, SliceBuilder, slice_str parser (RFC-008)
  ops.rs          ops/ module root
  ops/
    broadcast.rs  broadcast_shape, BroadcastCtx, apply_binary (RFC-006)
    broadcast/
      tests.rs    BroadcastCtx unit tests
    tensor_ops.rs Add/Sub/Mul/Div for &Tensor pairs
    scalar_ops.rs &Tensor op f64, f64 op &Tensor
    unary_ops.rs  Neg
  tests.rs        test module root
  tests/
    tensor.rs     construction, shape validation, fill ctors, arange, limits
    convert.rs    From/TryFrom
    error.rs      MattenError / DataFormat model
    shape.rs      row-major index helpers
    ops.rs        broadcasting, scalar ops
    reshape.rs    reshape/flatten/transpose/swap_axes/get
    slice.rs      SliceBuilder, slice_str
    math.rs       reductions, axis reductions, matmul, NaN policy
    dynamic.rs    dynamic test dispatcher
    dynamic/
      element.rs  Element model tests
      tensor.rs   dynamic construction, JSON, CSV
      lifecycle.rs storage, utility, is_none_mask, lifecycle
      guards.rs   accessor guards, diagnostics
      policy.rs   NumericPolicy, inspection helpers
```

Module style: `foo.rs` + `foo/` coexistence (Rust 2018+). No `mod.rs` files.

## Public re-exports

```rust
// Numeric core — always available:
pub use crate::error::{DataFormat, MattenError};
pub use crate::limits::MattenLimits;
pub use crate::slice::SliceBuilder;
pub use crate::tensor::Tensor;

// Dynamic on-ramp — under #[cfg(feature = "dynamic")]:
pub use crate::dynamic::Element;
pub use crate::dynamic::NumericPolicy;

// Hidden compiler-visibility plumbing (sealed trait chain):
#[doc(hidden)] pub use crate::slice::{IntoSliceRange, SliceConvert, SliceSpecRepr};
```

## Cargo feature matrix

```toml
[features]
default = ["serde", "json", "csv"]
serde   = ["dep:serde"]
json    = ["serde", "dep:serde_json"]
csv     = ["dep:csv"]
dynamic = []
```

Lean build: `matten = { version = "0.37.0", default-features = false }`.
The lean profile is the low-friction baseline. Older design snapshots mentioned
numeric compile-time targets, but those numbers are not maintained release
requirements; current gates focus on feature-matrix builds, dependency
boundaries, and documentation truth. The `default` profile is the convenient
PoC baseline; `dynamic` is off by default.

## Design invariants

1. **One primary user type.** Every user workflow starts with `use matten::Tensor`.
2. **No public lifetimes.** All numeric-core methods that take or return tensors use
   owned values. Internal helpers may borrow, but lifetimes never appear in the
   public API signature of a method that returns a `Tensor`.
3. **No public generics on Tensor.** The type is `Tensor`, not `Tensor<T>` or
   `Tensor<T, D>`. Generic dtype and dimension support belongs to the dynamic path (`dynamic`).
4. **`#![forbid(unsafe_code)]`.** Any future exception requires a dedicated RFC.
5. **Panic zone / Result zone split.** Convenience APIs for trusted local code
   may panic. Every external boundary returns `Result<_, MattenError>`.
6. **Checked arithmetic everywhere.** Shape products and allocation counts use
   `checked_mul`; overflow surfaces as `MattenError::Allocation`, never wraps.
7. **Row-major canonical order.** All operations that produce a new tensor
   materialise it in row-major contiguous order.

## Milestone sequence

| Version | RFC(s) | Content |
|---|---|---|
| 0.0.1 | — | M0: crate skeleton, `MattenError`/`DataFormat` |
| 0.1.0 | RFC-001–005 | M1: Tensor contract, shape model, scalar/vector/matrix |
| 0.2.0 | RFC-004 | M2: construction, `arange`, `From`/`TryFrom` |
| 0.3.0 | RFC-006 | M3: broadcasting, `Add`/`Sub`/`Mul`/`Div`/`Neg` |
| 0.4.0 | RFC-007/008 | M4: reshape, transpose, `SliceBuilder`, `slice_str` |
| 0.5.0 | RFC-009 | M5: serde, `from_json`, `from_csv` |
| 0.6.0–0.7.0 | RFC-010/014 | M6: reductions, matmul, examples, CI gates |
| 0.8.0 | RFC-011/012 | Dynamic alpha: `Element`, CoW `DynamicTensor`, dynamic JSON/CSV |
| 0.9.0 | RFC-013 | Dynamic hardening: `min_axis`/`max_axis`, missing-value helpers |
| 0.10.0–0.11.0 | — | Stabilization, post-audit, `get_flat`, NumPy fixtures |
| 0.12.0–0.13.2 | — | Dynamic lifecycle hardening; accessor guards; sealed slice traits |
| 0.13.3 | RFC-015/020 | API stabilization, release checklist, diagnostics |
| 0.14.0 | RFC-016/017/018 | Dynamic on-ramp: `NumericPolicy`, `MattenLimits`, `try_zeros`/`try_ones`/`try_full` |
| 0.15.0–0.15.1 | RFC-019/021 | Axis reductions, tutorial/example path, file splits |
| 0.16+ | RFC-022–026 | Companion-crate design phase (design-only RFCs) |
