# RFC-065 Phase 3 Educational Path Report Handoff

**Project:** `matten`  
**Related RFC:** RFC-065: Educational Visualization and Tensor Learning Path  
**Document kind:** Compact local-tool implementation handoff  
**Status:** Implemented and reviewed; retained as the RFC-065 Phase 3 record
**Scope:** Local-only fixed educational-path demo report in `tools/matten-report`  

---

## 1. Summary

This handoff defines the next RFC-065 slice after the educational shape/data docs
path.

The goal is to provide one local, deterministic Markdown/plain-text report that
turns the accepted educational path into an inspectable artifact. This helps
users see the same concepts from the command line without creating a public
visualization API, expression tracer, plotting backend, or published crate.

Authorized implementation target:

```text
tools/matten-report --demo educational-path
```

The report must be a fixed demo report, not an expression parser and not a
project analyzer.

## 2. Reviewer Background

RFC-063 added `tools/matten-report` as a workspace-excluded, `publish = false`
local reporting tool. Existing demos cover:

```text
data-readiness
shape-flow
dynamic-readiness
mlprep-standardization
```

RFC-065 then clarified the educational value of shape/data visual understanding.
Its first slice audited public positioning and added the overclaim guard. Its
second slice added worked Markdown explanations for:

```text
broadcasting shape alignment
reshape / flatten / transpose
axis reductions
matmul shape flow
dynamic readiness
standardization before/after
```

This third slice should test whether that educational path is useful as a local
report artifact. It must stay smaller than a new visualization product.

## 3. Differentiation From Existing Demos

This demo deliberately overlaps existing report families, but it must not be a
loose concatenation of them.

Existing demos are atomic:

```text
shape-flow              individual shape operations
dynamic-readiness       dynamic masks and conversion readiness
mlprep-standardization  preprocessing before/after values
```

`educational-path` is a consolidated guided walkthrough:

```text
read shapes first
then inspect how each operation changes shape or readiness
then decide when data is ready for numeric computation
```

The incremental value is:

```text
a single learner-oriented route across the accepted concepts
the "read the output shape before reading values" framing
a compact transpose explanation alongside reshape
links between numeric shape questions and dynamic-readiness questions
```

The implementation reviewer should reject an implementation that merely copies
the existing demos without this guided narrative.

## 4. Reuse Requirement

Implementation should reuse the already-verified values, shapes, and vocabulary
from existing demos and docs wherever possible.

Preferred sources:

```text
tools/matten-report shape-flow demo: broadcasting, reshape, axis reductions, matmul
tools/matten-report dynamic-readiness demo: none_mask, numeric_mask, strict policy readiness
tools/matten-report mlprep-standardization demo: standardization values, if values are printed
docs/src/examples/visual-understanding.md: educational wording and transpose explanation
```

Do not independently invent a second set of values for the same concept unless
there is a clear pedagogical reason and the new values are covered by stable
expected output. This avoids several exact-output tests drifting apart.

If the educational-path report shows `Element::text(...)`, label readiness
consistently with the existing strict policy: parseable text is still not
numeric-ready under `numeric_mask()` unless an explicit conversion policy later
allows it.

## 5. Scope

Allowed:

```text
one new fixed demo label: educational-path
deterministic Markdown/plain text output
stdout by default
explicit --output file behavior reused from existing tool
small hard-coded tensors / values
existing workspace crates only
local helper functions inside tools/matten-report
exact-output or stable-substring tests for the new report
README update for the new demo
release-checklist / CI command updates only if existing local-tool pattern requires them
```

Not authorized:

```text
Tensor::plot / Tensor::show / Tensor::trace / Tensor::backward
automatic expression tracing
lazy expression graph
autograd
public report API
public matten-report crate
public matten-viz crate
workspace membership
new dependencies
input mode for educational-path
operation-string parser
tensor-literal parser
source file scanning
project scanning
project mutation
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
version bump
```

## 6. Command Shape

Accepted commands:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo educational-path
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo educational-path --output target/matten-report-educational-path.md
```

The existing `--kind` compatibility behavior for demo labels may be preserved:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo educational-path --kind educational-path
```

Do not accept:

```text
--input ... --kind educational-path
--select ... with --demo educational-path
```

## 7. Output Contract

Required heading:

```text
# matten educational-path report
```

Required input section:

```text
## Input
demo: educational-path
note: fixed educational demo report, not automatic expression tracing
```

Required concept sections:

```text
## How to read shapes first
## Broadcasting
## Reshape and transpose
## Axis reductions
## Matrix multiplication
## Dynamic readiness
## Standardization
## What this report is not
```

The report should include small concrete values only where they make the concept
checkable. It should prefer shape/data meaning over long value dumps.

## 8. Required Content

### 8.1 How to read shapes first

