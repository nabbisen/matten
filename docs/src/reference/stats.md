# Statistics (core-lite)

Core `matten` provides exactly four statistics reductions (RFC-040), alongside the
`mean`/`mean_axis` already in [Reductions and matrix multiplication](./math.md):

- [`var`](#var--std) / [`std`](#var--std) — population variance / std over all elements.
- [`var_axis`](#var_axis--std_axis) / [`std_axis`](#var_axis--std_axis) — the same along one axis.

Anything with significant statistical policy — quantile, percentile, histogram,
covariance, correlation, z-score, sample variance — is **out of core scope** (a
possible future `matten-stats` companion). `matten` is a family-car PoC library,
not a statistics package.

## Population variance, not sample variance

All four use **population** variance (`ddof = 0`):

```text
mean = sum(x) / n
var  = sum((x - mean)^2) / n
std  = sqrt(var)
```

There is no sample-variance (`ddof = 1`) variant, no `var_with_ddof`, and no
`nanvar`/`nanstd` in core. A single-element tensor has variance `0.0`. A two-pass
algorithm is used (mean first, then squared deviations) to avoid the avoidable
cancellation of the naive one-pass `E[x^2] - E[x]^2`.

`NaN` propagates: any `NaN` element yields `NaN` (per-slice for the axis variants),
consistent with the other `f64` reductions. Use `try_numeric()` to convert a
dynamic tensor first; the statistics methods reject dynamic tensors.

## var / std

```rust
Tensor::var(&self) -> f64
Tensor::std(&self) -> f64
Tensor::try_var(&self) -> Result<f64, MattenError>
Tensor::try_std(&self) -> Result<f64, MattenError>
```

```text
[1, 2, 3, 4]  ->  mean 2.5,  var 1.25,  std sqrt(1.25) ≈ 1.118
```

The `try_*` forms return `MattenError::Unsupported` on a dynamic tensor. They also
guard the empty-tensor case with `MattenError::InvalidArgument`, but `matten`
forbids zero-sized dimensions, so an empty tensor is not constructible and that
branch is unreachable through normal construction.

## var_axis / std_axis

```rust
Tensor::var_axis(&self, axis: usize) -> Tensor
Tensor::std_axis(&self, axis: usize) -> Tensor
Tensor::try_var_axis(&self, axis: usize) -> Result<Tensor, MattenError>
Tensor::try_std_axis(&self, axis: usize) -> Result<Tensor, MattenError>
```

The reduced axis is removed from the output shape (no `keepdims`), matching the
existing axis reductions (`mean_axis`, `sum_axis`):

```text
[[1, 2, 3], [4, 5, 6]]  var_axis(0)  ->  [2.25, 2.25, 2.25]   // shape [3], per column
[[1, 2, 3], [4, 5, 6]]  var_axis(1)  ->  [2/3, 2/3]            // shape [2], per row
```

The `try_*` forms return `MattenError::Shape` if `axis >= rank`, or
`MattenError::Unsupported` on a dynamic tensor.

## Out of scope for core

```text
sample variance (ddof = 1)    quantile        percentile
histogram                     covariance      correlation
z-score                       nanvar/nanstd   statistical tests
```

These are deferred to a possible future `matten-stats` companion, which would only
be created once at least three clearly-useful, well-scoped APIs are accepted
(RFC-040 §9). Some (`z-score`) overlap with `matten-mlprep` and must not be
duplicated.

## Example

See [`16_variance_std.rs`](https://github.com/nabbisen/matten/blob/main/crates/matten/examples/16_variance_std.rs)
for a runnable walkthrough.
