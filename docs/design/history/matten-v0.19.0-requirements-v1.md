# `matten` Product Requirements Document / Software Requirements Specification

> **HISTORICAL SNAPSHOT — DO NOT CITE AS CURRENT.**
> As-built through v0.19.0 (RFCs 000–030). Superseded by the current RFC corpus
> (`rfcs/`) and user documentation (`docs/src/`); forward schedule lives in
> `ROADMAP.md`. This document froze ~35 RFCs ago and predates matten-data, the
> 0.20–0.30 API/companion work, and the visual/educational/migration programs.
> Retained for design traceability only. Section-by-section canonical owners:
> see `docs/design/README.md`. Terminology note: the "Phase 1/Phase 2/Sedan/SUV"
> vocabulary here is retired and is banned from user docs.

**Project:** `matten`  
**Document Kind:** Product Requirements Document (PRD) / Software Requirements Specification (SRS)  
**Document Version:** `2.1.0`  
**Target Crate Version:** as-built through the `0.19.0` family release (core `matten`, plus companions `matten-ndarray` and `matten-mlprep`)  
**Prepared For:** `matten` maintainers and implementation developers  
**Date:** 2026-06-21  
**Status:** Living requirements — reviewed and corrected against the shipped state through v0.19.0  
**Revision note:** `2.1.0` is a review correction over `2.0.0`: it keeps the as-built v0.19.0 framing, but removes remaining normative conflicts in the historical Phase 2 / feature / error sections, records the exact shipped `MattenError` shape, and clarifies companion-crate dynamic rejection as a contract that must be hardened in implementation. Canonical forward planning lives in `ROADMAP.md`; canonical design decisions live in `rfcs/`.

---

## 0. Reading Guide

This document defines the requirements for `matten`, a Rust multidimensional array library designed for rapid prototyping, data exploration, and business Proof of Concept workflows.

The document intentionally separates **requirements** from **implementation design**. It does, however, define enough API shape, feature boundaries, error policy, and acceptance criteria for an implementation team to start external design and RFC work without guessing the product intent.

Requirement language follows this convention:

- **MUST**: required for the stated phase.
- **SHOULD**: strongly recommended unless a later RFC gives a concrete reason to deviate.
- **MAY**: allowed but not required.
- **MUST NOT**: explicitly prohibited.

### 0.1 As-built status through v0.19.0 (read this first)

The body of this PRD below is the original kickoff requirement set, retained for
traceability. This subsection records what was actually built and where the
shipped reality diverged from the kickoff plan. Where the two disagree, **this
subsection and the RFCs are authoritative**; the original sections are historical
context.

**The project is now a Cargo workspace, not a single crate.** Core `matten` is one
member; two companion crates were added after kickoff. All three share one
**lock-step family version** (`0.19.0`), set in `[workspace.package].version`
(RFC-030). A crate's *maturity* is shown by a Status label, not by its version
number.

| Crate | Version | Maturity | Role |
|---|---|---|---|
| `matten` | 0.19.0 | stable (v0.x) | The core `f64` `Tensor` library (Phase 1) plus a feature-gated `dynamic` ingestion engine (Phase 2, reconceived — see below). |
| `matten-ndarray` | 0.19.0 | production-ready candidate | Conversion bridge `Tensor` ↔ `ndarray::ArrayD<f64>` (RFC-025, RFC-027). |
| `matten-mlprep` | 0.19.0 | beta | Transparent, deterministic preprocessing helpers over numeric tensors (RFC-024, RFC-028). |

**Phase 1 (the f64 Sedan) shipped as specified** and stabilized. Construction,
shape model, reshape/transpose/flatten/swap-axes, broadcasting, element-wise and
scalar operators, builder + string slicing, reductions (`sum`/`mean`/`min`/`max`
and axis reductions), `matmul`/`dot`, serde JSON, CSV ingestion, file loading, and
the `From`/`TryFrom` conversions are all implemented. The boundary-error rule
(panic zone vs `Result` zone) holds. `#![forbid(unsafe_code)]` holds.

**Phase 2 (the dynamic engine) was deliberately narrowed.** The kickoff PRD (§2.2,
§7, §9.2) envisaged a full heterogeneous engine with dynamic arithmetic and
`Arc`-backed copy-on-write slicing. RFC-016 reconceived `dynamic` as an
**ingestion-and-on-ramp layer**, not a second computation engine: it ingests mixed
JSON/CSV into `Element` values, supports missing-value cleanup (`fill_none`,
forward-fill, masks), and provides an **explicit `try_numeric()` on-ramp** to a
plain numeric `Tensor`. Dynamic reshape/slice/arithmetic remain **guarded** (they
return `MattenError::Unsupported` or panic per the zone they sit in) rather than
implementing CoW views. The CoW/dynamic-arithmetic requirements below are
therefore **deferred/superseded**, not shipped.

**Heavy interop and workflow features moved *out* of core, into companion crates**
(RFC-022 boundary policy). Core `matten` depends on none of `ndarray`,
`nalgebra`, `candle`, `polars`, or any `matten-*` crate — a rule enforced in CI by
`scripts/check-core-dependency-boundary.sh`. `ndarray` interoperability
(originally sketched as an in-core optional feature, §12.1) is now the separate
`matten-ndarray` crate. Preprocessing patterns (column standardization, min-max
scaling, bias column, train/test split) are the separate `matten-mlprep` crate.

**The public error surface consolidated.** The shipped `MattenError`
(`#[non_exhaustive]`) is `Shape`, `Broadcast`, `Allocation`, `Slice`,
`Parse { format: DataFormat }`, `Io`, `Unsupported` — replacing the larger,
finer-grained variant list sketched at kickoff (RFC-005). Companion crates define
their **own** error types (`MattenNdarrayError`, `MattenMlprepError`); core
`MattenError` does not grow companion variants (RFC-022 §8). `MattenError` derives
only `Debug` (it embeds `std::io::Error`); match it by variant, not `==`.

**Resource-safety limits are first-class.** `MattenLimits` (RFC-018) is the single
source of truth for `MAX_NDIM` and the allocation/`arange` element budgets;
`zeros`/`ones`/`full`/`arange` delegate through `try_*` to enforce them.

For the authoritative current schedule see `ROADMAP.md`; for the authoritative
design decisions see `rfcs/done/` (RFCs 000–030). The remaining sections of this
document are the original requirements, annotated where shipped reality differs.

### 0.2 Review corrections in v2.1

This revision makes the as-built state easier to consume by future developers. In
particular:

