# `matten` Project Roadmap and Milestones

> **HISTORICAL SNAPSHOT — DO NOT CITE AS CURRENT.**
> As-built through v0.19.0 (RFCs 000–030). Superseded by the current RFC corpus
> (`rfcs/`) and user documentation (`docs/src/`); forward schedule lives in
> `ROADMAP.md`. This document froze ~35 RFCs ago and predates matten-data, the
> 0.20–0.30 API/companion work, and the visual/educational/migration programs.
> Retained for design traceability only. Section-by-section canonical owners:
> see `docs/design/README.md`. Terminology note: the "Phase 1/Phase 2/Sedan/SUV"
> vocabulary here is retired and is banned from user docs.

**Project:** `matten`  
**Document Kind:** Project Schedule / Roadmap / Milestone Plan  
**Document Version:** `0.5.0`  
**Prepared For:** `matten` maintainers and implementation developers  
**Date:** 2026-06-21  
**Planning Baseline:** Original M0–M11 schedule plus as-built reconciliation through the `0.19.0` family release; updated with the v0.19.1 hardening gate and v0.20+ forward milestones  
**Status:** Historical milestone record + current detailed milestone plan. **`ROADMAP.md` remains the concise canonical forward schedule**, while this document records detailed milestone history, acceptance gates, and the current M12+ execution plan.  
**Revision note:** `0.5.0` keeps the M0–M11 historical record, but adds the active M12+ milestone track: v0.19.1 companion maturity hardening, v0.20+ `matten-data` beta-decision work, selective production-readiness gates, and deferred nalgebra/candle/streaming criteria. It also updates the quality gates, feature policy, risk register, and immediate next steps to match the as-built v0.19.0 family and the v0.19.0 source/doc review findings.

---

## 0. Reading Guide

This document translates the `matten` requirements into a staged implementation schedule. It is not a detailed internal design document. It defines release milestones, scope boundaries, sequencing, quality gates, and RFC themes so that the development team can start work without losing the project philosophy.

Requirement language follows this convention:

- **MUST**: required before the milestone can be considered complete.
- **SHOULD**: strongly recommended for the milestone unless a later RFC explicitly changes it.
- **MAY**: allowed but not required.
- **MUST NOT**: explicitly prohibited.

The dates below assume project kickoff on **2026-06-18**. They are planning dates, not release promises. Public release, especially any `1.0.0` release, requires explicit maintainer confirmation.

---

## 0.1 Review Incorporation Summary

This revision incorporates the external review of `matten-roadmap-milestones-v0.1.md` selectively. The review was treated as advisory, not authoritative. The following changes were accepted into the schedule:

| Review Point | Decision | Roadmap Change |
|---|---|---|
| String slicing parser may create parser complexity and runtime-only failures | **Partially accepted** | The Rust-native slice builder is now the primary M4 deliverable. `slice_str` remains valuable for NumPy familiarity, but it is bounded, secondary, and deferrable if it threatens the milestone. A public slice macro is not adopted in this roadmap. |
| Dynamic `Element::Text(String)` may bloat every element | **Accepted** | Phase 2 now requires an explicit memory-layout RFC and measurement gate before implementation. `Text(String)` is treated as a semantic requirement, not a permanently fixed storage representation. |
| M8 RFC gate may cause analysis paralysis | **Partially accepted** | M8 remains a design gate for production implementation, but a disposable dynamic-engine spike is now allowed in parallel to validate `Element`, CoW, and ergonomics before final RFC approval. |

> **Historical note.** This statement captured the original pre-implementation strategy. As-built through v0.19.0, the default `f64` Sedan shipped, but `dynamic` was narrowed by RFC-016 into an ingestion / cleanup / explicit numeric-on-ramp feature rather than a full SUV computation engine with CoW views and dynamic arithmetic.

---

## 0.2 Examples Strategy Incorporation Summary

This revision adds a scoped examples strategy before RFC-014 is written. The examples strategy was intentionally narrowed to preserve the `matten` concept: examples are executable documentation for already-designed public APIs, not a back door for adding advanced linear algebra, dataframe, machine-learning, or integration-framework scope.

| Example Scope Area | Decision | Roadmap Change |
|---|---|---|
| Core tensor examples | **Accepted for `0.1.0`** | M6 now has a required executable-example gate covering creation, shape inspection, reshape, element-wise math, broadcasting, slicing, JSON/CSV, and boundary errors. |
| Basic mathematical computing examples | **Accepted for `0.1.0` if backed by RFC-010** | M6 includes dot product, matrix-vector, matrix multiplication, reductions, normalization, and cosine similarity only when the corresponding API is already accepted. |
| Practical PoC patterns | **Deferred to `0.1.x`** | M7 may add standardization, min-max scaling, row-wise scoring, moving average, rolling windows, pairwise distance, and Gram matrix examples when implemented as small user-side workflows. |
| Dynamic examples | **Deferred to `0.2.0`** | Dynamic examples are tied to RFC-011/RFC-012/RFC-013 and must not be required for `0.1.0`. |
| Business scenario examples | **Future theme** | Business examples may be added later as optional tutorials, but are not a release gate for Phase 1. |
| `ndarray` / `nalgebra` / `candle` / `axum` bridge examples | **Future or feature-gated theme** | Bridge examples must not add default dependencies or imply that `matten` is an integration framework. |
| Advanced linear algebra, dataframe, ML training, GPU, sparse, or benchmark examples | **Out of current scope** | These are explicitly listed as non-goals unless a future strategic RFC changes the project identity. |

RFC-014 is now reserved for **Example Suite and Executable Documentation**. Missing-value behavior moves to a future dynamic RFC if it is not already covered by RFC-011/RFC-013.

---

## 0.3 v0.5 Current Update: v0.19.0 Reach and M12+ Forward Plan

This revision updates the roadmap after the project reached the `0.19.0` family release:

- core `matten` remains the stable pre-1.0 Sedan crate;
- `matten-ndarray` exists and is intended to be a **production-ready candidate** after the v0.19.1 hardening corrections;
- `matten-mlprep` exists and is intended to be **beta** after the v0.19.1 hardening corrections;
- lock-step family versioning is now the active version policy (RFC-030);
- maturity is expressed by crate status labels, not by separate crate version numbers;
- the next immediate milestone is **M12 / v0.19.1 companion maturity hardening**, not new feature expansion;
- `matten-data` remains a v0.20+ beta-decision topic and must not be rushed into dataframe scope;
- `nalgebra`, `candle`, and streaming remain deferred and require separate RFCs before implementation.

This document now has two layers:

1. **M0–M11:** historical Phase 1 / narrowed Phase 2 milestone record.
2. **M12+:** active detailed execution plan beyond v0.19.0.

---

## 1. Project Vision Recap

`matten` is a Rust multidimensional array library for rapid prototyping, data exploration, and business Proof of Concept workflows. It is intentionally a **family car** rather than a Formula 1 engine: easy to enter, forgiving, predictable, and friendly for users who want to write mathematical Rust quickly.

The roadmap preserves three architectural principles:

1. **DX over benchmarks**: prioritize simple APIs, readable failures, and fast iteration over peak throughput.
2. **Zero type puzzles**: avoid exposing complex generics, lifetimes, view types, or trait-bound stacks to ordinary users.
3. **Boundary safety**: internal math operations may panic with rich context, but I/O and parsing APIs MUST return `Result<Tensor, MattenError>`.

---

## 2. Roadmap Strategy

### 2.1 Release Train Overview

