# matten-report

`matten-report` is a local-only, `publish = false` reporting tool for small
plain-text readiness summaries.

Current supported reports:

```text
data-readiness
shape-flow
dynamic-readiness
mlprep-standardization
educational-path
```

Scope:

```text
local tool only
Markdown/plain text output
explicit input/output behavior
no public API
no published crate
no network
no telemetry
no project mutation
```

The local report tool enables matten's dynamic feature only for its own demo
reporting. This does not change core matten defaults or any published crate's
dependency graph.

The local report tool depends on matten-mlprep only for its own fixed demo
report. This does not change any published crate dependency graph or core
matten defaults.

Run the data-readiness demo report:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness
```

Demo mode uses the fixed `sales,cost` selection.

Run the shape-flow demo report:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo shape-flow
```

The shape-flow report is a fixed demo for common shape transformations. It is
not automatic expression tracing and does not inspect source files.

Run the dynamic-readiness demo report:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo dynamic-readiness
```

The dynamic-readiness report is a fixed demo for dynamic values, missing-value
masks, strict numeric readiness, and explicit policy conversion. It is not
automatic data profiling and does not accept input files.

Run the mlprep-standardization demo report:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo mlprep-standardization
```

The mlprep-standardization report is a fixed demo for `standardize_columns`,
before/after values, column means, population standard deviations, and unchanged
shape. It is not model-quality analysis and does not accept input files.

Run the educational-path demo report:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo educational-path
```

The educational-path report is a fixed guided walkthrough across shape-first
reading, broadcasting, reshape/transpose, axis reductions, matmul, dynamic
readiness, and standardization. It is not automatic expression tracing and does
not accept input files.

Run on a CSV file:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --input tools/matten-report/fixtures/small.csv \
  --kind data-readiness \
  --select sales,cost
```

Write to an explicit output file:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --demo data-readiness \
  --output target/matten-report-demo.md
```

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --demo shape-flow \
  --output target/matten-report-shape-flow.md
```

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --demo dynamic-readiness \
  --output target/matten-report-dynamic-readiness.md
```

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --demo mlprep-standardization \
  --output target/matten-report-mlprep-standardization.md
```

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --demo educational-path \
  --output target/matten-report-educational-path.md
```

Run the local hardening tests:

```bash
cargo test --manifest-path tools/matten-report/Cargo.toml
```

The tool never creates files unless `--output` is provided. It does not modify
input files.
