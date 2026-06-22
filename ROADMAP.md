# `matten` ROADMAP

**Project:** `matten`  
**Document Kind:** Canonical Project Roadmap  
**Document Version:** `1.2.0`  
**Date:** 2026-06-22  
**Status:** Canonical roadmap updated for v0.20+ materialization planning; RFC-032 is reserved/consumed by another issue, so the v0.20+ RFC sequence starts at RFC-033  
**Planning Baseline:** core `matten` completed RFC-015 through RFC-021 (shipped through v0.15.3); RFC-022 boundary confirmation shipped in v0.16.0; v0.17.0 introduced the Cargo workspace and the `matten-ndarray` companion crate under the family version (RFC-025, RFC-027); v0.18.0 introduced the `matten-mlprep` companion crate under the family version (RFC-024, RFC-028); v0.19.0 promoted `matten-ndarray` to production-ready candidate status and `matten-mlprep` to beta status under lock-step family versioning (RFC-029); v0.19.1 shipped feature-robust dynamic rejection (RFC-031); v0.19.2 confirmed the companion dependency/import convention (RFC-032). Under lock-step family versioning (RFC-030), every crate shares the family version (e.g. `0.19.2`); maturity is expressed by per-crate Status labels, not by separate version numbers. Next: v0.20+ materialization phase. RFC-032 is consumed by the companion dependency/import convention; v0.20+ planning starts at RFC-033. The first v0.20+ branch is `matten-data` decision/materialization; the second is small NumPy-inspired core comfort APIs that preserve the `matten` philosophy.

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

**RFC numbering note:** RFC-032 is reserved/consumed by another issue. New v0.20+ roadmap RFCs therefore begin at **RFC-033**.

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
| **v0.20.0** | v0.20+ design/materialization start | RFC-033 through RFC-037 for `matten-data`; RFC-038 for core comfort APIs | Design + selective implementation approval |
| **v0.20.x** | Minimal implementation phase | Small core comfort APIs; optional experimental `matten-data` if approved | Low-risk implementation only |
| **v0.21+** | Selective production readiness | `matten-data` beta/experimental/freeze decision; companion maturity decisions | Per-crate decisions |
| **Later** | Streaming / large CSV, `nalgebra`, `candle`, stats/linalg companions | Separate RFCs required | Design-only until reopened |

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

v0.20+ has two parallel tracks:

```text
Track A: matten-data decision/materialization
  Decide whether a small table-to-Tensor companion is worth building.

Track B: core numeric comfort APIs
  Add small NumPy-inspired Tensor conveniences only if they preserve the Sedan-first philosophy.
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
| RFC-038 | Core Numeric Comfort APIs | v0.20.x / v0.21 |
| RFC-039 | Shape Composition API Boundary | v0.21+ |
| RFC-040 | Small Statistics Boundary: Core vs Companion | v0.21+ |
| RFC-041 | Linear Algebra Boundary: Core Lite vs External Crates | v0.21+ |
| RFC-042 | Pandas-Inspired Scope Guard for `matten-data` | v0.21+ if needed |

RFC-042 may be folded into RFC-033 if the scope guard is already strong enough.

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

### 9.4 What v0.20+ must not do

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
