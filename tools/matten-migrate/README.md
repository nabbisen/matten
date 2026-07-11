# matten-migrate

`matten-migrate` is a local-only, `publish = false` advisory tool for
`matten` migration-readiness reports.

Scope:

```text
local tool only
advisory output only
heuristic text/dependency scan
non-mutating unless --output is provided
no rewrite/apply commands
no network
no telemetry
no source upload
no command execution inside the inspected project
```

Detection is heuristic. It may miss real `matten` usage and may over-report
source-like text as usage. It has not been validated against real downstream
projects. Treat results as a starting point for manual review.

Commands:

```bash
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- list-targets
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- inspect tools/matten-migrate/fixtures/simple-core-project
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- report tools/matten-migrate/fixtures/simple-core-project
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- report tools/matten-migrate/fixtures/simple-core-project --output target/matten-migration-report.md
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- suggest --target ndarray tools/matten-migrate/fixtures/receiver-method-project
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- suggest --target polars-pandas tools/matten-migrate/fixtures/common-rust-collisions-project
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- suggest --target stay-with-matten tools/matten-migrate/fixtures/simple-core-project
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- explain-api Tensor::matmul
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- explain-api matmul
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- explain-api matten_ndarray::to_arrayd
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- explain-api matten_data::Table
```

Supported commands:

```text
inspect <path>
report <path> [--output <path>]
suggest --target <target> <path>
explain-api <api-name>
list-targets
--help
```

`explain-api` prints a local static glossary entry for one curated API. The
catalog is advisory, curated, and incomplete; verify API details against the
reference docs before using the output as migration guidance.

Supported targets:

```text
ndarray
nalgebra
polars-pandas
candle
numpy
stay-with-matten
```

Supported aliases:

```text
polars
pandas
stay
matten
```

Not supported:

```text
rewrite
apply
check-bridges
explain-api --all
explain-api --json
explain-api --output <path>
explain-api --target <target>
suggest --all
suggest --output
```

Run local checks:

```bash
cargo test --manifest-path tools/matten-migrate/Cargo.toml
cargo clippy --manifest-path tools/matten-migrate/Cargo.toml -- -D warnings
```
