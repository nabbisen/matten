# `matten` ROADMAP

**Project:** `matten`  
**Document Kind:** Canonical Project Roadmap  
**Document Version:** `1.0.0`  
**Date:** 2026-06-21  
**Status:** First canonical issue — v0.16+ companion-crate reconciliation  
**Planning Baseline:** `matten` core has completed RFC-015 through RFC-021 implementation/hardening work (shipped through v0.15.3); v0.16+ starts companion-crate boundary work.

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
| **v0.20+** | `matten-data` beta decision | Decide whether small CSV/table-to-Tensor workflow deserves beta | Strategic gate, not automatic promotion |
| **v0.21+** | Selective production readiness | Promote only proven crates | Per-crate decisions |
| **Later** | Streaming / large CSV, `nalgebra`, `candle` | Separate RFCs required | Design-only until reopened |

---

## 4. v0.16.0 milestone: companion boundary confirmation

### Goal

Make the companion-crate model concrete without expanding core `matten`.

### Required work

- Implement RFC-022 as policy and project mechanics.
- Decide workspace layout.
- Define independent per-crate SemVer.
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

## 8. v0.20+ milestone: `matten-data` beta decision phase

### Goal

Decide whether `matten-data` deserves beta without becoming a dataframe engine.

`matten-data` may be scaffolded earlier, but it must not become the main v0.17 implementation target and must not be promoted before this decision gate.

### Required proof

The crate must prove this small workflow:

```text
CSV / table-like data
  -> inspect schema
  -> clean missing values
  -> select numeric columns
  -> explicit numeric conversion
  -> matten::Tensor
```

Possible beta API:

```rust
use matten_data::Table;

let table = Table::from_csv_path("sales.csv")?;
println!("{}", table.schema_summary());

let x = table
    .select_columns(["sales", "cost", "quantity"])?
    .fill_missing(0.0)?
    .to_tensor()?;
```

### Allowed beta scope

- CSV string/path ingestion;
- schema summary;
- column names;
- column selection;
- missing-value cleanup;
- explicit numeric conversion;
- Tensor output.

### Still forbidden

- joins;
- group-by;
- pivot;
- SQL-like query API;
- lazy execution;
- large-data streaming;
- window functions;
- dataframe-style indexing;
- ML preprocessing.

### Decision outcomes

At v0.20+, choose one:

```text
A) promote to beta
B) keep experimental
C) freeze/defer
```

Keeping it experimental is acceptable if the API is useful but not mature. Freezing is acceptable if the crate starts drifting into dataframe territory.

---

## 9. Later themes

### `matten-nalgebra`

Deferred until after `matten-ndarray` proves the bridge pattern. Requires a separate RFC or explicit reopening of RFC-025.

### `matten-candle`

Deferred longer because it brings device, dtype, ML, and dependency-tree complexity. Requires a separate RFC.

### Streaming / large CSV

Design-only until batch lifecycle, schema drift, malformed-row policy, memory budget, and sync-vs-async strategy are proven. May later live in `matten-data` or a separate `matten-stream`; undecided.

---

## 10. Companion crate versioning policy

Each companion crate uses **independent per-crate SemVer**.

A workspace may have coordinated release notes, but crate maturity is declared per crate.

Examples:

```text
matten          0.16.0
matten-ndarray 0.1.0
matten-mlprep  0.1.0
matten-data    0.1.0 experimental
```

Core `matten` version does not imply companion crate maturity.

---

## 11. Maturity labels

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

## 12. Companion error policy

Each companion crate defines its own error type.

Core `matten::MattenError` is for core tensor and boundary failures only. Companion crates may wrap `MattenError`, but core must not grow variants for companion-specific failure modes.

Bridge and conversion functions return `Result`:

```rust
to_arrayd(&tensor) -> Result<ArrayD<f64>, MattenNdarrayError>
```

Dynamic inputs to companion bridge/prep/data APIs should return `Err`, not panic, unless the API is explicitly documented as an internal panic-zone convenience.

---

## 13. Mechanical dependency-boundary gate

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
