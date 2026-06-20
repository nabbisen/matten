# RFC-024: `matten-mlprep` Scope and Non-goals

**Status:** Proposed  
**Target:** v0.16+ design, v0.18+ possible PoC  
**Theme:** Companion crate exploration  
**Depends on:** RFC-019, RFC-022  
**Related handoff:** `024-matten-mlprep-scope-and-non-goals-handoff.md`

## 1. Summary

This RFC defines the tentative scope of a future `matten-mlprep` companion crate.

`matten-mlprep` would provide small, transparent preprocessing helpers for proof-of-concept machine learning workflows. It must not become an ML framework. It should help users prepare numeric tensors, not train models.

## 2. Goals

- Provide small preprocessing helpers.
- Keep ML-specific concepts out of `matten` core.
- Reuse `Tensor` as the data container.
- Keep functions transparent and easy to inspect.
- Support small PoC workflows.

## 3. Non-goals

- No model training.
- No autograd.
- No neural network layers.
- No optimizer implementations.
- No GPU.
- No full scikit-learn clone.
- No hidden statistical magic.

## 4. External design

Illustrative future API:

```rust
use matten_mlprep::{standardize_columns, train_test_split};

let x = Tensor::load_csv("features.csv")?;
let x = standardize_columns(&x)?;
let (train, test) = train_test_split(&x, 0.8)?;
```

## 5. Data model

No new core data model.

`matten-mlprep` consumes numeric `Tensor` and returns numeric `Tensor`.

Possible small metadata structs:

```rust
pub struct Split<T> {
    pub train: T,
    pub test: T,
}
```

Avoid storing training state unless a transformer API is explicitly designed.

## 6. Data lifecycle

```text
numeric Tensor
  -> preprocessing helper
  -> numeric Tensor
  -> user model / external ML crate
```

If input is dynamic, user must call `try_numeric()` before using `matten-mlprep`.

## 7. Events and observable behavior

Preprocessing failures return `Result` when parameters are invalid.

Examples:

- split ratio outside `(0, 1)`;
- wrong tensor rank;
- zero standard deviation policy conflict.

## 8. Store access

Use public numeric accessors:

- `shape`;
- `as_slice`;
- construction APIs.

Do not access internal storage.

## 9. Public API candidates

Potential functions:

```rust
pub fn minmax_scale_columns(x: &Tensor) -> Result<Tensor, MattenError>;
pub fn standardize_columns(x: &Tensor) -> Result<Tensor, MattenError>;
pub fn add_bias_column(x: &Tensor) -> Result<Tensor, MattenError>;
pub fn train_test_split(x: &Tensor, ratio: f64) -> Result<(Tensor, Tensor), MattenError>;
```

All are tentative.

## 10. Cargo feature impact

Separate crate.

No new `matten` core feature.

## 11. Internal design

### 11.1 Simplicity

Implement with straightforward loops and `matten` APIs.

### 11.2 Axis reductions

`matten-mlprep` should use RFC-019 axis reductions once available.

### 11.3 Randomness

Avoid random split initially, or make RNG explicit.

Deterministic split by row index is simpler:

```text
first N rows -> train
remaining rows -> test
```

## 12. Examples

Examples belong in `matten-mlprep` after crate creation.

Potential:

```text
examples/standardize_columns.rs
examples/minmax_scale.rs
examples/train_test_split.rs
examples/add_bias_column.rs
```

Core `matten` may keep small pattern examples but should not grow full ML-prep utilities.

## 13. Acceptance criteria for future PoC

- No training API.
- No external ML framework dependency.
- Functions are transparent and documented.
- Uses public `matten` API only.
- Works for small 2D numeric tensors.
- Rejects dynamic tensors clearly.

## 14. QA checklist

- [ ] Parameter validation tests
- [ ] Shape validation tests
- [ ] zero variance tests
- [ ] deterministic split tests
- [ ] examples compile
- [ ] no core dependency pollution

## 15. Open questions

1. Should `matten-mlprep` exist before bridge crates?
2. Should preprocessing helpers return `MattenError` or crate-specific errors?
3. Should transformer objects be introduced later for fit/transform workflows?
