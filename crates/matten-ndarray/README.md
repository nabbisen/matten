# matten-ndarray

[![Crates.io](https://img.shields.io/crates/v/matten-ndarray.svg)](https://crates.io/crates/matten-ndarray)
[![Docs.rs](https://docs.rs/matten-ndarray/badge.svg)](https://docs.rs/matten-ndarray)
[![license](https://img.shields.io/crates/l/matten-ndarray.svg)](../../LICENSE)

> **Production-ready (`0.28.x` family).** A small conversion bridge between
> [`matten::Tensor`](https://crates.io/crates/matten) and
> `ndarray::ArrayD<f64>`. The scope is closed and the API is stable; still
> pre-1.0, so pin the minor version.

Part of the [`matten` workspace](../../README.md) — see it for the full family.

## Overview

`matten-ndarray` converts between `matten`'s numeric `Tensor` and ndarray's
dynamic-dimension `ArrayD<f64>`, and nothing else. It is the first companion
crate in the `matten` workspace and exists to let you hand data off to the
ndarray ecosystem when you outgrow `matten`'s family-car scope.

It adds **no dependency to core `matten`**, wraps none of the `ndarray` API, and
exposes no view or lifetime types.

## Quick start

```rust
use matten::Tensor;
use matten_ndarray::{from_arrayd, to_arrayd};

let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);

let arr = to_arrayd(&t)?;        // Tensor      -> ArrayD<f64>
let back = from_arrayd(arr)?;     // ArrayD<f64> -> Tensor
# Ok::<(), matten_ndarray::MattenNdarrayError>(())
```

> **Dependency style.** This crate depends on `matten`, but official examples import
> `Tensor` (and other core types) from `matten` directly:
>
> ```rust
> use matten::Tensor;
> use matten_ndarray::to_arrayd;
> ```
>
> This keeps ownership and feature selection clear: `Tensor` belongs to `matten`, and
> core features (e.g. `dynamic`) are enabled on the `matten` dependency. Declare both
> `matten` and this crate in your `Cargo.toml` (RFC-032).

## Design notes

- **Both directions copy.** No zero-copy is claimed; that would need layout
  guarantees out of scope for a copy-based bridge.
- **Logical order is preserved.** `from_arrayd` converts a non-standard-layout
  `ArrayD` (transposed / sliced) by its *logical* element order, never the raw
  backing buffer.
- **Zero-sized axes are rejected.** Core `matten` does not support zero-length
  dimensions, so `from_arrayd` returns an error for them.
- **Dynamic tensors are rejected, not panicked.** Passing a dynamic (`Element`)
  tensor returns `MattenNdarrayError::DynamicTensor` regardless of whether the
  companion `dynamic` feature is enabled (RFC-031); convert it with
  `Tensor::try_numeric()` first.
- **Supported `ndarray`:** the `0.17` minor (CI targets `0.17.2`).

## Conversion contract

`matten-ndarray` follows the bridge [conversion-contract](../../docs/src/migration/bridge-contracts.md)
template. The full contract:

| Dimension | `matten-ndarray` |
|---|---|
| Source / target | `matten::Tensor` ↔ `ndarray::ArrayD<f64>` |
| Direction | Bidirectional: `to_arrayd(&Tensor)`, `from_arrayd(ArrayD<f64>)` |
| Copy / view | Copies both directions; no zero-copy |
| Shape / rank | Shape preserved; rank bounded by core `matten` (over-rank → `Matten` error); a zero-length axis is rejected (→ `ZeroSizedAxis`) |
| Memory order | Row-major logical order both ways; `from_arrayd` honors non-standard layouts |
| Dynamic tensors | Rejected → `DynamicTensor` (unconditional; not a panic) |
| NaN | Passed through as ordinary `f64` |
| Missing values | Not reachable (numeric-only; dynamic rejected first) |
| Integer / text / bool | Not reachable (`f64`-only; dynamic element kinds rejected) |
| Errors | `Result<_, MattenNdarrayError>`; variants `DynamicTensor`, `ZeroSizedAxis`, `NdarrayShape`, `Matten` |
| Performance | Allocates and copies — convert once at the boundary, not in a hot loop |

## Compatibility

- **SemVer:** pre-1.0 (`0.x`). A `0.x` minor bump may contain breaking changes;
  patch releases are additive only. Pin the minor (`matten-ndarray = "0.28"`).
- **MSRV:** Rust 1.85 (edition 2024).
- **`matten`:** shares the `0.28.x` family version (RFC-030).
- **`ndarray`:** supports the `0.17` minor (requirement `"0.17"`; CI targets `0.17.2`). Because
  `to_arrayd`/`from_arrayd` use `ndarray::ArrayD<f64>`, the supported `ndarray` minor is part of the
  bridge's public type identity — build against `ndarray 0.17`. `ndarray 0.17.0` is yanked; use a
  non-yanked `0.17` patch. An `ndarray` minor bump is a compatibility event handled by a
  `matten-ndarray` minor bump (RFC-025 §6).
- A `1.0` release requires explicit maintainer confirmation.

## More detail

See the workspace [`ROADMAP.md`](../../ROADMAP.md), RFC-025 (bridge policy), and
RFC-027 (this crate's design) under [`rfcs/`](../../rfcs/).

## License

Apache-2.0 © nabbisen
