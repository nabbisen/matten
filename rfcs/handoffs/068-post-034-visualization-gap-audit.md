# RFC-068 Post-0.34 Visualization Gap Audit

**Project:** `matten`
**Related RFCs:** RFC-063, RFC-065, RFC-068
**Document kind:** Gap audit and next-decision recommendation
**Status:** Reviewed GO; retained as pre-handoff decision record
**Date:** 2026-07-16

---

## 1. Summary

After `0.34.0`, RFC-068 has proven the local static HTML pattern for four fixed
`tools/matten-report` demos:

```text
tools/matten-report --demo educational-path --format html --output <path>
tools/matten-report --demo shape-flow --format html --output <path>
tools/matten-report --demo dynamic-readiness --format html --output <path>
tools/matten-report --demo mlprep-standardization --format html --output <path>
```

The remaining visualization gap is not public visualization. It is the decision
point around the one fixed demo that still has no HTML output:

```text
tools/matten-report --demo data-readiness
```

Audit recommendation:

```text
Do not implement directly from this audit.
Ask review to choose whether to close the fixed-demo local HTML line now or
draft one dedicated handoff for demo-only data-readiness HTML.
Keep input-mode HTML deferred unless a separate review explicitly accepts its
user-controlled data policy.
```

---

## 2. Current Repository State

`tools/matten-report` currently supports five report families:

```text
data-readiness
shape-flow
dynamic-readiness
mlprep-standardization
educational-path
```

HTML support after `0.34.0` is:

```text
educational-path: supported
shape-flow: supported
dynamic-readiness: supported
mlprep-standardization: supported
data-readiness: not supported
input mode: not supported
```

Markdown/plain text remains the default format. HTML remains explicit-file-only
and local-only.

The current release checklist and CI already smoke the Markdown/plain-text
`data-readiness` demo, explicit Markdown output, input-mode CSV report, and the
four accepted HTML demos.

---

## 3. Remaining Candidates

| Candidate | Readiness | Recommendation |
|---|---|---|
| Close fixed-demo local HTML line | Medium | Valid if review decides four HTML artifacts are enough |
| `data-readiness` demo-only HTML | Medium-high | Best remaining feature candidate if visualization continues |
| input-mode HTML for CSV reports | Low | Defer; user-controlled data expands escaping, table-size, and output-policy review |
| shared HTML renderer cleanup | Low-medium | Allow only inside a selected implementation handoff if it removes concrete duplication |
| SVG output | Low | Future RFC only |
| Vega-Lite / JSON report output | Low | Future RFC only |
| public `matten-report` crate | Low | Future RFC only after stable report-model and renderer boundaries exist |
| public `matten-viz` crate | Low | Future RFC only |
| expression tracing / core visualization APIs | Low | Future RFC only; crosses the core API boundary |

---

## 4. Why Data-Readiness Is The Only Plausible Next Feature Slice

`data-readiness` is the sole fixed report family without HTML. It already has a
small deterministic demo input:

```text
region,sales,cost,note
north,100,40,ok
south,150,45,review
east,120,55,ok
```

A demo-only HTML artifact could make these concepts easier to scan:

```text
source columns
selected columns
columns left out
missing-value counts
strict numeric conversion success or readable failure
tensor shape and row-major preview when conversion succeeds
```

This would complete the fixed-demo local HTML set without exposing public
rendering APIs or accepting arbitrary user CSV content into HTML output.

---

## 5. Why Input-Mode HTML Should Remain Deferred

Input-mode HTML is materially different from fixed-demo HTML.

The command shape below accepts user-controlled CSV data and selected column
names:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --input tools/matten-report/fixtures/small.csv \
  --kind data-readiness \
  --select sales,cost \
  --format html \
  --output target/matten-report-input.html
```

Even with escaping, accepting this shape would require a separate policy review
for:

```text
large tables and output-size limits
column-name/value escaping coverage
error rendering
whether raw source values are shown or only summaries
whether parent directory/file behavior differs for user data
whether HTML output is still a local educational artifact or a general report product
```

This audit does not authorize that work.

---

## 6. Required Boundaries If A Data-Readiness Handoff Is Opened

Any follow-up handoff for `data-readiness` should be narrow:

```text
HTML output for --demo data-readiness only
explicit --output required
Markdown/plain text remains default
input-mode HTML remains rejected
static deterministic self-contained UTF-8 HTML
no JavaScript
no external assets
no data URLs
no network
no telemetry
no generated HTML checked into version control
no public API
no published crate
no workspace membership change for tools/matten-report
no new dependency in published crates
no project scanning
no project mutation
```

The accepted command shape, if review chooses to open the handoff, should be:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --demo data-readiness \
  --format html \
  --output target/matten-report-data-readiness.html
```

The following should remain rejected:

```text
--demo data-readiness --format html without --output
--input <csv-path> --kind data-readiness --select <columns> --format html --output <path>
```

If `data-readiness` demo HTML is later implemented, all fixed demos would support
HTML. The rejection tests should then use input-mode HTML as the remaining
negative policy case rather than pretending there is still an unsupported demo.

---

## 7. Why Not Public Visualization Yet

Four fixed local HTML artifacts prove the local artifact pattern is useful. They
still do not prove a stable public API surface.

Public visualization crates or core visualization APIs still need separate
answers for:

```text
stable report data model boundaries
versioned renderer/output contract
dependency policy for renderer backends
whether generated artifacts are educational examples or product features
security and escaping policy for user-controlled data
maintenance ownership
release gates and docs
```

This audit therefore keeps these deferred:

```text
public matten-report crate
public matten-viz crate
Tensor::plot / Tensor::show / Tensor::trace
automatic expression tracing
autograd
SVG
Vega-Lite
JSON report output
browser UI / notebook / dashboard
```

---

## 8. Review Questions

Review should decide:

```text
[ ] Should the fixed-demo local HTML line close after 0.34.0?
[ ] If not, is demo-only data-readiness HTML the right next slice?
[ ] Should input-mode HTML remain deferred?
[ ] Are public report/viz crates and core visualization APIs still correctly deferred?
[ ] Is a compact data-readiness HTML handoff enough, or does the next slice require a new RFC?
```

---

## 9. Audit Verdict

```text
READY FOR REVIEW OF THE POST-0.34 RFC-068 DECISION POINT
NOT READY FOR DIRECT IMPLEMENTATION
NOT READY FOR INPUT-MODE HTML
NOT READY FOR PUBLIC REPORT/VIZ CRATES
NOT READY FOR CORE VISUALIZATION APIS
```

Next action if this audit is accepted:

```text
Either close the local fixed-demo HTML line as complete for now, or draft a
dedicated RFC-068 data-readiness demo-only HTML handoff for review.
```

Accepted follow-up:

```text
Draft RFC-068 data-readiness demo-only HTML handoff for review.
```
