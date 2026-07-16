# RFC-069 Input-Mode HTML Implementation Handoff

**Project:** `matten`
**Related RFC:** RFC-069: Input-Mode HTML Report Policy
**Document kind:** Compact local-tool implementation handoff
**Status:** Drafted for review; no implementation authorized until review accepts this handoff
**Scope:** Local-only static HTML artifact for `tools/matten-report --input ... --kind data-readiness`

---

## 1. Summary

Implement the first RFC-069 feature slice by adding static HTML output to the
existing `tools/matten-report` data-readiness input mode:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --input tools/matten-report/fixtures/small.csv \
  --kind data-readiness \
  --select sales,cost \
  --format html \
  --output target/matten-report-input.html
```

The existing Markdown/plain-text input mode remains the default:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --input tools/matten-report/fixtures/small.csv \
  --kind data-readiness \
  --select sales,cost
```

This slice must remain local-tool-only, summary-oriented, bounded, escaped, and
data-readiness-only.

---

## 2. Reviewer Background

RFC-068 closed the fixed-demo local HTML line after `0.35.0`. RFC-069 starts a
fresh input-mode policy path because input-mode reports accept user-controlled
CSV data and selected column names.

The RFC-069 policy review accepted the planning direction with no conditions and
left one forward note for this handoff: do not only bound row/value volume.
Also bound the breadth and length of user-controlled strings such as wide CSV
column lists, long paths, and long headers.

This handoff turns that note into concrete implementation requirements.

---

## 3. Implementation Scope

Allowed:

```text
HTML output for --input <csv-path> --kind data-readiness --select <cols> only
explicit --output required for input-mode HTML
Markdown/plain text remains default
summary-only HTML content
success report HTML
numeric-conversion-error report HTML
bounded source/selected/left-out column displays
bounded input-path/header/error string displays
bounded row-major tensor preview
HTML escaping for every user-controlled string
small embedded CSS using the existing static HTML style pattern
exact/snapshot tests for success and conversion-error HTML
hostile-input escaping tests
bounds/truncation tests
HTML static/self-contained safety tests
README documentation for input-mode HTML
CI and release-checklist smoke commands for input-mode HTML
RFC/handoff/roadmap status updates
```

Not authorized:

```text
HTML stdout
HTML for --kind values other than data-readiness
HTML for fixed demos beyond existing behavior
full raw CSV table rendering
unbounded row-major tensor values
unbounded column lists
unbounded path/header/error strings
public report API
public matten-report crate
public matten-viz crate
workspace membership change
new dependencies in published crates
new dependencies in tools/matten-report unless separately justified and reviewed
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
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --input tools/matten-report/fixtures/small.csv \
  --kind data-readiness \
  --select sales,cost

cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --input tools/matten-report/fixtures/small.csv \
  --kind data-readiness \
  --select sales,cost \
  --format markdown

cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --input tools/matten-report/fixtures/small.csv \
  --kind data-readiness \
  --select sales,cost \
  --output target/matten-report-input.md

cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --input tools/matten-report/fixtures/small.csv \
  --kind data-readiness \
  --select sales,cost \
  --format html \
  --output target/matten-report-input.html
```

Rejected:

```text
--input <csv-path> --kind data-readiness --select <cols> --format html without --output
--input <csv-path> --kind <anything-else> --select <cols> --format html --output <path>
--format html with unknown format labels or ordinary CLI misuse
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
input label showing path-based input, not demo input
embedded CSS only
no script tags
no external href/src references
```

The visual layout should improve scanning with:

```text
input label
source columns
selected columns
columns left out
missing-value counts for selected columns
strict conversion status
tensor preview shape on success
row-major preview on success
conversion error summary on failure
```

The generated HTML must not imply it has rendered or profiled the full raw CSV
table. It is a bounded summary artifact.

---

## 6. Required Bounds

Use these display bounds in the first implementation:

```text
MAX_DISPLAY_COLUMNS = 12
MAX_DISPLAY_CHARS = 120
MAX_ERROR_CHARS = 240
MAX_TENSOR_PREVIEW_VALUES = 12
```

