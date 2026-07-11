# RFC-065: Educational Visualization and Tensor Learning Path

**Status:** Proposed  
**Target:** v0.30+ docs/examples planning; implementation requires accepted handoff(s)  
**Theme:** Educational tensor understanding, visual learning aids, and honest positioning  
**Depends on:** RFC-014, RFC-021, RFC-049, RFC-050, RFC-063  
**Related:** RFC-054, RFC-060, RFC-064  

---

## 1. Summary

This RFC proposes the next visualization direction after RFC-063: make `matten`
more useful for learning and teaching tensor-shaped computation.

The project should keep presenting `matten` as a small, dependency-light,
developer-experience-first tensor library. That scope is not only for disposable
proofs of concept. It is also suitable for:

```text
learning tensor shapes
teaching axis and broadcasting behavior
explaining small numerical transformations
inspecting small data-readiness problems
building readable Rust examples before moving to heavier ecosystems
```

The first work should remain documentation, examples, and local generated
artifacts. This RFC does not authorize a public `matten-viz` crate, plotting
dependency, automatic expression tracer, GUI, notebook integration, or runtime
visualization API.

---

## 2. Reviewer Overview

This RFC is intended to be reviewable without reading the project discussion
that led to it.

The current review question is not "should `matten` become a visualization
library?" The current review questions are:

```text
Does the already-adopted learning/teaching/small-workflows positioning remain
consistent and non-overclaiming across the current public docs?

Should the next follow-up work be docs/examples/local artifacts that explain
shapes, axes, and small data transformations?
```

The requested review should confirm:

```text
[ ] the background accurately reflects the current 0.29 project state
[ ] the existing educational positioning fits matten's family-car scope
[ ] the first slice is appropriately small and objectively checkable
[ ] the boundaries against public plotting/viz/autograd scope are strong enough
[ ] the positioning update does not overclaim production performance or scale
```

---

## 3. Background

`matten` began as a small, developer-experience-first tensor crate for Rust: one
primary `Tensor` type, low type-system burden, readable shape/data operations,
and explicit migration paths when users outgrow the family-car scope.

Several earlier RFC lines are relevant to this proposal:

```text
RFC-014 / RFC-021  examples and learning path
RFC-049            benchmark/positioning work, including "PoC, learning, and small workflows"
RFC-050            migration guide: outgrowing matten is a successful PoC outcome
RFC-060            surfaced benchmark evidence without performance-ranking claims
RFC-063            visual understanding and reporting
RFC-064            workspace dependency maintenance policy
```

RFC-063 shipped the first visual-understanding program in the `0.29.0` release
family. It added visual docs, example reports, and local `tools/matten-report`
demos for shape flow, dynamic readiness, data readiness, and standardization
readiness. It deliberately kept this work as docs/examples/local tooling:

```text
no Tensor::plot()
no public visualization API
no public matten-viz crate
no plotting dependency in core matten
no expression tracing
no autograd
```

The result is that `matten` now has enough explanatory material to be useful not
only for early prototypes, but also for learners and teachers who want a small
Rust-native way to understand tensor-shaped computation.

High-visibility docs in the current tree already use the newer
learning/teaching/small-workflows positioning. Therefore the first RFC-065 slice
must not pretend that the project is introducing that language for the first
time. Its job is to audit the live wording for consistency, remove any
production-adjacent drift, and add a small concrete educational improvement.

---

## 4. Prospect

The near-term prospect is an educational layer around already accepted APIs:

```text
shape and axis explanation
broadcasting diagrams
before/after summaries for small transforms
readiness summaries for messy data
guided example order for learners
clear migration guidance when matten is no longer enough
```

The longer-term prospect, if early slices prove valuable, is optional local
artifact generation that helps users see "how the expression draws" or "what the
data means." That prospect still starts outside the published crate graph.

Potential later artifacts may include:

```text
annotated Markdown reports
terminal-first shape/data summaries
small static examples for teaching material
local report demos in tools/matten-report
```

Public `matten-viz` or `matten-report` crates remain a separate future decision,
not an outcome authorized by this RFC.

---

## 5. Motivation

`matten` now has enough accepted core APIs and explanatory examples to serve an
educational role. Users can learn shape operations, broadcasting, reductions,
dynamic ingestion, and preprocessing without immediately entering a larger
ecosystem.

RFC-063 made visual understanding concrete through docs, example reports, and a
local `matten-report` tool. The next question is how to make that work coherent
for learners:

```text
How does this expression draw?
Which axis disappeared?
What data changed?
Which values are missing, text, numeric, or outlying?
Why does this operation produce that shape?
When should I stop using matten and migrate?
```

Answering these questions improves `matten`'s educational value without changing
its core computation model.

---

## 6. Goals

1. Audit high-visibility positioning so `matten` consistently stays within PoC,
   learning, teaching, and small-workflow scope.
