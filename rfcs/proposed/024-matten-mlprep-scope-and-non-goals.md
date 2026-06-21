# RFC-024: `matten-mlprep` Scope and Non-goals

**Status:** Proposed  
**Target:** v0.18 experimental; v0.19 beta decision / hardening  
**Theme:** Transparent preprocessing companion crate  
**Depends on:** RFC-019, RFC-022

---

## 1. Summary

This RFC defines the scope of a future `matten-mlprep` companion crate.

`matten-mlprep` provides small, transparent preprocessing helpers for numeric `Tensor` workflows. It must not become an ML framework. It should help users prepare data for external tools, not train models.

---

## 2. Goals

- Provide deterministic preprocessing helpers.
- Keep ML-specific concepts out of core `matten`.
- Reuse `Tensor` as the data container.
- Document shape rules clearly.
- Avoid hidden randomness.

---

## 3. Non-goals

- No model training.
- No autograd.
- No neural networks.
- No optimizers.
- No Candle dependency.
- No automatic ML pipelines.
- No hidden random split behavior.

---

## 4. Shape convention

All initial `matten-mlprep` APIs operate on 2D numeric tensors.

Convention:

```text
rows = samples
columns = features
```

This must be enforced. Silent transposition or ambiguous interpretation is not allowed.

---

## 5. Initial API scope

Allowed experimental APIs:

```rust
pub fn standardize_columns(x: &Tensor) -> Result<Tensor, MattenMlprepError>;
pub fn minmax_scale_columns(x: &Tensor) -> Result<Tensor, MattenMlprepError>;
pub fn add_bias_column(x: &Tensor) -> Result<Tensor, MattenMlprepError>;
pub fn train_test_split(x: &Tensor, train_ratio: f64) -> Result<(Tensor, Tensor), MattenMlprepError>;
```

Default `train_test_split` is ordered and deterministic:

```text
first floor(n_rows * train_ratio) rows -> train
remaining rows -> test
```

No shuffling happens in the default function.

---

## 6. Seeded split policy

If shuffled splitting is added later, it must be a separate explicit API:

```rust
pub fn train_test_split_seeded(
    x: &Tensor,
    train_ratio: f64,
    seed: u64,
) -> Result<(Tensor, Tensor), MattenMlprepError>;
```

The RNG must be dependency-light and documented. A tiny deterministic PRNG such as SplitMix64 is acceptable if exact reproducibility is tested and documented. Pulling `rand` is not allowed without a new dependency review.

---

## 7. Error policy

`matten-mlprep` defines its own error type.

Example:

```rust
pub enum MattenMlprepError {
    DynamicTensor,
    ExpectedMatrix { shape: Vec<usize> },
    InvalidRatio(f64),
    EmptyInput,
    ZeroVariance { column: usize },
    Matten(matten::MattenError),
}
```

---

## 8. Beta gate

`matten-mlprep` may move toward beta only if:

- APIs remain small;
- all transformations are deterministic;
- shape rules are documented;
- zero-variance behavior is explicit;
- train/test split behavior is explicit;
- examples are realistic;
- no training/autograd/framework scope enters the crate;
- core `matten` has no dependency on `matten-mlprep`.

---

## 9. Examples

Examples belong in `matten-mlprep`:

```text
examples/standardize_columns.rs
examples/minmax_scale.rs
examples/add_bias_column.rs
examples/train_test_split.rs
```

Core `matten` should not grow ML-prep APIs or examples beyond small user-side patterns.
