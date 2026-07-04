# matten-report

`matten-report` is a local-only, `publish = false` reporting tool for small
plain-text readiness summaries.

Current supported report kind:

```text
data-readiness
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

Run the demo report:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness
```

Demo mode uses the fixed `sales,cost` selection.

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

Run the local hardening tests:

```bash
cargo test --manifest-path tools/matten-report/Cargo.toml
```

The tool never creates files unless `--output` is provided. It does not modify
input files.