- historical Phase 2 requirements for CoW dynamic views and dynamic arithmetic are
  explicitly marked **superseded/deferred** rather than left as active MUSTs;
- the actual feature matrix is recorded as `default = ["serde", "json", "csv"]`,
  with `dynamic` optional;
- `Element::Text` is recorded as the shipped `Arc<str>` representation;
- the exact shipped `MattenError` fields are recorded;
- companion crates must reject dynamic tensors through `Result` errors, not panics.
  If feature unification can bypass that guard, it is a v0.19.1 implementation
  hardening item, not a change in product direction.

---

## 1. Executive Summary and Goals

### 1.1 Mission

`matten` is a developer-centric multidimensional array library for Rust. Its mission is to make early-stage mathematical computing in Rust feel close to NumPy or Pandas in approachability, while remaining native Rust and easy to integrate into Rust application stacks.

The primary value is **developer speed**, not benchmark leadership. `matten` is the “family car” for multidimensional data work: comfortable, predictable, forgiving, and easy to start. It is not intended to replace highly optimized engines such as `ndarray`, `nalgebra`, `candle`, or domain-specific linear algebra kernels.

### 1.2 Problem Statement

Existing Rust mathematical array libraries are excellent for performance-oriented and strongly typed numerical computing, but they can create friction during early prototyping:

- users must often understand generic type parameters, shape representations, view lifetimes, memory layout, and trait bounds;
- slicing and reshaping may expose borrowing, ownership, or contiguity constraints to users;
- compile errors can be intimidating to non-specialists;
- integrating quick data ingestion from JSON, CSV, web handlers, or business data sources usually requires additional glue code.

`matten` solves this by hiding internal complexity behind a small, concrete, stable public API.

### 1.3 Core Philosophy: DX Over Benchmarks

| Area | Conventional high-performance Rust approach | `matten` approach |
|---|---|---|
| Public type model | Generic arrays, views, storage abstractions | One primary public type: `Tensor` |
| Lifetimes | Views may expose lifetime propagation | No user-visible tensor lifetimes in common APIs |
| Reshape/slice behavior | May depend on layout and contiguity | Internally clone or remap so the user does not manage layout |
| Error handling inside math operations | Often explicit `Result` or trait-level failures | Human-readable fail-fast panics for internal prototyping operations |
| Error handling at I/O boundaries | Varies by ecosystem integration | Always return `Result<Tensor, MattenError>` |
| Performance target | Minimize allocation and maximize throughput | Minimize user friction and compile-time complexity |

### 1.4 Business Goals

| ID | Goal | Success Indicator |
|---|---|---|
| BG-01 | Reduce time from idea to runnable Rust numerical PoC | A new user can build a simple tensor workflow within 15 minutes using the tutorial |
| BG-02 | Enable Rust adoption by Python-oriented data users | Documentation includes direct NumPy/Pandas-style examples with Rust equivalents |
| BG-03 | Support web-service and business-data integration | JSON serialization/deserialization and CSV ingestion are documented and tested |
| BG-04 | Keep the first release small and learnable | Public API remains intentionally narrow and discoverable |
| BG-05 | Preserve migration paths to specialized crates | Documentation explains when to move from `matten` to `ndarray`, `nalgebra`, or `candle` |

### 1.5 Technical Goals

| ID | Goal | Acceptance Criteria |
|---|---|---|
| TG-01 | No visible lifetimes in standard tensor workflows | All beginner examples compile without explicit lifetime annotations |
| TG-02 | Single concrete numerical tensor in Phase 1 | Default `Tensor` stores `f64` values internally; no generic `Tensor<T>` in Phase 1 |
| TG-03 | NumPy-like runtime broadcasting | Element-wise operators support right-aligned broadcasting for compatible shapes |
| TG-04 | Safe boundary behavior | `from_json`, `from_csv`, and file-loading APIs return `Result<Tensor, MattenError>` |
| TG-05 | Low compile-time complexity | No recursive type-level shape arithmetic, public proc macros, or deep trait stacks are required |
| TG-06 | Phase 2 extensibility | Dynamic heterogeneous data can be added without changing the public `Tensor` name |

---

## 2. Product Scope

### 2.1 In Scope for Phase 1: “The Sedan”

Phase 1 is the default crate experience. It targets numerical PoCs using `f64` tensors.

Phase 1 MUST provide:

- `Tensor` backed by owned contiguous `Vec<f64>` storage;
- factory methods: `new`, `from_vec`, `zeros`, `ones`, `full`, and selected convenience constructors;
- shape inspection and validation;
- runtime reshape and transpose operations;
- element-wise arithmetic with broadcasting;
- scalar arithmetic with `f64`;
- basic reduction operations such as `sum`, `mean`, `min`, and `max`;
- indexing and slicing sufficient for common 1D/2D/ND PoC workflows;
- serde-compatible JSON representation;
- CSV and JSON ingestion APIs returning `Result`;
- clear panic messages for internal invalid operations.

### 2.2 In Scope for Phase 2: “The SUV”

> **As-built (RFC-016).** Phase 2 shipped as an **ingestion-and-on-ramp** layer, not
> a second computation engine. `Element` ingestion, missing-value utilities, and a
> JSON/CSV mixed-data path are implemented; the **copy-on-write `Arc` storage and
> dynamic arithmetic** below were *not* built — dynamic reshape/slice/arithmetic are
> guarded, and the intended workflow is "ingest → clean → `try_numeric()` → operate on
> a plain numeric `Tensor`." See §0.1 and RFC-016/017.

Phase 2 is enabled by `features = ["dynamic"]`. It targets messy real-world data containing numbers, text, booleans, and missing values.

Phase 2, as shipped through v0.19.0, MUST provide the narrowed on-ramp contract:

- dynamic `Element` values: `Float(f64)`, `Int(i64)`, `Text(Arc<str>)`, `Bool(bool)`, and `None`;
- heterogeneous tensor construction and JSON/CSV ingestion through explicit dynamic APIs such as `from_json_dynamic` and `from_csv_dynamic`;
- missing-value utilities such as `fill_none`, fallback-based `forward_fill_none`, `none_mask`, `numeric_mask`, and `count_none`;
- explicit numeric conversion through `try_numeric()` and `try_numeric_with(NumericPolicy)`;
- dynamic tensors must be rejected by numeric compute APIs with clear unsupported-operation messages rather than silently exposing empty numeric storage.

The following kickoff requirements are **superseded/deferred** and are not active v0.19.0 requirements unless a future RFC revives them:

- dynamic arithmetic as a peer computation engine;
- CoW dynamic reshape/slicing as a public performance guarantee;
- `Text(String)` as the fixed text storage representation.

