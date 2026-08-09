# matten-mlprep

[![Crates.io](https://img.shields.io/crates/v/matten-mlprep.svg)](https://crates.io/crates/matten-mlprep)
[![Docs.rs](https://docs.rs/matten-mlprep/badge.svg)](https://docs.rs/matten-mlprep)
[![license](https://img.shields.io/crates/l/matten-mlprep.svg)](../../LICENSE)

> **Production-ready (`0.45.x` family).** Small, transparent, deterministic preprocessing helpers for
> [`matten::Tensor`](https://crates.io/crates/matten). Not an ML framework. The
> API is stable; still pre-1.0, so pin the minor version.

Part of the [`matten` workspace](../../README.md) — see it for the full family.

## Overview

`matten-mlprep` provides a handful of plain functions for preparing numeric
tensors before handing them to an external tool. There is no model training, no
autograd, no optimizer, and **no hidden randomness** — every function is a pure,
deterministic transform you can read and reason about.

It depends only on core `matten` (no default features); it adds **no**
`ndarray`, `candle`, or `rand` dependency.

## Why / when

Use it for the boring-but-necessary steps between "I have a numeric `Tensor`" and
"I can feed a model": scale features, add an intercept column, carve out a test
set. When you need anything stateful or model-shaped, reach for a real ML crate —
this one deliberately stops at preprocessing.

## Quick start

```rust
use matten::Tensor;
use matten_mlprep::{add_bias_column, standardize_columns, train_test_split};

let x = Tensor::new(vec![1.0, 3.0, 5.0, 7.0], &[4, 1]);
let z = standardize_columns(&x)?;          // zero mean, unit std per column
let z = add_bias_column(&z)?;              // prepend a 1.0 intercept column
let (train, test) = train_test_split(&z, 0.75)?;
# Ok::<(), matten_mlprep::MattenMlprepError>(())
```

> **Dependency style.** This crate depends on `matten`, but official examples import
> `Tensor` (and other core types) from `matten` directly:
>
> ```rust
> use matten::Tensor;
> use matten_mlprep::standardize_columns;
> ```
>
> This keeps ownership and feature selection clear: `Tensor` belongs to `matten`, and
> core features (e.g. `dynamic`) are enabled on the `matten` dependency. Declare both
> `matten` and this crate in your `Cargo.toml` (RFC-032).

## Design notes

- **Convention:** rank-2 only, `rows = samples`, `columns = features`. No silent
  transposition; a non-2D input is an error.
- **Population std.** `standardize_columns` divides by `n` (like scikit-learn's
  `StandardScaler`).
- **Constant columns error, not silently zero.** A zero-variance / zero-range
  column returns `MattenMlprepError::ZeroVariance { column }` so you handle it
  deliberately.
- **`add_bias_column` prepends** the `1.0` column (intercept at index 0).
- **`train_test_split` is ordered and deterministic** — `first floor(n*ratio)`
  rows are train, the rest are test. No shuffle.
- **`train_test_split_seeded` is shuffled and deterministic** (RFC-077) — same
  `n_train` formula, but row order comes from a seeded Fisher-Yates shuffle
  (dependency-free SplitMix64 PRNG, RFC-024 §6). Same `(x, ratio, seed)`
  always reproduces the same split.
- **Dynamic tensors are rejected, not panicked** — regardless of whether the
  companion `dynamic` feature is enabled (RFC-031).

## Public API

The complete surface (the breaking-change baseline for this crate):

```rust
pub fn standardize_columns(x: &Tensor) -> Result<Tensor, MattenMlprepError>;
pub fn minmax_scale_columns(x: &Tensor) -> Result<Tensor, MattenMlprepError>;
pub fn add_bias_column(x: &Tensor)      -> Result<Tensor, MattenMlprepError>;
pub fn train_test_split(x: &Tensor, train_ratio: f64)
    -> Result<(Tensor, Tensor), MattenMlprepError>;
pub fn train_test_split_seeded(x: &Tensor, train_ratio: f64, seed: u64)
    -> Result<(Tensor, Tensor), MattenMlprepError>;

#[non_exhaustive]
pub enum MattenMlprepError {
    DynamicTensor,
    ExpectedMatrix { shape: Vec<usize> },
    InvalidRatio(f64),
    EmptySplit { rows: usize, train_ratio: f64 },
    ZeroVariance { column: usize },
    Matten(matten::MattenError),
}
```

## Limitations

- **Rank-2 only.** Inputs must be `[rows = samples, columns = features]`; other
  ranks are an error. No automatic reshaping or transposition.
- **No data cleaning.** `NaN`/`Inf` propagate to the output; clean your data
  first (e.g. via the core `dynamic` on-ramp) if it is not already numeric-clean.
- **Population std.** `standardize_columns` divides by `n` (not `n-1`).
- **Two split modes.** `train_test_split` is ordered (no shuffle);
  `train_test_split_seeded` (RFC-077) shuffles rows deterministically from a
  `u64` seed.
- **Not for large/streaming data.** These are eager, in-memory transforms.

## Compatibility

- **SemVer:** pre-1.0 (`0.x`). A `0.x` minor bump may break and carries migration
  notes; patch releases are additive only. Pin the release explicitly (`matten-mlprep = "0.45.0"`).
- **MSRV:** Rust 1.85 (edition 2024).
- **`matten`:** released with the `0.45.x` family version (RFC-030). The published
  manifest uses the workspace's broad pre-1.0 core requirement for maintenance
  (`matten = "0"`, RFC-064); users should still declare the matched family explicitly.
- A `1.0` release requires explicit maintainer confirmation.

## More detail

See the workspace [`ROADMAP.md`](../../ROADMAP.md) and RFC-024 (scope) / RFC-028
(design) under [`rfcs/`](../../rfcs/).

## License

Apache-2.0 © nabbisen
