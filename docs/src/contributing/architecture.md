# Architecture

## Source layout

```
src/
  lib.rs          crate root: public re-exports, #![forbid(unsafe_code)]
  error.rs        MattenError + DataFormat (RFC-005)
  shape.rs        validate_shape, strides, coord↔flat helpers (RFC-003)
  tensor.rs       Tensor struct + all impl Tensor blocks
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
    tensor.rs     construction, shape validation, fill ctors, arange
    convert.rs    From/TryFrom
    error.rs      MattenError / DataFormat model
    shape.rs      row-major index helpers
    ops.rs        broadcasting, scalar ops
    reshape.rs    reshape/flatten/transpose/swap_axes/get
    slice.rs      SliceBuilder, slice_str
```

Module style: `foo.rs` + `foo/` coexistence (Rust 2018+). No `mod.rs` files.

## Public re-exports

```rust
pub use crate::error::{DataFormat, MattenError};
pub use crate::tensor::Tensor;
// SliceBuilder is pub and reachable via Tensor::slice() return type;
// users never need to name it in imports.
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

Lean build: `matten = { version = "0.4", default-features = false }`.
The strict compile-time baseline (< 15 s on a modern laptop) applies to the
lean profile. The `default` profile is the convenient PoC baseline; `dynamic`
is off by default.

## Design invariants

1. **One primary user type.** Every user workflow starts with `use matten::Tensor`.
2. **No public lifetimes.** All Phase 1 methods that take or return tensors use
   owned values. Internal helpers may borrow, but lifetimes never appear in the
   public API signature of a method that returns a `Tensor`.
3. **No public generics on Tensor.** The type is `Tensor`, not `Tensor<T>` or
   `Tensor<T, D>`. Generic dtype and dimension support is Phase 2 (`dynamic`).
4. **`#![forbid(unsafe_code)]`.** Any future exception requires a dedicated RFC.
5. **Panic zone / Result zone split.** Convenience APIs for trusted local code
   may panic. Every external boundary returns `Result<_, MattenError>`.
6. **Checked arithmetic everywhere.** Shape products and allocation counts use
   `checked_mul`; overflow surfaces as `MattenError::Allocation`, never wraps.
7. **Row-major canonical order.** All operations that produce a new tensor
   materialise it in row-major contiguous order.

## Milestone sequence

| Version | Milestone | Content |
|---|---|---|
| 0.0.1 | M0 | Crate skeleton, `MattenError`/`DataFormat` surface |
| 0.1.0 | M1 | Core Tensor Contract: shape model, scalar/vector/matrix, index helpers |
| 0.2.0 | M2 | Construction & Conversion: fill ctors, `arange`, `From`/`TryFrom` |
| 0.3.0 | M3 | Broadcasting & Operators: `Add`/`Sub`/`Mul`/`Div`/`Neg`, scalar forms |
| 0.4.0 | M4 | Shape Operations & Slicing: reshape, transpose, `SliceBuilder`, `slice_str` |
| 0.5.0 | M5 | Boundary Integration: serde, `from_json`/`load_json`, `from_csv`/`load_csv` |
| 0.6.0 | M6 | Example suite (RFC-014), CI gates, `cargo check --examples` in CI |

Phase 2 (dynamic `Element` engine, CoW storage) follows as a separate feature
track gated behind `--features dynamic`.
