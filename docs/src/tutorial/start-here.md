# Start here

This is the recommended learning path for `matten`.

## Numeric tensors

If your data is already clean numeric values, follow these examples in order:

| Step | Example | What you learn |
|---:|---|---|
| 1 | `cargo run --example 00_quickstart` | Create, add, reshape |
| 2 | `cargo run --example 01_create_tensor` | All construction APIs |
| 3 | `cargo run --example 02_shape_and_size` | Shape inspection |
| 4 | `cargo run --example 04_elementwise_ops` | Element-wise arithmetic |
| 5 | `cargo run --example 06_broadcasting` | NumPy-style broadcasting |
| 6 | `cargo run --example 08_slicing_builder` | Slice builder API |
| 7 | `cargo run --example 22_matrix_multiplication` | `dot` / `matmul` |
| 8 | `cargo run --example 27_axis_reductions` | Row/column reductions |
| 9 | `cargo run --example 57_visual_shape_axis_summary` | Shape and axis readability |
| 10 | `cargo run --example 12_boundary_error_handling` | Safe error handling |

After these ten examples you understand the numeric core.

## Dynamic ingestion: messy data with `dynamic`

If your input has missing values, mixed types, or dirty CSV/JSON:

| Step | Example | What you learn |
|---:|---|---|
| 1 | `cargo run --example dynamic_00_quickstart --features dynamic,json,csv` | Dynamic lifecycle |
| 2 | `cargo run --example dynamic_02_missing_values --features dynamic,csv` | Missing values |
| 3 | `cargo run --example dynamic_05_dirty_csv_cleanup --features dynamic,csv` | Dirty CSV |
| 4 | `cargo run --example dynamic_07_on_ramp_summary --features dynamic` | Full on-ramp |
| 5 | `cargo run --example dynamic_06_numeric_policy --features dynamic` | Conversion policy |
| 6 | `cargo run --example dynamic_09_visual_readiness_summary --features dynamic` | Readiness summary |

## The lifecycle rule

Always follow this pattern with dynamic data:

```text
messy input
  → ingest as dynamic tensor    (from_json_dynamic / from_csv_dynamic)
  → inspect                     (schema_summary, numeric_mask, count_none)
  → clean                       (fill_none, forward_fill_none)
  → convert                     (try_numeric / try_numeric_with)
  → numeric tensor computation  (&a + &b, matmul, sum_axis, …)
```

Never call arithmetic, reductions, or slicing on a dynamic tensor directly —
those APIs reject dynamic tensors with a clear message directing you to
`try_numeric()` first.

Read the two main learning paths like this:

```text
clean numeric values
        |
        v
Tensor<f64>
        |
        v
shape ops, broadcasting, matmul, reductions

messy values
        |
        v
dynamic Tensor<Element>
        |
        v
inspect -> clean -> try_numeric
        |
        v
Tensor<f64>
        |
        v
shape ops, broadcasting, matmul, reductions
```

If an operation feels confusing, first ask which shape is being kept and which
axis is being collapsed. For example, `mean_axis(0)` on a `[rows, columns]`
matrix collapses rows and leaves one value per column.

## When to graduate from `matten`

`matten` is the family car: easy to start, honest about its limits. When you
need performance, static shapes, or advanced linear algebra, see
[Migration to specialised libraries](../reference/migration.md).
