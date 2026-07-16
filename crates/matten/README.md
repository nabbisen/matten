# matten

[![crates.io](https://img.shields.io/crates/v/matten.svg)](https://crates.io/crates/matten)
[![docs.rs](https://img.shields.io/docsrs/matten)](https://docs.rs/matten)
[![license](https://img.shields.io/crates/l/matten.svg)](../../LICENSE)

**A family-car Rust array (tensor) library — easy to start, predictable, and friendly for learning, teaching, small workflows, and early prototypes.**

## Overview

`matten` is a developer-experience-first multidimensional array (tensor) library
for Rust. It makes learning-oriented, teaching-oriented, early-stage numerical,
and data-exploration work feel close to NumPy/Pandas ergonomics while staying
native Rust: one concrete `Tensor` type, no visible lifetimes, no generic dtype
puzzles, and human-readable failures.

It deliberately favors **developer experience over peak performance**, and is not
a replacement for `ndarray`, `nalgebra`, or `candle` on hot paths.

## Why / when to use it

Reach for `matten` when you want to learn, teach, or prototype quickly: represent
vectors, matrices, and tensors, do simple shape work and arithmetic, and move
messy JSON/CSV in and out — without wrestling with views, lifetimes, or trait
bounds. When a prototype becomes performance-critical, `matten` is designed to
hand its flat data off to a specialized crate.

> **Status: active pre-1.0 development.** The numeric core is strong;
> The `dynamic` feature supports heterogeneous ingestion (`from_json_dynamic`, `from_csv_dynamic`),
> missing-value cleanup (`fill_none`, `none_mask`, `forward_fill_none`), and explicit conversion to
> numeric tensors (`try_numeric`). Dynamic reshape, slicing, arithmetic, reductions, and serde are
> **intentionally guarded** — call `try_numeric()` first. Dynamic is a guarded heterogeneous-ingestion
> and cleanup feature, not a full dynamic tensor arithmetic engine.
> RFC-000 through RFC-021 are in `rfcs/done/`. v1.0.0 requires explicit maintainer confirmation.
> See [CHANGELOG.md](../../CHANGELOG.md) and [docs.rs](https://docs.rs/matten) for details.
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
matten = { version = "0.35.0", default-features = false }
```

## Design notes

- **One primary type.** Users work through `matten::Tensor`. The public root also
  exposes `MattenError` and `DataFormat`; the dynamic `Element` engine is a feature-gated dynamic on-ramp;
  it is off by default.
- **Two error zones.** Local convenience APIs panic with actionable messages for
  fast PoC feedback; every external boundary returns `Result<_, MattenError>` and
  never panics on ordinary invalid input. `MattenError` derives only `Debug`, so
  match it by variant, not `==`.
- **Convenient by default, lean on request.** `default = ["serde", "json", "csv"]`
  for a smooth first run; `default-features = false` for the lean core.
- **Safe Rust only.** The crate is `#![forbid(unsafe_code)]`.

## More detail

- Full documentation: [docs.rs](https://docs.rs/matten) (API reference) and [nabbisen.github.io/matten](https://nabbisen.github.io/matten/) (mdBook documentation).
- Design and governance: the `rfcs/` pack and roadmap in the repository.
- License: Apache-2.0 (see the root `LICENSE` and `NOTICE`).
