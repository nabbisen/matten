# RFC-027: `matten-ndarray` Design and Implementation

**Status:** Implemented (v0.17.0)
**Target:** v0.17.0
**Theme:** First companion crate — prove the boundary model
**Depends on:** RFC-022 (boundary policy), RFC-025 (bridge policy)

---

## 1. Summary

This RFC is the detailed (internal/program) design for `matten-ndarray`, the
first companion crate. It implements the policy in RFC-025 §5–§9 and the
workspace structure deferred from RFC-022 §12.

`matten-ndarray` is a small, boring bridge between `matten::Tensor` and
`ndarray::ArrayD<f64>`. It does nothing else. It lives outside core `matten`,
adds no dependency to core, and follows the companion error and version
policies.

This RFC is the requirement/external/internal-design artifact required by the
project's design-before-coding workflow. RFC-025 is the policy; this RFC is the
buildable specification.

---

## 2. Workspace introduction

v0.17.0 introduces the Cargo workspace (RFC-022 §6, §12 Q1). The core crate
moves into `crates/matten/`; the bridge lands as `crates/matten-ndarray/`.

```text
(workspace root)
  Cargo.toml            [workspace] + [workspace.package] + [workspace.dependencies]
  Cargo.lock            single lockfile
  ROADMAP.md  rfcs/  docs/  scripts/  .github/   shared, repo-only
  crates/
    matten/             core crate (unchanged content; version 0.16.0)
    matten-ndarray/     this crate (version 0.1.0)
```

Shared design docs (`rfcs/`, `docs/`), `ROADMAP.md`, and CI (`scripts/`,
`.github/`) stay at the workspace root because they are cross-crate. Each crate
keeps its own `README.md`, `CHANGELOG.md`, `src/`, `examples/`, and `tests/`.

Versioning is independent per crate (RFC-022 §7): core `matten` remains `0.16.0`
(its published content is unchanged by the move); `matten-ndarray` debuts at
`0.1.0` (experimental). The "v0.17.0" label is the roadmap milestone, not a
single crate version.

## 3. Dependency shape

```toml
[dependencies]
matten = { workspace = true, default-features = false }
ndarray = { workspace = true }

[features]
# Forwarded so dynamic-tensor rejection can compile only when relevant.
dynamic = ["matten/dynamic"]
```

`default-features = false` keeps the bridge lean: it needs only the numeric core
of `matten` (`Tensor`, `shape`, `as_slice`/`to_vec`, `try_new`), not
serde/json/csv. The core dependency-boundary check (RFC-022 §10) is unaffected:
`matten-ndarray -> ndarray` is allowed; core `matten` still has no `ndarray`.

`ndarray` is pinned to the `0.16` minor (RFC-025 §6). A future `ndarray` minor
bump is a compatibility event handled by a `matten-ndarray` minor bump.

## 4. Public API

```rust
use matten_ndarray::{from_arrayd, to_arrayd, MattenNdarrayError};

let arr = to_arrayd(&tensor)?;          // Tensor      -> ArrayD<f64>
let tensor = from_arrayd(arr)?;          // ArrayD<f64> -> Tensor
```

Two free functions and one error type. No wrapper types, no broad re-export of
`ndarray`, no lifetime/view exposure (RFC-025 §3).

### 4.1 `to_arrayd`

```rust
pub fn to_arrayd(tensor: &matten::Tensor) -> Result<ndarray::ArrayD<f64>, MattenNdarrayError>;
```

- If the `dynamic` feature is enabled and the tensor is dynamic, return
  `Err(MattenNdarrayError::DynamicTensor)` — never let core panic.
- Otherwise read the contiguous row-major buffer (`tensor.to_vec()`) and build a
  standard-layout `ArrayD<f64>` with `ArrayD::from_shape_vec(IxDyn(shape), data)`.
- Scalars (`shape == []`) map to a rank-0 `ArrayD` of length 1.

