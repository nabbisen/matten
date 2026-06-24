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
| [`from_arrayd`](https://github.com/nabbisen/matten/blob/main/crates/matten-ndarray/examples/from_arrayd.rs) | `ndarray::ArrayD<f64>` → `matten::Tensor`, including a transposed (non-contiguous) input |
| [`to_arrayd`](https://github.com/nabbisen/matten/blob/main/crates/matten-ndarray/examples/to_arrayd.rs) | `matten::Tensor` → `ndarray::ArrayD<f64>` |

Both conversions **copy** data (no zero-copy claim) and preserve shape. Only numeric
tensors convert to `ndarray`.

```bash
cargo run -p matten-ndarray --example from_arrayd
cargo run -p matten-ndarray --example to_arrayd
```

## `matten-mlprep` — small preprocessing

| Example | What it shows |
|---|---|
| [`mlprep_standardize_columns`](https://github.com/nabbisen/matten/blob/main/crates/matten-mlprep/examples/standardize_columns.rs) | Per-column z-score (zero mean, unit std) |
| [`mlprep_minmax_scale`](https://github.com/nabbisen/matten/blob/main/crates/matten-mlprep/examples/minmax_scale.rs) | Per-column scaling into `[0, 1]` |
| [`mlprep_add_bias_column`](https://github.com/nabbisen/matten/blob/main/crates/matten-mlprep/examples/add_bias_column.rs) | Prepend a constant intercept column |
| [`mlprep_train_test_split`](https://github.com/nabbisen/matten/blob/main/crates/matten-mlprep/examples/train_test_split.rs) | Deterministic, ordered train/test split |

Convention throughout: rows are samples, columns are features; every transform is
deterministic with no hidden randomness and no model training.

```bash
cargo run -p matten-mlprep --example mlprep_standardize_columns
cargo run -p matten-mlprep --example mlprep_train_test_split
```

## `matten-data` — table-to-Tensor (Beta)

| Example | What it shows |
|---|---|
| [`data_00_quickstart`](https://github.com/nabbisen/matten/blob/main/crates/matten-data/examples/data_00_quickstart.rs) | The full happy path in one place |
| [`data_01_schema_summary`](https://github.com/nabbisen/matten/blob/main/crates/matten-data/examples/data_01_schema_summary.rs) | Inspect rows, columns, names, missing counts, kinds |
| [`data_02_select_columns`](https://github.com/nabbisen/matten/blob/main/crates/matten-data/examples/data_02_select_columns.rs) | Select by name; output order matches the request |
| [`data_03_missing_values`](https://github.com/nabbisen/matten/blob/main/crates/matten-data/examples/data_03_missing_values.rs) | Missing values never become zero silently |
| [`data_04_to_tensor`](https://github.com/nabbisen/matten/blob/main/crates/matten-data/examples/data_04_to_tensor.rs) | Output shape, row-major order, core interop |
| [`data_05_errors`](https://github.com/nabbisen/matten/blob/main/crates/matten-data/examples/data_05_errors.rs) | The common boundary errors |
| [`csv_to_tensor`](https://github.com/nabbisen/matten/blob/main/crates/matten-data/examples/csv_to_tensor.rs) | Comprehensive overview of the whole workflow |

`matten-data` is **Beta** and intentionally small. It is **not** a dataframe:
no group-by, join, merge, pivot, or query. Missing values and numeric conversion are
explicit, never silent. See [matten-data: table to Tensor](./data.md) for the full guide.

```bash
cargo run -p matten-data --example csv_to_tensor
```

## What this is not

Companion examples demonstrate accepted bridge/preprocessing APIs. They do not imply
that `matten` is a dataframe engine, an ML framework, a linear-algebra backend, or a
replacement for `ndarray`, `nalgebra`, NumPy, or Pandas.
