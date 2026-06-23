# matten

[![license](https://img.shields.io/crates/l/matten.svg)](./LICENSE)
[![matten docs.rs](https://img.shields.io/docsrs/matten?label=matten%20docs)](https://docs.rs/matten)
[![matten-ndarray docs.rs](https://img.shields.io/docsrs/matten-ndarray?label=ndarray%20docs)](https://docs.rs/matten-ndarray)
[![matten-mlprep docs.rs](https://img.shields.io/docsrs/matten-mlprep?label=mlprep%20docs)](https://docs.rs/matten-mlprep)
[![matten-data docs.rs](https://img.shields.io/docsrs/matten-data?label=data%20docs)](https://docs.rs/matten-data)
[![CI Test](https://github.com/nabbisen/matten/actions/workflows/test.yaml/badge.svg)](https://github.com/nabbisen/matten/actions/workflows/test.yaml)
[![CI Docs](https://github.com/nabbisen/matten/actions/workflows/docs.yaml/badge.svg)](https://github.com/nabbisen/matten/actions/workflows/docs.yaml)
[![matten crates.io](https://img.shields.io/crates/v/matten.svg?label=matten)](https://crates.io/crates/matten)
[![matten-ndarray crates.io](https://img.shields.io/crates/v/matten-ndarray.svg?label=ndarray)](https://crates.io/crates/matten-ndarray)
[![matten-mlprep crates.io](https://img.shields.io/crates/v/matten-mlprep.svg?label=mlprep)](https://crates.io/crates/matten-mlprep)
[![matten-data crates.io](https://img.shields.io/crates/v/matten-data.svg?label=data)](https://crates.io/crates/matten-data)

**A family-car multidimensional array (tensor) library for Rust** —
and a small, optional ecosystem of companion crates around it.

## Overview

This repository is a Cargo workspace, where core `matten` stays small and dependency-light.

### Crates

| Crate | Version | Status | What it is |
|---|---|---|---|
| [`matten`](./crates/matten) | 0.20.0 | stable (v0.x) | The core `f64` tensor library: construction, shape ops, broadcasting, slicing, reductions, matmul, JSON/CSV boundary APIs, and an optional `dynamic` ingestion on-ramp. |
| [`matten-ndarray`](./crates/matten-ndarray) | 0.20.0 | production-ready candidate | Conversion bridge between `matten::Tensor` and `ndarray::ArrayD<f64>`. |
| [`matten-mlprep`](./crates/matten-mlprep) | 0.20.0 | beta | Transparent, deterministic preprocessing helpers (standardize, min-max scale, bias column, train/test split). |

All crates share one **family version** (RFC-030): matching numbers mean a
matched, compatible set. A crate's **maturity is the Status column**, not its
version number — a crate at `0.19.0` may still be `beta`.

### matten

`matten` is a developer-experience-first multidimensional array (tensor) library
for Rust. It makes early-stage numerical and data-exploration work feel close to
NumPy/Pandas ergonomics while staying native Rust: one concrete `Tensor` type, no
visible lifetimes, no generic dtype puzzles, and human-readable failures.

It deliberately favors **developer experience over peak performance**, and is not
a replacement for `ndarray`, `nalgebra`, or `candle` on hot paths.

### matten-ndarray

`matten-ndarray` converts between `matten`'s numeric `Tensor` and ndarray's
dynamic-dimension `ArrayD<f64>`, and nothing else. It is the first companion
crate in the `matten` workspace and exists to let you hand data off to the
ndarray ecosystem when you outgrow `matten`'s family-car scope.

It adds **no dependency to core `matten`**, wraps none of the `ndarray` API, and
exposes no view or lifetime types.

### matten-mlprep

`matten-mlprep` provides a handful of plain functions for preparing numeric
tensors before handing them to an external tool. There is no model training, no
autograd, no optimizer, and **no hidden randomness** — every function is a pure,
deterministic transform you can read and reason about.

It depends only on core `matten` (no default features); it adds **no**
`ndarray`, `candle`, or `rand` dependency.

## Why / when

Use core `matten` to get a NumPy-like tensor going quickly in Rust without
generics, lifetimes, or view-type puzzles. Reach for a companion crate only when
you need to cross a boundary — e.g. `matten-ndarray` to hand data to the ndarray
ecosystem. Core stays a family car; companions are the trailer hitch.

## Quick start

### matten

```toml
[dependencies]
matten = "0.20"
```

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

More examples are [here](crates/matten/examples/).

### matten-ndarray

```toml
[dependencies]
matten = "0.20"
matten-ndarray = "0.20"
```

```rust
use matten::Tensor;
use matten_ndarray::{from_arrayd, to_arrayd};

let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);

let arr = to_arrayd(&t)?;         // Tensor      -> ArrayD<f64>
let back = from_arrayd(arr)?;     // ArrayD<f64> -> Tensor
# Ok::<(), matten_ndarray::MattenNdarrayError>(())
```

More examples are [here](crates/matten-ndarray/examples/).

### matten-mlprep

```toml
[dependencies]
matten = "0.20"
matten-mlprep = "0.20"
```

```rust
use matten::Tensor;
use matten_mlprep::{add_bias_column, standardize_columns, train_test_split};

let x = Tensor::new(vec![1.0, 3.0, 5.0, 7.0], &[4, 1]);
let z = standardize_columns(&x)?;          // zero mean, unit std per column
let z = add_bias_column(&z)?;              // prepend a 1.0 intercept column
let (train, test) = train_test_split(&z, 0.75)?;
# Ok::<(), matten_mlprep::MattenMlprepError>(())
```

More examples are [here](crates/matten-mlprep/examples/).

## Design notes

### matten

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

### matten-ndarray

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

### matten-mlprep

- **Convention:** rank-2 only, `rows = samples`, `columns = features`. No silent
  transposition; a non-2D input is an error.
- **Population std.** `standardize_columns` divides by `n` (like scikit-learn's
  `StandardScaler`).
- **Constant columns error, not silently zero.** A zero-variance / zero-range
  column returns `MattenMlprepError::ZeroVariance { column }` so you handle it
  deliberately.
- **`add_bias_column` prepends** the `1.0` column (intercept at index 0).
- **`train_test_split` is ordered and deterministic** — `first floor(n*ratio)`
  rows are train, the rest are test. No shuffle. (A seeded variant is planned;
  see RFC-024 §6.)
- **Dynamic tensors are rejected, not panicked** (with the `dynamic` feature).

## More detail

- Core docs: [`docs/`](./docs) (mdBook).
- Roadmap: [`ROADMAP.md`](./ROADMAP.md) (canonical for v0.16+).
- Design decisions: [`rfcs/`](./rfcs) — see RFC-022 (boundary policy), RFC-025
  (bridge policy), RFC-030 (family versioning).

## License

Licensed under the Apache License, Version 2.0. See [`LICENSE`](./LICENSE) and
[`NOTICE`](./NOTICE).
