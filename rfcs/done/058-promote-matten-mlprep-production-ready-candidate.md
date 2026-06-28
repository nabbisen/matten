# RFC-058: Promote `matten-mlprep` — Beta → Production-Ready Candidate

**Status:** Implemented (v0.26.0); architect-accepted (review 2026-06-27)
**Target Release:** v0.26.0 (architect-preferred companion-maturity step). May bundle with the `matten-data` maturity decision if that RFC lands at the same time, but **must not be blocked** on it.
**Related:** RFC-024 / RFC-028 (`matten-mlprep`), RFC-029 (companion maturity labels), RFC-030 (lock-step family versioning), RFC-057 (`matten-ndarray` promotion precedent), ROADMAP §7 (maturity ladder + `matten-mlprep` beta gate)
**Scope:** Advance the `matten-mlprep` maturity Status label one rung, from **Beta** to **production-ready candidate**. Label and documentation only — no API, runtime, or dependency change.

---

## 1. Summary

`matten-mlprep` has held **Beta** status since v0.19.0 (RFC-029). It is a small, scope-closed
companion of transparent preprocessing helpers. This RFC audits it against the ROADMAP's
*production-ready candidate* signals, finds it meets them, and proposes advancing it **one rung**
to *production-ready candidate* — deliberately **not** straight to production-ready, because a
documented functional limitation (ordered-only split) fits the candidate framing better (§5).

This is the second step of the companion-maturity line opened by RFC-057. It is label/docs only:
under lock-step family versioning (RFC-030) the crate keeps the shared family version, and it is
**not** a v1.0 statement.

## 2. Motivation

After `matten-ndarray` reached production-ready (RFC-057), the remaining companion decisions are
`matten-mlprep` and `matten-data`. `matten-mlprep` is the older of the two (Beta since v0.19.0),
has a stable surface, and a strong test suite — the natural next decision. Settling its rung tells
users how seriously they can lean on it.

## 3. The bar (ROADMAP *production-ready candidate* signals)

```text
strong tests
examples in CI
clear error types
documented compatibility policy
no known P0/P1 issues
release checklist complete
```

It must also still satisfy the **Beta decision gate** it already holds (API small/teachable;
deterministic; shape rules documented; zero-variance behavior explicit; train/test split behavior
explicit; no ML-framework scope).

## 4. Audit against the bar

| Signal | Evidence | Verdict |
|---|---|---|
| Strong tests | `tests/preprocessing.rs`: 17 tests — standardize known-values + per-column zero-mean/unit-std, constant-column → `ZeroVariance`, NaN-column propagates (not mis-flagged as zero-variance); minmax unit-interval + constant-column error + NaN; bias prepends ones; split ordered partition/shapes, determinism, invalid-ratio rejection, empty-train rejection, single-row zero-variance; actionable error Display | ✅ |
| Examples in CI | All four examples are **executed** in the `smoke` CI job (`mlprep_standardize_columns`, `mlprep_minmax_scale`, `mlprep_add_bias_column`, `mlprep_train_test_split`; namespaced via `[[example]]` to avoid binary-name collisions), in addition to the workspace `cargo check --examples` | ✅ |
| Clear error types | `MattenMlprepError` (`#[non_exhaustive]`) with 6 documented variants — `DynamicTensor`, `ExpectedMatrix`, `InvalidRatio`, `EmptySplit`, `ZeroVariance`, `Matten` — `Display` with actionable messages, `std::error::Error` + `source()` | ✅ |
| Documented compatibility policy | README §Compatibility: SemVer pre-1.0 (`0.x`, breaking-on-minor with migration notes), MSRV 1.85, shares the `0.25.x` `matten` family version (RFC-030) | ✅ |
| No known P0/P1 issues | None recorded; the Beta gate (ROADMAP §7) is fully satisfied | ✅ |
| Release checklist complete | Covered by the lock-step family release process | ✅ |

**Beta decision gate** (re-verified): API is four small functions + one error enum (teachable);
functions are deterministic (`split_is_deterministic` test; ordered split); shape rules documented
(matrix-expected, `ExpectedMatrix` error); zero-variance behavior explicit (errors, not silent
zeros); train/test split behavior explicit (ordered, `first floor(n·ratio)`); only dependency is
`matten` (no ML-framework scope). All ✅.

## 5. The rung judgment: candidate, not production-ready

`matten-mlprep` arguably meets several *production-ready* signals too (mature docs, stable API,
compat/MSRV policy). The author nonetheless recommends advancing **only to candidate**, for two
reasons:

1. **A documented functional limitation.** `train_test_split` is **ordered only** — it does not
   shuffle. Shuffled (optionally seeded) splits are common in ML preprocessing, so this is a real
   caveat, not a cosmetic one. The candidate rung — "usable seriously **if** the documented limits
   are acceptable" — describes this honestly; full production-ready ("recommend as a normal
   dependency for its documented scope") would overstate it.
2. **Ladder discipline.** `matten-ndarray` spent real time at candidate before production-ready
   (RFC-057). Advancing one rung at a time keeps the maturity ladder meaningful.

If the architect judges the ordered-split limitation acceptable for full production-ready, the
promotion target can be raised; the author's recommendation is candidate.

**Architect ruling (2026-06-27): production-ready candidate. Full production-ready is deferred** —
the ordered-only split is a real functional caveat, acceptable for candidate ("usable seriously if
the documented limits are acceptable") but not yet for a broad production-ready recommendation.

### 5.1 Future full-production-ready exit criteria (P2 — recorded, not required here)

Before `matten-mlprep` is later promoted candidate → production-ready, settle the split story via
**one** of:

- **Option A — keep ordered-only and justify it.** Document explicitly that the crate's scope is
  *deterministic, transparent preprocessing helpers* (not randomized ML-workflow management) and
  that ordered split is intentionally sufficient for that scope.
- **Option B — add shuffled / seeded split first.** A *separate feature RFC* (it raises API and
  RNG/dependency-policy questions) adding e.g. a seeded shuffled split or a small split-options
  type. This is out of scope for RFC-058.
- **Option C — stay candidate until the split story is settled** (the current path).

These are recorded for the future promotion only; none is required for this RFC.

## 6. API-snapshot stance

As with RFC-057, **no dedicated `public-api-snapshot` file** is added — confirmed by architect
ruling (2026-06-27). The surface is four functions + one error enum; the README "Public API" block
plus rustdoc is the snapshot-equivalent and must stay exact, listing: the four functions; the
`MattenMlprepError` enum; the deterministic ordered-split behavior; the zero-variance /
constant-column error behavior; the matrix-shape expectation; the dynamic-tensor rejection
behavior; the no-ML-framework scope; and the `matten`-only dependency.

## 7. Proposed implementation (label + docs only)

- `crates/matten-mlprep/README.md`: lead badge/text **Beta** → **production-ready candidate**.
- `crates/matten-mlprep/src/lib.rs`: `# Status` / crate-doc maturity line → production-ready candidate.
- `crates/matten-mlprep/Cargo.toml`: **verify the `description` for stale "Beta" wording** and
  correct it if present (architect P1). It is expected to already be maturity-neutral; confirm.
- `docs/src/reference/compatibility.md`, docs maturity tables, the external-design maturity
  progression, and the ROADMAP `matten-mlprep` gate: reflect the new rung.
- Extend the RFC-057 maturity-label freshness guard (or add a sibling check) so `matten-mlprep`'s
  own current-status files no longer say **Beta** once promoted, with historical contexts exempt.
- CHANGELOG + ROADMAP entries.

No source/API/runtime/dependency change. The crate stays at the family version (RFC-030).

## 8. Acceptance criteria

- [ ] Architect confirms `matten-mlprep` meets the *production-ready candidate* signals (§4) and
      rules on the rung (candidate vs production-ready, §5).
- [ ] Status label updated consistently across README, `lib.rs`, compatibility page, docs maturity
      tables, external-design progression, and ROADMAP — no stale **Beta** wording for the crate
      outside historical contexts (CHANGELOG, RFCs, migration narrative).
- [ ] A context-aware maturity-label check (or manual verification) confirms the above.
- [ ] Published-dependency-isolation guard still confirms core `matten` carries no companion
      dependency; no `Cargo.toml` dependency expansion (still `matten`-only).
- [ ] All four examples continue to execute in CI; the 17-test suite remains green.

## 9. Non-goals

- **Not a v1.0** and **not full production-ready** (recommended rung is candidate; §5).
- No API/signature/error-variant/runtime change; **no shuffled or seeded split** is added here (a
  separate feature RFC if ever wanted); no new preprocessing functions; no ML-framework scope.
- No dependency change (stays `matten`-only); no change to lock-step family versioning.
- Does **not** decide `matten-data` maturity — that is a separate RFC.

## 10. Sequencing

If accepted, this is the second companion-maturity step (after RFC-057). The `matten-data` Beta
decision follows as its own RFC. Once the companion rungs are settled, a deliberate v1.0 readiness
review (core + companions) can be put to the maintainer — v1.0 still requires explicit maintainer
confirmation.