| Track | Target | Engine | Main Purpose | Release Type |
|---|---:|---|---|---|
| Foundation | `0.0.x` | Internal scaffolding | Establish crate structure, tests, docs, CI, and API contract discipline | Private / pre-release |
| Phase 1 Alpha | `0.1.0-alpha.*` | Sedan, `f64` | Validate core `Tensor`, shape checks, creation APIs, and basic math | Internal or limited users |
| Phase 1 Beta | `0.1.0-beta.*` | Sedan, `f64` | Complete reshape, transpose, slicing, broadcasting, serde, conversions | Limited users |
| Phase 1 Stable | `0.1.0` | Sedan, `f64` | First usable PoC-focused numerical library | Public candidate |
| Phase 1 Stabilization | `0.1.x` | Sedan, `f64` | Polish docs, error messages, edge cases, compatibility | Patch/minor releases |
| Phase 2 Alpha | `0.2.0-alpha.*` | SUV, `dynamic` | Introduce `Element` and CoW storage behind feature flag | Experimental |
| Phase 2 Beta | `0.2.0-beta.*` | SUV, `dynamic` | CSV/JSON messy data workflows, missing-value utilities | Limited users |
| Phase 2 Stable | `0.2.0` | SUV, `dynamic` | Usable heterogeneous data workflows | Public candidate |

> **As-built correction.** The table above is the historical kickoff release-train model. The shipped Phase 2 was narrowed to an ingestion/on-ramp engine, not a full SUV compute engine. The active post-v0.19 train is:

| Track | Target | Scope | Release Type |
|---|---:|---|---|
| Companion hardening | `0.19.1` | Fix companion dynamic rejection, maturity-status docs, RFC lifecycle, and release-doc checks | Patch / quality release |
| `matten-data` decision | `0.20+` | Decide whether the small table-to-Tensor workflow deserves beta | Decision gate |
| Selective companion maturity | `0.21+` | Promote only crates with proven scope, tests, docs, and no core pollution | Maturity releases |
| Deferred bridges | later | `matten-nalgebra`, `matten-candle` only after separate RFCs | Future |
| Streaming / large CSV | later | Design spike only until batch/error/schema/memory policy is proven | Future / high-risk |

### 2.2 Scope Rule

The roadmap is deliberately split into **small, shippable milestones**. Each milestone MUST preserve a working crate, passing tests, and a coherent public API.

The team SHOULD avoid implementing Phase 2 internals before Phase 1 is stable enough to validate the public `Tensor` API. Phase 2 may be designed in parallel, but it MUST NOT destabilize the default `f64` path.

---

## 3. Milestone Summary

> **As-built status.** The original M0–M11 schedule below is retained as the Phase 1/
> Phase 2 plan of record. The **Status** column records what actually shipped. The
> Phase 2 milestones (M8–M11) shipped in a **narrowed** form — an ingestion/on-ramp
> engine, not the CoW/dynamic-arithmetic engine originally planned (RFC-016). After
> M11 the project entered the workspace/companion-crate arc (v0.16–v0.19), summarized
> in §3.1 and detailed in `ROADMAP.md`.

| Milestone | Name | Output | Release Target | Status |
|---|---|---|---|---|
| M0 | Kickoff and Crate Skeleton | Repository, CI, quality baseline | `0.0.1` | shipped |
| M1 | Core Tensor Contract | `Tensor`, shape validation, accessors | `0.1.0-alpha.1` | shipped |
| M2 | Creation and Conversion APIs | `new`, `zeros`, `ones`, `full`, `From`/`Into` | `0.1.0-alpha.2` | shipped |
| M3 | Element-wise Math and Broadcasting | operators, scalar ops, broadcasting | `0.1.0-alpha.3` | shipped |
| M4 | Shape Operations and Slicing | reshape/transpose/swap-axes, slice builder, `slice_str` | `0.1.0-beta.1` | shipped |
| M5 | Boundary Integration | serde, JSON/CSV `Result` APIs, `MattenError` | `0.1.0-beta.2` | shipped |
| M6 | Phase 1 Release Hardening + Required Examples | docs, executable examples, QA | `0.1.0` | shipped |
| M7 | Phase 1 Feedback / Stabilization | patch fixes, docs, optional pattern examples | `0.1.x`–`0.16.0` | shipped (stabilized through `0.16.0`) |
| M8 | Dynamic Engine Design Lock and Spike | RFCs + spike for `Element`, storage, serde | design gate | shipped — **narrowed** to ingestion/on-ramp (RFC-016) |
| M9 | Dynamic Engine Alpha | `dynamic` feature, `Element`, storage | `0.2.0-alpha.1` | shipped as ingestion engine; **no CoW/dynamic arithmetic** |
| M10 | Messy Data Workflows | CSV/JSON mixed data, missing-value helpers | `0.2.0-beta.1` | shipped (`fill_none`, forward-fill, masks, `try_numeric` on-ramp) |
| M11 | Dynamic Release Hardening | memory checks, dynamic examples, docs | `0.2.0` | shipped (numeric on-ramp model; coercion via `NumericPolicy`, RFC-017) |

### 3.1 Post-Phase-2 arc: workspace & companion crates (v0.16–v0.19)

After the core/dynamic work, the project added a workspace and companion crates.
**`ROADMAP.md` is canonical** for this and all forward planning; the summary:

```text
v0.16.0  Companion boundary confirmation — RFC-022 policy + dependency-boundary CI
v0.17.0  Cargo workspace introduced; matten-ndarray 0.1.0 experimental (RFC-027)
v0.18.0  matten-mlprep 0.1.0 experimental (RFC-028)
v0.19.0  matten-ndarray -> production-ready candidate; matten-mlprep -> beta (RFC-029)
         Lock-step family versioning; family aligned to 0.19.0 (RFC-030)
v0.20+   matten-data beta-decision phase (RFC-023)
later    nalgebra / candle / streaming, each behind its own RFC (RFC-025 §10, RFC-026)
```

The core stays small and depends on none of `ndarray`/`nalgebra`/`candle`/`polars`/
`matten-*` (enforced by `scripts/check-core-dependency-boundary.sh`). Versioning is
lock-step (RFC-030): one family version, maturity shown by per-crate Status labels.

### 3.2 Active M12+ milestone summary

| Milestone | Name | Target | Output | Status |
|---|---|---:|---|---|
| M12 | Companion Maturity Hardening | `0.19.1` | robust dynamic rejection, maturity-status docs, RFC lifecycle cleanup, release-doc checks | current next |
| M13 | `matten-data` Beta-Decision Preparation | `0.20+` | decision package for whether to promote, keep experimental, or freeze/defer `matten-data` | planned |
| M14 | `matten-data` Minimal Workflow, if approved | `0.20.x`/`0.21.x` | tiny CSV/table-to-Tensor workflow only; no dataframe scope | conditional |
| M15 | Selective Production-Readiness Phase | `0.21+` | promote only proven companion crates; preserve core simplicity | planned |
| M16 | Deferred Bridge Crates | later | `matten-nalgebra` / `matten-candle` only by separate RFC | deferred |
| M17 | Streaming / Large CSV Exploration | later | batch/error/schema/memory design spike; implementation only after policy is proven | deferred |

---

## 4. Detailed Milestones

## M0: Kickoff and Crate Skeleton

**Target Window:** 2026-06-18 to 2026-06-24  
**Target Release:** `0.0.1` internal baseline  
**Theme:** Make the repository boring, testable, and ready for API design.

### Scope

M0 establishes the project structure before feature development begins.

M0 MUST include:

- `Cargo.toml` with crate metadata, license, edition, feature placeholders, and dependency policy.
- Initial `src/lib.rs` with public module boundary decisions.
- CI for `cargo fmt`, `cargo clippy`, `cargo test`, and doc tests.
- Minimal README with project philosophy and non-goals.
- Initial changelog.
- Initial `MattenError` placeholder, even if most internal operations still panic.
- A documented policy that the public user-facing API remains centered around `matten::Tensor`.

M0 SHOULD include:

- `deny(unsafe_code)` unless a later RFC explicitly approves an exception.
- `rust-version` in `Cargo.toml`.
- `examples/hello_tensor.rs` as a smoke example.

