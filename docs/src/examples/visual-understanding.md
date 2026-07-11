# Visual understanding examples

These examples make shapes, axes, data readiness, and preprocessing effects easier
to inspect in plain terminal output. They are examples and local tooling only:
no plotting dependency, no public visualization API, and no generated image assets.

Use this page when code runs but the shape, axis, or data meaning is still hard
to see. The examples are intended for learning and teaching small tensor
workflows, not for dashboarding or large-data visualization.

## Runnable examples

| Area | Example | Run |
|---|---|---|
| Core shape and axis flow | [`57_visual_shape_axis_summary.rs`](https://github.com/nabbisen/matten/blob/main/crates/matten/examples/57_visual_shape_axis_summary.rs) | `cargo run -p matten --example 57_visual_shape_axis_summary` |
| Dynamic readiness | [`dynamic_09_visual_readiness_summary.rs`](https://github.com/nabbisen/matten/blob/main/crates/matten/examples/dynamic_09_visual_readiness_summary.rs) | `cargo run -p matten --example dynamic_09_visual_readiness_summary --features dynamic` |
| Table-to-Tensor readiness | [`data_06_visual_readiness_summary.rs`](https://github.com/nabbisen/matten/blob/main/crates/matten-data/examples/data_06_visual_readiness_summary.rs) | `cargo run -p matten-data --example data_06_visual_readiness_summary` |
| Standardization effect | [`visual_standardize_summary.rs`](https://github.com/nabbisen/matten/blob/main/crates/matten-mlprep/examples/visual_standardize_summary.rs) | `cargo run -p matten-mlprep --example mlprep_visual_standardize_summary` |

## Local report tool

`tools/matten-report` is a workspace-excluded, `publish = false` local tool for
deterministic Markdown/plain-text reports. It is not a published crate and not a
public API.

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo shape-flow
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo dynamic-readiness
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo mlprep-standardization
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo educational-path
```

Input mode is currently accepted only for `data-readiness`:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --input tools/matten-report/fixtures/small.csv \
  --kind data-readiness \
  --select sales,cost
```

## Scope

These examples answer practical inspection questions:

```text
Which shape did this operation produce?
Which axis did the reduction collapse?
Which dynamic values are numeric, text, or missing?
Which selected table columns can become a numeric Tensor?
What did standardization change, and what shape stayed the same?
```

## Worked questions

These small questions are the fastest way to check the shape or data meaning
before reading a longer reference page.

### Broadcasting shape alignment

Broadcasting is read from the trailing axis leftward. A dimension of `1` expands
to match the other side.

```text
left shape:   [3, 1]
right shape:  [1, 4]
              ------
result shape: [3, 4]

axis 1: left has 1, right has 4, so left repeats across 4 columns
axis 0: left has 3, right has 1, so right repeats across 3 rows
```

One way to picture the values:

```text
left [3, 1]        right [1, 4]        result [3, 4]

[ 1 ]              [10 20 30 40]       [11 21 31 41]
[ 2 ]         +                         [12 22 32 42]
[ 3 ]                                   [13 23 33 43]
```

Ask for the output shape first. If every aligned pair is equal, `1`, or missing
on one side, the operation has a shape to compute.

### Reshape, flatten, and transpose

Reshape and flatten keep the same row-major tape. They only change the grouping.

```text
shape [2, 3]

[ 1  2  3 ]
[ 4  5  6 ]

flat tape: 1  2  3  4  5  6

reshape [3, 2]

[ 1  2 ]
[ 3  4 ]
[ 5  6 ]
```

Transpose changes the coordinate meaning instead:

```text
input [2, 3]        transpose [3, 2]

[ 1  2  3 ]         [ 1  4 ]
[ 4  5  6 ]    ->   [ 2  5 ]
                    [ 3  6 ]
```

Read it this way: reshape asks "where are the row breaks?", while transpose
asks "which axis does each coordinate belong to?"

### Axis reductions

For a `[rows, columns]` matrix, axis reductions answer "which axis disappears?"

```text
input shape: [3, 2]

rows axis    = axis 0
columns axis = axis 1

mean_axis(0): collapse rows, keep columns
  [3, 2] -> [2]
  result has one mean per column

mean_axis(1): collapse columns, keep rows
  [3, 2] -> [3]
  result has one mean per row
```

Read a reduction from the output shape first: the missing axis is the one the
operation summarized.

### Matmul shape flow

For matrix multiplication, the inner dimensions must match. The outer dimensions
become the output shape.

```text
left shape       right shape       result shape
[m, n]       x   [n, p]        ->  [m, p]
    ^             ^
    |             |
    shared inner dimension
```

For concrete shapes:

```text
[2, 3] x [3, 4] -> [2, 4]

left rows are kept:       2
right columns are kept:   4
shared dimension:         3
```

Each result cell is one left row dotted with one right column.

### Dynamic readiness

Dynamic tensors are for inspection and cleanup before numeric computation. A
readiness question is about which values can cross the `try_numeric()` boundary.

```text
dynamic values: [ Float(1.0), None, Text("x"), Int(4) ]

none_mask():    [    0.0,     1.0,     0.0,    0.0 ]
numeric_mask(): [    1.0,     0.0,     0.0,    1.0 ]
```

Interpret the masks like this:

```text
None        -> missing; fill or otherwise handle it first
Text("x")   -> not numeric under the strict policy
Float/Int   -> can become f64
```

After cleanup, call `try_numeric()` before arithmetic, reductions, slicing,
reshape, or matmul.

### Standardization before and after

Standardization changes scale, not shape. A column-standardization workflow
should be read like this:

```text
input tensor
  shape [rows, columns]
  columns may have different centers and scales

standardized tensor
  same shape [rows, columns]
  each selected numeric column is centered and scaled
```

For runnable output, use the existing `matten-mlprep` visual example:

```bash
cargo run -p matten-mlprep --example mlprep_visual_standardize_summary
```

That example is the source of truth for the exact reported values.

They deliberately do not add:

```text
Tensor::plot or Tensor::show
automatic expression tracing
SVG, HTML, Vega-Lite, or JSON output
notebook, GUI, or dashboard integration
published report or visualization crates
```
