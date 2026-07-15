# RFC-068 MLPREP-Standardization HTML Artifact Handoff

**Project:** `matten`
**Related RFC:** RFC-068: Rich Local Visualization Artifacts
**Document kind:** Compact local-tool implementation handoff
**Status:** Implemented for review; review pending
**Scope:** Local-only static HTML artifact for `tools/matten-report --demo mlprep-standardization`

---

## 1. Summary

Implement the next RFC-068 local visualization slice by adding one static HTML
output mode to the existing local `tools/matten-report` mlprep-standardization
demo.

Accepted command:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo mlprep-standardization --format html --output target/matten-report-mlprep-standardization.html
```

The existing Markdown/plain-text output remains the default:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo mlprep-standardization
```

This slice extends HTML to exactly one additional fixed report family. It must
not add HTML for `data-readiness` or input-mode reports.

---

## 2. Reviewer Background

RFC-068 already shipped static local HTML artifacts for:

```text
educational-path
shape-flow
dynamic-readiness
```

The post-0.33 continuation audit recommends `mlprep-standardization` as the next
candidate because it is fixed demo-only, has no input-file mode, and explains a
before/after numerical transformation:

```text
input values
before column means
before population standard deviations
standardize_columns(input)
output values
after column means
after population standard deviations
unchanged shape
```

This handoff does not reopen public visualization APIs or richer renderer
formats. It only extends the reviewed local static HTML pattern to one more
existing fixed demo.

---

## 3. Implementation Scope

Allowed:

```text
HTML output for --demo mlprep-standardization only
explicit --output required for mlprep-standardization HTML
Markdown/plain text remains default
private mlprep-standardization data builder if useful
small embedded CSS using the existing local HTML style pattern
semantic headings, tables, before/after comparison rows, and shape rows
exact Markdown output test remains passing
exact HTML output snapshot test for mlprep-standardization
HTML static/self-contained safety test for mlprep-standardization
README update for mlprep-standardization HTML
release-checklist and CI smoke command for mlprep-standardization HTML
RFC/handoff/roadmap status updates
```

Implementation note:

```text
The current HTML policy accepts educational-path, shape-flow, and
dynamic-readiness. Update validate_format_policy(), the mirrored render_report()
HTML guard, their user-facing error message, and the
html_format_is_limited_to_accepted_html_demos test. The rejection test must
switch to a still-unsupported family such as data-readiness, and the error
should name all accepted HTML demos.
```

Not authorized:

```text
HTML for data-readiness
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
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo mlprep-standardization
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo mlprep-standardization --format markdown
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo mlprep-standardization --output target/matten-report-mlprep-standardization.md
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo mlprep-standardization --format html --output target/matten-report-mlprep-standardization.html
```

Rejected:

```text
--demo mlprep-standardization --format html without --output
--format html for data-readiness
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
title containing "matten mlprep-standardization report"
top-level heading for matten mlprep-standardization report
the same conceptual sections as the Markdown mlprep-standardization demo
embedded CSS only
no script tags
no external href/src references
```

The visual layout should improve scanning with:

```text
input shape and values
before column means
before population standard deviations
operation row for standardize_columns(input)
output shape and values
after column means
after population standard deviations
unchanged-shape note
short note preserving "not model-quality analysis"
```

The implementation should stay modest and deterministic. Do not introduce a
general renderer abstraction unless it removes obvious duplication with the
existing HTML helpers without changing behavior.

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
rfcs/handoffs/068-mlprep-standardization-html-artifact-handoff.md
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
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo mlprep-standardization
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo mlprep-standardization --format markdown
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo mlprep-standardization --output target/matten-report-mlprep-standardization.md
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo mlprep-standardization --format html --output target/matten-report-mlprep-standardization.html
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness --format html --output target/matten-report-data-readiness.html
cargo run --manifest-path tools/matten-report/Cargo.toml -- --input tools/matten-report/fixtures/small.csv --kind data-readiness --select sales,cost --format html --output target/matten-report-input.html
```

Expected failures:

```text
--demo mlprep-standardization --format html without --output
--demo data-readiness --format html --output target/matten-report-data-readiness.html
--input tools/matten-report/fixtures/small.csv --kind data-readiness --select sales,cost --format html --output target/matten-report-input.html
```

If full workspace verification is requested for release prep later, run the
normal release checklist separately. This handoff does not require a version bump
or release-prep gate.

---

## 8. Review Checklist

```text
[ ] --demo mlprep-standardization --format html works with explicit --output
[ ] --demo mlprep-standardization --format html without --output returns a readable error
[ ] Markdown/plain text mlprep-standardization output remains byte-stable except intentional CLI help text
[ ] --format html remains rejected for data-readiness and input mode
[ ] existing accepted HTML policy, error text, and rejection tests are generalized to educational-path, shape-flow, dynamic-readiness, and mlprep-standardization
[ ] generated mlprep-standardization HTML is static and self-contained
[ ] generated mlprep-standardization HTML has no script tag, external asset reference, data URL, or network reference
[ ] generated mlprep-standardization HTML is deterministic
[ ] exact mlprep-standardization HTML snapshot test exists
[ ] mlprep-standardization HTML safety/property test exists
[ ] no public API, published crate, dependency, workspace membership, version, tag, or publish change
[ ] README documents the new local HTML command
[ ] release checklist includes the mlprep-standardization HTML output command
[ ] CI includes the mlprep-standardization HTML output command
```

---

## 9. Handoff Verdict

This is a narrow continuation of the accepted RFC-068 local artifact pattern. It
does not authorize public visualization APIs, report/viz crates, SVG/Vega-Lite,
input-mode HTML, data-readiness HTML, or expression tracing.

After this slice is reviewed, the next RFC-068 decision should choose whether
the fixed-demo HTML series is complete or whether `data-readiness` demo HTML is
still worth a separate reviewed handoff.
