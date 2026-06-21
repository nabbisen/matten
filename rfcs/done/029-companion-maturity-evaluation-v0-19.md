# RFC-029: Companion Maturity Evaluation (v0.19.0)

**Status:** Implemented (v0.19.0)
**Target:** v0.19.0
**Theme:** Companion maturity hardening — promote only what stayed small
**Depends on:** RFC-022 (maturity labels), RFC-024, RFC-025, RFC-027, RFC-028

---

## 1. Summary

v0.19.0 is a hardening and decision milestone, not new-feature work. It evaluates
the two existing companion crates against their maturity gates (RFC-022 §9,
ROADMAP §7), applies the hardening needed to close gaps, and records the
resulting decisions:

- **`matten-ndarray` → production-ready candidate.**
- **`matten-mlprep` → beta.**

Both remain pre-1.0. Maturity labels are signaled through README text, badges,
and docs — not through the version number (RFC-022 §9). In the same release,
RFC-030 adopted **lock-step family versioning**, so both crates align to the
family version `0.19.0`; their maturity (production-ready candidate / beta) is
carried entirely by the Status label, not the number. No public API or behavior
changes in this release; core `matten` also aligns to `0.19.0`.

`matten-data`, `matten-nalgebra`, `matten-candle`, and streaming are **not**
promoted; they remain future work (RFC-023, RFC-025 §10, RFC-026).

---

## 2. `matten-ndarray` — production-ready candidate

### 2.1 Gate evaluation (ROADMAP §7 + RFC-022 §9)

| Criterion | Status |
|---|---|
| scalar/vector/matrix/N-D conversions work | Pass — tested for ranks 0–4 |
| roundtrip tests reliable | Pass — strengthened (rank-4, permuted 3-D, NaN/Inf, fractional fidelity) |
| dynamic tensors rejected clearly | Pass — `DynamicTensor`, no panic |
| copy behavior documented | Pass — README + rustdoc |
| no zero-copy promise unless implemented | Pass — explicitly disclaimed |
| examples run in CI | Pass — `to_arrayd`, `from_arrayd` |
| core `matten` has no `ndarray` dependency | Pass — boundary CI |
| clear error type | Pass — `MattenNdarrayError` (`source()` for wrapped) |
| documented compatibility policy | **Added in v0.19.0** (was the only gap) |
| no known P0/P1 issues | Pass |

### 2.2 Why it qualifies fast

The crate's scope is fully closed: two owned-conversion functions, no open design
questions, no view/lifetime surface. The only gate gap was a written
compatibility policy. With that added and the roundtrip suite strengthened, the
crate meets the production-ready-candidate bar.

### 2.3 Hardening applied

- Added a **Compatibility** section (SemVer for `0.x`, MSRV 1.85, supported
  `ndarray` = `0.16.x`, the `ndarray`-bump policy from RFC-025 §6).
- Strengthened tests: rank-4 roundtrip; a 3-D permuted-axes (non-standard-layout)
  roundtrip; `NaN`/`Inf` passthrough both directions; fractional-value fidelity;
  `to_arrayd` produces standard layout.
- README status: experimental → **production-ready candidate**.

---

## 3. `matten-mlprep` — beta

### 3.1 Gate evaluation (RFC-024 §8 + ROADMAP §7)

| Criterion | Status |
|---|---|
| API small and teachable | Pass — 4 pure functions |
| functions deterministic | Pass — no RNG / state / environment reads |
| shape rules documented | Pass — rank-2, rows=samples, cols=features |
| zero-variance behavior explicit | Pass — `ZeroVariance` error, not silent zeros |
| train/test split behavior explicit | Pass — ordered, documented formula |
| no ML-framework scope | Pass — no training/autograd/optimizer |
| core has no dependency on `matten-mlprep` | Pass — boundary CI |
| documented limitations | **Added in v0.19.0** |
| public API snapshot or equivalent | **Added in v0.19.0** |

### 3.2 Decision rationale

`matten-mlprep`'s beta gate is structural (RFC-022 §9: README beta text, examples
in CI, documented limitations, an API snapshot, migration notes on breaking
changes) rather than usage-based. Every structural criterion is met or added
here. Because the scope is fully closed — four pure, fully-specified transforms —
the risk that field usage reveals a spec gap is low, unlike open-scope crates
such as `matten-data`. "Beta" honestly describes the crate: useful for small real
workflows with an API that is mostly stable.

This is an *early* beta: the crate shipped in v0.18.0 and has limited field usage.
The label is reversible while pre-1.0; a breaking change would carry migration
notes (RFC-022 §9).

### 3.3 Hardening applied

- Added a **Limitations** section (rank-2 only; population std; `NaN`/`Inf`
  propagate, not cleaned; ordered split only — no seeded/shuffled split yet; no
  streaming / large data).
- Added a **Public API** snapshot section (the four signatures + error variants)
  as the breaking-change baseline.
- Added tests for documented edge behavior: `NaN` column propagation in both
  scalers (not mistaken for `ZeroVariance`); single-row matrix is `ZeroVariance`.
- README status: experimental → **beta**.

---

## 4. Maturity-label mechanics (recap of RFC-022 §9)

- Labels are signaled by README badge/text and docs; the version stays `0.x`.
- `0.x` SemVer: a minor bump may break; additive docs/tests are a patch bump.
- A production-ready (not candidate) label and any v1.0 still require explicit
  maintainer confirmation (RFC-022 §7, §9).

## 5. Not promoted

- `matten-data` — remains future; its beta decision is a separate v0.20+ gate
  (RFC-023).
- `matten-nalgebra`, `matten-candle` — deferred (RFC-025 §10).
- streaming / large CSV — design-only (RFC-026).

## 6. Next

v0.20+ — `matten-data` beta-decision phase (RFC-023), per ROADMAP §8.
