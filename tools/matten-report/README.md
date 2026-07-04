# matten-report

`matten-report` is a local-only, `publish = false` reporting tool for small
plain-text readiness summaries.

Current supported reports:

```text
data-readiness
shape-flow
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

Run the local hardening tests:

```bash
cargo test --manifest-path tools/matten-report/Cargo.toml
```

The tool never creates files unless `--output` is provided. It does not modify
input files.
