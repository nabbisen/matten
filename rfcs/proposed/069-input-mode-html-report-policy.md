# RFC-069: Input-Mode HTML Report Policy

**Status:** Proposed
**Target:** post-0.35 reviewed policy decision; no implementation authorized
**Theme:** Safe local HTML reporting for user-provided `matten-report` CSV input
**Depends on:** RFC-001, RFC-037, RFC-063, RFC-065, RFC-068
**Related:** RFC-023, RFC-034, RFC-035, RFC-066, RFC-067

---

## 1. Summary

This RFC proposes the policy boundary for possibly allowing local static HTML
output from `tools/matten-report` input mode:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --input tools/matten-report/fixtures/small.csv \
  --kind data-readiness \
  --select sales,cost \
  --format html \
  --output target/matten-report-input.html
```

Today this command is rejected. That rejection is correct until this RFC, its
handoff, and a later implementation review explicitly accept the user-controlled
data policy.

The proposed direction is conservative:

```text
local tool only
data-readiness input mode only
explicit --output required
summary-oriented HTML, not full raw CSV rendering
bounded tensor/value preview
static self-contained HTML
no JavaScript
no external assets
no public crate or core API
```

This RFC does not itself authorize implementation. It defines what review must
accept before input-mode HTML can be implemented.

---

## 2. Background

RFC-068 completed local static HTML artifacts for every fixed
`tools/matten-report --demo ...` family through `0.35.0`.

Input-mode HTML is different from fixed-demo HTML because it accepts
user-controlled inputs:

```text
file path
CSV headers
CSV cell values
selected column names
conversion errors derived from input data
potentially large tables
```

The fixed-demo renderer can rely on deterministic embedded data. Input mode
cannot. Even with escaping, it needs a separate policy for output size, visible
raw values, error rendering, and whether this is still an educational local
artifact rather than a general reporting product.

---

## 3. Goals

1. Decide whether input-mode HTML is acceptable at all.
2. Keep the scope limited to `tools/matten-report --input ... --kind data-readiness`.
3. Preserve Markdown/plain text as the default output.
4. Keep HTML explicit-file-only with `--output`.
5. Require HTML escaping for every user-controlled string.
6. Avoid unbounded raw table rendering.
7. Require bounded tensor/value previews.
8. Keep all published crates and core APIs unchanged.
9. Keep public report/viz crates deferred.

---

## 4. Non-goals

This RFC does not authorize:

```text
[ ] implementation before review accepts the policy and handoff
[ ] HTML stdout
[ ] HTML for report kinds other than input-mode data-readiness
[ ] full raw CSV table rendering
[ ] large-data or streaming report support
[ ] public matten-report crate
[ ] public matten-viz crate
[ ] Tensor::plot(), Tensor::show(), Tensor::trace(), or Tensor::backward()
[ ] expression tracing
[ ] autograd
[ ] SVG output
[ ] Vega-Lite JSON output
[ ] JSON report output
[ ] JavaScript
[ ] external CSS, fonts, images, or network assets
[ ] data URLs
[ ] notebook, GUI, dashboard, or browser server integration
[ ] new dependency in published crates
[ ] workspace membership change for tools/matten-report
[ ] telemetry
[ ] project scanning or project mutation
[ ] version bump, tag, or publish action
```

---

## 5. Proposed Policy

### 5.1 Accepted command shape

If implementation is later authorized, the accepted input-mode HTML command
should be:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --input <csv-path> \
  --kind data-readiness \
  --select <col1,col2> \
  --format html \
  --output <report.html>
```

The existing Markdown/plain-text input mode remains valid and remains the
default:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --input <csv-path> \
  --kind data-readiness \
  --select <col1,col2>
```

### 5.2 Output policy

Input-mode HTML must be:

```text
local file output only
explicit --output only
static
self-contained
deterministic for the same input file and selected columns
UTF-8
print-friendly
free of JavaScript
free of external assets
free of data URLs
free of network access
```

The tool should not silently create parent directories unless an existing
reviewed tool policy accepts that behavior.

### 5.3 Data display policy

Input-mode HTML should be summary-oriented. It may show:

```text
input label using the provided path
source column names
selected column names
columns left out
missing-value counts for selected columns
strict numeric conversion status
conversion error summary when conversion fails
tensor shape when conversion succeeds
bounded row-major tensor preview when conversion succeeds
```

It must not show:

```text
the full raw CSV table
unbounded row-major tensor values
unbounded lists of failing cells
source file contents outside the existing report summary
```

The future implementation handoff should choose explicit preview limits before
coding starts.

### 5.4 Escaping policy

Every user-controlled string written to HTML must go through an explicit escape
helper. This includes:

```text
input path labels
column names
selected column labels
conversion errors
any rendered data values
```

Tests must include hostile or HTML-like text in at least one path, header, value,
or error-bearing fixture before input-mode HTML is accepted.

---

## 6. Risk Assessment

| Risk | Treatment |
|---|---|
| HTML injection through CSV headers or values | Escape all user-controlled text; test hostile input |
| Oversized generated HTML | Forbid full raw table rendering; require bounded previews |
| Product-scope drift into a general reporting engine | Keep local tool only; data-readiness input mode only |
| Public API/dependency drift | No published crate changes; no new published dependencies |
| Confusion with fixed demos | Label input as path-based/user input, not demo data |
| Error leakage/noise | Render concise conversion summaries; avoid unbounded failing-cell lists |

---

## 7. Review Questions

Review should decide:

```text
[ ] Should input-mode HTML be opened as a separately reviewed feature candidate?
[ ] Is summary-only HTML the right boundary, with full raw CSV rendering rejected?
[ ] Should the first implementation include both success and numeric-conversion error reports?
[ ] What exact preview limits should a future implementation handoff require?
[ ] Are public report/viz crates, core visualization APIs, SVG/Vega-Lite/JSON, expression tracing, and autograd still correctly deferred?
```

If this RFC is accepted, the next step should be a compact implementation
handoff. No implementation should start directly from this RFC alone.
