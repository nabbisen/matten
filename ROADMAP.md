# `matten` ROADMAP

**Project:** `matten`  
**Document Kind:** Canonical Project Roadmap  
**Document Version:** `1.74.0`
**Date:** 2026-07-15
**Status:** Canonical roadmap updated for the 0.34.0 RFC-068 visualization-continuation release. Release scope is one local static HTML artifact for `tools/matten-report --demo mlprep-standardization --format html --output <path>`, with Markdown/plain text still default and `data-readiness` plus input-mode HTML still out of scope. This release does not authorize public report/viz crates, core visualization APIs, expression tracing, autograd, dependency changes in published crates, SVG/Vega-Lite/JSON/notebook/browser scope, or companion maturity changes.
**Planning Baseline:** core `matten` completed RFC-015 through RFC-021 (shipped through v0.15.3); RFC-022 boundary confirmation shipped in v0.16.0; v0.17.0 introduced the Cargo workspace and the `matten-ndarray` companion crate under the family version (RFC-025, RFC-027); v0.18.0 introduced the `matten-mlprep` companion crate under the family version (RFC-024, RFC-028); v0.19.0 promoted `matten-ndarray` to production-ready candidate status and `matten-mlprep` to beta status under lock-step family versioning (RFC-029); v0.19.1 shipped feature-robust dynamic rejection (RFC-031); v0.19.2 confirmed the companion dependency/import convention (RFC-032); v0.19.3 added the RFC-033-042 v0.20+ design set; v0.20.0 shipped the matten-data experimental scaffold (RFC-033); v0.20.1 shipped the matten-data table/CSV-to-Tensor API (RFC-034, RFC-035, Experimental). Under lock-step family versioning (RFC-030), every crate shares the family version (e.g. `0.19.2`); maturity is expressed by per-crate Status labels, not by separate version numbers. Next: v0.20+ materialization phase. RFC-032 is consumed by the companion dependency/import convention; v0.20+ planning starts at RFC-033. The first v0.20+ branch is `matten-data` decision/materialization; the second is small NumPy-inspired core comfort APIs that preserve the `matten` philosophy; the third is the examples program (RFC-043–048), which demonstrates famous small math/numerical problems and companion workflows without expanding product scope.

---

## 0. Authority and purpose

This `ROADMAP.md` is the canonical roadmap for v0.16 and later.

When documents disagree, resolve in this order:

1. accepted RFC for the specific topic;
2. external design public contract;
3. this roadmap and milestone gates;
4. requirements documents;
5. drafts, prototypes, and discussion memos.

The v0.16+ prospect supersedes older schedule lines that placed `matten-data` at v0.17 and bundled all bridge crates at v0.19.

**RFC numbering note:** RFC-032 is reserved/consumed by another issue. New v0.20+ roadmap RFCs therefore begin at **RFC-033**. The examples program follows the v0.20+ boundary/design RFCs and uses **RFC-043 through RFC-048**.

---

## 0.1 Documentation-governance track

The v0.19.0 requirements, external-design, and roadmap snapshot documents are
historical inputs, not current authority. The tracked docs-governance handoffs
close their remaining value in this order:

```text
1. docs-governance-01-spec-coverage-gap-closure-handoff.md
   Resolve the three unowned spec fragments before archival:
   non-binding performance targets, golden/fuzz/property testing status, and Display formatting.
   Status: implemented and reviewed in docs/design/coverage-gap-resolution.md.

2. docs-governance-02-spec-archival-and-ownership-rule-handoff.md
   Archive the v0.19.0 specs as tracked history and write down the ownership rule.
   Status: implemented and reviewed in docs/design/README.md and docs/design/history/.

3. docs-governance-03-philosophy-distillation-handoff.md
   Distill the tracked archived specs into an evergreen Philosophy page after archival exists.
   Status: implemented and reviewed in docs/src/philosophy.md.
```

The intended ownership model is:

```text
rfcs/                = normative decisions
docs/src/            = user-facing evergreen contract and positioning
ROADMAP.md           = forward schedule and milestone history
docs/design/history/ = dated historical design snapshots only
```

These handoffs are tracked in `rfcs/handoffs/README.md`. They are docs/design
work only: no public API, dependency, version, or release-scope change.
The tracked ownership rule lives in `docs/design/README.md`; the v0.19.0 snapshots live under
`docs/design/history/` and must not be cited as current authority.

---

## 1. Long-term positioning

`matten` core remains a **Sedan-first** Rust tensor library:

- one primary public type: `Tensor`;
- concrete `f64` numeric computation by default;
- clear shape, broadcasting, slicing, reduction, and matrix APIs;
- dynamic ingestion/cleanup as an explicit on-ramp;
- boundary-safe `Result` APIs for parsing and I/O;
- readable panic messages for local mathematical misuse.

The core crate is **not** a dataframe engine, ML framework, streaming engine, GPU backend, or wrapper around external numeric crates.

Companion crates may extend workflows, but they must remain optional and must not pollute the dependency graph of core `matten`.

---

## 2. Core boundary rule

Use this rule for every new proposal:

> If the feature makes `Tensor` itself simpler, safer, clearer, or easier to construct/inspect/clean/explicitly convert, it may belong in core `matten`.  
> If it adds table semantics, ML semantics, external framework dependencies, streaming lifecycle, domain workflow, or bridge behavior, it belongs in a companion crate.

Good dependency direction:

```text
matten-ndarray -> matten
matten-mlprep  -> matten
matten-data    -> matten
```

Forbidden dependency direction:

```text
matten -> matten-ndarray
matten -> matten-mlprep
matten -> matten-data
matten -> ndarray
matten -> nalgebra
matten -> candle-core
matten -> polars
matten -> arrow
matten -> datafusion
```

---

## 3. v0.16+ release themes

| Version | Theme | Primary milestone | Implementation posture |
|---|---|---|---|
| **v0.16.0** | Companion boundary confirmation | RFC-022 policy, workspace structure, dependency-boundary CI | Core policy + mechanics only |
| **v0.17.0** | `matten-ndarray` experimental | First low-risk companion crate | Small conversion implementation |
| **v0.18.0** | `matten-mlprep` experimental | Transparent numeric preprocessing | Small helper implementation |
| **v0.19.0** | Companion maturity hardening | `matten-ndarray` production-ready candidate; `matten-mlprep` beta decision | Hardening / QA / docs |
| **v0.19.1** | Companion hardening patch | RFC-031 feature-robust dynamic rejection; RFC lifecycle/doc cleanup | Patch / quality release |
| **v0.19.2** | Companion dependency/import policy | RFC-032: explicit dependency style confirmed; companion `pub use matten;` deferred; release-doc guard added | Documentation/tooling patch |
| **v0.19.3** | v0.20+ planning materialization | RFC-033–042 added as proposed design set; ROADMAP reconciled to lock-step + RFC-032; architect rulings applied | Documentation/planning patch |
| **v0.20.0** | v0.20+ design/materialization start | RFC-033 `matten-data` experimental scaffold (shell only); workspace member + dependency pins | Design + selective implementation approval |
| **v0.20.1** | `matten-data` table API | RFC-034 + RFC-035 implemented: `Table`/CSV ingestion/schema/numeric → `Tensor` (Experimental); `examples/csv_to_tensor.rs` shipped | Low-risk implementation |
| **v0.20.2** | Examples program planning | RFC-043–048 added as proposed examples RFC set + compact handoff; reconciled to the additive 30+ band, dedup against the existing suite, and shipped `matten-data` | Documentation/planning patch |
| **v0.20.3** | Examples: structure + beginner band | RFC-043 example structure/policy + RFC-044 beginner examples (`30_`–`32_`: magic square, Fibonacci-by-matrix, graph path counting); docs + smoke runs | Additive examples/docs |
| **v0.20.4** | Examples: matrix-iteration band | RFC-045 examples (`33_` Markov chain, `34_` tiny PageRank); docs + smoke runs | Additive examples/docs |
| **v0.20.5** | Benchmarking program planning | RFC-049 added as proposed (benchmark harness, complexity metrics, positioning report); ROADMAP Track D added | Documentation/planning patch |
| **v0.20.x** | Minimal implementation phase | Small core comfort APIs; new 30+ famous-problem examples; audit/improve existing companion examples | Low-risk implementation only |
| **v0.21.0** | Shape composition (RFC-039) | `concatenate` + `stack` in core (borrowed slices, try_/panic, MattenLimits, dynamic-reject); repeat/tile/meshgrid deferred | Additive core API (v0.21 boundary review) |
| **v0.21.1** | Linalg core-lite (RFC-041) | `norm` (L2/Frobenius), `trace` (rectangular via min(rows,cols)), `outer`; decomposition/BLAS/sparse rejected | Additive core API |
| **v0.21.2** | Statistics core (RFC-040) | `var`/`std` + `var_axis`/`std_axis`, population variance (ddof=0), two-pass; quantile/histogram/cov/corr deferred | Additive core API |
| **v0.21.3** | matten-data scope guard (RFC-042) | Three-check release-docs guard (filename / public-API identifier / non-goal context); may land earlier | Docs/tooling |
| **v0.21.4** | Release-truth & CI-gate patch | v0.21.3 deep-review P1 fixes: 0.20→0.21 doc drift, family-aware release-docs guard wired into CI | Docs/tooling |
| **v0.22.0** | **`matten-data` promoted to Beta** | RFC-036 six-example suite (`data_00`–`data_05`) + explicit malformed-CSV test cleared the RFC-023 §9 gate; status Experimental→Beta; `data.md` guide; guards/CI updated. No library/API change | Maturity milestone |
| **v0.22.1** | RFC-049 Phase 1 — internal benchmark baseline | Accepted RFC-049 (staged). Added isolated `benchmarks/` criterion harness (workspace-excluded, publish=false) + methodology docs + baseline report template; core micro set & 5 scenario workloads; boundary guard now forbids criterion in core; CI compile-checks harness only (no speed gates). Phases 2–4 deferred. No published-crate code change | Tooling/docs |
| **v0.22.2** | Lifecycle wording cleanup | v0.22.0 handoff-review P2 follow-up: RFC-023 §9 gains a clarification that the malformed-CSV criterion is met by a structured-error/no-panic test (Csv or RaggedRow, never panic/silent), not a parser-error test; RFC-036 note updated. Historical CHANGELOG/ROADMAP entries left intact. No code/API/guard/CI change | Docs/lifecycle |
| **v0.22.3** | RFC-032 scope carve-out + published-dep isolation guard | Benchmarking/positioning review follow-up. RFC-032 §5.1 records that workspace-excluded publish=false tooling (RFC-031 fixture, RFC-049 harness) is outside the published-family convention. Added scripts/check-published-dependency-isolation.sh (per-crate peer-dep matrix; matten-ndarray→ndarray allowed) wired into CI + checklist. RFC-049 Phase 2 design settled & annotated (B1–B4) but NOT implemented; added BASELINE-READY-CHECKLIST. No library/API change | Docs/tooling |
| **v0.32.0** | Rich local visualization artifacts | RFC-068 implemented: static local HTML artifacts for `tools/matten-report --demo educational-path` and `tools/matten-report --demo shape-flow`, preserving Markdown default and deferring public crates/SVG/Vega-Lite/expression tracing | Local-tool visualization artifact |
| **v0.33.0** | Visualization continuation release | RFC-068 continuation implemented: static local HTML artifact for `tools/matten-report --demo dynamic-readiness`, preserving Markdown default and deferring data-readiness/input-mode HTML, public crates, SVG/Vega-Lite, and expression tracing | Local-tool visualization artifact |
| **v0.34.0** | Visualization continuation release | RFC-068 continuation implemented: static local HTML artifact for `tools/matten-report --demo mlprep-standardization`, preserving Markdown default and deferring data-readiness/input-mode HTML, public crates, SVG/Vega-Lite, and expression tracing | Local-tool visualization artifact |
| **v0.21+** | Selective production readiness | `matten-data` maturity decision **resolved → Beta in v0.22.0** (RFC-023/RFC-036); remaining companion maturity decisions; harder numerical/ML-like examples as APIs mature; benchmarking & positioning (RFC-049 — **Phase 1 internal baseline accepted/shipped in v0.22.1**; Phases 2–4 deferred) | Per-crate decisions |
| **Later** | Streaming / large CSV, `nalgebra`, `candle`, stats/linalg companions | Separate RFCs required | Design-only until reopened |

> **Performance-watch (P2, not a release blocker).** The RFC-049 Phase 1 internal baseline
> showed `sum_mean_axis` (~1.31 ms on 64×64) is the most expensive core path by a wide
> margin — ~400× the whole-tensor `sum_mean` and ~17× a 64×64 `matmul`. Recorded as a
> regression-visibility anchor: investigate the axis-reduction implementation cost only if
> benchmarks or real user workflows show axis reductions becoming a practical bottleneck.
> `matten` is DX-first, not a performance crate, so this is not a fix-now item, and Phase 2
> was not blocked on it (architect ruling, 2026-06-24).

---

## 4. v0.16.0 milestone: companion boundary confirmation

### Goal

Make the companion-crate model concrete without expanding core `matten`.

### Required work

- Implement RFC-022 as policy and project mechanics.
- Decide workspace layout.
- Define the workspace versioning model (independent per-crate SemVer initially;
  superseded by lock-step family versioning in v0.19.0, RFC-030).
- Define companion error-type policy.
- Define maturity labels.
- Add mechanical dependency-boundary CI.
- Mark old in-core bridge examples/features as superseded.
- Update RFC-023 through RFC-026 target headers to match this roadmap.

### Acceptance gate

`v0.16.0` is complete only if all of the following hold:

```text
[ ] core matten has no direct dependency on ndarray/nalgebra/candle/polars/arrow/datafusion
[ ] core matten has no dependency on matten-* companion crates
[ ] companion crate policy is documented in RFC-022
[ ] ROADMAP.md is the canonical future schedule
[ ] external design bridge sections are marked superseded
[ ] users can still ignore all companion crates
```

### Explicit non-goals

- No dataframe API in core.
- No ML preprocessing API in core.
- No external bridge API in core.
- No streaming CSV API in core.

---

## 5. v0.17.0 milestone: `matten-ndarray` experimental

### Goal

Prove the companion-crate pattern with the lowest-risk useful crate.

### Why first

