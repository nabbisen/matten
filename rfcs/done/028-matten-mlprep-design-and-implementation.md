# RFC-028: `matten-mlprep` Design and Implementation

**Status:** Implemented (v0.18.0; matten-mlprep 0.1.0)
**Target:** v0.18.0 (experimental)
**Theme:** Second companion crate — transparent numeric preprocessing
**Depends on:** RFC-019 (axis reductions), RFC-022 (boundary policy), RFC-024 (mlprep scope)

---

## 1. Summary

This RFC is the detailed (internal/program) design for `matten-mlprep`, the
second companion crate. It implements RFC-024's accepted scope: four small,
transparent, deterministic preprocessing helpers over 2D numeric tensors.

The crate's defining property is **no magic**: every transformation is a plain,
deterministic function the user can reason about. There is no model training, no
autograd, no optimizer, no hidden randomness, and no ML-framework dependency
(RFC-024 §3). It depends only on core `matten` (no default features) — no
`ndarray`, no `candle`, no `rand`.

RFC-024 is the policy; this RFC is the buildable specification and is the
design-before-coding artifact for v0.18.0.

---

## 2. Dependency shape

```toml
[dependencies]
matten = { path = "../matten", version = "0.16", default-features = false }

[features]
# Forwarded so dynamic-tensor rejection compiles only where relevant.
dynamic = ["matten/dynamic"]
```

`default-features = false` keeps the crate lean: it needs only the numeric core
(`Tensor`, `shape`, `as_slice`, `new`). No new third-party dependency is added.
The core dependency-boundary check (RFC-022 §10) is unaffected: core `matten`
still depends on nothing here.

---

## 3. Shape convention (RFC-024 §4, enforced)

All APIs operate on **rank-2** tensors only, with:

```text
rows    = samples
columns = features
```

A non-2D tensor is rejected with `ExpectedMatrix`. There is no silent
transposition or rank promotion. A single shared guard (`matrix_dims`) enforces
this and the dynamic-rejection rule for every entry point.

---

## 4. Public API

```rust
use matten_mlprep::{
    add_bias_column, minmax_scale_columns, standardize_columns, train_test_split,
    MattenMlprepError,
};
```

### 4.1 `standardize_columns(x) -> Result<Tensor, _>`

Per-column z-score: `out[i,j] = (x[i,j] - mean_j) / std_j`.

- `std_j` is the **population** standard deviation (divide by `n`), matching
  scikit-learn's `StandardScaler`.
- **Zero-variance policy (explicit):** if any column is constant (`std_j == 0`),
  return `Err(ZeroVariance { column: j })` for the first such column. The crate
  does *not* silently emit a zero column; the caller must drop or handle constant
  features explicitly. This is the transparent behavior RFC-024 §8 requires.
- `NaN`/`Inf` in a column propagate to that column's output (the data is assumed
  already clean; use the core dynamic on-ramp first if it is not).

### 4.2 `minmax_scale_columns(x) -> Result<Tensor, _>`

Per-column min-max scaling to `[0, 1]`:
`out[i,j] = (x[i,j] - min_j) / (max_j - min_j)`.

- **Zero-range policy:** a constant column (`max_j == min_j`) has no range to
  scale and returns `Err(ZeroVariance { column: j })` (same condition, same
  variant). Explicit, not silent.
- `NaN` propagates (a column containing `NaN` yields `NaN` min/max and output).

### 4.3 `add_bias_column(x) -> Result<Tensor, _>`

Prepends a constant `1.0` column: `[n, m] -> [n, m+1]` with column `0` all ones
and the original features shifted to columns `1..=m`. Prepending (intercept at
index 0) matches the common `w · [1, x]` convention. Documented position; no
ambiguity.

### 4.4 `train_test_split(x, train_ratio) -> Result<(Tensor, Tensor), _>`

Ordered, deterministic, no shuffle (RFC-024 §5):

```text
n_train = floor(n_rows * train_ratio)
train   = rows[0 .. n_train]
test    = rows[n_train .. n_rows]
```

- `train_ratio` must be finite and `0.0 < ratio < 1.0`, else `InvalidRatio(ratio)`.
- For any `ratio < 1.0`, `n_train <= n_rows - 1` always holds, so `test` is never
  empty. The only failure is `n_train == 0` (when `n_rows * ratio < 1`), reported
  as `EmptySplit { rows, train_ratio }`.