### Exit Criteria

- Fresh checkout builds with default features.
- CI passes on Linux.
- No public API exposes storage internals.
- README explains that `matten` prioritizes PoC DX over benchmark leadership.

### Non-goals

- No math operations beyond smoke compilation.
- No dynamic feature implementation.
- No performance optimization work.

---

## M1: Core Tensor Contract

**Target Window:** 2026-06-25 to 2026-07-08  
**Target Release:** `0.1.0-alpha.1`  
**Theme:** Define the concrete Phase 1 `Tensor` and shape model.

### Scope

M1 creates the default numerical engine: a concrete, owned, `f64`-backed tensor.

M1 MUST include:

- Public `Tensor` type.
- Internal storage equivalent to owned `Vec<f64>` plus owned `Vec<usize>` shape.
- Shape validation for data length versus shape product.
- Scalar support via zero-dimensional shape `[]`.
- Public accessors:
  - `shape(&self) -> &[usize]`
  - `ndim(&self) -> usize`
  - `len(&self) -> usize`
  - `is_empty(&self) -> bool`
  - `as_slice(&self) -> &[f64]`
  - `to_vec(&self) -> Vec<f64>`
- Debug formatting that is useful during PoC debugging.
- Panic messages that include actual shape, expected shape, and element counts.

M1 SHOULD include:

- `PartialEq` for testability.
- `Clone` for DX and internal copying.
- Internal helper functions for shape product, axis normalization, and index flattening.

### Exit Criteria

- Users can create and inspect tensors without generic parameters or lifetime annotations.
- Invalid shape construction panics with actionable context.
- Beginner examples compile with only `use matten::Tensor;`.

### Non-goals

- No slicing.
- No broadcasting.
- No serde implementation beyond reserving feature layout.

---

## M2: Creation and Conversion APIs

**Target Window:** 2026-07-09 to 2026-07-22  
**Target Release:** `0.1.0-alpha.2`  
**Theme:** Make tensor creation trivial from ordinary Rust values.

### Scope

M2 expands factory methods and standard conversions.

M2 MUST include:

- Factory methods:
  - `Tensor::new(data: Vec<f64>, shape: &[usize]) -> Tensor`
  - `Tensor::zeros(shape: &[usize]) -> Tensor`
  - `Tensor::ones(shape: &[usize]) -> Tensor`
  - `Tensor::full(shape: &[usize], value: f64) -> Tensor`
  - `Tensor::from_vec(data: Vec<f64>) -> Tensor`
- Standard conversions:
  - `impl From<Vec<f64>> for Tensor`
  - `impl From<Tensor> for Vec<f64>`
  - `impl From<Vec<Vec<f64>>> for Tensor`
  - `impl TryFrom<Tensor> for Vec<Vec<f64>>`
- Validation for ragged nested vectors.
- Documentation examples for each constructor.

M2 SHOULD include:

- `Tensor::scalar(value: f64) -> Tensor`.
- `Tensor::eye(n: usize) -> Tensor` only if it does not expand the milestone too much.
- A short NumPy-to-`matten` creation comparison in docs.

### Exit Criteria

- Users can create 0D, 1D, and 2D tensors easily.
- Ragged nested vectors are rejected with a readable error or panic according to API boundary policy.
- All creation APIs have doc tests.

### Non-goals

- No random number API unless a later RFC defines dependency policy.
- No generic dtype support.
- No matrix-specific type separate from `Tensor`.

---

## M3: Element-wise Math and Broadcasting

**Target Window:** 2026-07-23 to 2026-08-12  
**Target Release:** `0.1.0-alpha.3`  
**Theme:** Make common mathematical operations feel natural.

### Scope

M3 implements standard arithmetic operators and NumPy-like broadcasting.

M3 MUST include:

- Element-wise operators for borrowed tensors:
  - `impl Add for &Tensor`
  - `impl Sub for &Tensor`
  - `impl Mul for &Tensor`
  - `impl Div for &Tensor`
  - `impl Neg for &Tensor`
- Scalar arithmetic for common forms:
  - `&Tensor + f64`
  - `&Tensor - f64`
  - `&Tensor * f64`
  - `&Tensor / f64`
- Runtime broadcasting rules:
  - right-aligned shapes;
  - dimensions match when equal or when one side is `1`;
  - scalar shape `[]` broadcasts to any shape.
- Panic-on-mismatch messages that show left shape, right shape, attempted operation, and incompatible dimension.
- Unit tests for scalar, 1D, 2D, and at least one 3D broadcasting case.

M3 SHOULD include:

- `sum()`, `mean()`, `min()`, `max()` as whole-tensor reductions if implementation is small.
- Internal broadcasting iterator abstraction that does not become public API.
- Floating-point comparison helper only for tests.

### Exit Criteria

- `&a + &b` works for same-shape tensors and broadcast-compatible tensors.
- Users do not need to call explicit broadcasting APIs in common cases.
- Shape mismatch panics are easier to understand than compiler trait errors.

### Non-goals

- No advanced linear algebra.
- No BLAS integration.
- No GPU integration.
- No optimized strided view system in Phase 1.

---

## M4: Shape Operations and Slicing

**Target Window:** 2026-08-13 to 2026-09-02  
**Target Release:** `0.1.0-beta.1`  
**Theme:** Provide the minimum array-shaping language users expect from NumPy-like workflows.

### Scope

M4 implements reshaping, axis operations, and slicing while preserving the no-lifetime API.

M4 MUST include:

- Shape manipulation:
  - `reshape(&self, new_shape: &[usize]) -> Tensor`
  - `transpose(&self) -> Tensor`
  - `swap_axes(&self, axis1: usize, axis2: usize) -> Tensor`
  - `flatten(&self) -> Tensor`
- Phase 1 semantics: operations MAY deep-copy internally to ensure the result is owned and contiguous.
- Slice builder as the primary slicing API:
  - `tensor.slice().range(..).range(0..2).build()`
  - support for all, range, range-from, range-to, and single-index dimensions.
- Tests proving that slice outputs are independent owned tensors in Phase 1.
- Detailed `MattenError` variants for invalid slice specifications that originate from user-provided parsing-style inputs.

M4 SHOULD include:

- Bounded string slicing as a secondary convenience API:
  - `slice_str("0:2, :") -> Result<Tensor, MattenError>`
  - minimum grammar: `:`, `start:end`, `start:`, `:end`, and single integer.
- Better pretty-printing for 1D and 2D tensors.

M4 MAY defer:

- `slice_str` itself if parser complexity threatens the milestone. If deferred, the release notes MUST clearly state that the builder API is the supported slicing interface for that release.
- Step slicing such as `::2`; this is not required for `0.1.0-beta.1`.

M4 MUST NOT include:

- A public slicing macro unless a later RFC explicitly approves it. A small macro may look concise, but it would add a second public syntax surface before the builder semantics are proven.

### Exit Criteria

- Users can reshape and slice through the builder API without borrowing or view-lifetime management.
- If `slice_str` is included, invalid string slice syntax returns `Result::Err`, not panic.
- Invalid internal shape operations panic with helpful context when they are not I/O-style boundaries.
- The builder API remains documented as the canonical slicing API even when `slice_str` exists.

### Non-goals

- No lazy views in default Phase 1.
- No mutation-through-view API.
- No advanced indexing masks.

---

## M5: Boundary Integration

**Target Window:** 2026-09-03 to 2026-09-23  
**Target Release:** `0.1.0-beta.2`  
**Theme:** Make `matten` usable in real Rust application boundaries.

### Scope

M5 implements serde, JSON/CSV ingestion, and a consistent boundary error model.

M5 MUST include:

- `serde` feature enabled by default unless dependency policy changes.
- `Serialize` / `Deserialize` for Phase 1 `Tensor`.
- `MattenError` with clear display messages.
- Boundary APIs:
  - `Tensor::from_json(json: &str) -> Result<Tensor, MattenError>`
  - `Tensor::to_json(&self) -> Result<String, MattenError>`
  - `Tensor::from_csv(csv: &str) -> Result<Tensor, MattenError>`
  - `Tensor::load_from_file(path: impl AsRef<Path>) -> Result<Tensor, MattenError>` if file I/O is accepted for this milestone.
- JSON behavior for nested numeric arrays.
- CSV behavior for numeric rectangular data.
- Tests for malformed JSON, ragged JSON, non-numeric JSON, malformed CSV, and ragged CSV.

M5 SHOULD include:

- Optional `csv` dependency behind a `csv` feature if dependency size is a concern.
- Example with a web handler returning or receiving `Tensor` JSON.
- Error messages with line/column information for CSV when available.

### Exit Criteria

- I/O APIs never panic on malformed user input.
- Internal math operations continue to panic with rich context for fast PoC feedback.
- JSON round-trip is documented and tested.

### Non-goals

- No mixed-type JSON ingestion in Phase 1.
- No database integration crate.
- No streaming large CSV reader yet.

---

## M6: Phase 1 Release Hardening and Required Examples

**Target Window:** 2026-09-24 to 2026-10-14  
**Target Release:** `0.1.0` candidate  
**Theme:** Make the default numerical library usable, documented, and stable enough for early adopters.

### Scope

M6 is the release-readiness milestone for the Phase 1 Sedan engine.

M6 MUST include:

- Complete tutorial for a numerical PoC workflow.
- API reference docs with runnable examples.
- README with:
  - quick start;
  - family-car philosophy;
  - comparison to `ndarray`, `nalgebra`, `candle`, and NumPy at a conceptual level;
  - known limitations;
  - migration guidance.
- Compatibility test matrix for stable Rust.
- Panic message review.
- Public API review using `cargo public-api` or equivalent process.
- Minimum benchmark smoke tests for allocation-heavy operations, not to optimize but to document expectations.
- A required executable example suite that demonstrates accepted Phase 1 APIs without creating new scope:
  - `examples/00_quickstart.rs`;
  - `examples/01_create_tensor.rs`;
  - `examples/02_shape_and_size.rs`;
  - `examples/03_reshape_flatten.rs`;
  - `examples/04_elementwise_ops.rs`;
  - `examples/05_scalar_ops.rs`;
  - `examples/06_broadcasting.rs`;
  - `examples/07_transpose_swap_axes.rs` if RFC-007 has accepted axis operations;
  - `examples/08_slicing_builder.rs`;
  - `examples/09_slice_str.rs` only if RFC-008 accepts the bounded string parser for `0.1.0`;
  - `examples/10_json_roundtrip.rs`;
  - `examples/11_csv_numeric_loading.rs`;
  - `examples/12_boundary_error_handling.rs`.
- A required basic mathematical example suite, limited to APIs already accepted by RFC-010:
  - `examples/math/20_dot_product.rs`;
  - `examples/math/21_matrix_vector_product.rs`;
  - `examples/math/22_matrix_multiplication.rs`;
  - `examples/math/23_sum_mean.rs`;
  - `examples/math/24_min_max.rs`;
  - `examples/math/25_normalize_vector.rs`;
  - `examples/math/26_cosine_similarity.rs`.
- CI checks for examples:
  - `cargo check --examples`;
  - `cargo test --examples`;
  - a small selected `cargo run --example ...` smoke set for quickstart, broadcasting, matrix multiplication, and JSON/CSV boundaries.

M6 SHOULD include:

- `CHANGELOG.md` entry for `0.1.0`.
- A short `examples/README.md` that explains how examples are grouped and which ones are release-gated.
- Tiny committed fixtures under `examples/data/` for JSON and CSV examples.

### Exit Criteria

- A new user can complete the tutorial in under 15 minutes.
- All doc tests pass.
- Required examples compile and the smoke-run subset succeeds.
- Every required example teaches one accepted API or one small user-side pattern.
- No example requires a new public API that has not been accepted by an RFC.
- Public API is intentionally narrow and does not expose internal storage types.
- Known limitations are explicit, especially deep-copy overhead.

### Non-goals

- No Phase 2 dynamic implementation required.
- No claim of performance leadership.
- No `1.0.0` stability promise.
- No advanced linear algebra examples such as inverse, determinant, eigenvalues, SVD, QR, or Cholesky.
- No dataframe examples such as joins, group-by, pivot, or SQL-like queries.
- No ML training, automatic differentiation, GPU, sparse, or large benchmark examples.
- No default-path bridge examples for `ndarray`, `nalgebra`, `candle`, `axum`, or database crates.

---

## M7: Phase 1 Feedback, Stabilization, and Optional Example Patterns

**Target Window:** 2026-10-15 to 2026-11-11  
**Target Release:** `0.1.x`  
**Theme:** Improve reliability without widening the API too quickly.

### Scope

M7 incorporates early user feedback and stabilizes the default feature path.

M7 MUST include:

- Triage of API pain points from early usage.
- Bug fixes for shape, slicing, broadcasting, and serde edge cases.
- Improved diagnostics for the most common panics and boundary errors.
- Documentation corrections.
- Compatibility policy for `0.1.x`.

M7 SHOULD include:

- Additional examples driven by real user workflows, limited to practical PoC patterns that do not require new major APIs:
  - `examples/patterns/standardize_columns.rs`;
  - `examples/patterns/minmax_scaling.rs`;
  - `examples/patterns/rowwise_scoring.rs`;
  - `examples/patterns/column_summary.rs`;
  - `examples/patterns/moving_average.rs`;
  - `examples/patterns/rolling_windows_basic.rs`;
  - `examples/patterns/pairwise_distance.rs`;
  - `examples/patterns/gram_matrix.rs`.
- Small ergonomic additions only when they do not undermine the public API simplicity.
- Internal refactoring to prepare for Phase 2 without changing Phase 1 behavior.

### Exit Criteria

- No known critical correctness bugs in Phase 1 core operations.
- Optional examples remain small, runnable, and honest about limitations.
- Dynamic feature design can proceed without requiring a breaking rewrite of the default public API.

### Non-goals

- No large new feature families.
- No performance rewrite.
- No public view type introduction.

---

## M8: Dynamic Engine Design Lock and Spike

> **As-built note (covers M8–M11).** The Phase 2 design lock concluded that the
> originally-planned `Arc`/copy-on-write storage and dynamic arithmetic were **not**
> worth their complexity for `matten`'s PoC mission. RFC-016 reconceived `dynamic`
> as an **ingestion-and-on-ramp** engine: ingest mixed JSON/CSV into `Element`,
> clean missing values (`fill_none`, forward-fill, `none_mask`/`numeric_mask`), then
> call `try_numeric()` to obtain a plain numeric `Tensor` for computation. Dynamic
> reshape/slice/arithmetic stay guarded. Coercion is governed by an explicit
> `NumericPolicy` (RFC-017). The M8–M11 descriptions below that assume CoW views and
> dynamic arithmetic are therefore the *original plan*, not the shipped engine.

**Target Window:** 2026-11-12 to 2026-12-02  
**Target Release:** design/spike gate  
**Theme:** Freeze the Phase 2 direction while allowing disposable implementation evidence before production coding.

### Scope

M8 produces and approves RFCs for the `dynamic` feature. It also allows a disposable spike branch so that storage and ergonomics can be tested in code before the RFCs are finalized.

M8 MUST include RFCs for:

- `Element` semantics and coercion behavior.
- Text storage representation options, including at least `String`, `Box<str>`, `Arc<str>`, string interning, and small-string optimization crates.
- Dynamic `Tensor` storage model under `features = ["dynamic"]`.
- Copy-on-Write design using `Arc<Vec<Element>>` or approved equivalent.
- Feature-gating rules that preserve the user-facing `Tensor` name.
- Dynamic serde behavior for numbers, strings, booleans, and nulls.
- Missing-value APIs such as `fill_none` and forward-fill behavior.
- Compatibility with Phase 1 examples where possible.

M8 MUST include measurement criteria for:

- `std::mem::size_of::<Element>()` or equivalent storage-size measurement.
- Peak memory use for representative mixed tensors.
- Clone/materialization behavior during slicing and mutation.

M8 SHOULD include:

- A disposable implementation spike for `Element`, CoW slicing, and mixed serde.
- Memory stress target definition.
- Migration guide from Phase 1 numeric tensors to Phase 2 mixed data tensors.
- Discussion of whether `dynamic` changes default serde JSON shape.

### Exit Criteria

- Implementation team can code Phase 2 without guessing storage, text representation, or coercion rules.
- The default Phase 1 path remains protected from dynamic feature complexity.
- Spike code has either been discarded or explicitly promoted through RFC review; it MUST NOT become production code accidentally.

### Non-goals

- No production implementation required.
- No production commitment to spike code.
- No broad dataframe API.
- No SQL connector.

---

## M9: Dynamic Engine Alpha

**Target Window:** 2026-12-03 to 2027-01-13  
**Target Release:** `0.2.0-alpha.1`  
**Theme:** Implement the first heterogeneous data engine behind the `dynamic` feature.

### Scope

M9 introduces the `dynamic` feature with the minimum useful `Element` and CoW behavior.

M9 MUST include:

- `Element` semantic variants:
  - floating-point number;
  - integer number;
  - text;
  - boolean;
  - missing/null.
- A concrete text representation selected by RFC-011/RFC-012. The roadmap does not hard-lock production storage to `Text(String)` if measurement shows that doing so is too memory-heavy.
- `Element` helper methods:
  - `as_f64` or `try_as_f64`
  - `is_none`
  - basic numeric coercion policy
- Dynamic feature build path.
- CoW-backed storage prototype.
- Slicing and reshape behavior that avoids immediate deep copy when possible.
- Mutation behavior that materializes or clones safely.
- Basic mixed JSON serialization/deserialization.
- Memory layout report produced during alpha development, including `Element` size and representative tensor memory use.

M9 SHOULD include:

- Tests comparing Phase 1 and Phase 2 behavior on purely numeric data.
- Internal instrumentation for clone/materialization counts.
- Initial memory stress tests.
- A text-heavy dataset test, because text representation is the most likely source of dynamic-engine memory bloat.

### Exit Criteria

- `cargo test --features dynamic` passes.
- Numeric examples still work under `dynamic` unless an RFC explicitly documents differences.
- Slicing a large dynamic tensor does not deep-copy immediately.
- The selected `Element` representation has documented memory trade-offs and does not conflict with the Phase 2 memory goals without explicit maintainer acceptance.

### Non-goals

- No full dataframe abstraction.
- No group-by or join operations.
- No optimized query engine.

---

## M10: Messy Data Workflows

**Target Window:** 2027-01-14 to 2027-02-10  
**Target Release:** `0.2.0-beta.1`  
**Theme:** Make Phase 2 useful for real-world JSON/CSV and missing-value workflows.

### Scope

M10 completes the core dynamic workflows promised by the requirements.

M10 MUST include:

- Mixed-type JSON ingestion returning `Result<Tensor, MattenError>`.
- CSV ingestion policy for numeric and mixed fields.
- Missing-value helpers:
  - `fill_none`
  - `is_none`-based operations
  - optional forward-fill if approved by RFC
- Type coercion behavior for arithmetic on numeric-compatible values.
- Clear errors for non-coercible arithmetic.
- Documentation examples for messy business data.

M10 SHOULD include:

- Column extraction examples.
- Clear docs explaining what `matten` is not: not a full Pandas clone, not a dataframe engine, not a database.
- Memory behavior documentation for CoW views.

### Exit Criteria

- Users can ingest a small messy JSON or CSV dataset, clean missing values, slice columns, and run simple numeric operations.
- Boundary errors are recoverable and actionable.
- Dynamic examples do not require users to understand `Arc` or CoW internals.

### Non-goals

- No schema inference framework beyond simple runtime behavior.
- No lazy expression optimizer.
- No SQL-like query API.

---

## M11: Dynamic Release Hardening

**Target Window:** 2027-02-11 to 2027-03-10  
**Target Release:** `0.2.0` candidate  
**Theme:** Stabilize the `dynamic` feature for early public use.

### Scope

M11 hardens Phase 2 and updates documentation for the two-engine model.

M11 MUST include:

- Dynamic feature documentation.
- Cross-feature API compatibility review.
- Serde compatibility tests across Phase 1 and Phase 2.
- Memory stress tests for representative dynamic slicing workloads.
- Error message review for coercion and parsing failures.
- Release notes documenting limitations and migration paths.
- Required dynamic executable examples, limited to accepted dynamic APIs:
  - `examples/dynamic/00_dynamic_quickstart.rs`;
  - `examples/dynamic/01_mixed_elements.rs`;
  - `examples/dynamic/02_missing_values.rs`;
  - `examples/dynamic/03_fill_none.rs`;
  - `examples/dynamic/04_numeric_coercion.rs`;
  - `examples/dynamic/05_dirty_csv_cleanup.rs`.

M11 SHOULD include:

- Small examples for mixed CSV cleanup.
- Documentation explaining when to choose default vs `dynamic` feature.
- Web API ingestion examples only as future/feature-gated material after serde behavior is stable; they MUST NOT add default dependencies.

### Exit Criteria

- `cargo test` and `cargo test --features dynamic` pass.
- Dynamic feature does not regress Phase 1 compile time or API simplicity for default users.
- Known limitations are documented before release.

### Non-goals

- No `1.0.0` release.
- No promise that dynamic internals are final.
- No GPU or sparse tensor implementation.

---

## M12: Companion Maturity Hardening

**Target Release:** `0.19.1`  
**Theme:** Make the v0.19 companion maturity claims mechanically trustworthy before expanding scope.

### Scope

M12 is a quality and reconciliation milestone. It does not add a new product family. It hardens the crates introduced in v0.17–v0.19.

M12 MUST include:

- robust dynamic-input rejection across companion crates;
- companion rustdoc/status cleanup;
- RFC-024 / RFC-025 lifecycle cleanup;
- active-doc versioning and lock-step policy cleanup;
- strengthened release-doc checks;
- verification that core `matten` remains dependency-light under all features.

### Required implementation fixes

M12 MUST fix the feature-fragile dynamic rejection found in review:

```text
matten/dynamic enabled
companion dynamic feature not enabled
dynamic Tensor passed to companion API
  -> must return companion Err
  -> must not panic through core numeric accessors
```

Preferred design:

```rust
impl Tensor {
    pub fn is_dynamic(&self) -> bool {
        // available unconditionally;
        // returns false when core is compiled without dynamic
    }
}
```

Then companion crates SHOULD call `tensor.is_dynamic()` unconditionally.

### Required documentation fixes

M12 MUST align:

- `matten-ndarray` rustdoc and Cargo description with **production-ready candidate** status;
- `matten-mlprep` rustdoc with **beta** status;
- active docs with RFC-030 lock-step family versioning;
- mdBook install snippets with the `0.19` family line or a version-neutral form;
- migration docs so `matten-ndarray` is the recommended ndarray bridge.

### Required RFC lifecycle fixes

M12 MUST resolve:

```text
RFC-024
  move to done or mark implemented/evaluated by RFC-028/RFC-029

RFC-025
  move to done as bridge policy implemented for ndarray, with nalgebra/candle deferred
  or split future nalgebra/candle into new RFCs
```

