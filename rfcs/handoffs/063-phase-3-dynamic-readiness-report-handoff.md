# RFC-063 Phase 3 Dynamic-Readiness Report Handoff

**Project:** `matten`  
**Related RFC:** RFC-063: Visual Understanding and Reporting  
**Document kind:** Compact local-tool planning handoff  
**Status:** Implemented and architect-reviewed; prepared for `0.29.0-pre.6`; retained as implementation record  
**Scope:** Local-only `tools/matten-report` dynamic-readiness demo report  

---

## 1. Summary

This handoff defines the next RFC-063 local-tool slice after the
`0.29.0-pre.5` demo-only shape-flow report.

The goal is to make dynamic tensor readiness visible before numeric computation:
what values are present, where missing values are, which cells are numeric under
the default policy, and what an explicit conversion policy changes.

Authorized implementation target:

```text
tools/matten-report --demo dynamic-readiness
```

The report should be deterministic Markdown/plain text, stdout by default, and
optionally written only when `--output` is provided.

This is a fixed demo report, not automatic data profiling, source scanning, or
policy recommendation.

## 2. Scope

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
dynamic-readiness demo report
small hard-coded dynamic tensor
Element::Float, Element::Int, Element::None, and Element::Text values
missing-value mask summary
numeric-convertible mask summary
strict conversion failure
explicit policy conversion success
local helper functions inside tools/matten-report
exact-output tests for the dynamic-readiness Markdown
README, CI, and release-checklist command updates
```

Not authorized in this slice:

```text
input mode for dynamic-readiness
CSV / JSON / tensor-literal parsing for dynamic-readiness
automatic data profiling
automatic cleanup recommendations
data quality score
model-readiness claim
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
matten-mlprep standardization report
shape-flow input mode
arbitrary expression parser
automatic Tensor operation tracing
```

## 3. Planned Files

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

## 4. Design Decisions And Assumptions

### 4.1 Report Kind

Add one new demo label:

```text
dynamic-readiness
```

Accepted command shape:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo dynamic-readiness
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo dynamic-readiness --output target/matten-report-dynamic-readiness.md
```

The first dynamic-readiness implementation should be demo-only. It should not
accept arbitrary dynamic tensor input, CSV, JSON, Rust source files, operation
strings, or cleanup-policy flags.

Do not add:

```text
--input for dynamic-readiness
--kind dynamic-readiness with input files
CSV / JSON ingestion for this report
dynamic tensor literal parsing
policy selection flags
automatic fixes
source file scanning
```

Input mode for dynamic-readiness would require designing a data-input and policy
surface. That is not authorized by this slice.

### 4.2 Local Tool Dependency Policy

This slice may require changing the local tool's `matten` dependency from:

```toml
matten = { path = "../../crates/matten", default-features = false }
```

to:

```toml
matten = { path = "../../crates/matten", default-features = false, features = ["dynamic"] }
```

This dependency change is proposed for review on the following basis:

```text
tools/matten-report is workspace-excluded
tools/matten-report has publish = false
the dependency remains path-only and local
no published crate dependency graph changes
no core matten default feature changes
CI/release gates use --manifest-path for the tool
```

Do not add external dependencies. Do not move the tool into the workspace.

If review rejects enabling the dynamic feature in `tools/matten-report`, this
handoff should be revised before implementation.

The implementation must document this boundary in `tools/matten-report/README.md`:

```text
The local report tool enables matten's dynamic feature only for its own demo
reporting. This does not change core matten defaults or any published crate's
dependency graph.
```

### 4.3 Demo Data

The report should use a small fixed tensor similar to the existing
`dynamic_09_visual_readiness_summary` example:

```text
shape: [2, 3]
values:
  Float(1.0)
  Text("2.5")
  None
  Int(4)
  Text("6.0")
  Float(8.0)
```

Use local formatting helpers for `Element` values and schema summaries whenever
raw `Debug` or `schema_summary` output could be brittle, noisy, or
implementation-shaped. Do not add public formatting APIs.

Preferred stable value rendering:

```text
Float(1.0)
Text("2.5")
None
Int(4)
Text("6.0")
Float(8.0)
```

Preferred stable schema summary:

```text
Float: 2
Int: 1
Text: 2
None: 1
```

### 4.4 Output Contract

Required report heading:

```text
# matten dynamic-readiness report
```

Required sections:

```text
## Input
demo: dynamic-readiness
note: fixed demo report, not automatic data profiling

## Dynamic values
shape: [2, 3]
row-major values: [...]
schema summary: ...

## Readiness masks
none mask: [0.0, 0.0, 1.0, 0.0, 0.0, 0.0]
numeric mask: strict policy readiness [1.0, 0.0, 0.0, 1.0, 0.0, 1.0]
strict numeric-ready: false

## Strict conversion
result: error: ...

## Explicit policy conversion
policy: none_as(0.0) + allow_text_parse()
converted shape: [2, 3]
converted row-major values: [1.0, 2.5, 0.0, 4.0, 6.0, 8.0]
```

The report should explain readiness without judging the data. It may say a value
is missing or not numeric under the strict policy. It must not say the data is
clean, dirty, good, bad, production-ready, model-ready, or recommended for a
specific workflow.

The numeric mask is a strict-policy readiness mask. The explicit policy changes
the conversion result, not the original strict mask.

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
stable policy wording
```

Preferred wording:

```text
dynamic readiness
missing values
numeric mask
strict conversion
explicit policy conversion
```

Avoid:

```text
data quality
clean / dirty
automatic cleanup
profile
recommendation
model-ready
dashboard
visualization framework
```

### 4.6 Implementation Assumptions

The implementation may use current `matten` APIs:

```text
Tensor::from_elements
Tensor::shape
Tensor::to_elements
Tensor::schema_summary
Tensor::none_mask
Tensor::numeric_mask
Tensor::is_numeric_convertible
Tensor::try_numeric
Tensor::try_numeric_with
NumericPolicy::default().none_as(0.0).allow_text_parse()
```

If using an API makes the report code awkward, prefer small local formatting
helpers over adding public APIs. Do not change core `matten` for this slice.

## 5. Required Tests And Gates

Observed implementation gates on 2026-07-04:

```text
cargo fmt --all --check
cargo fmt --check --manifest-path tools/matten-report/Cargo.toml
cargo check --manifest-path tools/matten-report/Cargo.toml
cargo test --manifest-path tools/matten-report/Cargo.toml
cargo clippy --manifest-path tools/matten-report/Cargo.toml -- -D warnings
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo dynamic-readiness
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo dynamic-readiness --output target/matten-report-dynamic-readiness.md
bash scripts/check-release-docs.sh
git diff --check
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo shape-flow
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
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo dynamic-readiness
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo dynamic-readiness --output target/matten-report-dynamic-readiness.md
bash scripts/check-release-docs.sh
git diff --check
```

If the implementation changes the local tool dependency features, also run:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo shape-flow
cargo check --workspace --all-features
```

Required exact-output tests:

```text
dynamic_readiness_report_matches_expected_markdown
demo_dynamic_readiness_allows_output
dynamic_readiness_input_mode_is_not_supported
data_readiness_report_still_matches_expected_markdown
shape_flow_report_still_matches_expected_markdown
```

## 6. Generated Artifacts

Allowed generated artifact during local verification:

```text
target/matten-report-dynamic-readiness.md
```

No generated report artifact should be committed.

The tool must not create files unless `--output` is provided.

## 7. Acceptance Checklist

```text
[x] dynamic-readiness is demo-only
[x] no --input mode for dynamic-readiness
[x] no policy flags
[x] exact-output test covers dynamic-readiness Markdown
[x] data-readiness exact-output tests still pass
[x] shape-flow exact-output tests still pass
[x] tool README clarifies this is fixed demo reporting, not data profiling
[x] local tool enables matten/dynamic only for itself
[x] published crate dependency graph is unchanged
[x] schema/Element rendering is stable and exact-output-test friendly
[x] numeric mask is clearly strict-policy readiness
[x] explicit policy conversion succeeds with the fixed demo values
[x] no generated dynamic-readiness report is committed
[x] no public API, workspace-membership, external-dependency, or output-format expansion
[x] any local matten dynamic-feature dependency is documented and remains tool-only
```

## 8. Known Limitations

This slice intentionally does not explain arbitrary user data.

It does not parse CSV, JSON, Rust code, or tensor literals for dynamic-readiness.
It only provides one deterministic, reviewed demo that explains how dynamic
values, masks, strict conversion, and explicit policy conversion relate.

Future report families remain deferred:

```text
matten-mlprep standardization report
integration with migration reports
HTML / SVG / Vega-Lite / JSON output
public report APIs
input modes for shape-flow or dynamic-readiness
```

## 9. Recommended Next Step

Retain this file as the implementation record for the `--demo dynamic-readiness`
local-tool slice prepared for `0.29.0-pre.6`.
