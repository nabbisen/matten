# matten-stats

[![Crates.io](https://img.shields.io/crates/v/matten-stats.svg)](https://crates.io/crates/matten-stats)
[![Docs.rs](https://docs.rs/matten-stats/badge.svg)](https://docs.rs/matten-stats)
[![license](https://img.shields.io/crates/l/matten-stats.svg)](../../LICENSE)

> **Experimental (`0.38.x` family).** A new companion crate with no usage history (RFC-078).
> The three-function surface is small and deliberately scoped, but has not yet
> earned a higher maturity label. Pin the exact version.

Part of the [`matten` workspace](../../README.md) — see it for the full family.

## Overview

`matten-stats` provides small, explicit scalar statistics over [`matten::Tensor`]:
`covariance`, `correlation`, and `quantile`. These are the three APIs RFC-040 §8
deliberately kept out of core `matten`, and RFC-078 accepts them into their own
companion once RFC-040 §9's gate is cleared. It depends only on core `matten`
(no default features) — no third-party dependency of any kind.

## The `matten-mlprep` boundary

```text
matten-mlprep  transforms tensors for ML pipelines: Tensor -> Tensor
matten-stats   computes scalar statistical summaries: Tensor -> f64
```

No function appears in both crates.

## The `ddof = 1` divergence from core — read this before using `covariance`

Core `matten`'s `var`/`std` are **population** statistics (`ddof = 0`).
**`covariance` and `correlation` in this crate use the sample estimator
(`ddof = 1`, i.e. divide by `n - 1`)**, matching the near-universal default in
inferential statistics (NumPy's `cov`/`corrcoef`, R, pandas). This is a
deliberate divergence from the rest of the family, not an oversight.
`correlation` is unaffected by the choice — the `n - 1` factors cancel in the
ratio — so only `covariance`'s numeric output actually differs.

## Quick start

```rust
use matten::Tensor;
use matten_stats::{correlation, covariance, quantile};

let x = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[4]);
let y = Tensor::new(vec![2.0, 4.0, 6.0, 8.0], &[4]);

let cov = covariance(&x, &y)?;   // sample covariance, ddof = 1
let r = correlation(&x, &y)?;    // Pearson correlation, ddof-invariant
let median = quantile(&x, 0.5)?; // linear interpolation
# Ok::<(), matten_stats::MattenStatsError>(())
```

## Design notes

- **Quantile method: linear interpolation** between the two nearest ranks of
  the sorted sample (NumPy's `"linear"` method). No alternative method
  (nearest, lower, higher, midpoint) is provided in this release.
- **Non-finite input values are rejected explicitly**, never silently
  propagated as `NaN`.
- **Zero variance in `correlation` is an explicit error**
  (`MattenStatsError::ZeroVariance`), not a silent `NaN`.
- **`covariance`/`correlation` accept any equal-length pair** — shape beyond
  element count is not constrained; values are read in row-major order.
- **Dynamic tensors are rejected, not panicked**, regardless of whether the
  `dynamic` feature is enabled.

## Public API

The complete surface (the breaking-change baseline for this crate):

```rust
pub fn covariance(x: &Tensor, y: &Tensor)  -> Result<f64, MattenStatsError>;
pub fn correlation(x: &Tensor, y: &Tensor) -> Result<f64, MattenStatsError>;
pub fn quantile(x: &Tensor, q: f64)        -> Result<f64, MattenStatsError>;

#[non_exhaustive]
pub enum MattenStatsError {
    DynamicTensor,
    Empty,
    LengthMismatch { left: usize, right: usize },
    NonFiniteValue,
    ZeroVariance,
    InvalidQuantile(f64),
}
```

## Limitations

- **No histogram, z-score, skew, kurtosis, or mode.** RFC-040 §8 left
  histogram's bin-selection policy unresolved; the others were never proposed.
- **Scalar pair APIs only.** No matrix-wide covariance/correlation over many
  columns, and no axis-wise variants.
- **`Empty` on an input with fewer than 2 elements**, not on a literally
  zero-length tensor — `matten::Tensor` cannot represent zero elements at all
  (every dimension must be non-zero), so the only reachable "too few
  elements" case for `covariance`/`correlation` is exactly 1 element per side.
- **Not for large/streaming data.** These are eager, in-memory computations.

## Compatibility

- **SemVer:** pre-1.0 (`0.x`). A `0.x` minor bump may break and carries
  migration notes; patch releases are additive only. Pin the release
  explicitly (`matten-stats = "0.38.0"`).
- **MSRV:** Rust 1.85 (edition 2024).
- **`matten`:** released with the `0.38.x` family version (RFC-030). The
  published manifest uses the workspace's broad pre-1.0 core requirement for
  maintenance (`matten = "0"`, RFC-064); users should still declare the
  matched family explicitly.
- A `1.0` release requires explicit maintainer confirmation.

## More detail

See the workspace [`ROADMAP.md`](../../ROADMAP.md) and RFC-040 (boundary) /
RFC-078 (this crate) under [`rfcs/`](../../rfcs/).

## License

Apache-2.0 © nabbisen
