# RFC-025: Bridge Crate Policy for ndarray, nalgebra, and candle

**Status:** Proposed  
**Target:** policy in v0.16; `matten-ndarray` experimental in v0.17; nalgebra/candle deferred  
**Theme:** External numeric ecosystem bridge strategy  
**Depends on:** RFC-015, RFC-022  
**Supersedes:** older plan that bundled ndarray/nalgebra/candle implementation together around v0.19

---

## 1. Summary

This RFC defines bridge-crate policy. It does not require all bridges to be implemented together.

The first concrete bridge should be `matten-ndarray` because it is low-risk, broadly useful for N-dimensional numeric work, and validates the companion-crate model without changing core `matten`.

`matten-nalgebra` and `matten-candle` are deferred and require later per-crate RFCs or explicit reopening.

---

## 2. Goals

- Let users graduate from `matten` to specialized crates.
- Keep core dependency-light.
- Keep bridge APIs small and boring.
- Define external dependency version policy.
- Avoid broad wrapper frameworks.

---

## 3. Non-goals

- No bridge dependency in core `matten`.
- No optional `ndarray` feature in core.
- No broad re-export of external crate APIs.
- No automatic backend switching.
- No Candle/device/dtype bridge until a later RFC.

---

## 4. Bridge order

```text
v0.17
  matten-ndarray experimental

v0.19+
  matten-ndarray production-ready candidate if mature

later
  matten-nalgebra after separate RFC
  matten-candle after separate RFC
```

---

## 5. `matten-ndarray` initial scope

Target API:

```rust
use matten_ndarray::{from_arrayd, to_arrayd};

let arr = to_arrayd(&tensor)?;
let tensor = from_arrayd(arr)?;
```

Allowed:

- `Tensor -> ndarray::ArrayD<f64>`;
- `ndarray::ArrayD<f64> -> Tensor`;
- scalar/vector/matrix/N-D tests;
- clear conversion errors;
- dynamic tensors return `Err` unless explicitly converted to numeric first.

Forbidden:

- zero-copy promise before design;
- wrappers for ndarray operations;
- broad type/lifetime exposure;
- dependency in core `matten`.

### 5.1 Layout and shape correctness rules

An `ndarray::ArrayD<f64>` is **not** guaranteed to be standard row-major
contiguous: transposed, sliced, or otherwise non-standard-stride arrays carry a
logical shape that does not match their backing-buffer order. Core `matten::Tensor`
is always row-major contiguous, so:

- `from_arrayd` MUST convert by **logical element order**, not by copying the raw
  backing buffer. It MUST NOT assume the input is standard row-major. The
  implementation MUST either call ndarray's standard-layout conversion
  (`array.as_standard_layout()`) or iterate in logical order before constructing
  the row-major `Tensor`. Copying the raw buffer of a non-standard-layout array
  would silently transpose or scramble the data.
- `from_arrayd` MUST return `Err` if the input shape contains any **zero-length
  axis**, because core `matten` rejects zero-sized dimensions. The error MUST be
  a clear companion-crate error, not a panic.
- `to_arrayd` produces a standard-layout `ArrayD<f64>` from the (already
  contiguous) `Tensor` buffer.

---

## 6. External dependency version policy

Bridge crates inherit dependency-version risk from their target crates.

For experimental `matten-ndarray`:

- choose one narrow tested `ndarray` minor version;
- document the supported version range;
- treat `ndarray` minor bumps as compatibility events;
- update `matten-ndarray` minor version when external breaking changes affect users.

Do not promise broad `ndarray` version compatibility until CI tests it.

The same rule will matter more strongly for `candle`, whose dependency tree is heavier.

---

## 7. Error policy

Each bridge crate defines its own error type.

Example:

```rust
pub enum MattenNdarrayError {
    DynamicTensor,
    Shape(String),
    NdarrayShape(ndarray::ShapeError),
    Matten(matten::MattenError),
}
```

A dynamic tensor should return `Err(MattenNdarrayError::DynamicTensor)` rather than panic.

---

## 8. Examples

Bridge examples live in bridge crates:

```text
matten-ndarray/examples/to_arrayd.rs
matten-ndarray/examples/from_arrayd.rs
```

Core `matten` must not include:

```text
examples/integration/ndarray_bridge.rs
cargo check --examples --features ndarray
```

Those old in-core feature-gated examples are superseded by companion crates.

---

## 9. Acceptance criteria for `matten-ndarray` experimental

- Conversion roundtrips pass for scalar/vector/matrix/N-D.
- `from_arrayd` preserves logical element order for **non-standard-layout**
  `ArrayD` inputs (transposed / sliced / non-standard-stride), proven by a test
  that converts a transposed `ArrayD` and checks element order.
- `from_arrayd` rejects shapes containing a zero-sized axis with a clear
  companion error (not a panic).
- Dynamic inputs return `Err` (e.g. `MattenNdarrayError::DynamicTensor`), not a panic.
- Copy behavior is documented.
- Supported `ndarray` version is documented.
- Examples compile in `matten-ndarray` CI.
- Core dependency-boundary CI still passes.

---

## 10. Deferred bridges

### `matten-nalgebra`

Deferred until after `matten-ndarray` proves the pattern. It is 2D-focused and should not be bundled with N-D array conversion.

### `matten-candle`

Deferred longer due device, dtype, backend, and ML expectation complexity. Requires separate RFC.
