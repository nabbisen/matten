# RFC-057: Promote `matten-ndarray` — Production-Ready Candidate → Production-Ready

**Status:** Implemented (v0.25.0); architect-accepted (review 2026-06-27)
**Target Release:** v0.25.0 (architect-preferred companion-maturity milestone opener). A v0.24.x patch is **not** recommended unless release cadence demands it — the maturity label is user-visible and strategic.
**Related:** RFC-025 (ndarray bridge), RFC-027 (bridge error type), RFC-029 (companion maturity labels), RFC-030 (lock-step family versioning), ROADMAP §7 (maturity ladder + `matten-ndarray` gate)
**Scope:** Promote the `matten-ndarray` companion crate's maturity Status label from *production-ready candidate* to *production-ready*. Label and documentation only — no API, runtime, or dependency change.

---

## 1. Summary

`matten-ndarray` has held *production-ready candidate* status since v0.19.0 (RFC-029). It is the
most mature companion crate and the only one through which core `matten` hands data to a heavier
ecosystem. This RFC audits it against the ROADMAP's *production-ready* signals, finds it meets the
bar, and proposes promoting its Status label to **production-ready**.

This is the first concrete step of the post-v0.24 *companion maturity* theme on the road to a
considered v1.0. It is explicitly **not** a v1.0 release: per the ROADMAP, the production-ready
label "does not automatically imply version 1.0," which still requires explicit maintainer
confirmation. Under lock-step family versioning (RFC-030) the crate keeps the shared family
version; only its Status label changes.

## 2. Motivation

v0.24 closed the last tracked *pre-1.0 core consistency* item (the fallible-reduction surface).
The remaining pre-1.0 work is the set of companion maturity decisions the ROADMAP has tracked
since v0.21+. `matten-ndarray` is the natural first decision: it is small, scope-closed, heavily
tested, and is the load-bearing bridge cited throughout the migration guide (RFC-050–053). Settling
its status removes ambiguity for users choosing whether to depend on it "seriously."

## 3. The bar (ROADMAP *production-ready* signals)

```text
mature docs
stable API
compatibility and MSRV policy
clear release notes
no hidden dependency surprises
```

The crate must also still satisfy the *production-ready candidate* signals it already holds
(strong tests; examples in CI; clear error types; documented compatibility policy; no known
P0/P1 issues; release checklist complete).

## 4. Audit against the bar

| Signal | Evidence | Verdict |
|---|---|---|
| Strong tests | `tests/conversion.rs`: 17 tests — roundtrips (scalar/vector/matrix/N-D/rank-4), logical-order preservation under transposed/sliced/permuted `ArrayD`, zero-axis rejection, rank-overflow → `Matten` error, `std::error::Error` + `source()`, dynamic rejection (not panic), numeric-under-`dynamic`-feature, NaN/Inf passthrough, fractional fidelity, standard-layout output | ✅ |
| Stable API | Public surface = `to_arrayd`, `from_arrayd`, and `MattenNdarrayError` (`#[non_exhaustive]`). README: "scope is closed and the API is stable." No views/lifetimes/generic dtypes | ✅ |
| Clear error types | `MattenNdarrayError` with 4 documented variants, `Display`, `std::error::Error` + `source()`, `#[non_exhaustive]` (future variants are non-breaking) | ✅ |
| Mature docs | Crate-level rustdoc with runnable doctest + Status/Behavior/Feature-flags sections; README with overview, API table, compatibility, copy semantics | ✅ |
| Compatibility + MSRV policy | README §Compatibility: SemVer pre-1.0 (`0.x`), MSRV 1.85 (edition 2024), `ndarray` `0.16` minor supported and pinned; bumping the supported `ndarray` minor is a compatibility event (RFC-025 §6) | ✅ |
| Clear release notes | Lock-step family versioning (RFC-030); changes recorded in the root `CHANGELOG.md` | ✅ |
| No hidden dependency surprises | Dependencies are `matten` + `ndarray`, both documented; the published-dependency-isolation guard enforces that core `matten` carries no `ndarray` dependency | ✅ |
| Examples in CI | The `matten-ndarray` CI job builds the crate, tests, and doctests; the workspace `check` job compiles all examples (`cargo check --workspace --examples`) | ⚠️ compiled, not executed (see §5) |
| No known P0/P1 issues | None recorded; the candidate gate checklist (ROADMAP §7) is fully satisfied | ✅ |

