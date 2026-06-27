# Worked example: linear regression (gradient descent) readiness

This applies the [readiness report template](../readiness-report.md) to the
`35_linear_regression_gradient_descent` example. It is illustrative — the example itself runs
on toy data; the report imagines the same code scaled to real data and asks what would change.

> **This report is advisory. It does not prove production readiness, does not guarantee a
> target library is better, and does not perform automatic conversion.**

---

# matten Migration Readiness Report

## Summary

Batch gradient descent for a linear model `ŷ = X · θ`. At the example's toy size, **stay with
`matten`**. If the same code runs on a real design matrix (thousands of samples, many
features), **move the per-step matrix products to `ndarray`** via the `matten-ndarray` bridge,
keeping `matten` for setup. A closed-form solve in `nalgebra` is an optional redesign.

## Current matten usage

- `X` is a `[samples, 2]` `Tensor` (leading bias column, so `θ = [b, w]`).
- Each step runs **two `Tensor::matmul` calls**: predictions `X · θ` (`[n,2] × [2] → [n]`) and
  the gradient `Xᵀ · residual` (`[2,n] × [n] → [2]`).
- `Xᵀ` is formed once with `Tensor::transpose` and reused.
- The residual and the `θ` update are plain Rust (`zip`/`map`).
- The loop runs many iterations (2000 in the example).

## Production pressure signals

- **Runtime pressure (signal 2): present at scale.** The two matmuls per step, over many
  iterations, are the hot path once `X` is large. This is the kernel worth moving.
- **Data-size pressure (signal 1): present at scale.** A large design matrix stresses
  `matten`'s copy-on-reshape/slice behavior.
- **Linear-algebra pressure (signal 4): partial.** The problem *can* be solved without
  iteration, via the normal equations — which needs a solver/decomposition `matten` lacks.
- **Dependency policy (signal 8): low cost.** `matten-ndarray` is already an available bridge,
  so the ndarray path adds little.
- **Ecosystem/team (signals 9–10): Rust.**
- Axis-reduction, dataframe, ML/device, and dynamic-ingestion signals are **not** present here.

## Recommended target(s)

- **`ndarray` (primary).** Keep the gradient-descent structure as-is and run the two matrix
  products as `ndarray` `.dot()` (BLAS-backed for large `X`). This is a near-direct port.
- **`nalgebra` (optional redesign).** If you would rather not iterate, reformulate as a
  closed-form normal-equation solve using a decomposition. That is a change of *algorithm*,
  chosen for capability, not a mechanical port.
- **Toy size: stay with `matten`.** The signals only bite at real data sizes.

## Direct conversion candidates

- `X`, `Xᵀ`, and `θ` → `ArrayD<f64>` with `matten_ndarray::to_arrayd`, **converted once before
  the loop**.
- The two `matmul` calls → `ndarray` `.dot()`.
- Final `θ` back to a `Tensor` with `from_arrayd` if downstream code expects one.

## Manual redesign areas

- The plain-Rust residual and `θ` update become `ndarray` elementwise operations — small, but
  not a literal copy-paste.
- The optional closed-form solve is a genuine redesign (assemble `XᵀX` and `Xᵀy`, solve), not a
  translation of the existing loop.

## Bridge crates / tools

- **`matten-ndarray`** (`to_arrayd` / `from_arrayd`): copies both directions, `f64` on both
  sides, so **no precision change**. See the
  [bridge contract](../bridge-contracts.md).
- The `nalgebra` option has **no bridge crate**; conversion is manual via `DMatrix::from_row_slice`
  (mind the row- vs column-major boundary).

## Risks

- **Converting inside the loop.** Convert `X`/`Xᵀ`/`θ` once, before iterating — not per step.
- **Column-major trap (nalgebra option only).** Build `DMatrix` with `from_row_slice` so the
  row-major data is not silently transposed.
- **Over-migration.** Keep `matten` for constructing `X` and `y`; only the kernel needs to move.

## Next steps

1. Profile at a realistic data size to confirm the matmuls are the bottleneck.
2. Convert `X`, `Xᵀ`, `θ` once via `matten-ndarray`; run the loop with `ndarray` `.dot()`.
3. Keep `matten` for data construction and glue.
4. Reassess later: if you want a non-iterative solve, move to `nalgebra`; if the model grows
   into trained ML with autodiff/GPU, that is a [Candle](../playbooks/candle.md) question.
