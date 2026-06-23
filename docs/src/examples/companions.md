# Companion crate examples

Each companion crate ships its own runnable examples, living in that crate's
`examples/` directory (never in core `matten`). They are small, deterministic, and
self-checking, and they all respect the one-way dependency rule: companions depend
on `matten`, but core `matten` depends on no companion.

These examples were audited and improved in place under RFC-048; the program does
not add duplicate or renamed companion examples.

## `matten-ndarray` — interop with `ndarray`

| Example | What it shows |
|---|---|
| `from_arrayd` | `ndarray::ArrayD<f64>` → `matten::Tensor`, including a transposed (non-contiguous) input |
| `to_arrayd` | `matten::Tensor` → `ndarray::ArrayD<f64>` |

Both conversions **copy** data (no zero-copy claim) and preserve shape. Only numeric
tensors convert to `ndarray`.

```bash
cargo run -p matten-ndarray --example from_arrayd
cargo run -p matten-ndarray --example to_arrayd
```

## `matten-mlprep` — small preprocessing

| Example | What it shows |
|---|---|
| `mlprep_standardize_columns` | Per-column z-score (zero mean, unit std) |
| `mlprep_minmax_scale` | Per-column scaling into `[0, 1]` |
| `mlprep_add_bias_column` | Prepend a constant intercept column |
| `mlprep_train_test_split` | Deterministic, ordered train/test split |

Convention throughout: rows are samples, columns are features; every transform is
deterministic with no hidden randomness and no model training.

```bash
cargo run -p matten-mlprep --example mlprep_standardize_columns
cargo run -p matten-mlprep --example mlprep_train_test_split
```

## `matten-data` — table-to-Tensor (Experimental)

| Example | What it shows |
|---|---|
| `csv_to_tensor` | CSV string → inspect → select columns → fill missing → numeric `Tensor` |

`matten-data` is **Experimental** and intentionally small. It is **not** a dataframe:
no group-by, join, merge, pivot, or query. Missing values and numeric conversion are
explicit, never silent.

```bash
cargo run -p matten-data --example csv_to_tensor
```

## What this is not

Companion examples demonstrate accepted bridge/preprocessing APIs. They do not imply
that `matten` is a dataframe engine, an ML framework, a linear-algebra backend, or a
replacement for `ndarray`, `nalgebra`, NumPy, or Pandas.
