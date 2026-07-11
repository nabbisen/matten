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
```

Supported commands:

```text
inspect <path>
report <path> [--output <path>]
list-targets
--help
```

Not supported:

```text
rewrite
apply
explain-api
suggest --target
check-bridges
```

Run local checks:

```bash
cargo test --manifest-path tools/matten-migrate/Cargo.toml
cargo clippy --manifest-path tools/matten-migrate/Cargo.toml -- -D warnings
```
