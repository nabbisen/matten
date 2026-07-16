# RFC-069 Input-Mode HTML Policy Audit

**Project:** `matten`
**Related RFCs:** RFC-063, RFC-065, RFC-068, RFC-069
**Document kind:** Policy audit and next-decision recommendation
**Status:** Drafted for review; no implementation authorized
**Date:** 2026-07-16

---

## 1. Summary

This audit starts the post-RFC-068 input-mode HTML decision as a fresh reviewed
theme.

Current source state:

```text
all fixed tools/matten-report demos support --format html with --output
input mode supports data-readiness Markdown/plain text
input mode rejects --format html
```

Recommended decision:

```text
Open input-mode HTML only as a narrow future implementation candidate.
Require a separate implementation handoff before code changes.
Keep the future feature summary-oriented, bounded, escaped, static, local-only,
and limited to --input ... --kind data-readiness.
```

This audit does not authorize implementation, version changes, release prep,
public APIs, new dependencies, tags, or publishing.

---

## 2. Current Behavior

Accepted today:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --input tools/matten-report/fixtures/small.csv \
  --kind data-readiness \
  --select sales,cost
```

Rejected today:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --input tools/matten-report/fixtures/small.csv \
  --kind data-readiness \
  --select sales,cost \
  --format html \
  --output target/matten-report-input.html
```

The rejection is deliberate and should remain until review accepts both this
policy and a later implementation handoff.

---

## 3. Why This Is Not An RFC-068 Continuation

RFC-068 closed the fixed-demo local HTML line after `0.35.0`. Input-mode HTML
crosses a different boundary because the report data is user-controlled:

```text
CSV path
headers
cell values
selected column names
conversion errors
table size
```

The project should not treat input-mode HTML as "one more demo." It should be
tracked as RFC-069 or an equivalent fresh review path.

---

## 4. Recommended Future Implementation Boundary

If review accepts opening this candidate, the next handoff should authorize only:

```text
tools/matten-report --input <csv-path> --kind data-readiness --select <cols> --format html --output <path>
Markdown/plain text remains default
--format html without --output remains rejected
static self-contained UTF-8 HTML
no JavaScript
no external assets
no data URLs
no network
summary-only report content
bounded tensor/value preview
HTML escaping for all user-controlled strings
exact output/snapshot tests
hostile-input escaping tests
input-mode HTML safety tests
CI and release-checklist smoke commands only after implementation review accepts them
```

The future handoff should reject:

```text
full raw CSV table rendering
unbounded row-major value rendering
HTML stdout
HTML for other report kinds
public report/viz crates
core Tensor visualization APIs
SVG, Vega-Lite, or JSON output
notebook/browser/dashboard scope
new dependencies in published crates
workspace membership changes for tools/matten-report
```

---

## 5. Open Policy Decisions

Review should decide before implementation:

```text
success-only HTML vs success plus numeric-conversion-error HTML
exact tensor preview limit
whether conversion errors may include selected user values
whether output should include the input path as provided or a sanitized display form
whether hostile-input tests need a dedicated fixture or can be constructed inline
```

Recommended defaults:

```text
include both success and numeric-conversion-error reports
show selected column names and missing counts
show shape on success
show a small bounded row-major preview on success
show concise conversion error text on failure
do not show the full raw CSV table
do not show unbounded failing-cell details
```

---

## 6. Review Questions

Review should decide:

```text
[ ] Is RFC-069 the right fresh boundary after RFC-068 closure?
[ ] Should input-mode HTML be opened as a narrow implementation candidate?
[ ] Is summary-only, bounded HTML the right safety policy?
[ ] Should success and numeric-conversion-error paths both be in the first implementation handoff?
[ ] Are public report/viz crates and core visualization APIs still correctly deferred?
[ ] Is the ROADMAP/RFC/handoff tracking sufficient before implementation planning?
```
