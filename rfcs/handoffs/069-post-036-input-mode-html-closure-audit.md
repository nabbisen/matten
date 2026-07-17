# RFC-069 Post-0.36 Input-Mode HTML Closure Audit

**Project:** `matten`
**Related RFCs:** RFC-063, RFC-065, RFC-068, RFC-069
**Document kind:** Closure audit and next-theme recommendation
**Status:** Drafted for review; no implementation authorized
**Date:** 2026-07-17

---

## 1. Summary

After `0.36.0`, RFC-069 has completed the first and only reviewed input-mode
local HTML report:

```text
tools/matten-report --input <csv> --kind data-readiness --select <cols> --format html --output <path>
```

Audit recommendation:

```text
Close RFC-069 after 0.36.0 for the reviewed data-readiness input-mode HTML
scope.
Do not keep adding input-mode HTML paths by default.
Treat public report/viz crates, JSON/SVG/Vega-Lite output, expression tracing,
autograd, and core visualization APIs as separate future RFC decisions.
```

This audit is a planning/status record only. It does not authorize code changes,
release metadata changes, tags, publishing, public APIs, new dependencies, or a
new release.

---

## 2. Current Repository State

`tools/matten-report` currently supports local static HTML for:

```text
fixed demo: educational-path
fixed demo: shape-flow
fixed demo: dynamic-readiness
fixed demo: mlprep-standardization
fixed demo: data-readiness
input mode: data-readiness
```

The input-mode HTML support added by RFC-069 is intentionally narrower than a
general CSV renderer:

```text
data-readiness only
summary-only output
explicit --output required
Markdown/plain text remains default
success and numeric-conversion-error reports covered
user-controlled strings escaped
column lists, long fields, conversion errors, and tensor previews bounded
local static artifact only
```

The implementation does not expose a public report model, public visualization
crate, reusable renderer API, core `Tensor` visualization method, JSON report
format, SVG output, Vega-Lite output, browser integration, notebook integration,
or expression-tracing semantics.

---

## 3. Why RFC-069 Should Close

RFC-069 deliberately answered one narrow question:

```text
Can the already-reviewed local HTML artifact pattern be safely extended from
fixed demo data to one user-controlled input-mode report?
```

The reviewed answer is now implemented and released for data-readiness input
mode. Continuing under RFC-069 without a new decision would blur separate
product and safety questions:

```text
whether every input-mode report should have HTML
whether raw CSV values should ever be rendered in larger tables
whether report data should become a public model
whether generated reports should gain machine-readable JSON output
whether visualization should move from local tooling into published crates
whether core Tensor APIs should expose visualization or expression graphs
```

Those are not cleanup work for RFC-069. They are new scope decisions.

---

## 4. Candidate Next Themes

| Candidate | Readiness | Recommendation |
|---|---|---|
| RFC-069 data-readiness input-mode HTML | Complete | Close after `0.36.0` |
| More input-mode HTML paths | Low | Future RFC/handoff only after a concrete report path is named |
| Raw CSV/table HTML rendering | Low | Defer; larger output-size and data-display policy needed |
| JSON report output | Medium-low | Future RFC if review/testing or automation needs it |
| Public `matten-report` crate | Low | Future RFC only after report-model ownership is stable |
| Public `matten-viz` crate | Low | Future RFC only; not implied by local artifacts |
| SVG output | Low | Future RFC/handoff only |
| Vega-Lite output | Low | Future RFC/handoff only; would introduce dependency/product questions |
| Notebook/browser/dashboard integration | Low | Future RFC only |
| Core visualization APIs (`Tensor::plot`, `Tensor::show`) | Low | Future RFC only; crosses core API boundary |
| Expression tracing / operation graph visualization | Low | Future RFC only; crosses computation-model boundary |
| Autograd / `backward()` | Low | Future RFC only; separate tensor semantics and API design |

---

## 5. Recommended Next Project Move

If review accepts this audit, the next project move should not be another
automatic visualization feature. Recommended order:

```text
1. Record RFC-069 closure after 0.36.0.
2. Choose one future theme explicitly:
   - public visualization/report readiness audit,
   - JSON report policy audit,
   - non-visual backlog audit such as streaming/large CSV,
   - bridge policy revisit for nalgebra/candle,
   - benchmark hard-gate policy audit.
3. Require a fresh RFC or handoff before implementation.
```

The strongest visualization candidate is a readiness audit for public
`matten-report` / `matten-viz` ownership. It should be audit-only first because
the local artifacts do not yet prove a stable public report model, dependency
policy, or renderer contract.

The strongest non-visual candidate is a streaming/large CSV readiness audit.
That work has long-standing roadmap visibility, but it should remain separate
from local visualization so it does not inherit HTML/report assumptions.

---

## 6. Recommended Tracker Updates

If this audit is accepted, durable project records should say:

```text
RFC-069: complete after 0.36.0 for data-readiness input-mode local HTML
more input-mode HTML: deferred until a concrete report path is reviewed
public report/viz crates: deferred; separate RFC required
JSON/SVG/Vega-Lite output: deferred; separate RFC or handoff required
core visualization APIs / expression tracing / autograd: deferred; separate RFC required
```

No current-family install snippets, crate versions, release notes, public API
docs, or published crate metadata need to change for this audit.

---

## 7. Review Questions

Review should decide:

```text
[ ] Should RFC-069 close after the 0.36.0 data-readiness input-mode HTML release?
[ ] Is it correct to avoid further input-mode HTML work without a named report path?
[ ] Are public report/viz crates still correctly deferred?
[ ] Are JSON, SVG, Vega-Lite, notebook/browser, expression tracing, and autograd still correctly deferred?
[ ] Is a public visualization/report readiness audit the right future visualization candidate?
[ ] Are ROADMAP.md, rfcs/README.md, and the handoff index sufficient tracking surfaces for this audit?
```
