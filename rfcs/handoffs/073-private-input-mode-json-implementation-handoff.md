# RFC-073 Private Input-Mode JSON Implementation Handoff

**Project:** `matten`  
**Related RFC:** RFC-073: Private Input-Mode JSON Report Policy  
**Document kind:** Detailed local-tool implementation handoff  
**Status:** Accepted; implementation completed and prepared for review; release decision unauthorized
**Date:** 2026-07-23

---

## 1. Purpose

Implement one bounded private JSON path in the workspace-excluded,
`publish = false` `tools/matten-report` binary:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --input tools/matten-report/fixtures/small.csv \
  --kind data-readiness \
  --select sales,cost \
  --format json \
  --output target/matten-report-input.json
```

The implementation is one coherent checkpoint. It adds no public API or
schema, no dependency, no published-crate behavior, and no release decision.

## 2. Authority And Reviewed Policy

RFC-073 is the design authority. Its accepted rulings are:

```text
data-readiness CSV input only
explicit --format json and --output
private schema_version 0 / private-local
input_mode = "csv"
summary-only allowlist
structured user-data bounds
success and strict-conversion-error report artifacts
tensor-construction failure remains a pre-write command error
other pre-write failures preserve absent/existing destinations
std::fs::write failures have no destination-state guarantee
serde_json encoding and non-finite rejection
fixed-demo JSON remains byte-identical
no public surface or release authorization
```

The accepted output-failure distinction must not be weakened:

| Failure stage | Required destination contract |
|---|---|
| Before `output::write` | no new artifact; existing destination byte-identical |
| Inside current `std::fs::write` | no destination-state guarantee |

## 3. Scope

### In Scope

```text
allow explicit input-mode data-readiness JSON in CLI policy
document the command in CLI help
route one built DataReadinessReportData value to an input JSON renderer
add input_mode = "csv" private envelope
add explicit private bounded JSON representation types
relocate the four shared report display limits to render::common without behavior change
render success and conversion-error outcomes
keep tensor previews finite and capped at 12
add exact source-owned snapshots and focused policy tests
add header-only CSV fixture and pre-write destination tests
add input JSON success/error smoke commands to CI and release checklist
update local-tool and visual-understanding docs
align RFC/roadmap status after implementation
```

### Out Of Scope

```text
JSON stdout
input JSON for any report kind other than data-readiness
raw CSV rows/cells/table export
public or stable schema
schema_version 1
public Report or renderer types
public matten-report / matten-viz crate
workspace membership or publishability changes
new dependencies
HTML behavior or structure changes beyond importing shared display constants
new error-code taxonomy
atomic output, rollback, cleanup, or write-failure injection
SVG / Vega-Lite / JavaScript / notebook / server / GUI scope
core Tensor visualization, expression tracing, or autograd
mathematics, bridges, streaming, or large-CSV changes
version, changelog, release notes, release prep, tag, or publish
checked-in generated JSON artifacts
```

## 4. Required Module Ownership

Use the existing RFC-072 graph:

```text
cli -> request policy
app -> load/build once, then select format
report::data_readiness -> format-neutral report data and normalization
render::common -> format-neutral report display limits
render::json::data_readiness::input -> CSV-input JSON representation
output -> unchanged stdout/file delivery
```

Required file shape:

```text
tools/matten-report/src/render/json/data_readiness.rs
tools/matten-report/src/render/json/data_readiness/input.rs
tools/matten-report/src/render/json/data_readiness/input/tests.rs
tools/matten-report/src/render/json/data_readiness/input/tests/success.rs
tools/matten-report/src/render/json/data_readiness/input/tests/error.rs
tools/matten-report/src/render/json/data_readiness/input/tests/policy.rs
```

The exact test split may be reduced if every resulting Rust file remains below
300 physical lines. No file may exceed the enforced 500-line ceiling.

Ownership rules:

```text
data_readiness.rs keeps fixed-demo JSON ownership and exposes its input child
input.rs owns input JSON structs, bounded-value mapping, conversion mapping, and serialization
render::common owns the four shared report display-limit constants
model.rs continues to own tensor-preview behavior and fixed-demo model types while reading the shared value limit
html::data_readiness reads the shared limits without changing rendering behavior
report::data_readiness remains unchanged
app must not inspect conversion details
input renderer must not load CSV, select columns, or construct Tensor values
output.rs must remain unchanged
```

Do not introduce a forwarding renderer. `app` should call
`render::json::data_readiness::input::render(&data)` directly.

## 5. CLI And App Changes

### CLI

`validate_json_format_policy` should accept:

```text
Input::CsvPath { .. } when kind == ReportKind::DataReadiness
```

The existing first check still requires `--output <report.json>`. Replace the
rejection test with:

```text
input_mode_json_requires_output
input_mode_json_allows_explicit_output
```

The accepted configuration must preserve the input path, selected columns,
kind, JSON format, and output path.

Add this exact usage shape to help:

```text
matten-report --input <csv-path> --kind data-readiness --select <col1,col2> --format json --output <report.json>
```

This help line is an expected process-contract change. The implementation
review must record the new exact help bytes and SHA-256. All other usage and
policy text must remain unchanged.

### App

Remove both input-mode JSON rejection branches. The CSV path must retain this
order:

```text
Table::from_csv_path
report::data_readiness::build
format dispatch
render::json::data_readiness::input::render
output::write after render_report returns
```

The data model is built once. Markdown and HTML calls remain unchanged.

## 6. Exact Private JSON Shape

### 6.1 Envelope

Input JSON field order is normative for deterministic source snapshots:

```json
{
  "schema_version": 0,
  "schema_status": "private-local",
  "tool": "matten-report",
  "report_kind": "data-readiness",
  "input_mode": "csv",
  "limits": {},
  "data": {}
}
```

Define an input-specific private envelope in the input renderer. Do not add an
optional field to the fixed-demo envelope because that risks output drift.
Rename the existing fixed-demo envelope helper only if needed for clarity; if
renamed, all five fixed-demo artifacts must remain byte-identical.

Envelope values are exact:

```text
schema_version = 0
schema_status = "private-local"
tool = "matten-report"
report_kind = "data-readiness"
input_mode = "csv"
```

Pretty-print through `serde_json::to_string_pretty` and append one newline.

### 6.2 Limits

`limits` field order and values:

```json
{
  "max_display_columns": 12,
  "max_display_chars": 120,
  "max_error_chars": 240,
  "max_tensor_preview_values": 12
}
```

Relocate all four existing values to `render::common` as crate-private
format-neutral presentation constants. `html::data_readiness`,
`json::model::json_tensor_preview`, and the new input JSON owner must import
them from that single owner. This is a constant relocation only: HTML output,
fixed-demo JSON output, and all truncation behavior remain byte-identical.

The snapshot and policy tests must assert the four emitted values. The single
constant owner makes HTML/JSON drift structurally impossible without adding a
cross-format test dependency.

### 6.3 Bounded String

Exact field order:

```json
{
  "value": "prefix",
  "truncated": false,
  "shown_chars": 6,
  "total_chars": 6,
  "limit": 120
}
```

Rules:

```text
count Rust chars / Unicode scalar values, not bytes
value is the first min(total_chars, limit) chars
do not append "..."; metadata carries truncation
shown_chars equals value.chars().count()
truncated equals total_chars > limit
```

Use limit 120 for input labels and column names, and 240 for normalized
conversion-error messages.

### 6.4 Bounded List

Exact field order:

```json
{
  "items": [],
  "truncated": false,
  "shown_items": 0,
  "total_items": 0,
  "limit": 12
}
```

Rules:

```text
retain source order
take the first min(total_items, 12) items
do not add a synthetic "... N more" item
shown_items equals items.len()
truncated equals total_items > 12
```

Every string item is itself a bounded string. Missing-count entries use:

```json
{
  "column": { "value": "sales", "truncated": false, "shown_chars": 5, "total_chars": 5, "limit": 120 },
  "missing": 0
}
```

The entire missing-count entry list is bounded to 12.

### 6.5 Data Object

Exact field order:

```text
input_label
source_columns
selected_columns
left_out_columns
missing_counts
numeric_conversion
```

The first field is one bounded string. The next four fields use bounded lists;
their items follow Sections 6.3-6.4.

### 6.6 Numeric Conversion

Success field order:

```json
{
  "status": "success",
  "tensor": {
    "shape": [],
    "values": [],
    "truncated": false,
    "shown_values": 0,
    "total_values": 0,
    "limit": 12
  }
}
```

Use the existing `JsonTensorPreview` and `json_tensor_preview`; do not duplicate
finite-number validation or tensor truncation.

Conversion-error field order:

```json
{
  "status": "error",
  "message": {
    "value": "normalized message",
    "truncated": false,
    "shown_chars": 18,
    "total_chars": 18,
    "limit": 240
  }
}
```

The error variant has no `tensor` or raw `MattenDataError` fields. Use an
internally tagged private enum or an equivalent explicit representation that
serializes `status` first.

## 7. Data Disclosure And Determinism

Allowed data:

```text
bounded provided-path label; no canonicalization
bounded source/selected/left-out column names
bounded missing counts for selected columns
bounded normalized conversion-error text
bounded finite numeric tensor preview
shape and truncation metadata
```

Forbidden data:

```text
full raw rows or table
unselected cell values
unbounded strings/lists/tensor values
timestamps, hostnames, environment, random IDs, package/build metadata
canonicalized or independently discovered paths
```

The same input bytes, path spelling, selection order, and tool build must
produce byte-identical output.

## 8. Failure Contract And Tests

### 8.1 Report Outcomes

```text
try_numeric fails -> exit 0, bounded error JSON artifact
try_numeric and to_tensor succeed -> exit 0, success JSON artifact
```

### 8.2 Pre-Write Command Failures

These must fail before `output::write`:

```text
input open / CSV parse
missing, duplicate, or empty selection
to_tensor failure after successful conversion
JSON serialization or representation-policy rejection
```

Required process assertions:

```text
nonzero exit
stdout empty
one newline-terminated stderr message with "matten-report error: " prefix
absent output path remains absent
existing output bytes remain unchanged
```

Add `tools/matten-report/fixtures/header_only.csv` with exactly:

```csv
a,b
```

The process harness must run the zero-row path once with an absent destination
and once with an existing sentinel destination. Both use `--select a,b`.

Non-finite input must be rejected by the JSON representation policy before
writing. Cover the renderer mapping directly; add a process case only if a
small stable CSV fixture reaches that path without changing `matten-data`.

### 8.3 Write-Time Failure

Retain the existing missing-parent process case as routing evidence:

```text
nonzero exit
stdout empty
stderr prefix and trailing newline
```

Do not assert destination absence/preservation as a general `fs::write`
contract. Do not add atomic-write code or failure injection.

## 9. Source-Owned Test Matrix

### CLI Tests

```text
input JSON without --output rejects with the existing exact policy error
input data-readiness JSON with explicit output returns the expected Config
other input report kinds remain rejected by the existing kind policy
```

### Renderer Snapshot Tests

```text
small.csv success exact full-string JSON
non_numeric.csv conversion-error exact full-string JSON
one trailing newline
input_mode = "csv"
limits object and field order exact
rendering the same data twice is byte-identical
```

Keep snapshots as Rust string literals beside the renderer. Do not check in
generated `.json` files.

### Renderer Policy Tests

```text
quotes, backslashes, controls, and non-ASCII round-trip through serde_json
120-char and 240-char boundaries pass without truncation
121-char and 241-char inputs truncate with exact metadata
wide source/selected/left-out/missing lists expose first 12 and exact totals
long Unicode strings count chars rather than UTF-8 bytes
14 tensor values expose 12 and report total_values = 14
NaN / +Infinity / -Infinity reject with the established error
no synthetic "... N more" list item
no raw full long value appears
```

### Existing Regression Tests

```text
all 60 current tool tests remain
all five fixed-demo JSON exact snapshots remain byte-identical
fixed-demo data-readiness remains 952 bytes with SHA-256
  6491d3856293572e80f0388be6002703178336447f24afb330087c82ad680fac
