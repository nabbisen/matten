# RFC-059: `matten-data` Maturity Decision — Beta → Production-Ready Candidate

**Status:** Implemented (v0.27.0); architect-accepted conditionally (review 2026-06-27); both promotion-blocking hygiene fixes applied
**Target Release:** v0.27.0 (architect-preferred; completes the companion-maturity line)
**Related:** RFC-033 (`matten-data` Beta-decision & scope lock), RFC-034 / RFC-035 (table model & CSV ingestion), RFC-036 (Beta promotion, v0.22.0), RFC-042 (anti-scope guard), RFC-030 (lock-step family versioning), RFC-057 / RFC-058 (companion-maturity precedents), ROADMAP §9.2 Track A
**Scope:** Decide the `matten-data` maturity rung. Recommended: advance the Status label one rung, **Beta → production-ready candidate**, *conditional* on two hygiene fixes (§5). Label/docs/packaging only — no API, runtime, or dependency change, and **no scope expansion** (the RFC-042 on-ramp scope lock is preserved).

---

## 1. Summary

`matten-data` is the last companion without a settled post-Beta maturity decision. It has been
**Beta** since v0.22.0 (RFC-036). This RFC audits it against the *production-ready candidate*
signals. The code substantively meets the bar — it is in fact the most-tested companion — but the
audit surfaced **two real hygiene findings** that must be fixed first (§5). With those fixed, the
author recommends advancing **one rung to production-ready candidate**, which fits a deliberately
scope-locked CSV→tensor on-ramp ("usable seriously if the documented limits are acceptable").

Because `matten-data` is the newest companion and the findings show maturity-label upkeep had
lapsed, **hold-at-Beta is a legitimate alternative** the architect may prefer (§6). Either way, the
two findings should be fixed.

This is label/docs/packaging only: no API/runtime/dependency change, no scope expansion, the crate
stays at the family version (RFC-030), and it is **not** a v1.0 or full-production-ready statement.

## 2. Motivation

`matten-ndarray` is production-ready (RFC-057) and `matten-mlprep` is a production-ready candidate
(RFC-058). Settling `matten-data` completes the companion-maturity line and tells users how
seriously they can lean on the ingestion on-ramp — within its locked scope.

## 3. The bar

*Production-ready candidate* signals (ROADMAP):

```text
strong tests
examples in CI
clear error types
documented compatibility policy
no known P0/P1 issues
release checklist complete
```

Plus a `matten-data`-specific constraint: the promotion must **not** weaken the RFC-033/RFC-042
scope lock. `matten-data` stays an ingestion on-ramp — *not a dataframe engine* (no joins, group-by,
pivots, query layer).

## 4. Audit against the bar

| Signal | Evidence | Verdict |
|---|---|---|
| Strong tests | `src/tests.rs`: **34 tests** (CSV parsing incl. ragged-row / empty-input / quoting, schema summary, column selection incl. duplicate/missing, missing-value fill, strict numeric conversion + non-numeric/missing rejection, `to_tensor`, error display) — the most-tested companion | ✅ |
| Examples in CI | All **seven** examples (`csv_to_tensor`, `data_00`–`data_05`) are **executed** in the `smoke` job; the `data` job adds `cargo check --examples`, a crate-scoped `clippy -D warnings`, tests, doctests, and the RFC-042 anti-scope guard | ✅ |
| Clear error types | `MattenDataError` (`#[non_exhaustive]`, 11 variants: `Csv`, `Io`, `EmptyInput`, `MissingColumn`, `DuplicateColumn`, `DuplicateSelection`, `RaggedRow`, `NonNumericValue`, `MissingValue`, `EmptySelection`, `Matten`) with `Display` (actionable) + `std::error::Error` + `source()` | ✅ |
| Documented compatibility policy | README §Compatibility: pre-1.0 SemVer, shares the `matten` family version (RFC-030), MSRV 1.85 | ✅ |
| No known P0/P1 issues | None; two **P2** hygiene findings only (§5) | ⚠️ (P2 only) |
| Release checklist complete | Covered by the lock-step family process; `matten-data` also carries its own scope guard | ✅ |
| Scope lock intact | "Not a dataframe library" section + RFC-042 three-check guard (file names / public API / README), CI-enforced | ✅ |

