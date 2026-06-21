# matten workspace

[![CI](https://github.com/nabbisen/matten/actions/workflows/ci.yml/badge.svg)](https://github.com/nabbisen/matten/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](./LICENSE)

> A family-car multidimensional array (tensor) library for Rust — and a small,
> optional ecosystem of companion crates around it.

## Overview

This repository is a Cargo workspace. Core `matten` is a developer-experience-first
numeric `Tensor` library for small numerical trials and PoCs. Companion crates
extend workflows *around* the core without enlarging it; core `matten` stays
small and dependency-light.

## Crates

| Crate | Version | Status | What it is |
|---|---|---|---|
| [`matten`](./crates/matten) | 0.16.0 | stable (v0.x) | The core `f64` tensor library: construction, shape ops, broadcasting, slicing, reductions, matmul, JSON/CSV boundary APIs, and an optional `dynamic` ingestion on-ramp. |
| [`matten-ndarray`](./crates/matten-ndarray) | 0.1.0 | experimental | Conversion bridge between `matten::Tensor` and `ndarray::ArrayD<f64>`. |
| [`matten-mlprep`](./crates/matten-mlprep) | 0.1.0 | experimental | Transparent, deterministic preprocessing helpers (standardize, min-max scale, bias column, train/test split). |

Companion crates use **independent SemVer** (RFC-022): a core `matten` version
does not imply any companion's maturity.

## Why / when

Use core `matten` to get a NumPy-like tensor going quickly in Rust without
generics, lifetimes, or view-type puzzles. Reach for a companion crate only when
you need to cross a boundary — e.g. `matten-ndarray` to hand data to the ndarray
ecosystem. Core stays a family car; companions are the trailer hitch.

## Quick start

```toml
[dependencies]
matten = "0.16"
```

```rust
use matten::Tensor;

let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
let b = Tensor::ones(&[2, 2]);
let c = &a + &b;
assert_eq!(c.shape(), &[2, 2]);
```

See each crate's README for its own quick start.

## Design notes

- **Bounded core.** Core `matten` never depends on `ndarray`, `nalgebra`,
  `candle`, `polars`, or any `matten-*` companion. This is enforced in CI by
  `scripts/check-core-dependency-boundary.sh` (RFC-022 §10).
- **Companions are optional.** Depending on `matten` never pulls a companion or
  its heavy dependencies.

## More detail

- Roadmap: [`ROADMAP.md`](./ROADMAP.md) (canonical for v0.16+).
- Design decisions: [`rfcs/`](./rfcs) — see RFC-022 (boundary policy), RFC-025
  (bridge policy), RFC-027 (`matten-ndarray` design).
- Core docs: [`docs/`](./docs) (mdBook).

## License

Apache-2.0 © nabbisen