fixed-demo Markdown remains 404 bytes
module-boundary normal and self-test gates remain green
```

The new total test count must be reported at implementation review; do not
rename or replace existing tests to preserve the count.

## 10. Process Harness, CI, And Checklist

Extend `tools/matten-report/tests/process-boundary.sh` with:

```text
input JSON success file: exact bytes/SHA-256, stdout/stderr empty
input JSON conversion-error file: exact bytes/SHA-256, stdout/stderr empty
header-only absent destination: nonzero, routing correct, file absent
header-only existing destination: nonzero, routing correct, sentinel unchanged
```

Record new input artifact bytes/hashes and the changed help bytes/hash in the
implementation review. Keep existing Markdown and fixed-demo JSON anchors
unchanged. Preserve the deliberate Markdown-digest mutation self-test.

Add these smoke commands to both `.github/workflows/test.yaml` and
`docs/src/contributing/release-checklist.md`:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --input tools/matten-report/fixtures/small.csv \
  --kind data-readiness --select sales,cost --format json \
  --output target/matten-report-input.json

cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --input tools/matten-report/fixtures/non_numeric.csv \
  --kind data-readiness --select sales,cost --format json \
  --output target/matten-report-input-error.json
```

Do not add release commands or version-specific release claims.

## 11. Documentation

