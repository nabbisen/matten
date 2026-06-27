# Review Request — RFC-050–054 Production Migration Program

**Project:** `matten`
**Date:** 2026-06-27
**Scope:** RFC-050, RFC-051, RFC-052, RFC-053 (implemented); RFC-054 (documentation-only / deferred)
**Shipped across:** v0.23.0 – v0.23.4
**Submitted for:** structured review and binding ruling
**Author of this note:** implementation maintainer

This note accompanies the review bundle. It is written to orient the review, not to
pre-judge it. Acceptance-criteria statements below are claims to verify against the files, not
settled conclusions.

---

## 1. What is submitted

The RFC-050–053 migration program is implemented as documentation, a bridge-contract policy,
and release-guard changes. Core `matten` gains **no** new dependency, and no new bridge crate
was created. RFC-054 (`matten-migrate` CLI) is documentation-only and deferred; no
`tools/matten-migrate` exists.

The bundle contains the RFC docs, the implementation handoffs, the full `docs/src/migration/`
tree, and the touched files (matten-ndarray contract table, SUMMARY, cross-links, the guard
script, CHANGELOG, ROADMAP). A file-path list is in §7.

## 2. Release mapping

| Release | RFC(s) | Contents |
|---|---|---|
| v0.23.0 | 050, 052 (Rust) | Migration foundation (index, when-to-migrate, target-selection, common-pitfalls) + `ndarray` and `nalgebra` playbooks + migration overclaim guard |
| v0.23.1 | 052 (rest) | Polars/Pandas, Candle, NumPy playbooks (completes RFC-052 target set) |
| v0.23.2 | 051 | Bridge conversion-contract template + bridge-crate policy + matten-ndarray contract table |
| v0.23.3 | — | Version-string hygiene + self-updating drift guard (see §4) |
| v0.23.4 | 053 | Readiness checklist, report template, worked example (completes the batch) |

## 3. Acceptance-criteria status (to verify)

