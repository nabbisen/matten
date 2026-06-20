# matten

[![crates.io](https://img.shields.io/crates/v/matten.svg)](https://crates.io/crates/matten)
[![docs.rs](https://img.shields.io/docsrs/matten)](https://docs.rs/matten)
[![CI](https://github.com/nabbisen/matten/actions/workflows/ci.yml/badge.svg)](https://github.com/nabbisen/matten/actions/workflows/ci.yml)
[![license](https://img.shields.io/crates/l/matten.svg)](./LICENSE)

**A family car of Rust array (tensor) library — easy to start, predictable, and friendly for quick numerical trials / PoCs.**

## Overview

`matten` is a developer-experience-first multidimensional array (tensor) library
for Rust. It makes early-stage numerical and data-exploration work feel close to
NumPy/Pandas ergonomics while staying native Rust: one concrete `Tensor` type, no
visible lifetimes, no generic dtype puzzles, and human-readable failures.

It deliberately favors **developer experience over peak performance**, and is not
a replacement for `ndarray`, `nalgebra`, or `candle` on hot paths.

## Why / when to use it

Reach for `matten` when you want to prototype quickly: represent vectors,
matrices, and tensors, do simple shape work and arithmetic, and move messy
JSON/CSV in and out — without wrestling with views, lifetimes, or trait bounds.
When a prototype becomes performance-critical, `matten` is designed to hand its
flat data off to a specialized crate.

> **Status: `0.12.0` — Dynamic lifecycle hardened; all architect review items addressed.** The complete Phase 1 + Phase 2 API is
> in place: construction, shape ops, arithmetic, broadcasting, slicing, reductions,
> matrix multiplication, JSON/CSV serde, and the `dynamic` heterogeneous-data engine.
> All 15 design RFCs are in `rfcs/done/`. v1.0.0 requires explicit maintainer
> confirmation. See [CHANGELOG.md](./CHANGELOG.md) and [docs.rs](https://docs.rs/matten) for details.
> The full mdBook lives in `docs/` in the repository (not included in the crate package).

## Quick start

```rust
use matten::Tensor;

let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
assert_eq!(a.shape(), &[2, 2]);
assert_eq!(a.ndim(), 2);

// Boundary-style construction is recoverable instead of panicking:
use matten::MattenError;
let bad = Tensor::try_new(vec![1.0, 2.0, 3.0], &[2, 2]);
assert!(matches!(bad, Err(MattenError::Shape { .. })));
```

Lean install (smallest dependency footprint):

```toml
matten = { version = "0.1", default-features = false }
```

## Design notes

- **One primary type.** Users work through `matten::Tensor`. The public root also
  exposes `MattenError` and `DataFormat`; the dynamic `Element` engine is a Phase 2,
  feature-gated addition.
- **Two error zones.** Local convenience APIs panic with actionable messages for
  fast PoC feedback; every external boundary returns `Result<_, MattenError>` and
  never panics on ordinary invalid input. `MattenError` derives only `Debug`, so
  match it by variant, not `==`.
- **Convenient by default, lean on request.** `default = ["serde", "json", "csv"]`
  for a smooth first run; `default-features = false` for the lean core.
- **Safe Rust only.** The crate is `#![forbid(unsafe_code)]`.

## More detail

- Full documentation: [docs.rs](https://docs.rs/matten) (API reference) and the `docs/` mdBook in the repository.
- Design and governance: the `rfcs/` pack and roadmap in the repository.
- License: Apache-2.0 (see `LICENSE` and `NOTICE`).