### 2.3 Out of Scope for Initial Phases

The following are non-goals for Phase 1 and Phase 2 unless later RFCs explicitly promote them:

- GPU acceleration;
- BLAS/LAPACK integration;
- sparse matrix formats;
- symbolic computation;
- automatic differentiation;
- distributed arrays;
- high-performance ML tensor kernels;
- compile-time shape proofs;
- generic `Tensor<T>` as the primary user-facing type;
- replacing `ndarray`, `nalgebra`, or `candle` for performance-sensitive production workloads.

---

## 3. Architecture and Cargo Feature Scope

### 3.1 Feature Strategy

`matten` MUST expose a stable high-level namespace. The ordinary user should start with:

```rust
use matten::Tensor;
```

Dynamic users MAY additionally import `Element`:

```rust
#[cfg(feature = "dynamic")]
use matten::{Element, Tensor};
```

The crate SHOULD use Cargo features to separate storage engines and optional integrations.

As-built v0.19.0 feature layout:

```toml
[features]
default = ["serde", "json", "csv"]
serde = ["dep:serde"]
json = ["serde", "dep:serde_json"]
csv = ["dep:csv"]
dynamic = []
```

`default-features = false` remains the lean core profile. The default feature set
chooses PoC convenience by enabling JSON and CSV boundary helpers.

The exact dependency graph MAY be refined during implementation, but the following product-level policy is fixed:

- serde integration is a first-class requirement;
- JSON ingestion APIs are boundary APIs and MUST return `Result`;
- CSV ingestion APIs are boundary APIs and MUST return `Result`;
- enabling `dynamic` MUST NOT change the public name `Tensor`.

### 3.2 Phase 1 Storage Model

Phase 1 internal storage is conceptually:

```rust
pub struct Tensor {
    data: Vec<f64>,
    shape: Vec<usize>,
}
```

The fields MUST NOT be public in the first release. Access is provided through methods such as:

```rust
impl Tensor {
    pub fn shape(&self) -> &[usize];
    pub fn ndim(&self) -> usize;
    pub fn len(&self) -> usize;
    pub fn is_scalar(&self) -> bool;
    pub fn as_slice(&self) -> &[f64];
    pub fn to_vec(&self) -> Vec<f64>;
}
```

Phase 1 MUST prioritize independence and simplicity:

- reshaping returns a new `Tensor`;
- slicing returns a new `Tensor`;
- transposition returns a new `Tensor`;
- no returned tensor view borrows from the source tensor;
- the user never handles a lifetime-bearing view type.

Internal cloning is acceptable and expected. The implementation SHOULD document that Phase 1 can allocate more than performance-focused libraries.

### 3.3 Phase 2 Dynamic Storage Model

As-built v0.19.0 dynamic storage is an ingestion/on-ramp implementation detail,
not a public view engine. `Element::Text` is represented as `Arc<str>` for cheap
clones:

```rust
#[cfg(feature = "dynamic")]
pub enum Element {
    Float(f64),
    Int(i64),
    Text(std::sync::Arc<str>),
    Bool(bool),
    None,
}
```

Dynamic tensors are allowed to store heterogeneous values, expose inspection and
cleanup helpers, and convert explicitly to numeric tensors. They do **not** promise
public CoW views, dynamic arithmetic, or mutation-through-view behavior in the
v0.19.0 contract.

### 3.4 Conditional Compilation Policy

Feature flags MUST NOT create unrelated user experiences. The following rules apply:

- `Tensor` remains the central public type in all configurations.
- Phase 1 APIs MUST continue to compile when `dynamic` is disabled.
- Dynamic-only APIs MUST be clearly gated with `#[cfg(feature = "dynamic")]` and documented as such.
- Companion crates that receive dynamic tensors MUST return their own `DynamicTensor` errors rather than panicking.
- Public docs MUST show which examples require which features.
- Internal storage modules MAY differ completely by feature.

---

## 4. Data Model Requirements

### 4.1 Shape Model

`Tensor` shape is a runtime vector of dimension lengths.

Requirements:

| ID | Requirement |
|---|---|
| SHP-01 | Shape MUST be stored as ordered dimensions, e.g. `[2, 3]` for two rows and three columns. |
| SHP-02 | Shape product MUST equal data length for fully materialized tensors. |
| SHP-03 | Empty shape `[]` MUST represent a scalar tensor containing exactly one element. |
| SHP-04 | Dimension values in Phase 1 SHOULD be greater than zero. Zero-sized dimensions MAY be added later by RFC. |
| SHP-05 | Rank SHOULD be capped at 8 in the first release unless implementation cost is clearly low. |
| SHP-06 | Shape product calculation MUST use checked multiplication to prevent overflow. |
| SHP-07 | Panic messages and errors MUST include both shape and data-length context. |

### 4.2 Layout Model

Phase 1 MUST use row-major contiguous layout for materialized data. For shape `[2, 3]`, flat storage order is:

```text
[[a, b, c],
 [d, e, f]]

flat = [a, b, c, d, e, f]
```

Phase 2 MAY represent non-contiguous views internally, but public methods that return `Vec` or slices MUST materialize or expose data in canonical row-major order.

### 4.3 Scalar Model

A scalar tensor is shape `[]` with length `1`.

Examples:

```rust
let x = Tensor::scalar(3.0);
assert_eq!(x.shape(), &[]);
assert_eq!(x.len(), 1);
```

Scalar tensors MUST participate in broadcasting:

```text
[] + [2, 3] -> [2, 3]
```

---

## 5. Functional Requirements

### 5.1 Tensor Construction

#### REQ-001: `Tensor::new`

```rust
pub fn new(data: Vec<f64>, shape: &[usize]) -> Tensor;
```

Phase 1 behavior:

- MUST validate `data.len() == product(shape)`;
- MUST panic with an actionable message if the data length does not match;
- MUST treat `shape == []` as scalar and therefore require `data.len() == 1`;
- MUST store data in row-major order.

Example panic shape:

```text
matten shape mismatch: data length 5 cannot fill shape [2, 3] because the shape requires 6 elements
```

#### REQ-002: `Tensor::from_vec`

```rust
pub fn from_vec(data: Vec<f64>) -> Tensor;
```

Behavior:

- MUST create a one-dimensional tensor with shape `[data.len()]`;
- MUST preserve input order;
- SHOULD be equivalent to `Tensor::new(data, &[len])`.

#### REQ-003: Fill Constructors

