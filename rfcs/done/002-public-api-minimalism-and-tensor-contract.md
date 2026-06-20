# RFC-002: Public API Minimalism and `Tensor` Contract

> RFC status: Implemented (0.1.0)
> Project: `matten`  
> Target audience: library implementers, maintainers, reviewers  
> Design level: external design first; internal design where needed  
> Last updated: 2026-06-18

## 1. Summary

This RFC defines the public API minimalism policy for `matten`. The crate is centered on a single primary user type, `matten::Tensor`. Default Phase 1 is fixed to `f64`, does not expose generic dtype parameters, does not expose lifetime-bearing views, and does not require users to import internal layout or storage traits. Phase 2 may expose `Element` under the `dynamic` feature, but `Tensor` remains the primary container.

## 2. Motivation

The core value proposition of `matten` is that Rust developers can prototype multidimensional numerical and messy business data workflows without learning a complex static shape system. If the public API exposes `Tensor<T, D>`, explicit lifetimes, view types, hidden storage engines, or many traits, the library becomes another performance-oriented F1 tool rather than the intended family car.

## 3. Goals

- Freeze `Tensor` as the primary user-facing type.
- Avoid public generics for the default numeric tensor.
- Avoid public lifetime propagation for common operations.
- Keep root exports narrow and stable.
- Make feature-gated additions additive, not disruptive.
- Preserve a migration path to performance libraries through standard conversions.

## 4. Non-goals

- No type-level dimensions.
- No public `Tensor<T, D>` in Phase 1.
- No public borrowed tensor view type in Phase 1.
- No macro-first user experience.
- No attempt to hide the fact that large Phase 1 tensors may allocate heavily.

## 5. Cargo Features

| Feature | Public API effect |
|---|---|
| default | Exposes `Tensor`, `MattenError`, and `DataFormat`. |
| `serde` | Enables `Serialize`/`Deserialize` for `Tensor`. |
| `json` | Adds `from_json` / `load_json` (implies `serde`). |
| `csv` | Adds `from_csv` / `load_csv`. |
| `dynamic` | Adds `Element` and mixed-data methods while preserving `Tensor`. |

Root exports:

```rust
pub use crate::tensor::Tensor;
pub use crate::error::{MattenError, DataFormat};

#[cfg(feature = "dynamic")]
pub use crate::element::Element;
```

## 6. Data Model

The external data model is intentionally small:

```rust
pub struct Tensor { /* fields private */ }

pub enum MattenError { /* canonical in RFC-005 */ }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataFormat { Json, Csv } // canonical in RFC-005

#[cfg(feature = "dynamic")]
pub enum Element {
    Float(f64),
    Int(i64),
    Text(/* representation decided by RFC-011 */),
    Bool(bool),
    None,
}
```

The fields of `Tensor` are private. Users observe tensors only through methods such as `shape`, `ndim`, `len`, `as_slice`, conversions, arithmetic, and serializers.

## 7. Data Lifecycle

User-visible lifecycle:

1. Create a `Tensor` from vectors, fill constructors, JSON/CSV, or dynamic data.
2. Inspect shape and data through safe accessors.
3. Transform shape using reshape/axis/slice APIs.
4. Compute using element-wise operators, reductions, and explicit matmul/dot.
5. Export through vectors or serde.

Internal storage may change between features or releases while the public lifecycle remains stable.

## 8. Events

There is no public event system. The following public-lifecycle events are relevant for documentation and tests:

- tensor constructed;
- tensor cloned/materialized;
- shape changed;
- arithmetic result allocated;
- boundary parse accepted or rejected;
- feature-gated dynamic value coerced or rejected.

These are not callbacks. They are conceptual events used to define expected behavior.

## 9. Store Access

`Tensor` storage is private. Users must not depend on field layout.

Allowed direct data access in Phase 1:

```rust
pub fn as_slice(&self) -> &[f64];
pub fn to_vec(&self) -> Vec<f64>;
impl From<Tensor> for Vec<f64>;
```

`as_slice` is allowed because Phase 1 guarantees contiguous owned storage. Under `dynamic`, `as_slice` for `f64` may remain available only for numeric tensors if an RFC explicitly defines the behavior; otherwise dynamic-specific accessors must be added without breaking default users.

## 10. Public API

Minimum public methods governed by this RFC:

```rust
impl Tensor {
    pub fn shape(&self) -> &[usize];
    pub fn ndim(&self) -> usize;
    pub fn len(&self) -> usize;
    pub fn is_scalar(&self) -> bool;
    pub fn as_slice(&self) -> &[f64]; // Phase 1/default numeric contract.
}
```

Public examples must be simple:

```rust
use matten::Tensor;

let a = Tensor::ones(&[2, 2]);
let b = Tensor::zeros(&[2, 2]);
let c = &a + &b;
assert_eq!(c.shape(), &[2, 2]);
```

Examples must not require:

```rust
use matten::internal::Phase1Storage;
use matten::traits::BroadcastShape;
let x: Tensor<f64, Ix2> = todo!();
```

## 11. Internal Design

### 11.1 Module layout

Recommended initial layout:

```text
src/
  lib.rs
  tensor.rs
  shape.rs
  error.rs
  ops.rs
  constructors.rs
  format.rs
  slice.rs
  io.rs
  dynamic/      # feature-gated later
```

Internal modules may define traits, helper structs, and feature-specific storage engines, but ordinary users must not import them.

### 11.2 Sealed internal traits

Internal traits may be used to organize implementation, but they should be `pub(crate)` or sealed. A public extension trait is not allowed unless a later RFC justifies it.

### 11.3 Compatibility with future dynamic storage

`Tensor` must be designed so that feature-gated storage changes do not require renaming the public type. Two acceptable internal patterns:

```rust
pub struct Tensor {
    inner: TensorInner,
}
```

or feature-swapped internal fields:

```rust
pub struct Tensor {
    #[cfg(not(feature = "dynamic"))]
    data: Vec<f64>,
    #[cfg(feature = "dynamic")]
    inner: dynamic::DynamicTensor,
    shape: Vec<usize>,
}
```

The second option is riskier because conditional fields can complicate code paths. RFC-012 will decide the Phase 2 storage form.

## 12. Error Handling

This RFC does not define error variants. It requires API categorization:

- local convenience APIs may panic if documented;
- boundary APIs return `Result<Tensor, MattenError>`;
- root API must expose `MattenError` and `DataFormat`.

## 13. Testing

- Public examples should compile without explicit lifetimes.
- No rustdoc example should import internal modules.
- `cargo test` must cover default build.
- `cargo test --features dynamic` must confirm Phase 1 examples still compile when dynamic is enabled.
- Public API audit should fail if root exports grow without RFC approval.

## 14. Acceptance Criteria

- `use matten::Tensor;` is sufficient for default examples.
- No default public `Tensor<T>` generic exists.
- No public lifetime-bearing view appears in Phase 1 examples.
- `Element` is exposed only under `dynamic`.
- Internal storage remains private.
- This RFC is referenced by all public API expansion RFCs.
