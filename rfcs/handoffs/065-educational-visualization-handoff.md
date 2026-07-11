# RFC-065 Educational Visualization Handoff

**Project:** `matten`  
**Related RFC:** RFC-065: Educational Visualization and Tensor Learning Path  
**Document kind:** Compact planning handoff  
**Status:** Implemented and reviewed; retained as the RFC-065 Phase 1 record
**Scope:** Positioning consistency audit, overclaim guard, and first educational visualization docs slice  

---

## 1. Summary

This handoff defines the first reviewable slice for RFC-065.

The slice is intentionally documentation-first:

```text
public positioning consistency audit
overclaim guard in release docs checks
small educational path through existing docs/examples
no new public API
no new core dependency
no public visualization crate
no generated image assets
```

The goal is to make the existing RFC-063 visual-understanding work legible as an
educational feature of `matten`, without changing the core computation boundary.

## 2. Reviewer Background

This handoff assumes the reviewer may not have followed the conversation that
created RFC-065.

The short background is:

```text
matten 0.29.0 shipped RFC-063 visual-understanding docs, examples, and local
tools/matten-report demos.

Those artifacts help users inspect shapes, axes, readiness, and preprocessing
effects in Markdown/plain-text form.

The project owner now wants the next visualization direction to support
educational use: helping users understand "how the expression draws" and "what
the data means."

The public README and crate docs now mostly use learning/teaching/small-workflow
positioning, but the review found that one rustdoc phrase drifted too far toward
production-adjacent "business workflows" language.
```

The first slice therefore combines two related actions:

```text
audit and reconcile the public positioning
make the existing visual-understanding path read as educational support
add one objective guard against future positioning drift
```

It does not attempt to design or implement a visualization product.

## 3. Review Overview

The reviewer should evaluate this handoff as a first-slice boundary, not as a
full implementation plan for every possible visualization idea.

The core review questions are:

```text
Is the project background clear enough to review from the document alone?
Is education a reasonable explicit use case within matten's existing scope?
Does the first slice avoid public API and dependency expansion?
Does the first slice objectively guard against positioning overclaims?
Does the handoff preserve RFC-063's deferred boundaries around matten-viz,
plotting, tracing, notebooks, and autograd?
Does the positioning change avoid both underclaiming and overclaiming?
```

Expected reviewer output:

```text
accept / reject / request changes on RFC-065 direction
accept / reject / request changes on this first implementation slice
identify any wording that could imply performance, scale, or visualization API
commitment beyond the intended scope
```

## 4. Authorized Direction

If accepted, implement the first slice by updating:

```text
README.md
crates/matten/README.md
crates/matten/src/lib.rs
docs/src/introduction.md
docs/src/philosophy.md
docs/src/tutorial/start-here.md
docs/src/examples/visual-understanding.md
scripts/check-release-docs.sh
```

Optional small additions:

```text
docs/src/SUMMARY.md
docs/src/examples/index.md
docs/src/reference/math.md
docs/src/reference/shape-ops.md
```

Only touch optional files when they improve navigation or remove stale wording.

## 5. Positioning Requirement

High-visibility text should say that `matten` is useful for:

```text
learning
teaching
PoC / early prototypes
small workflows
readable first versions before migration
```

High-visibility text should not add broader production-adjacent claims such as
`business workflows` unless a later RFC explicitly authorizes that vocabulary.

It must still say that `matten` is not intended to replace:

```text
ndarray
nalgebra
candle
full dataframe engines
ML frameworks
GPU/sparse/distributed array systems
```

Avoid wording that implies:

```text
PoC as the sole use case
throwaway numerical trials as the sole use case
toy-code positioning
production-performance leadership
business-critical or production-scale workflow claims
```

## 6. First Slice Acceptance Criteria

```text
[ ] RFC-065 exists under rfcs/proposed/
[ ] RFC index links RFC-065
[ ] root README positioning remains within learning/teaching/small-workflow scope
[ ] crate README positioning remains within learning/teaching/small-workflow scope
[ ] crate-level rustdoc positioning removes production-adjacent "business workflows" drift
[ ] mdBook introduction/philosophy remain within learning/teaching/small-workflow scope
[ ] visual-understanding examples remain clearly examples/local tooling only
[ ] one small worked educational shape/data explanation is present
[ ] release-doc guard checks positioning consistency and overclaim phrases
[ ] no public API is added
[ ] no dependency is added
[ ] mdBook build succeeds
```

## 7. Deferred Work

Later slices may consider:

```text
small annotated output examples
tools/matten-report learner demo
release-note wording for the next release family
```

Still not authorized:

```text
matten-viz public crate
matten-report public crate
Tensor::plot / Tensor::show
automatic expression tracing
SVG / HTML / notebook output
GUI or dashboard output
autograd or Tensor::backward
```

## 8. Suggested Review Focus

Review should focus on:

```text
whether the positioning is honest
whether the educational direction is useful
whether the deferred boundaries are strong enough
whether the first slice is too broad or too narrow
whether any wording accidentally implies performance or production-scale claims
```
