# RFC-068 Shape-Flow HTML Artifact Handoff

**Project:** `matten`
**Related RFC:** RFC-068: Rich Local Visualization Artifacts
**Document kind:** Compact local-tool implementation handoff
**Status:** Implemented and reviewed; shipped in 0.32.0
**Scope:** Local-only static HTML artifact for `tools/matten-report --demo shape-flow`

---

## 1. Summary

Implement the next RFC-068 feature slice by adding one static HTML output mode
to the existing local `tools/matten-report` shape-flow demo.

Accepted command:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo shape-flow --format html --output target/matten-report-shape-flow.html
```

The existing Markdown/plain-text output remains the default:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo shape-flow
```

This slice extends HTML to exactly one additional fixed report family. It must
not add HTML for data-readiness, dynamic-readiness, mlprep-standardization, or
input-mode reports.

---

## 2. Reviewer Background

RFC-068 Phase 1 proved one static local HTML artifact for the educational-path
report. The follow-up shared-data refactor removed cross-format drift risk for
that report and added exact HTML snapshot coverage.

The next smallest useful visualization step is shape-flow HTML because the
existing `shape-flow` demo is fixed, small, and explicitly about reading shape
transformations:

```text
broadcast add
reshape
axis reductions
matrix multiplication
```

This handoff does not reopen public visualization APIs or richer renderer
formats. It only extends the local static HTML pattern to one already-reviewed
fixed demo.

---

## 3. Implementation Scope

Allowed:

```text
HTML output for --demo shape-flow only
explicit --output required for shape-flow HTML
Markdown/plain text remains default
private shape-flow data builder if useful
small embedded CSS using the existing local HTML style pattern
semantic headings, tables, code blocks, and shape/value rows
exact Markdown output test remains passing
exact HTML output snapshot test for shape-flow
HTML static/self-contained safety test for shape-flow
README update for shape-flow HTML
release-checklist and CI smoke command for shape-flow HTML
RFC/handoff/roadmap status updates
```

Implementation note:

```text
The current HTML policy is educational-path-only. Update validate_format_policy(),
the mirrored render_report() HTML guard, their user-facing error message, and
the existing html_format_is_limited_to_educational_path_demo test. The rejection
test must switch from shape-flow to a still-unsupported family such as
dynamic-readiness or mlprep-standardization, and the error should name both
accepted HTML demos.
```

Not authorized:

```text
HTML for data-readiness
HTML for dynamic-readiness
HTML for mlprep-standardization
input-mode HTML
Tensor::plot / Tensor::show / Tensor::trace / Tensor::backward
automatic expression tracing
lazy expression graph
autograd
public report API
public matten-report crate
public matten-viz crate
workspace membership
new dependencies
operation-string parser
tensor-literal parser
source file scanning
project scanning
project mutation
SVG output
Vega-Lite JSON
JSON output
images
data URLs
JavaScript
external CSS / fonts / assets
network access
telemetry
version bump
tag or publish action
```

---

## 4. Command Behavior

Accepted:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo shape-flow
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo shape-flow --format markdown
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo shape-flow --output target/matten-report-shape-flow.md
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo shape-flow --format html --output target/matten-report-shape-flow.html
```

Rejected:

```text
--demo shape-flow --format html without --output
--format html for data-readiness
--format html for dynamic-readiness
--format html for mlprep-standardization
--format html for input mode
unknown format labels
```

Ordinary CLI misuse must return a readable error and must not panic.

---

## 5. Output Requirements

The HTML report must include:

```text
<!doctype html>
<html lang="en">
<meta charset="utf-8">
title containing "matten shape-flow report"
top-level heading for matten shape-flow report
the same conceptual sections as the Markdown shape-flow demo
embedded CSS only
no script tags
no external href/src references
```

The visual layout should improve scanning with:

```text
shape-flow tables for input/result shapes
operation labels
pre/code blocks for result values
short note preserving "not automatic expression tracing"
```

The implementation should stay modest and deterministic. Do not introduce a
general renderer abstraction unless it removes obvious duplication with the
existing educational-path HTML helpers without changing behavior.

---

## 6. Suggested Files

Expected implementation files:

```text
tools/matten-report/src/main.rs
tools/matten-report/README.md
docs/src/contributing/release-checklist.md
.github/workflows/test.yaml
```

Expected tracking files:

```text
rfcs/proposed/068-rich-local-visualization-artifacts.md
rfcs/handoffs/068-shape-flow-html-artifact-handoff.md
rfcs/handoffs/README.md
rfcs/README.md
ROADMAP.md
```

No version files, changelog, release notes, tags, or publish actions are in
scope for this handoff.

---

## 7. Verification

Minimum implementation verification:

```bash
cargo fmt --all --check
cargo fmt --manifest-path tools/matten-report/Cargo.toml --check
cargo fmt --manifest-path tools/matten-migrate/Cargo.toml --check
cargo check --manifest-path tools/matten-report/Cargo.toml
cargo test --manifest-path tools/matten-report/Cargo.toml
cargo clippy --manifest-path tools/matten-report/Cargo.toml -- -D warnings
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo shape-flow
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo shape-flow --format html --output target/matten-report-shape-flow.html
bash scripts/check-release-docs.sh
git diff --check
```

If documentation under `docs/src/` changes, also run:

```bash
mdbook build docs
```

Remove generated `docs/book` afterward if created.

---

## 8. Acceptance Criteria

```text
[ ] Markdown/plain text shape-flow output remains unchanged except intentional CLI help text
[ ] --format html works for shape-flow with explicit --output
[ ] --format html without --output fails clearly
[ ] --format html remains rejected for data-readiness, dynamic-readiness, mlprep-standardization, and input mode
[ ] existing educational-path-only HTML policy, error text, and rejection test are generalized to educational-path plus shape-flow
[ ] generated shape-flow HTML is static and self-contained
[ ] generated shape-flow HTML has no script tag, external asset reference, data URL, or network reference
[ ] generated shape-flow HTML is deterministic
[ ] exact shape-flow HTML snapshot test exists
[ ] shape-flow HTML safety/property test exists
[ ] no dependency is added
[ ] no public API or published crate changes are made
[ ] no generated HTML artifact is checked in
[ ] release checklist includes the shape-flow HTML output command
[ ] CI includes the shape-flow HTML output command
```

---

## 9. Follow-up Boundary

Do not continue from this handoff directly into additional HTML report families,
SVG, Vega-Lite, public crates, source scanning, or expression tracing. After this
slice is reviewed, the next RFC-068 decision should choose whether another
existing report family merits HTML or whether the local HTML experiment is
sufficient for v0.32.0.
