# RFC-063 Phase 3 MLPrep Standardization Report Handoff

**Project:** `matten`  
**Related RFC:** RFC-063: Visual Understanding and Reporting  
**Document kind:** Compact local-tool planning handoff  
**Status:** Implemented and architect-reviewed; shipped in `0.29.0-pre.7` and included in `0.29.0`; retained as implementation record
**Scope:** Local-only `tools/matten-report` mlprep-standardization demo report  

---

## 1. Summary

This handoff defines the next RFC-063 local-tool slice after the
`0.29.0-pre.6` demo-only dynamic-readiness report.

The goal is to make preprocessing effects visible before users hand numeric
tensors to later experiments: what `standardize_columns` changes, what stays the
same, and how per-column mean/std move from the original values to the
standardized values.

Authorized implementation target:

```text
tools/matten-report --demo mlprep-standardization
```

The report should be deterministic Markdown/plain text, stdout by default, and
optionally written only when `--output` is provided.

This is a fixed demo report, not model-quality analysis, data-quality scoring,
training guidance, automatic preprocessing selection, or automatic data
profiling.

## 2. Scope Followed

This handoff follows RFC-063's accepted local-tool boundaries:

```text
local-only report tool
publish = false
workspace-excluded
Markdown/plain text output
existing workspace crates only
std::env::args parsing only
explicit output path or stdout
no public API commitment
```

Allowed for this slice:

```text
mlprep-standardization demo report
small hard-coded numeric tensor
standardize_columns(input)
before/after shape summary
before/after row-major value preview
before/after per-column mean
before/after per-column population standard deviation
local helper functions inside tools/matten-report
exact-output tests for the mlprep-standardization Markdown
README, CI, and release-checklist command updates
```

Not authorized in this slice:

```text
input mode for mlprep-standardization
CSV / JSON / tensor-literal parsing for mlprep-standardization
automatic preprocessing recommendation
automatic cleanup
data quality score
model quality score
model-readiness claim
training loop
train/test split report
min-max scaling report
bias-column report
project scanning
source-file inspection
public report API
published crate
workspace membership
new external dependencies
SVG output
HTML output
Vega-Lite JSON
JSON output
images
ANSI color
terminal-width-dependent layout
notebook integration
GUI / dashboard
network access
telemetry
automatic project mutation
shape-flow input mode
dynamic-readiness input mode
```

## 3. Files Changed

Planned implementation files:

```text
tools/matten-report/Cargo.toml
tools/matten-report/src/main.rs
tools/matten-report/README.md
.github/workflows/test.yaml
docs/src/contributing/release-checklist.md
```

Optional documentation files if useful:

```text
rfcs/handoffs/README.md
rfcs/done/063-visual-understanding-and-reporting.md
```

Version files, changelog, and release notes remain out of scope until a later
release-prep step is explicitly requested.

The implementation must document this local dependency boundary in
`tools/matten-report/README.md`:

```text
The local report tool depends on matten-mlprep only for its own fixed demo
report. This does not change any published crate dependency graph or core
matten defaults.
```

## 4. Design Decisions And Assumptions

### 4.1 Report Kind

Add one new demo label:

```text
mlprep-standardization
```

Accepted command shape:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo mlprep-standardization
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo mlprep-standardization --output target/matten-report-mlprep-standardization.md
```

The first mlprep-standardization implementation should be demo-only. It should
not accept arbitrary tensor input, CSV files, JSON files, Rust source files,
operation strings, preprocessing options, or policy flags.

Do not add:

```text
--input for mlprep-standardization
--kind mlprep-standardization with input files
CSV / JSON ingestion for this report
dynamic tensor literal parsing
preprocessing selection flags
source file scanning
```

Input mode for mlprep-standardization would require designing a data-input and
preprocessing-policy surface. That is not authorized by this slice.

### 4.2 Local Tool Dependency Policy

This slice may require adding a local path-only dependency to the publish-false
tool:

```toml
matten-mlprep = { path = "../../crates/matten-mlprep" }
```

This dependency change is proposed for review on the following basis:

```text
tools/matten-report is workspace-excluded
tools/matten-report has publish = false
the dependency remains path-only and local
no published crate dependency graph changes
no core matten default feature changes
CI/release gates use --manifest-path for the tool
matten-mlprep already belongs to the workspace family
```

Do not add external dependencies. Do not move the tool into the workspace. Do
not enable `matten-mlprep` optional features unless implementation proves they
are required; this report should use numeric tensors only.

If review rejects adding `matten-mlprep` as a local tool dependency, this
handoff should be revised before implementation.

### 4.3 Demo Data

The report should use a small fixed tensor aligned with the existing
`mlprep_visual_standardize_summary` example:

```text
shape: [3, 2]
row-major values:
  8.0, 80.0
  10.0, 100.0
  12.0, 120.0
```

Interpretation:

```text
rows    = samples
columns = features
```

Operation:

```text
standardize_columns(input)
```

The report should show that standardization changes values, not shape:

```text
shape flow: [3, 2] -> [3, 2]
```

Use stable local numeric formatting helpers. Preferred precision is three
decimal places for report values and column statistics, with tiny absolute
values rendered as `0.000` to avoid negative-zero output. The implementation
should use a local rule equivalent to:

```text
if abs(value) < 0.0005, render as 0.000
```

Preferred fixed statistics:

```text
before mean: [10.000, 100.000]
before std:  [1.633, 16.330]
after mean:  [0.000, 0.000]
after std:   [1.000, 1.000]
```

Preferred standardized row-major values:

```text
[-1.225, -1.225, 0.000, 0.000, 1.225, 1.225]
```

### 4.4 Output Contract

Required report heading:

```text
# matten mlprep-standardization report
```

Required sections:

```text
## Input
demo: mlprep-standardization
note: fixed demo report, not automatic model-quality analysis

