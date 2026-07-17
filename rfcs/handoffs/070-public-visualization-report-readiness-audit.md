# RFC-070 Public Visualization and Report Readiness Audit

**Project:** `matten`
**Related RFCs:** RFC-022, RFC-030, RFC-063, RFC-065, RFC-068, RFC-069, RFC-070
**Document kind:** Readiness audit and next-step recommendation
**Status:** Drafted for review; no implementation authorized
**Date:** 2026-07-17

---

## 1. Summary

RFC-070 asks whether the local `tools/matten-report` artifacts are mature enough
to justify a future public `matten-report` or `matten-viz` product surface.

Audit verdict:

```text
Not ready for public report/viz crates yet.
Ready to keep local artifacts as private tool-owned implementation.
Ready to consider a future JSON report-schema policy audit if maintainers want
to move toward public renderer APIs later.
```

This audit does not authorize implementation, a public crate, a public report
model, a reusable renderer API, JSON/SVG/Vega-Lite output, notebook/browser
integration, core `Tensor` visualization APIs, expression tracing, autograd,
dependency changes, release prep, tags, publishing, or generated artifacts.

---

## 2. Evidence Reviewed

Primary evidence:

```text
tools/matten-report/src/main.rs
rfcs/proposed/070-public-visualization-report-readiness-audit.md
rfcs/done/063-visual-understanding-and-reporting.md
rfcs/done/065-educational-visualization-and-tensor-learning-path.md
rfcs/done/068-rich-local-visualization-artifacts.md
rfcs/done/069-input-mode-html-report-policy.md
rfcs/handoffs/068-shared-educational-report-model-handoff.md
rfcs/handoffs/069-post-036-input-mode-html-closure-audit.md
ROADMAP.md
rfcs/README.md
```

Observed code shape:

```text
tools/matten-report is a workspace-excluded local binary tool.
tools/matten-report/src/main.rs has no public Rust API surface.
Report data structs are private and report-specific.
Markdown and HTML rendering live in private functions in one binary file.
HTML escaping and display bounds exist as private helpers.
Static HTML is supported for five fixed demos and one data-readiness input mode.
No JSON, SVG, Vega-Lite, notebook/browser, public crate, or core visualization API exists.
```

The local tool has proven useful as a maintained educational/reporting artifact.
It has not yet proven a stable public report model or renderer contract.

---

## 3. Report Model Readiness

Current state:

```text
educational-path: private EducationalPathReportData family
shape-flow: private ShapeFlowReportData family
dynamic-readiness: private DynamicReadinessReportData family
mlprep-standardization: private MlprepStandardizationReportData family
data-readiness fixed demo: private DataReadinessReportData
data-readiness input mode: private InputDataReadinessReportData
```

These models are useful internal shapes, but they are not one public report
schema. They mix several concepts:

```text
teaching narrative
shape-flow checkpoints
dynamic readiness masks
ML preprocessing before/after summaries
CSV schema and numeric-conversion summaries
bounded tensor previews
```

Stable enough for public naming today:

```text
local report family names
Markdown/plain text as default output
explicit file output for HTML
bounded previews for user-controlled input
HTML escaping requirement
```

Not stable enough for public API today:

```text
one cross-family Report enum
one public section/block schema
one public tensor-preview schema
one public diagnostic schema for conversion failures
one public compatibility promise for renderer inputs
```

Conclusion:

```text
No public report model should be introduced yet.
The first prerequisite for public renderer APIs is either a private shared
report schema extraction or a JSON-schema policy audit that proves the stable
shape of report data before publication.
```

---

## 4. Renderer Boundary Readiness

Current state:

```text
Markdown renderers and HTML renderers are private functions.
Some reports share private data builders; others still have family-specific
rendering paths.
HTML output uses private `html_escape`, `write_html_pre`, and
`write_shape_flow_table` helpers.
Input-mode HTML has private display caps for columns, fields, errors, and
tensor previews.
Exact snapshot tests and static/self-contained safety tests cover the local
HTML artifacts.
```

Strengths:

```text
no JavaScript
no external assets
no network references
explicit --output for HTML
escaping helper exists
display bounds exist for user-controlled input mode
local snapshot coverage exists
```

Gaps before public renderer APIs:

```text
renderer inputs are private and not versioned
HTML style/structure is not a compatibility contract
escaping/display policy is private tool behavior, not a public API contract
Markdown and HTML output compatibility is not specified as public surface
renderer errors are not modeled as a public error type
```

