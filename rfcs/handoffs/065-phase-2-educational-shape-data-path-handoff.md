# RFC-065 Phase 2 Educational Shape/Data Path Handoff

**Project:** `matten`  
**Related RFC:** RFC-065: Educational Visualization and Tensor Learning Path  
**Document kind:** Compact implementation handoff  
**Status:** Implemented and reviewed; retained as the RFC-065 Phase 2 record
**Scope:** Documentation/examples only; educational shape and data-understanding path  

---

## 1. Summary

This handoff defines the next small implementation slice after the accepted
RFC-065 first slice.

Phase 1 established the positioning guardrails:

```text
learning / teaching / small workflows are in scope
production-scale and performance-leadership claims are out of scope
no public visualization API
no new dependency
no public matten-viz or matten-report crate
```

Phase 2 should now add concrete educational value by making a few common tensor
questions visually understandable in the mdBook and existing examples path.

The implementation should be docs/examples only:

```text
ASCII diagrams
Markdown tables
worked shape/data questions
links from the learning path
no runtime code unless fixing a doc/example typo requires it
```

## 2. Reviewer Background

RFC-063 shipped the first visual-understanding program in the `0.29.0` family:
visual docs, example summaries, and local `tools/matten-report` demos. RFC-065
then clarified that this work also supports educational use.

The first RFC-065 slice was reviewed and accepted as a good commit point. It:

```text
audited high-visibility positioning
removed production-adjacent rustdoc wording
added a release-doc overclaim guard
added one worked mean_axis(0) vs mean_axis(1) shape question
```

The next slice should not revisit that positioning decision. It should answer:

```text
Which few visual explanations would help a learner understand matten faster?
Where should they live so users can find them?
Can this be done without adding API, dependencies, or generated assets?
```

## 3. Goals

1. Strengthen the educational path through existing docs.
2. Add small worked explanations for common shape/data misunderstandings.
3. Keep the visual style text-first and reviewable.
4. Keep all work out of the published API surface.
5. Preserve RFC-063/RFC-065 guardrails around visualization, reporting, tooling,
   and performance claims.

## 4. Non-goals

This slice does not authorize:

```text
[ ] Tensor::plot(), Tensor::show(), Tensor::trace(), or Tensor::backward()
[ ] automatic expression tracing
[ ] lazy expression graphs
[ ] public matten-viz or matten-report crates
[ ] new tools/matten-report demos
[ ] SVG, HTML, Vega-Lite, JSON, notebook, GUI, or dashboard output
[ ] generated image/binary assets
[ ] new runtime dependencies
[ ] core algorithm/API changes
[ ] benchmark charts or speed/ranking claims
[ ] release version bump
```

Local `tools/matten-report` expansion is deliberately deferred. This slice should
prove the educational content shape in docs first.

`Phase 2` is an internal RFC/handoff planning label only. Do not use `Phase 1`,
`Phase 2`, `Phase-1`, or `Phase-2` wording in any `docs/src/**` page or
`crates/matten/examples/**` text added by this slice; the existing release-doc
guard rejects retired phase vocabulary in user-facing docs and examples.

## 5. Authorized Files

Primary files:

```text
docs/src/tutorial/start-here.md
docs/src/examples/visual-understanding.md
docs/src/reference/shape-ops.md
docs/src/reference/math.md
docs/src/reference/dynamic.md
```

Optional navigation/index files:

```text
docs/src/examples/index.md
docs/src/SUMMARY.md
```

Process files, only if the accepted implementation needs to document or guard
the slice:

```text
scripts/check-release-docs.sh
rfcs/done/063-visual-understanding-and-reporting.md
rfcs/done/065-educational-visualization-and-tensor-learning-path.md
```

Do not edit version files, changelog, Cargo manifests, or source APIs for this
slice unless a reviewer explicitly changes the scope.

## 6. Required Educational Increments

Implement at least three of the following small worked explanations.

### 6.1 Broadcasting shape alignment

Explain one broadcast operation by aligning shapes from the right:

```text
[3, 1]
[1, 4]
-----
[3, 4]
```

The explanation should answer:

```text
Which dimension expands?
Which shape is kept?
What output shape should I expect before reading the values?
```

### 6.2 Reshape / flatten / transpose

Explain the difference between changing grouping and changing axis meaning:

```text
reshape / flatten: preserve row-major tape, change grouping
transpose: move coordinates to different axes
```

Use one compact ASCII example. Do not introduce view/lifetime language beyond
the existing owned-copy policy.

### 6.3 Matmul shape flow

Explain one matrix-matrix multiplication by naming the dimensions:

```text
[m, n] x [n, p] -> [m, p]
```

The explanation should make the shared inner dimension and resulting outer
dimensions visible.

### 6.4 Dynamic readiness

Explain one messy-data readiness question:

```text
Which values are missing?
Which values are numeric?
Which values still block try_numeric()?
```

Use existing `Element`, mask, and `try_numeric()` vocabulary. Do not imply that
dynamic tensors support arithmetic before conversion.

### 6.5 Standardization before/after

Explain one preprocessing effect:

```text
input columns -> standardized columns
same shape, changed scale
mean approximately 0, std approximately 1
```

This can link to the existing `matten-mlprep` visual-standardization example
instead of duplicating the full content. Prefer linking over restating numeric
results. If implementation restates values, verify them against the runnable
`matten-mlprep` example output at implementation time, including whether the
reported standard deviation is population or sample based.

## 7. Navigation Requirement

The implementation should make the educational path discoverable from
`docs/src/tutorial/start-here.md` and `docs/src/examples/visual-understanding.md`.

Avoid creating a large new section unless the existing pages become too dense.
The preferred shape is:

```text
tutorial/start-here.md
  -> visual-understanding examples
  -> reference pages for shape/math/dynamic details
```

## 8. Content Rules

Use text-first visuals:

```text
ASCII diagrams
Markdown tables
short before/after blocks
small code snippets only when they directly support the explanation
```

Avoid:

```text
marketing language
speed or scale claims
large examples
generated assets
one-off diagrams that contradict existing examples
duplicating long reference sections into tutorial pages
```

Every worked explanation should be checkable by reading the accepted API docs or
running an existing example.

## 9. Acceptance Criteria

```text
[ ] at least three required educational increments are implemented
[ ] tutorial path links users to the visual-understanding path
[ ] visual-understanding page groups the worked explanations coherently
[ ] reference pages remain the source of detailed API truth
[ ] no public API is added
[ ] no dependency is added
[ ] no generated image/binary assets are added
[ ] no public matten-viz / matten-report crate is introduced
[ ] no tools/matten-report demo is added in this slice
[ ] no `Phase 1` / `Phase 2` wording appears in user-facing docs or examples
[ ] RFC-065 positioning/overclaim guard still passes
[ ] mdBook build succeeds
[ ] matten doctests pass if rustdoc is touched
```

## 10. Suggested Checks

Run:

```bash
bash scripts/check-release-docs.sh
mdbook build docs
git diff --check
```

Run this only if rustdoc or doc-tested Rust examples are touched:

```bash
cargo test -p matten --doc
```

If generated `docs/book` output appears after `mdbook build`, remove it before
requesting review.

## 11. Review Focus

Ask the reviewer to check:

```text
whether the examples are pedagogically correct
whether the diagrams match matten's actual shape semantics
whether the scope stays docs/examples only
whether the path is discoverable without bloating the book
whether any text implies plotting, expression tracing, autograd, or production-scale capability
```