Update:

```text
tools/matten-report/README.md
docs/src/examples/visual-understanding.md
ROADMAP.md
rfcs/README.md
rfcs/proposed/073-private-input-mode-json-report-policy.md
rfcs/handoffs/README.md
```

Documentation must state:

```text
input JSON is private schema v0 and explicit-file-only
only data-readiness input is supported
success and conversion-error reports are summary-only and bounded
raw CSV export and public compatibility are not provided
release/version remains undecided
```

Do not update `CHANGELOG.md`, versioned compatibility history, manifests, or
release notes during implementation.

## 12. Implementation Order

Use one implementation/review checkpoint in this order:

1. Add input JSON representation types, bounds, envelope, and renderer tests.
2. Open CLI policy and app routing; update help and CLI/app tests.
3. Add the header-only fixture and process/filesystem assertions.
4. Add CI/checklist smoke commands and user-facing documentation.
5. Update RFC/roadmap implementation status without closing the lifecycle or
   choosing a release.
6. Run formatting once after all code edits, then run the full gate set.

Do not create intermediate review requests unless implementation exposes a
policy contradiction or changes this handoff's scope.

## 13. Required Verification

```bash
cargo fmt --all --check
cargo fmt --manifest-path tools/matten-report/Cargo.toml --check
cargo fmt --manifest-path tools/matten-migrate/Cargo.toml --check
cargo check --manifest-path tools/matten-report/Cargo.toml
cargo test --manifest-path tools/matten-report/Cargo.toml
cargo clippy --manifest-path tools/matten-report/Cargo.toml --all-targets -- -D warnings
bash tools/matten-report/tests/process-boundary.sh
bash tools/matten-report/tests/module-boundaries.sh
bash tools/matten-report/tests/module-boundaries.sh --self-test
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
bash scripts/check-release-docs.sh
mdbook build docs
git diff --check
```

