# matten-ndarray

[![Crates.io](https://img.shields.io/crates/v/matten-ndarray.svg)](https://crates.io/crates/matten-ndarray)
[![Docs.rs](https://docs.rs/matten-ndarray/badge.svg)](https://docs.rs/matten-ndarray)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](../../LICENSE)

> **Production-ready candidate (`0.19.x` family release).** A small conversion bridge between
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

## Design notes

- **Both directions copy.** No zero-copy is claimed; that would need layout
  guarantees out of scope for an experimental bridge.
- **Logical order is preserved.** `from_arrayd` converts a non-standard-layout
  `ArrayD` (transposed / sliced) by its *logical* element order, never the raw
  backing buffer.
- **Zero-sized axes are rejected.** Core `matten` does not support zero-length
  dimensions, so `from_arrayd` returns an error for them.
- **Dynamic tensors are rejected, not panicked.** With the `dynamic` feature,
  passing a dynamic (`Element`) tensor returns `MattenNdarrayError::DynamicTensor`;
  convert it with `Tensor::try_numeric()` first.
- **Supported `ndarray`:** the `0.16` minor.

## Compatibility

- **SemVer:** pre-1.0 (`0.x`). A `0.x` minor bump may contain breaking changes;
  patch releases are additive only. Pin the minor (`matten-ndarray = "0.19"`).
- **MSRV:** Rust 1.85 (edition 2024).
- **`matten`:** shares the `0.19` family version (RFC-030).
- **`ndarray`:** supports the `0.16` minor. An `ndarray` minor bump is treated as
  a compatibility event and handled by a `matten-ndarray` minor bump (RFC-025 §6);
  broad `ndarray` version compatibility is not promised until CI tests it.
- A `1.0` release requires explicit maintainer confirmation.

## More detail

See the workspace [`ROADMAP.md`](../../ROADMAP.md), RFC-025 (bridge policy), and
RFC-027 (this crate's design) under [`rfcs/`](../../rfcs/).

## License

Apache-2.0 © nabbisen
