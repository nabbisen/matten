# matten-stats

[![Crates.io](https://img.shields.io/crates/v/matten-stats.svg)](https://crates.io/crates/matten-stats)
[![Docs.rs](https://docs.rs/matten-stats/badge.svg)](https://docs.rs/matten-stats)
[![license](https://img.shields.io/crates/l/matten-stats.svg)](../../LICENSE)

> **Production-ready candidate (`0.44.x` family).** A companion crate (RFC-078), promoted
> in RFC-084 once its six-function surface settled (RFC-083). The candidate label denotes
> a settled surface and a narrowed recommendation, not field-tested usage history — this
> crate has none. Pin the exact version.

Part of the [`matten` workspace](../../README.md) — see it for the full family.

## Overview

`matten-stats` provides small, explicit statistics over [`matten::Tensor`]:
`covariance`, `covariance_population`, `correlation`, `quantile`, `skewness`,
`kurtosis`, and `histogram`. These are statistics APIs RFC-040 §8 deliberately
kept out of core `matten`, accepted into their own companion once RFC-040 §9's
gate was cleared (RFC-078), then expanded (RFC-083, RFC-090). It depends only on
core `matten` (no default features) — no third-party dependency of any kind.

## The `matten-mlprep` boundary

```text
matten-mlprep  transforms tensors for ML pipelines:  Tensor -> Tensor
matten-stats   computes statistical summaries:        f64, or a small owned
               struct where the summary is inherently vector-valued (e.g.
               Histogram) -- never a Tensor
```

No function appears in both crates.

## Estimator conventions — read this before using `covariance`, `skewness`, or `kurtosis`

Core `matten`'s `var`/`std` are **population** statistics (`ddof = 0`). Each
function below matches the convention its ecosystem name is expected to
carry — the estimator **differs per function**, deliberately, because the
ecosystem's own defaults differ per function (RFC-078 §4.1, RFC-083 §4.1):

```text
covariance             sample,     ddof = 1        (NumPy/R/pandas `cov`/`corrcoef` default)
covariance_population  population, ddof = 0        (explicit in the name; no default to choose)
correlation            ddof-invariant                 (the n - 1 factors cancel; see below)
skewness               g1,  uncorrected                (SciPy `skew(bias=True)` default; NOT pandas' `.skew()`)
kurtosis               g2,  uncorrected, EXCESS         (SciPy `kurtosis(fisher=True, bias=True)` default; NOT pandas' `.kurt()`)
```

`correlation` is unaffected by the `ddof` choice — the `n - 1` factors cancel
algebraically in the ratio — so only `covariance`'s numeric output actually
differs from its population counterpart. **pandas' `.skew()`/`.kurt()`
bias-correct and return a different number** than `skewness`/`kurtosis` for
the same input; do not assume the two ecosystems agree. This is a deliberate
divergence from the rest of the family, not an oversight.

## Quick start

```rust
use matten::Tensor;
use matten_stats::{
    correlation, covariance, covariance_population, histogram, kurtosis, quantile, skewness,
};

let x = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[4]);
let y = Tensor::new(vec![2.0, 4.0, 6.0, 8.0], &[4]);

let cov = covariance(&x, &y)?;             // sample covariance, ddof = 1
let cov_pop = covariance_population(&x, &y)?; // population covariance, ddof = 0
let r = correlation(&x, &y)?;              // Pearson correlation, ddof-invariant
let median = quantile(&x, 0.5)?;           // linear interpolation
let skew = skewness(&x)?;                  // uncorrected g1
let kurt = kurtosis(&x)?;                  // uncorrected excess (Fisher) g2
let h = histogram(&x, 2)?;                 // caller-chosen bin count, no automatic rule
# Ok::<(), matten_stats::MattenStatsError>(())
```

## Design notes

- **Quantile method: linear interpolation** between the two nearest ranks of
  the sorted sample (NumPy's `"linear"` method). No alternative method
  (nearest, lower, higher, midpoint) is provided in this release.
- **Non-finite input values are rejected explicitly**, never silently
  propagated as `NaN`.
- **Zero variance is an explicit error**
  (`MattenStatsError::ZeroVariance`) in `correlation`, `skewness`,
  `kurtosis`, and `histogram`, not a silent `NaN` or (for `histogram`) an
  invented range.
- **`covariance`/`covariance_population`/`correlation` accept any
  equal-length pair** — shape beyond element count is not constrained; values
  are read in row-major order.
- **`covariance_population` accepts a single-element pair** and returns
  `0.0` — unlike `covariance`, whose `n - 1` divisor would be zero.
- **`histogram` has no automatic bin-count rule** — not Sturges, not
  Freedman-Diaconis, not Scott, no `"auto"`. `bins` is a required argument
  (RFC-090 §4.1).
- **Dynamic tensors are rejected, not panicked**, regardless of whether the
  `dynamic` feature is enabled.

## Public API

The complete surface (the breaking-change baseline for this crate):

```rust
pub fn covariance(x: &Tensor, y: &Tensor)            -> Result<f64, MattenStatsError>;
pub fn covariance_population(x: &Tensor, y: &Tensor) -> Result<f64, MattenStatsError>;
pub fn correlation(x: &Tensor, y: &Tensor)           -> Result<f64, MattenStatsError>;
pub fn quantile(x: &Tensor, q: f64)                  -> Result<f64, MattenStatsError>;
pub fn skewness(x: &Tensor)                          -> Result<f64, MattenStatsError>;
pub fn kurtosis(x: &Tensor)                          -> Result<f64, MattenStatsError>;
pub fn histogram(x: &Tensor, bins: usize)            -> Result<Histogram, MattenStatsError>;

pub struct Histogram {
    pub counts: Vec<usize>,
    pub edges: Vec<f64>,
}

#[non_exhaustive]
pub enum MattenStatsError {
    DynamicTensor,
    Empty,
    LengthMismatch { left: usize, right: usize },
    NonFiniteValue,
    ZeroVariance,
    InvalidQuantile(f64),
    InvalidBinCount,
    AllocationLimit { requested_bins: usize, limit: usize },
}
```

## Limitations

- **No automatic histogram bin-count rule, z-score, mode, matrix-wide/axis-wise
  forms, or percentile aliases.** RFC-090 resolved histogram's bin-selection
  policy as "the caller decides" rather than an automatic rule; z-score belongs
  to `matten-mlprep`'s `Tensor -> Tensor` shape instead (`standardize_columns`);
  the others were never proposed or were rejected as pure sugar (RFC-083 §6).
- **No histogram range parameter, density normalisation, or N-D histograms.**
  The range is always `[min(x), max(x)]`; counts are raw, not normalised; only
  1-D input is accepted (RFC-090 §4.2, §9).
- **No bias-corrected `skewness`/`kurtosis` variant.** Both are the
  uncorrected (SciPy-default) estimator only; a bias-corrected form is a
  purely additive follow-up if ever wanted (RFC-083 §4.1).
- **Scalar pair APIs only.** No matrix-wide covariance/correlation over many
  columns, and no axis-wise variants.
- **`Empty` on an input with fewer than 2 elements**, not on a literally
  zero-length tensor — `matten::Tensor` cannot represent zero elements at all
  (every dimension must be non-zero), so the only reachable "too few
  elements" case for `covariance`/`correlation`/`skewness`/`kurtosis` is
  exactly 1 element per side (`covariance_population` alone accepts that).
- **Not for large/streaming data.** These are eager, in-memory computations.

## Compatibility

- **SemVer:** pre-1.0 (`0.x`). A `0.x` minor bump may break and carries
  migration notes; patch releases are additive only. Pin the release
  explicitly (`matten-stats = "0.44.0"`).
- **MSRV:** Rust 1.85 (edition 2024).
- **`matten`:** released with the `0.44.x` family version (RFC-030). The
  published manifest uses the workspace's broad pre-1.0 core requirement for
  maintenance (`matten = "0"`, RFC-064); users should still declare the
  matched family explicitly.
- A `1.0` release requires explicit maintainer confirmation.

## More detail

See the workspace [`ROADMAP.md`](../../ROADMAP.md) and RFC-040 (boundary) /
RFC-078 (this crate) under [`rfcs/`](../../rfcs/).

## License

Apache-2.0 © nabbisen
