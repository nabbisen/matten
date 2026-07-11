# RFC-054 `matten-migrate` First Tool Handoff

**Project:** `matten`
**Related RFC:** RFC-054: `matten-migrate` Assisted Migration Tool
**Document kind:** Compact implementation handoff
**Status:** Accepted; implementation prepared for review
**Scope:** Local-only advisory migration inspection/report tool, first slice only

---

## 1. Summary

This handoff defines the first possible implementation slice for RFC-054 after
the readiness audit.

Authorized first slice, if this handoff is accepted:

```text
tools/matten-migrate
publish = false
workspace-excluded
local-only
advisory-only
non-mutating
```

The tool should help a user inspect a local Rust project that uses `matten` and
produce a conservative Markdown migration-readiness report. It must not rewrite
source code, edit `Cargo.toml`, run network lookups, upload source, or claim that
any target is automatically better.

---

## 2. Reviewer Background

RFC-054 was accepted only as a future direction and explicitly deferred until
the migration guide foundation stabilized. That foundation now exists:

```text
RFC-050 production migration guide
RFC-051 bridge conversion contracts
RFC-052 target playbooks
RFC-053 readiness report template
one worked readiness report
```

The readiness audit concluded:

```text
READY TO DRAFT FIRST-TOOL HANDOFF
NOT READY FOR DIRECT IMPLEMENTATION
```

The handoff therefore keeps one more review gate before code. It defines the
smallest acceptable first tool and the fixtures/evidence needed to avoid brittle
source-scanning overreach.

---

## 3. First-Slice Commands

Implement only:

```text
matten-migrate inspect <path>
matten-migrate report <path>
matten-migrate list-targets
matten-migrate --help
```

Do not implement in this slice:

```text
matten-migrate explain-api <api-name>
matten-migrate suggest --target <target>
matten-migrate check-bridges
matten-migrate rewrite
matten-migrate apply
```

`explain-api` is intentionally excluded from the first slice. It may be useful
later, but it risks becoming a correctness oracle before the inspection model is
proven.

---

## 4. Packaging

Create:

```text
tools/matten-migrate/Cargo.toml
tools/matten-migrate/src/main.rs
tools/matten-migrate/README.md
tools/matten-migrate/fixtures/
```

Tool policy:

```text
publish = false
workspace-excluded
version = 0.0.0
path-only local dependencies if needed
no dependency added to core matten
no dependency added to published companion crates
```

Root `Cargo.toml` must exclude `tools/matten-migrate` like the existing
`tools/matten-report` tool.

Prefer the standard library first. If a parsing dependency is proposed, the
handoff implementation must justify why it is tool-only and why plain text
scanning is insufficient.

---

## 5. Input and Output Policy

Input:

```text
local filesystem path only
directory or file path accepted only if explicitly handled
no network
no source upload
no shelling out to build the target project
no command execution inside the inspected project
```

Output:

```text
inspect: terminal summary
report: Markdown to stdout by default
report --output <path>: optional explicit output file
list-targets: terminal list of known migration targets
```

The tool must never create or modify files unless `--output <path>` is provided.
It must never modify the inspected project.

---

## 6. Detection Model

Use conservative, transparent detection. The first version may scan:

```text
Cargo.toml dependency names and feature strings
Rust source text for known matten API names
example file names
```

Do not claim AST-level certainty. Do not claim the project compiles. Do not run
`cargo metadata`, `cargo check`, or external tools in the inspected project for
the first slice.

Bound the scan:

```text
scan Cargo.toml files
scan *.rs files
skip non-Rust/text blobs except Cargo.toml
do not follow symlinks outside the inspected path
skip or cap very large files with a reported warning
keep traversal deterministic
```

Suggested signal groups:

```text
core Tensor construction: Tensor::new, try_new, from_vec, scalar, zeros, ones
shape operations: reshape, flatten, transpose, squeeze, expand_dims
reductions: sum, mean, *_axis, var, std
linear algebra: matmul, dot, outer, norm, trace
dynamic ingestion: Element, NumericPolicy, try_numeric, from_json_dynamic, from_csv_dynamic
matten-data: matten_data, Table, select_columns, fill_missing, try_numeric, to_tensor
matten-ndarray: matten_ndarray, to_arrayd, from_arrayd
ml-like pressure: repeated matmul/dot patterns only as a weak signal
```

Each detected signal must be reported as evidence, not proof.

Dataframe pressure terms should align with the RFC-042 scope vocabulary:

```text
groupby
group_by
join
merge
pivot
query
rolling
dataframe
data_frame
series
loc
iloc
```

`matten-data` usage alone should not recommend Polars/Pandas. Recommend the
Polars/Pandas playbook only when table-to-Tensor usage appears with dataframe
pressure terms or explicit user-facing table-analytics pressure.

---