Required behavior:

```text
source columns: show at most 12 names, then an "... N more" marker
selected columns: show at most 12 names, then an "... N more" marker
columns left out: show at most 12 names, then an "... N more" marker
input path label: escape and display-cap at 120 characters
column/header labels: escape and display-cap at 120 characters
conversion error text: escape and display-cap at 240 characters
row-major tensor preview: show at most 12 values, then an "... N more" marker
```

The cap is display-only. The existing table parsing, selected-column lookup, and
numeric conversion semantics must not change.

---

## 7. Escaping Requirements

Every user-controlled string written to HTML must go through the HTML escape
helper before writing:

```text
input path labels
source column names
selected column names
left-out column names
conversion error text
any future rendered data values
truncation markers only after the marker content is fixed by code
```

Tests must include hostile or HTML-like content covering at least:

```text
CSV header containing <script> or an attribute-like string
CSV value that causes a numeric conversion error and contains HTML-like text
long header or path display truncation
wide CSV column-list truncation
```

The generated HTML must not contain raw hostile strings as executable markup.

---

## 8. Suggested Implementation Shape

Expected code direction:

```text
reuse the existing data-readiness summary computations where practical
extract a private DataReadinessReportData variant that can represent demo and input-mode summaries
add a render_input_data_readiness_html_report(...) function or a shared renderer with an explicit input label
keep Markdown output byte-identical unless a reviewed test update is necessary
keep existing fixed-demo HTML snapshots stable unless this handoff explicitly justifies shared-renderer churn
update validate_format_policy() so input-mode data-readiness HTML with --output is accepted
keep --format html without --output rejected before input-kind checks
keep unsupported input kinds rejected
```

Do not add a general report model public API.

---

## 9. Suggested Files

Expected implementation files:

```text
tools/matten-report/src/main.rs
tools/matten-report/README.md
docs/src/contributing/release-checklist.md
.github/workflows/test.yaml
```

Expected tracking files:

```text
rfcs/proposed/069-input-mode-html-report-policy.md
rfcs/handoffs/069-input-mode-html-policy-audit.md
rfcs/handoffs/069-input-mode-html-implementation-handoff.md
rfcs/handoffs/README.md
rfcs/README.md
ROADMAP.md
```

No version files, changelog, release notes, tags, or publish actions are in
scope for this handoff.

---

## 10. Verification

Minimum implementation verification:

```bash
cargo test --manifest-path tools/matten-report/Cargo.toml
cargo run --manifest-path tools/matten-report/Cargo.toml -- --input tools/matten-report/fixtures/small.csv --kind data-readiness --select sales,cost --format html --output target/matten-report-input.html
cargo run --manifest-path tools/matten-report/Cargo.toml -- --input tools/matten-report/fixtures/non_numeric.csv --kind data-readiness --select sales,cost --format html --output target/matten-report-input-error.html
bash scripts/check-release-docs.sh
git diff --check
```

Implementation review should additionally inspect generated HTML or exact
snapshots for:

```text
success-path sections
numeric-conversion-error sections
no script tags
no external href/src references
escaped hostile input
column-list truncation
long-field truncation
bounded tensor preview
no full raw CSV table
```

Generated HTML files under `target/` must not be committed.

---

## 11. Acceptance Checklist

```text
[ ] input-mode data-readiness HTML works only with explicit --output
[ ] input-mode HTML without --output fails clearly
[ ] Markdown/plain-text input mode remains default
[ ] unsupported input kinds remain rejected
[ ] success-path HTML is covered by exact/snapshot tests
[ ] numeric-conversion-error HTML is covered by exact/snapshot tests
[ ] hostile input is escaped
[ ] wide column lists are display-bounded
[ ] long path/header/error strings are display-bounded
[ ] row-major tensor preview is display-bounded
[ ] generated HTML is static and self-contained
[ ] README, CI, and release checklist document/smoke the accepted command
[ ] no public API, published dependency, workspace membership, version, tag, or publish change
```