Conclusion:

```text
Keep renderers private to tools/matten-report.
Do not publish reusable renderer APIs until renderer input schema, output
compatibility, escaping policy, bounds policy, and error model are separately
reviewed.
```

---

## 5. Crate Boundary Readiness

Current boundary:

```text
core matten has no dependency on reporting or visualization code
published companion crates do not depend on tools/matten-report
tools/matten-report depends on matten, matten-data, and matten-mlprep locally
tools/matten-report is not a published crate and is not a workspace member
```

A future public crate could be shaped several ways:

| Candidate | Readiness | Notes |
|---|---|---|
| Keep `tools/matten-report` local only | High | Current proven model; lowest maintenance and dependency risk |
| Published `matten-report` crate | Low | Needs report schema, public error model, maturity label, dependency policy |
| Published `matten-viz` crate | Low | Needs renderer model and output-format decisions first |
| Workspace-excluded `tools/matten-viz` | Medium-low | Could remain local, but still needs a concrete use case |
| Core `Tensor` visualization APIs | Not ready | Crosses core boundary and should remain separate |

Recommended crate posture:

```text
Do not create public report/viz crates now.
Keep local tools workspace-excluded and publish=false.
If a public crate is later proposed, start with an explicit crate-boundary RFC
that decides dependency direction, lock-step versioning, maturity label, public
API snapshot policy, and whether the crate accepts report data or computes it.
```

---

## 6. Output Format Readiness

| Output | Current readiness | Recommendation |
|---|---|---|
| Markdown/plain text | Proven local default | Keep as local default |
| Static HTML | Proven for local fixed demos and one input-mode path | Keep local-tool-owned |
| JSON report data | Plausible prerequisite | Consider separate policy audit before public renderer APIs |
| SVG | Not ready | Defer; needs schema/security/asset policy |
| Vega-Lite | Not ready | Defer; dependency and product-surface questions |
| Notebook/browser/dashboard | Not ready | Defer; different product surface |

JSON deserves separate attention because it could force report-schema clarity
without committing to a public renderer crate. It should still be treated as a
future RFC or handoff, not as an implementation detail of this audit.

---

## 7. Core Tensor Boundary

Core `matten` should remain visualization-free.

Still deferred:

```text
Tensor::plot()
Tensor::show()
Tensor::backward()
expression tracing / operation graph visualization
autograd
lazy graph construction
plotting dependencies in core
```

The existing `Tensor::trace()` is a linear-algebra diagonal-sum helper and is
not expression tracing. Any future expression-tracing feature should use a
separate computation-model RFC and should not reuse the existing linalg meaning.

Conclusion:

```text
Public visualization, if it ever happens, should live outside core matten.
Core should continue to expose tensor computation and boundary APIs only.
```

---

## 8. Readiness Verdict

| Boundary | Verdict |
|---|---|
| Report model | Not ready for public API |
| Renderer boundary | Not ready for public API |
| Crate boundary | Not ready for public crate |
| Output formats | Markdown/HTML proven locally; JSON/SVG/Vega-Lite not authorized |
| Core Tensor boundary | Keep closed to visualization |

Overall:

```text
RFC-070 should not move directly to implementation.
The project should keep tools/matten-report local and private.
The most useful next visualization step, if any, is a JSON report-schema policy
audit or private report-model extraction handoff.
```

Recommended next candidates:

| Candidate | Recommendation |
|---|---|
| JSON report-schema policy audit | Best next visualization candidate if maintainers want public readiness progress |
| Private report-model extraction inside `tools/matten-report` | Useful only if it reduces duplication without public API |
| Public `matten-report` crate | Defer |
| Public `matten-viz` crate | Defer |
| Core visualization APIs | Defer |

---

## 9. Review Questions

Review should decide:

```text
[ ] Is "not ready for public report/viz crates yet" the correct verdict?
[ ] Is JSON report-schema policy the right next visualization candidate if work continues?
[ ] Should private report-model extraction be considered before any JSON/public work?
[ ] Are public `matten-report` and `matten-viz` correctly deferred?
[ ] Are core `Tensor` visualization, expression tracing, and autograd correctly deferred?
[ ] Are ROADMAP.md, rfcs/README.md, and the handoff index sufficient tracking surfaces for this audit?
```
