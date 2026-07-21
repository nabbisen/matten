# RFC-072 `matten-report` Modularization Implementation Handoff

**Project:** `matten`
**Related RFCs:** RFC-013, RFC-022, RFC-043, RFC-063, RFC-068, RFC-069, RFC-070, RFC-071, RFC-072
**Document kind:** Detailed implementation handoff
**Status:** Accepted; Slice 0 reviewed and committed; Slice 1 nested-group/visibility remediation prepared for rereview; later extraction unauthorized
**Date:** 2026-07-21

---

## 1. Purpose

Translate accepted RFC-072 into small, behavior-preserving implementation units
for the workspace-excluded `tools/matten-report` binary.

This handoff defines the distinct pre-movement process baseline, target module
and test layout, dependency direction, family/format-sized movement units,
review stops, and final gates. Review may authorize those units; this document
does not implement them.

## 2. Authority And Scope

RFC-072 remains the design authority. Implementation must preserve:

```text
workspace-excluded publish=false binary; no src/lib.rs
private or pub(crate) visibility only
the current Cargo dependencies and feature selections
CLI/help/errors/output bytes and process routing
private schema_version 0 JSON ownership
the process baseline before any Rust source movement
the reviewed-above-500-ELOC rule for production and test Rust files
```

## 3. Committed Baseline

Source baseline before RFC-072 implementation:

```text
385b462 Close RFC-070 visualization readiness audit
tools/matten-report/src/main.rs: 5,023 physical lines
59 top-level functions, 10 constants, 45 structs, 4 enums
59 unit tests in one inline module beginning at line 2,641
```

The current owner-committed RFC-072 state is the design authority. Before
implementation, `cargo test --manifest-path tools/matten-report/Cargo.toml`
passes all 59 tests.

## 4. Target Source Layout

Use Rust 2024 and the established `foo.rs` plus `foo/` style:

```text
tools/matten-report/src/
  main.rs                         process entry only
  app.rs                          orchestration and dispatch
  app/tests.rs
  request.rs                      Config, Input, OutputFormat, ReportKind
  cli.rs                          parsing, usage, format policy
  cli/tests.rs
  output.rs                       stdout / explicit-file write
  report.rs                       report-family root
  report/data_readiness.rs
  report/shape_flow.rs
  report/dynamic_readiness.rs
  report/mlprep_standardization.rs
  report/educational_path.rs
  render.rs                       renderer dispatch
  render/common.rs
  render/markdown.rs
  render/markdown/<family>.rs
  render/html.rs
  render/html/document.rs
  render/html/<family>.rs
  render/json.rs
  render/json/model.rs
  render/json/<family>.rs
```

Tests live beside owners through `foo/tests.rs` or smaller modules below
`foo/tests/`. Do not create empty placeholders. Split any proposed test file
that would exceed 500 ELOC.

Shell gates:

```text
tools/matten-report/tests/process-boundary.sh
tools/matten-report/tests/module-boundaries.sh
```

## 5. Types, Errors, And Visibility

`request.rs` owns internal `Config`, `Input`, `OutputFormat`, and `ReportKind`.
`ReportKind` has the five current variants and an `as_str()`-style mapping that
preserves exact CLI identifiers. It must not change quoted error text.

Preserve the existing error split:

```text
CLI/policy: Result<_, String>
application/report/render/I/O: Result<_, Box<dyn Error>>
process stderr prefix: "matten-report error: "
```

Do not introduce a new error enum. No externally public `pub` item, public
facade, `src/lib.rs`, or `[lib]` target is allowed.

## 6. Dependency Direction

Allowed:

```text
main -> app
app -> cli, request, report, render, output
cli -> request
report -> matten, matten-data, matten-mlprep
render -> request, report, serde, serde_json
output -> std
```

Forbidden:

```text
report -> app, cli, render, output
render -> app, cli, output
cli -> app, report, render, output
output -> any crate module or non-std dependency
request -> app, cli, report, render, output
```

`tools/matten-report/tests/module-boundaries.sh` must conservatively reject
these imports/paths, externally public items, `src/lib.rs`, a Cargo `[lib]`
target, or `publish` other than `false`. It must allow `pub(crate)` and print a
direct file/line explanation on failure.

Slice 1 creates this guard before its movement checkpoint. The guard must have a
dependency-free `--self-test` mode (or equivalent temporary-fixture mode) that
proves representative direct, grouped, and fully qualified forbidden paths
fail; an externally public item fails; and `pub(crate)` passes. Use temporary
fixtures outside production source rather than mutating tracked files.

