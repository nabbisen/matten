# RFC-063 Phase 1 Visual Docs Handoff

**Project:** `matten`  
**Related RFC:** RFC-063: Visual Understanding and Reporting  
**Document kind:** Compact docs implementation handoff  
**Status:** Implemented for Phase 1; retained as implementation record  
**Scope:** Markdown / ASCII documentation only  

---

## 1. Scope

Implemented scope for Phase 1 of RFC-063:

```text
docs diagrams
ASCII shape diagrams
Markdown tables
tutorial/reference explanation improvements
```

Do not add:

```text
image assets
generated visual artifacts
SVG output
Vega-Lite JSON
HTML output
local tools
public APIs
crate dependencies
example helper APIs
```

## 2. Priority order

### Priority 1: core shape and axis understanding

Target pages:

```text
docs/src/reference/operators.md
docs/src/reference/shape-ops.md
docs/src/reference/math.md
```

Required diagrams:

```text
broadcasting shape alignment
reshape and flatten
transpose and swap_axes
matmul shape flow
mean_axis(0) vs mean_axis(1)
var_axis / std_axis behavior
concatenate vs stack
```

### Priority 2: dynamic and data-readiness understanding

Target pages:

```text
docs/src/reference/dynamic.md
docs/src/examples/data.md
docs/src/tutorial/start-here.md
```

Required diagrams:

```text
dynamic ingestion lifecycle
missing-value mask meaning
numeric-convertible mask meaning
matten-data Table -> selected columns -> NumericTable -> Tensor
```

Keep `docs/src/examples/data.md` small if included in Phase 1. Core shape and
axis docs come first.

## 3. Diagram style

Use formats that are readable in source diffs and rendered mdBook output:

```text
plain Markdown tables
fenced text diagrams
small aligned ASCII blocks
short before/after snippets
```

Avoid clever formatting that only works in one renderer. A reader should still
understand the diagram in a terminal, code review, or plain text editor.

## 4. Wording rules

Reports and diagrams explain; they do not judge.

Allowed:

```text
axis 0 reduction collapses rows and returns one value per column
shape [3] broadcasts across the leading axis
column "age" has one missing value
```

Avoid:

```text
this dataset is clean
this operation is optimal
this model is good
this library is faster
this migration is required
```

## 5. Non-goals

Phase 1 must not implement:

```text
Tensor::plot
Tensor::show
lazy expression graph
automatic expression tracing
autograd / backward
plotting framework integration
notebook workflow
GUI / dashboard
dataframe-style reporting
large-data visualization
```

## 6. Acceptance checklist

```text
[x] mdBook builds
[x] all diagrams are Markdown / ASCII only
[x] no new files under image/asset paths
[x] no generated artifacts checked in
[x] no Cargo.toml dependency changes
[x] examples referenced by docs exist
[x] diagrams are readable in plain Markdown
[x] no plotting / notebook / GUI positioning
[x] no public API snapshot change required
```

Suggested local checks:

```bash
mdbook build docs
bash scripts/check-release-docs.sh
```

Observed Phase 1 verification:

```bash
mdbook build . --dest-dir ../target/mdbook-phase1-check   # from docs/
bash scripts/check-release-docs.sh
```