```rust
pub fn zeros(shape: &[usize]) -> Tensor;
pub fn ones(shape: &[usize]) -> Tensor;
pub fn full(shape: &[usize], value: f64) -> Tensor;
pub fn scalar(value: f64) -> Tensor;
```

Behavior:

- MUST validate shape;
- MUST allocate exactly `product(shape)` elements for Phase 1;
- `scalar(value)` MUST produce shape `[]`.

#### REQ-004: Range Constructors

The crate SHOULD provide at least one simple range constructor:

```rust
pub fn arange(start: f64, end: f64, step: f64) -> Tensor;
```

Behavior:

- MUST panic if `step == 0.0`;
- MUST produce a one-dimensional tensor;
- SHOULD follow Python-like half-open semantics where `end` is excluded.

### 5.2 Shape Inspection

#### REQ-010: Shape Accessors

```rust
pub fn shape(&self) -> &[usize];
pub fn ndim(&self) -> usize;
pub fn len(&self) -> usize;
pub fn is_scalar(&self) -> bool;
pub fn is_vector(&self) -> bool;
pub fn is_matrix(&self) -> bool;
```

Behavior:

- `shape()` MUST return dimensions without allowing mutation;
- `ndim()` MUST return `shape().len()`;
- `len()` MUST return logical element count;
- `is_scalar()` / `is_vector()` / `is_matrix()` MUST be cheap structural predicates.

`is_empty()` is not part of the v0.19.0 public contract because zero-sized
tensors are rejected. If zero-sized dimensions are accepted by a future RFC,
`is_empty()` may be reconsidered then.

### 5.3 Reshape and Axis Operations

#### REQ-020: Reshape

```rust
pub fn reshape(&self, new_shape: &[usize]) -> Tensor;
```

Behavior:

- MUST validate that `product(new_shape) == self.len()`;
- MUST return a new `Tensor`;
- Phase 1 MUST NOT expose memory-contiguity restrictions to the user;
- Phase 1 MAY clone data directly;
- Phase 2 MAY share storage and update metadata;
- MUST panic on invalid shape with an actionable message.

#### REQ-021: Flatten

```rust
pub fn flatten(&self) -> Tensor;
```

Behavior:

- MUST return shape `[self.len()]`;
- MUST preserve row-major logical order.

#### REQ-022: Transpose

```rust
pub fn transpose(&self) -> Tensor;
pub fn t(&self) -> Tensor;
```

Behavior:

- For rank 2 tensors, MUST swap axes `[rows, cols] -> [cols, rows]` and reorder data logically.
- For rank greater than 2, `transpose()` SHOULD reverse axis order by default unless later RFC defines a NumPy-compatible axis parameter.
- `t()` SHOULD be an alias for `transpose()`.
- MUST panic if transpose is not meaningful for the current rank only if no well-defined behavior is adopted. Rank 0 and rank 1 MAY return cloned self.

#### REQ-023: Swap Axes

```rust
pub fn swap_axes(&self, axis_a: usize, axis_b: usize) -> Tensor;
```

Behavior:

- MUST validate axis bounds;
- MUST return a new tensor with axes swapped;
- MUST panic on invalid axis with a message including rank and requested axes.

### 5.4 Indexing and Element Access

#### REQ-030: Flat Access

```rust
pub fn get_flat(&self, index: usize) -> Option<f64>;
pub fn set_flat(&mut self, index: usize, value: f64);
```

Behavior:

- `get_flat` MUST return `None` when out of bounds;
- `set_flat` MAY panic when out of bounds, but the message MUST include index and length;
- Phase 2 dynamic equivalents SHOULD return or accept `Element` values.

#### REQ-031: Multidimensional Access

```rust
pub fn get(&self, indices: &[usize]) -> Option<f64>;
pub fn set(&mut self, indices: &[usize], value: f64);
```

Behavior:

- MUST validate rank: `indices.len() == self.ndim()`;
- MUST compute row-major offset;
- `get` MUST return `None` on invalid index;
- `set` MAY panic on invalid index for internal DX consistency.

### 5.5 Slicing

#### REQ-040: Builder-Based Slicing

Builder-based slicing is the primary structured API.

Illustrative API:

```rust
let subset = tensor
    .slice()
    .range(0..2)
    .all()
    .build()?;
```

Requirements:

- MUST support all elements for an axis;
- MUST support inclusive-start/exclusive-end ranges;
- SHOULD support stepped ranges;
- MUST return `Result<Tensor, MattenError>` from `build()` because slicing can be driven by user input;
- Phase 1 MUST return an owned tensor with copied data;
- Phase 2 MAY return a view-like tensor backed by shared storage.

#### REQ-041: String-Based Slicing

```rust
pub fn slice_str(&self, spec: &str) -> Result<Tensor, MattenError>;
```

String slicing is a convenience API, not the only slicing API.

Minimum grammar:

| Syntax | Meaning | Example |
|---|---|---|
| `:` | all elements in axis | `:, 0` |
| `n` | one index | `0, :` |
| `start:end` | half-open range | `0:2, :` |
| `start:` | start to axis end | `1:, :` |
| `:end` | axis start to end | `:2, :` |
| `start:end:step` | stepped range | `0:10:2` |

Requirements:

- MUST return `Result<Tensor, MattenError>` for parse errors and range errors;
- MUST include the original slice string in parse-error messages;
- SHOULD ignore surrounding whitespace;
- MUST NOT use `panic!` for malformed slice strings.

### 5.6 Broadcasting

#### REQ-050: Broadcast Compatibility

Broadcasting MUST follow NumPy-style right-aligned semantics.

Two shapes are compatible when, for every aligned dimension from the end:

- dimensions are equal; or
- one dimension is `1`; or
- one side has no dimension because it is lower rank.

Examples:

| Left | Right | Result | Compatible |
|---|---|---|---|
| `[3, 4]` | `[3, 4]` | `[3, 4]` | yes |
| `[3, 4]` | `[4]` | `[3, 4]` | yes |
| `[3, 1]` | `[1, 4]` | `[3, 4]` | yes |
| `[]` | `[2, 3]` | `[2, 3]` | yes |
| `[2, 3]` | `[2]` | none | no |

#### REQ-051: Broadcast Failure

Element-wise internal operations MAY panic on incompatible shapes. The panic MUST include both shapes and the operation name.

Example:

```text
matten broadcast error in add: shapes [2, 3] and [2] are not compatible
```

### 5.7 Math Operators

#### REQ-060: Element-Wise Tensor Operators

`matten` MUST implement standard Rust operator traits for borrowed tensors:

```rust
impl std::ops::Add for &Tensor;
impl std::ops::Sub for &Tensor;
impl std::ops::Mul for &Tensor;
impl std::ops::Div for &Tensor;
impl std::ops::Neg for &Tensor;
```

Behavior:

- operators MUST apply element-wise semantics;
- operators MUST support broadcasting;
- operators MUST allocate a new result tensor;
- operators MUST NOT mutate operands;
- division by zero follows IEEE 754 `f64` behavior in Phase 1.

#### REQ-061: Scalar Operators

`matten` SHOULD support scalar operations with `f64` in both owned and borrowed forms where practical:

```rust
&tensor + 1.0
&tensor - 1.0
&tensor * 2.0
&tensor / 2.0
```

Reverse scalar operations such as `1.0 + &tensor` MAY be limited by Rust orphan rules and SHOULD be provided only where legally implementable.

#### REQ-062: Matrix Multiplication

`matten` SHOULD provide explicit matrix multiplication rather than overloading `*` for matrix product.

```rust
pub fn matmul(&self, rhs: &Tensor) -> Tensor;
pub fn dot(&self, rhs: &Tensor) -> Tensor;
```

Minimum Phase 1 behavior:

- vector dot: `[n] x [n] -> []`;
- matrix-vector: `[m, n] x [n] -> [m]`;
- matrix-matrix: `[m, n] x [n, p] -> [m, p]`;
- invalid shapes MAY panic with actionable messages.

Rationale: `*` remains element-wise, matching NumPy array behavior and avoiding ambiguity for beginners.

### 5.8 Reductions and Basic Statistics

#### REQ-070: Whole-Tensor Reductions

```rust
pub fn sum(&self) -> f64;
pub fn mean(&self) -> f64;
pub fn min(&self) -> f64;
pub fn max(&self) -> f64;
```

Behavior:

- MUST operate over all elements in Phase 1;
- `mean` MUST panic or return `NaN` for empty tensors if zero-sized tensors are later supported; behavior MUST be documented;
- `min` and `max` MUST document behavior around `NaN`.

#### REQ-071: Axis Reductions

Axis reductions SHOULD be included after basic whole-tensor reductions:

```rust
pub fn sum_axis(&self, axis: usize) -> Tensor;
pub fn mean_axis(&self, axis: usize) -> Tensor;
```

Behavior:

- MUST validate axis bounds;
- MUST remove the reduced axis from the resulting shape unless a later RFC introduces `keepdims`.

### 5.9 Formatting and Debuggability

#### REQ-080: Debug Output

`Debug` output MUST include shape and data information.

For small tensors, output SHOULD be readable enough for terminal debugging.

Example:

```text
Tensor(shape=[2, 2], data=[[1.0, 2.0], [3.0, 4.0]])
```

Large tensors SHOULD be abbreviated.

#### REQ-081: Display Output

`Display` MAY be implemented, but if implemented it SHOULD prioritize readability over full fidelity.

### 5.10 Data Integration and Boundary APIs

#### REQ-090: Serde Representation

With the `serde` feature enabled, `Tensor` MUST support serialization and deserialization.

Recommended external representation:

```json
{
  "shape": [2, 2],
  "data": [1.0, 2.0, 3.0, 4.0]
}
```

Requirements:

- serialized data MUST be row-major;
- deserialization MUST validate shape/data consistency;
- malformed tensor JSON MUST return a deserialization error, not panic;
- dynamic mode MUST preserve `Element` variants through JSON where practical.

#### REQ-091: JSON Ingestion Convenience

```rust
pub fn from_json(input: &str) -> Result<Tensor, MattenError>;
pub fn to_json(&self) -> Result<String, MattenError>;
```

Behavior:

- MUST be boundary-safe and return `Result`;
- SHOULD accept nested JSON arrays such as `[[1.0, 2.0], [3.0, 4.0]]` in addition to the canonical `{shape, data}` form;
- Phase 1 MUST reject strings, booleans, and null values with a clear error;
- Dynamic mode maps mixed JSON values through `Tensor::from_json_dynamic`, not through the numeric-only `from_json`.

#### REQ-092: CSV Ingestion

```rust
pub fn from_csv(input: &str) -> Result<Tensor, MattenError>;
// no v0.19.0 public from_csv_with_headers API
```

Phase 1 behavior:

- MUST parse numeric CSV cells as `f64`;
- MUST reject non-numeric cells with row/column context;
- MUST require rectangular rows;
- MUST return shape `[rows, columns]`.

Phase 2 behavior:

- SHOULD infer `Int`, `Float`, `Bool`, `Text`, or `None` per cell;
- SHOULD preserve missing cells as `Element::None`.

#### REQ-093: File Loading

```rust
pub fn load_json(path: impl AsRef<std::path::Path>) -> Result<Tensor, MattenError>;
pub fn load_csv(path: impl AsRef<std::path::Path>) -> Result<Tensor, MattenError>;
```

Behavior:

- MUST return `Result` for I/O errors;
- MUST preserve source path information in errors when possible;
- MUST NOT panic for missing files, permission errors, malformed input, or invalid encoding.

### 5.11 Conversion Traits

#### REQ-100: Phase 1 Conversions

The following conversions MUST be implemented:

```rust
impl From<Vec<f64>> for Tensor;
impl From<Vec<Vec<f64>>> for Tensor;
impl From<Tensor> for Vec<f64>;
```

The following SHOULD be implemented if ergonomic and unambiguous:

```rust
impl TryFrom<Tensor> for Vec<Vec<f64>>;
```

Behavior:

- `From<Vec<f64>>` MUST create shape `[n]`;
- `From<Vec<Vec<f64>>>` MUST require rectangular rows and panic if rows are ragged;
- `TryFrom<Tensor> for Vec<Vec<f64>>` MUST return an error if the tensor is not rank 2.

#### REQ-101: Phase 2 Conversions

With `dynamic` enabled, conversions involving `Element` SHOULD be provided:

```rust
#[cfg(feature = "dynamic")]
impl From<Vec<Element>> for Tensor;

#[cfg(feature = "dynamic")]
impl From<Vec<Vec<Element>>> for Tensor;
```

Numeric-only conversion APIs MUST remain available.

---

## 6. Error Model

### 6.1 Boundary Error Rule

The most important error policy is:

> Internal mathematical operations may panic to optimize prototype writing speed, but all I/O and text-parsing boundaries MUST return `Result<Tensor, MattenError>`.

Boundary APIs include:

- JSON parsing;
- CSV parsing;
- file loading;
- string-based slicing;
- deserialization;
- conversion from ambiguous external data.

