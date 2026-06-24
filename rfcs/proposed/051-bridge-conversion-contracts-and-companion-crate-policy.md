# RFC-051 — Bridge Conversion Contracts and Companion-Crate Policy

**Project:** `matten`  
**Milestone:** v0.23+ planning  
**Status:** Accepted (architect ruling 2026-06-24); implementation planned for v0.23.x  
**Document type:** RFC  
**Primary audience:** bridge-crate maintainers, library users, API reviewers  
**Depends on:** RFC-025 Bridge Crate Policy, RFC-032 Companion Dependency and Import Convention, RFC-050 Production Migration Guide  
**Related:** `matten-ndarray`, future `matten-nalgebra`, future `matten-polars`, future `matten-candle`  

---

## 1. Summary

This RFC defines conversion-contract rules for `matten` bridge crates.

Bridge crates should help users move between `matten` and production ecosystems without adding heavy dependencies to core `matten`.

Existing `matten-ndarray` is the model:

```text
core matten:
  owns Tensor and core semantics

bridge crate:
  depends on matten and one target ecosystem
  converts explicitly
  documents copy/view behavior
  rejects unsupported tensor kinds clearly
  does not re-export core Tensor
```

This RFC generalizes that policy for future bridge crates.

---

## 2. Motivation

Migration guidance is not enough if users cannot safely move data.

Users need to know:

```text
Does this conversion copy?
Does it preserve shape?
Does it preserve row-major order?
Does it accept dynamic tensors?
Does it preserve missing values?
Does it preserve integer/text/bool values?
What happens to NaN?
What errors can happen?
```

Without a consistent conversion contract, every bridge crate becomes a one-off design.

---

## 3. Goals

1. Define common conversion contract language for all bridge crates.
2. Preserve core dependency boundaries.
3. Make copy/view behavior explicit.
4. Make dynamic tensor handling explicit.
5. Make dtype/value conversion policies explicit.
6. Keep each bridge crate small and target-specific.
7. Ensure bridge crates do not hide target-library semantics behind `matten`.

---

## 4. Non-goals

This RFC does not authorize:

```text
[ ] adding target-library dependencies to core matten
[ ] making bridge crates re-export Tensor
[ ] universal conversion among all libraries
[ ] automatic code migration
[ ] zero-copy guarantees unless explicitly designed and tested
[ ] dtype-rich tensor support in core matten
[ ] dataframe semantics in matten-data
```

---

## 5. Bridge crate naming

Recommended naming:

```text
matten-ndarray
matten-nalgebra
matten-polars
matten-candle
```

Avoid:

```text
matten-production
matten-super
matten-all
matten-interop
```

because those imply a broad universal layer.

---

## 6. Dependency policy

Each bridge crate may depend on:

```text
matten
one target ecosystem crate
small error/dev dependencies if justified
```

Example:

```toml
[dependencies]
matten = { version = "0.x", default-features = false }
ndarray = "..."
```

Bridge crates must not add dependencies to core `matten`.

Bridge crates must not make other bridge crates required.

Allowed:

```text
matten-ndarray -> matten + ndarray
matten-nalgebra -> matten + nalgebra
```

Disallowed:

```text
matten -> ndarray
matten -> nalgebra
matten-data -> polars
matten-ndarray -> matten-nalgebra, unless separately justified
```

---

## 7. Import and re-export policy

Bridge crates must follow RFC-032:

```rust
use matten::Tensor;
use matten_ndarray::to_arrayd;
```

They must not do:

```rust
pub use matten::Tensor;
```

Rationale:

```text
Tensor ownership remains clear.
Users keep explicit dependencies.
Core feature access remains clear.
```

---

## 8. Conversion contract template

Every bridge crate must document:

```text
source type
target type
direction
copy or view
shape/rank policy
memory order policy
dynamic tensor policy
NaN policy
missing-value policy
integer/text/bool policy
error behavior
performance caveat
examples
```

Suggested table:

| Field | Required answer |
|---|---|
| Direction | `Tensor -> Target`, `Target -> Tensor`, or both |
| Copy behavior | Always copy / may copy / view |
| Shape behavior | Preserved / restricted / transformed |
| Dynamic tensors | accepted / rejected / converted with policy |
| Missing values | rejected / filled / preserved by target |
| Unsupported values | error variant |
| Target limitation | documented |

---

## 9. Error contract

Each bridge crate should define its own error enum:

```rust
pub enum MattenNdarrayError { ... }
pub enum MattenNalgebraError { ... }
```

It should preserve `MattenError` where relevant but not expose target-library internals unnecessarily.

Common error categories:

```text
UnsupportedTensorKind
UnsupportedRank
UnsupportedShape
UnsupportedValue
TargetError
MattenError
```

For bridge crates, `UnsupportedRank` is often better than overloading `Shape`.

These names are **illustrative categories for future bridge crates, not a required enum
schema.** Existing bridge crates do not need to rename or expand their public error enums
if their current variants already document the conversion contract clearly. For example,
`matten-ndarray` uses `DynamicTensor` / `ZeroSizedAxis` / `NdarrayShape` / `Matten`
(wrapping core `MattenError`) and is considered compliant as-is.

---

## 10. Dynamic tensor policy

Default rule:

```text
Bridge crates should reject dynamic Tensor values unless the target-specific RFC
explicitly defines a value conversion policy.
```

Reason:

```text
core dynamic is an ingestion/on-ramp path, not a dtype-rich compute engine
```

Accepted dynamic migration path:

```text
dynamic Tensor / Table
  -> cleanup / fill / explicit try_numeric
  -> numeric Tensor
  -> bridge conversion
```

If a target supports richer value types, such as a dataframe library, conversion must be target-specific and explicit.

---

## 11. `matten-ndarray` contract status

`matten-ndarray` should remain the reference bridge.

Expected contract:

```text
Tensor -> ndarray::ArrayD<f64>
ArrayD<f64> -> Tensor
copies both directions
numeric tensors only
rejects dynamic tensors
preserves shape
preserves row-major logical element order
```

Any deviation must be documented.

---

## 12. Future `matten-nalgebra`

`matten-nalgebra` should be narrower than `matten-ndarray`.

Likely conversions:

```text
rank-1 Tensor -> nalgebra::DVector<f64>
rank-2 Tensor -> nalgebra::DMatrix<f64>
DVector<f64> -> Tensor [n]
DMatrix<f64> -> Tensor [rows, cols]
```

Non-goals:

```text
N-D tensors
decompositions in matten
automatic solve/inverse wrappers
```

---

## 13. Future `matten-polars`

`matten-polars` should be treated cautiously.

It must not turn `matten-data` into a dataframe library.

Possible direction:

```text
matten-data Table -> polars DataFrame
selected numeric columns -> Tensor
```

Potentially risky and should require its own RFC.

---

## 14. Future `matten-candle`

`matten-candle` should focus on migration to ML tensor/model workflows.

Possible direction:

```text
numeric Tensor -> candle_core::Tensor
```

Risks:

```text
device handling
dtype handling
CUDA/Metal features
model loading expectations
```

This must be a separate RFC before implementation.

---

## 15. Acceptance criteria

This RFC is implemented when:

```text
[ ] Bridge conversion contract template exists in docs.
[ ] `matten-ndarray` docs are audited against the template (documentation-only — see note below).
[ ] Future bridge-crate RFC template references this RFC.
[ ] Release-doc checks ensure bridge crates do not re-export core Tensor.
[ ] Core `matten` gains no new target-library dependency.
```

The `matten-ndarray` audit is **documentation-only**: map the existing
`MattenNdarrayError` variants into the contract table. Do **not** introduce an API change
(such as adding `UnsupportedRank` / `UnsupportedTensorKind`) solely to satisfy this audit;
the existing error surface already documents the conversion contract.

---

## 16. Implementation plan

1. Add `docs/src/migration/bridge-contracts.md`.
2. Update `matten-ndarray` README with the contract table.
3. Add future bridge crate RFC template.
4. Add acceptance checklist to bridge handoffs.
5. Defer new bridge crates until target-specific demand exists.

---

## 17. Open questions

1. Should `matten-nalgebra` be prioritized before Polars/Candle?
2. Should bridge crates expose free functions only, or also extension traits?
3. Should conversion functions use `to_*` / `from_*` naming consistently?
   **Resolved (architect ruling 2026-06-24):** yes. `to_<target>` / `from_<target>` is the
   default bridge convention, following the `to_arrayd` / `from_arrayd` precedent in
   `matten-ndarray` (e.g. `to_dmatrix` / `from_dmatrix`, `to_dvector` / `from_dvector`).
   A target may deviate only if it has a stronger idiom, justified in that bridge's RFC.
4. Should all bridge crates include example parity with a `matten` original example?
