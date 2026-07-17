# RFC-070: Public Visualization and Report Readiness Audit

**Status:** Proposed; audit-only, no implementation authorized
**Target:** Post-0.36 readiness decision for public reporting / visualization scope
**Theme:** Decide whether `matten-report` / `matten-viz` is ready to become a public product surface
**Depends on:** RFC-022, RFC-030, RFC-063, RFC-065, RFC-068, RFC-069
**Related:** RFC-049, RFC-054, RFC-066, RFC-067

---

## 1. Summary

This RFC proposes an audit-only readiness review for public visualization and
reporting in the `matten` family.

It does **not** authorize a public `matten-report` crate, a public `matten-viz`
crate, a reusable renderer API, a report model API, JSON/SVG/Vega-Lite output,
notebook/browser integration, core `Tensor` visualization methods, expression
tracing, autograd, dependency changes, version bumps, tags, or publishing.

The audit should answer:

```text
Are the local visualization/report artifacts mature enough to justify a future
public reporting or visualization product surface?
If not, what concrete prerequisites must be met first?
If yes, what narrow public surface should a separate future RFC or handoff
propose?
```

Expected initial posture:

```text
Not ready for public crates yet.
Ready for an evidence-based readiness audit.
```

---

## 2. Background

The project now has a substantial local visualization/reporting line:

```text
RFC-063: visual docs, canonical visual examples, local report-tool foundation
RFC-065: educational visualization positioning and learner path
RFC-068: local static HTML artifacts for all fixed demos
RFC-069: local static HTML for data-readiness input mode
```

These RFCs deliberately kept visualization out of public crates and out of core
`matten`:

```text
no public matten-report crate
no public matten-viz crate
no public report model
no reusable renderer API
no Tensor::plot() or Tensor::show()
no expression tracing
no autograd
no plotting dependency in published crates
```

After `0.36.0`, the local artifact line is useful enough to raise the next
product question, but not enough to answer it automatically. Local private
renderers and hand-authored report data do not by themselves prove a stable
public API, ownership boundary, dependency policy, compatibility promise, or
maintenance model.

---

## 3. Goals

1. Audit whether a future public `matten-report` or `matten-viz` crate is
   justified.
2. Identify whether a stable internal report data model exists or must be
   designed first.
3. Decide whether renderer internals should stay private to `tools/matten-report`.
4. Classify JSON, SVG, Vega-Lite, notebook/browser integration, and other output
   formats by readiness and risk.
5. Define dependency-policy questions before any public visualization crate is
   proposed.
6. Confirm that core `matten` should remain visualization-free unless a future
   RFC explicitly changes that boundary.
7. Produce a prerequisite list for any future implementation RFC or handoff.
8. Keep this audit separate from release preparation and public API changes.

---

## 4. Non-goals

This RFC does not authorize:

```text
[ ] implementation
[ ] public matten-report crate
[ ] public matten-viz crate
[ ] public report model API
[ ] reusable renderer API
[ ] workspace membership change for tools/matten-report
[ ] crates.io publishing
[ ] version bump, release prep, tag, or publish action
[ ] Tensor::plot(), Tensor::show(), or Tensor::backward()
[ ] expression tracing
[ ] autograd
[ ] JSON report output
[ ] SVG output
[ ] Vega-Lite output
[ ] JavaScript
[ ] external CSS, fonts, images, or network assets
[ ] notebook, browser, dashboard, GUI, or server integration
[ ] plotting dependency in core matten
[ ] new dependency in any published crate
[ ] generated checked-in report artifacts
[ ] performance-ranking charts
[ ] large-data or streaming visualization
[ ] project scanning, source-code analysis, or project mutation
```

If the audit recommends any of these, that recommendation still requires a
separate future RFC or reviewed handoff before implementation.

---

## 5. Audit Questions

### 5.1 Report Model Readiness

Review:

```text
tools/matten-report/src/main.rs
rfcs/handoffs/068-shared-educational-report-model-handoff.md
rfcs/handoffs/068-*-html-*.md
rfcs/handoffs/069-*.md
```

Questions:

```text
[ ] Is there a reusable report data model, or only private per-report structs?
[ ] Are report concepts stable across educational-path, shape-flow,
    dynamic-readiness, mlprep-standardization, data-readiness, and input mode?
[ ] Which concepts are stable enough to name publicly?
[ ] Which concepts are local-demo-only and should not become public API?
[ ] Would a public report model force premature compatibility promises?
```