## 7. Report Shape

The Markdown report should follow the RFC-053 manual template vocabulary, with
two deliberate tool-specific heading changes:

```text
Current matten usage       -> Detected matten usage
Recommended target(s)      -> Suggested target playbooks
```

Those names are intentionally more cautious for a heuristic tool: the tool
detects evidence and suggests docs to read; it does not know the full current
usage and does not recommend a target as a decision.

```text
# matten Migration Readiness Report

## Summary
## Detected matten usage
## Production pressure signals
## Suggested target playbooks
## Direct conversion candidates
## Manual redesign areas
## Bridge crates / tools
## Risks
## Next steps
```

Required disclaimer near the top:

```text
This report is advisory. It does not prove production readiness, does not guarantee
a target library is better, and does not perform automatic conversion.
Detection is a heuristic text/dependency scan. It may miss real matten usage and
may over-report source-like text as usage. It has not been validated against real
downstream projects; treat results as a starting point for manual review.
```

Suggestions must link conceptually to the existing playbooks:

```text
ndarray
nalgebra
Polars / Pandas
Candle
NumPy
stay with matten
```

Use cautious wording:

```text
consider
may indicate
read the playbook
manual review required
```

Avoid:

```text
must migrate
production-ready
automatic conversion
guaranteed
faster
best target
```

---

## 8. Required Fixtures

Because no real downstream-project usage is proven yet, fixtures are
load-bearing. Include both curated and messy fixtures.

Minimum fixtures:

```text
simple-core-project
  Cargo.toml with matten
  src/main.rs with Tensor construction, reshape, mean_axis

data-project
  Cargo.toml with matten + matten-data
  src/main.rs with Table::from_csv_str, select_columns, fill_missing, try_numeric, to_tensor

ndarray-bridge-project
  Cargo.toml with matten + matten-ndarray
  src/main.rs with to_arrayd/from_arrayd

messy-nonproject
  no Cargo.toml
  mixed Rust/text files
  comments mentioning matten APIs
  source-like strings that should be reported with low confidence or ignored

no-matten-project
  Cargo.toml without matten
  src/main.rs without matten usage
```

Tests must prove:

```text
no-matten project reports no matten usage, not an error
comments/prose do not become high-confidence API detections
matten-data signals point to Polars/Pandas only when dataframe pressure terms are present
matten-ndarray signals point to the bridge path, not automatic migration
report output is deterministic
report exact-output test proves disclaimer presence and forbidden wording absence
--output writes only the requested file
```

---

## 9. CLI Policy

`inspect <path>`:

```text
prints detected crates/features/signals
prints limitations
does not generate a long report
```

`report <path>`:

```text
prints Markdown report to stdout
accepts --output <path>
uses deterministic ordering
```

`list-targets`:

```text
prints target names and one-line descriptions
does not inspect a project
```

Errors:

```text
missing path: readable error
unsupported command: readable error
bad --output path: no partial mutation beyond normal failed create/write behavior
```

---

## 10. Required Documentation

Add:

```text
tools/matten-migrate/README.md
```

Update, only if the implementation exists:

```text
docs/src/migration/index.md
docs/src/migration/readiness-report.md
docs/src/contributing/release-checklist.md
.github/workflows/test.yaml
```

Docs must say:

```text
local-only
advisory-only
non-mutating
heuristic detection may miss or over-report usage
not validated against real downstream projects yet
manual review required
no rewrite/apply commands
no network
no telemetry
no source upload
```

---

## 11. Verification

Required checks for the implementation review:

```text
cargo check --manifest-path tools/matten-migrate/Cargo.toml
cargo test --manifest-path tools/matten-migrate/Cargo.toml
cargo clippy --manifest-path tools/matten-migrate/Cargo.toml -- -D warnings
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- list-targets
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- inspect tools/matten-migrate/fixtures/simple-core-project
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- report tools/matten-migrate/fixtures/simple-core-project
bash scripts/check-release-docs.sh
git diff --check
```

Optional if docs are changed:

```text
mdbook build docs
```

---

## 12. Acceptance Criteria

The first implementation is accepted when:

```text
[ ] tool is workspace-excluded and publish=false
[ ] no published crate source or dependency graph changes
[ ] no rewrite/apply/source mutation exists
[ ] no network/telemetry/source-upload behavior exists
[ ] inspect/report/list-targets work deterministically
[ ] Markdown report uses RFC-053 advisory wording plus detection-uncertainty disclosure
[ ] fixtures cover curated and messy inputs
[ ] tests cover false-positive limitations
[ ] exact-output tests cover report disclaimer and forbidden wording
[ ] CI and release checklist include manifest-path checks
[ ] docs describe limitations honestly
```

---

## 13. Explicitly Deferred

Still deferred after this first slice:

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
suggest --target
check-bridges
```
