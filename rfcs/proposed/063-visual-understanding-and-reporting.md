# RFC-063: Visual Understanding and Reporting

**Status:** Accepted for planning with scope clarifications; Phase 1 visual docs implemented in `0.29.0-pre.1`; Phase 2 canonical example reports implemented in `0.29.0-pre.2`; optional `matten-mlprep` visual-standardization example implemented in `0.29.0-pre.3`; Phase 3 local-tool slices implemented in `0.29.0-pre.4` for `matten-data` readiness and `0.29.0-pre.5` for demo-only shape-flow
**Target:** v0.29+ design/docs phase, with implementation split by later handoffs  
**Theme:** Visual understanding, explanatory reporting, and optional visualization tooling  
**Depends on:** RFC-014, RFC-021, RFC-022, RFC-043, RFC-048, RFC-050, RFC-053  
**Related:** RFC-040, RFC-041, RFC-042, RFC-049, RFC-060  

---

## 1. Summary

This RFC establishes a scoped visualization direction for `matten`.

The goal is not to turn `matten` into a plotting library. The goal is to help users
understand:

```text
what the expression is doing
what the shape transformation means
what the data looks like
where missing / unusual values are
how a small computation changes the data
```

The first phase is Markdown / ASCII documentation only and has been implemented.
Later phases may add example-only reporting helpers, and still later phases may
consider local reporting or visualization tooling, but those later phases require
separate handoffs or RFC approval. Core `matten` must remain dependency-light
and computation-focused.

This RFC deliberately keeps visualization aligned with the project's "family car"
positioning: make first understanding easier, without competing with Plotters,
Vega-Lite, Polars, notebooks, dashboards, or ML frameworks.

---

## 2. Motivation

Code is code, and calculation is calculation. For beginners and PoC users, the
hard part is often not "can this operation run?" but:

```text
Did I reduce the axis I meant?
Did broadcasting happen the way I think?
What did reshape / transpose actually do?
Does this table column contain missing values or text?
Is this dynamic tensor ready for numeric computation?
What did standardization change?
```

`matten` already optimizes for time to a runnable PoC. Visual understanding is a
natural extension of that mission: it shortens the time from "the code runs" to
"I understand what it means."

The current documentation has many executable examples, but most of the feedback
is textual assertions or printed arrays. A small visual/reporting layer can make
the same accepted APIs more legible without expanding the mathematical scope.

---

## 3. Goals

1. Make shape and data transformations easier to understand.
2. Improve the tutorial path with visual explanations for accepted APIs.
3. Add example-level visual summaries only after a compact Phase 2 handoff.
4. Keep core `matten` free of plotting, SVG, browser, GUI, notebook, or graph
   dependencies.
5. Prefer Markdown / terminal / simple static output before richer renderers.
6. Keep visualization honest: explanatory, not marketing, not benchmark ranking.
7. Record a path toward optional `publish = false` tooling, while making clear
   that this RFC alone does not authorize such tooling.

---

## 4. Non-goals

This RFC does not authorize:

```text
[ ] plotting dependencies in core matten
[ ] changing Tensor operators to build a lazy expression graph
[ ] Tensor::plot(), Tensor::show(), or Tensor::backward()
[ ] notebook integration as a required workflow
[ ] browser UI / GUI / dashboard framework
[ ] automatic expression tracing inside Add/Sub/Mul/Div/matmul
[ ] graph-library, dataframe, or ML-framework scope
[ ] performance-claim charts or "faster than" visual marketing
[ ] visualization of huge data sets
```

Visualization must not become a backdoor for adding dataframe behavior, serious
linear algebra, autograd, or large-data streaming.

---

## 5. Design posture

### 5.1 Phase 1: docs first

The first implementation work is documentation only:

```text
Markdown diagrams
ASCII shape diagrams
Markdown tables
tutorial/reference explanation improvements
no new runtime dependency
no public API commitment
no checked-in generated image assets
no generated visual artifacts
```

This lets the project discover which visual forms are useful in the lowest-risk,
most reviewable format.

### 5.2 Phase 2: examples later

Example-only terminal / Markdown reporting helpers are accepted as a later
implementation target, not as implementation-complete work from this RFC.

Phase 2 requires a compact handoff before coding. Helpers must stay local to
examples, add no dependencies, and create no public API commitment.

### 5.3 Optional tooling later

If the examples prove valuable, a later review may consider a local tool. This
RFC did not authorize a tool by itself; the first separate Phase 3 handoff chose
and implemented:

```text
tools/matten-report      # publish = false, workspace-excluded
```

Still-deferred possibilities:

```text
tools/matten-viz         # publish = false, workspace-excluded
crates/matten-report     # only after a later stability review
crates/matten-viz        # only after a later stability review
```

Initial tooling should follow the `benchmarks/` and RFC-054 pattern: local,
optional, `publish = false`, and kept out of the published crate dependency graph.
The first implemented tool slice is Markdown/plain text `matten-data` readiness
only. Other output formats, dependencies, report families, security guarantees,
and CI behavior remain subject to separate handoffs or RFCs.

### 5.4 Core remains computation-only

Core `matten` may expose data that makes visualization easy (`shape`, `as_slice`,
`schema_summary`, masks, reductions), but it should not own rendering.

Any renderer-specific dependencies belong outside core.

---

## 6. Candidate work

### 6.1 First priority: core shape and axis understanding

Add or improve mdBook pages with compact diagrams for:

```text
broadcasting shape alignment
reshape and flatten
transpose and swap_axes
matmul shape flow
mean_axis(0) vs mean_axis(1)
var_axis / std_axis behavior
concatenate vs stack
```

These are the first implementation priority because they are core `matten`
concepts, are easy to explain with ASCII diagrams, require no new APIs or
dependencies, and reduce beginner mistakes directly.

Phase 1 diagrams must be Markdown / ASCII only. No generated images, checked-in
image assets, SVG, Vega-Lite, or HTML output are accepted in Phase 1.

### 6.2 Second priority: dynamic and data-readiness understanding

After core shape docs, add docs or later examples for:

```text
dynamic ingestion lifecycle
matten-data Table -> selected columns -> NumericTable -> Tensor
missing-value masks
numeric-convertible masks
```

This supports `matten-data` and dynamic ingestion without turning either into a
dataframe or reporting product.

### 6.3 Third priority: example-level terminal summaries

After Phase 1 review and a Phase 2 handoff, add examples that render small
tensors as terminal-friendly summaries:

```text
small matrix heatmap using ASCII blocks or symbols
missing-value mask view
numeric-convertible mask view
before / after standardization summary
row and column mean summary
tiny graph adjacency matrix view
tiny PageRank iteration table
small CSV schema report
```

These helpers should remain local to examples at first. They must not create a
public API commitment. If any example becomes canonical, it must run in CI smoke.

### 6.4 Reporting helpers for data meaning

`matten-data` is a strong candidate for reporting because its users are already
asking "what does this data mean before I convert it?"

Candidate report concepts:

```text
schema summary as Markdown
missingness summary
column kind counts
numeric min / max / mean summary after explicit numeric conversion
small histogram counts for numeric columns
```

The first version should be example-only or docs-only. A public API requires a
follow-up design section or handoff because it affects the `matten-data` surface.

Reports must be descriptive, not prescriptive. They may say:

```text
column "age": numeric-looking, 1 missing value
column "name": text
tensor shape: [3, 4]
axis 0 reduction: collapses rows, returns one value per column
```

They must not say:

```text
this dataset is clean
this model is good
this migration is required
this operation is optimal
this library is faster
```

### 6.5 Expression explanation without expression graphs

"How the expression draws" is valuable, but automatic expression tracing is not
accepted. The current operator model is eager and simple; it must stay that way.

Allowed direction:

```rust
// illustrative only; not accepted API
Explain::new()
    .input("a", &a)
    .input("b", &b)
    .op("broadcast add", "a + b")
    .render_markdown();
```

or a tutorial/example helper that manually names the steps:

```text
Step 1: a shape [2, 3]
Step 2: b shape [3] broadcasts to [2, 3]
Step 3: result shape [2, 3]
```

Rejected direction:

```text
record every Tensor operation automatically
make Tensor store parent operation nodes
add backward/autograd-style graph construction
```

---

## 7. Possible phasing

### Phase 1: Documentation diagrams

Scope:

```text
docs/src/reference/operators.md
docs/src/reference/shape-ops.md
docs/src/reference/math.md
docs/src/reference/dynamic.md
docs/src/examples/data.md
docs/src/tutorial/start-here.md
```

Phase 1 may include `docs/src/examples/data.md` only if the addition is small
and still Markdown / ASCII only. The priority order is core shape and axis docs
first, then dynamic/data-readiness docs.

Acceptance:

```text
[ ] no crate dependency changes
[ ] diagrams are readable in plain Markdown
[ ] examples referenced by diagrams already exist or are added separately
[ ] docs avoid plotting-framework or notebook positioning
[ ] no checked-in image assets
[ ] no generated visual artifacts
[ ] no SVG / Vega-Lite / HTML output
```

### Phase 2: Example-only reporting helpers

Scope:

```text
new examples in crates/matten/examples/
optional examples in crates/matten-data/examples/
optional examples in crates/matten-mlprep/examples/
```

Acceptance:

```text
[ ] helper functions stay inside examples
[ ] examples compile and run in CI smoke list if selected as canonical
[ ] no public API change
[ ] no plotting dependency
[ ] output is deterministic and small
[ ] compact Phase 2 handoff exists before implementation
```

### Phase 3: Optional local reporting tool

First implemented slice, authorized by the Phase 3 local-tool handoff:

```text
tools/matten-report
publish = false
workspace-excluded unless a later review says otherwise
Markdown/plain text only
matten-data readiness report only
```

Deferred output formats, requiring later review:

```text
HTML fragment
SVG file
Vega-Lite JSON
```

Acceptance:

```text
[ ] no dependency leaks into published crates
[ ] local-only, no telemetry, no network
[ ] explicit input/output paths
[ ] no automatic project mutation
[ ] output format and examples documented
[ ] separate RFC or handoff authorizes implementation
```

### Phase 4: Public companion only if proven

A published `matten-report` or `matten-viz` crate is not accepted by this RFC.
It requires a later RFC or an explicit amendment after the local tool proves value.

---

## 8. Dependency policy

Allowed in Phase 1 / Phase 2:

```text
standard library only
existing matten workspace crates
Markdown / ASCII output produced by examples
```

Not allowed until later review:

```text
small SVG writer
serde_json for Vega-Lite JSON if already justified
Plotters only in an unpublished tool, never core
HTML generation
```

Even `serde_json` is not accepted merely because it is already familiar or small;
output-format choices belong to a future tooling review.

Not allowed without a later RFC:

```text
GUI frameworks
browser automation
notebook runtime integration
large plotting stacks in published crates
dataframe libraries
ML frameworks
```

---

## 9. Relationship to existing boundaries

### Core `matten`

Core remains the numeric tensor library. It should not grow `plot` or `show`
methods. Visual examples may use `Tensor::shape`, `as_slice`, reductions, and
mask tensors.

### `matten-data`

`matten-data` may be the best place to explain "what the data means" at table
boundaries. It must remain CSV-to-Tensor preparation, not a dataframe engine.

### `matten-mlprep`

`matten-mlprep` may show before/after summaries for scaling and bias-column
operations. It must remain deterministic preprocessing, not model training.

### `matten-ndarray`

Visualization is not a bridge responsibility. Users who need rich plotting from
`ndarray` should use the ndarray ecosystem or external plotting crates.

### Migration docs

Visual reports may help users decide when to stay with `matten` or migrate to
`ndarray`, `nalgebra`, Polars/Pandas, or Candle, but must not claim automatic
conversion or production replacement.

---

## 10. Open questions

The initial review answered the first five open questions:

1. Start with core shape docs first. Then add one `matten-data` reporting example,
   then one `matten-mlprep` before/after summary example.
2. Phase 1 is Markdown / ASCII only. No generated images and no checked-in image
   assets yet.
3. The first local-tool slice chose `tools/matten-report` for Markdown/plain text
   `matten-data` readiness reporting. `tools/matten-viz` and published report/viz
   crates remain deferred.
4. SVG, HTML, and Vega-Lite output remain deferred until Markdown / terminal
   output proves useful and a later handoff or RFC accepts the format.
5. Report helpers should not become public APIs now. Keep them in docs,
   examples, local helper functions inside examples, or possibly a future
   `publish = false` tool.

Phase 2 example set answered by the compact handoff:

```text
crates/matten/examples/57_visual_shape_axis_summary.rs
crates/matten/examples/dynamic_09_visual_readiness_summary.rs
crates/matten-data/examples/data_06_visual_readiness_summary.rs
```

---

## 11. Acceptance criteria for this RFC

This RFC is accepted when:

```text
[ ] the project agrees visualization is a DX theme
[ ] the first phase is docs/examples only
[ ] core matten remains dependency-light and rendering-free
[ ] plotting/framework/notebook scope is explicitly rejected
[ ] any future tool or companion requires separate implementation approval
[ ] Phase 1 is Markdown/ASCII only
[ ] no checked-in image assets in Phase 1
[ ] no generated visual artifacts in Phase 1
[ ] no local tool implementation from this RFC alone
[ ] no public report/viz API from this RFC alone
[ ] any Phase 2 canonical examples run in CI
[ ] any Phase 3 tool requires a separate RFC or handoff
```

Phase 1 is complete once the Markdown / ASCII visual docs ship. This umbrella RFC
remains in `rfcs/proposed/` while later phases are pending; move it to
`rfcs/done/` only when the project either implements or explicitly closes the
remaining phases.