2. Build a coherent educational path around existing APIs before adding new
   tooling.
3. Prefer small static visual artifacts, Markdown tables, terminal summaries,
   and copyable examples.
4. Keep all educational visualization outside the core public API.
5. Reuse the RFC-063 `tools/matten-report` pattern for local artifacts when
   needed: workspace-excluded, `publish = false`, deterministic output.
6. Keep migration guidance visible so educational success does not become an
   overclaim about performance, scale, autograd, or production analytics.

---

## 7. Non-goals

This RFC does not authorize:

```text
[ ] Tensor::plot(), Tensor::show(), Tensor::trace(), or Tensor::backward()
[ ] public matten-viz or matten-report crates
[ ] plotting dependencies in core matten
[ ] browser UI, GUI, dashboard, or notebook requirements
[ ] automatic expression tracing through operators
[ ] lazy expression graphs or autograd
[ ] dataframe, ML-framework, or symbolic-math scope
[ ] performance-claim charts
[ ] visualization of large data sets
[ ] network access or telemetry in local tools
```

If a public visualization crate becomes desirable, it must be proposed by a
separate RFC or explicit amendment after the local/reporting value is proven.

---

## 8. Positioning Audit

The project should keep already-adopted wording that avoids implying `matten` is
only for PoC trials. The preferred public positioning is:

```text
matten is a family-car tensor library for readable Rust numerical work,
learning, teaching, small workflows, and early prototypes.
```

Allowed wording:

```text
PoC, learning, and small workflows
learning and teaching tensor-shaped computation
small serious workflows
time to first understanding
time to a runnable PoC
```

Avoid bare wording that narrows or over-broadens the project:

```text
PoC as the sole use case
throwaway numerical trials as the sole use case
toy-code positioning
language that implies the crate is not useful for real small workflows
business-critical or production-scale workflow claims
production-performance claims
speed or benchmark-ranking claims
```

This is a positioning clarification, not a scope expansion into high-performance
or large-scale numerical computing.

---

## 9. Candidate Work

### 9.1 Educational landing path

Add or refine a short path through the mdBook that answers:

```text
I have a matrix. How do I know what shape each operation produces?
I have messy CSV/JSON data. How do I know whether it can become numeric?
I have standardized data. How do I see what changed?
I am learning tensors. Which examples should I run first?
```

This can build on:

```text
docs/src/tutorial/start-here.md
docs/src/examples/visual-understanding.md
docs/src/reference/shape-ops.md
docs/src/reference/math.md
docs/src/reference/dynamic.md
docs/src/migration/index.md
```

### 9.2 Static visual artifacts

Prefer text-first artifacts:

```text
ASCII shape diagrams
Markdown tables
small terminal reports
before/after summaries
annotated example output
```

Generated images, SVG, HTML, Vega-Lite JSON, or notebook output remain deferred.

### 9.3 Expression-shape explanation

Useful near-term examples:

```text
(&a + &b) shape alignment
reshape / flatten / transpose
mean_axis(0) vs mean_axis(1)
matmul shape flow
standardize_columns before/after
dynamic numeric-readiness mask
```

The implementation should explain shapes and data effects directly. It should
not require expression capture or operator instrumentation.

### 9.4 Local artifact generation

If a handoff authorizes more tooling, it should extend local tooling before
public crates:

```text
tools/matten-report --demo educational-path
tools/matten-report --demo expression-shapes
```

These are examples of possible report families, not approved command names.

---

## 10. Packaging Policy

Core `matten` remains dependency-light and computation-focused.

Any educational report or visualization tooling should start as:

```text
tools/<name>
publish = false
workspace-excluded unless explicitly justified
no dependency added to core matten
no public API promise
```

The project should not introduce `crates/matten-viz` or `crates/matten-report`
until a later review establishes:

```text
stable user value
clear public API boundary
dependency policy
security and file-output behavior
maintenance owner
documentation and release gates
```

---

## 11. Acceptance Criteria For The First Slice

The first implementation slice should be small enough for direct review:

```text
[ ] audit public positioning language in README / crate docs / mdBook entrance
[ ] reconcile any wording that exceeds the allowed vocabulary
[ ] add or refine one concrete educational visual-understanding path or worked example
[ ] add no new public API
[ ] add no new dependency to core matten
[ ] add no generated binary assets
[ ] keep migration/outgrowing guidance visible
[ ] include mdBook or doc checks in the review evidence
[ ] add an objective positioning/overclaim check to the release-doc guard
```

---

## 12. Open Questions

1. Should the first educational path be part of `Tutorial`, `Examples`, or both?
2. Should `tools/matten-report` gain a learner-oriented demo, or should the next
   slice remain documentation-only?
3. Should the project eventually reserve the `matten-viz` crate name, or wait
   until a real public API is justified?