Internal operations that MAY panic include:

- incompatible tensor arithmetic;
- invalid reshape dimensions;
- invalid axis in axis operations;
- out-of-bounds mutation APIs;
- matrix multiplication shape mismatch.

### 6.2 `MattenError`

As-built v0.19.0 `matten` defines this public error type:

```rust
#[derive(Debug)]
#[non_exhaustive]
pub enum MattenError {
    Shape {
        operation: &'static str,
        message: String,
    },
    Broadcast {
        left: Vec<usize>,
        right: Vec<usize>,
    },
    Allocation {
        requested_elements: usize,
        message: String,
    },
    Slice {
        input: Option<String>,
        message: String,
    },
    Parse {
        format: DataFormat,
        message: String,
    },
    Io {
        path: std::path::PathBuf,
        source: std::io::Error,
    },
    Unsupported {
        operation: &'static str,
        message: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DataFormat {
    Json,
    Csv,
}
```

Requirements:

- `MattenError` MUST implement `Debug`, `Display`, and `std::error::Error`;
- it MUST NOT derive `Clone`, `PartialEq`, or `Eq` while it embeds `std::io::Error`;
- tests and users SHOULD match by variant (`matches!`) rather than equality;
- companion crates MUST define their own error types and MAY wrap `MattenError`;
- core `MattenError` MUST NOT grow companion-specific variants.

### 6.3 Error Message Quality

Every panic or error message SHOULD answer:

1. what operation failed;
2. what input was received;
3. what was expected;
4. how the user can fix it.

Bad:

```text
shape mismatch
```

Good:

```text
matten reshape error: cannot reshape shape [2, 3] with 6 elements into [4, 2], which requires 8 elements
```

---

## 7. Phase 2 Dynamic Data Requirements

> **As-built (RFC-016/017).** The `Element` model, missing-value utilities
> (`fill_none`, `forward_fill_none`, `none_mask`/`numeric_mask`), mixed JSON/CSV
> ingestion, and the explicit `try_numeric()` numeric on-ramp shipped. The
> **`Arc`/CoW storage model and in-place dynamic arithmetic did not ship**; dynamic
> tensors are an ingestion/cleanup surface, and computation happens after
> conversion to a numeric `Tensor`. Coercion is governed by an explicit
> `NumericPolicy` (RFC-017), exposed at the crate root under `dynamic`. Treat the
> arithmetic/CoW requirements in this section as deferred unless a future RFC
> revives them.

### 7.1 `Element` Semantics

```rust
#[cfg(feature = "dynamic")]
#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    Float(f64),
    Int(i64),
    Text(std::sync::Arc<str>),
    Bool(bool),
    None,
}
```

Requirements:

- `Element::None` represents missing data, not numeric zero;
- `Float` and `Int` are numeric-convertible under strict `NumericPolicy`;
- `Text` and `Bool` are not numeric by default;
- `try_as_f64()` returns `Option<f64>`;
- `Element::text(...)` is the constructor for text values;
- display/debug output MUST distinguish `None` from string `"None"`.

### 7.2 Dynamic Arithmetic

Dynamic arithmetic is **not** part of the v0.19.0 contract. Dynamic tensors are
for ingestion, inspection, cleanup, and explicit numeric conversion. Ordinary
math happens after:

```text
dynamic Tensor -> fill / inspect / clean -> try_numeric() -> numeric Tensor
```

Numeric APIs that receive dynamic tensors MUST reject them clearly instead of
performing silent mixed-type computation. Result-zone APIs return
`MattenError::Unsupported`; panic-zone convenience APIs may panic with an
actionable unsupported-operation message.

Permitted conversion is governed by `NumericPolicy`:

- strict/default: `Float` and `Int` accepted; `Bool`, `Text`, and `None` rejected;
- explicit policy methods such as `allow_bool()`, `allow_text_parse()`,
  `none_as(value)`, and `none_as_nan()` may widen conversion.

### 7.3 Missing Value Utilities

Phase 2 as-built provides the following public cleanup/inspection shape:

```rust
pub fn fill_none(&self, value: impl Into<Element>) -> Tensor;
pub fn forward_fill_none(&self, fallback: impl Into<Element>) -> Tensor;
pub fn none_mask(&self) -> Tensor;
pub fn numeric_mask(&self) -> Tensor;
pub fn is_none_mask(&self) -> Tensor;
pub fn count_none(&self) -> usize;
pub fn is_numeric_convertible(&self) -> bool;
pub fn schema_summary(&self) -> String;
pub fn try_numeric(&self) -> Result<Tensor, MattenError>;
pub fn try_numeric_with(&self, policy: NumericPolicy) -> Result<Tensor, MattenError>;
```

`drop_none_rows` and fallible no-fallback forward-fill are not part of the
v0.19.0 public contract. They require a future RFC if desired.

### 7.4 Dynamic JSON Behavior

| JSON value | Phase 1 behavior | Phase 2 behavior |
|---|---|---|
| number with decimal | `f64` | `Element::Float` |
| integer | converted to `f64` | `Element::Int` when safely representable |
| string | error | `Element::Text` |
| boolean | error | `Element::Bool` |
| null | error | `Element::None` |
| nested arrays | tensor shape inference | tensor shape inference |
| ragged arrays | error unless explicit padding API is used | SHOULD support explicit padding with `None` only through a named API |

Ragged JSON arrays MUST NOT be silently padded by default. Silent padding can hide data quality problems. A separate method such as `from_json_ragged_padded` MAY be added later.

---

## 8. API Ergonomics Requirements

### 8.1 Beginner Import Rule

The default tutorial MUST begin with:

```rust
use matten::Tensor;
```

No beginner tutorial example should require importing private modules, traits, or storage types.

### 8.2 No Generic Tensor in Phase 1

The public Phase 1 tensor type MUST NOT be written as `Tensor<T>`.

Rationale:

- reduces compiler diagnostics complexity;
- avoids forcing users to choose numeric types too early;
- keeps examples visually close to Python/NumPy;
- supports the “start now, optimize later” product strategy.

### 8.3 No Public View Lifetimes in Phase 1

The public Phase 1 API MUST NOT expose borrowed tensor view types such as `TensorView<'a>` in core tutorials.

Internal code MAY use helper structs, but user workflows should remain owned and lifetime-free.

### 8.4 Discoverability

Method names SHOULD be familiar to NumPy/Pandas users where reasonable:

- `reshape`
- `transpose`
- `flatten`
- `shape`
- `sum`
- `mean`
- `slice_str`
- `zeros`
- `ones`
- `full`
- `arange`