The surface is a small, scope-locked model: `Table` (`from_csv_str` / `from_csv_path` /
`schema_summary` / `select_columns` / `fill_missing` / `try_numeric`), `NumericTable` (`to_tensor`),
`SchemaSummary` / `ColumnSummary` / `ColumnKind`, `CellValue`, `MattenDataError`.

## 5. Two findings (P2, must-fix before promotion)

The audit found two real issues. Both are **doc/packaging hygiene, not code-quality problems**, and
both are worth fixing **regardless** of the rung decision:

1. **Stale `Cargo.toml` description.** It still reads *"Experimental table-to-Tensor preparation
   companion…"* even though `matten-data` was promoted Experimental → Beta in v0.22.0. crates.io is
   currently advertising the wrong maturity. The v0.22.0 promotion updated the README/lib.rs but
   missed the package description. **Fix:** set a maturity-neutral description (no "Experimental").
2. **Missing example `required-features`.** All six `data_0X` examples call the CSV constructors
   (`from_csv_str` / `from_csv_path`), which are gated behind the default-on `csv` feature, but only
   `csv_to_tensor` declares `required-features = ["csv"]`. **Evidence:** `cargo build -p matten-data
   --examples --no-default-features` fails with `E0599: from_csv_str not found` (example
   `data_03_missing_values`). Default builds are fine (csv is default-on), so CI/checklist using
   default features never hit it. **Fix:** add `[[example]]` entries with
   `required-features = ["csv"]` for `data_00`–`data_05`.

That the maturity label drifted in the package description is itself mild evidence that
`matten-data` had not been getting candidate-grade upkeep — relevant to §6.

## 6. The rung judgment

**Recommendation: Beta → production-ready candidate, conditional on the §5 fixes.**

With the findings fixed, `matten-data` meets every candidate signal — arguably more convincingly
than `matten-mlprep` did (34 tests vs 17; a CI-enforced scope guard; all examples executed). The
candidate rung — "usable seriously if the documented limits are acceptable" — is an exact fit for a
CSV→tensor on-ramp whose limits (small in-memory PoC data; not a dataframe engine) are the most
rigorously documented and guard-enforced of all the companions.

**Alternative considered and rejected: hold at Beta.** `matten-data` is the newest companion (Beta
only since v0.22.0), CSV ingestion has the widest edge-case surface of the companions, and the §5
findings show label upkeep had lapsed. A maintainer wanting one more soak cycle could keep it at
Beta and just apply the §5 fixes. **Architect ruling (2026-06-27): hold-at-Beta is *not* chosen** —
the two findings are packaging/docs hygiene, not immature runtime behavior, and after the fixes Beta
would understate the crate's maturity.

This RFC does **not** propose full production-ready: like `matten-mlprep`, one rung at a time.
**Architect ruling: full production-ready is deferred** and requires a *separate future review*
after at least one candidate cycle — `matten-data` is the newest companion, CSV/table ingestion has
a wide edge-case surface, large/streaming CSV is explicitly deferred, and it is deliberately an
on-ramp, not a dataframe engine.

**Architect ruling (2026-06-27): RFC-059 accepted conditionally — promote Beta → production-ready
candidate once the two §5 hygiene fixes are applied (both promotion-blocking).**

## 7. Proposed implementation

Hygiene fixes — **both promotion-blocking** (architect ruling):

- `crates/matten-data/Cargo.toml`: replace the stale `"Experimental…"` `description` with a
  **maturity-neutral** one (no `Experimental` / `Beta` / `candidate` wording — the description must
  stay stable across rung changes), e.g. `"CSV/table-to-Tensor preparation companion for matten
  (small PoC datasets)."`.
