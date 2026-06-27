# Compatibility and stability policy

## Public API contract

`matten` exports the following public names from the crate root:

```rust
use matten::Tensor;                  // always
use matten::{MattenError, DataFormat}; // always
use matten::MattenLimits;            // always (RFC-018)
use matten::SliceBuilder;            // always; returned by Tensor::slice()
use matten::Element;                 // #[cfg(feature = "dynamic")]
use matten::NumericPolicy;           // #[cfg(feature = "dynamic")] (RFC-017)
```

`SliceBuilder` is returned by `Tensor::slice()` and is held by value; users
do not need to import it by name in the common case.

`IntoSliceRange` and `SliceConvert` are hidden implementation plumbing for
`SliceBuilder::range`. They are exported `#[doc(hidden)]` and use a private
`sealed::Sealed` supertrait so downstream crates cannot meaningfully implement
them. Users never need to name them in imports.

`SliceSpecRepr` is `#[doc(hidden)]`; it is a visibility-chain artefact and
not part of the stable API.

## Panic zone vs Result zone

This split is a **permanent design decision** and will not change:

| Zone | When | Guarantee |
|---|---|---|
| Panic | Local, trusted, literal construction | Rich `matten … error in …:` message |
| Result | Any external boundary (parsing, files, user shapes) | `Result<Tensor, MattenError>` — never panics on ordinary input |

See [Error model](./error-model.md) for the full list of each zone's APIs.

## Feature flags

| Feature | Default | Stability |
|---|---|---|
| `serde` | yes | stable |
| `json` | yes | stable |
| `csv` | yes | stable |
| `dynamic` | no | stable (dynamic ingestion) |

Disabling default features is supported: `default-features = false` gives
the lean core. Enabling `dynamic` does not rename or remove any numeric Tensor API.

## v0.x compatibility

`matten` is on the `v0.x` line. The policy:

- **Breaking changes are allowed** but must be documented in CHANGELOG.
- **Public API churn decreases** after each minor release.
- **Feature-gated additions** (new `#[cfg]` methods) are not breaking.
- **`#[non_exhaustive]`** on `MattenError` and `DataFormat` means match arms
  must include a wildcard — new variants may be added without a semver break.

## v1.0 requirements

v1.0.0 requires explicit maintainer confirmation. Before that can happen:

- public API review must be complete;
- `cargo public-api` snapshot must be taken and approved;
- the panic/Result split must be finalised;
- the `serde` canonical format must be declared stable;
- limitations and non-goals must be clearly documented.

## MSRV

`rust-version = "1.85"` (Rust 2024 edition). The MSRV may be relaxed in a
future release; it will not be raised without a documented breaking change.

## Deferred items

The following items were considered and explicitly deferred:

| Item | Status | Reason |
|---|---|---|
| `is_empty()` | Deferred | Zero-sized dims rejected; always false. Future RFC. |
| `set_flat` | Not implemented | Mutation API deferred. |
| `arange` max elements | `1<<20` (~1 M) | Lowered from `1<<28` in v0.12.0 for OOM safety. |
| `get_flat` | **Implemented** | `Tensor::get_flat(index) -> Option<f64>` added in v0.11.0. |
| Negative slice indices | Deferred | Not in RFC-008 grammar for `0.1.0`. |
| Step slicing `::2` | Supported | `slice_str("0:10:2")` grammar works. |
| Mutable element API | Deferred | Internal Arc-shared storage / CoW is implemented; the public mutation API that would expose CoW is intentionally deferred. |
| Dynamic slicing via builder | Deferred | `slice().build()` works on numeric tensors only. Use `get_element` column-by-column for dynamic. |
| Batched matmul (rank > 2) | Deferred | RFC-010 scope: `[m,n]×[n,p]` maximum. |
| Axis reductions on dynamic | Not needed yet | Convert with `try_numeric()` first. |

## Phase status

The **v0.20 family** completed the materialization phase: the core numeric comfort
APIs (RFC-038 — elementwise, selection, creation, and shape helpers) and the
`30_`–`40_` famous-problem examples program (RFC-043–048). The `matten-data`
CSV→tensor ingestion API first shipped in this family (RFC-034, RFC-035).

The **v0.21 family** delivered selective boundary implementation: shape composition
(`concatenate` / `stack`), small statistics (`var` / `std`), linalg-lite helpers
(`norm` / `trace` / `outer`), and the `matten-data` scope guard. These are additive
under lock-step family versioning (RFC-030).

The **v0.22 family** promotes `matten-data` to **Beta**: the RFC-036 example suite
(`data_00`–`data_05`) plus an explicit malformed-CSV test complete the documented
Beta gate (RFC-023 §9). Maturity is a per-crate Status label, not a separate version,
under lock-step family versioning (RFC-030).

The **v0.23 family** adds the production migration guide (RFC-050–052): when to stay vs.
migrate, per-target playbooks (`ndarray`, `nalgebra`, Polars/Pandas, Candle, NumPy), and the
bridge conversion-contract template with the `matten-ndarray` reference contract. This is
documentation only — no public API, runtime, or dependency change, and core `matten` gains
no dependency.