### Acceptance criteria

M12 is complete when:

- `cargo test --workspace --all-targets` passes;
- companion dynamic rejection has a regression test;
- release-doc checks catch stale `0.15` snippets, stale maturity labels, and active-doc independent-SemVer wording;
- `scripts/check-core-dependency-boundary.sh` checks core with `--all-features --edges normal,build`;
- `matten-ndarray` can honestly remain production-ready candidate;
- `matten-mlprep` can honestly remain beta.

### Non-goals

- No `matten-data` implementation.
- No `nalgebra` or `candle` bridge.
- No streaming / large CSV implementation.
- No v1 discussion.

---

## M13: `matten-data` Beta-Decision Preparation

**Target:** `v0.20+`  
**Theme:** Decide whether `matten-data` deserves beta without drifting into dataframe scope.

### Scope

M13 is a decision milestone, not an automatic implementation milestone.

The team MUST evaluate whether a very small table-to-Tensor workflow is genuinely useful:

```text
CSV / table-like data
  -> inspect schema
  -> select columns
  -> clean missing values
  -> explicit numeric conversion
  -> Tensor output
```

### Candidate allowed scope

If `matten-data` proceeds, the allowed scope is limited to:

- CSV/string/path ingestion;
- schema summary;
- column names;
- column selection;
- missing-value cleanup;
- explicit numeric conversion;
- `Tensor` output.

### Forbidden scope

M13 MUST continue to forbid:

- joins;
- group-by;
- pivot;
- SQL-like query APIs;
- lazy execution;
- window functions;
- dataframe-style indexing;
- large-data engine claims;
- ML preprocessing that belongs in `matten-mlprep`.

### Decision outcomes

M13 must choose one of:

```text
A) promote matten-data to beta-track implementation
B) keep matten-data experimental / design-only
C) freeze or defer matten-data because existing tools serve the need better
```

### Acceptance criteria

M13 is complete when:

- the decision is recorded in an RFC or ROADMAP update;
- scope remains smaller than a dataframe library;
- example workflows can be taught in README + a few examples;
- the project can clearly explain when to use Polars/DataFusion/etc. instead.

---

## M14: `matten-data` Minimal Workflow, If Approved

**Target:** `v0.20.x` / `v0.21.x` only if M13 approves  
**Theme:** Implement only the smallest table-to-Tensor path.

### Scope

M14 is conditional. It SHOULD NOT start until M13 has explicitly approved implementation.

Possible public API shape:

```rust
use matten_data::Table;

let table = Table::from_csv_path("sales.csv")?;
println!("{}", table.schema_summary());

let x = table
    .select_columns(["sales", "cost", "quantity"])?
    .fill_missing(0.0)?
    .to_tensor()?;
```

### Acceptance criteria

- API is small enough to teach quickly.
- Missing-value and numeric-conversion behavior is explicit.
- Duplicate headers and ragged rows have documented behavior.
- Error messages are actionable.
- No dataframe-scope APIs are introduced.
- Core `matten` does not depend on `matten-data`.

---

## M15: Selective Production-Readiness Phase

**Target:** `v0.21+`  
**Theme:** Promote only proven crates.

By M15, the project should evaluate crates independently by maturity label, while retaining lock-step family versioning.

Possible status after M15:

```text
matten
  stable pre-1.0 core

matten-ndarray
  production-ready or production-ready candidate

matten-mlprep
  production-ready candidate or beta

matten-data
  beta / experimental / frozen depending on M13-M14 result
```

A companion crate is production-ready only if:

- public API is stable enough for its documented scope;
- examples compile in CI;
- limitations are clear;
- no hidden core dependency pollution exists;
- errors are crate-local and actionable;
- the crate can be ignored completely by users who only need core `matten`.

---

## M16: Deferred Bridge Crates

**Target:** later  
**Theme:** Bridge only where user value exceeds maintenance cost.

`matten-nalgebra` and `matten-candle` remain deferred.

Before either is implemented, the team MUST write a separate RFC covering:

- target external crate version;
- version-coupling / bump policy;
- copy vs view behavior;
- supported tensor ranks and shapes;
- error type;
- dependency cost;
- why the bridge belongs in the `matten` family.

No future bridge may add dependencies to core `matten`.

---

## M17: Streaming / Large CSV Exploration

**Target:** later  
**Theme:** Design before implementation.

Streaming remains high risk because it requires policy decisions about:

- batch lifecycle;
- schema drift;
- malformed rows;
- fail-fast vs skip behavior;
- sync vs async API;
- memory budget;
- relationship to `matten-data`;
- whether it belongs in `matten-data` or `matten-stream`.

Implementation SHOULD NOT begin until these are settled by RFC.

---

## 5. RFC Roadmap

> **As-built.** The forecast below (RFC-001…) was renumbered and extended during
> implementation. The actual pack is RFCs **000–030**, all resolved in
> `rfcs/done/`: 000 lifecycle; 001–014 core (threat model, API minimalism, shape,
> construction, errors, broadcasting, reshape/axes, slicing, serde/JSON/CSV,
> reductions/matmul, dynamic `Element`, dynamic storage, testing gates, examples);
> 015–021 stabilization (API stabilization, **dynamic ingestion on-ramp [016]**,
> numeric policy [017], resource limits [018], axis reductions [019], diagnostics
> [020], tutorial gate [021]); 022–026 workspace & companions (boundary [022],
> matten-data scope [023], matten-mlprep scope [024], bridge policy [025],
> streaming policy [026]); 027–030 companion impl & policy (matten-ndarray [027],
> matten-mlprep [028], maturity evaluation [029], lock-step versioning [030]). See
> `rfcs/README.md`.

The original RFC table below is retained for traceability. It is no longer a future work queue. The actual RFC pack has reached RFC-030 and is recorded in `rfcs/done/` as described above.

Current forward RFC work SHOULD focus on M12+ topics, not reopening completed Phase 1/Phase 2 RFCs.

| RFC | Title | Target Milestone | Purpose |
|---:|---|---|---|
| RFC-001 | Threat Model and Boundary Safety Policy | M0-M1 | Define panic zone, `Result` zone, input limits, allocation safety, parser safety, and `unsafe` policy |
| RFC-002 | Public API Minimalism and `Tensor` Contract | M1 | Freeze the rule that users primarily import `matten::Tensor` and do not see generic/lifetime-heavy surfaces |
| RFC-003 | Shape Model, Scalar Semantics, and Validation | M1 | Define shape product, zero-dimensional tensors, max dimension policy, and overflow handling |
| RFC-004 | Construction and Conversion APIs | M2 | Define constructors and `From`/`Into`/`TryFrom` behavior |
| RFC-005 | Error Model, Panic Messages, and Boundary APIs | M2-M5 | Make panic-vs-`Result` rules enforceable |
| RFC-006 | Broadcasting and Element-Wise Operators | M3 | Define NumPy-like shape compatibility, borrowed tensor ops, scalar ops, and error cases |
| RFC-007 | Reshape, Axis Operations, and Indexing | M4 | Define ownership/copy behavior, transpose, swap axes, and axis validation |
| RFC-008 | Slicing API: Builder and `slice_str` | M4 | Define builder-first semantics, bounded parser grammar, deferral criteria, and errors |
| RFC-009 | Serde, JSON, CSV, and Boundary Integration | M5 | Define external data contracts and malformed-input behavior |
| RFC-010 | Reductions, Basic Statistics, and Matrix Multiplication | M6 | Define the small mathematical-computing surface allowed before `0.1.0` |
| RFC-011 | Dynamic `Element` Model and Coercion | M8 | Define heterogeneous data semantics, text representation, missing values, and coercion policy |
| RFC-012 | Dynamic Storage, View Metadata, and Copy-on-Write | M8 | Define `Arc`/CoW design, materialization rules, and memory measurement gates |
| RFC-013 | Testing, Compatibility, and Release Gates | M6-M11 | Define compatibility matrix, CI gates, property tests, and release approval criteria |
| RFC-014 | Example Suite and Executable Documentation | M6-M11 | Define required examples, optional examples, future examples, example CI, fixtures, and anti-scope rules |