- **RFC-050:** migration index, when-to-stay/migrate, target-selection matrix, ≥3 example-
  based migration references, dependency-light statements, required wording ("outgrowing
  matten is a successful PoC outcome"), no "faster than"/"drop-in replacement" claims, no core
  dependency added.
- **RFC-051:** contract template, bridge-crate policy, matten-ndarray contract table, "do not
  re-export Tensor" rule, "bridges own their target dependency" rule, future-bridge checklist,
  no new bridge crate, no core target-library dependency.
- **RFC-052:** all five playbooks (ndarray, nalgebra, Polars/Pandas, Candle, NumPy), decision
  tree, ≥3 example→target mappings, no "replaces the target"/"target always better" claims,
  positioning sections cite only accepted RFC-049 results (Rust targets) or state honestly
  that no benchmark exists (non-Rust targets).
- **RFC-053:** report template (9 sections), readiness checklist mapping signals to playbooks,
  one worked example, required advisory disclaimer present, no CLI, no source-scanning tool.

## 4. Decisions and judgment calls for your attention

These are points where a choice was made; they are flagged for confirmation, not defended.

1. **Version bump.** The program opened with a **minor** bump to v0.23.0 (per the handoff's
   recommended target), though the changes are documentation-only and the usual convention
   treats docs as a patch. Subsequent increments were patches (v0.23.1–.4).
2. **"Phase 2" wording in user docs.** User-facing migration prose cites "the accepted RFC-049
   Rust peer comparison" without the "Phase 2" label, to avoid collision with the retired
   dynamic-ingestion "Phase 1/Phase 2" wording guard. The canonical "RFC-049 Phase 2" name is
   retained in RFCs/CHANGELOG/benchmark docs.
3. **Banned phrases kept out entirely.** Rather than rely on negation-aware grepping, the
   migration docs avoid the literal phrases "faster than" / "drop-in replacement" / "automatic
   conversion" even in negated educational sentences (reworded, e.g. "swap matten out
   unchanged"), so the overclaim guard is a clean phrase match.
4. **Version-string hygiene detour (v0.23.3).** The v0.23.0 family bump left stale `0.22`
   pins/labels across READMEs, the core `lib.rs` rustdoc, and ~10 doc pages; the broken pins
   (`matten = "0.22"`, a caret requirement excluding 0.23.x) would have held copying users on
   the old family. Root cause: the existing version guard hardcoded `CURRENT_MINOR="22"` with a
   manual-bump comment that was missed. The guard now **derives the minor from `Cargo.toml`**.
   Worth confirming: the dynamic derivation, the guard's scope (`USER_DOCS`), and that only
   genuinely historical `0.22` references were preserved.
5. **Guard exception for the advisory disclaimer.** The RFC-053 disclaimer contains "does not
   perform automatic conversion." The overclaim guard's exception was widened to allow that
   negated form; verified it still flags a positive "automatic conversion" claim.
6. **matten-ndarray contract table** was filled in against `convert.rs`/`error.rs` (copies both
   ways, numeric-only, rejects dynamic via `DynamicTensor` unconditionally, preserves logical
   row-major through non-standard layouts, rejects zero-sized axes, `Result` never panics,
   variants `DynamicTensor`/`ZeroSizedAxis`/`NdarrayShape`/`Matten`).
7. **nalgebra path is manual.** No `matten-nalgebra` bridge was created; the playbook documents
   the manual `from_row_slice` conversion and the column-major caution, and the bridge-crate
   policy states new bridges need separate approval.
8. **Path note.** The handoff referenced `docs/src/companions.md`; the actual file is
   `docs/src/examples/companions.md`, which is where the bridge cross-link was added.

## 5. Out of scope / not done

RFC-054 implementation (`tools/matten-migrate`); any new bridge crate; any new core `matten`
API or dependency; RFC-049 Phase 3 (cross-language peer benchmarks) — the non-Rust playbooks
state no benchmark exists rather than inventing numbers.

## 6. Open items beyond this review (for awareness)

- RFC lifecycle bookkeeping: RFC-050–053 are still in `rfcs/proposed/` and would move to
  reflect completion once this review clears.
- `nalgebra` MSRV-policy decision (pending if upgrading the pinned version).
- Windows 11 / WSL2 benchmark baseline (open question, unrelated to this batch).

## 7. Review set (file paths)

RFC docs:
```
rfcs/proposed/050-production-migration-guide-and-bridge-strategy.md
rfcs/proposed/051-bridge-conversion-contracts-and-companion-crate-policy.md
rfcs/proposed/052-production-target-playbooks.md
rfcs/proposed/053-migration-readiness-diagnostics-and-report-format.md
rfcs/proposed/054-matten-migrate-assisted-migration-tool.md
```
Handoffs:
```
rfcs/handoffs/050-053-production-migration-implementation-handoff.md
rfcs/handoffs/050-053-acceptance-qa-checklist.md
rfcs/handoffs/050-053-release-guard-checklist.md
rfcs/handoffs/054-deferred-implementation-note.md
```
Migration docs:
```
docs/src/migration/index.md
docs/src/migration/when-to-migrate.md
docs/src/migration/target-selection.md
docs/src/migration/common-pitfalls.md
docs/src/migration/bridge-contracts.md
docs/src/migration/bridge-crate-policy.md
docs/src/migration/readiness-checklist.md
docs/src/migration/readiness-report.md
docs/src/migration/examples/linear-regression-gd-readiness.md
docs/src/migration/playbooks/index.md
docs/src/migration/playbooks/ndarray.md
docs/src/migration/playbooks/nalgebra.md
docs/src/migration/playbooks/polars-and-pandas.md
docs/src/migration/playbooks/candle.md
docs/src/migration/playbooks/python-numpy.md
```
Touched:
```
crates/matten-ndarray/README.md
docs/src/SUMMARY.md
docs/src/reference/migration.md
docs/src/examples/companions.md
scripts/check-release-docs.sh
CHANGELOG.md
ROADMAP.md
```

## 8. Verification performed

Each release ran the gate suite green: `cargo fmt --check`; the four guard scripts
(core-dependency-boundary, published-dependency-isolation, matten-data-scope, release-docs,
including the migration overclaim and dynamic version-string guards); `cargo test --workspace
--all-features` (11 suites); the matten-ndarray doctests; the RFC-031 regression fixture; and,
where code/resolution was touched, the MSRV 1.85 check. SUMMARY links were verified to resolve
to real files. No core `Cargo.toml` dependency changed in any release.

---

Requesting review and a binding ruling. Findings at any priority (P0/P1/P2) with line-level
corrections are welcome; the migration tree is intended to be the stable base that any future
RFC-054 tooling would build on.