Introduce the report as an educational route:

```text
1. ask what shape each input has
2. ask which axes align, disappear, or remain
3. read the output shape before reading values
4. convert dynamic data before numeric computation
```

### 8.2 Broadcasting

Use the same verified shape pattern from the docs:

```text
[3, 1] + [1, 4] -> [3, 4]
```

Show which axis expands for each input. If values are shown, keep the small
`[1; 2; 3] + [10 20 30 40]` example or an equally small equivalent.

### 8.3 Reshape and transpose

Explain:

```text
reshape / flatten preserve the row-major tape and change grouping
transpose changes coordinate meaning
```

Use one compact `[2, 3]` input and avoid implying views or borrowed storage.

### 8.4 Axis reductions

Use:

```text
mean_axis(0): [rows, columns] -> [columns]
mean_axis(1): [rows, columns] -> [rows]
```

Do not use `Phase 1` / `Phase 2` wording in user-facing output.

### 8.5 Matrix multiplication

Use:

```text
[m, n] x [n, p] -> [m, p]
[2, 3] x [3, 4] -> [2, 4]
```

Make the shared inner dimension visible.

### 8.6 Dynamic readiness

Use existing dynamic vocabulary:

```text
Element::Float
Element::Int
Element::None
Element::text(...)
none_mask()
numeric_mask()
try_numeric()
```

State clearly that arithmetic, reductions, slicing, reshape, and matmul belong
after `try_numeric()`.

Use the same strict policy labeling as the existing dynamic-readiness demo:
`Text("2.5")` is still not numeric-ready under `numeric_mask()` even though a
later explicit conversion policy may parse it.

### 8.7 Standardization

For standardization, either:

```text
link conceptually to the existing mlprep-standardization demo
```

or compute a tiny fixed example using existing `matten-mlprep`. If numeric
values are printed, they must be verified by tests or stable expected output.
Do not make model-quality or machine-learning claims.

## 9. Files

Planned implementation files:

```text
tools/matten-report/src/main.rs
tools/matten-report/README.md
```

Optional process/docs files if needed:

```text
docs/src/examples/visual-understanding.md
docs/src/contributing/release-checklist.md
.github/workflows/test.yaml
rfcs/handoffs/README.md
```

Do not edit version files, changelog, Cargo manifests, or published crate source
unless a reviewer explicitly changes the scope.

When implementing the new label, update the supported-demo registry, usage text,
and unsupported-demo error list together so help and error output stay readable.

## 10. Output Rules

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

Avoid wording:

```text
expression graph
trace
backward
autograd
plot
dashboard
production performance
faster than
business-critical
production-scale
```

The report may include "not automatic expression tracing" only in the explicit
non-goal note.

## 11. Acceptance Criteria

```text
[ ] `--demo educational-path` renders the required report
[ ] `--demo educational-path --output <path>` writes only to the requested path
[ ] `--kind educational-path` is accepted with the matching demo label if existing demo compatibility is preserved
[ ] `--input ... --kind educational-path` is rejected
[ ] `--select` with the educational-path demo is rejected
[ ] report output is deterministic
[ ] report output contains no timestamps, absolute paths, ANSI color, generated images, JSON, SVG, or HTML
[ ] report output contains no `Phase 1` / `Phase 2` wording
[ ] report is a consolidated guided walkthrough, not a loose concatenation of existing demos
[ ] report reuses already-verified shapes/values/vocabulary where concepts overlap existing demos
[ ] dynamic readiness wording matches the existing strict policy readiness semantics
[ ] tests cover the new demo label and output contract
[ ] tests cover input/select rejection for the new demo label
[ ] README documents the new demo and its local-only scope
[ ] supported-demo registry, usage text, and unsupported-demo error list are updated together
[ ] no public API is added
[ ] no dependency is added
[ ] no public crate is added
[ ] no version bump is made
[ ] release-doc guard passes
[ ] local matten-report tests pass
```

## 12. Suggested Checks

Run:

```bash
bash scripts/check-release-docs.sh
cargo test --manifest-path tools/matten-report/Cargo.toml
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo educational-path
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo educational-path --output target/matten-report-educational-path.md
git diff --check
```

If mdBook docs are touched, also run:

```bash
mdbook build docs
```

If `mdbook build docs` generates `docs/book`, remove it before requesting review.

## 13. Review Focus

Ask the reviewer to check:

```text
whether the report is educational rather than a product surface
whether the report adds the guided read-shapes-first framing and transpose explanation
whether overlapping values/wording trace to existing verified demos
whether the output matches actual matten shape/data semantics
whether the command rejects unauthorized input/parser modes
whether scope remains local-tool only
whether no dependency/API/public-crate boundary is crossed
whether wording avoids plotting, tracing, autograd, performance, and production-scale claims
```