Also run the complete report-tool smoke matrix from CI/release checklist,
including both new input JSON commands. Remove generated `docs/book` and report
artifacts after verification.

## 14. Acceptance Criteria

```text
[ ] CLI accepts only explicit-file data-readiness input JSON
[ ] app builds report data once before JSON dispatch
[ ] private envelope says schema v0 / private-local / data-readiness / csv
[ ] exact limits object is 12 / 120 / 240 / 12
[ ] render::common is the single owner of all four shared report display limits
[ ] HTML and fixed-demo JSON behavior remains byte-identical after constant relocation
[ ] every user-controlled string/list is bounded with structured metadata
[ ] success and conversion-error snapshots are exact and deterministic
[ ] no raw CSV table or unbounded content is emitted
[ ] finite-number rejection and 12-value tensor preview are reused
[ ] header-only tensor failure preserves absent and existing destinations
[ ] write-time policy is not strengthened beyond current fs::write behavior
[ ] all five fixed-demo JSON outputs and existing process anchors are preserved except reviewed help growth
[ ] module boundaries and 500-line guard remain green
[ ] CI, release checklist, and current user docs cover the feature
[ ] no public API/schema/crate, dependency, version, release, or unrelated scope changed
```

## 15. Review Stop

Handoff acceptance authorizes this one implementation checkpoint. It does not
authorize a release decision. After implementation review and commit, choose
separately whether to prepare a release or close RFC-073 without one.