### 4.2 `from_arrayd`

```rust
pub fn from_arrayd(array: ndarray::ArrayD<f64>) -> Result<matten::Tensor, MattenNdarrayError>;
```

- Reject any shape containing a zero-length axis with
  `Err(MattenNdarrayError::ZeroSizedAxis(shape))` — core `matten` forbids
  zero-sized dims (RFC-025 §5.1).
- Convert by **logical** element order, not raw buffer order. `ArrayD` may be
  non-standard layout (transposed/sliced/non-standard strides); copying the raw
  buffer would silently transpose data. Use `array.as_standard_layout()` and
  iterate, producing row-major data.
- Build via `matten::Tensor::try_new(data, shape)` (boundary-safe); map any
  `MattenError` (e.g. rank > 8) to `MattenNdarrayError::Matten`.

## 5. Error type (RFC-025 §7)

```rust
#[derive(Debug)]
#[non_exhaustive]
pub enum MattenNdarrayError {
    /// A dynamic (`Element`) tensor was passed; convert with `try_numeric()` first.
    DynamicTensor,
    /// The ndarray shape contained a zero-length axis (unsupported by core matten).
    ZeroSizedAxis(Vec<usize>),
    /// ndarray could not build the target array (e.g. shape/length mismatch).
    NdarrayShape(ndarray::ShapeError),
    /// Core matten rejected the conversion (e.g. rank exceeds MAX_NDIM).
    Matten(matten::MattenError),
}
```

- Defines its own error type; core `MattenError` does not grow companion
  variants (RFC-022 §8).
- Implements `Display` and `std::error::Error` (with `source()` for the wrapped
  cases). `#[non_exhaustive]` so future variants are not breaking.
- Dynamic input returns `Err`, never a panic (RFC-025 §7).

## 6. Copy behavior (documented honestly)

Both directions **copy**. `matten::Tensor` owns a contiguous `Vec<f64>`;
`to_arrayd` hands ndarray an owned buffer, and `from_arrayd` materializes an
owned row-major `Vec<f64>`. No zero-copy is claimed (RFC-025 §3); zero-copy
would require layout guarantees that are out of scope for an experimental bridge.

## 7. Test plan (validates the design, not the code)

| Test | Validates |
|---|---|
| roundtrip scalar `[]` | rank-0 handling |
| roundtrip vector `[n]` | rank-1 |
| roundtrip matrix `[m,n]` | rank-2, row-major order |
| roundtrip N-D `[2,3,4]` | rank-N |
| `from_arrayd` on a transposed `ArrayD` | **logical-order** conversion (§4.2) |
| `from_arrayd` on a sliced (non-contiguous) `ArrayD` | non-standard layout |
| `from_arrayd` with a zero-length axis | `ZeroSizedAxis` rejection |
| `to_arrayd` on a dynamic tensor (feature on) | `DynamicTensor` rejection, no panic |
| value equality after roundtrip | data fidelity |

Examples (`examples/to_arrayd.rs`, `examples/from_arrayd.rs`) live in this crate,
not core (RFC-025 §8), and run in CI.

## 8. Acceptance criteria

Mirrors RFC-025 §9:

- roundtrips pass for scalar/vector/matrix/N-D;
- `from_arrayd` preserves logical order for non-standard-layout inputs;
- `from_arrayd` rejects zero-sized axes with a clear error;
- dynamic input returns `Err`, not panic;
- copy behavior and supported `ndarray` version documented;
- examples compile/run in CI;
- core dependency-boundary check still passes.

## 9. Non-goals

- No `ndarray` operation wrappers, no broad re-export, no view/lifetime API.
- No zero-copy.
- No `nalgebra`/`candle` (separate later RFCs).
- No change to core `matten`'s public API.

## 10. Open questions

None blocking. If users need `ArrayView`/`ArrayViewMut` bridges later, that is a
separate additive proposal once the owned-conversion path has proven stable.
