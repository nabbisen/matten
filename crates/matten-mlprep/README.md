# matten-mlprep

[![Crates.io](https://img.shields.io/crates/v/matten-mlprep.svg)](https://crates.io/crates/matten-mlprep)
[![Docs.rs](https://docs.rs/matten-mlprep/badge.svg)](https://docs.rs/matten-mlprep)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](../../LICENSE)

> **Experimental (0.1.x).** Small, transparent, deterministic preprocessing
> helpers for [`matten::Tensor`](https://crates.io/crates/matten). Not an ML
> framework. The API may change; pin the version.

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
  rows are train, the rest are test. No shuffle. (A seeded variant is planned;
  see RFC-024 §6.)
- **Dynamic tensors are rejected, not panicked** (with the `dynamic` feature).

## More detail

See the workspace [`ROADMAP.md`](../../ROADMAP.md) and RFC-024 (scope) / RFC-028
(design) under [`rfcs/`](../../rfcs/).

## License

Apache-2.0 © nabbisen
