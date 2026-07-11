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

## Worked shape question

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

They deliberately do not add:

```text
Tensor::plot or Tensor::show
automatic expression tracing
SVG, HTML, Vega-Lite, or JSON output
notebook, GUI, or dashboard integration
published report or visualization crates
```