**Original candidate gate** (ROADMAP §7), re-verified: scalar/vector/matrix/N-D conversions work;
roundtrip tests reliable; dynamic tensors rejected clearly (unconditional `DynamicTensor`); copy
behavior documented; no zero-copy promise ("Both directions copy; no zero-copy is claimed"); core
`matten` has no `ndarray` dependency. All ✅. The single ⚠️ is "examples run in CI."

## 5. The "examples run in CI" checkbox (already satisfied) and the API-snapshot question (settled)

**Correction (post-acceptance).** The candidate gate's "examples run in CI" item was *already*
satisfied. The dedicated `smoke` CI job runs every workspace example via `cargo run --example` on
every push/PR, including both bridge examples:

```bash
cargo run -p matten-ndarray --example to_arrayd
cargo run -p matten-ndarray --example from_arrayd
```

The original audit examined only the `bridge` job (crate tests/doctests) and the `check` job
(`cargo check --examples`, compile-only) and missed the `smoke` job, so it reported the examples as
compiled-but-not-executed. They are in fact executed in standard CI. **No CI change is required for
the promotion** — the architect's P1 condition is met by the pre-existing `smoke` job. (Both
examples were additionally re-verified locally to run and print `ok`.)

**API snapshot: skipped by architect ruling.** No dedicated `public-api-snapshot`-style file is
added for `matten-ndarray` — for a two-function surface it would cost more maintenance than it is
worth. The README/rustdoc API table instead serves as the snapshot-equivalent and must remain
exact, listing: both public functions; the error enum; copy behavior; dynamic-rejection behavior;
the supported `ndarray` minor; zero-sized-axis behavior; and the no-zero-copy promise.

## 6. Proposed implementation (label + docs only)

On acceptance, the promotion is realized as:

- `crates/matten-ndarray/README.md`: badge/lead line *production-ready candidate* → *production-ready*.
- `crates/matten-ndarray/src/lib.rs`: `# Status` section → production-ready.
- `crates/matten-ndarray/Cargo.toml`: `description` "Production-ready candidate conversion bridge…"
  → "Production-ready conversion bridge…".
- `docs/src/reference/compatibility.md` and any maturity-table references: reflect the new label.
- CI: no change needed — the pre-existing `smoke` job already executes both bridge examples
  (`cargo run --example to_arrayd` / `from_arrayd`) on every push/PR (§5).
- A context-aware stale-label check (or manual verification per checklist) confirming no current
  status page still calls `matten-ndarray` a "candidate" outside historical contexts (P2 follow-up).
- CHANGELOG + ROADMAP entries; ROADMAP maturity table updated.

No source/API/runtime/dependency change. The crate stays at the family version (RFC-030).

## 7. Acceptance criteria (architect-ruled)

- [x] `to_arrayd` and `from_arrayd` examples execute in CI — already covered by the pre-existing
      `smoke` job (`cargo run --example`); no CI change required (§5).
- [ ] The README/rustdoc API table lists all seven items in §5 (both functions; error enum; copy
      behavior; dynamic rejection; supported `ndarray` minor; zero-sized-axis behavior; no
      zero-copy promise).
- [ ] The Status label is updated consistently across README, `lib.rs`, `Cargo.toml` description,
      `docs/src/reference/compatibility.md`, the docs maturity tables, ROADMAP, and CHANGELOG.
- [ ] No stale current-status "production-ready candidate" / "candidate" wording survives for
      `matten-ndarray` outside historical contexts (CHANGELOG release entries, RFC history,
      migration narrative). A context-aware release-docs/maturity check or manual verification
      confirms this (P2 follow-up).
- [ ] The published-dependency-isolation guard still confirms core `matten` carries no `ndarray`
      dependency; no `Cargo.toml` dependency expansion beyond `matten` + `ndarray`; no API surface
      change appears in docs beyond label/status wording.

## 8. Non-goals

- **Not a v1.0.** Production-ready is a Status label; a v1 release still requires explicit
  maintainer confirmation (ROADMAP).
- No API, signature, error-variant, runtime, or dependency change; no zero-copy work; no broadening
  of `ndarray` version support beyond the documented `0.16` minor (a separate compatibility event).
- Does **not** decide `matten-mlprep` or `matten-data` maturity — those are separate Beta decisions
  with their own gates (ROADMAP §7) and would be their own RFCs.
- No change to lock-step family versioning.

## 9. Sequencing

If accepted, this is a natural opener for a v0.25.0 *companion-maturity* line. The `matten-mlprep`
and `matten-data` Beta decisions follow as separate RFCs, after which a deliberate v1.0 readiness
review (core + companions) can be put to the maintainer.