- Both outputs are rank-2: `[n_train, m]` and `[n_test, m]`.

A future shuffled variant is `train_test_split_seeded(x, ratio, seed)` using a
tiny in-crate PRNG (SplitMix64); it is **out of scope for 0.1.0** (RFC-024 §6)
and deferred until exact-reproducibility tests are written.

---

## 5. Error type (refines RFC-024 §7)

```rust
#[derive(Debug)]
#[non_exhaustive]
pub enum MattenMlprepError {
    /// A dynamic tensor was passed; convert with `Tensor::try_numeric()` first.
    DynamicTensor,
    /// Input was not rank-2 (mlprep operates on [rows=samples, cols=features]).
    ExpectedMatrix { shape: Vec<usize> },
    /// `train_ratio` was not a finite value in the open interval (0, 1).
    InvalidRatio(f64),
    /// A split would leave the train set empty (`floor(rows * ratio) == 0`).
    EmptySplit { rows: usize, train_ratio: f64 },
    /// A column has no variance / range and cannot be scaled.
    ZeroVariance { column: usize },
    /// Core `matten` rejected a constructed result.
    Matten(matten::MattenError),
}
```

`EmptySplit` refines RFC-024's example `EmptyInput`: core `matten` forbids
zero-sized tensors, so a literally empty input cannot be constructed; the real
failure mode is a split that empties one side. Implements `Display` and
`std::error::Error` (with `source()` for `Matten`). `#[non_exhaustive]`.

Companion error policy (RFC-022 §8): its own error type; core `MattenError`
gains no mlprep variants. Dynamic input returns `Err`, never a panic.

---

## 6. Module layout (logical boundaries, all well under 300 ELOC)

```text
crates/matten-mlprep/src/
  lib.rs      crate docs + re-exports
  error.rs    MattenMlprepError
  util.rs     matrix_dims() shared guard (rank-2 + dynamic rejection)
  scale.rs    standardize_columns, minmax_scale_columns (+ column-stat helpers)
  bias.rs     add_bias_column
  split.rs    train_test_split
```

## 7. Determinism

Every function is a pure deterministic transform of its input. No RNG, no
global state, no time, no environment reads. `train_test_split` is an ordered
slice. Examples and tests are reproducible bit-for-bit.

## 8. Test plan (validates the design specifications)

| Test | Validates |
|---|---|
| `standardize_columns` known values | per-column z-score, population std |
| standardized column has mean≈0, std≈1 | correctness property |
| `standardize_columns` constant column | `ZeroVariance` error (not zeros) |
| `minmax_scale_columns` to [0,1] | per-column scaling, endpoints 0 and 1 |
| `minmax_scale_columns` constant column | `ZeroVariance` error |
| `add_bias_column` shape + column 0 | `[n,m]->[n,m+1]`, ones prepended |
| `train_test_split` ordered partition | row ranges, shapes, determinism |
| `train_test_split` invalid ratio (`<=0`, `>=1`, `NaN`) | `InvalidRatio` |
| `train_test_split` empties train | `EmptySplit` |
| any function on non-2D input | `ExpectedMatrix` |
| any function on dynamic input (feature on) | `DynamicTensor`, no panic |

Examples (RFC-024 §9): `standardize_columns.rs`, `minmax_scale.rs`,
`add_bias_column.rs`, `train_test_split.rs` — in this crate, not core.

## 9. Acceptance criteria (RFC-024 §8, ROADMAP §6)

- row=sample / column=feature convention enforced (shared guard);
- split-ratio validation tested;
- zero-variance behavior explicit and documented (errors, not silent zeros);
- transformations deterministic; examples deterministic;
- no training/autograd/framework scope;
- core `matten` has no dependency on `matten-mlprep` (boundary CI passes).

## 10. Non-goals / deferred

- `train_test_split_seeded` (SplitMix64) — deferred (RFC-024 §6).
- Robust/quantile scalers, one-hot encoding, imputation — not in 0.1.0.
- Any `fit`/`transform` stateful scaler objects — out of scope; these are pure
  functions, by design.
