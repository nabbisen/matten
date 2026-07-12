# RFC-049 Lifecycle Closure Handoff

**Project:** `matten`
**Related RFC:** RFC-049: Benchmarking, Complexity Metrics, and Positioning Report
**Document kind:** Lifecycle closure / status-resolution handoff
**Status:** Accepted; lifecycle closure prepared for implementation review
**Scope:** RFC bookkeeping and documentation-truth alignment only

---

## 1. Summary

RFC-049 has delivered the reviewed benchmarking and positioning program through
Phase 3:

```text
Phase 1: internal baseline
Phase 2: Rust peer comparison
Phase 3: code-shape-first NumPy/Pandas reference comparison
```

Phase 4 is deliberately outside RFC-049's closed scope:

```text
hard regression thresholds / performance gates
```

This handoff proposes a lifecycle decision, not new benchmark code:

```text
Close RFC-049 as implemented for Phases 1-3.
Extract Phase 4 to a future separate RFC/release-policy decision.
Move RFC-049 from rfcs/proposed/ to rfcs/done/ only after review acceptance.
```

The goal is to remove the stale "open because Phase 4 exists" ambiguity while
preserving the prohibition on hard gates.

---

## 2. Pre-Closure State

Before this lifecycle-closure slice, RFC-049 remained in `rfcs/proposed/` with this status shape:

```text
Phase 1 implemented and accepted
Phase 2 implemented and accepted
Phase 3 implemented and accepted
Phase 4 deferred before closure
```

It also says:

```text
Per the 4-folder RFC lifecycle, this RFC stays in proposed/ until fully
implemented or explicitly split/resolved (Phase 4 remains).
```

The implemented evidence now exists:

```text
benchmarks/reports/internal-baseline-v0.1.md
benchmarks/reports/internal-baseline-v0.2.md
benchmarks/reports/peer-comparison-v0.1.md
benchmarks/reports/peer-comparison-v0.2.md
benchmarks/reports/python-reference-comparison-v0.1.md
docs/src/benchmarks/index.md
docs/src/benchmarks/methodology.md
docs/src/benchmarks/results.md
rfcs/handoffs/049-benchmarking-developer-handoff.md
rfcs/handoffs/049-phase-3-python-reference-comparison-handoff.md
```

---

## 3. Proposed Closure Boundary

If accepted, perform a docs/RFC bookkeeping slice only:

```text
rfcs/proposed/049-benchmarking-complexity-metrics-and-positioning-report.md
  -> rfcs/done/049-benchmarking-complexity-metrics-and-positioning-report.md
```

Update status/index wording to say:

```text
Implemented for Phases 1-3.
Phase 4 hard gates are outside RFC-049's closed scope and require a future
separate RFC or explicit release-policy decision.
```

Do not implement:

```text
hard speed gates
hard memory gates
new benchmark workloads
new Python/SciPy/Candle references
new CI benchmark execution
ordinary CI performance thresholds
published-crate dependency changes
runtime/API changes
```

---

## 4. Required Checks Before Closure

Before moving RFC-049 to `done/`, verify:

```text
[ ] rfcs/README.md moves row 049 to Done and rewrites it to drop "Phases 3-4 deferred"
[ ] row 049 says Phases 1-3 are implemented and Phase 4 belongs to future RFC/policy ownership
[ ] RFC-049 Status field matches its new `done/` location
[ ] RFC-049 no longer says it stays in `proposed/` because Phase 4 remains
[ ] Phase 4 remains explicitly unauthorized and extracted to future RFC/policy ownership
[ ] benchmark docs do not describe Phase 1, 2, or 3 as unimplemented
[ ] docs do not imply hard gates exist
[ ] reports retain non-ranking / no-marketing framing
[ ] benchmark report discoverability covers internal-baseline v0.1/v0.2, peer-comparison v0.1/v0.2,
    and python-reference-comparison v0.1
[ ] release-doc guard still passes
[ ] git diff --check passes
```

Recommended commands:

```text
bash scripts/check-release-docs.sh
git diff --check
```

No Rust build is required for a pure RFC/docs move unless implementation files
are touched.

---

## 5. Acceptance Criteria

The closure is accepted when:

```text
[ ] RFC-049 is no longer the only open proposed RFC solely because Phase 4 exists
[ ] future hard gates require a separate RFC or explicit release-policy decision
[ ] existing benchmark artifacts remain discoverable, including v0.2 internal/peer reports
[ ] no benchmark result is reframed as ranking/marketing proof
[ ] no source/runtime/API/dependency behavior changes
```

---

## 6. Still Deferred

Still deferred after closure:

```text
Phase 4 hard gates
regression threshold policy
SciPy reference comparisons
Candle reference comparisons
GPU / accelerator benchmarks
ordinary CI performance runs
published-crate dependency changes
```
