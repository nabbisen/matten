# RFC-063 Phase 3 Shape-Flow Report Handoff

**Project:** `matten`  
**Related RFC:** RFC-063: Visual Understanding and Reporting  
**Document kind:** Compact local-tool planning handoff  
**Status:** Implemented and architect-reviewed; retained as implementation record  
**Scope:** Local-only `tools/matten-report` shape-flow demo report  

---

## 1. Summary

This handoff defines the next RFC-063 local-tool slice after the
`0.29.0-pre.4` `matten-data` readiness report.

The goal is to make common shape transformations visible for small shape and
axis operations without adding expression tracing, plotting, generated images,
public APIs, or published-crate dependencies.

Authorized implementation target:

```text
tools/matten-report --demo shape-flow
```

The report should be deterministic Markdown/plain text, stdout by default, and
optionally written only when `--output` is provided.

This is a fixed demo report, not automatic expression tracing.

## 2. Scope Followed

This handoff follows RFC-063's accepted boundaries:

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
shape-flow demo report
small hard-coded tensors and operations
shape explanations for broadcasting, reshape, reductions, and matmul
local helper functions inside tools/matten-report
exact-output tests for the shape-flow Markdown
README, CI, and release-checklist command updates
```

Not authorized in this slice:

```text
arbitrary expression parser
automatic Tensor operation tracing
lazy expression graph
autograd / backward
Tensor::plot
Tensor::show
public report API
published crate
workspace membership
new dependencies
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
project scanning
project mutation
dynamic readiness report
matten-mlprep standardization report
```

## 3. Files Changed

Planned implementation files:

```text
tools/matten-report/src/main.rs
tools/matten-report/README.md
.github/workflows/test.yaml
docs/src/contributing/release-checklist.md
```

Optional documentation files if useful:

```text
rfcs/proposed/063-visual-understanding-and-reporting.md
rfcs/handoffs/README.md
```

Version files, changelog, and release notes remain out of scope until a later
release-prep step is explicitly requested.

Use the actual workflow path in this repository. At the time this handoff was
accepted, that path is:

```text
.github/workflows/test.yaml
```

## 4. Design Decisions And Assumptions

### 4.1 Report Kind

Add one new demo label:

```text
shape-flow
```

Accepted command shape:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo shape-flow
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo shape-flow --output target/matten-report-shape-flow.md
```

The first shape-flow implementation should be demo-only. It should not accept
arbitrary tensor input, operation strings, expression files, or source scanning.

Do not add:

```text
--input for shape-flow
--kind shape-flow with input files
operation string parsing
tensor-literal parsing
source file scanning
```

Input mode for shape-flow would require designing a mini expression or shape
language, which is not authorized by this slice.

### 4.2 Output Contract

Required report heading:

```text
# matten shape-flow report
```

Required sections:

```text
## Input
demo: shape-flow
note: fixed demo report, not automatic expression tracing

## Broadcast add
input a: shape [2, 3]
input b: shape [3]
operation: a + b
shape flow: [2, 3] + [3] -> [2, 3]

## Reshape
input: shape [2, 3]
operation: reshape([3, 2])
shape flow: [2, 3] -> [3, 2]

## Axis reductions
input: shape [2, 3]
mean_axis(0): [2, 3] -> [3]
mean_axis(1): [2, 3] -> [2]

## Matrix multiplication
left: shape [2, 3]
right: shape [3, 2]
operation: left.matmul(right)
shape flow: [2, 3] @ [3, 2] -> [2, 2]
```

The report may include row-major values only when they make the shape flow
clearer and remain small. Values are secondary; the report contract is about
shape and axis meaning.

### 4.3 Output Rules

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
stable operation order
```

Preferred wording:

```text
shape flow
axis reduction
broadcast add
matrix multiplication
```

Avoid:

```text
expression graph
trace
backward
plot
show
dashboard
visualization framework
```

### 4.4 Implementation Assumptions

The implementation may use current `matten` APIs to prove the reported shapes:

```text
Tensor::from_rows or equivalent construction helpers
Tensor::reshape
Tensor::mean_axis
Tensor::matmul
Tensor::shape
```

If using an API makes the report code awkward, prefer small local shape-summary
helpers over adding public APIs. Do not change core `matten` for this slice.

## 5. Tests And Gates Run

Observed implementation checks:

```bash
cargo fmt --all --check
cargo fmt --check --manifest-path tools/matten-report/Cargo.toml
cargo check --manifest-path tools/matten-report/Cargo.toml
cargo test --manifest-path tools/matten-report/Cargo.toml
cargo clippy --manifest-path tools/matten-report/Cargo.toml -- -D warnings
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo shape-flow
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo shape-flow --output target/matten-report-shape-flow.md
bash scripts/check-release-docs.sh
git diff --check
```

Additional observed smoke commands:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness --output target/matten-report-demo.md
cargo run --manifest-path tools/matten-report/Cargo.toml -- --input tools/matten-report/fixtures/small.csv --kind data-readiness --select sales,cost
```

Required exact-output tests:

```text
shape_flow_report_matches_expected_markdown
demo_shape_flow_allows_output
unsupported_demo_label_remains_readable
data_readiness_report_still_matches_expected_markdown
```

If the implementation touches CI or docs, run the relevant broader gates before
requesting review.

## 6. Generated Artifacts

Allowed generated artifact during local verification:

```text
target/matten-report-shape-flow.md
```

No generated report artifact should be committed.

The tool must not create files unless `--output` is provided.

## 7. Acceptance Checklist

```text
[x] shape-flow is demo-only
[x] no --input mode for shape-flow
[x] exact-output test covers shape-flow Markdown
[x] data-readiness exact-output tests still pass
[x] workflow path matches the actual repo workflow file
[x] output states or README clarifies this is not expression tracing
[x] no generated shape-flow report is committed
[x] no public API, dependency, workspace-membership, or output-format expansion
```

## 8. Known Limitations

This slice intentionally does not explain arbitrary user expressions.

It does not parse Rust code, record operator history, or infer shapes from source
files. It only provides one deterministic, reviewed demo that explains common
shape and axis transformations.

Future report families remain deferred:

```text
dynamic readiness report
matten-mlprep standardization report
integration with migration reports
HTML / SVG / Vega-Lite / JSON output
public report APIs
```

## 9. Recommended Next Step

Retain this file as the implementation record for the `--demo shape-flow` local-tool slice
prepared for `0.29.0-pre.5`.
