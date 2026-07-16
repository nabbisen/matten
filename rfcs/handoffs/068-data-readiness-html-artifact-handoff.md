# RFC-068 Data-Readiness HTML Artifact Handoff

**Project:** `matten`
**Related RFC:** RFC-068: Rich Local Visualization Artifacts
**Document kind:** Compact local-tool implementation handoff
**Status:** Drafted for review; no implementation authorized
**Scope:** Local-only static HTML artifact for `tools/matten-report --demo data-readiness`

---

## 1. Summary

Implement the next RFC-068 local visualization slice by adding static HTML output
to the existing fixed `tools/matten-report` data-readiness demo only.

Accepted command:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness --format html --output target/matten-report-data-readiness.html
```

The existing Markdown/plain-text output remains the default:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness
```

This slice must not add HTML for input-mode CSV reports.

---

## 2. Reviewer Background

RFC-068 has already shipped local static HTML artifacts for four fixed demos:

```text
educational-path
shape-flow
dynamic-readiness
mlprep-standardization
```

The post-0.34 visualization gap audit records that `data-readiness` is now the
only fixed `tools/matten-report` demo without HTML. Review accepted that audit
and confirmed this handoff is the next conservative continuation if fixed-demo
HTML work proceeds.

The `data-readiness` demo uses fixed embedded CSV data:

```text
region,sales,cost,note
north,100,40,ok
south,150,45,review
east,120,55,ok
```

Demo mode selects `sales,cost`, so strict numeric conversion succeeds. This
handoff intentionally covers that success-path demo only. It must not use
`missing.csv`, `non_numeric.csv`, or arbitrary user input to show a failure-path
HTML artifact in this slice.

---

## 3. Implementation Scope

Allowed:

```text
HTML output for --demo data-readiness only
explicit --output required for data-readiness HTML
Markdown/plain text remains default
private fixed data-readiness report data builder if useful
small embedded CSS using the existing local HTML style pattern
semantic headings, tables, source/selected/left-out columns, missing counts, conversion status, and tensor preview
exact Markdown output test remains passing
exact HTML output snapshot test for data-readiness
HTML static/self-contained safety test for data-readiness
README update for data-readiness HTML
release-checklist and CI smoke command for data-readiness HTML
RFC/handoff/roadmap status updates
```

Implementation note:

```text
After this slice, every fixed --demo report family will support HTML. The
remaining negative HTML policy case should be input-mode HTML. Update
validate_format_policy(), render_report(), supported_html_demos(), user-facing
error text, and rejection tests accordingly. Do not leave a test that pretends a
fixed demo remains unsupported.
```

Not authorized:

```text
HTML for --input <csv-path> --kind data-readiness
HTML for missing.csv / non_numeric.csv failure fixtures
general input-mode report renderer
Tensor::plot / Tensor::show / Tensor::trace / Tensor::backward
automatic expression tracing
lazy expression graph
autograd
public report API
public matten-report crate
public matten-viz crate
workspace membership
new dependencies
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
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness --format markdown
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness --output target/matten-report-data-readiness.md
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness --format html --output target/matten-report-data-readiness.html
```

Rejected:

```text
--demo data-readiness --format html without --output
--input tools/matten-report/fixtures/small.csv --kind data-readiness --select sales,cost --format html --output target/matten-report-input.html
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
title containing "matten data-readiness report"
top-level heading for matten data-readiness report
the same conceptual sections as the Markdown data-readiness demo
embedded CSS only
no script tags
no external href/src references
```

The visual layout should improve scanning with:

```text
input label: demo: data-readiness
source columns: region, sales, cost, note
selected columns: sales, cost
columns left out: region, note
missing-value counts for sales and cost
strict conversion: success
tensor preview shape: [3, 2]
tensor preview row-major values: [100.0, 40.0, 150.0, 45.0, 120.0, 55.0]
```

The generated HTML should not imply it has profiled arbitrary user CSV data. It
should be clear from content that this is the fixed demo report.

---

## 6. Input-Mode Deferral

This handoff deliberately keeps input-mode HTML rejected:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --input tools/matten-report/fixtures/small.csv \
  --kind data-readiness \
  --select sales,cost \
  --format html \
  --output target/matten-report-input.html
```

Input-mode HTML requires separate review because it accepts user-controlled CSV
data and selected column names. The future policy question includes:

```text
large tables and output-size limits
column-name/value escaping coverage
error rendering
whether raw source values are shown or only summaries
whether failure fixtures should render HTML
whether HTML output is still local educational artifact scope or a general report product
```

---

## 7. Suggested Files

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
rfcs/handoffs/068-data-readiness-html-artifact-handoff.md
rfcs/handoffs/068-post-034-visualization-gap-audit.md
rfcs/handoffs/README.md
rfcs/README.md
ROADMAP.md
```

No version files, changelog, release notes, tags, or publish actions are in
scope for this handoff.

---

## 8. Verification

Minimum implementation verification:

```bash
cargo fmt --all --check
cargo fmt --manifest-path tools/matten-report/Cargo.toml --check
cargo fmt --manifest-path tools/matten-migrate/Cargo.toml --check
cargo check --manifest-path tools/matten-report/Cargo.toml
cargo test --manifest-path tools/matten-report/Cargo.toml
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness --format markdown
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness --output target/matten-report-data-readiness.md
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness --format html --output target/matten-report-data-readiness.html
cargo run --manifest-path tools/matten-report/Cargo.toml -- --input tools/matten-report/fixtures/small.csv --kind data-readiness --select sales,cost --format html --output target/matten-report-input.html
bash scripts/check-release-docs.sh
git diff --check
```

Expected failures:

```text
--demo data-readiness --format html without --output
--input tools/matten-report/fixtures/small.csv --kind data-readiness --select sales,cost --format html --output target/matten-report-input.html
```

If full workspace verification is requested for release prep later, run the
normal release checklist separately. This handoff does not require a version bump
or release-prep gate.

---

## 9. Review Checklist

```text
[ ] --demo data-readiness --format html works with explicit --output
[ ] --demo data-readiness --format html without --output returns a readable error
[ ] Markdown/plain text data-readiness output remains byte-stable except intentional CLI help text
[ ] input-mode HTML remains rejected
[ ] supported HTML demo policy covers all fixed demos
[ ] generated data-readiness HTML is static and self-contained
[ ] generated data-readiness HTML has no script tag, external asset reference, data URL, or network reference
[ ] generated data-readiness HTML is deterministic
[ ] exact data-readiness HTML snapshot test exists
[ ] data-readiness HTML safety/property test exists
[ ] README, CI, and release checklist include the data-readiness HTML output command
[ ] RFC/Roadmap tracking is updated without implying input-mode HTML or public visualization APIs
[ ] no public API, published crate, dependency, workspace membership, version, tag, or publish change
```

---

## 10. Handoff Verdict

This is a narrow continuation of the accepted RFC-068 local artifact pattern. It
does not authorize public visualization APIs, report/viz crates, input-mode
HTML, SVG/Vega-Lite/JSON, expression tracing, autograd, or any published-crate
dependency change.

If this handoff is reviewed GO, implementation may start for demo-only
`data-readiness` HTML.
