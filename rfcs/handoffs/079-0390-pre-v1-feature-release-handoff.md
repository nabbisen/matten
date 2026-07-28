# RFC-079 `0.39.0` Pre-v1 Feature Release: Implementation Handoff

**Project:** `matten`
**Related RFC:** RFC-079 (design authority)
**Document kind:** Detailed release-preparation handoff
**Status:** Accepted and executed; `0.39.0` release-prep committed. Both blocking owner decisions
were made before implementation — version bump confirmed, matten-stats first-publish deferred
pending external ddof review (RFC-079 §3) — and the narrowed scope (0.39.0 ships RFC-077 only) was
implemented accordingly. Publish/tag remain a separate, not-yet-authorized step
**Date:** 2026-07-28

---

## 1. Purpose

Prepare the `0.39.0` release commit as one reviewable slice. RFC-079 is the design authority.

**No tag. No publish.** This ends at a reviewed release-prep commit.

## 2. Preconditions — both now satisfied

```text
RFC-079 and this handoff accepted (GO, conditional on D1, applied)
the owner's explicit confirmation of the version bump: OBTAINED — bump to 0.39.0 confirmed
the owner's decision on RFC-079 §3: DEFER — matten-stats's first publish waits on an external
  read of RFC-078 §4.1 obtained by someone outside this project's assistant session; it is
  NOT published in 0.39.0 and NOT mentioned in the [0.39.0] CHANGELOG entry
RFC-077 and RFC-078 closed (ea8fd23, 7fb9c7c)
working tree clean
```

The owner considered and rejected both options RFC-079 §3 originally posed (external read now vs. accept
the risk now) in favor of a third: defer the publish entirely until the read exists. This narrows the
release's scope — see §7, which is now a no-op for this release.

## 3. What makes this release different from RFC-076's

This is a **`0.x` minor release**, not the v1.0 preparation. The differences matter, because RFC-076's
handoff is longer and more invasive and must not be used as a template:

```text
pre-1.0 wording          STAYS — still accurate on 0.x. Do NOT sweep it.
compatibility.md v1.0    UNTOUCHED — no 1.0 compatibility statement
cargo public-api         NOT required (RFC-076 §4.1 maintainer decision stands)
#[doc(hidden)] question  NOT reopened
maturity promotions      NONE
```

If you find yourself editing `pre-1.0` text or `compatibility.md`'s v1.0 section, stop — you are following
the wrong document.

## 4. Implementation order

1. **Version bump** (§5) — do it first so the retarget can be verified against a settled number.
2. **Version-string retarget** (§6) — 33 strings, 15 files.
3. **CHANGELOG `[0.39.0]`** (§8) — RFC-077 only; no `matten-stats` mention (§7 is a no-op this release).
4. **Tracking** — ROADMAP release-table row, Status, history row; `rfcs/README.md`.
5. **One formatting pass**, then the full gate set (§9).

## 5. Version bump