RFC-014 MUST explicitly state that examples do not create new API requirements by themselves. An example is eligible for the required suite only when the API it demonstrates has already been accepted by an earlier RFC.

### 5.1 Active / Future RFC candidates after v0.19.0

The following are candidate RFC themes. Numbers are intentionally not assigned here unless the repository already reserves them.

| Candidate | Target | Purpose |
|---|---|---|
| Companion dynamic rejection hardening | M12 / v0.19.1 | Ensure dynamic tensors return companion errors across feature-unification scenarios. |
| `matten-data` beta-decision RFC | M13 / v0.20+ | Decide promote / keep experimental / freeze. |
| `matten-data` minimal implementation RFC | M14, conditional | Specify a tiny table-to-Tensor workflow if M13 approves. |
| `matten-nalgebra` bridge RFC | M16, deferred | Evaluate version coupling, scope, copy behavior, and user value. |
| `matten-candle` bridge RFC | M16, deferred | Evaluate heavy dependency cost and ML-framework boundary. |
| Streaming / large CSV RFC | M17, deferred | Define batch lifecycle, schema drift, error policy, and memory budget before implementation. |

## 6. Dependency and Feature Policy

### 6.1 Default Dependency Policy

The default crate SHOULD remain small and quick to compile.

Default dependencies SHOULD be limited to:

- `serde` if retained as a first-class default integration;
- small error/display support only if justified;
- no heavy numerical backend dependency in Phase 1.

The project MUST NOT introduce `ndarray`, `nalgebra`, `candle`, BLAS, or GPU dependencies into the default path unless a later RFC explicitly changes the project identity.

### 6.2 Feature Flags

| Feature | Default | Intended Scope |
|---|---:|---|
| `serde` | Yes | Serialization / deserialization support |
| `json` | Yes | JSON parsing and output helpers; depends on `serde` |
| `csv` | Yes | Small rectangular CSV ingestion |
| `dynamic` | No | Heterogeneous ingestion, cleanup, masks, and explicit `try_numeric()` on-ramp |

Feature flags MUST NOT require users to import a different primary public tensor name. The user-facing type remains `Tensor`.

`dynamic` is **not** a full CoW computation engine in the shipped design. It is an ingestion/on-ramp feature. Dynamic arithmetic, dynamic broadcasting, and dynamic matmul remain out of core scope unless a future RFC reopens them.

Public macro APIs SHOULD be avoided before `0.1.0`. This keeps the crate aligned with the low-macro, low-trait-complexity philosophy. A later slicing macro may be proposed only if the builder API has proven insufficient and the macro does not become a type puzzle.

---

## 7. Quality Gates

## 7.1 Every Milestone Gate

Each milestone MUST satisfy the workspace-appropriate quality gates:

- `cargo fmt --all --check`
- `bash scripts/check-core-dependency-boundary.sh`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` or a documented equivalent policy
- `cargo test --workspace --all-targets`
- `cargo test --workspace --doc`
- `cargo check --workspace --examples --all-features`
- no accidental public exposure of internal modules
- README / rustdoc / mdBook / RFC index updated when behavior or maturity status changes

## 7.1.1 M4 Slicing Gate

Before M4 is considered complete, the project MUST verify:

- the builder API works without lifetimes, view types, or public storage exposure;
- parser implementation, if included, is small enough to audit;
- invalid `slice_str` input returns `MattenError`;
- string slicing is documented as a convenience API, not the canonical internal slicing model.

## 7.2 Phase 1 Release Gate

Before `0.1.0`, the project MUST have:

- working required default-feature examples from M6;
- no required example that depends on unaccepted API surface;
- documented panic behavior;
- documented boundary `Result` behavior;
- tested broadcasting;
- tested slicing;
- tested serde JSON round-trip;
- tested CSV ingestion if included;
- public API review;
- changelog entry.

## 7.3 Phase 2 / Dynamic Release Gate

> **As-built.** The original CoW/dynamic-computation release gate is superseded by
> RFC-016's ingestion/on-ramp model.

For the shipped dynamic feature, the required gate is:

- `dynamic` feature test matrix;
- tests for all `Element` semantic variants;
- mixed JSON/CSV tests;
- missing-value workflow tests;
- `NumericPolicy` tests;
- `try_numeric()` strict/permissive conversion tests;
- required dynamic examples from M11 or their renamed shipped equivalents;
- documentation explaining that dynamic is for ingestion/cleanup and that computation should happen after explicit conversion to numeric `Tensor`.

CoW materialization tests are no longer a release gate unless a future RFC revives CoW dynamic views.

## 7.4 M12 Companion Maturity Gate

Before v0.19.1 can be considered complete, the project MUST verify:

- dynamic tensors passed to companion crates return crate-local `Err`, not panic;
- `matten-ndarray` rustdoc, README, Cargo description, and ROADMAP status agree;
- `matten-mlprep` rustdoc, README, Cargo description, and ROADMAP status agree;
- RFC-024 and RFC-025 lifecycle status no longer contradicts RFC-027/RFC-028/RFC-029;
- active docs use lock-step family versioning per RFC-030;
- release-doc checks catch stale version snippets and stale maturity labels.

## 7.5 M13 `matten-data` Decision Gate

Before `matten-data` can move toward beta, the project MUST verify:

- the workflow is useful without dataframe ambition;
- API surface fits in README plus a few examples;
- missing values and numeric conversion are explicit;
- duplicate headers, ragged rows, and non-convertible columns have documented behavior;
- no join/group-by/pivot/SQL/lazy/window functionality appears;
- core `matten` remains independent from `matten-data`.

---

## 8. Testing Strategy

### 8.1 Unit Tests

Unit tests SHOULD cover:

- shape validation;
- scalar shape `[]`;
- constructor behavior;
- conversion behavior;
- broadcasting edge cases;
- slicing grammar;
- axis validation;
- boundary error handling;
- dynamic element coercion;
- dynamic text-heavy storage behavior.

### 8.2 Property-style Tests

Property-style tests SHOULD be introduced selectively for:

- reshape preserving element count;
- broadcasting output shape;
- transpose shape inversion;
- slice bounds safety;
- serialization round-trip where feasible.

The project SHOULD avoid heavy test-time dependencies until they are justified.

### 8.3 Example Tests

All examples in README and docs MUST compile as doc tests or be mirrored by integration tests. Required examples MUST be checked in CI. Examples that require optional cargo features MUST be clearly marked and tested only under those features.

Examples MUST follow the scope filter from RFC-014:

- demonstrate accepted APIs;
- stay small and copy-pasteable;
- avoid default extra dependencies;
- avoid advanced linear algebra, dataframe, ML training, GPU, sparse, and benchmark-marketing scope.

### 8.4 Benchmark Smoke Tests

Benchmarks are not used to compete with performance libraries. They are used to detect accidental pathological regressions.

Benchmark smoke tests SHOULD cover:

- tensor construction;
- element-wise addition;
- broadcasting;
- reshape;
- slice;
- JSON/CSV ingestion;
- dynamic CoW slicing once Phase 2 begins.

---

## 9. Documentation Roadmap

| Milestone | Documentation Deliverable |
|---|---|
| M0 | README philosophy and quick start stub |
| M1 | Core `Tensor` docs and shape explanation |
| M2 | Creation/conversion guide |
| M3 | Arithmetic and broadcasting guide |
| M4 | Reshape/slicing guide with examples |
| M5 | Boundary integration guide for JSON/CSV and web payload concepts |
| M6 | Complete Phase 1 tutorial, required example suite, and release notes |
| M7 | FAQ from early user feedback and optional practical example patterns |
| M8 | Dynamic design notes and migration plan |
| M9 | Dynamic alpha notes and spike findings |
| M10 | Messy data tutorial draft |
| M11 | Feature selection guide: default vs `dynamic`, plus required dynamic examples |
| M12 | Companion maturity docs: rustdoc status, README status, RFC lifecycle, release-doc script coverage |
| M13 | `matten-data` decision memo: promote / experimental / freeze, with scope checklist |
| M14 | `matten-data` minimal tutorial only if implementation is approved |
| M15 | Production-readiness status matrix across family crates |
| M16 | Separate bridge RFCs and migration guides only if nalgebra/candle are approved |
| M17 | Streaming design notes only after batch/error/schema/memory policy is settled |

---

## 10. Risk Register

| Risk | Probability | Impact | Mitigation |
|---|---:|---:|---|
| String slicing parser grows into a mini-language | Medium | Medium | Builder-first M4 scope; bounded parser grammar; defer parser if it threatens milestone |
| Dynamic `Element` representation causes memory bloat | High | High | RFC-011/RFC-012 memory layout gate; test text-heavy datasets; allow optimized text representation |
| RFC gate delays Phase 2 learning | Medium | Medium | Allow disposable spike branch; require RFC approval only before production implementation |
| API grows too quickly and loses simplicity | Medium | High | Public API review at every milestone; RFC gate for new modules |
| Companion maturity labels drift between README/rustdoc/ROADMAP | Medium | Medium | M12 release-doc checks; crate-level status must match maturity decision |
| Dynamic companion rejection panics under feature unification | Medium | High | M12 unconditional `is_dynamic()` guard and regression fixture |
| Lock-step versioning confuses users about maturity | Medium | Medium | Status labels in README/rustdoc; explain version = compatibility, status = maturity |
| `matten-data` drifts into dataframe scope | High | High | M13 beta-decision gate; forbid join/group-by/pivot/SQL/lazy execution |
| `ndarray` version coupling creates bridge maintenance treadmill | Medium | Medium | RFC-025 version range; bump policy; keep bridge API tiny |
| Example suite becomes hidden scope expansion | Medium | High | RFC-014 gates examples against accepted APIs; defer advanced/business/integration examples |
| Phase 1 deep copies surprise users on large tensors | High | Medium | Document limitation; add warnings in docs; provide migration path to dynamic/other crates |
| Broadcasting bugs create silent wrong results | Medium | High | Exhaustive compatibility tests; panic on ambiguous/incompatible shapes |
| Dynamic feature contaminates default compile time | Medium | High | Keep `dynamic` disabled by default; compile-time checks for default path |
| `Tensor` behavior differs too much across features | Medium | Medium | Cross-feature compatibility RFC and tests |
| CSV/JSON scope expands into dataframe territory | High | Medium | Explicit non-goals; keep ingestion and cleanup minimal |
| Panic policy is misunderstood as production-safe | Medium | Medium | Boundary Error Rule must be visible in README/tutorial |
| Performance criticism distracts from mission | High | Low | Keep messaging clear: this is a PoC DX crate, not benchmark leader |

---

## 11. Non-goals Across the Roadmap

The following are intentionally out of scope for this roadmap unless a later strategic RFC changes direction:

- competing with `ndarray`, `nalgebra`, `candle`, or BLAS on performance;
- exposing generic `Tensor<T>` in Phase 1;
- exposing lifetime-based view types to ordinary users;
- GPU acceleration;
- sparse tensors;
- distributed arrays;
- automatic differentiation;
- full dataframe functionality;
- SQL query engine;
- expression optimizer;
- advanced linear algebra decomposition APIs;
- production-stable `1.0.0` API guarantee;
- examples that imply full dataframe behavior such as joins, group-by, pivot, or query execution;
- examples that imply advanced linear algebra scope such as eigenvalues, decompositions, or matrix inversion;
- examples that imply ML framework scope such as training loops, autograd, neural networks, or GPU execution;
- examples that add default dependencies on `ndarray`, `nalgebra`, `candle`, web frameworks, or database crates.

---

## 12. Release Decision Checklist

Before any public release candidate, maintainers SHOULD answer:

1. Does the release preserve the family-car identity?
2. Can a beginner complete the quick start without type annotations beyond ordinary Rust variable declarations?
3. Are panics actionable and human-readable?
4. Do all I/O and parsing boundaries return `Result`?
5. Are limitations stated honestly?
6. Are public APIs narrow enough to support future change?
7. Does default compilation remain lightweight?
8. Are examples realistic and runnable?
9. Do examples demonstrate accepted APIs rather than creating new implicit scope?
10. Are feature flags understandable?
11. Is this release version appropriate, or should it remain alpha/beta?

---

## 13. Recommended Immediate Next Steps

The next work is **not** new feature expansion. The immediate next milestone is M12 / v0.19.1 hardening.

1. Fix companion dynamic rejection so dynamic tensors return companion `Err`, not panic, even when only core `matten/dynamic` is enabled.
2. Align companion rustdoc / Cargo description / README maturity labels:
   - `matten-ndarray`: production-ready candidate;
   - `matten-mlprep`: beta.
3. Clean up RFC lifecycle status:
   - RFC-024 should no longer remain merely proposed after `matten-mlprep` shipped and reached beta;
   - RFC-025 should be marked implemented-for-ndarray with nalgebra/candle deferred, or split into future bridge RFCs.
4. Update active docs and scripts:
   - no stale `0.15` install snippets;
   - no active independent-SemVer wording;
   - release-doc checks catch stale maturity/status labels.
5. Re-run full workspace quality gates, including the core dependency-boundary script.
6. Only after M12 is clean, start M13: the `matten-data` beta-decision phase.

M13 must choose promote / keep experimental / freeze. It must not become automatic `matten-data` implementation.



---

## 14. Document History

| Version | Date | Change |
|---|---|---|
| 0.3.0 | 2026-06-18 | Original roadmap/milestone plan with M0–M11 and scoped example strategy. |
| 0.4.0 | 2026-06-21 | Reconciled the original M0–M11 plan with as-built state through v0.19.0; recorded narrowed dynamic model and companion-crate arc. |
| 0.5.0 | 2026-06-21 | Added active M12+ milestones, v0.19.1 hardening gate, v0.20+ `matten-data` beta-decision gate, updated quality gates, feature policy, risk register, documentation roadmap, and immediate next steps. |

---

## Appendix A: Review Points Not Fully Adopted

### A.1 Public Slicing Macro

The review suggested considering a lightweight declarative macro such as `slice![0..2, ..]`. This roadmap does not adopt that recommendation for the initial schedule. The reason is not that a macro is technically bad, but that `matten` should avoid opening multiple public syntax surfaces before the builder API is proven.

A macro MAY be reconsidered after `0.1.0` if:

- the builder API is confirmed to be too verbose in real user examples;
- the macro remains a thin wrapper over the builder model;
- the macro does not introduce hidden type-level complexity;
- the macro can produce clearer diagnostics than `slice_str`.

### A.2 Removing `slice_str` Completely

This roadmap does not remove `slice_str` completely, because NumPy-style string slicing remains valuable for exploratory and tutorial-oriented workflows. However, the parser is now secondary and explicitly deferrable. The project should not sacrifice the reliable builder API merely to ship string slicing earlier.

### A.3 Weakening the M8 Design Gate Too Much

> **Historical note.** M8 has already completed, and RFC-016 narrowed the dynamic engine.

The review correctly warned about analysis paralysis, but removing the M8 gate entirely would create a different risk: accidental Phase 2 API lock-in around an unmeasured `Element` representation. The revised plan therefore keeps M8 as a design gate while allowing spike code to inform the RFCs.
