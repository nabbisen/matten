# Practical numeric recipes

A set of small, self-contained numeric recipes that combine core `matten` primitives
into common data-processing patterns. Each file is a single runnable example with
hard-coded data, assertions, and stable output.

These live in the `50_`–`56_` band, separate from the core tutorial (`01_`–`13_`),
the numeric building blocks (`20_`–`28_`), and the famous-problem examples (`30_`–`40_`).

## Examples

### `50_rowwise_scoring.rs`

Row-wise weighted scoring: multiply each row of a feature matrix by a weight vector,
then sum across columns to produce one score per row. Shows broadcasting between a
`[rows, cols]` tensor and a `[cols]` weight vector, followed by `sum_axis`.

```bash
cargo run --example 50_rowwise_scoring
```

Source: [`50_rowwise_scoring.rs`](https://github.com/nabbisen/matten/blob/main/crates/matten/examples/50_rowwise_scoring.rs)

### `51_standardize_columns.rs`

Z-score normalisation of each column (zero mean, unit variance) using only
`mean_axis`, broadcasting, and element-wise arithmetic — no external crate needed.

```bash
cargo run --example 51_standardize_columns
```

Source: [`51_standardize_columns.rs`](https://github.com/nabbisen/matten/blob/main/crates/matten/examples/51_standardize_columns.rs)

### `52_minmax_scaling.rs`

Min-max (0–1) scaling of each column using `min_axis`, `max_axis`, and broadcasting.
A common feature-normalisation step before ML algorithms.

```bash
cargo run --example 52_minmax_scaling
```

Source: [`52_minmax_scaling.rs`](https://github.com/nabbisen/matten/blob/main/crates/matten/examples/52_minmax_scaling.rs)

### `53_gram_matrix.rs`

Gram matrix: `G = X · Xᵀ`, computed with `matmul`. Used in kernel methods and
feature covariance. Shows that a single `matmul` call produces a symmetric
`[n, n]` similarity matrix from an `[n, d]` data matrix.

```bash
cargo run --example 53_gram_matrix
```

Source: [`53_gram_matrix.rs`](https://github.com/nabbisen/matten/blob/main/crates/matten/examples/53_gram_matrix.rs)

### `54_pairwise_distance.rs`

Pairwise Euclidean distances between rows using the identity
`‖a−b‖² = ‖a‖² + ‖b‖² − 2aᵀb`, computed with broadcasting and `matmul`.
Demonstrates efficient distance computation without an explicit loop over pairs.

```bash
cargo run --example 54_pairwise_distance
```

Source: [`54_pairwise_distance.rs`](https://github.com/nabbisen/matten/blob/main/crates/matten/examples/54_pairwise_distance.rs)

### `55_moving_average.rs`

Simple moving average over a 1-D series using slice windows (`slice_str`).
Shows a sliding-window pattern with overlapping slices and `mean` reduction.

```bash
cargo run --example 55_moving_average
```

Source: [`55_moving_average.rs`](https://github.com/nabbisen/matten/blob/main/crates/matten/examples/55_moving_average.rs)

### `56_rolling_windows_basic.rs`

Rolling window sum and max over overlapping slices of a 1-D series.
Extends the moving-average idea to multiple aggregations in one pass.

```bash
cargo run --example 56_rolling_windows_basic
```

Source: [`56_rolling_windows_basic.rs`](https://github.com/nabbisen/matten/blob/main/crates/matten/examples/56_rolling_windows_basic.rs)

## What this is not

These recipes show how to compose accepted `matten` APIs into common patterns. They
do not imply that `matten` is a feature-engineering framework, a signal-processing
library, or a statistics package. For preprocessing helpers with a proper API, see
`matten-mlprep`.