`matten-ndarray` is the best first companion because it is small, useful in mathematical/laboratory workflows, and unlikely to change the product identity.

### Experimental scope

```rust
use matten_ndarray::{from_arrayd, to_arrayd};

let arr = to_arrayd(&tensor)?;
let tensor = from_arrayd(arr)?;
```

Allowed:

- `Tensor -> ndarray::ArrayD<f64>`;
- `ndarray::ArrayD<f64> -> Tensor`;
- scalar/vector/matrix/N-D conversion tests;
- clear conversion errors;
- dynamic tensors return `Err` unless converted through `try_numeric()` first;
- copy behavior documented honestly.

Forbidden:

- adding `ndarray` to core `matten`;
- wrapping the `ndarray` API broadly;
- promising zero-copy before it is designed and tested;
- adding `nalgebra`/`candle` in the same milestone.

### Acceptance gate

```text
[ ] conversion roundtrips are tested (scalar/vector/matrix/N-D)
[ ] from_arrayd preserves logical order for non-standard-layout ArrayD inputs
[ ] from_arrayd rejects zero-sized axes with a clear companion error
[ ] dynamic input returns Result::Err, not panic
[ ] ndarray version policy is documented
[ ] core matten dependency-boundary check still passes
[ ] examples live in matten-ndarray, not core matten
```

---

## 6. v0.18.0 milestone: `matten-mlprep` experimental

### Goal

Provide small, transparent numeric preprocessing helpers without becoming an ML framework.

### Experimental scope

Allowed initial APIs:

```rust
standardize_columns(&x)
minmax_scale_columns(&x)
add_bias_column(&x)
train_test_split(&x, 0.8)
```

Default `train_test_split` semantics:

```text
2D tensors only
rows = samples
columns = features
ordered deterministic split
no hidden randomness
first floor(n_rows * train_ratio) rows -> train
remaining rows -> test
```

If shuffled split is added later, it must be explicit:

```rust
train_test_split_seeded(&x, 0.8, seed)
```

### Forbidden

- model training;
- autograd;
- neural networks;
- optimizers;
- hidden randomness;
- implicit Candle dependency;
- automatic ML pipelines.

### Acceptance gate

```text
[ ] row/sample and column/feature convention is enforced
[ ] split ratio validation is tested
[ ] zero-variance policy is documented
[ ] examples are deterministic
[ ] core matten dependency-boundary check still passes
```

---

## 7. v0.19.0 milestone: maturity hardening

### Goal

Promote only companion crates that stayed small and useful.

### `matten-ndarray` production-ready candidate gate

```text
[ ] scalar/vector/matrix/N-D conversions work
[ ] roundtrip tests are reliable
[ ] dynamic tensors are rejected clearly
[ ] copy behavior is documented
[ ] no zero-copy promise unless implemented
[ ] examples run in CI
[ ] core matten has no ndarray dependency
```

**Status:** All criteria met; `matten-ndarray` promoted **candidate → production-ready** in
v0.25.0 (RFC-057). Both bridge examples (`to_arrayd`, `from_arrayd`) are already executed in CI by
the pre-existing `smoke` job — RFC-057's initial audit missed it; no CI change was needed.

### `matten-mlprep` beta decision gate

```text
[ ] API is small and teachable
[ ] functions are deterministic
[ ] shape rules are documented
[ ] zero-variance behavior is explicit
[ ] train/test split behavior is explicit
[ ] no ML-framework scope entered
```

**Status:** All criteria met; `matten-mlprep` promoted **Beta → production-ready candidate** in
v0.26.0 (RFC-058). Full production-ready is deferred — `train_test_split` is ordered-only (no
shuffle); the candidate → production-ready exit criteria are recorded in RFC-058 §5.1.

---

## 8. v0.19.1 milestone: companion hardening patch

### Goal

Finish the v0.19 maturity work before expanding scope.

### Required work

- Implement RFC-031: feature-robust dynamic rejection and unconditional `Tensor::is_dynamic()`.
- Keep companion `dynamic` mirror features for compatibility; document them as compatibility forwarding features.
- Move / mark RFC-024 as done.
- Move / mark RFC-025 as done for `matten-ndarray`, with `nalgebra` and `candle` explicitly deferred.
- Align companion rustdoc, README, Cargo descriptions, and status labels.
- Strengthen release-doc checks for stale version snippets, stale maturity labels, and active independent-SemVer wording.
- Fix known small lints such as `manual_contains` in `matten-ndarray`.

### Acceptance gate

```text
[ ] dynamic Tensor passed to matten-ndarray returns MattenNdarrayError::DynamicTensor, not panic
[ ] dynamic Tensor passed to matten-mlprep returns MattenMlprepError::DynamicTensor, not panic
[ ] the guarantee does not depend on enabling companion dynamic mirror features
[ ] companion dynamic mirror features remain present for v0.19.1 compatibility
[ ] RFC-024 / RFC-025 lifecycle status is no longer contradictory
[ ] release-doc script detects stale status/version/versioning drift
[ ] workspace tests and core dependency-boundary check pass
```

### Explicit non-goals

- No `matten-data` implementation in v0.19.1.
- No removal of companion `dynamic` features.
- No breaking change.
- No v0.20 scope bundled into the patch.


---

## 9. v0.20+ milestone: materialize the next safe expansion

### Goal

v0.20+ has four parallel tracks:

```text
Track A: matten-data decision/materialization
  Decide whether a small table-to-Tensor companion is worth building.

Track B: core numeric comfort APIs
  Add small NumPy-inspired Tensor conveniences only if they preserve the Sedan-first philosophy.

Track C: examples program
  Demonstrate accepted APIs through famous small math / numerical-computing problems
  without creating hidden dataframe, SciPy, or ML-framework scope.

Track D: benchmarking & positioning
  Build a reproducible, honest evidence base (time, memory, ELOC, dependency
  footprint, regression visibility). Measurement and positioning only — not a
  performance contest, and not a reason to chase larger ecosystems.
```

The release must not become a broad clone of NumPy, SciPy, or Pandas.

The v0.20+ motto is:

> Borrow familiar API ideas. Shrink them to `matten`-sized scope. Stop before dataframe, SciPy, or ML-framework expectations.

---

### 9.1 RFC numbering for v0.20+

RFC-032 is already consumed by another issue. v0.20+ roadmap RFCs therefore start at RFC-033.

| RFC | Theme | Target |
|---:|---|---|
| RFC-033 | `matten-data` Beta-Decision and Scope Lock | v0.20.0 |
| RFC-034 | `matten-data` Table Model and Public API Boundary | v0.20.0 |
| RFC-035 | CSV Ingestion, Schema Summary, Missing Values, and Numeric Conversion | v0.20.0 |
| RFC-036 | `matten-data` Examples, Documentation, and Release Gate | v0.20.0 |
| RFC-037 | Deferred Streaming and Large CSV Policy | v0.20.0 / later |
| RFC-038 | Core Numeric Comfort APIs | **Done** (v0.20.9–v0.20.12) |
| RFC-039 | Shape Composition API Boundary | **Implemented** (v0.21.0) — `concatenate` + `stack` in core; `repeat`/`tile`/`meshgrid` deferred |
| RFC-040 | Small Statistics Boundary: Core vs Companion | **Implemented** (v0.21.2) — `var`/`std` + `var_axis`/`std_axis` (population); quantile/histogram/cov/corr deferred |
| RFC-041 | Linear Algebra Boundary: Core Lite vs External Crates | **Implemented** (v0.21.1) — `norm` + `trace` + `outer` in core; decomposition/BLAS/sparse rejected |
| RFC-042 | Pandas-Inspired Scope Guard for `matten-data` | **Implemented** (v0.21.3) — three-check anti-scope guard (file names / public API / README); CI-enforced |
| RFC-043 | Example Program Structure, Quality Gate, and Documentation Policy | v0.20.x |
| RFC-044 | Beginner Core Math Examples | v0.20.x |
| RFC-045 | Matrix Iteration and Graph/Probability Examples | v0.20.x |
| RFC-046 | Numerical Methods and Scientific Toy Examples | v0.21+ or after needed APIs |
| RFC-047 | Small ML-Like Examples Without ML-Framework Scope | v0.21+ |
| RFC-048 | Companion-Crate Examples | v0.20.x / v0.21+ |
| RFC-049 | Benchmarking, Complexity Metrics, and Positioning Report | v0.20.x planning / v0.21+ maturity hardening |

RFC-042 may be folded into RFC-033 if the scope guard is already strong enough. RFC-043 through RFC-048 are examples/documentation RFCs: they demonstrate accepted APIs and workflows, but do not authorize new product scope by themselves. RFC-049 is a non-API measurement/positioning RFC: it adds a benchmark harness and reports in an isolated, `publish = false` package and must not add runtime dependencies to core `matten` or any companion.

---

### 9.2 Track A: `matten-data` decision/materialization

#### Goal

Decide whether `matten-data` deserves beta without becoming a dataframe engine.

`matten-data` may be scaffolded earlier, but it must not be promoted before the v0.20+ decision gate.

> **Resolved:** `matten-data` reached **Beta** in v0.22.0 (RFC-036) and was promoted **Beta →
> production-ready candidate** in v0.27.0 (RFC-059), with the RFC-042 scope lock preserved (still an
> on-ramp, not a dataframe engine). Full production-ready is deferred to a separate future review.

#### Required proof

The crate must prove this small workflow:

```text
CSV / table-like data
  -> inspect schema
  -> clean missing values
  -> select numeric columns
  -> explicit numeric conversion
  -> matten::Tensor
```

Possible API shape:

```rust
use matten_data::Table;

let table = Table::from_csv_path("sales.csv")?;
println!("{}", table.schema_summary());

let x = table
    .select_columns(["sales", "cost", "quantity"])?
    .fill_missing(0.0)?
    .try_numeric()?
    .to_tensor()?;
```

#### Allowed beta scope

- CSV string/path ingestion;
- schema summary;
- column names;
- column selection;
- missing-value cleanup;
- explicit numeric conversion;
- Tensor output.

#### Still forbidden

- joins;
- group-by;
- pivot;
- SQL-like query API;
- lazy execution;
- large-data streaming;
- window functions;
- dataframe-style indexing;
- ML preprocessing.

#### Decision outcomes

At v0.20+, choose one:

```text
A) promote to beta
B) keep experimental
C) freeze/defer
```

Keeping it experimental is acceptable if the API is useful but not mature. Freezing is acceptable if the crate starts drifting into dataframe territory.

#### Acceptance gate

```text
[ ] RFC-033 through RFC-036 accepted before implementation expands
[ ] RFC-037 explicitly defers streaming / large CSV implementation
[ ] core matten has no dependency on matten-data
[ ] matten-data has no dataframe/query/lazy API
[ ] missing-value and numeric-conversion policy is explicit
[ ] duplicate-header and ragged-row policy is documented
[ ] error type is crate-local
[ ] examples are small and do not imply Pandas replacement
```

---

### 9.3 Track B: core numeric comfort APIs

**Status: Complete (RFC-038, shipped across v0.20.9–v0.20.12).** The four bands below
all shipped: elementwise math (v0.20.9), selection `argmin`/`argmax` (v0.20.10),
creation `linspace`/`eye` (v0.20.11), and shape `squeeze`/`expand_dims` (v0.20.12).

#### Goal

Make core `matten` more pleasant for PoC mathematical work by adding small familiar APIs inspired by NumPy, without changing project identity.

Candidate RFC:

```text
RFC-038: Core Numeric Comfort APIs
```

#### Good core candidates

```rust
Tensor::linspace(start, end, count)
Tensor::eye(n)
tensor.clip(min, max)
tensor.abs()
tensor.sqrt()
tensor.exp()
tensor.ln()
tensor.argmin()
tensor.argmax()
tensor.squeeze()
tensor.expand_dims(axis)
```

These fit core if they remain:

- Tensor-centered;
- dependency-light;
- easy to document;
- shape-obvious;
- useful for beginner/intermediate numeric workflows.

#### Needs separate boundary review

```rust
stack(...)
concatenate(...)
repeat(...)
tile(...)
meshgrid(...)
var(...)
std(...)
quantile(...)
histogram(...)
```

These are useful but have enough shape/statistics policy risk to need focused RFC review.

#### Core comfort acceptance gate

```text
[ ] no heavy dependency added
[ ] API is small and teachable
[ ] behavior is obvious for scalar/vector/matrix/N-D where applicable
[ ] NaN/Inf behavior is documented where relevant
[ ] panic-zone vs Result-zone is clear
[ ] examples compile in CI
[ ] no generic Tensor<T> or dtype system introduced
```

---

### 9.4 Track C: examples program

**Status: Complete (RFC-043–048, shipped across v0.20.3–v0.20.13).** All example
bands shipped: structure/policy (v0.20.3), beginner 30–32 (v0.20.3), matrix iteration
33–34 (v0.20.4), companion audit (v0.20.6), numerical methods 35–36 (v0.20.7) and
39–40 (v0.20.13), and ML-like 37–38 (v0.20.8). RFC-043–048 are in `rfcs/done/`. The
optional `41_adjacency_walks_extended` remains a not-reserved conditional candidate.

#### Goal

Increase `matten` examples using famous and recognizable small math / numerical-computing problems while preserving the project philosophy.

The examples program should make users understand:

```text
what Tensor can represent
how small vector/matrix algorithms look in matten
where companion crates fit
what matten intentionally does not do
```

The examples must not become hidden product commitments.

#### RFCs

| RFC | Theme | Implementation posture |
|---:|---|---|
| RFC-043 | Example Program Structure, Quality Gate, and Documentation Policy | Start first; docs/CI/policy foundation |
| RFC-044 | Beginner Core Math Examples | Low-risk examples; can start early |
| RFC-045 | Matrix Iteration and Graph/Probability Examples | Intermediate examples; can start after RFC-044 |
| RFC-046 | Numerical Methods and Scientific Toy Examples | Add after APIs are available; some examples may wait for RFC-038 |
| RFC-047 | Small ML-Like Examples Without ML-Framework Scope | Add cautiously; no ML framework implication |
| RFC-048 | Companion-Crate Examples | All companions shipped (incl. `matten-data` v0.20.1); audit/improve existing examples, do not duplicate |

#### Example groups

New famous-problem examples use a fresh additive **30+ band**; the existing
`00_`–`28_` core suite, the `dynamic_*` set, and the named examples are preserved
and never renumbered (architect ruling, RFC-043–048 review Q1).

