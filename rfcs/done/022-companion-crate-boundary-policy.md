# RFC-022: Companion Crate Boundary Policy

**Status:** Implemented (v0.16.0)  
**Target:** v0.16.0  
**Theme:** Companion boundary confirmation  
**Depends on:** RFC-015, RFC-016, RFC-021  
**Supersedes:** older v0.16+ planning lines that treated v0.16 as design-only without mechanical gates

---

## 1. Summary

This RFC defines the enforceable boundary between core `matten` and future `matten-*` companion crates.

Core `matten` remains Sedan-first: a simple, DX-oriented numeric `Tensor` crate with an optional dynamic ingestion/cleanup/on-ramp feature. Companion crates may add table preparation, preprocessing, bridge conversion, or streaming workflows, but those workflows must remain optional and must not add dependencies to core `matten`.

v0.16.0 is therefore a boundary-confirmation release. It should implement policy, documentation, workspace structure, and CI checks before new companion implementation proceeds.

---

## 2. Goals

- Protect core `matten` from dependency growth and scope drift.
- Establish companion-crate naming, versioning, and release policy.
- Define independent per-crate maturity labels.
- Define companion error-type policy.
- Make the dependency boundary mechanically testable in CI.
- Clarify that v0.17 starts with `matten-ndarray`, not `matten-data`.
- Keep `matten-data`, `matten-mlprep`, bridge crates, and streaming outside core.

---

## 3. Non-goals

- This RFC does not implement `matten-ndarray`, `matten-data`, or `matten-mlprep`.
- This RFC does not add ndarray/nalgebra/candle dependencies to core.
- This RFC does not add dataframe, ML, or streaming APIs to core `Tensor`.
- This RFC does not define any v1.0 release.

---

## 4. Core boundary rule

A feature may belong in core `matten` only if it directly helps users construct, inspect, clean, validate, or explicitly convert a `Tensor` while keeping the primary `Tensor` API simple.

A feature belongs in a companion crate if it introduces:

- external framework dependencies;
- table/dataframe semantics;
- ML preprocessing semantics;
- bridge conversion to another crate;
- streaming/batch lifecycle;
- domain-specific workflows.

---

## 5. Dependency direction

Allowed:

```text
matten-ndarray -> matten
matten-mlprep  -> matten
matten-data    -> matten
matten-stream  -> matten
```

Forbidden:

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

Core `matten` must not expose optional features such as `ndarray`, `nalgebra`, `candle`, `polars`, or `dataframe`.

---

## 6. Workspace and crate layout

A future workspace may look like:

```text
crates/
  matten/
  matten-ndarray/
  matten-mlprep/
  matten-data/
```

But the layout alone does not imply lock-step releases or equal maturity. Each crate must publish independently when appropriate.

---

## 7. Versioning model

Each crate uses **independent per-crate SemVer**.

Examples:

```text
matten          0.16.0
matten-ndarray 0.1.0
matten-mlprep  0.1.0
matten-data    0.1.0 experimental
```

Rules:

- Core `matten` version does not imply companion maturity.
- Workspace release notes may coordinate multiple crates.
- Companion crates may reach beta/production-ready at different times.
- v1.0 for any crate requires explicit maintainer confirmation.

Lock-step workspace versioning is rejected because it would create false maturity signals.

---

## 8. Companion error-type policy

Each companion crate must define its own error type.

Examples:

```rust
pub enum MattenNdarrayError {
    DynamicTensor,
    Shape(String),
    NdarrayShape(ndarray::ShapeError),
    Matten(matten::MattenError),
}
```

Rules:

- Core `MattenError` is for core tensor and boundary failures.
- Companion crates may wrap `MattenError`.
- Core must not grow variants for companion-specific failure modes.
- Bridge/conversion functions return `Result`.
- Dynamic inputs to companion APIs should return `Err`, not panic, unless a method is explicitly documented as panic-zone convenience.

---

## 9. Maturity labels

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

Team believes the crate can be used seriously if documented limits are acceptable.

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
- release notes;
- no hidden dependency surprises.

---

## 10. Mechanical dependency-boundary CI

v0.16.0 must add a CI-enforced dependency-boundary check.

A script such as this must be present:

```text
scripts/check-core-dependency-boundary.sh
```

It should fail if package `matten` depends on any forbidden crate:

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

The check MUST inspect the core package with **all features enabled**, so that an
optional dependency behind a non-default feature (e.g. `ndarray = { optional = true }`
gated by a `ndarray` feature) cannot slip past a default-feature `cargo tree`:

```bash
cargo tree -p "$CORE_PACKAGE" --all-features --edges normal,build --no-dedupe
```

`--all-features` closes the optional-dependency blind spot; `--edges normal,build`
restricts the boundary to the dependency graph that ships to downstream users
(dev-only dependencies are out of scope for this gate). The check should treat
“package not found” / “nothing depends on it” as success and an actual normal/build
dependency path as failure.

---

## 11. v0.16 acceptance criteria

- `ROADMAP.md` is updated and canonical.
- RFC-023 through RFC-026 target headers align with the v0.16+ roadmap.
- External design in-core bridge examples are marked superseded.
- Companion SemVer policy is documented.
- Companion error policy is documented.
- Maturity labels are documented.
- Dependency-boundary CI exists.
- Core `matten` has no companion or bridge dependencies.

---

## 12. Open questions (resolved / deferred at implementation)

RFC-022 ships its policy, documentation, and CI gate in v0.16.0. The following
were resolved or explicitly deferred at that point rather than blocking the move
to `done/` (see RFC-000 § "Granularity of transitions"):

1. **Workspace restructuring timing — deferred to v0.17.0.** v0.16.0 adds only
   the policy, the dependency-boundary CI, and the doc reconciliation; core
   `matten` remains a single crate. The `crates/matten/...` workspace layout is
   introduced when the first companion crate (`matten-ndarray`) lands, so the
   restructuring and its first real consumer arrive together.
2. **Per-crate implementation RFC numbering — resolved: start at RFC-027.**
   Numbers are sequential and never reused (RFC-000), so the first companion
   implementation RFC is RFC-027.
3. **Changelog strategy — deferred to v0.17.0.** Decided alongside the workspace
   move; the working assumption is per-crate changelogs with coordinated
   workspace release notes, consistent with independent per-crate SemVer (§7).
