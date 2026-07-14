# RFC-068 Shared Educational Report Data Handoff

**Project:** `matten`
**Related RFC:** RFC-068: Rich Local Visualization Artifacts
**Document kind:** Compact local-tool refactor handoff
**Status:** Implemented for review; review pending
**Scope:** Behavior-neutral shared data extraction for `tools/matten-report --demo educational-path`

---

## 1. Summary

Prepare the next RFC-068 implementation slice by extracting the duplicated fixed
educational-path computations used by the Markdown and HTML reports into one
private data builder inside `tools/matten-report`.

This handoff answers RFC-068 open question 2 for the immediate next slice:
after the first HTML artifact proved useful, the Markdown and HTML educational
reports should share one fixed report data model before another HTML report
family is added.

The slice is a local-tool refactor only. It must preserve the existing CLI,
existing Markdown output, existing HTML output, tests, dependencies, and scope
boundaries.

---

## 2. Audit Finding

The committed RFC-068 Phase 1 implementation has two independent educational
report renderers:

```text
render_educational_path_report()
render_educational_path_html_report()
```

Both currently construct the same fixed tensors and derived values:

```text
broadcasting inputs, result shape, and result values
reshape and transpose input/result shapes and values
mean_axis(0) and mean_axis(1) shapes and values
matmul input/result shapes and values
dynamic readiness shape and masks
standardization shape, means, and population standard deviations
"what this report is not" bullets
```

The duplication was acceptable for the first HTML artifact because it kept the
first implementation small. It becomes a drift risk before expanding HTML to
another report family, because future edits could update one educational report
format but not the other.

---

## 3. Implementation Scope

Allowed:

```text
private data structs or helper functions inside tools/matten-report/src/main.rs
one shared educational_path_data() builder
owned values copied from fixed demo tensors into the shared data model
Markdown renderer rewritten to consume the shared data
HTML renderer rewritten to consume the shared data
tests proving existing Markdown output remains exact
tests proving existing HTML output remains exact
tests proving existing HTML safety properties remain true
tests or assertions covering canonical shared values across both formats
README/handoff/RFC/roadmap status wording if needed
```

Suggested private structure shape:

```rust
struct EducationalPathReportData {
    reading_steps: Vec<&'static str>,
    broadcast: ShapeFlowData,
    reshape_transpose: ReshapeTransposeData,
    axis_reductions: AxisReductionData,
    matmul: MatmulData,
    dynamic_readiness: DynamicReadinessData,
    standardization: StandardizationData,
    non_goals: Vec<&'static str>,
}
```

The exact struct names and field grouping can differ if the implementation stays
clear and private.

Not authorized:

```text
new CLI option
new output format
HTML for any other report family
input-mode HTML
public API
public matten-report crate
workspace membership
new dependency
SVG / Vega-Lite / JSON / notebook / browser runtime scope
expression tracing
autograd
source scanning
project scanning
project mutation
version bump
tag or publish action
```

---

## 4. Behavior Requirements

The refactor must be behavior-neutral:

```text
--demo educational-path Markdown output remains byte-identical
generated HTML output remains byte-identical
--demo educational-path --format html --output <path> remains deterministic
--format html still requires --output
--format html remains rejected for every other demo and input mode
generated HTML remains static and self-contained
generated HTML still has no script tag, external asset reference, data URL, or network reference
no generated HTML artifact is checked in
```

The refactor should add an exact-output HTML snapshot test before or during the
shared-data extraction. If the implementation intentionally changes generated
HTML content or whitespace, the exact snapshot must be updated in the same
review slice and the change must be called out for review.

---

## 5. Suggested Files

Expected implementation files:

```text
tools/matten-report/src/main.rs
```

Expected tracking/documentation files if status is updated:

```text
rfcs/proposed/068-rich-local-visualization-artifacts.md
rfcs/handoffs/068-shared-educational-report-model-handoff.md
rfcs/handoffs/README.md
rfcs/README.md
ROADMAP.md
```

No user-facing documentation change is required unless the CLI behavior or
README wording changes. This slice should not change public docs just to describe
an internal refactor.

---

## 6. Verification

Minimum implementation verification:

```bash
cargo fmt --all --check
cargo fmt --manifest-path tools/matten-report/Cargo.toml --check
cargo fmt --manifest-path tools/matten-migrate/Cargo.toml --check
cargo check --manifest-path tools/matten-report/Cargo.toml
cargo test --manifest-path tools/matten-report/Cargo.toml
cargo clippy --manifest-path tools/matten-report/Cargo.toml -- -D warnings
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo educational-path
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo educational-path --format html --output target/matten-report-educational-path.html
bash scripts/check-release-docs.sh
git diff --check
```

If documentation under `docs/src/` changes, also run:

```bash
mdbook build docs
```

Remove generated `docs/book` afterward if created.

---

## 7. Acceptance Criteria

```text
[ ] educational-path fixed computations live in one private shared builder
[ ] Markdown and HTML renderers consume the shared data model
[ ] existing Markdown exact-output test remains passing
[ ] HTML exact-output snapshot test is added or kept passing
[ ] existing HTML static/self-contained safety test remains passing
[ ] generated HTML remains deterministic
[ ] HTML validation policy remains unchanged
[ ] no public API or published crate changes are made
[ ] no dependency is added
[ ] no new report family or output format is added
[ ] CI/release-checklist fmt blind spot remains closed by per-tool fmt checks
```

---

## 8. Follow-up Boundary

After this behavior-neutral refactor is reviewed, the next RFC-068 feature slice
can consider one additional local HTML report family. The likely candidate is
`shape-flow`, because it already focuses on shape transformations and has simple
fixed demo data.

Do not implement `shape-flow` HTML, SVG, Vega-Lite, public report crates, or
expression tracing as part of this refactor.
