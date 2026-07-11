# RFC-054 `matten-migrate` API Explanation Handoff

**Project:** `matten`
**Related RFC:** RFC-054: `matten-migrate` Assisted Migration Tool
**Document kind:** Compact implementation handoff
**Status:** Accepted; implementation prepared for review
**Scope:** Local-only static API explanation output for `tools/matten-migrate`

---

## 1. Summary

This handoff defines the next small RFC-054 slice after the reviewed
`suggest --target` command.

Authorized next slice, if this handoff is accepted:

```text
matten-migrate explain-api <api-name>
```

The command should print a deterministic, local, static explanation for one
migration-relevant `matten` API or bridge API. It must not inspect a project,
scrape documentation, infer source intent, mutate files, edit `Cargo.toml`, run
network lookups, or claim complete API coverage.

The command is a glossary entry:

```text
"You asked about Tensor::matmul; here is what it means in matten, where it tends
to map in migration playbooks, and what manual checks remain."
```

It is not a compatibility oracle, rewrite engine, or target-selection decision.

---

## 2. Background

The current RFC-054 local tool supports:

```text
inspect <path>
report <path> [--output <path>]
suggest --target <target> <path>
list-targets
--help
```

The remaining RFC-054 command that can be added without scanning expansion is:

```text
explain-api <api-name>
```

`check-bridges`, configuration files, public crates, and rewrite/apply remain
larger future work.

---

## 3. Command Shape

Implement only:

```text
matten-migrate explain-api <api-name>
```

No path argument is accepted in this slice.

Reject unsupported forms with readable errors:

```text
matten-migrate explain-api
matten-migrate explain-api <api-name> <path>
matten-migrate explain-api --target ndarray <api-name>
matten-migrate explain-api --json <api-name>
matten-migrate explain-api --all
```

Do not implement in this slice:

```text
matten-migrate check-bridges
matten-migrate rewrite
matten-migrate apply
matten-migrate explain-api --all
matten-migrate explain-api --json
matten-migrate explain-api --output <path>
matten-migrate explain-api --target <target>
```

No source or output file should be written by `explain-api`.

---

## 4. API Catalog Boundary

Use a small curated static catalog. The first implementation should not try to
cover every public API.

Required first catalog:

```text
Tensor::new
Tensor::try_new
Tensor::from_vec
Tensor::reshape
Tensor::try_reshape
Tensor::flatten
Tensor::transpose
Tensor::sum
Tensor::mean
Tensor::sum_axis
Tensor::mean_axis
Tensor::dot
Tensor::matmul
Tensor::norm
Tensor::trace
Tensor::outer
Tensor::try_numeric
Tensor::from_json_dynamic
Tensor::from_csv_dynamic
matten_ndarray::to_arrayd
matten_ndarray::from_arrayd
matten_data::Table
matten_data::try_numeric
matten_data::to_tensor
```

Optional aliases may be accepted only if deterministic and documented:

```text
new -> Tensor::new
matmul -> Tensor::matmul
dot -> Tensor::dot
reshape -> Tensor::reshape
sum_axis -> Tensor::sum_axis
mean_axis -> Tensor::mean_axis
to_arrayd -> matten_ndarray::to_arrayd
from_arrayd -> matten_ndarray::from_arrayd
Table -> matten_data::Table
```

Unsupported APIs should fail clearly and point users to the public API snapshot
or migration guide, not guess.

Each catalog entry must be verified against
`docs/src/reference/public-api-snapshot.md` before writing the glossary prose:

```text
existence
exact qualification
real access path
```

Do not describe a method that is only reachable indirectly as though it were a
direct method. Concrete trap to avoid:

```text
matten_data::to_tensor is NumericTable::to_tensor reached through
Table::try_numeric()?.to_tensor(); it is not Table::to_tensor.
```

Bare aliases are allowed only when unambiguous. Ambiguous short names must
require the qualified form and fail helpfully:

```text
try_numeric -> error listing Tensor::try_numeric and matten_data::try_numeric
to_tensor   -> error listing matten_data::to_tensor / NumericTable::to_tensor access path
```

---

## 5. Output Shape

Print deterministic Markdown to stdout:

```text
# matten API Migration Note

> advisory disclaimer

API: `<canonical-api>`.

## What it means in matten
## Migration relevance
## Possible target playbooks
## Manual checks
## Related docs
```

Required tone:

```text
this API usually means
read this playbook if
manual review should confirm
profile before moving hot paths
not a drop-in replacement
```

Forbidden tone:

```text
must migrate
best target
guaranteed
faster
automatic conversion
complete API coverage
```

The command should include the advisory disclaimer. It does not need the
project-detection disclaimer because it does not scan a project, but it must
state that the catalog is curated and incomplete.

---

## 6. Suggested API Group Semantics

Construction:

```text
Tensor::new / try_new / from_vec
matten meaning:
  construct numeric tensors from row-major data and runtime shape
migration relevance:
  often maps to target-specific array constructors
playbooks:
  ndarray, nalgebra, numpy, candle depending on workload
manual checks:
  shape order, allocation, panic vs Result boundary
```