## Operation
operation: standardize_columns(input)
meaning: each column is centered to mean 0 and population standard deviation 1

## Before
shape: [3, 2]
row-major values: [8.000, 80.000, 10.000, 100.000, 12.000, 120.000]
column mean: [10.000, 100.000]
column population std: [1.633, 16.330]

## After
shape: [3, 2]
row-major values: [-1.225, -1.225, 0.000, 0.000, 1.225, 1.225]
column mean: [0.000, 0.000]
column population std: [1.000, 1.000]

## Shape meaning
shape flow: [3, 2] -> [3, 2]
rows: samples unchanged
columns: features unchanged
```

The report should explain the preprocessing effect without judging the data. It
may say values changed, shape stayed the same, and columns now have mean 0 and
population standard deviation 1. It must label standard deviation as population
standard deviation or population std at least once. It must not say the data is
clean, dirty, good, bad, production-ready, model-ready, or recommended for a
specific workflow.

### 4.5 Output Rules

Hard output rules:

```text
no timestamps
no random data
no absolute paths in demo output
no ANSI color
no Unicode block charts
no terminal-width-dependent wrapping
no unordered map iteration in output
stable section order
stable value order
stable numeric precision
stable wording
```

Preferred wording:

```text
standardization
before
after
column mean
population std
shape flow
samples
features
```

Avoid:

```text
data quality
clean / dirty
automatic cleanup
profile
recommendation
model-ready
training-ready
dashboard
visualization framework
```

### 4.6 Implementation Assumptions

The implementation may use current APIs:

```text
Tensor::new
Tensor::shape
Tensor::as_slice
Tensor::mean_axis(0)
Tensor::std_axis(0)
matten_mlprep::standardize_columns
```

If using an API makes the report code awkward, prefer small local formatting
helpers over adding public APIs. Do not change core `matten` or
`matten-mlprep` for this slice.

## 5. Tests And Gates Run

Observed implementation gates on 2026-07-04:

```text
cargo fmt --all --check
cargo fmt --check --manifest-path tools/matten-report/Cargo.toml
cargo check --manifest-path tools/matten-report/Cargo.toml
cargo test --manifest-path tools/matten-report/Cargo.toml
cargo clippy --manifest-path tools/matten-report/Cargo.toml -- -D warnings
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo mlprep-standardization
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo mlprep-standardization --output target/matten-report-mlprep-standardization.md
bash scripts/check-release-docs.sh
git diff --check
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo shape-flow
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo dynamic-readiness
cargo check --workspace --all-features
```

All commands above completed successfully in the implementation thread.

Required implementation checks before review:

```bash
cargo fmt --all --check
cargo fmt --check --manifest-path tools/matten-report/Cargo.toml
cargo check --manifest-path tools/matten-report/Cargo.toml
cargo test --manifest-path tools/matten-report/Cargo.toml
cargo clippy --manifest-path tools/matten-report/Cargo.toml -- -D warnings
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo mlprep-standardization
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo mlprep-standardization --output target/matten-report-mlprep-standardization.md
bash scripts/check-release-docs.sh
git diff --check
```

If the implementation changes local tool dependencies, also run:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo shape-flow
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo dynamic-readiness
cargo check --workspace --all-features
```

Required exact-output tests:

```text
mlprep_standardization_report_matches_expected_markdown
demo_mlprep_standardization_allows_output
mlprep_standardization_input_mode_is_not_supported
data_readiness_report_still_matches_expected_markdown
shape_flow_report_still_matches_expected_markdown
dynamic_readiness_report_matches_expected_markdown
```

## 6. Generated Artifacts

Allowed generated artifact during local verification:

```text
target/matten-report-mlprep-standardization.md
```

No generated report artifact should be committed.

The tool must not create files unless `--output` is provided.

## 7. Known Limitations

This slice intentionally does not explain arbitrary user data.

It does not parse CSV, JSON, Rust code, or tensor literals for
mlprep-standardization. It only provides one deterministic, reviewed demo that
explains how `standardize_columns` changes values while preserving shape.

Future report families remain deferred:

```text
mlprep min-max scaling report
mlprep bias-column report
mlprep train/test split report
input modes for shape-flow, dynamic-readiness, or mlprep-standardization
integration with migration reports
HTML / SVG / Vega-Lite / JSON output
public report APIs
```

## 8. Recommended Next Step

Retain this file as the implementation record for the `--demo
mlprep-standardization` local-tool slice shipped in `0.29.0-pre.7` and included
in `0.29.0`.

## Acceptance Checklist

```text
[x] mlprep-standardization is demo-only
[x] no --input mode for mlprep-standardization
[x] no preprocessing flags
[x] exact-output test covers mlprep-standardization Markdown
[x] data-readiness exact-output tests still pass
[x] shape-flow exact-output tests still pass
[x] dynamic-readiness exact-output tests still pass
[x] tool README clarifies this is fixed demo reporting, not model-quality analysis
[x] local tool uses matten-mlprep only for itself
[x] published crate dependency graph is unchanged
[x] report labels std as population std at least once
[x] negative zero is normalized in output
[x] numeric rendering is stable and exact-output-test friendly
[x] report explains standardization as value change with unchanged shape
[x] no generated mlprep-standardization report is committed
[x] no public API, workspace-membership, external-dependency, or output-format expansion
[x] any local matten-mlprep dependency is documented and remains tool-only
```