```text
Cargo.toml [workspace.package]  version = "0.38.0" -> "0.39.0"
Cargo.lock                       FIVE package entries -> 0.39.0
                                 (matten, matten-data, matten-mlprep,
                                  matten-ndarray, matten-stats)
crates/matten/src/lib.rs         //! ```toml install pin -> matten = { version = "0.39.0", ... }
edition = "2024"                 unchanged
rust-version = "1.85"            unchanged
```

Let Cargo regenerate the lock (`cargo check --workspace`); do **not** hand-edit it. Then:

```bash
cargo metadata --format-version 1 --no-deps   # expect FIVE crates, all 0.39.0
cargo check --workspace                        # proves the lock is in sync
git diff --name-only -- 'crates/*/src/'        # expect ONLY crates/matten/src/lib.rs
```

That last command is the cheapest defect detector in the slice — it has caught nothing in eight releases
because it has never been violated. Keep it that way.

## 6. Version-string retarget — 33 strings, 15 files

`0.38.0` → `0.39.0`, and `0.38.x family` → `0.39.x family`.

| File | Count |
|---|---:|
| `README.md` | 11 |
| `crates/matten-stats/README.md` | 3 |
| `crates/matten-mlprep/README.md` | 3 |
| `crates/matten-ndarray/README.md` | 3 |
| `docs/src/examples/data.md` | 3 |
| `crates/matten-data/README.md` | 1 |
| `crates/matten/README.md` | 1 |
| `crates/matten/src/lib.rs` | 1 |
| `docs/src/contributing/architecture.md` | 1 |
| `docs/src/contributing/release-checklist.md` | 1 |
| `docs/src/quick-start.md` | 1 |
| `docs/src/reference/boundary.md` | 1 |
| `docs/src/reference/compatibility.md` | 1 |
| `docs/src/reference/dynamic.md` | 1 |
| `docs/src/reference/public-api-snapshot.md` | 1 |

Notes:

- `crates/matten-stats/README.md` is **new to this list** — it did not exist at `0.38.0`. Do not skip it
  because it is absent from any older release's file list.
- `docs/src/reference/compatibility.md`'s **per-family history section legitimately keeps older versions**.
  Retarget only the current-family pin; leave the history alone.
- `docs/src/reference/public-api-snapshot.md`'s header says "current v0.38 release family" — retarget it,
  and the statement that the public API did not change is still true for core (`matten-stats` is a new
  crate, not a core API change).

Verification:

```bash
git grep -nE "0\.38\.(0|x)"
# expect ONLY:
#   CHANGELOG.md / ROADMAP.md / rfcs/            release + decision history
#   docs/src/reference/compatibility.md          per-family history section
#   docs/design/v1-readiness-audit.md            dated audit report -- do NOT retarget
#   scripts/check-release-docs.sh                guard comment recording the 0.38.0 incident
```

The last two are easy to miss because neither is "history" in the CHANGELOG/ROADMAP/rfcs sense:
`v1-readiness-audit.md` is a dated report whose finding ("eight releases 0.31.0 -> 0.38.0 with no
published-crate change") is the evidence base for RFC-074's MD-2 — retargeting it would corrupt
the measurement that motivated this entire release. `check-release-docs.sh` has a code comment
describing the `0.38.0` incident that motivated its ROADMAP parity guard; that comment's whole
point is to name the version where the incident happened.

## 7. Release-checklist — do not touch this release

`docs/src/contributing/release-checklist.md` has **zero** `matten-stats` mentions today, and this release
**leaves it that way**. Teaching it about the fifth crate (per-crate command blocks; publish-ordering)
implies `matten-stats` is a normal part of the publish sequence — it is not, in this release, because its
first publish is deferred (§2, RFC-079 §3). Do this work in whatever release actually first publishes
`matten-stats`, not here.

`cargo package --workspace` already exercises all five crates regardless of this document (packaging is
not publishing), so nothing about verification (§9) depends on this file changing.

## 8. CHANGELOG `[0.39.0]`

Follow RFC-079 §6.1's structure. Two things must appear, and a reviewer will check each:

```text
[ ] train_test_split_seeded named, with its one-line behavior
[ ] NOT a single mention of matten-stats — not Added, not deferred, not a forward-looking note
```

**Do not add an RFC-075 §3.1 local-tool-only justification.** This release changes a published crate
(`matten-mlprep`), so that rule does not apply — adding it would be actively wrong and would misdescribe
the release.

**Do not add a `matten-stats` line "for completeness."** The instinct to mention it (even as "implemented,
not yet published") is understandable but wrong here: RFC-079 §3 was written specifically so this decision
would not leak into the record by implication. A `[0.39.0]` CHANGELOG entry describes `0.39.0`'s shipped
contents, full stop.

## 9. Verification

```bash
cargo fmt --all --check
cargo fmt --manifest-path tools/matten-report/Cargo.toml --check
cargo fmt --manifest-path tools/matten-migrate/Cargo.toml --check
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings
cargo clippy --all-targets --no-default-features --features dynamic -- -D warnings
cargo test --all-targets
cargo test --doc --all-features
# complete feature matrix — release-checklist.md §2
cargo +1.85.0 build && cargo +1.85.0 test --all-features
cargo package --workspace                      # FIVE crates
bash scripts/check-release-docs.sh
bash scripts/check-core-dependency-boundary.sh
bash scripts/check-published-dependency-isolation.sh
bash scripts/check-matten-data-scope.sh
bash scripts/check-benchmark-dependency-sync.sh
bash scripts/check-streaming-scope.sh
bash tools/matten-report/tests/process-boundary.sh
bash tools/matten-report/tests/module-boundaries.sh
bash tools/matten-report/tests/module-boundaries.sh --self-test
mdbook build docs && rm -rf docs/book
git diff --check
```

Plus the per-crate example runs, now including `stats_covariance`, `stats_correlation`, `stats_quantile`.

## 10. What the review request must report

```text
[ ] the owner's version-bump confirmation, and the §3 disposition (deferred, external read pending)
[ ] cargo metadata: five crates, all 0.39.0 (matten-stats included, unpublished)
[ ] git diff --name-only -- 'crates/*/src/'  (expect exactly one file)
[ ] git grep -nE "0\.38\.(0|x)"  with every remaining hit accounted for as history
[ ] confirmation NO pre-1.0 wording and NO compatibility.md v1.0 section changed
[ ] CHANGELOG: the two required statements from §8, and confirmation of ZERO matten-stats mentions
[ ] confirmation release-checklist.md is untouched (§7)
[ ] cargo package --workspace packaging five crates (packaging, not publishing)
[ ] the five report-tool anchors and 69 tool tests unchanged
[ ] the full §9 gate set, including MSRV
[ ] confirmation no tag was created and nothing was published, matten-stats specifically included
```

## 11. Known pitfalls

1. **Treating this like RFC-076.** That is the v1.0 prep. Sweeping `pre-1.0` wording here would falsely
   claim a compatibility promise the project has not made.
2. **Missing `crates/matten-stats/README.md`** in the retarget — it is new and absent from older lists.
3. **Retargeting `compatibility.md`'s per-family history** — those old versions are correct history.
4. **Hand-editing `Cargo.lock`** — regenerate with `cargo check --workspace`.
5. **Adding the RFC-075 §3.1 justification** — does not apply to a published-crate release.
6. **Implying `matten-stats` ships this release** — it does not (§2). Do not soften `Experimental`
   for it either, since that question does not even arise until it actually publishes.
7. **Bumping without the owner's explicit confirmation** — the `3.22.0` standing rule.
8. **Any `.rs` change beyond the doc pin** — the one-file check catches it.
9. **Retargeting a historical record** — `docs/design/v1-readiness-audit.md`'s dated findings and
   `scripts/check-release-docs.sh`'s incident-recording comment must keep their `0.38` references.
   Rewriting either is silent damage: no gate checks either file's content, so a false retarget
   would corrupt the record and nothing would fail.
10. **Mentioning `matten-stats` in the CHANGELOG or touching the release checklist "for
    completeness."** Both are silent damage in the other direction from #6: they would record a
    publish decision (deferred) as if it were closer to settled than it is. Leave both alone.

## 12. Review stop

Acceptance makes the release-prep commit a commit point. **Tag and publish remain a separate
owner-authorized step (RFC-079 §12), and that publish list explicitly excludes `matten-stats`.**

Note for that step: `matten-stats`'s first publication, whenever it happens, claims the crate name on
crates.io permanently — that is why it is deferred out of this release rather than bundled in. Everything
this release *does* publish is a routine additive `0.x` bump to an already-published crate.

**And before RFC-076 is ever executed:** it assumes four crates and is already stale (RFC-079 §9),
independent of when `matten-stats` publishes. It also rests on a question RFC-067 never answered — whether
a v1.0 family may include an `Experimental` crate.
