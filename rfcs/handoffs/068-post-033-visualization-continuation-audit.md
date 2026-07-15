# RFC-068 Post-0.33 Visualization Continuation Audit

**Project:** `matten`
**Related RFCs:** RFC-063, RFC-065, RFC-068
**Document kind:** Readiness audit and next-slice recommendation
**Status:** Draft for review; does not authorize implementation
**Date:** 2026-07-15

---

## 1. Summary

RFC-068 can continue after `0.33.0`, but only through another narrow local
artifact slice.

The `0.32.0` and `0.33.0` releases proved the current local static HTML pattern
for three fixed `tools/matten-report` demos:

```text
tools/matten-report --demo educational-path --format html --output <path>
tools/matten-report --demo shape-flow --format html --output <path>
tools/matten-report --demo dynamic-readiness --format html --output <path>
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
Open one follow-up handoff for local HTML on the mlprep-standardization report.
Do not open public visualization crates or core visualization APIs yet.
Do not implement directly from this audit; require handoff review first.
```

---

## 2. Current Repository State

The repository records RFC-068 as implemented for:

```text
0.32.0: educational-path HTML and shape-flow HTML
0.33.0: dynamic-readiness HTML
```

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
dynamic-readiness: supported
data-readiness: not supported
mlprep-standardization: not supported
input mode: not supported
```

The supported HTML artifacts are local-only, explicit-file outputs. Markdown
remains the default format.

---

## 3. Continuation Candidates

| Candidate | Readiness | Recommendation |
|---|---|---|
| `mlprep-standardization` HTML | High | Best next slice |
| `data-readiness` demo HTML | Medium | Defer until the remaining fixed-demo HTML value is clear |
| input-mode HTML | Low | Defer; user-controlled CSV/report data expands escaping, table-size, and output-policy review |
| shared HTML renderer cleanup | Medium | Allow only if needed by the selected slice |
| SVG output | Low | Future RFC only |
| Vega-Lite / JSON report output | Low | Future RFC only |
| public `matten-report` crate | Low | Future RFC only after repeated local value and stable data-model boundaries |
| public `matten-viz` crate | Low | Future RFC only; not justified by current local-tool evidence |
| expression tracing / core visualization APIs | Low | Future RFC only; crosses the core API boundary |

---

## 4. Recommended Next Slice

Draft a compact implementation handoff for:

```text
RFC-068 mlprep-standardization local HTML artifact
```

Accepted command shape should be:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --demo mlprep-standardization \
  --format html \
  --output target/matten-report-mlprep-standardization.html
```

The existing Markdown/plain-text output must remain valid and default:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo mlprep-standardization
```

Why this candidate first:

```text
mlprep-standardization is fixed demo-only
it has no input-file mode
it already explains a before/after educational transformation
before/after means and standard deviations benefit from table comparison
it exercises matten-mlprep without changing any published dependency graph
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

## 6. Why Not Data-Readiness Or Input-Mode HTML Yet

`data-readiness` is useful, but it is closer to user-provided data workflows.
Even a demo-only HTML slice would invite questions about table size, CSV-like
escaping, schema display, and whether input-mode HTML should follow.

`mlprep-standardization` avoids those issues. It stays fixed-demo-only and has
a clearer visual payoff: compare raw values, standardized values, before means,
after means, before standard deviations, after standard deviations, and unchanged
shape.

Input-mode HTML remains a larger policy decision because report contents can be
user-controlled and potentially large.

---

## 7. Why Not Public Visualization Yet

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

The current evidence supports one more local artifact. It does not support a
published crate or core API.

---

## 8. Review Questions

Review should decide:

```text
[ ] Is mlprep-standardization the right next local HTML candidate?
[ ] Should data-readiness demo HTML come before mlprep-standardization?
[ ] Should input-mode HTML remain deferred?
[ ] Are the public crate/API deferrals still correct?
[ ] Is a new implementation handoff enough, or does the next slice require a new RFC?
```

---

## 9. Audit Verdict

```text
READY TO DRAFT A NARROW MLPREP-STANDARDIZATION HTML HANDOFF
NOT READY FOR DIRECT IMPLEMENTATION
NOT READY FOR PUBLIC REPORT/VIZ CRATES
NOT READY FOR CORE VISUALIZATION APIS
```

Next action if this audit is accepted:

```text
Draft RFC-068 mlprep-standardization local HTML artifact handoff for review.
```