Wire the normal guard and its self-test into CI and the release checklist during
Slice 1. They remain durable gates through structural closure.

### 6.1 Final Call And Data Flow

No universal report enum is introduced. `app` matches the validated input and
`ReportKind`, constructs one family-specific report value, and passes a shared
reference to the selected family/format renderer:

```text
CLI -> Config
app -> input loading
app -> report::<family>::build(...)
app -> render::<format>::<family>(&report_data)
app -> output::write(...)
```

Every renderer consumes report-owned data. A renderer must not call `Table`
selection/schema/numeric conversion, `Tensor` computation, MLPrep computation,
or a report builder. JSON mapping may create private JSON payload values from a
report value because those payloads remain representation-owned.

## 7. Slice 0: Process-Boundary Baseline

Slice 0 is a distinct review and commit checkpoint. It cannot be combined with
Slice 1 or any Rust source movement.

Create `tools/matten-report/tests/process-boundary.sh` with:

```text
#!/usr/bin/env bash and set -euo pipefail
repository root derived from script location
task-specific variables; never HOME or CODEX_HOME
cargo build using tools/matten-report/Cargo.toml
direct execution of the built binary, not cargo run
validated temporary path under target/ or .git-exclude/tmp/
trap-based cleanup of case files
case-specific failure messages
no new Rust dependency
```

Use an explicit repository-local target directory so caller environment or
Cargo configuration cannot make the harness execute a stale binary:

```bash
env TMPDIR="$MATTEN_REPORT_PROCESS_TMP" cargo build \
  --manifest-path tools/matten-report/Cargo.toml \
  --target-dir target/matten-report-process
```

Execute `target/matten-report-process/debug/matten-report` directly. Do not
fall back to Cargo's default target location, and do not honor an unrelated
caller `CARGO_TARGET_DIR` when choosing the executable.

Before the build, derive these task-specific paths from the repository root:

```text
MATTEN_REPORT_PROCESS_TARGET=<repo>/target/matten-report-process
MATTEN_REPORT_PROCESS_TMP=<repo>/target/matten-report-process/tmp
```

Create the temporary directory, resolve/validate that it remains below the
repository `target/` tree, and pass it as `TMPDIR` only for the Cargo build.
Never repurpose `HOME`, `CODEX_HOME`, or a caller-global temporary directory.
This linker-temporary hardening must not change the executable path above.

Observed baseline fingerprints on 2026-07-21:

| Artifact | Bytes | SHA-256 |
|---|---:|---|
| `--help` stdout | 1,509 | `78b2e3342c78847fe3976d2e71da1735d6e70f57ddb9d65f0be52b202f23f342` |
| `--demo data-readiness` stdout | 404 | `bdb6014f637455ed235af7eedcda0872b9161f76e362661bbbbe3fe8247e4c22` |
| fixed-demo data-readiness JSON file | 952 | `6491d3856293572e80f0388be6002703178336447f24afb330087c82ad680fac` |

Embed byte counts and digests in the harness; do not add generated expected
report files.

Required cases:

```text
A: --help
   exit 0; exact 1,509-byte stdout; empty stderr

B: --demo data-readiness --format html
   exit 1; empty stdout
   exact stderr: matten-report error: --format html requires --output <report.html>\n

C: --demo data-readiness
   exit 0; exact 404-byte stdout; empty stderr

D: --demo data-readiness --format json --output <temp>/report.json
   exit 0; empty stdout/stderr; exact 952-byte file

E: --demo data-readiness --output <temp>/missing/report.md
   missing parent directory; nonzero exit; empty stdout
   exactly one newline-terminated stderr line beginning "matten-report error: "
```

Do not freeze the platform-specific filesystem detail. The observed Linux text
is `No such file or directory (os error 2)`.

Wire the harness into `.github/workflows/test.yaml` and
`docs/src/contributing/release-checklist.md` after the local-tool build/tests.
Retain existing smoke commands.

Slice 0 review stop:

```text
[ ] all five cases pass against monolithic main.rs
[ ] a deliberate expected-digest mutation makes the harness fail
[ ] CI and release-checklist wiring exists
[ ] no production Rust source moved
[ ] no output, Cargo, dependency, or public-surface change occurred
```

Commit Slice 0 separately after review.

## 8. Slice 1: Entry, Request, CLI, App, And Output

Begin only after the Slice 0 commit exists.

Move to `request.rs`:

```text
Config
Input
OutputFormat
ReportKind and current report-kind string mappings
```

Move to `cli.rs`:

```text
Action when still needed
parse_args
take_value
parse_select
parse_format
validate_format_policy
validate_html_format_policy
validate_json_format_policy
supports_html_demo / supported_html_demos
supports_json_demo / supported_json_demos
require_kind_or_demo_label
is_supported_demo
usage
```

Move final stdout/file choice to `output.rs`. Move `run`, input loading, and
temporary report dispatch to `app.rs`. Leave `main.rs` only module declarations,
the `app::run` call, stderr prefix, and failure exit.

Before declaring Slice 1 complete, create
`tools/matten-report/tests/module-boundaries.sh`, run its temporary-fixture
self-test, run it against the moved source, and wire both modes into CI and the
release checklist. Representative mutation cases must cover:

```text
report file containing use crate::render -> fail
report file containing use crate::{cli, render} -> fail
report file containing crate::output::write(...) -> fail
source containing pub struct Leaked -> fail
source containing pub(crate) struct Internal -> pass
```

Move parser/policy tests to `cli/tests.rs`, including help, required arguments,
demo/input rejection, format-output requirements, JSON rejection, supported
labels, and readable error tests. Keep dispatch tests in `app/tests.rs`.

Slice 1 gates:

```text
[ ] process harness unchanged and green
[ ] all 59 unit tests retained and green
[ ] exact report snapshots unchanged
[ ] main.rs has no parsing, report construction, rendering, or output-choice logic
[ ] CLI/help/error bytes remain exact
[ ] module-boundary guard passes
[ ] module-boundary guard self-test proves forbidden/public failures and pub(crate) success
[ ] normal guard and self-test are wired into CI/release checklist
[ ] no production or test Rust file above 500 ELOC without justification
```

Slice 1 implementation checkpoint note: `render.rs` and `render/tests.rs`
temporarily preserve the unchanged combined report/render bodies and exact
snapshots moved out of the entry point. They remain above 500 lines so this
checkpoint does not combine entry/CLI/app ownership with the later reviewed
family/format extractions. They must not grow and must be split by those later
units; this is not a permanent size-policy exception.

Stop for review before report-family extraction.

## 9. Phase 2: Report-Family Data Builders

Each family is a separate movement unit and checkpoint. Report modules own
private data and computation, never Markdown/HTML/JSON or final I/O.

Each family unit also changes its existing renderer signatures so `app`
constructs the report value once and renderers borrow it. Renderer bodies may
remain in their old file until Phase 3, but they must stop constructing data in
Phase 2.

### 9.1 Shape Flow

Move `ShapeFlowReportData`, its broadcast/reshape/axis/matmul child structs, and
`shape_flow_report_data`. Require all shape-flow snapshots and the process
harness to pass. Change Markdown/HTML/JSON shape-flow entry points to consume
`&ShapeFlowReportData`; `app` calls the builder before format dispatch.

### 9.2 Dynamic Readiness

Move `DynamicReadinessReportData`, `DynamicValueData`,
`DynamicSchemaSummaryRow`, `dynamic_readiness_report_data`, and
`dynamic_schema_summary_rows`. Require all dynamic-readiness snapshots and the
process harness to pass.

Keep `format_dynamic_element` in `report/dynamic_readiness.rs` as private
normalization used while constructing `DynamicValueData.element`. All renderers
consume that stored string. This avoids `report -> render` and preserves the
single current textual representation. Change Markdown/HTML/JSON entry points
to consume `&DynamicReadinessReportData`; `app` constructs it first.

### 9.3 MLPrep Standardization

Move `MlprepStandardizationReportData` and
`mlprep_standardization_report_data`. Require all MLPrep snapshots and the
process harness to pass. Change all three format entry points to consume
`&MlprepStandardizationReportData`; `app` constructs it first.

### 9.4 Educational Path

Move `EducationalPathReportData`, all `Educational*Data` child structs, and
`educational_path_report_data`. Require all educational-path snapshots and the
process harness to pass. Change all three format entry points to consume
`&EducationalPathReportData`; `app` constructs it first.

### 9.5 Data Readiness

Move last because this family covers fixed and user-controlled input. Replace
the two overlapping private report structs with one report-owned value:

```rust
pub(crate) struct DataReadinessReportData {
    pub(crate) input_label: String,
    pub(crate) source_columns: Vec<String>,
    pub(crate) selected_columns: Vec<String>,
    pub(crate) left_out_columns: Vec<String>,
    pub(crate) missing_counts: Vec<DataReadinessMissingCount>,
    pub(crate) conversion: DataReadinessConversion,
}

pub(crate) enum DataReadinessConversion {
    Success { tensor_shape: Vec<usize>, tensor_values: Vec<f64> },
    Error { message: String },
}
```

This is private behavior-preserving normalization, not a public report model.
The fixed demo produces `Success`; input mode may produce either variant.

Move or replace:

```text
DEMO_CSV
DataReadinessReportData
DataReadinessMissingCount
data_readiness_demo_report_data
InputDataReadinessReportData
InputDataReadinessConversion
input_data_readiness_report_data
left_out_columns
describe_data_error
```

Create one private builder that accepts input label, `&Table`, and selected
columns and performs `select_columns`, schema summary, numeric conversion,
tensor conversion, left-out calculation, and data-error normalization. The demo
builder parses `DEMO_CSV` and delegates to it. Input mode loads the table in
`app` and delegates to it.

Change data-readiness entry points to:

```text
Markdown: render(&DataReadinessReportData)
fixed HTML: render_demo(&DataReadinessReportData)
input HTML: render_input(&DataReadinessReportData)
fixed JSON: payload(&DataReadinessReportData), accepting only Success
```

`render_table_report` must stop querying or converting `Table`; tests that
exercise missing columns, duplicate selections, missing values, and nonnumeric
values call the report builder first, then render when construction succeeds.
Require fixed-demo snapshots, input success/error/bounds/escaping tests,
selection/conversion error tests, and the process harness to pass.

Each family checkpoint occurs before the next family moves. Review may group
only adjacent units whose combined changed Rust source is below 500 lines and
whose snapshot evidence is listed separately.

## 10. Phase 3A: Shared Rendering Helpers

Move only genuinely format-neutral presentation helpers to
`render/common.rs`:

```text
format_fixed_values
format_fixed_value
```

Ownership corrections:

```text
format_dynamic_element -> report/dynamic_readiness.rs
describe_data_error -> report/data_readiness.rs
write_list -> render/markdown/data_readiness.rs
```

HTML escaping/document helpers stay under `render/html`; JSON envelope,
finite-value, and preview helpers stay under `render/json`. Put display limits
in the narrowest owning renderer and document any limit shared by HTML/JSON.

Checkpoint: every exact snapshot and the process harness pass.

## 11. Phase 3B: Markdown Renderer Units

Move one family at a time:

```text
render_table_report -> render/markdown/data_readiness.rs, renamed to consume &DataReadinessReportData
render_shape_flow_report -> render/markdown/shape_flow.rs
render_dynamic_readiness_report -> render/markdown/dynamic_readiness.rs
render_mlprep_standardization_report -> render/markdown/mlprep_standardization.rs
render_educational_path_report -> render/markdown/educational_path.rs
write_list -> render/markdown/data_readiness.rs
```

Move each family's exact Markdown tests beside its renderer. Keep the 404-byte
data-readiness process fingerprint unchanged. Every renderer accepts its
family's report-data reference and performs no report computation. Review after
data readiness and after the remaining fixed-demo units, or earlier if a unit
approaches 500 lines.

## 12. Phase 3C: HTML Renderer Units

First move the document/security helpers:

```text
render_html_document
write_html_document_start / write_html_document_end
write_html_pre
html_escape
write_shape_flow_table
```

Keep escaping tests beside `render/html/document.rs`. Do not change CSS, tag
order, escaping, final newlines, or static/self-contained policy.

Then move family units separately:

```text
render_shape_flow_html_report
render_dynamic_readiness_html_report
render_mlprep_standardization_html_report
render_educational_path_html_report
render_data_readiness_html_report
render_input_data_readiness_html_report
cap_display / format_display_list / format_tensor_preview with input data-readiness HTML
```

Data-readiness fixed/input rendering may share one production module, but split
its tests if they exceed 500 ELOC. Each HTML function receives the matching
report-data reference; none calls a builder. After every family unit, require
its exact HTML snapshot, static/self-contained assertion, applicable
hostile-input escaping/bounds tests, and the process harness.

## 13. Phase 3D: Private JSON Renderer Units

First move to `render/json/model.rs` and `render/json.rs`:

```text
JsonReportEnvelope
JsonTensorPreview
family-specific Json* payload structs
render_json_envelope
json_tensor_preview
ensure_finite_values
render_fixed_demo_json_report dispatch
```