Beginner core examples (new files):

```text
30_magic_square_checker.rs
31_fibonacci_matrix_power.rs
32_graph_path_counting.rs
```

Cross-reference / improve in place instead of duplicating (already shipped):

```text
existing 26_cosine_similarity.rs   (cosine similarity)
existing pairwise_distance.rs      (vector distance)
existing 25_normalize_vector.rs
```

Matrix iteration / graph / probability examples:

```text
33_markov_chain_weather.rs
34_tiny_pagerank.rs
```

Optional candidate, not reserved — `41_adjacency_walks_extended.rs`: add only if the
Phase 0 inventory shows it teaches a distinct concept beyond `32_graph_path_counting.rs`
(otherwise drop it).

Numerical methods examples:

```text
35_linear_regression_gradient_descent.rs
36_heat_equation_1d.rs
39_finite_difference_derivative.rs   # shipped v0.20.13 (RFC-038 linspace)
40_trapezoidal_integration.rs        # shipped v0.20.13 (RFC-038 linspace)
```

Small ML-like examples:

```text
37_kmeans_small.rs
38_nearest_neighbor_classification.rs
```

Companion examples (all shipped; audit/improve existing files, do not duplicate):

```text
crates/matten-ndarray/examples/from_arrayd.rs, to_arrayd.rs
crates/matten-mlprep/examples/standardize_columns.rs, train_test_split.rs
crates/matten-data/examples/csv_to_tensor.rs   # shipped in v0.20.1; audit/improve as needed
```

#### Implementation order

```text
0. Inventory existing examples first (audit before adding anything)
1. RFC-043: docs/src/examples/index.md, example structure, CI/example policy
2. RFC-044: beginner examples (30+ band; cross-reference existing distance/cosine)
3. RFC-045: matrix-iteration examples
4. RFC-048: audit/improve existing companion examples
5. RFC-046: numerical-method examples
6. RFC-047: small ML-like examples
```

#### Acceptance gate

```text
[ ] existing examples inventoried before adding any new file (audit-first)
[ ] new examples use the additive 30+ band; existing 00-28 suite not renumbered
[ ] no example duplicates a concept the existing suite already teaches
[ ] examples compile in CI
[ ] examples run deterministically
[ ] examples use small hard-coded data
[ ] examples explain problem, math idea, Tensor representation, and expected output
[ ] examples use only accepted APIs
[ ] companion examples live in companion crates
[ ] no example implies dataframe, SciPy, ML-framework, GPU, or large-data scope
[ ] future-only examples are deferred until their required APIs exist
[ ] the test.yaml smoke list is extended deliberately as runnable examples land
```

#### Non-goals

The examples program must not add examples for:

```text
large CSV
streaming CSV
dataframe group-by
join / merge / pivot
SVD / PCA as core examples
neural network training
autograd
GPU/device usage
sparse matrices
database ingestion
web/network data loading
```

---

### 9.5 Track D: benchmarking & positioning (RFC-049)

#### Goal

Build a reproducible, honest evidence base for where `matten` sits, measured rather
than asserted: execution time, memory behavior where practical, example-code ELOC,
dependency footprint, and regression visibility. The deliverable is a positioning
report, not a leaderboard.

This is a non-API, measurement-only program. It does not add public runtime APIs and
must not pull benchmark tooling into core `matten` or any companion.

#### Posture and sequencing

RFC-049 is **planning now / implementation v0.21+ maturity hardening**. Methodology
docs and the harness skeleton may begin in v0.20.x; the bulk lands during v0.21+
hardening, and peer/reference comparisons follow only after an internal baseline is
stable. Scenario benchmarks track the examples program: they cover only shipped
examples (today `pairwise_distance`, `26_cosine_similarity`, and `30_`–`34_`), and
grow as RFC-046/047 bands land.

#### Phases

```text
Phase 1: internal baseline (matten only) — core micro + scenario + companion workloads
Phase 2: Rust peer comparison (ndarray, nalgebra) — "peer comparison" wording
Phase 3: ecosystem reference (NumPy, Pandas table-to-Tensor only) — "reference comparison" wording
Phase 4: regression tracking — record/trend first; soft warnings, then thresholds via a later RFC
```

SciPy and Candle are deferred references; they are out of scope until a separate,
task-specific decision accepts them.

#### Hard constraints (binding)

```text
[ ] benchmark code lives in an isolated `publish = false` package; never a core/companion dependency
[ ] the core dependency-boundary script still passes (no criterion/ndarray/nalgebra in core)
[ ] no Python required for ordinary Rust build/test/CI
[ ] no network access and no external dataset downloads during benchmarks
[ ] no hard CI speed-fail gate initially (harness/schema failures may fail; "slower" may not)
[ ] reports use tradeoff language; never "matten beats / replaces X"
```

#### Acceptance gate (initial)

```text
[ ] methodology docs + non-goals documented (docs/src/benchmarks/*)
[ ] internal baseline harness compiles and runs on one maintainer machine
[ ] selected core + companion benchmarks compile under correct features
[ ] ELOC methodology documented; report template exists with environment metadata
[ ] no runtime dependency added to core matten; boundary check passes
[ ] reports avoid replacement/marketing claims
```

---

### 9.6 What v0.20+ must not do

v0.20+ must not become:

```text
a NumPy clone
a SciPy clone
a Pandas clone
a dataframe engine
an ML framework
a large-data streaming engine
a linalg backend wrapper
```

Borrow ergonomic ideas, not ecosystem scope.


---

## 10. Later themes

### `matten-nalgebra`

Deferred until after `matten-ndarray` proves the bridge pattern. Requires a separate RFC. RFC-025 is considered implemented for `matten-ndarray`; future `nalgebra` work must not rely on implied acceptance.

### `matten-candle`

Deferred longer because it brings device, dtype, ML, and dependency-tree complexity. Requires a separate RFC.

### Streaming / large CSV

Design-only until batch lifecycle, schema drift, malformed-row policy, memory budget, and sync-vs-async strategy are proven. May later live in `matten-data` or a separate `matten-stream`; undecided.

### `matten-stats`

Possible later companion or small-core extension area. Requires RFC-040 before implementation. Candidate topics include variance, standard deviation, covariance, correlation, quantile, and histogram. These APIs have policy traps (`ddof`, NaN behavior, interpolation), so they must not be rushed into core.

### Examples program continuation

The examples program may continue after RFC-043 through RFC-048, but only as demonstration work over accepted APIs. New examples that require new public API should cite or wait for the relevant RFC. Examples must not be used to smuggle in dataframe, SciPy, ML-framework, large-data, GPU, or serious-linalg scope.

### `matten-linalg-lite`

Possible later boundary topic. Requires RFC-041 before implementation. Core may keep only small obvious helpers such as `norm`, `trace`, or `outer` if accepted. Serious linear algebra such as inverse, determinant, eigenvalues, SVD, QR, and Cholesky should remain outside core or be delegated through external crates.

---

## 11. Workspace versioning policy

The workspace uses **lock-step family versioning** (RFC-030, which supersedes the
earlier independent-per-crate-SemVer plan). Every crate shares one version, set in
`[workspace.package].version`:

```text
matten          0.19.0
matten-ndarray 0.19.0
matten-mlprep  0.19.0
```

- **Version = compatibility.** Matching numbers mean a matched, compatible set —
  no per-crate compatibility matrix for users.
- **Maturity = the Status label** (experimental / beta / production-ready
  candidate / production-ready), declared per crate in its README/docs. A crate
  at `0.19.0` may still be `beta`; the version does not imply maturity.

This fits the project's reality: the crates are released together as milestone
artifacts. If a crate ever needs an independent release cadence, the model is
revisited (back to independent SemVer, with the per-crate `CHANGELOG`/`LICENSE`
split of RFC-022 §12).

### 11.1 Workspace file conventions (resolved v0.19.0)

While the crates ship together as **milestone tarballs** (not yet published to
crates.io), the workspace keeps the structure simple:

- a **single root `CHANGELOG.md`**, ordered by milestone, recording each crate's
  version change inside the relevant entry;
- **root-only `LICENSE`/`NOTICE`**; each crate is licensed by its inherited SPDX
  `license = "Apache-2.0"` field (no per-crate license file is required by cargo
  or crates.io when that field is set).

Per-crate `CHANGELOG`s and per-crate `LICENSE`/`NOTICE` files are reintroduced at
the point crates begin **independent crates.io publication** — the moment a
crate's own version history and self-contained `.crate` artifact start to earn
their maintenance cost (RFC-022 §12).

---

## 12. Maturity labels

### Experimental

Useful for feedback. API may change. Not recommended for production dependency without pinning.

Signals:

- README warning;
- version 0.x;
- docs say experimental;
- changelog may include breaking changes;
- examples are small.

### Beta

Useful for small real workflows. API is intended to be mostly stable, but still pre-1.0.

Signals:

- README beta badge/text;
- examples in CI;
- documented limitations;
- public API snapshot or equivalent;
- breaking changes require migration notes.

### Production-ready candidate

The team believes the crate can be used seriously if the documented limits are acceptable.

Signals:

- strong tests;
- examples in CI;
- clear error types;
- documented compatibility policy;
- no known P0/P1 issues;
- release checklist complete.

### Production-ready

Stable enough to recommend as a normal dependency for its documented scope.

Signals:

- mature docs;
- stable API;
- compatibility and MSRV policy;
- clear release notes;
- no hidden dependency surprises.

This label does not automatically imply version 1.0. A v1 release still requires explicit maintainer confirmation.

---

## 13. Companion dependency and import style

Canonical documentation should preserve this ownership model:

```text
matten owns Tensor.
companions add focused workflows around Tensor.
```

Official examples SHOULD prefer explicit user dependencies:

```toml
[dependencies]
matten = "0.19"
matten-ndarray = "0.19"
```

and canonical imports:

```rust
use matten::Tensor;
use matten_ndarray::to_arrayd;
```

In the current policy a companion MUST NOT re-export `matten`. The limited
single-dependency convenience path (`pub use matten;`) is **deferred by RFC-032**
(§3.3) and may be revisited only after demonstrated user demand and a follow-up
RFC/decision. The release-doc check (`scripts/check-release-docs.sh`) enforces this:
it fails if any companion contains `pub use matten`.

```rust
// FORBIDDEN in the current policy (RFC-032 §3.2/§3.3)
pub use matten;            // whole-crate convenience re-export: deferred
pub use matten::Tensor;    // broad core-type re-export: forbidden
pub use matten::MattenError;
pub use matten::Element;
pub use matten::NumericPolicy;
```

This policy keeps ownership, feature selection, maturity labels, and dependency/security review clear.


---

## 14. Companion error policy

Each companion crate defines its own error type.

Core `matten::MattenError` is for core tensor and boundary failures only. Companion crates may wrap `MattenError`, but core must not grow variants for companion-specific failure modes.

Bridge and conversion functions return `Result`:

```rust
to_arrayd(&tensor) -> Result<ArrayD<f64>, MattenNdarrayError>
```

Dynamic inputs to companion bridge/prep/data APIs should return `Err`, not panic, unless the API is explicitly documented as an internal panic-zone convenience.

---

## 15. Mechanical dependency-boundary gate

The v0.16 release must add a CI check proving that core `matten` has no forbidden dependency direction.

The check should fail if core `matten` depends on:

```text
ndarray
nalgebra
candle-core
polars
arrow
datafusion
matten-ndarray
matten-mlprep
matten-data
```

A script such as `scripts/check-core-dependency-boundary.sh` should run in CI. It
MUST inspect the core package with all features enabled so optional dependencies
behind non-default features cannot slip past:

```bash
cargo tree -p "$CORE_PACKAGE" --all-features --edges normal,build --no-dedupe
```

A plain `cargo tree -p matten` is insufficient: an `ndarray = { optional = true }`
dependency gated by a non-default feature would not appear, producing a false pass.


---

## 16. Document history