Rust-specific names MAY be used where they are clearer or safer.

---

## 9. Non-Functional Requirements

### 9.1 Compilation Speed

| ID | Requirement | Target |
|---|---|---|
| NFR-001 | Minimal examples SHOULD compile quickly in fresh debug builds | under 15 seconds on a modern developer laptop |
| NFR-002 | Incremental rebuilds after editing core modules SHOULD remain fast | under 3 seconds for common changes |
| NFR-003 | Public API MUST avoid deep trait-recursion patterns | no type-level shape arithmetic in Phase 1 |
| NFR-004 | Public API MUST avoid proc-macro dependency for normal use | no required public derive/proc macro for user tensor operations |

### 9.2 Memory Behavior

#### Phase 1

Phase 1 accepts higher allocation overhead in exchange for simplicity.

| Scenario | Expected Behavior |
|---|---|
| Small tensors under 1 MiB | cloning overhead is acceptable |
| Medium tensors around 1–100 MiB | documentation warns about copy-heavy behavior |
| Large tensors over 100 MiB | docs recommend Phase 2 or specialized crates |

Phase 1 SHOULD avoid unnecessary extra copies inside a single operation, but it does not need to implement view sharing.

#### Phase 2

Phase 2 SHOULD reduce memory overhead for large, messy data by using shared storage and copy-on-write behavior.

Requirements:

- slicing large dynamic tensors SHOULD avoid immediate full data duplication;
- reshape SHOULD be metadata-only when possible;
- mutation MUST not modify other tensors that share storage;
- no reference cycles are allowed in tensor storage.

### 9.3 Runtime Performance

`matten` SHOULD be reasonably efficient for small and medium PoC datasets, but runtime performance is secondary to API simplicity.

Performance requirements:

- algorithms SHOULD be straightforward and predictable;
- no intentionally exponential behavior in normal tensor operations;
- broadcasting SHOULD avoid constructing unnecessary expanded intermediate tensors when implementation complexity remains reasonable;
- documentation MUST state that highly optimized numerical workloads should use specialized crates.

### 9.4 Safety and Rust Policy

| ID | Requirement |
|---|---|
| NFR-010 | The crate SHOULD use safe Rust only for Phase 1. |
| NFR-011 | Any future `unsafe` MUST be isolated, documented, and justified by RFC. |
| NFR-012 | Panic usage MUST be intentional and documented according to the Boundary Error Rule. |
| NFR-013 | Public APIs MUST not expose invalid internal states. |

### 9.5 Documentation

The crate MUST include:

- `README.md` with a 5-minute quickstart;
- API-level rustdoc examples for key constructors and operations;
- a “When not to use matten” section;
- examples comparing basic NumPy and `matten` workflows;
- feature-gated examples for `dynamic`, JSON, and CSV;
- a clear explanation of panic-vs-Result rules.

Doc tests SHOULD compile and pass in CI.

---

## 10. Acceptance Criteria for Phase 1 v0.1.0

> **Status: met.** All Phase 1 acceptance criteria below were satisfied at `0.1.0`
> and the core was subsequently hardened and stabilized through `0.16.0` (RFCs
> 001–021): public-API minimalism, the boundary-safety/threat model, resource
> limits, axis reductions, human-readable diagnostics, and the executable-example
> quality gate. The criteria are retained as the historical definition of "Phase 1
> done."

Phase 1 can be considered complete when all of the following are true:

1. A user can create tensors with `new`, `from_vec`, `zeros`, `ones`, `full`, and `scalar`.
2. Shape mismatches in constructors and reshapes produce clear panic messages.
3. `&Tensor + &Tensor`, `&Tensor - &Tensor`, `&Tensor * &Tensor`, and `&Tensor / &Tensor` work with broadcasting.
4. Scalar arithmetic with `f64` works in common borrowed forms.
5. `reshape`, `flatten`, `transpose`, and `swap_axes` work for documented ranks.
6. Builder slicing and `slice_str` exist, with string slicing returning `Result`.
7. JSON and CSV boundary functions return `Result<Tensor, MattenError>`.
8. `From<Vec<f64>>`, `From<Vec<Vec<f64>>>`, and `From<Tensor> for Vec<f64>` are implemented.
9. `Debug` output includes shape and readable data.
10. README quickstart compiles and runs.
11. Rustdoc examples compile.
12. No public API requires explicit lifetime annotations.
13. No public `Tensor<T>` generic is introduced.
14. The crate builds with default features and with `--no-default-features` if such support is committed.

---

## 11. Proposed API Examples

### 11.1 Default Numerical PoC

```rust
use matten::Tensor;

fn main() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let b = Tensor::ones(&[2, 2]);

    let c = &a + &b;
    let d = &c * 2.0;

    println!("a = {:?}", a);
    println!("d = {:?}", d);

    let flat = d.flatten();
    assert_eq!(flat.shape(), &[4]);
}
```

### 11.2 Broadcasting

```rust
use matten::Tensor;

fn main() {
    let matrix = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let bias = Tensor::new(vec![10.0, 20.0, 30.0], &[3]);

    let result = &matrix + &bias;
    assert_eq!(result.shape(), &[2, 3]);
}
```

### 11.3 Reshape, Transpose, and Matmul

```rust
use matten::Tensor;

fn main() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let b = a.transpose();

    assert_eq!(b.shape(), &[3, 2]);

    let x = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);
    let y = a.matmul(&x);

    assert_eq!(y.shape(), &[2]);
}
```

### 11.4 Boundary-Safe JSON and CSV

```rust
use matten::Tensor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"[[1.0, 2.0], [3.0, 4.0]]"#;
    let t = Tensor::from_json(json)?;
    assert_eq!(t.shape(), &[2, 2]);

    let csv = "1.0,2.0\n3.0,4.0\n";
    let c = Tensor::from_csv(csv)?;
    assert_eq!(c.shape(), &[2, 2]);

    Ok(())
}
```

### 11.5 Dynamic Data Example

```rust
#[cfg(feature = "dynamic")]
use matten::{Element, Tensor};

#[cfg(feature = "dynamic")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"
    [
        [1, "active", true],
        [2, null, false],
        [3, "pending", true]
    ]
    "#;

    let data = Tensor::from_json_dynamic(json)?;
    assert_eq!(data.shape(), &[3, 3]);

    let cleaned = data.fill_none(Element::text("unknown"));
    let status = cleaned.slice_str(":, 1")?;

    println!("status = {:?}", status);
    Ok(())
}
```

---

## 12. Compatibility and Migration Guidance

