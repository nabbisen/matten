# RFC-068 Post-0.32 Visualization Continuation Audit

**Project:** `matten`
**Related RFCs:** RFC-063, RFC-065, RFC-068
**Document kind:** Readiness audit and next-slice recommendation
**Status:** Draft for review; does not authorize implementation
**Date:** 2026-07-15

---

## 1. Summary

RFC-068 can continue, but only through another narrow local-artifact slice.

The `0.32.0` release completed the currently authorized rich-local-visualization
scope:

```text
tools/matten-report --demo educational-path --format html --output <path>
tools/matten-report --demo shape-flow --format html --output <path>
```

The remaining visualization ideas are still intentionally deferred:

```text
public matten-report crate
public matten-viz crate
core visualization APIs
Tensor::plot / Tensor::show / Tensor::trace
automatic expression tracing
autograd
SVG
Vega-Lite
JSON report output
browser UI / notebook / dashboard
```

Audit recommendation:

```text
Open one follow-up handoff for local HTML on the dynamic-readiness report.
Do not open public visualization crates or core visualization APIs yet.
Do not implement directly from this audit; require handoff review first.
```

---

## 2. Current Repository State

The repository records RFC-068 as implemented for `0.32.0`.

Current local `tools/matten-report` report families:

```text
data-readiness
shape-flow
dynamic-readiness
mlprep-standardization
educational-path
```

Current HTML support:

```text
educational-path: supported
shape-flow: supported
data-readiness: not supported
dynamic-readiness: not supported
mlprep-standardization: not supported
input mode: not supported
```

The supported HTML artifacts are local-only, explicit-file outputs. Markdown
remains the default format.

---

## 3. Continuation Candidates

| Candidate | Readiness | Recommendation |
|---|---|---|
| `dynamic-readiness` HTML | High | Best next slice |
| `data-readiness` demo HTML | Medium | Defer until input-mode policy is settled |
| `mlprep-standardization` HTML | Medium | Defer until dynamic-readiness proves the second non-shape report pattern |
| input-mode HTML | Low | Defer; user-controlled CSV/report data expands escaping and output-policy review |
| shared HTML renderer cleanup | Medium | Allow only if needed by the selected slice |
| SVG output | Low | Future RFC only |
| Vega-Lite / JSON report output | Low | Future RFC only |
| public `matten-report` crate | Low | Future RFC only after repeated local value |
| public `matten-viz` crate | Low | Future RFC only; not justified by current local-tool evidence |
| expression tracing / core visualization APIs | Low | Future RFC only; crosses core API boundary |

---

## 4. Recommended Next Slice

Draft a compact implementation handoff for:

```text
RFC-068 dynamic-readiness local HTML artifact
```

Accepted command shape should be:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --demo dynamic-readiness \
  --format html \
  --output target/matten-report-dynamic-readiness.html
```

The existing Markdown/plain-text output must remain valid and default:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo dynamic-readiness
```

Why this candidate first:

```text
dynamic-readiness is fixed demo-only
it already explains visual/data meaning rather than only shape mechanics
it has no input-file mode
it exercises dynamic values, missing-value masks, and explicit numeric readiness
it can reuse the existing static HTML policy without public API expansion
```

---

## 5. Required Boundaries

The next handoff must keep these constraints:

```text
local tool only
Markdown/plain text remains default
HTML requires explicit --output
HTML is static, deterministic, self-contained, and UTF-8
no JavaScript
no external assets
no data URLs
no network
no telemetry
no generated report checked into version control
no public API
no published crate
no workspace membership change for tools/matten-report
no new dependency in published crates
no source-code scanning
no project mutation
```

The next handoff must not authorize:

```text
HTML for data-readiness
HTML for mlprep-standardization
input-mode HTML
SVG
Vega-Lite
JSON output
Tensor::plot / Tensor::show / Tensor::trace / Tensor::backward
automatic expression tracing
lazy expression graphs
autograd
public matten-report crate
public matten-viz crate
```

---

## 6. Why Not Public Visualization Yet

The local artifacts are proving that visual explanation is useful. They do not
yet prove a stable public API surface.

A public visualization crate would need answers that the repository has not yet
settled:

```text
stable report data model boundaries
versioned output contract
renderer format ownership
dependency policy for visualization backends
how much of matten-report becomes user-facing API
whether reports are educational examples or product features
```

The current evidence supports another local artifact. It does not support a
published crate or core API.

---

## 7. Review Questions

Review should decide:

```text
[ ] Is dynamic-readiness the right next local HTML candidate?
[ ] Should data-readiness demo HTML come before dynamic-readiness?
[ ] Should input-mode HTML remain deferred?
[ ] Are the public crate/API deferrals still correct?
[ ] Is a new implementation handoff enough, or does the next slice require a new RFC?
```

---

## 8. Audit Verdict

```text
READY TO DRAFT A NARROW DYNAMIC-READINESS HTML HANDOFF
NOT READY FOR DIRECT IMPLEMENTATION
NOT READY FOR PUBLIC REPORT/VIZ CRATES
NOT READY FOR CORE VISUALIZATION APIS
```

Next action if this audit is accepted:

```text
Draft RFC-068 dynamic-readiness local HTML artifact handoff for review.
```
