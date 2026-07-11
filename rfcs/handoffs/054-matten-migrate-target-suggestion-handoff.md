# RFC-054 `matten-migrate` Target Suggestion Handoff

**Project:** `matten`
**Related RFC:** RFC-054: `matten-migrate` Assisted Migration Tool
**Document kind:** Compact implementation handoff
**Status:** Accepted; implementation prepared for review
**Scope:** Local-only target-specific advisory output for `tools/matten-migrate`

---

## 1. Summary

This handoff defines the next small RFC-054 slice after the reviewed first
local advisory tool.

Authorized next slice, if this handoff is accepted:

```text
matten-migrate suggest --target <target> <path>
```

The command should inspect a local project with the existing scanner and print a
target-specific advisory note. It must not pick a target for the user, mutate
source, edit `Cargo.toml`, run network lookups, execute commands in the
inspected project, or claim that any target is automatically better.

The command is a focused reading aid:

```text
"You asked about ndarray; here is the local evidence that may or may not make
that playbook worth reading."
```

It is not a migration decision engine.

---

## 2. Background

The first RFC-054 implementation is complete:

```text
3c1207e Add local matten-migrate advisory tool
47aa8d6 Align RFC-054 status after matten-migrate slice
```

Current supported commands:

```text
inspect <path>
report <path> [--output <path>]
list-targets
--help
```

The first tool already emits cautious report-level target playbook suggestions.
This slice narrows the interaction to a target the user explicitly asks about.
That keeps the boundary honest: the tool explains evidence relative to a chosen
target, rather than recommending a target as a decision.

---

## 3. Command Shape

Implement only:

```text
matten-migrate suggest --target <target> <path>
```

Accepted targets:

```text
ndarray
nalgebra
polars-pandas
candle
numpy
stay-with-matten
```

Optional aliases may be accepted only if they are deterministic and documented:

```text
polars
pandas
stay
matten
```

Reject unsupported forms with readable errors:

```text
matten-migrate suggest <path>
matten-migrate suggest --target <target>
matten-migrate suggest --target unknown <path>
matten-migrate suggest --all <path>
```

Do not implement in this slice:

```text
matten-migrate rewrite
matten-migrate apply
matten-migrate explain-api
matten-migrate check-bridges
matten-migrate suggest --all
matten-migrate suggest --output <path>
matten-migrate suggest --target <target> --edit
matten-migrate suggest --target <target> --json
```

No source or output file should be written by `suggest` in this slice.

---

## 4. Output Shape

Print deterministic Markdown to stdout:

```text
# matten Migration Target Suggestion

> advisory disclaimer
> detection-limits disclaimer

Target: `<target>`.
Project: `<project>`.

## Local evidence
## Target fit notes
## Manual checks
## Risks
## Next steps
```

Required tone:

```text
read this playbook if
may be relevant when
manual review should decide
profile before moving hot paths
stay with matten if small/readable glue is enough
```

Forbidden tone:

```text
must migrate
best target
guaranteed
faster
drop-in replacement
automatic conversion
```

The output must include the same advisory and heuristic-detection disclaimers
used by `report`.

---

## 5. Target-Specific Rules

`ndarray`:

```text
positive evidence:
  shape operations
  reductions
  matten-ndarray bridge
  N-D Tensor construction
notes:
  good candidate to read when dense N-D operations become measured hot paths
  bridge copies both ways and stays f64
manual checks:
  rank/shape assumptions
  conversion boundary frequency
```

`nalgebra`:

```text
positive evidence:
  linear algebra
  matmul / dot / norm / trace
notes:
  good candidate to read for dense matrix/vector redesigns and solvers
manual checks:
  fixed vs dynamic dimensions
  solver/decomposition needs
```

`polars-pandas`:

```text
positive evidence:
  matten-data usage plus dataframe pressure
notes:
  do not suggest merely because matten-data appears
  useful only when table analytics such as group-by/join/pivot/query are real requirements
manual checks:
  whether table work should remain a simple ingestion boundary
```

`candle`:

```text
positive evidence:
  no strong local evidence in this slice unless explicit training/device/model terms are detected by a reviewed scanner refinement
notes:
  avoid claiming training/device needs unless source evidence is explicit
  first slice should usually say "no strong local evidence"
  do not treat a single matmul/dot occurrence as ML pressure
manual checks:
  training loop
  device execution
  model serialization
```

`numpy`:

```text
positive evidence:
  Python ecosystem handoff must be explicit or manually confirmed
notes:
  first slice may often say "no strong local evidence"
manual checks:
  whether downstream work belongs next to Python data/science tooling
```

`stay-with-matten`:

```text
positive evidence:
  no strong production pressure
  small construction/ingestion/glue usage
  dynamic ingestion used as cleanup before numeric conversion
notes:
  staying with matten is a valid outcome
manual checks:
  readability
  workload size
  whether profiling shows real hot paths
```

---

## 6. Safety and Detection Boundary

Reuse the existing scanner unless the implementation review explicitly accepts a
small, local refinement.

Keep the first-tool safety model:

```text
local filesystem path only
no network
no telemetry
no source upload
no command execution inside the inspected project
no source mutation
no Cargo.toml mutation
no output file mutation for suggest
scan Cargo.toml and *.rs only
skip symlinks
cap large files with warnings
deterministic traversal and output
```

Do not introduce a parsing dependency in this slice. If implementation proves
the standard-library scanner is insufficient, stop and return for another
handoff instead of adding a parser opportunistically.

---

## 7. Fixtures and Tests

Extend the existing `tools/matten-migrate` test suite.

Required tests:

```text
[ ] exact output for one positive target, preferably ndarray or nalgebra
[ ] polars-pandas does not become positive for matten-data alone
[ ] polars-pandas is positive for matten-data plus dataframe pressure
[ ] stay-with-matten is positive for no-pressure/simple usage
[ ] receiver-method numeric evidence remains detected
[ ] common Rust collision fixture remains clean
[ ] unsupported targets are rejected
[ ] missing target/path errors are readable
[ ] rewrite/apply/explain-api/check-bridges remain rejected
[ ] suggest writes no files
```

The exact-output test must assert advisory wording and absence of forbidden
phrases.

---

## 8. Documentation

Update:

```text
tools/matten-migrate/README.md
docs/src/contributing/release-checklist.md
.github/workflows/test.yaml
```

Update user-facing migration docs only if needed. If updated, keep the wording
short and preserve the existing boundaries:

```text
local-only
advisory-only
not a source rewriter
manual review required
heuristic detection may miss or over-report usage
```

---

## 9. Verification

Required checks for implementation review:

```text
cargo fmt --all
cargo check --manifest-path tools/matten-migrate/Cargo.toml
cargo test --manifest-path tools/matten-migrate/Cargo.toml
cargo clippy --manifest-path tools/matten-migrate/Cargo.toml -- -D warnings
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- suggest --target ndarray tools/matten-migrate/fixtures/receiver-method-project
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- suggest --target polars-pandas tools/matten-migrate/fixtures/common-rust-collisions-project
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- suggest --target stay-with-matten tools/matten-migrate/fixtures/simple-core-project
bash scripts/check-release-docs.sh
git diff --check
```

Optional if user-facing docs are changed:

```text
mdbook build docs
```

---

## 10. Acceptance Criteria

The implementation is accepted when:

```text
[ ] suggest --target <target> <path> works deterministically
[ ] output is target-specific but advisory
[ ] unsupported targets/forms fail clearly
[ ] no source/output mutation exists
[ ] no network/telemetry/source-upload behavior exists
[ ] no published crate source or dependency graph changes
[ ] existing inspect/report/list-targets behavior is not regressed
[ ] false-positive regression fixtures still pass
[ ] exact-output tests cover disclaimers and forbidden wording
[ ] CI and release checklist include suggest smoke commands
```

---

## 11. Still Deferred

Still deferred after this slice:

```text
rewrite/apply
Cargo.toml mutation
automatic bridge dependency insertion
AST parser
cargo metadata / cargo check integration
ML/code-transformation assistance
remote analysis
public matten-migrate crate
stable report-format promise
explain-api
check-bridges
suggest --all
suggest --output
configuration file support
```