### 12.1 Relationship to `ndarray`

> **As-built (RFC-022/025/027).** The kickoff plan for an in-core optional `ndarray`
> conversion feature was replaced by the standalone **`matten-ndarray`** companion
> crate, so core `matten` keeps **no** `ndarray` dependency. Use
> `matten_ndarray::{to_arrayd, from_arrayd}` for `Tensor` ↔ `ndarray::ArrayD<f64>`.
> The `into()`-to-`Vec<f64>` migration path below still holds.

`matten` SHOULD document migration paths to `ndarray` for users who outgrow Phase 1 performance. As of v0.19.0, that migration path is the separate `matten-ndarray` companion crate, not an in-core optional feature. Future in-core bridge features are prohibited unless a later RFC explicitly reverses RFC-022/RFC-025.

### 12.2 Relationship to `nalgebra`

`nalgebra` is specialized for linear algebra with strong static and dynamic matrix types. `matten` SHOULD not imitate `nalgebra` type-level semantics. If interoperability is later added, it should be explicit and feature-gated.

### 12.3 Relationship to `candle`

`candle` is a machine-learning tensor library. `matten` is not an ML framework. Any future `candle` conversion should be positioned as an integration convenience, not as a core purpose.

---

## 13. Open Questions for Next Design Stage

> **Resolved.** These kickoff open questions were settled during implementation:
> (1) zero-sized dimensions are **rejected** (deferred to a future RFC if ever needed);
> (2) defaults are `["serde", "json", "csv"]`, with `default-features = false` for the lean core;
> (3) `Tensor::random` was **not** added (no `rand` dependency);
> (4) `slice_str` does **not** support negative indices yet;
> (5) rank is capped at `MAX_NDIM = 8` via `MattenLimits` (RFC-018);
> (6) `MattenError` is `Shape`/`Broadcast`/`Allocation`/`Slice`/`Parse`/`Io`/`Unsupported` (RFC-005);
> (7) `TryFrom<Tensor> for Vec<Vec<f64>>` consumes;
> (8) `dynamic` lives in the same crate, feature-gated, introduced after Phase 1 stabilized;
> (9) interop is via **separate companion crates** (`matten-ndarray` shipped; `nalgebra`/`candle` deferred, RFC-025), not in-core optional features.

The following should be resolved in external design or RFCs:

1. Should zero-sized dimensions be supported in v0.1.0, or deferred?
2. Should `serde`, `json`, and `csv` all be default features, or should only `serde` be default?
3. Should `Tensor::random` be included in v0.1.0, and if so, should it require a `rand` feature?
4. Should `slice_str` use Python-like negative indices in the first release?
5. Should rank be capped at 8, or should arbitrary rank be allowed with runtime vectors?
6. What is the exact public shape of `MattenError`?
7. Should `TryFrom<Tensor> for Vec<Vec<f64>>` consume or borrow the tensor?
8. Should Phase 2 dynamic mode be in the same crate from the beginning or introduced after Phase 1 stabilizes?
9. Should there be optional conversions to `ndarray`, `nalgebra`, or `candle` in early releases?

---

## 14. Recommended RFC Breakdown After Requirements Approval

The following RFCs are recommended before implementation begins:

| RFC | Theme | Purpose |
|---|---|---|
| RFC-001 | Core Tensor Data Model | Shape, scalar semantics, row-major layout, validation policy |
| RFC-002 | Error and Panic Policy | Boundary Error Rule, `MattenError`, panic message standards |
| RFC-003 | Construction and Conversion API | Constructors, `From`/`TryFrom`, shape inference |
| RFC-004 | Broadcasting and Element-Wise Operators | NumPy-like broadcasting, scalar ops, operator trait scope |
| RFC-005 | Reshape, Transpose, and Indexing | Axis operations, flat/multidimensional access |
| RFC-006 | Slicing API | Builder slicing, string grammar, errors |
| RFC-007 | JSON/CSV Integration | Serde representation, nested array support, CSV parse policy |
| RFC-008 | Basic Reductions and Matmul | `sum`, `mean`, `matmul`, `dot`, NaN policy |
| RFC-009 | Dynamic Element Model | `Element`, missing values, dynamic JSON/CSV behavior |
| RFC-010 | Dynamic Storage and CoW | `Arc` storage, view metadata, mutation isolation |

> **As-built RFC arc (000–030).** The kickoff list above was a forecast. The
> implementation produced a renumbered, broader set, all in `rfcs/done/`:
>
> - **000** RFC lifecycle policy.
> - **001–014** core: threat model, public-API minimalism, shape model, construction,
>   error model, broadcasting, reshape/axis ops, slicing, serde/JSON/CSV, reductions/
>   matmul, dynamic `Element`, dynamic storage, testing/release gates, examples.
> - **015–021** stabilization: API stabilization, dynamic ingestion on-ramp (016),
>   numeric coercion policy (017), resource limits (018), axis reductions (019),
>   diagnostics quality (020), tutorial/example gate (021).
> - **022–026** workspace & companions: boundary policy (022), `matten-data` scope (023),
>   `matten-mlprep` scope (024), bridge policy (025), large-CSV/streaming policy (026).
> - **027–030** companion implementations & policy: `matten-ndarray` design (027),
>   `matten-mlprep` design (028), maturity evaluation (029), lock-step family
>   versioning (030, superseding 022 §7).
>
> RFC-016 narrowed the dynamic engine; RFC-022/025 moved interop out of core into
> companion crates. See `rfcs/README.md` for the index.

---

## 15. Document History

| Version | Date | Change |
|---|---|---|
| 1.0.0 | 2026-06-18 | Initial comprehensive PRD/SRS generated from project instructions, draft requirements, and prototype notes |
| 2.0.0 | 2026-06-21 | Reconciled with the as-built state through the `0.19.0` family release: added §0.1 as-built status (workspace + companion crates, lock-step versioning, consolidated error surface, resource limits); annotated Phase 2 as a narrowed ingestion/on-ramp engine (RFC-016); marked Phase 1 acceptance criteria met; pointed `ndarray` interop to the `matten-ndarray` companion; resolved the kickoff open questions; recorded the actual RFC arc (000–030). Canonical forward planning moved to `ROADMAP.md`. |
| 2.1.0 | 2026-06-21 | Review correction pass: removed remaining normative conflicts in Phase 2 / feature / error sections, recorded exact shipped `MattenError` fields, updated dynamic `Element::Text` to `Arc<str>`, aligned feature defaults to v0.19.0, and clarified companion dynamic rejection as a contract. |
