# Examples index

All `matten` examples live in `examples/`. They are grouped by purpose.

## Core examples (Phase 1 numeric)

These examples demonstrate the default `matten` API. No extra features required.

| File | What it shows |
|---|---|
| `00_quickstart.rs` | First look: create, add, reshape |
| `01_create_tensor.rs` | All construction APIs |
| `02_shape_and_size.rs` | Shape inspection |
| `03_reshape_flatten.rs` | Reshape and flatten |
| `04_elementwise_ops.rs` | Element-wise arithmetic |
| `05_scalar_ops.rs` | Scalar multiplication and division |
| `06_broadcasting.rs` | NumPy-style broadcasting |
| `07_transpose_swap_axes.rs` | Axis permutation |
| `08_slicing_builder.rs` | Slice builder API (canonical) |
| `09_slice_str.rs` | String slice API (convenience) |
| `10_json_roundtrip.rs` | JSON serialization round-trip |
| `11_csv_numeric_loading.rs` | Numeric CSV loading |
| `12_boundary_error_handling.rs` | Handling errors at data boundaries |
| `13_resource_limits.rs` | `MattenLimits`, `try_zeros`/`try_ones`/`try_full` |

## Math examples

| File | What it shows |
|---|---|
| `20_dot_product.rs` | Vector dot product |
| `21_matrix_vector_product.rs` | Matrix × vector |
| `22_matrix_multiplication.rs` | Matrix × matrix |
| `23_sum_mean.rs` | Whole-tensor and axis reductions |
| `24_min_max.rs` | Min and max with NaN policy |
| `25_normalize_vector.rs` | L2 normalisation |
| `26_cosine_similarity.rs` | Cosine similarity |
| `27_axis_reductions.rs` | Axis reductions and NaN propagation |
| `28_column_statistics.rs` | Per-column statistics workflow |

## Applied problems (famous small math)

Recognizable small math / numerical-computing problems, used to show what a
`Tensor` can represent. These live in a fresh `30+` band so the core suite above
stays stable. Write-ups: [Beginner applied math](./beginner-math.md),
[Matrix iteration](./matrix-iteration.md),
[Numerical methods](./numerical-methods.md), and [ML-like](./ml-like.md).

| File | What it shows |
|---|---|
| `30_magic_square_checker.rs` | Row/column/diagonal sums via `get` |
| `31_fibonacci_matrix_power.rs` | Fibonacci via repeated `matmul` |
| `32_graph_path_counting.rs` | Walk counting via adjacency-matrix powers |
| `33_markov_chain_weather.rs` | Distribution over time via vector × matrix `matmul` |
| `34_tiny_pagerank.rs` | PageRank power iteration via matrix × vector `matmul` |
| `35_linear_regression_gradient_descent.rs` | Batch gradient descent via `matmul` + `transpose` |
| `36_heat_equation_1d.rs` | Explicit finite-difference stencil as `matmul` iteration |
| `37_kmeans_small.rs` | Lloyd's k-means on a `[points, features]` data matrix |
| `38_nearest_neighbor_classification.rs` | 1-NN classification over a labeled data matrix |
| `39_finite_difference_derivative.rs` | Central-difference derivative on a `linspace` grid |
| `40_trapezoidal_integration.rs` | Trapezoidal rule via `linspace` + elementwise + `sum` |

Vector distance and cosine similarity are already covered above — see
`pairwise_distance.rs`, `25_normalize_vector.rs`, and `26_cosine_similarity.rs`
rather than a duplicate in this band.

## Pattern examples

Small practical PoC patterns using accepted Phase 1 APIs.

| File | What it shows |
|---|---|
| `standardize_columns.rs` | Column standardisation (z-score) |
| `minmax_scaling.rs` | Min-max feature scaling |
| `rowwise_scoring.rs` | Row-wise scoring |
| `column_summary.rs` | Column summary statistics |
| `moving_average.rs` | Simple moving average |
| `rolling_windows_basic.rs` | Rolling window extraction |
| `pairwise_distance.rs` | Pairwise Euclidean distances |
| `gram_matrix.rs` | Gram matrix (X × Xᵀ) |

## Dynamic examples (`--features dynamic`)

These require the `dynamic` feature for heterogeneous data ingestion.

| File | Features | What it shows |
|---|---|---|
| `dynamic_00_quickstart.rs` | `dynamic,json,csv` | Dynamic lifecycle overview |
| `dynamic_01_mixed_elements.rs` | `dynamic` | Mixed `Element` types |
| `dynamic_02_missing_values.rs` | `dynamic,csv` | Missing value detection |
| `dynamic_03_fill_none.rs` | `dynamic` | Filling missing values |
| `dynamic_04_numeric_coercion.rs` | `dynamic` | Element-level coercion |
| `dynamic_05_dirty_csv_cleanup.rs` | `dynamic,csv` | Real-world CSV cleanup |
| `dynamic_06_numeric_policy.rs` | `dynamic` | `NumericPolicy` API |
| `dynamic_07_on_ramp_summary.rs` | `dynamic` | Complete on-ramp workflow |

## Companion crate examples

These live in each companion crate's own `examples/` directory, not in core
`matten`. See [Companion crate examples](./companions.md) for the write-up.

| Crate | Example | What it shows |
|---|---|---|
| `matten-ndarray` | `from_arrayd`, `to_arrayd` | `ArrayD` ↔ `Tensor` interop (copies, shape-preserving) |
| `matten-mlprep` | `mlprep_standardize_columns`, `mlprep_minmax_scale`, `mlprep_add_bias_column`, `mlprep_train_test_split` | Small deterministic preprocessing |
| `matten-data` | `csv_to_tensor` | CSV → clean → numeric `Tensor` (Experimental) |

## Running examples

```bash
# Phase 1 (no features needed):
cargo run --example 00_quickstart
cargo run --example 27_axis_reductions

# Dynamic:
cargo run --example dynamic_06_numeric_policy --features dynamic
cargo run --example dynamic_07_on_ramp_summary --features dynamic,csv
```

## Scope rule

Every example demonstrates **accepted APIs only**. Examples are not a back
door for adding new mathematical operations, dataframe behavior, or ML scope.
