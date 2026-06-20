# RFC-025: Bridge Crate Policy for ndarray, nalgebra, and candle

**Status:** Proposed  
**Target:** v0.16+ design, v0.19+ possible PoC  
**Theme:** Ecosystem bridge strategy  
**Depends on:** RFC-015, RFC-022  
**Related handoff:** `025-bridge-crate-policy-for-ndarray-nalgebra-and-candle-handoff.md`

## 1. Summary

This RFC defines how `matten` should interoperate with specialized Rust mathematical and ML crates without adding their dependencies to core.

The core crate should remain light. Bridges to `ndarray`, `nalgebra`, and `candle` should be implemented as companion crates or carefully isolated optional crates, not as default features.

## 2. Goals

- Avoid lock-in for `matten` users.
- Keep core dependency-light.
- Provide migration paths to performance/specialized crates.
- Avoid forcing `ndarray`, `nalgebra`, or `candle` compile costs on core users.
- Document when users should graduate from `matten`.

## 3. Non-goals

- No bridge implementation in core.
- No wrapper around all external crate APIs.
- No automatic backend switching.
- No GPU bridge in core.
- No attempt to compete with specialized crates.

## 4. External design

Possible future crates:

```text
matten-ndarray
matten-nalgebra
matten-candle
```

Example:

```rust
use matten_ndarray::to_arrayd;

let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
let arr = to_arrayd(&t)?;
```

## 5. Data model

Bridge crates should convert:

```text
matten::Tensor <-> external tensor/matrix type
```

For dynamic tensors:

```text
dynamic Tensor -> try_numeric() -> bridge
```

Do not bridge heterogeneous tensors directly unless a target crate supports it naturally.

## 6. Data lifecycle

```text
matten PoC tensor
  -> bridge conversion
  -> specialized crate workflow
```

Or:

```text
specialized crate result
  -> matten Tensor for simple downstream processing
```

## 7. Events and observable behavior

Bridge conversions may fail if:

- shape rank unsupported;
- data is dynamic;
- target crate requires contiguous layout differently;
- device allocation fails for ML backend.

Failures should use bridge-crate errors, not necessarily `MattenError`.

## 8. Store access

Bridge crates must use public APIs:

- `shape`;
- `as_slice`;
- `to_vec`;
- `Tensor::try_new`.

They must not access internal fields.

## 9. Public API candidates

### 9.1 ndarray

```rust
pub fn to_arrayd(t: &Tensor) -> Result<ndarray::ArrayD<f64>, BridgeError>;
pub fn from_arrayd(a: ndarray::ArrayD<f64>) -> Result<Tensor, BridgeError>;
```

### 9.2 nalgebra

Because nalgebra is strongest for statically/dynamically sized 2D matrices:

```rust
pub fn to_dmatrix(t: &Tensor) -> Result<nalgebra::DMatrix<f64>, BridgeError>;
pub fn from_dmatrix(m: nalgebra::DMatrix<f64>) -> Tensor;
```

### 9.3 candle

```rust
pub fn to_candle_tensor(
    t: &Tensor,
    device: &candle_core::Device,
) -> Result<candle_core::Tensor, BridgeError>;
```

## 10. Cargo feature impact

No core feature in `matten`.

Each bridge crate owns its dependency.

## 11. Internal design

### 11.1 Dependency isolation

Never add this to core:

```toml
ndarray = ...
nalgebra = ...
candle-core = ...
```

unless a future RFC reverses this policy with strong justification.

### 11.2 Shape mapping

- ndarray supports N-D, best general bridge.
- nalgebra supports 2D matrix/vector best.
- candle supports ML tensor but may involve device/dtype constraints.

## 12. Examples

Examples belong in bridge crates.

Core docs may include non-runnable conceptual links until crates exist.

Future examples:

```text
matten-ndarray/examples/to_arrayd.rs
matten-nalgebra/examples/to_dmatrix.rs
matten-candle/examples/to_candle_tensor.rs
```

## 13. Acceptance criteria

- Core `matten` has no bridge dependencies.
- Bridge crates use public API only.
- Conversion limitations are documented.
- Dynamic bridge path requires explicit `try_numeric`.
- Examples compile in bridge crate CI.

## 14. QA checklist

- [ ] Dependency scan confirms isolation
- [ ] 1D/2D/ND conversion tests
- [ ] shape mismatch tests
- [ ] dynamic rejection tests
- [ ] roundtrip tests where possible
- [ ] feature compile tests

## 15. Open questions

1. Should bridge crates share version numbers with `matten`?
2. Which bridge should be first?
3. Should bridges use `MattenError` or crate-specific `BridgeError`?
