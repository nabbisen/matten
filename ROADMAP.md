# `matten` ROADMAP

**Project:** `matten`  
**Document Kind:** Canonical Project Roadmap  
**Document Version:** `1.27.0`  
**Date:** 2026-06-23  
**Status:** Canonical roadmap updated for v0.20+ materialization planning, the examples program, and the benchmarking & positioning program. RFC-032 is consumed by the companion dependency/import convention; v0.20+ design starts at RFC-033; the examples program is RFC-043 through RFC-048; benchmarking & positioning is RFC-049 (Track D).  
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

### `matten-mlprep` beta decision gate

```text
[ ] API is small and teachable
[ ] functions are deterministic
[ ] shape rules are documented
[ ] zero-variance behavior is explicit
[ ] train/test split behavior is explicit
[ ] no ML-framework scope entered
```

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