| Version | Date | Change |
|---|---|---|
| 1.0.0 | 2026-06-21 | First canonical v0.16+ roadmap after companion-crate reconciliation. |
| 1.1.0 | 2026-06-22 | Updated v0.20+ materialization plan. RFC-032 is reserved/consumed elsewhere, so v0.20+ planning starts at RFC-033. Added v0.19.1 hardening milestone, `matten-data` RFC sequence RFC-033–037, core comfort RFC-038+, companion dependency/import style, and later stats/linalg boundary themes. |
| 1.2.0 | 2026-06-22 | Reconciled to shipped reality and architect rulings (v0.19.3): §13 corrected so the companion `pub use matten;` convenience re-export is deferred per RFC-032 (release-doc guard forbids it); planning baseline corrected to lock-step family versions (no per-crate `0.1.x`); added v0.19.2 and v0.19.3 release-theme rows. |
| 1.3.0 | 2026-06-23 | Added examples program planning for RFC-043–048 and compact examples implementation handoff. Added v0.19.4 release-theme row; expanded v0.20+ to Track C for examples; added RFC-043–048 table entries, example groups, implementation order, acceptance gates, and non-goals. |
| 1.4.0 | 2026-06-23 | Reconciled the examples program to architect rulings (v0.20.2): new famous-problem examples use an additive 30+ band (existing 00-28 suite preserved); cosine/distance and companion examples are cross-referenced/audited, not duplicated; matten-data csv_to_tensor marked shipped in v0.20.1; docs path examples/index.md; CI smoke-list update requirement added. Fixed the v0.19.4 version regression: replaced with accurate v0.20.0/v0.20.1/v0.20.2 release-theme rows. |
| 1.5.0 | 2026-06-23 | Added the benchmarking & positioning program (RFC-049) as Track D: goal, posture/sequencing, phases, hard constraints, and acceptance gate; added RFC-049 to the v0.20+ RFC table; recorded the shipped v0.20.3/v0.20.4 example bands and the v0.20.5 benchmarking-planning row in the release-theme table. RFC-049 is non-API and measurement-only. |
| 1.6.0 | 2026-06-23 | Marked Track B (core numeric comfort APIs, RFC-038) complete: all four bands shipped across v0.20.9 (elementwise), v0.20.10 (selection), v0.20.11 (creation), and v0.20.12 (shape). Updated the RFC-038 row to Done and added a completion status note to §9.3. RFC-038 moved to `rfcs/done/`. |
| 1.7.0 | 2026-06-23 | Marked Track C (examples program, RFC-043–048) complete: shipped the deferred numerical examples 39–40 (finite-difference derivative, trapezoidal integration) in v0.20.13, which finishes the additive 30+ band. Closed RFC-043–048 to `rfcs/done/` with shipped-version annotations; added a §9.4 completion note and corrected the 39/40 lines from deferred to shipped. The optional `41_adjacency_walks_extended` remains a not-reserved conditional candidate. |
| 1.8.0 | 2026-06-23 | Ingested the v0.21 boundary architect rulings (RFC-039–042, all 13 questions accepted with added constraints). Marked RFC-039/040/041/042 accepted-for-implementation (Status updated; rulings recorded in each RFC) and set targets v0.21.0 (039 concatenate/stack), v0.21.1 (041 norm/trace/outer), v0.21.2 (040 var/std/var_axis/std_axis), v0.21.3 (042 scope guard). Added the v0.21.0–.3 release-theme rows. RFCs remain in `proposed/` per the 4-folder lifecycle until each ships. |
| 1.9.0 | 2026-06-23 | Architect accepted the v0.20 series handoff (phase closed) and completed a v0.20.14 codebase deep review (no P0; P1 documentation/release-truth findings). Applied the review as v0.20.15: Patch A (doc-truth cleanup — stale 0.15/0.19 strings → 0.20, root README crate table + matten-data row, public-API snapshot header/InvalidArgument/try_reshape row, matten-data and intro skeleton wording, operators matmul) and Patch B (hardened `check-release-docs.sh` with doc-truth checks). Patch C (RFC-023/026/036/037 lifecycle clarification, P2) deferred to v0.21 planning. Optional `41_` confirmed as a conditional candidate. |
| 1.10.0 | 2026-06-23 | Audited the project since v0.19.0 across four dimensions (codebase↔RFCs, tests↔requirements/external-design, codebase↔tests, docs↔codebase). Result: consistent. Confirmed accepted-but-unshipped RFCs (039–042) are not prematurely implemented, RFC-038/033–035/043–048 are implemented with accurate done-status, the full suite passes with zero ignored tests, and docs match the public surface. One documentation gap fixed in v0.20.16 (public-API snapshot Element method list). |
| 1.11.0 | 2026-06-23 | Ingested and applied the pre-v0.19.0 audit architect rulings (Q1–Q4) as v0.20.17. Q1: retired "Phase 1/Phase 2" wording from user-facing docs (48 occurrences across 14 files) in favor of numeric-Tensor/dynamic-ingestion terminology, plus a release-docs guard against reintroduction (history retained in rfcs/ and CHANGELOG). Q2: added an RFC-013 lifecycle note (property/fuzz testing is aspirational, not a current gate); tracked an optional future "Testing Strategy Refresh" (candidate RFC-050, after RFC-049). Q3: added RFC-014↔RFC-043 cross-references (RFC-043 is the current examples-program authority; RFC-014 historical). Q4: added an RFC-012 clarification (internal Arc-shared CoW implemented; public mutation API intentionally deferred). The separately-deferred Patch C (RFC-023/026/036/037 lifecycle) remains pending its own ruling. |
| 1.12.0 | 2026-06-23 | Build/repo hygiene (v0.20.18): git-ignored the RFC-031 fixture's Cargo.lock (`/tests/fixtures/*/Cargo.lock`) so the repository tracks a single workspace lock; the fixture stays excluded (feature-unification isolation is required for the regression). Clarified the root Cargo.toml exclusion comment. Evaluated and rejected a proposed members/exclude `tests/*` manifest change (it fails `cargo metadata` and would not remove the second lock). No code/API/behavior change. Also (same v0.20.18): pointed the README documentation link at the published mdBook (nabbisen.github.io/matten); added per-example source-code links across the examples pages; and retired four hyphenated `Phase-1` references missed by v0.20.17 (guard now matches `Phase[ -]1`). |
| 1.13.0 | 2026-06-24 | Examples reorganization (v0.20.19) per architect ruling: renamed the seven unnumbered skill-demos into a new `50_`–`56_` practical-recipes band; retired the `hello_tensor.rs` and `column_summary.rs` fossils; created docs/src/examples/practical-recipes.md and updated index.md/beginner-math.md/SUMMARY.md; fixed a stale "Phase 2" docstring in dynamic_00; updated CI smoke runs; added a naming-band guard (core examples must match `NN_` or `dynamic_NN_`). No public API or behavior change. |
| 1.14.0 | 2026-06-24 | Opened the v0.21 line with RFC-039 shape composition (v0.21.0): `concatenate` (existing axis) and `stack` (new axis) added to core as borrowed-slice associated functions with try_/panic pairs, MattenLimits allocation checks, and dynamic rejection; `repeat`/`tile`/`meshgrid` remain deferred. Added 20 unit tests + the `14_concatenate_stack` example; new reference page shape-composition.md and a public-API-snapshot section; RFC-039 moved to done/. |
| 1.15.0 | 2026-06-24 | v0.21.1: RFC-041 linalg core-lite. Added `norm` (L2/Frobenius over all elements, NaN-propagating, panic-only like `sum`/`mean`), `trace` (rank-2, rectangular via `min(rows,cols)`, with `try_trace`), and `outer` (rank-1 × rank-1 → `[m,n]`, MattenLimits-checked, with `try_outer`) in a new `linalg.rs` module (math.rs kept under 300 ELOC). Decomposition/inverse/determinant/eigen/SVD/QR/LU/Cholesky/sparse/BLAS remain rejected from core. Added 16 unit tests + the `15_norm_trace_outer` example; new reference page linalg.md (with the required "not a linear algebra backend" boundary wording) and a public-API-snapshot section; RFC-041 moved to done/. |
| 1.16.0 | 2026-06-24 | v0.21.2: RFC-040 statistics core. Added `var`/`std` and `var_axis`/`std_axis` (population variance, ddof=0; `var = sum((x-mean)^2)/n`; two-pass; NaN-propagating) in a new `stats.rs` module (math.rs kept under 300 ELOC), with `try_*` forms (`Unsupported` on dynamic, `Shape` on invalid axis, defensive `InvalidArgument` on the not-constructible empty case). Sample variance / quantile / percentile / histogram / covariance / correlation / z-score remain deferred; no `matten-stats` companion scaffolded. Added 14 unit tests + the `16_variance_std` example; new reference page stats.md and a public-API-snapshot section; RFC-040 moved to done/. |
| 1.17.0 | 2026-06-24 | v0.21.3: RFC-042 matten-data anti-scope guard — completes the v0.21 boundary-work batch. Added scripts/check-matten-data-scope.sh with three PRECISE checks (RFC-042 §8 / Q13): (1) example file-name guard (rejects dataframe-story names like join_customers_orders.rs), (2) public-API identifier guard over crates/matten-data/src (rejects pub DataFrame/Series types and pub groupby/join/merge/pivot/query/loc/iloc fns, matched as definitions), (3) positive README scope-statement check ("not a dataframe library"). Deliberately NO broad body-scan of index/join/loc/query (so Path::join, var index, joined/join_tables, location all pass). Wired into the matten-data CI job and the release checklist; tested against all RFC §8 must-fail / must-not-fail cases. No Rust code change. |
| 1.18.0 | 2026-06-24 | v0.21.4: applied the v0.21.3 deep-review P1 release-truth fixes. Architect confirmed Q1–Q5 (norm panic-only; var_axis/std_axis try_ forms; defensive empty guard; dedicated scope-guard script; new modules) and ruled Q6 = yes (wire release-docs guard into CI). Fixes: corrected 0.20→0.21 documentation drift across READMEs, lib.rs, quick-start, boundary/dynamic/architecture/introduction/compatibility, and the public-API snapshot (now family-only "current v0.21 family" to prevent future patch drift); retired "Phase 1" wording from four examples; made check-release-docs.sh current-family-aware (CURRENT_MINOR variable; rejects non-current install pins / X.Y.x family labels / "current vX.Y family" prose; allows historical full-patch refs and generic examples) and extended its retired-wording scan to examples; wired check-release-docs.sh into the CI check job; added scope-guard + release-docs guard to the release checklist. Deep review flagged two future-optional (pre-v1.0) consistency RFCs — Result-form reductions (try_sum/try_mean/try_min/try_max/try_norm) and try_*_axis — tracked, not required for v0.21. No library code change. |
| 1.19.0 | 2026-06-24 | v0.22.0: **matten-data promoted to Beta.** Implemented the full RFC-036 six-example suite (data_00_quickstart, data_01_schema_summary, data_02_select_columns, data_03_missing_values, data_04_to_tensor, data_05_errors), keeping csv_to_tensor.rs as the comprehensive overview (architect Option 1); added an explicit malformed-CSV test (`malformed_csv_is_a_structured_error_never_a_panic`, 34 tests total). Completes the RFC-023 §9 Beta gate. Flipped status Experimental→Beta across matten-data README, lib.rs, root README table, companions.md, compatibility.md; bumped family 0.21→0.22; added docs/src/examples/data.md (wired into SUMMARY + companions). Guards: CURRENT_MINOR 21→22 and a new matten-data-must-say-Beta check. CI: matten-data job gains `cargo check --examples` + `cargo test -p matten-data`; smoke job runs all six data_* examples. RFC-036 → Implemented, RFC-023 → Resolved (Outcome B → Beta); both moved to done/. **Finding surfaced:** the lenient+flexible csv config never emits parser errors for &str input (unterminated quote → structural RaggedRow; bad header → Csv), so the malformed-CSV test asserts the real no-panic/structured-error contract rather than a parser-error variant the config cannot produce. No library/API/behavior change. |
| 1.20.0 | 2026-06-24 | v0.22.1: RFC-049 accepted with a staged mandate; implemented **Phase 1 only** (PR-049-1 methodology docs + PR-049-2 internal Rust baseline harness). Added a workspace-excluded, publish=false `benchmarks/` criterion harness (workloads in a criterion-free lib; benches isolated), covering a core micro set + five scenario workloads from examples 26/33/34/35/36; methodology docs under docs/src/benchmarks/; an internal-baseline report template + results-commit policy. Extended the core dependency-boundary guard to forbid criterion in core matten's tree (§7); added a CI benchmarks job that compile-checks the harness only (no speed/memory gates, §5); git-ignored benchmarks/Cargo.lock. RFC-049 → Accepted (stays in proposed/ until fully implemented; Phases 2–4 deferred until Phase 1 yields a credible baseline report). Memory policy = Linux peak RSS via /usr/bin/time -v (informative, not a gate). No published-crate code/API/behavior change. |
| 1.21.0 | 2026-06-24 | v0.22.2: applied the v0.22.0 handoff-review P2 follow-up. Added a clarification note to RFC-023 §9 (and a pointer in RFC-036) recording that the malformed-CSV Beta-gate criterion is satisfied by a structured-error/no-panic malformed-input test — Csv or RaggedRow, never a panic or silently-wrong Table — rather than a low-level csv parser-error test, since the lenient flexible(true) &str reader resolves unterminated quotes to structural RaggedRow validation; a byte-level invalid-UTF-8 test is intentionally not added (no public path; tests the dependency, not matten-data). Historical CHANGELOG/ROADMAP entries left unchanged. No Rust code, API, guard, or CI change. |
| 1.22.0 | 2026-06-24 | v0.22.3: applied the benchmarking/positioning review. Part A (RFC-032 scope): Option A — added RFC-032 §5.1 confirming workspace-excluded publish=false internal tooling (the RFC-031 fixture, the RFC-049 benchmark harness) is outside the published, user-facing family convention's packaging scope, while still following ownership-clarity (no core-type re-export; import from matten); no change to benchmarks/ or the fixture; RFC-032 guard deliberately NOT extended to scan excluded tooling. Guards: added scripts/check-published-dependency-isolation.sh (RFC-049 §B1) proving each published crate is peer-dep-free — core/matten-data/matten-mlprep forbid criterion/ndarray/nalgebra; matten-ndarray forbids criterion/nalgebra but is allowed ndarray (bridge); passes today; wired into CI after the RFC-022 core guard and into the release checklist; negative-tested. Part B (RFC-049 Phase 2): design settled and annotated onto RFC-049 (B1 isolation guard, B2 structural peers-feature + fixed comparable-task list, B3 opt-in/off-by-default build & CI, B4 baseline-report entry precondition) and marked DESIGNED-NOT-AUTHORIZED; added benchmarks/reports/BASELINE-READY-CHECKLIST.md; updated methodology + report template. Phase 2 implementation deliberately NOT started (awaits a maintainer-run credible baseline report + separate authorization). No Rust code/API/runtime change. |
| 1.23.0 | 2026-06-24 | v0.22.4: **RFC-049 Phase 2 — Rust peer comparison (opt-in)**, plus the accepted Phase 1 internal baseline and a workspace-config fix. Architect accepted the maintainer-run baseline (Ubuntu 26.04, virtualized; Baseline ID matten-rfc049-internal-baseline-v0.1) and authorized Phase 2 under prior constraints. Implemented the peer harness in the workspace-excluded benchmark crate: a `peers` feature (ndarray+nalgebra optional deps) OFF by default; workloads/peers/{ndarray,nalgebra}_tasks.rs covering the fixed comparable set (cosine, small matmul, Markov, PageRank, linreg GD, heat), each documenting comparability; a required-features=[peers] peers bench giving a three-way matten/ndarray/nalgebra comparison from identical data; peer-comparison-v0.1.md template (limitations + non-ranking disclaimer, "Rust peer comparison" wording). Verified default --no-run compiles ZERO peer crates and the published-isolation guard still passes (peer deps never reach published crates). Separate benchmarks-peers.yml workflow (manual/weekly) compile-checks --features peers only — kept out of ordinary CI, no speed gates. Marked Phase 2 implemented across RFC-049/methodology/README; completed the accepted baseline report (real medians, peak RSS 44,728 kB, Baseline ID + acceptance marker, sum_mean_axis clarified). Fixed the workspace exclude glob (tests/fixtures/* does not expand in Cargo exclude) by making the RFC-031 fixture self-excluding with an empty [workspace] table. Recorded sum_mean_axis (~1.31 ms; ~400x sum_mean, ~17x 64x64 matmul) as a P2 performance-watch / regression-visibility anchor (not a fix-now item, not a Phase 2 blocker). Phase 3 (NumPy/Pandas) and hard speed gates remain unauthorized. No published-crate code/API/runtime change. |
| 1.24.0 | 2026-06-24 | v0.22.5: v0.22.4 deep-review release-truth reconciliation (docs/RFC/guard only; no library code, API, runtime, or benchmark-logic change). The architect's v0.22.4 codebase deep review accepted the Phase 2 harness/template (no P0/P1 source blockers) and requested status-text fixes; the companion benchmarking/positioning review accepted the baseline (archival-ready) + peer template and confirmed nalgebra-on-all-six + official-numbers-pending. Fixes: (P1) rewrote docs/src/benchmarks/index.md status (Phase 1 accepted; Phase 2 harness/template implemented, official numbers pending; only Phase 3/4 deferred); (P1) reconciled the RFC-049 header Status/Target/Acceptance with the already-updated Phase 2 body (removes the internal inconsistency); (P2) benchmarks/README.md title drops "Phase 1", peer command comment clarified to "ordinary CI ... manual/scheduled peers workflow"; (P2) peer-comparison-v0.1.md gained a top-level "Template only; official numbers pending; do not cite sandbox" marker and migration-tone interpretation guidance; (P2) methodology distinguishes harness/template-implemented from official-report-complete. Added a scoped benchmark-status-drift guard to check-release-docs.sh (flags docs/src/benchmarks describing Phase 2 as unimplemented; excludes RFC history/CHANGELOG; Phase 3/4 deferral still allowed; positive/negative tested; rides the existing CI release-docs gate). Official Phase 2 peer numbers still pending a maintainer run; Phase 3 + hard gates remain unauthorized. |
| 1.25.0 | 2026-06-24 | v0.22.6: accepted and ingested the production-migration RFC set (RFC-050–054) — planning/docs only, no library code/API/runtime/dependency change. Theme: a documented, honest "family-car → super-car" exit ramp from matten to heavier ecosystems (ndarray, nalgebra, Polars, Candle, NumPy, Pandas) with NO heavy dependency added to core matten; migration support lives in docs, bridge crates, and (later, if ever) workspace-excluded tooling. Added rfcs/proposed/050–054 (Production Migration Guide & Bridge Strategy; Bridge Conversion Contracts & Companion-Crate Policy; Production Target Playbooks; Migration Readiness Diagnostics & Report Format; deferred matten-migrate CLI) + the handoff bundle in rfcs/handoffs/ (implementation handoff, RFC-054 deferred note, acceptance/QA + release-guard checklists); updated rfcs/README.md index. Applied the architect's review-of-review ruling: resolved the RFC-050 number collision by KEEPING the migration set at 050–054 and renumbering the earlier Testing-Strategy-Refresh earmark to RFC-055 (rfcs/done/013 note updated); clarified RFC-051 §9 error categories are illustrative (matten-ndarray's DynamicTensor/ZeroSizedAxis/NdarrayShape/Matten is compliant as-is) and §15 audit is documentation-only (no new error variant); resolved RFC-051 §17 → to_<target>/from_<target> default naming; softened RFC-052 deprecated ndarray .into_shape wording and added a pending-peer-numbers acceptance rule (no numeric claims until official RFC-049 Phase 2 numbers accepted); added an RFC-054 workspace-excluded/publish=false placement note; made the release-guard checklist phrase-anchored only (no bare-word bans). Implementation of RFC-050–053 targets v0.23.0/v0.23.x; RFC-054 remains deferred; RFC-055 (testing refresh) remains a future candidate. |
| 1.26.0 | 2026-06-25 | v0.22.7: RFC-049 Phase 2 accepted — documentation reconciliation (docs/RFC/report wording only; no library code/API/runtime/dependency change). Architect accepted the official maintainer-run Rust peer comparison (commit 007031c, v0.22.6, baseline machine class) as the RFC-049 Phase 2 official report; Phase 2 is now COMPLETE, Phase 3 (NumPy/Pandas) + Phase 4 (hard gates) remain unauthorized, no optimization required. Official medians (matten/ndarray/nalgebra): cosine 674ns/231ns/160ns; matmul 64x64 118.9us/12.74us/16.23us; markov 1.03us/1.34us/1.41us (matten competitive/inverted); pagerank 6.21us/607ns/607ns; linreg 1.75us/769ns/832ns; heat 5.23us/556ns/565ns. Honest finding: peers generally lower-overhead on dense kernels (expected for DX-first matten); widest ~7-10x on matmul/pagerank/heat, modest ~2-4x on cosine/linreg, inverted on markov; matrix-vector path widest vs vector-matrix competitive (recorded as positioning/regression-visibility, NOT a defect). Reconciled peer-comparison-v0.1.md (acceptance marker + Report ID matten-rfc049-rust-peer-comparison-v0.1; natural-representation clarification; CORRECTED nalgebra note: 0.33.3 pinned by MSRV-1.85 floor via MSRV-aware resolver, 0.35 needs Rust 1.89 = future MSRV-policy decision, not a 1.93-toolchain constraint), benchmark docs/README/methodology (pending -> accepted), RFC-049 header/annotation/index, and added an RFC-052 task-scoped citation note (playbooks may now cite results; no ranking/faster-than/migration-mandate). RFC-049 stays in proposed/ until Phases 3-4 resolve. |
| 1.27.0 | 2026-06-25 | v0.23.0: production migration guide — first release (RFC-050 foundation + RFC-052 Rust playbooks). Docs only; NO library code/API/runtime/dependency change, core matten gains no dependency. First stage of the family-car -> super-car migration program (RFC-050-054); architect prioritized the Rust-target playbooks, delivered here. Added docs/src/migration/: index (migration promise: outgrowing matten is a successful PoC outcome; dependency-light; not an auto code-rewriter), when-to-migrate (stay-vs-migrate pressure signals), target-selection (workload->ecosystem matrix + decision path), common-pitfalls (row-major vs column-major, convert-once, f64/f32, dynamic->numeric); playbooks/index (decision tree) + full ndarray and nalgebra playbooks (choose/don't-choose/concept-mapping/example-migrations/conversion-path/pitfalls/positioning/checklist). ndarray playbook leads with the matten-ndarray bridge (to_arrayd/from_arrayd); nalgebra documents manual from_row_slice (no matten-nalgebra bridge yet). Positioning notes cite the ACCEPTED RFC-049 peer comparison task-scoped (no ranking/faster-than). Added a scoped migration overclaim guard to check-release-docs.sh (phrase-anchored, future/deferred exception, positive+negative tested); new SUMMARY Migration section; reference/migration.md cross-links the full guide (no duplication). Staged for v0.23.x: remaining RFC-052 playbooks (Polars/Pandas, Candle, NumPy), RFC-051 bridge-contract pages + matten-ndarray contract table, RFC-053 readiness diagnostics. RFC-054 (matten-migrate CLI) deferred. |
| 1.28.0 | 2026-06-25 | v0.23.1: production migration guide — RFC-052 completed (remaining target playbooks). Docs only; no library code/API/runtime/dependency change. Added the cross-paradigm/cross-language playbooks: polars-and-pandas.md (dataframe path; states matten-data is an on-ramp and will NOT grow group-by/join/pivot/query; enter the dataframe lib at the data source), candle.md (ML path; careful NOT to imply matten is an ML framework; f64->f32 boundary), python-numpy.md (Python scientific path; manual/conceptual serialization hand-off, no in-process Rust<->Python bridge). Each follows the standard 8-section playbook structure. Positioning notes state honestly that NO benchmark exists for these targets (cross-paradigm/cross-language = RFC-049 Phase 3, not authorized) — choose by capability/ecosystem fit, not measured speed. Moved the three targets from later-revision to available across playbooks/index, target-selection, index; SUMMARY lists all five playbooks. RFC-052 target set now complete. Still staged for v0.23.x: RFC-051 bridge-contract pages + matten-ndarray contract table, RFC-053 readiness diagnostics. RFC-054 (matten-migrate CLI) deferred. |
| 1.29.0 | 2026-06-25 | v0.23.2: production migration guide — RFC-051 bridge conversion contracts. Docs only; no library code/API/runtime/dependency change. Added bridge-contracts.md (13-dimension conversion-contract template + filled matten-ndarray reference contract, verified against convert.rs/error.rs: copies both ways, numeric-only, rejects dynamic via DynamicTensor (unconditional, not panic), preserves logical row-major through non-standard layouts, rejects zero-sized axes, Result never panics; RFC-051 error categories noted as illustrative not required) and bridge-crate-policy.md (own target dep; never re-export Tensor — confirmed matten-ndarray exports only to_arrayd/from_arrayd/MattenNdarrayError; to_<target>/from_<target> naming; future-bridge checklist; new bridges need separate approval; CI isolation guard). Added the contract table to crates/matten-ndarray/README.md; cross-linked docs/src/examples/companions.md; SUMMARY lists the two pages. RFC-051 acceptance criteria met. Still staged: RFC-053 readiness diagnostics (last in batch). RFC-054 (matten-migrate CLI) deferred. NOTE/FINDING: the v0.23.0 family bump left stale '0.22' version references across all four crate READMEs + ~15 doc locations, and the pin examples (matten = "0.22") now mis-pin users to the old family (caret excludes 0.23.x); the release-docs guard did not catch this. Recommended a focused version-string-hygiene + guard release next. |
| 1.30.0 | 2026-06-26 | v0.23.3: version-string hygiene + self-updating drift guard. Docs/release-tooling only; no library code/API/runtime/dependency change. FIXED: stale '0.22' strings -> '0.23' across all four crate READMEs, root README, core lib.rs rustdoc, and ~10 doc pages (quick-start, examples/data, reference/boundary, reference/dynamic, contributing/architecture, public-api-snapshot, plus 0.22.x/'current 0.22 family' labels). Install pins were a real bug, not cosmetic: caret 'matten = "0.22"' resolves >=0.22.0,<0.23.0, so copying it held users on the old family and hid the 0.23 migration guide. Historical refs (promoted to Beta in v0.22.0; per-family compatibility history) preserved; added a v0.23 family entry to compatibility.md. CHANGED: check-release-docs.sh version guard now derives the current minor DYNAMICALLY from Cargo.toml instead of a hardcoded CURRENT_MINOR=22 — that hardcoded value (manual-bump-per-release) was missed at v0.23.0, which is precisely why the stale 0.22 pins shipped unflagged. Guard keeps 'family' adjacency so generic patch-notation like (0.13.x) is not flagged; verified green on corrected docs and that a simulated 0.24.0 bump immediately flags stale 0.23 strings. Root cause of the v0.23.0-0.23.2 stale-version finding now closed. Next: RFC-053 readiness diagnostics (last in the migration batch); RFC-054 (matten-migrate CLI) deferred. |
| 1.31.0 | 2026-06-26 | v0.23.4: production migration guide — RFC-053 migration-readiness diagnostics (COMPLETES the RFC-050-053 migration batch). Docs only; no library code/API/runtime/dependency change. Added readiness-checklist.md (10 pressure signals — data-size/runtime/axis-reduction/linear-algebra/dataframe/ML-device/dynamic-ingestion/dependency-policy/ecosystem/team-language — each mapped to a target playbook, with explicit stay-with-matten outcomes; advisory, no source-scanner), readiness-report.md (manual fillable template with the 9 required sections: Summary/Current usage/Pressure signals/Recommended targets/Direct conversion candidates/Manual redesign areas/Bridge crates-tools/Risks/Next steps + required advisory disclaimer), and examples/linear-regression-gd-readiness.md (template filled for 35_linear_regression_gradient_descent, written against its actual structure: two matmul/step + reused transpose + iterative loop; recommends moving per-step matrix products to ndarray via the matten-ndarray bridge at real sizes, nalgebra closed-form as optional redesign, stay-with-matten at toy size). SUMMARY + migration/index link the pages. Guard: migration overclaim check now allows the negated advisory disclaimer (does not perform automatic conversion) while still flagging positive automatic-conversion claims (verified both directions). MIGRATION PROGRAM COMPLETE: RFC-050 (foundation) + RFC-051 (bridge contracts) + RFC-052 (all playbooks) + RFC-053 (readiness) done; RFC-054 (matten-migrate CLI) deferred. |
| 1.32.0 | 2026-06-27 | v0.23.5: RFC-050-054 deep-review response (P1+P2) and migration-batch lifecycle close. Docs/release-tooling/RFC-bookkeeping only; no library code/API/runtime/dependency change. P1 FIXED: restored 13 CHANGELOG release headings (v0.23.3 back to v0.21.4) that a heading-eating regression had nested under a single ## [0.23.4] — block content was intact, only headings lost; versions/dates taken from this ROADMAP history. P1 FIXED: Candle snippet in reference/migration.md now builds the Tensor from f64 data (was vec![1.0f32,..], but Tensor is f64) and casts to f32 only at the Candle boundary. P2: reference/migration.md softened 'always one line', made ndarray section bridge-first (matten_ndarray::to_arrayd) with manual ArrayD::from_shape_vec as fallback, replaced brittle 'four exports' wording with a pointer to the public-API snapshot/CHANGELOG; check-release-docs.sh gained a CURRENT_MINOR extraction sanity check and a CHANGELOG-heading guard (top heading must equal workspace version; no release block may hold >1 ### Threat model — the lost-heading signature; both tested). LIFECYCLE: RFC-050/051/052/053 -> Implemented and moved proposed/ -> done/ (v0.23.0, v0.23.2, v0.23.0-v0.23.1, v0.23.4); rfcs/README Done/Proposed tables reconciled; RFC-054 stays proposed as accepted-future-direction with deferral confirmed. Architect deep review (2026-06-27) accepted RFC-050-053 as implemented and approved the done/ move after P1. |
| 1.33.0 | 2026-06-27 | v0.24.0: Result-form reductions — complete the fallible reduction surface (RFC-055 scalar + RFC-056 axis). Additive public API in core matten; no new dependency, no breaking change, f64-only. ADDED: try_sum/try_mean/try_min/try_max/try_norm (Result<f64>; Unsupported on dynamic; NaN propagates as a value) and try_sum_axis/try_mean_axis/try_min_axis/try_max_axis (Result<Tensor>; Shape on out-of-range axis, Unsupported on dynamic, dynamic checked first to match try_var_axis; reduced axis removed from output shape). CHANGED: panic forms now delegate to their try_ engines via unwrap_or_else(panic!) — same pattern var/std already use, so forms cannot diverge; behaviour unchanged (still panic on dynamic / bad axis), but panic-message TEXT now comes from MattenError Display (architect-accepted; text is not API contract). norm RULING REVERSED (deep review 2026-06-27): prior v0.21 norm-panic-only decision reversed, try_norm added, rustdoc corrected. No new MattenError variant; reused Shape (axis) / Unsupported (dynamic). Internal: shared reject_dynamic helper (now reused by stats reductions too) + check_axis helper. Tests: try_ Ok-path equals panic form, axis==rank/>rank -> Shape, dynamic -> Unsupported (incl. precedence), panic forms still panic, no exact-panic-string asserts; doctests on each try_ form; lean/dynamic/all-features + MSRV 1.85 green. Family bump 0.23.5->0.24.0; user-doc pins/labels retargeted 0.23->0.24 (caught by self-updating drift guard); compatibility.md v0.24 entry; public-api-snapshot lists the 9 new methods. Architect accepted both RFCs full set (no P0; P1 = rustdoc/error-contract clarity, met). RFC-055/056 -> rfcs/done/. |
| 1.34.0 | 2026-06-27 | v0.24.1: v0.24.0 deep-review response (P1+P2+optional P3). Docs/release-tooling/test only; no library code, public API, runtime behaviour, or dependency change. Architect deep review (2026-06-27) accepted v0.24.0 as the correct RFC-055/RFC-056 implementation (no P0; all main claims verified by static source review) subject to one P1 release-truth fix. P1 FIXED: docs/src/introduction.md still said 'current 0.23 family' — updated to current 0.24 family + RFC-055/056 reduction-surface completion (layered on v0.23 migration guide, v0.22 matten-data Beta). The stale line used the un-prefixed 'current 0.N family' spelling, which the release-docs guard missed because it only matched 'current v0.N family'. P2 FIXED: check-release-docs.sh current-family-prose check now matches 'current v?0.N family' (optional v prefix), so a stale family ref can no longer hide behind a spelling difference; verified green on corrected docs and that both spellings of a non-current minor are flagged. P3 (optional) ADDED: try_axis_reductions_on_vector_give_scalar — rank-1 try_*_axis(0) scalar-output test for all four axis try forms (collapse to scalar identically to panic forms). Patch bump 0.24.0->0.24.1 (minor unchanged, so no user-doc version retarget needed). Full gate green (fmt, 4 guards incl. broadened family check, lean/dynamic/all-features tests + doctests, MSRV 1.85, RFC-031 fixture). Also carried (no separate changelog entry, per maintainer): crates/matten/README.md documentation link now points to the published nabbisen.github.io mdBook. Architect: after the P1 fix lands, v0.24.0 is review-clean. |
| 1.35.0 | 2026-06-27 | v0.24.2: test-organization refactor — co-locate unit tests with their modules. Internal only; no library code, public API, runtime behaviour, or dependency change; no test added/removed/re-gated (counts identical to v0.24.1: lean 243, dynamic 323, all-features 355 main + 100 doctests). Prompted by maintainer: the centralized src/tests/ tree was unfamiliar/non-standard and the testing guideline was clarified to require co-location. AUDIT FINDING: no inline #[test]/mod tests{} blocks anywhere (tests were already externalized) — and the layout was already INCONSISTENT (src/ops/broadcast.rs already co-located via src/ops/broadcast/tests.rs while 22 files sat in src/tests/). MIGRATED: 18 test files src/tests/<mod>.rs -> src/<mod>/tests.rs, each wired by #[cfg(test)] mod tests; in its parent (matching the broadcast precedent); removed src/tests.rs, src/tests/, and the central mod tests; in lib.rs. The two non-eponymous test files co-located with the module they exercise: shape_ops -> src/tensor/ops/tests.rs, elementwise -> src/ops/elementwise/tests.rs. SPLIT math tests (~478 lines) into themed groups src/math/tests/{whole,axis,matmul,dynamic}.rs (matmul group added because math.rs also covers dot/matmul, which the original 3-group suggestion overlooked); all groups well under 300. dynamic test sub-tree moved intact to src/dynamic/tests/ with feature-gating preserved (still skipped in lean). Migration verified behavior-neutral via absolute crate:: paths (no super::, no cross-file test helpers). Full gate green: fmt, 4 guards, lean/dynamic/all-features + doctests, MSRV 1.85, RFC-031 fixture. Patch bump 0.24.1->0.24.2 (minor unchanged, no user-doc version retarget). Also corrected a stale RFC-013 earmark (future Testing Strategy Refresh candidate RFC-055->RFC-057, since 055/056 took the v0.24 reductions). |
| 1.36.0 | 2026-06-27 | v0.24.3: fix an unused-import warning in the split math dynamic tests. Test-only; no library code, public API, runtime behaviour, or dependency change. FIXED: crates/matten/src/math/tests/dynamic.rs (created by the v0.24.2 split) imported MattenError+Tensor unconditionally, but all its tests are #[cfg(feature="dynamic")], so the import was unused in any non-dynamic build (incl. default cargo test/clippy) — flagged by the release checklist's cargo clippy -D warnings step. Now feature-gated (#[cfg(feature="dynamic")] use crate::{MattenError, Tensor};) to match the gated tests: neither unused without the feature nor missing with it. Verified: a maintainer-uploaded fix that DELETED the import was rejected because it breaks --features dynamic (E0433 cannot find Tensor/MattenError); the gated-import form is the correct fix. Clippy clean across default/lean/dynamic/all-features (-D warnings). PROCESS NOTE: v0.24.2 shipped with this warning because the local pre-tarball gate skipped the checklist's clippy -D warnings steps (lines 16-19) — those steps already existed and CI enforces them; they are now run on every release. No gate/CI change was needed (the protection already existed). Separately FLAGGED (not fixed here, out of scope): matten-data example data_02_select_columns lacks required-features=["csv"], so it fails to build under a workspace-wide --no-default-features --all-targets clippy (CI/checklist use -p matten for the lean clippy, so neither hits it). Patch bump 0.24.2->0.24.3. |
| 1.37.0 | 2026-06-27 | v0.25.0: companion-maturity line opens — promote matten-ndarray to production-ready (RFC-057). Label/docs/CI only; no API, runtime, error-variant, or dependency change to any crate; core matten unchanged. Architect accepted RFC-057 (no P0) with one required condition: examples must EXECUTE in CI, not just compile (P1). Applied: matten-ndarray status candidate->production-ready in crate README + lib.rs + Cargo.toml description + workspace README crate table + external-design maturity progression (added v0.25.0 entry; v0.19.0 candidate entry kept as history) + ROADMAP gate marked passed. bridge examples (to_arrayd/from_arrayd) were ALREADY executed in CI by the pre-existing smoke job — RFC-057's initial audit examined only the bridge/check jobs and missed smoke, so its 'compiled-not-executed' gap was inaccurate; no CI change was needed (P1 already met). Verified locally both examples print ok. API-snapshot file SKIPPED per architect ruling (two-function surface; README conversion-contract table is the snapshot-equivalent — verified it states both fns, error enum, copy, dynamic rejection, ndarray 0.16 minor, zero-axis rejection (made explicit), no-zero-copy). P2 stale-label guard added to check-release-docs.sh (context-aware: fails if matten-ndarray's own status files still say candidate; historical CHANGELOG/RFC/migration refs untouched). NOT v1.0 (status label only; lock-step family version retained; v1.0 needs explicit maintainer confirmation). matten-mlprep / matten-data stay Beta (separate decisions). Minor bump 0.24.3->0.25.0; user-doc pins/labels retargeted 0.24->0.25 (drift guard); introduction.md + compatibility.md describe the v0.25 family. Full gate green incl. clippy -D warnings + CI example execution. RFC-057 -> rfcs/done/. |
| 1.38.0 | 2026-06-27 | v0.26.0: companion-maturity continues — promote matten-mlprep Beta->production-ready candidate (RFC-058). Label/docs only; no API, runtime, error-variant, or dependency change; core matten unchanged; matten-mlprep stays matten-only. Architect accepted RFC-058 (no P0): rung is production-ready CANDIDATE, full production-ready DEFERRED because train_test_split is ordered-only (no shuffle) — a real caveat fitting 'usable seriously if documented limits acceptable'. Audited against candidate signals (verified, incl. CI): 17-test suite (determinism/zero-variance/NaN/shape/split edges), all FOUR examples executed in smoke job (namespaced [[example]] targets), non_exhaustive MattenMlprepError w/ Display+source, documented compat/MSRV policy, matten-only dep, Beta gate re-verified. Applied: status Beta->candidate in crate README + lib.rs (Status rewritten: stable small surface, ordered-split caveat noted) + workspace README table + external-design progression (added v0.26.0; v0.19.0 beta entry kept as history) + compatibility.md v0.26 entry + ROADMAP gate marked passed. Cargo.toml description verified maturity-neutral (no change). API-snapshot file SKIPPED per architect ruling (README Public API block + rustdoc is snapshot-equivalent; verified 8 elements incl. ordered-split, zero-variance, matrix-shape, dynamic rejection, no-ML scope, matten-only dep). P1: no stale Beta wording (added context-aware mlprep stale-label guard to check-release-docs.sh; mirrors ndarray guard) + all four examples still execute in CI; README/rustdoc accurately describe ordered-only split. P2: future full-production-ready exit criteria recorded (RFC-058 §5.1, Options A/B/C). NOT v1.0 (status label; lock-step family version retained). matten-data stays Beta (separate decision; may bundle in a future minor but did not block this RFC). Minor bump 0.25.0->0.26.0; user-doc pins/labels retargeted 0.25->0.26 (drift guard); introduction.md + compatibility.md describe v0.26 family. Full gate green incl. clippy -D warnings. RFC-058 -> rfcs/done/. |
| 1.39.0 | 2026-06-27 | v0.27.0: companion-maturity line COMPLETES — promote matten-data Beta->production-ready candidate (RFC-059). Label/docs/packaging only; no API, runtime, error-variant, or dependency change; NO scope expansion (RFC-042 lock preserved — still a CSV->tensor on-ramp, not a dataframe engine); core matten unchanged. Architect accepted RFC-059 CONDITIONALLY (no P0): two promotion-blocking hygiene fixes required first, both applied + verified: (1) stale Cargo.toml description 'Experimental...' (stale since v0.22.0 Beta) -> maturity-neutral 'CSV/table-to-Tensor preparation companion for matten (small PoC datasets).'; (2) data_00-data_05 examples lacked required-features=[csv] (cargo build --examples --no-default-features failed E0599 from_csv_str) -> added [[example]] entries, verified the --no-default-features build now succeeds by skipping gated examples + all 7 still execute in smoke. Audited vs candidate signals (verified incl. CI): 34 tests (most-tested companion), all 7 examples executed in smoke, 11-variant non_exhaustive MattenDataError w/ Display+source, compat/MSRV policy, RFC-042 anti-scope guard, own clippy gate. Rung: production-ready CANDIDATE; hold-at-Beta REJECTED (findings are packaging hygiene, not immature runtime); full production-ready DEFERRED (newest companion, wide CSV edge-case surface, streaming deferred) - separate future review. Applied: status Beta->candidate in crate README+lib.rs + workspace README table + companions.md/data.md/index.md (index.md was stale 'Experimental') + external-design progression (v0.27.0 entry; v0.19/0.22 history kept) + compatibility.md v0.27 + ROADMAP Track-A resolution note. README line-8 history extended (promoted to Beta v0.22.0, then candidate v0.27.0). API-snapshot file SKIPPED per ruling (README Public API block + rustdoc snapshot-equivalent; larger surface so kept exact: 7 types/methods + csv-feature/missing-value/numeric/scope-lock/error behaviors). P2: updated the v0.22.0 'must say Beta' check to enforce candidate, context-aware (historical Beta narrative + compatibility per-family history allowed; lead label/lib.rs/Cargo.toml checked). NOT v1.0. Minor bump 0.26.0->0.27.0; pins/labels retargeted 0.26->0.27 (drift guard); introduction.md + compatibility.md describe v0.27 family. Full gate green incl. clippy -D warnings + RFC-042 scope guard + no-default-features example build. RFC-059 -> rfcs/done/. NOTE for future tidy (out of scope): matten-ndarray Cargo.toml description embeds 'Production-ready' — the same neutrality principle applied here would suggest making it neutral too. |
| 1.40.0 | 2026-06-27 | v0.27.1: documentation & packaging legibility (RFC-060 + RFC-061). Docs/metadata only; no code, API, runtime, or dependency change; maintainer-authorized (docs-only, not an architect-ruling cycle). RFC-060: added docs/src/benchmarks/results.md (wired into SUMMARY under Benchmarks) — a CURATED summary of the accepted Phase 1 internal baseline + Phase 2 Rust peer comparison with representative medians and every RFC-049 caveat (workload/environment-specific, machine class + commit, accepted Baseline/Report IDs, 'not a ranking / not a faster-than claim'); full reports in benchmarks/reports/ stay the single source of truth; harness isolation preserved (no criterion/nalgebra in book/workspace/published graph). Added a release-docs freshness guard tying the page's cited Baseline/Report IDs to the report files. RFC-061 (maintainer chose Option D): kept the term 'production-ready'; added a small clarifying note at the TWO doc entrances only (root README by the crate table; mdbook introduction) that maturity labels describe stability within matten's documented PoC/small-data scope, NOT performance/scale — no rung renamed, no per-occurrence qualifier. Also applied the agreed description-neutrality tidy: matten-ndarray Cargo.toml description 'Production-ready conversion bridge...' -> 'Conversion bridge...'; all four crate descriptions now maturity-neutral (matten-ndarray maturity unchanged — still production-ready in README/lib.rs/table). PATCH bump 0.27.0->0.27.1 (minor unchanged; no family-label retarget). RFC-060/061 -> rfcs/done/. Reduced docs gate green (fmt, 4 guards incl. new freshness guard, cargo check). |
| 1.41.0 | 2026-06-27 | v0.28.0: matten-ndarray supported ndarray version 0.16->0.16+0.17 (RFC-062). Public-dependency compatibility event (the bridge exposes ndarray::ArrayD<f64> in to_arrayd/from_arrayd, so the supported ndarray minor is part of its PUBLIC type identity) — NOT a routine cargo update. Architect ACCEPTED Option B (range >=0.16.1, <0.18), subject to a compatibility-matrix CI; no P0. Requirement widened from "0.16" to ">=0.16.1, <0.18"; Cargo resolves ndarray to the consumer's minor (a project with no other ndarray dep gets latest-in-range 0.17.2). DECISION-DETERMINING verification (per the ruling's hard line): the UNCHANGED bridge source compiled + passed 17 conversion tests + 3 doctests + both examples against BOTH 0.16.1 AND 0.17.2 via cargo update -p ndarray --precise — NO version-conditional code needed, so Option B holds (fallback to Option A not triggered). P1 satisfied: added CI matrix job bridge-ndarray-compat (ndarray=[0.16.1,0.17.2]; test + --doc + both examples per pin, fresh checkout so per-job --precise lock edits are not committed); docs state public type identity + yanked-0.17.0 caveat (not a tested target) + docs.rs-renders-one-minor caveat (README Compatibility + Supported-ndarray bullet + lib.rs rustdoc); core matten remains ndarray-free (published-dependency-isolation green). MSRV untouched (1.85; ndarray 0.17.2 declares rust-version 1.64). No bridge API/signature/behavior/copy-semantics/dynamic-rejection/error/zero-copy change. Committed Cargo.lock resolves ndarray to 0.17.2 (latest in range). Family minor 0.27.1->0.28.0 (lock-step RFC-030; whole family bumps though only matten-ndarray materially affected); pins/family labels retargeted 0.27->0.28; introduction.md + compatibility.md describe v0.28 family. RFC-049 peer benchmark (snapshot at ndarray 0.16.1) deliberately NOT re-run — future separate task (out of scope per ruling §9). P2 (deferred, low): release-checklist item for future public-dependency-minor changes. Full gate green. RFC-062 -> rfcs/done/. |
| 1.43.0 | 2026-06-28 | v0.28.1 (FINAL, unpublished — consolidates the prior 1.42.0 entry): matten-ndarray ndarray support NARROWED to Option A. (A) RFC-062 reversal: maintainer chose Option A (ndarray = "0.17", single-version) over the architect-accepted Option B range (>=0.16.1, <0.18) that shipped in v0.28.0 — to keep Cargo.toml simple/readable; ndarray 0.17 is a small backwards-compatible upgrade so the range's only benefit (sparing 0.16 users) was judged not worth the baggage. NOT a CI-forced fallback (bridge compiled fine on both minors); a legibility judgment call. Architect ruling pre-listed Option A as acceptable (§3.1/§13) -> applied directly, no re-review. Cargo resolves ndarray to 0.17.2 (latest non-yanked 0.17 patch). REMOVED the bridge-ndarray-compat CI matrix (one supported minor -> standard bridge job against resolved 0.17.2 suffices). Docs simplified: matten-ndarray README compatibility + Supported-ndarray lines + lib.rs rustdoc + compatibility.md v0.28 entry + introduction.md -> 'supports the 0.17 minor' (resolved minor still part of public type identity; 0.17.0 yanked -> use non-yanked patch); range-specific docs.rs/multi-minor caveats dropped. RFC-062 (in done/) amended: header status -> Option A as of v0.28.1 + Addendum recording the reversal. v0.28.0 CHANGELOG entry (Option B range) PRESERVED as the delivered tarball's record; [0.28.1] now documents the narrowing. No bridge API/behavior/error/copy-semantics/zero-copy change; core matten ndarray-free; MSRV 1.85 holds with 0.17.2. (B) RFC-062 P2 RESOLVED: 'Public-dependency-minor changes' gate added to release-checklist.md (precedent ref updated: single-version vs range both covered; no longer cites the removed matrix). (C) Entrance README.md: small dynamic on-ramp example (Element heterogeneous tensor -> try_numeric_with(NumericPolicy::default().none_as(0.0)) -> clean f64 [1.0,2.5,0.0,4.0]); verified compiles+runs under --features dynamic; off-by-default + dynamic-guide link. Held at 0.28.1 (v0.28.0 and this revision both unpublished); minor unchanged, no family-label retarget. Full gate green (code release: dependency requirement change). No open RFC-062 items remain. |
| 1.44.0 | 2026-06-28 | v0.28.2: benchmark docs/reports only (no code/API/runtime/dependency change; maintainer-directed). (A) BENCHMARK RESULT REFRESH: added v0.2 reports (internal-baseline-v0.2.md, peer-comparison-v0.2.md) from a maintainer run at workspace 0.28.1 (commit ef06369, rustc 1.93.1, same 8vCPU AMD VM class as v0.1) under the UNCHANGED RFC-049 methodology. New IDs ...-v0.2. Done as a VERSIONED refresh (not an in-place overwrite) because the v0.1 reports are architect-ACCEPTED artifacts with v0.1-suffixed IDs and the v0.x naming was designed for refreshes; v0.1 retained as the accepted reference with a 'superseded for current numbers' banner. v0.2 numbers match v0.1 within VM variance (no internal regression v0.22.x->v0.28.1; sum_mean_axis still ~400x sum_mean; peer pattern holds — markov competitive/inverted, matmul/pagerank/heat ~8-11x). Peak RSS NOT captured this run (VM lacked GNU /usr/bin/time; informative-only, never a gate) — noted honestly. v0.2 explicitly labeled maintainer-run, NOT separately architect-reviewed (methodology+program remain accepted). (B) ENV-CAPTURE SNIPPET: added the missing runnable capture block (generalized from the maintainer's bench-01 script, stale '(0.22.3)' comment dropped) to benchmarks/README.md under a new 'How to regenerate (with environment capture)' section that consolidates capture+compile-check+timings+memory+peers; methodology.md Environment-recording now points to it; README's duplicate Running/Memory sections folded in. (C) TWO-AUDIENCE RESTRUCTURE: book benchmarks index routes reader -> results.md (curated readable summary, refreshed to v0.2 numbers, reframed as the reader view) vs maintainer -> methodology.md + harness README regenerate section. RFC-060 freshness guard extended to also map the v0.2 report IDs. PATCH bump 0.28.1->0.28.2 (minor unchanged; no family-label retarget). Reduced docs gate green. Note: 0.28.0/0.28.1/0.28.2 all unpublished. |
| 1.45.0 | 2026-06-28 | v0.28.3: benchmark-harness config only (no published crate touched; maintainer-directed). Bumped the out-of-workspace peer benchmark's optional ndarray pin 0.16->0.17 (benchmarks/Cargo.toml) so the peer comparison tracks the bridge (which moved to ndarray 0.17 in v0.28.x) rather than lagging a minor. Reasoning (maintainer asked 'any reason NOT to bump given the implementation was bumped?'): none of substance — the 0.16 pin was lag, not a deliberate choice; VERIFIED the one thing that could have blocked it: the peers bench compiles clean against ndarray 0.17.2 (lock 0.16.1->0.17.2, no errors). Harness is publish=false / excluded from the workspace, so no published crate, workspace dep, or public API changes. SEQUENCING (honest transient): the v0.2 peer numbers were measured at 0.16.1 and predate the pin; refreshed 0.17 numbers must come from the maintainer's machine class on the next peers run (NOT generated in-container — environment consistency), so peer-comparison-v0.2.md + results.md now state pin=0.17, numbers=0.16.1-pending. Split out from v0.28.2 at maintainer direction (initially folded into the unpublished v0.28.2; maintainer required it be its own v0.28.3 — v0.28.2 restored to benchmark-docs/reports-only). PATCH bump 0.28.2->0.28.3. Reduced docs gate green. 0.28.0-0.28.3 all unpublished. |
| 1.46.0 | 2026-06-28 | v0.28.4: benchmark results refresh + dependency-sync drift guard (no published crate touched; maintainer-directed). (A) DRIFT GUARD: added scripts/check-benchmark-dependency-sync.sh — parses the workspace ndarray requirement (root Cargo.toml [workspace.dependencies]) and the harness peer pin (benchmarks/Cargo.toml) and FAILS if they diverge. The benchmark harness is workspace-excluded so it can't inherit { workspace = true }; the pin is manual, and this guard makes 'forgot to sync' impossible to miss (the exact v0.28.3 situation). Verified: passes when both 0.17, fails with a clear fix-it message when harness set to 0.16. Wired into CI check job (after published-dependency-isolation) + release-checklist source verification + referenced from the RFC-062 P2 public-dependency-minor checklist item. (B) BENCHMARK REFRESH: replaced the v0.2 reports (internal-baseline-v0.2.md, peer-comparison-v0.2.md) + reader results.md with a fresh v0.28.3 run (commit 5953c9f, same 8vCPU AMD VM, rustc 1.93.1). KEY: the peer comparison now runs at ndarray 0.17.2 (env-capture log showed ndarray 0.16.1->0.17.2) — matching the shipped bridge, resolving the 'measured at 0.16.1, pending 0.17 refresh' caveat from v0.28.3. v0.2 IDs kept (v0.2 was never an accepted/frozen artifact, unlike v0.1; user said 'replace the existing'). Internal numbers within VM variance of v0.1; relative peer positioning unchanged (markov competitive/ahead of BOTH peers ~924ns; matmul/pagerank/heat ~7.5-9x; cosine/linreg 2-4x). Absolute peer timings ~40% lower than the 0.16.1 run but ALL THREE libs moved together = VM-load effect, not a code change; noted honestly (positioning is the durable signal). Peak RSS again not captured (no GNU time on VM). v0.1 remains the architect-accepted reference; v0.2 maintainer-run, not separately reviewed. PATCH bump 0.28.3->0.28.4. Reduced docs gate + new guard green. 0.28.3 PUBLISHED; 0.28.4 is the next release. |
| 1.47.0 | 2026-06-28 | v0.28.5: dynamic-JSON ingestion example + equal-on-ramps framing (docs/examples only; maintainer-directed). Motivated by a maintainer question — JSON felt unsupported. Audit found JSON support itself is solid (default `json` feature; from_json/from_json_dynamic/load_json; thorough boundary.md ## JSON section; core examples 10_json_roundtrip + 11_csv_numeric_loading symmetric and indexed). The ONE asymmetry was in the dynamic on-ramp examples: from_csv_dynamic had two dedicated examples (dynamic_02 missing values, dynamic_05 dirty cleanup) while from_json_dynamic appeared only inside dynamic_00. (data.md being CSV-only is correct — it's the matten-data companion page, CSV-only by RFC-042.) FIX: added crates/matten/examples/dynamic_08_json_ingestion.rs mirroring dynamic_02's structure exactly — cfg(not dynamic) fallback main; cfg(json) from_json_dynamic("[[1, 2.5, null], [4.0, 5, 6]]") with cfg(not json) from_elements fallback; demonstrates count_none/none_mask, strict try_numeric Err, then try_numeric_with(NumericPolicy::default().none_as(0.0)) -> clean [1.0,2.5,0.0,4.0,5.0,6.0]. All APIs verified against source before writing (from_json_dynamic accepts nested 2D mixed int/float/null; null->None, int->Int, float->Float per dynamic/parse/json.rs; as_slice/is_dynamic/none_mask/try_numeric_with confirmed). Verified runs under dynamic,json and compiles clean under dynamic-only + no-features (both fallbacks, zero warnings). No [[example]] entry needed (compiles feature-less like siblings 00-07; only 10/11/12 carry required-features). FRAMING: index.md dynamic section now states from_json_dynamic and from_csv_dynamic are equal on-ramps differing only in format + lists dynamic_08 + adds a json run line; dynamic_07 Step 1 comment made format-neutral (was 'a CSV row'). Wired dynamic_08 into CI smoke (--features dynamic,json). PATCH bump 0.28.4->0.28.5. Gate: fmt, 5 guards, clippy all-targets all-features, build --examples all-features, ran dynamic_08 + siblings, release-docs — all green. 0.28.4 PUBLISHED; 0.28.5 next. |
| 1.48.0 | 2026-07-03 | v0.29.0-pre.1: RFC-063 Phase 1 visual-understanding docs prerelease. Added RFC-063 + compact Phase 1 handoff; implemented Markdown/ASCII-only diagrams across operators, shape ops, math, shape composition, statistics, dynamic, matten-data, and tutorial start-here. Scope remained docs/RFC/handoff only: no public API, runtime behavior, dependency, tool, generated artifact, image asset, or maturity-label change. RFC-063 stays in `rfcs/proposed/` as an umbrella: Phase 1 implemented; Phase 2 examples require a compact handoff; Phase 3 tooling and Phase 4 companion crates require later approval. Current-family docs retargeted to exact prerelease pins (`0.29.0-pre.1`), and the release-doc guard now accepts SemVer prerelease versions/pins. |
| 1.49.0 | 2026-07-03 | v0.29.0-pre.2: RFC-063 Phase 2 canonical visual-summary examples prerelease. Added exactly the accepted first implementation set: `57_visual_shape_axis_summary`, `dynamic_09_visual_readiness_summary`, and `data_06_visual_readiness_summary`; helpers remain local to examples, output is deterministic/plain terminal text, and no public API/dependency/tool/image/generated-artifact/plotting/notebook/GUI scope was added. Wired canonical examples into CI smoke + release checklist; `data_06` is CSV feature-gated; dynamic example compiles without `dynamic` and runs with it. Example docs and tutorial path link the new readability summaries without user-facing process-phase wording. RFC-063 remains proposed as umbrella; Phase 3+ requires later approval. Current-family docs retargeted to exact prerelease pins (`0.29.0-pre.2`). |
| 1.50.0 | 2026-07-03 | v0.29.0-pre.3: RFC-063 optional `matten-mlprep` visual-standardization summary prerelease. After Phase 2 acceptance, added exactly one standardization-only companion example, `mlprep_visual_standardize_summary`, showing before/after column mean, before/after column std, and unchanged shape using deterministic hard-coded data. Also applied accepted Phase 2 P2 wording polish: dynamic readiness output now says converted shape/values instead of clean shape/values. Wired the mlprep example into CI smoke + release checklist + companion docs. No public API, dependency, tool, generated artifact, image, plotting, notebook, GUI, runtime, MSRV, or maturity-label change. Phase 3 tooling remains deferred. Current-family docs retargeted to exact prerelease pins (`0.29.0-pre.3`). |
| 1.51.0 | 2026-07-04 | v0.29.0-pre.4: RFC-063 Phase 3 first local-tool prerelease. Added `tools/matten-report`, a workspace-excluded `publish = false` local Markdown/plain-text report tool for `matten-data` readiness only, with explicit `--demo data-readiness` and `--input <csv> --kind data-readiness --select <cols>` modes plus optional `--output`. Added deterministic fixtures and exact-output tests for success, missing values, non-numeric values, and CLI policy; wired manifest-path check/test/Clippy/smoke commands into CI and release checklist. Accepted dependency policy delta: path-only local deps for this unpublished excluded tool, with API drift caught by local gates and no prerelease version-sync chore. No public API, published crate, workspace membership, dependency leak, JSON/SVG/HTML/Vega-Lite, plotting, notebook, GUI, telemetry, network, runtime, MSRV, or maturity-label change. Future report families remain deferred. Current-family docs retargeted to exact prerelease pins (`0.29.0-pre.4`). |
| 1.52.0 | 2026-07-04 | v0.29.0-pre.5: RFC-063 Phase 3 shape-flow local-tool prerelease. Extended `tools/matten-report` with the accepted second report family: `--demo shape-flow`, a fixed deterministic Markdown/plain-text report for broadcasting, reshape, `mean_axis(0)`, `mean_axis(1)`, and matmul shape flow. Shape-flow remains demo-only: no `--input` mode, no arbitrary expression parser, no source scanning, no automatic Tensor operation tracing, no lazy graph, no public API, no published crate, no workspace membership, no new dependency, no SVG/HTML/Vega-Lite/JSON/images/ANSI/notebook/GUI scope. Added exact-output tests and parser-policy coverage, kept data-readiness exact-output tests, documented the fixed-demo boundary in the tool README, and wired shape-flow smoke commands into CI + release checklist. Current-family docs retargeted to exact prerelease pins (`0.29.0-pre.5`). |
| 1.53.0 | 2026-07-12 | Docs-governance Handoff 01 implementation prepared for review. Added `docs/design/coverage-gap-resolution.md` to resolve the three pre-archival coverage gaps: retired unmaintained numeric compile/rebuild/memory targets as live requirements, inventoried current NumPy golden coverage while keeping property/fuzz work as future hardening, and recorded that `Tensor` has `Debug` but no `Display` contract. Aligned contributing docs, RFC-013, and compatibility docs. No public API, dependency, version, release-scope, runtime, benchmark, or test-gate change. |
| 1.54.0 | 2026-07-12 | Docs-governance Handoff 02 implementation prepared for review. Archived the v0.19.0 requirements, external-design, and roadmap snapshots under `docs/design/history/` with historical-only banners; added `docs/design/README.md` with the four-plane ownership rule and the README-note-over-RFC-066 disposition; linked the rule from `rfcs/README.md`; kept `docs/design/**` outside the mdBook. No public API, dependency, version, release-scope, runtime, or user-doc contract change. |
| 1.55.0 | 2026-07-13 | Docs-governance Handoff 03 implementation prepared for review. Expanded `docs/src/philosophy.md` from a stub into an evergreen principles page distilled from tracked `docs/design/history/` snapshots: developer-experience-first tensor work, family-car positioning, one concrete `Tensor`, no visible lifetime burden, concrete-before-generic dynamic ingestion, panic-local/Result-boundary split, explicit non-goals, and a short migration pointer. No public API, dependency, version, release-scope, runtime, benchmark guarantee, or mdBook structure change. |
| 1.56.0 | 2026-07-13 | RFC-054 lifecycle closure status alignment. Closed `matten-migrate` as implemented for the reviewed local advisory tool scope, moved RFC-054 to `rfcs/done/`, and recorded that rewrite/apply, source mutation, Cargo.toml editing, public `matten-migrate` packaging, and stronger migration automation are extracted to future RFC/release-policy ownership. The active RFC index now has no proposed RFCs. Roadmap-only alignment; no public API, dependency, version, release-scope, runtime, or tool behavior change. |
| 1.57.0 | 2026-07-13 | Proposed RFC-066: v1.0 readiness audit and release decision gate. Opens an audit-only RFC to review public API snapshot evidence, panic/Result boundary stability, serde/canonical format stability, documented limitations/non-goals, companion maturity under lock-step family versioning, and release-gate evidence before any v1.0 decision. This does not authorize a v1.0 release, version bump, tag, publish, API change, dependency change, or companion promotion. |
| 1.58.0 | 2026-07-13 | Proposed RFC-067: v1.0 family maturity policy. Drafts the RFC-066 MD-1 policy answer: production-ready-candidate companions are not automatic v1.0 blockers if a future v1.0 release RFC explicitly lists each crate's maturity label, confirms API stability and documented caveats, and avoids silent companion promotion. This is policy planning only; no v1.0 release preparation, version bump, tag, publish, API change, dependency change, or companion promotion is authorized. |
| 1.59.0 | 2026-07-13 | Implemented RFC-067 as repository policy and recorded the RFC-066 MD-1 resolution in the v1 readiness audit, compatibility policy, and release checklist. Candidate-labeled companions are not automatic v1.0 blockers, but any future v1.0 release RFC must include the RFC-067 family maturity table and decide each candidate-labeled crate explicitly. RFC-067 moved to `rfcs/done/`. No v1.0 release preparation, version bump, tag, publish, API change, dependency change, or companion promotion is authorized. |
| 1.60.0 | 2026-07-13 | Prepared v0.31.0 as an RFC-066/RFC-067 cleanup release. Closed RFC-066 as implemented for the reviewed audit-only scope, kept RFC-067 implemented as repository policy, retargeted current-family documentation to 0.31.0, and added release notes for the v1.0 readiness audit / family maturity policy cleanup. No v1.0 release preparation, tag, publish, public API change, dependency change, runtime behavior change, MSRV change, feature-flag change, maturity-label change, or companion promotion is authorized. |
| 1.61.0 | 2026-07-13 | Proposed RFC-068: rich local visualization artifacts. Opens the next visualization phase after RFC-063/RFC-065 with a conservative first slice: static self-contained HTML output for the existing local `tools/matten-report --demo educational-path` report, with Markdown/plain text remaining the default. This is planning/handoff work only; no implementation, public API, public report/viz crate, SVG/Vega-Lite/JSON output, expression tracing, autograd, published-crate dependency change, version bump, tag, publish, or companion maturity change is authorized. |
| 1.62.0 | 2026-07-14 | RFC-068 Phase 1 implementation prepared for review. Added `tools/matten-report --demo educational-path --format html --output <path>` as a static self-contained local HTML artifact, kept Markdown/plain text as the default, required explicit `--output` for HTML, and rejected HTML for all other report families/input mode. Added std-only escaping/rendering, parser and HTML-safety tests, README documentation, and CI/release-checklist smoke commands. No public API, public report/viz crate, new dependency, published-crate graph change, SVG/Vega-Lite/JSON/notebook/browser scope, expression tracing, autograd, version bump, tag, publish, or companion maturity change is authorized. |
| 1.63.0 | 2026-07-15 | RFC-068 shared educational report data handoff drafted. Audit found that the educational-path Markdown and HTML renderers duplicate the same fixed tensor computations and derived values; the next proposed slice is a behavior-neutral private data-model extraction inside `tools/matten-report` before expanding HTML to another report family. No CLI behavior, output format, public API, public report/viz crate, dependency, published-crate graph, SVG/Vega-Lite/JSON/notebook/browser scope, expression tracing, autograd, version bump, tag, publish, or companion maturity change is authorized. |
| 1.64.0 | 2026-07-15 | RFC-068 shared educational report data implementation prepared for review. Extracted the educational-path fixed tensor computations and derived values into one private data builder consumed by both Markdown and HTML renderers, preserving byte-identical output and adding an exact HTML snapshot test alongside the existing HTML safety test. No CLI behavior, output format, public API, public report/viz crate, dependency, published-crate graph, SVG/Vega-Lite/JSON/notebook/browser scope, expression tracing, autograd, version bump, tag, publish, or companion maturity change is authorized. |
| 1.65.0 | 2026-07-15 | RFC-068 shape-flow HTML handoff drafted. The next proposed feature slice extends local static HTML output to exactly one additional fixed report family, `tools/matten-report --demo shape-flow`, with explicit `--output`, exact HTML snapshot coverage, CI/release-checklist smoke commands, and the same static/no-JS/no-network/no-external-asset boundary as educational-path HTML. Markdown/plain text remains default. No HTML for other report families or input mode, public API, public report/viz crate, dependency, published-crate graph, SVG/Vega-Lite/JSON/notebook/browser scope, expression tracing, autograd, version bump, tag, publish, or companion maturity change is authorized. |
| 1.66.0 | 2026-07-15 | RFC-068 shape-flow HTML implementation prepared for review. Added `tools/matten-report --demo shape-flow --format html --output <path>` as the second static self-contained local HTML artifact, generalized HTML policy/error text to accept educational-path and shape-flow only, kept Markdown/plain text as default, and kept HTML rejected for data-readiness, dynamic-readiness, mlprep-standardization, and input mode. Added exact shape-flow HTML snapshot and safety tests, README documentation, and CI/release-checklist smoke commands. No public API, public report/viz crate, dependency, published-crate graph, SVG/Vega-Lite/JSON/notebook/browser scope, expression tracing, autograd, version bump, tag, publish, or companion maturity change is authorized. |
| 1.67.0 | 2026-07-15 | Prepared 0.32.0 as an RFC-068 rich local visualization-artifact release. Closed RFC-068 as implemented for local static HTML artifacts covering `educational-path` and `shape-flow`, retargeted current-family documentation to 0.32.0, and added release notes for the local-tool visualization scope. Markdown/plain text remains default, HTML requires explicit `--output`, and no public report/viz crate, public API, published dependency, runtime behavior, MSRV, feature-flag, maturity-label, companion-promotion, tag, publish, SVG, Vega-Lite, JSON report, notebook, GUI, expression tracing, or autograd scope is authorized. |
| 1.68.0 | 2026-07-15 | Drafted the post-0.32 RFC-068 visualization continuation audit. The 0.32.0 release scope remains local static HTML artifacts for `tools/matten-report --demo educational-path` and `tools/matten-report --demo shape-flow`; the follow-up audit recommends handoff review for `tools/matten-report --demo dynamic-readiness` HTML before any implementation. No direct implementation, public report/viz crate, core visualization API, expression tracing, autograd, dependency change in published crates, SVG/Vega-Lite/JSON/notebook/browser scope, tag, publish, or companion maturity change is authorized. |
| 1.69.0 | 2026-07-15 | Drafted the RFC-068 dynamic-readiness local HTML artifact handoff. The proposed next slice is local static HTML for `tools/matten-report --demo dynamic-readiness`, keeping Markdown/plain text as default and requiring explicit `--output`; `data-readiness`, `mlprep-standardization`, input-mode HTML, public report/viz crates, core visualization APIs, expression tracing, autograd, dependency changes in published crates, SVG/Vega-Lite/JSON/notebook/browser scope, version bump, tag, publish, and companion maturity changes remain out of scope. |
| 1.70.0 | 2026-07-15 | RFC-068 dynamic-readiness local HTML implementation prepared for review. Added `tools/matten-report --demo dynamic-readiness --format html --output <path>` as the third static self-contained local HTML artifact, shared the fixed dynamic-readiness report data between Markdown and HTML renderers while preserving Markdown output, generalized HTML policy/error text to accept educational-path, shape-flow, and dynamic-readiness only, and kept HTML rejected for data-readiness, mlprep-standardization, and input mode. Added exact dynamic-readiness HTML snapshot and safety tests, README documentation, and CI/release-checklist smoke commands. No public API, public report/viz crate, dependency, published-crate graph, SVG/Vega-Lite/JSON/notebook/browser scope, expression tracing, autograd, version bump, tag, publish, or companion maturity change is authorized. |
| 1.71.0 | 2026-07-15 | Prepared 0.33.0 as an RFC-068 visualization-continuation release. Stopped feature work for the current release after the reviewed dynamic-readiness local HTML artifact, retargeted current-family documentation to 0.33.0, and added release notes for the local-tool visualization continuation scope. Markdown/plain text remains default, HTML requires explicit `--output`, and no public report/viz crate, public API, published dependency, runtime behavior, MSRV, feature-flag, maturity-label, companion-promotion, tag, publish, SVG, Vega-Lite, JSON report, notebook, GUI, expression tracing, or autograd scope is authorized. |
| 1.72.0 | 2026-07-15 | Drafted the post-0.33 RFC-068 visualization continuation audit and mlprep-standardization local HTML artifact handoff. The proposed next reviewed slice is local static HTML for `tools/matten-report --demo mlprep-standardization`, keeping Markdown/plain text as default and requiring explicit `--output`; `data-readiness`, input-mode HTML, public report/viz crates, core visualization APIs, expression tracing, autograd, dependency changes in published crates, SVG/Vega-Lite/JSON/notebook/browser scope, version bump, tag, publish, and companion maturity changes remain out of scope. |
| 1.73.0 | 2026-07-15 | RFC-068 mlprep-standardization local HTML implementation prepared for review. Added `tools/matten-report --demo mlprep-standardization --format html --output <path>` as the fourth static self-contained local HTML artifact, shared the fixed mlprep-standardization report data between Markdown and HTML renderers while preserving Markdown output, generalized HTML policy/error text to accept educational-path, shape-flow, dynamic-readiness, and mlprep-standardization only, and kept HTML rejected for data-readiness and input mode. Added exact mlprep-standardization HTML snapshot and safety tests, README documentation, and CI/release-checklist smoke commands. No public API, public report/viz crate, dependency, published-crate graph, SVG/Vega-Lite/JSON/notebook/browser scope, expression tracing, autograd, version bump, tag, publish, or companion maturity change is authorized. |
| 1.74.0 | 2026-07-15 | Recorded 0.34.0 as an RFC-068 visualization-continuation release. Stopped feature work for the current release after the reviewed mlprep-standardization local HTML artifact, retargeted current-family documentation to 0.34.0, and added release notes for the local-tool visualization continuation scope. Markdown/plain text remains default, HTML requires explicit `--output`, and no public report/viz crate, public API, published dependency, runtime behavior, MSRV, feature-flag, maturity-label, companion-promotion, SVG, Vega-Lite, JSON report, notebook, GUI, expression tracing, autograd, data-readiness HTML, or input-mode HTML scope is authorized. |