### 5.2 Renderer Boundary Readiness

Review:

```text
tools/matten-report/src/main.rs
README.md
docs/src/examples/visual-understanding.md
docs/src/examples/data.md
docs/src/reference/compatibility.md
```

Questions:

```text
[ ] Are Markdown and HTML renderers cleanly separated from report computation?
[ ] Are HTML escaping and display bounds centralized enough for reuse?
[ ] Are renderer errors and output policies documented enough for public use?
[ ] Would public renderer APIs need snapshot/compatibility tests first?
[ ] Should renderers remain private until JSON or another structured output
    proves a stable report schema?
```

### 5.3 Crate Boundary Readiness

Review:

```text
rfcs/done/022-companion-crate-boundary-policy.md
rfcs/done/030-workspace-versioning-model-lockstep.md
rfcs/done/032-companion-dependency-and-import-convention.md
rfcs/done/063-visual-understanding-and-reporting.md
rfcs/done/068-rich-local-visualization-artifacts.md
rfcs/done/069-input-mode-html-report-policy.md
```

Questions:

```text
[ ] Would public reporting belong in a published companion crate, a local tool,
    or a workspace-excluded tool?
[ ] If published, would it share the lock-step family version?
[ ] Would it depend on `matten`, `matten-data`, `matten-mlprep`, or only accept
    plain report data?
[ ] Would it violate the dependency direction rules?
[ ] What maturity label would be honest at introduction?
```

### 5.4 Output Format Readiness

Classify these outputs:

```text
Markdown/plain text
static HTML
JSON report data
SVG
Vega-Lite
notebook/browser/dashboard integration
```

Questions:

```text
[ ] Which outputs are local-tool-only and already proven?
[ ] Which outputs would need a public schema?
[ ] Which outputs would require new dependencies?
[ ] Which outputs create security or generated-artifact review burden?
[ ] Is JSON a prerequisite before any public renderer crate?
```

### 5.5 Core Boundary Readiness

Review:

```text
rfcs/done/002-public-api-minimalism-and-tensor-contract.md
rfcs/done/022-companion-crate-boundary-policy.md
rfcs/done/063-visual-understanding-and-reporting.md
ROADMAP.md
```

Questions:

```text
[ ] Should core `matten` continue to expose no visualization APIs?
[ ] Would `Tensor::plot()` or `Tensor::show()` conflict with the core boundary?
[ ] Is expression tracing a separate computation-model RFC?
[ ] Is autograd a separate tensor-semantics RFC?
[ ] Are public report/viz crates sufficient if visualization later becomes public?
```

---

## 6. Expected Audit Outputs

The audit should produce:

```text
[ ] readiness verdict: not ready / conditionally ready / ready for narrow RFC
[ ] prerequisite list for any public crate or public renderer API
[ ] dependency-policy recommendation
[ ] report-model ownership recommendation
[ ] output-format readiness table
[ ] explicit deferral list
[ ] recommended next RFC or handoff, if any
```

The audit should prefer "not ready yet" unless it can name a concrete, narrow,
stable, testable public surface.

---

## 7. Initial Risk Assessment

| Risk | Treatment |
|---|---|
| Turning local helper structure into premature public API | Audit report model stability before any public crate |
| Dependency creep through plotting/rendering libraries | Require dependency-policy review before implementation |
| Confusing local artifacts with supported product surface | Keep current artifacts local-tool-owned until a future RFC says otherwise |
| Security drift through richer output formats | Keep JavaScript, network assets, data URLs, and generated checked-in artifacts unauthorized |
| Core scope creep | Keep `Tensor` visualization, expression tracing, and autograd separate RFC topics |
| Maintenance cost of a public crate | Require maturity label, compatibility policy, and test strategy before implementation |

---

## 8. Review Questions

Review should decide:

```text
[ ] Is RFC-070 correctly scoped as audit-only?
[ ] Is public `matten-report` / `matten-viz` readiness the right next visualization RFC?
[ ] Are the non-goals broad enough to prevent accidental implementation scope?
[ ] Are the audit questions sufficient to decide report model, renderer, crate, output, and core boundaries?
[ ] Should JSON report output be treated as a possible prerequisite for public renderer APIs?
[ ] Should the expected posture remain "not ready for public crates yet" unless the audit proves otherwise?
[ ] Are ROADMAP.md and rfcs/README.md the only tracking surfaces needed for this proposal?
```
