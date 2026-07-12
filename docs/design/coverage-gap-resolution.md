# Docs-Governance Coverage-Gap Resolution

**Project:** `matten`
**Document kind:** Documentation-governance resolution note
**Status:** Prepared for review
**Source handoff:** `rfcs/handoffs/docs-governance-01-spec-coverage-gap-closure-handoff.md`
**Date:** 2026-07-12

This note resolves the three unowned fragments identified before archiving the v0.19.0
requirements/design snapshots. It records whether each fragment is part of the current contract,
where the current owner lives, and what remains future work.

This is documentation governance only. It adds no public API, dependency, version bump, release
scope, test gate, benchmark claim, or runtime behavior.

## 1. Performance Targets

Decision: **retire the numeric v0.19-era targets as maintained requirements**.

The old snapshot mentioned:

```text
minimal examples compile under 15 seconds
incremental rebuild under 3 seconds
memory tiers: under 1 MiB clone-ok, 1-100 MiB warn, above 100 MiB use specialised crates
```

These numbers are not enforced in CI, not maintained by current benchmark reports, and not part of
the release contract. Current benchmark documentation deliberately treats timing and memory data as
workload- and machine-specific evidence, not a promise.

Current owner:

```text
docs/src/benchmarks/methodology.md
docs/src/benchmarks/results.md
benchmarks/reports/*.md
rfcs/done/049-benchmarking-complexity-metrics-and-positioning-report.md
```

Policy outcome:

```text
matten keeps compile-time and memory footprint small as a design preference, but publishes no
numeric compile-time, rebuild-time, or memory-tier guarantee unless a future RFC or release-policy
decision adopts one.
```

## 2. Golden, Property, and Fuzz Testing

Decision: **record the real implemented coverage; keep property/fuzz as future hardening candidates**.

Observed current state:

```text
crates/matten/tests/golden/numpy_broadcasting.json
crates/matten/tests/golden/numpy_matmul.json
crates/matten/tests/smoke.rs
```

`crates/matten/tests/smoke.rs` contains NumPy golden tests for broadcasting and matrix
multiplication. The golden files are checked in as JSON and do not require Python at test time.

Property-based tests and a fuzz harness are not currently present as release gates. RFC-013 already
records that property tests and fuzzing remain future hardening candidates rather than current
release requirements. Current release confidence comes from unit/integration tests, checked-in
golden/reference fixtures, example smoke runs, feature-matrix sweeps, dependency-boundary guards,
and release-documentation guards.

Current owner:

```text
rfcs/done/013-testing-compatibility-and-release-gates.md
docs/src/contributing/development-process.md
crates/matten/tests/golden/
crates/matten/tests/smoke.rs
```

Future property/fuzz work should be introduced by a focused RFC or explicit hardening slice.

## 3. Tensor Display Formatting

Decision: **record `Display` for `Tensor` as not part of the current contract**.

Observed current state:

```text
crates/matten/src/tensor.rs
```

`Tensor` implements `Debug`, with compact shape/data output. `Tensor` does not implement
`std::fmt::Display`. Error types and some dynamic/supporting types implement `Display`, but
matrix-like `Tensor` display formatting is not a current user-facing API contract.

Current owner:

```text
docs/src/reference/compatibility.md
docs/src/reference/public-api-snapshot.md
crates/matten/src/tensor.rs
```

Any future `Tensor` display format should be designed separately because formatting becomes a
user-visible compatibility surface once documented.

## 4. Follow-On Sequence

With these decisions recorded, archival may proceed in the next docs-governance slice:

```text
1. coverage-gap closure: this note
2. spec archival and ownership rule: docs-governance-02
3. philosophy distillation: docs-governance-03, after tracked archives exist
```