- `crates/matten-data/Cargo.toml`: add explicit `[[example]]` entries with
  `required-features = ["csv"]` for the six CSV-using examples — `data_00_quickstart`,
  `data_01_schema_summary`, `data_02_select_columns`, `data_03_missing_values`, `data_04_to_tensor`,
  `data_05_errors` (mirroring the existing `csv_to_tensor` entry).
- **Verification:** `cargo build -p matten-data --examples --no-default-features` must succeed (by
  correctly *skipping* the gated examples); all seven examples must still execute under default
  features in the `smoke` job.

If the rung is **candidate** (ruled):

- `crates/matten-data/README.md` lead and `src/lib.rs` Status: **Beta** → **production-ready candidate**.
- `README.md` crate table; `docs/src/reference/compatibility.md`; the external-design maturity
  progression (add a v0.27.0 entry; keep the v0.19/0.22 history); `docs/src/examples/data.md`
  status note; ROADMAP gate/decision row.
- **No stale current-status `Beta`/`Experimental` wording** for `matten-data` outside historical
  contexts (CHANGELOG release entries, RFC history, maturity-progression timeline, historical
  migration narrative). Extend the maturity-label freshness guard so `matten-data`'s own
  current-status files no longer say **Beta**/**Experimental** once promoted (context-aware; do not
  globally ban the words) — mirroring RFC-057/058 (P2).

No API-snapshot file (architect ruling, consistent with RFC-057/058): the README "Public API" /
status block plus rustdoc is the snapshot-equivalent. Because the surface is larger than
`matten-ndarray`/`matten-mlprep`, the block must be kept *especially* exact — covering the types
`Table`, `NumericTable`, `SchemaSummary`, `ColumnSummary`, `ColumnKind`, `CellValue`,
`MattenDataError`; the methods `from_csv_str`, `from_csv_path`, `schema_summary`, `select_columns`,
`fill_missing`, `try_numeric`, `to_tensor`; and the behaviors: the `csv` feature requirement,
missing-value handling, strict numeric conversion, the scope lock (not a dataframe), and the error
model. (A dedicated snapshot file may become worthwhile later if this block drifts; not required
now.)

No source/API/runtime/dependency change; no scope expansion; stays at the family version (RFC-030).

## 8. Acceptance criteria

- [ ] Architect rules on the rung (candidate — recommended — vs hold-at-Beta) and confirms the §5
      fixes are required.
- [ ] §5 fixes applied: neutral package description; all CSV-using examples carry
      `required-features = ["csv"]` (verified by `cargo build -p matten-data --examples
      --no-default-features` succeeding or correctly skipping the gated examples).
- [ ] If promoted: Status label updated consistently; no stale **Beta**/**Experimental** wording in
      current-status files outside historical contexts; maturity-label guard extended.
- [ ] RFC-042 anti-scope guard still passes; no new public API; no dependency change; published
      dependency isolation unaffected.
- [ ] 34-test suite and all seven examples remain green in CI.

## 9. Non-goals

- **No scope expansion.** No joins/group-by/pivot/query; the RFC-042 lock is preserved. This is the
  hard line for `matten-data`.
- **Not v1.0** and **not full production-ready** (recommended rung is candidate).
- No API/signature/error-variant/runtime change; no new ingestion formats; no streaming/large-CSV
  work (RFC-037 stays deferred); no dependency change beyond the existing `matten` + optional `csv`.
- No change to lock-step family versioning.

## 10. Versioning & sequencing

Target **v0.27.0** (label/docs/packaging release; visible final companion-maturity step). With this
settled, the maturity ladder reads: `matten-ndarray` production-ready, `matten-mlprep` and
`matten-data` production-ready candidates. The natural next milestone is a deliberate **v1.0
readiness review** (core + companions), which requires explicit maintainer confirmation.
