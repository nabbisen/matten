# RFC-070 Private Report Model Extraction Handoff

**Project:** `matten`
**Related RFCs:** RFC-063, RFC-065, RFC-068, RFC-069, RFC-070
**Document kind:** Local-tool refactor handoff
**Status:** Drafted for review; no implementation authorized
**Date:** 2026-07-17

---

## 1. Summary

Prepare the first post-audit RFC-070 prerequisite by extracting a small private
report-model layer inside `tools/matten-report`.

The RFC-070 readiness audit found that local Markdown/HTML artifacts are useful
and maintained, but not ready for public `matten-report` / `matten-viz` crates,
public renderer APIs, or one stable cross-family report schema. This handoff
therefore proposes an internal-only consolidation slice:

```text
reduce renderer duplication inside tools/matten-report
keep all report model types private
preserve every CLI behavior and output contract
avoid JSON/public API/public crate scope
```

This is not a public visualization product step. It is a maintenance refactor
that can make later policy audits more concrete without publishing any report
model.

---

## 2. Motivation

`tools/matten-report/src/main.rs` now contains several private report families:

```text
DataReadinessReportData
InputDataReadinessReportData
ShapeFlowReportData
DynamicReadinessReportData
MlprepStandardizationReportData
EducationalPathReportData
```

That local structure is good enough for fixed Markdown and HTML artifacts. The
remaining risk is not user-facing behavior; it is maintenance drift between
family-specific renderers and helper functions as the local tool evolves.

The RFC-070 audit explicitly rejected a public report model for now because the
stable concepts are still narrower than a reusable API:

```text
local report family names
Markdown/plain text default
explicit HTML output file
bounded previews
escaping and display bounds for user-controlled input
```

This handoff asks review to authorize only the private refactor that is useful
even if public report/viz crates never happen.

---

## 3. Implementation Scope

Allowed:

```text
private structs, enums, and helper functions inside tools/matten-report/src/main.rs
private module extraction inside tools/matten-report/src/ if it keeps the binary local
shared private section/table/tensor-preview helpers where they reduce duplication
shared private HTML document shell and style helpers where output remains stable
shared private report-family metadata helpers where they reduce branching drift
rewiring Markdown/HTML renderers to consume private helper outputs
behavior-neutral test updates that preserve exact output snapshots
README or planning-status wording only if needed to explain the internal refactor
```

Suggested target shape:

```rust
struct LocalReportDocument {
    title: &'static str,
    sections: Vec<LocalReportSection>,
}

struct LocalReportSection {
    heading: &'static str,
    blocks: Vec<LocalReportBlock>,
}

enum LocalReportBlock {
    Paragraph(String),
    BulletList(Vec<String>),
    Table(LocalReportTable),
    TensorPreview(String),
}
```

The exact names and granularity can differ. The important constraint is that the
types remain private implementation details owned by `tools/matten-report`.

Not authorized:

```text
public Report enum or public report schema
public renderer API
public matten-report crate
public matten-viz crate
workspace membership change
new CLI option
new report family
new output format
JSON / SVG / Vega-Lite / notebook / browser runtime scope
JavaScript or external assets
expression tracing
autograd
core Tensor visualization APIs
project scanning
project mutation
new dependency
version bump
release prep
tag or publish action
generated artifact checked into the repository
```

---

## 4. Behavior Requirements

The implementation must be behavior-neutral:

```text
default Markdown/plain text output remains unchanged
HTML still requires explicit --output
all existing supported HTML paths remain supported
all existing rejected format/input combinations remain rejected
all exact Markdown snapshots remain byte-identical unless review explicitly approves a diff
all exact HTML snapshots remain byte-identical unless review explicitly approves a diff
HTML remains static, self-contained, escaped, and free of script/external/network/data-url references
input-mode reports remain summary-only and bounded
no generated HTML artifact is checked in
```

If exact output must change to remove duplication, the implementation review
request must call out every intentional user-visible diff. The preferred result
is no output diff.

---

## 5. Suggested Work Order

1. Identify duplication shared by at least two report families or two formats.
2. Extract only the smallest private helper/model layer that removes real drift
   risk.
3. Keep report-family computation builders private and family-specific unless a
   shared helper is clearly simpler.
4. Run existing exact-output tests before broadening the refactor.
5. Add tests only where the extraction creates a new shared invariant.

Avoid extracting a large general schema just to resemble a future public API.
That would front-load maintenance cost without a reviewed product contract.

---

## 6. Suggested Files

Expected implementation files:

```text
tools/matten-report/src/main.rs
```

Optional implementation files if the private code becomes easier to review when
split:

```text
tools/matten-report/src/report_model.rs
tools/matten-report/src/render.rs
```

Expected tracking files if status is updated after implementation review:

```text
rfcs/handoffs/070-private-report-model-extraction-handoff.md
rfcs/handoffs/README.md
rfcs/README.md
ROADMAP.md
```

No public documentation update is required unless user-facing behavior changes.

---

## 7. Verification

Minimum implementation verification:

```bash
cargo fmt --all --check
cargo fmt --manifest-path tools/matten-report/Cargo.toml --check
cargo check --manifest-path tools/matten-report/Cargo.toml
cargo test --manifest-path tools/matten-report/Cargo.toml
cargo clippy --manifest-path tools/matten-report/Cargo.toml -- -D warnings
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo educational-path
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo shape-flow
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo dynamic-readiness
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo mlprep-standardization
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo educational-path --format html --output target/matten-report-educational-path.html
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo shape-flow --format html --output target/matten-report-shape-flow.html
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo dynamic-readiness --format html --output target/matten-report-dynamic-readiness.html
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo mlprep-standardization --format html --output target/matten-report-mlprep-standardization.html
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness --format html --output target/matten-report-data-readiness.html
bash scripts/check-release-docs.sh
git diff --check
```

If `docs/src/` changes, also run:

```bash
mdbook build docs
```

Remove generated `docs/book` afterward if created.

---

## 8. Acceptance Criteria

```text
[ ] private report-model/helper extraction reduces real duplication
[ ] extracted types and helpers are private to tools/matten-report
[ ] no public API, public crate, workspace membership, dependency, or output format is added
[ ] all existing Markdown/plain text behavior remains unchanged
[ ] all existing HTML behavior remains unchanged
[ ] all existing rejection paths remain unchanged
[ ] exact snapshot coverage remains passing
[ ] static/self-contained/escaped HTML safety coverage remains passing
[ ] input-mode display bounds remain passing
[ ] ROADMAP.md and RFC indices accurately record this as a private refactor only
```

---

## 9. Follow-up Boundary

After this handoff is implemented and reviewed, the next RFC-070 decision should
still be a planning decision, not automatic feature work.

Likely follow-up candidates:

| Candidate | Status |
|---|---|
| JSON report-schema policy audit | Still separate; may be easier after private model extraction |
| Public `matten-report` crate | Still deferred |
| Public `matten-viz` crate | Still deferred |
| Core visualization APIs | Still deferred |
| Additional local HTML feature work | Requires a separate handoff or RFC |
