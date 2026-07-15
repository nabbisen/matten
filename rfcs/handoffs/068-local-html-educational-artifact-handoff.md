# RFC-068 Phase 1 Local HTML Educational Artifact Handoff

**Project:** `matten`
**Related RFC:** RFC-068: Rich Local Visualization Artifacts
**Document kind:** Compact local-tool implementation handoff
**Status:** Implemented and reviewed; shipped in 0.32.0
**Scope:** Local-only static HTML artifact for `tools/matten-report --demo educational-path`

---

## 1. Summary

Implement the first RFC-068 slice by adding one static HTML output mode to the
existing local `tools/matten-report` educational-path demo.

Accepted command:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo educational-path --format html --output target/matten-report-educational-path.html
```

The existing Markdown/plain-text output remains the default:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo educational-path
```

The slice must stay local-tool-only. It must not add a public API, public crate,
published-crate dependency, generated checked-in artifact, SVG renderer, browser
UI, notebook integration, expression tracer, or autograd behavior.

---

## 2. Reviewer Background

RFC-063 established visual understanding through Markdown/ASCII docs, canonical
examples, and the local `tools/matten-report` tool.

RFC-065 added the educational path and the existing `educational-path` demo,
which is currently Markdown/plain text only.

RFC-068 asks for the next smallest richer artifact: one deterministic,
self-contained HTML file for that already-reviewed educational path.

---

## 3. Implementation Scope

Allowed:

```text
one new CLI option: --format markdown|html
markdown as default format
HTML output for --demo educational-path only
explicit --output required for --format html
small embedded CSS in the generated HTML
semantic headings, tables, code blocks, simple shape/readiness blocks
local HTML escaping helper
tests for parser behavior and generated HTML substrings or exact output
README update for the new format
release-checklist update for the new HTML command
CI update for the new HTML command
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
HTML for other report families
input mode for educational-path HTML
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
```

---

## 4. Command Behavior

Accepted:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo educational-path --format html --output target/matten-report-educational-path.html
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo educational-path
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo educational-path --format markdown
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo educational-path --format markdown --output target/matten-report-educational-path.md
```

Rejected:

```text
--format html without --output
--format html for data-readiness
--format html for shape-flow
--format html for dynamic-readiness
--format html for mlprep-standardization
unknown format labels
```

Error messages should be concise and ordinary CLI misuse must not panic.

---

## 5. Output Requirements

The HTML report must include:

```text
<!doctype html>
<html lang="en">
<meta charset="utf-8">
title containing "matten educational-path report"
top-level heading for matten educational-path report
the same conceptual sections as the Markdown educational-path demo
embedded CSS only
no script tags
no external href/src references
```

The visual layout should improve scanning with:

```text
section blocks
shape-flow tables
before/after standardization table
dynamic readiness masks as small labeled rows
short notes preserving the existing educational wording
```

The implementation should not over-polish. This is a first artifact, not a
visual design system.

---

## 6. Suggested Files

Expected implementation files:

```text
tools/matten-report/src/main.rs
tools/matten-report/README.md
docs/src/contributing/release-checklist.md
.github/workflows/test.yaml
```

The implementation must add the accepted HTML command to both the release
checklist and CI. Existing `matten-report` demos are gated by documented run
commands, and the HTML writer should get the same regression protection.

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
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo educational-path
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo educational-path --format html --output target/matten-report-educational-path.html
bash scripts/check-release-docs.sh
git diff --check
```

If docs are changed, also run:

```bash
mdbook build docs
```

Remove generated `docs/book` afterward if created.

---

## 8. Acceptance Criteria

```text
[ ] Markdown/plain text output remains unchanged except intentional CLI help text
[ ] --format html works for educational-path with explicit --output
[ ] --format html without --output fails clearly
[ ] --format html is rejected for other report families in this slice
[ ] generated HTML is static and self-contained
[ ] generated HTML has no script tag, no external asset reference, and no network reference
[ ] generated HTML is deterministic
[ ] no dependency is added
[ ] no public API or published crate changes are made
[ ] no generated HTML artifact is checked in
[ ] tool tests cover accepted and rejected command shapes
[ ] release checklist includes the new educational-path HTML output command
[ ] CI includes the new educational-path HTML output command
```

---

## 9. Follow-up Boundary

Do not continue from this handoff directly into SVG, Vega-Lite, public crates, or
additional HTML report families. Those require a separate handoff or RFC
amendment after the first HTML artifact is reviewed.