Payload structs remain private to `render::json`; do not expose them from
`report` or create a canonical schema module.

Then move each mapping separately:

```text
data_readiness_json_payload
shape_flow_json_payload
dynamic_readiness_json_payload
mlprep_standardization_json_payload
educational_path_json_payload
```

Each mapping accepts its family's report-data reference and performs only
representation mapping plus JSON-specific finite-value/preview policy. The
JSON dispatcher must not invoke report builders. Move exact JSON tests with
their mappings. The representative data-readiness JSON remains 952 bytes with
the recorded SHA-256. After each family, require its snapshot, determinism,
finite-value policy, input-mode JSON rejection, and process harness.

## 14. Phase 4: Structural Closure

After all ownership moves:

1. Remove forwarding helpers that no longer define ownership.
2. Run the module-boundary guard and its temporary-fixture self-test; inspect
   every `pub(crate)` use.
3. Split any production or test Rust file above 500 ELOC.
4. Record reviewed justification for any unavoidable exception.
5. Remove empty modules and placeholders.
6. Confirm `main.rs` is a thin entry point.
7. Confirm Cargo dependencies/features are unchanged.
8. Confirm all 59 original test names still exist.

Conservative size check:

```bash
find tools/matten-report/src -type f -name '*.rs' -print0 | xargs -0 wc -l
```

Physical lines conservatively bound ELOC. Review files above 300 physical lines
for a useful split even when below 500.

## 15. Formatting And Final Gates

Complete implementation first, run formatting once, then run checks:

```bash
cargo fmt --manifest-path tools/matten-report/Cargo.toml
cargo fmt --manifest-path tools/matten-report/Cargo.toml --check
cargo check --manifest-path tools/matten-report/Cargo.toml
cargo test --manifest-path tools/matten-report/Cargo.toml
cargo clippy --manifest-path tools/matten-report/Cargo.toml -- -D warnings
bash tools/matten-report/tests/process-boundary.sh
bash tools/matten-report/tests/module-boundaries.sh
bash tools/matten-report/tests/module-boundaries.sh --self-test
bash scripts/check-release-docs.sh
git diff --check
```

Also execute the existing report-tool smoke block from CI/release checklist.

Expected final properties:

```text
59 original unit tests retained and passing
5 process cases passing
module-boundary guard and mutation/self-test cases passing
all Markdown/HTML/JSON snapshots byte-identical
no public item or library target
no Cargo dependency or feature change
no generated report artifact checked in
```

## 16. Review And Commit Sequence

Required owner-visible review points:

```text
R0 this handoff
R1 Slice 0 process baseline and wiring
R2 Slice 1 entry/request/CLI/app/output
R3 report-builder family units in reviewed small batches
R4 shared and Markdown renderer units
R5 HTML renderer units
R6 private JSON renderer units
R7 structural closure and full implementation review
```

Do not begin the next unit until the prior checkpoint is observed. Commit
boundaries may match review points. Slice 0 never combines with source movement.

## 17. Explicitly Deferred Work

```text
input-mode JSON or new report families/formats
public report/viz crates, APIs, models, or schemas
library target or workspace membership
dependency/feature changes or CLI framework adoption
error-model redesign or performance optimization
expression tracing, autograd, or Tensor visualization APIs
broader mathematics or ecosystem bridges
version bump, release preparation, tag, or publish action
```

## 18. Review Questions

```text
[ ] Is Slice 0 concrete enough to prove process behavior before movement?
[ ] Are byte counts and SHA-256 anchors suitable exact-output evidence?
[ ] Is filesystem failure preserved without freezing OS-specific text?
[ ] Is the module tree clear without premature abstraction?
[ ] Are request/CLI/app/report/render/output ownership and direction correct?
[ ] Does app construct each family value once before renderer dispatch?
[ ] Does unified private data-readiness data remove Table work from Markdown?
[ ] Are dynamic/data-error normalization correctly report-owned?
[ ] Is write_list correctly Markdown-owned?
[ ] Is the dependency guard conservative and maintainable?
[ ] Are guard delivery, self-test, and CI/release wiring explicit in Slice 1?
[ ] Are builder and renderer movements sufficiently small?
[ ] Are JSON payloads private and renderer-owned?
[ ] Are tests moved beside owners without another test monolith?
[ ] Does the ELOC policy apply symmetrically to production and tests?
[ ] Are review stops and final gates sufficient?
[ ] Does the handoff avoid features, public API, dependencies, and release work?
```
