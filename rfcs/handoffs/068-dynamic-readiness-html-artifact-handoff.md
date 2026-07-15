# RFC-068 Dynamic-Readiness HTML Artifact Handoff

**Project:** `matten`
**Related RFC:** RFC-068: Rich Local Visualization Artifacts
**Document kind:** Compact local-tool implementation handoff
**Status:** Draft for review; does not authorize implementation until accepted
**Scope:** Local-only static HTML artifact for `tools/matten-report --demo dynamic-readiness`

---

## 1. Summary

Implement the next RFC-068 local visualization slice by adding one static HTML
output mode to the existing local `tools/matten-report` dynamic-readiness demo.

Accepted command:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo dynamic-readiness --format html --output target/matten-report-dynamic-readiness.html
```

The existing Markdown/plain-text output remains the default:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo dynamic-readiness
```

This slice extends HTML to exactly one additional fixed report family. It must
not add HTML for data-readiness, mlprep-standardization, or input-mode reports.

---

## 2. Reviewer Background

RFC-068 already shipped static local HTML artifacts for:

```text
educational-path
shape-flow
```

The post-0.32 continuation audit recommends `dynamic-readiness` as the next
candidate because it is fixed demo-only, has no input-file mode, and explains
data meaning rather than only shape mechanics:

```text
dynamic values
missing-value masks
strict numeric-readiness masks
strict conversion failure
explicit policy conversion
```

This handoff does not reopen public visualization APIs or richer renderer
formats. It only extends the reviewed local static HTML pattern to one more
existing fixed demo.

---

## 3. Implementation Scope

Allowed:

```text
HTML output for --demo dynamic-readiness only
explicit --output required for dynamic-readiness HTML
Markdown/plain text remains default
private dynamic-readiness data builder if useful
small embedded CSS using the existing local HTML style pattern
semantic headings, tables, code blocks, masks, and conversion rows
exact Markdown output test remains passing
exact HTML output snapshot test for dynamic-readiness
HTML static/self-contained safety test for dynamic-readiness
README update for dynamic-readiness HTML
release-checklist and CI smoke command for dynamic-readiness HTML
RFC/handoff/roadmap status updates
```

Implementation note:

```text
The current HTML policy accepts educational-path and shape-flow. Update
validate_format_policy(), the mirrored render_report() HTML guard, their
user-facing error message, and the html_format_is_limited_to_accepted_html_demos
test. The rejection test must switch to a still-unsupported family such as
mlprep-standardization, and the error should name all accepted HTML demos.
```

Not authorized:

```text
HTML for data-readiness
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
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo dynamic-readiness
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo dynamic-readiness --format markdown
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo dynamic-readiness --output target/matten-report-dynamic-readiness.md
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo dynamic-readiness --format html --output target/matten-report-dynamic-readiness.html
```

Rejected:

```text
--demo dynamic-readiness --format html without --output
--format html for data-readiness
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
title containing "matten dynamic-readiness report"
top-level heading for matten dynamic-readiness report
the same conceptual sections as the Markdown dynamic-readiness demo
embedded CSS only
no script tags
no external href/src references
```

The visual layout should improve scanning with:

```text
dynamic-value table showing coordinates and element values
schema summary table
none mask row
strict numeric-readiness mask row
strict numeric-ready status
strict conversion rejection
explicit policy conversion row
converted shape and converted row-major values
short note preserving "not automatic data profiling"
```

The implementation should stay modest and deterministic. Do not introduce a
general renderer abstraction unless it removes obvious duplication with the
existing educational-path and shape-flow HTML helpers without changing behavior.

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
rfcs/done/068-rich-local-visualization-artifacts.md
rfcs/handoffs/068-dynamic-readiness-html-artifact-handoff.md
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
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo dynamic-readiness
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo dynamic-readiness --format html --output target/matten-report-dynamic-readiness.html
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
[ ] Markdown/plain text dynamic-readiness output remains unchanged except intentional CLI help text
[ ] --format html works for dynamic-readiness with explicit --output
[ ] --format html without --output fails clearly
[ ] --format html remains rejected for data-readiness, mlprep-standardization, and input mode
[ ] existing accepted HTML policy, error text, and rejection test are generalized to educational-path, shape-flow, and dynamic-readiness
[ ] generated dynamic-readiness HTML is static and self-contained
[ ] generated dynamic-readiness HTML has no script tag, external asset reference, data URL, or network reference
[ ] generated dynamic-readiness HTML is deterministic
[ ] exact dynamic-readiness HTML snapshot test exists
[ ] dynamic-readiness HTML safety/property test exists
[ ] no dependency is added
[ ] no public API or published crate changes are made
[ ] no generated HTML artifact is checked in
[ ] release checklist includes the dynamic-readiness HTML output command
[ ] CI includes the dynamic-readiness HTML output command
```

---

## 9. Follow-up Boundary

Do not continue from this handoff directly into additional HTML report families,
input-mode HTML, SVG, Vega-Lite, public crates, source scanning, or expression
tracing. After this slice is reviewed, the next RFC-068 decision should choose
whether `mlprep-standardization` merits the final fixed-demo HTML artifact,
whether `data-readiness` needs a separate input-mode HTML policy audit, or
whether the local HTML experiment is sufficient for the current release family.
