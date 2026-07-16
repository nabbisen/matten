# RFC-068 Post-0.35 Fixed-Demo HTML Closure Audit

**Project:** `matten`
**Related RFCs:** RFC-063, RFC-065, RFC-068
**Document kind:** Closure audit and next-theme recommendation
**Status:** Drafted for review; no implementation authorized
**Date:** 2026-07-16

---

## 1. Summary

After `0.35.0`, RFC-068 has completed local static HTML output for every fixed
`tools/matten-report` demo:

```text
tools/matten-report --demo educational-path --format html --output <path>
tools/matten-report --demo shape-flow --format html --output <path>
tools/matten-report --demo dynamic-readiness --format html --output <path>
tools/matten-report --demo mlprep-standardization --format html --output <path>
tools/matten-report --demo data-readiness --format html --output <path>
```

Audit recommendation:

```text
Close the RFC-068 fixed-demo local HTML line after 0.35.0.
Do not keep adding visualization work under RFC-068 by default.
Treat input-mode HTML, public report/viz crates, SVG/Vega-Lite/JSON output,
expression tracing, and core visualization APIs as separate future RFC or
handoff decisions.
```

This audit is a planning/status record only. It does not authorize code changes,
release metadata changes, tags, publishing, public APIs, or new dependencies.

---

## 2. Current Repository State

`tools/matten-report` currently supports five fixed demo report families:

```text
educational-path
shape-flow
dynamic-readiness
mlprep-standardization
data-readiness
```

HTML support after `0.35.0` is:

```text
educational-path: supported
shape-flow: supported
dynamic-readiness: supported
mlprep-standardization: supported
data-readiness: supported
input mode: rejected
```

Markdown/plain text remains the default format. HTML remains explicit-file-only,
local-only, deterministic, static, self-contained, and generated on demand.

The completed fixed-demo series provides visual understanding for:

```text
learner shape/data path
shape flow and broadcasting/reshape/reduction/matmul shape checkpoints
dynamic-readiness boundaries
ML preprocessing standardization flow
data-readiness CSV-to-tensor success path
```

---

## 3. Why The Fixed-Demo Line Should Close

RFC-068 was intentionally scoped as a conservative visualization advance:

```text
local tool only
fixed demo data only
no public API
no published report/viz crate
no published-crate dependency change
no JavaScript or external assets
no core Tensor visualization methods
no expression tracing or autograd
```

The local fixed-demo HTML goal is now saturated. Every existing fixed demo has a
reviewed HTML artifact, and the remaining gaps are qualitatively different from
the original fixed-demo scope.

Continuing feature work under the same RFC without a new decision would blur
the boundary between:

```text
small deterministic educational artifacts
user-controlled input reporting
public renderer/product surface
core expression/operation visualization
```

The project should therefore record RFC-068 as complete for fixed-demo local
HTML and require a separate review before expanding into any of those larger
areas.

---

## 4. Remaining Candidates And Recommended Treatment

| Candidate | Readiness | Recommendation |
|---|---|---|
| Fixed-demo local HTML | Complete | Close this line after `0.35.0` |
| Input-mode HTML for CSV reports | Low-medium | Future policy handoff only; do not implement from RFC-068 closure |
| Public `matten-report` crate | Low | Future RFC only after stable report-model ownership exists |
| Public `matten-viz` crate | Low | Future RFC only; do not infer from local HTML artifacts |
| SVG output | Low | Future RFC/handoff only; not a continuation default |
| Vega-Lite / JSON report output | Low | Future RFC/handoff only |
| Notebook/browser/dashboard integration | Low | Future RFC only |
| Core visualization APIs (`Tensor::plot`, `Tensor::show`) | Low | Future RFC only; crosses core API boundary |
| Expression tracing / operation graph visualization | Low | Future RFC only; crosses computation-model boundary |
| Autograd / `backward()` | Low | Future RFC only; separate tensor semantics and API design |

---

## 5. Why Input-Mode HTML Is Not A Cleanup Item

Input-mode HTML is not just "HTML for the last remaining report path." After
`0.35.0`, the last remaining unsupported HTML path accepts user-controlled CSV
data:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --input tools/matten-report/fixtures/small.csv \
  --kind data-readiness \
  --select sales,cost \
  --format html \
  --output target/matten-report-input.html
```

That shape needs a separate policy because it raises questions the fixed-demo
line deliberately avoided:

```text
large tables and output-size limits
column-name/value escaping coverage for arbitrary input
whether raw source values are rendered or summarized
HTML rendering for failure cases
whether input-mode reports become a general report product
whether generated files need stronger path/output rules
```

The correct status after this closure is therefore:

```text
input-mode HTML remains rejected until a separate review accepts it
```

---

## 6. Recommended Tracker Updates

If this audit is accepted, the durable project records should say:

```text
RFC-068 fixed-demo local HTML line: complete after 0.35.0
RFC-068: closed for the local fixed-demo artifact scope
input-mode HTML: deferred; separate policy handoff required
public report/viz crates: deferred; separate RFC required
core visualization APIs / expression tracing / autograd: deferred; separate RFC required
```

No current-family install snippets, crate versions, release notes, or public API
docs need to change for this audit.

---

## 7. Review Questions

Review should decide:

```text
[ ] Should the RFC-068 fixed-demo local HTML line close after 0.35.0?
[ ] Is input-mode HTML correctly treated as a separate future policy decision?
[ ] Are public report/viz crates and core visualization APIs still correctly deferred?
[ ] Should the next visualization action require a fresh RFC/handoff rather than continuing RFC-068 automatically?
[ ] Are ROADMAP.md, rfcs/README.md, and RFC-068 status tracking sufficient for this closure record?
```