Shape operations:

```text
reshape / try_reshape / flatten / transpose
matten meaning:
  shape or axis layout changes over numeric tensors
migration relevance:
  often maps to ndarray or NumPy reshape/view/copy decisions
manual checks:
  whether target operation is a view or copy
  row-major logical order
```

Reductions:

```text
sum / mean / sum_axis / mean_axis
matten meaning:
  whole-tensor or axis reductions
migration relevance:
  often maps to ndarray/NumPy axis reductions; may matter when measured hot
manual checks:
  axis meaning
  NaN behavior
  whether result shape is preserved or dropped
```

Linear algebra:

```text
dot / matmul / norm / trace / outer
matten meaning:
  core-lite dense numeric helpers, not a full linalg backend
migration relevance:
  ndarray or nalgebra playbooks may be relevant when these dominate runtime
manual checks:
  rank/shape cases
  solver/decomposition needs
  target semantics and error model
```

Dynamic ingestion:

```text
try_numeric / from_json_dynamic / from_csv_dynamic
matten meaning:
  explicit cleanup and conversion from dynamic values before numeric work
migration relevance:
  often remains a useful matten boundary before handing numeric data elsewhere
manual checks:
  conversion policy
  missing/non-numeric values
  whether data cleanup belongs in table/Python tooling
```

Bridge/data helpers:

```text
matten_ndarray::to_arrayd / from_arrayd
matten_data::Table / try_numeric / to_tensor
matten meaning:
  explicit dependency boundary or table-to-Tensor on-ramp
migration relevance:
  bridge conversion should happen at boundaries, not inside hot loops
manual checks:
  copy boundary
  dynamic rejection
  whether table analytics exceed matten-data scope
```

---

## 7. Safety and Maintenance Boundary

Keep this slice:

```text
static catalog only
local-only
stdout-only
no project path
no network
no telemetry
no source upload
no source mutation
no Cargo.toml mutation
no parser dependency
no docs scraping
no claim that the catalog is complete
```

If the catalog becomes large enough to require generation from docs or Rustdoc,
stop and return for another handoff.

---

## 8. Tests

Extend the existing `tools/matten-migrate` test suite.

Required tests:

```text
[ ] exact output for Tensor::matmul
[ ] alias resolves deterministically, e.g. matmul -> Tensor::matmul
[ ] bridge API entry renders, e.g. matten_ndarray::to_arrayd
[ ] data API entry renders, e.g. matten_data::Table
[ ] unsupported API fails clearly
[ ] ambiguous bare API aliases fail clearly and list qualified candidates
[ ] missing API fails clearly
[ ] extra path/argument fails clearly
[ ] --json/--all/--output/--target forms are rejected
[ ] check-bridges/rewrite/apply remain rejected
[ ] existing inspect/report/suggest/list-targets tests still pass
```

Exact-output tests must assert advisory wording, curated/incomplete catalog
wording, and absence of forbidden phrases.

---

## 9. Documentation

Update:

```text
tools/matten-migrate/README.md
docs/src/contributing/release-checklist.md
.github/workflows/test.yaml
```

User-facing migration docs do not need updates unless the implementation changes
the public story. If updated, keep wording short:

```text
local static glossary
advisory-only
curated and incomplete
not source analysis
not a source rewriter
```

---

## 10. Verification

Required checks for implementation review:

```text
cargo fmt --all
cargo check --manifest-path tools/matten-migrate/Cargo.toml
cargo test --manifest-path tools/matten-migrate/Cargo.toml
cargo clippy --manifest-path tools/matten-migrate/Cargo.toml -- -D warnings
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- explain-api Tensor::matmul
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- explain-api matmul
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- explain-api matten_ndarray::to_arrayd
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- explain-api matten_data::Table
bash scripts/check-release-docs.sh
git diff --check
```

Optional if user-facing docs are changed:

```text
mdbook build docs
```

---

## 11. Acceptance Criteria

The implementation is accepted when:

```text
[ ] explain-api <api-name> works deterministically for the accepted catalog
[ ] output is static, advisory, and scoped to one API
[ ] unsupported APIs/forms fail clearly
[ ] output states catalog is curated and incomplete
[ ] each catalog entry was checked against docs/src/reference/public-api-snapshot.md for existence, qualification, and access path
[ ] ambiguous bare aliases require qualified names and have tests
[ ] no source/output mutation exists
[ ] no network/telemetry/source-upload behavior exists
[ ] no published crate source or dependency graph changes
[ ] existing inspect/report/suggest/list-targets behavior is not regressed
[ ] exact-output tests cover disclaimers and forbidden wording
[ ] CI and release checklist include explain-api smoke commands
```

---

## 12. Still Deferred

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
check-bridges
explain-api --all
explain-api --json
explain-api --output
configuration file support
generated API catalog
complete API coverage
```
