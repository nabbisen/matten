# RFC-076 v1.0 Release Preparation: Implementation Handoff

**Project:** `matten`
**Related RFC:** RFC-076 (design authority)
**Document kind:** Detailed release-preparation handoff
**Status:** Drafted for review; implementation unauthorized until RFC-076 and this handoff are accepted
**Date:** 2026-07-28

---

## 1. Purpose

Translate RFC-076 into an executable, reviewable release-preparation slice producing the `1.0.0` release
commit. RFC-076 is the design authority; where the two differ, RFC-076 wins and this handoff is wrong.

**This handoff does not authorize tagging or publishing.** It ends at a reviewed release-prep commit.

## 2. Preconditions

Do not start until all of these hold:

```text
RFC-076 accepted and this handoff accepted
working tree clean at the accepted base commit
RFC-075 closed (done) — MD-2 resolved, serde declared, maturity table recorded
RFC-075 §3.1 rule present in release-checklist.md §7 and CHANGELOG.md conventions
0.38.0 released, tagged, and status-aligned
```

## 3. Step 0 — Tooling prerequisite (resolved: not required)

`cargo public-api` was blocked in this environment (`cargo-public-api` not installed; no nightly
toolchain present — `rustup toolchain list` shows 9 toolchains, none nightly, and `cargo public-api`
needs nightly rustdoc JSON output). Per RFC-076's original §4.1, that blocker was reported rather than
silently skipped.

**Maintainer decision (2026-07-28, RFC-076 §4.1 revised): `cargo public-api` is not required for this
release.** The rationale is recorded in RFC-076 §4.1 — the automated root-export allowlist guard, the
hand-maintained and independently re-audited `public-api-snapshot.md`, and eight releases of zero
functional churn already provide the verification this step existed to add, and a one-time nightly
dependency was judged disproportionate. This step is **skipped**, not blocked; do not attempt to install
`cargo-public-api` or a nightly toolchain as part of this implementation.

**Do not** add `cargo-public-api` to any manifest, CI workflow, or `scripts/`. The release checklist's
`cargo public-api` step (§5) remains as optional future guidance, unchanged and unused by this release.

## 4. Scope

### In scope

```text
recording the decision not to run cargo public-api, with rationale (§3, RFC-076 §4.1)
compatibility.md rewrite (1.0 statement, incl. the §4.2 doc(hidden) items now covered)
the 19-site pre-1.0 / 0.x sweep across 5 phrasing families
public-api-snapshot.md retarget
CHANGELOG.md [1.0.0] entry
version bump 0.38.0 -> 1.0.0 (Cargo.toml, Cargo.lock, lib.rs doc pin)
ROADMAP.md + rfcs/README.md + rfcs/handoffs/README.md tracking
RFC-076 status update
```

### Out of scope — a diff touching any of these is a defect

```text
any crates/*/src/*.rs change except the lib.rs doc-comment install pin
any tools/ change
any Cargo dependency, feature, edition, or rust-version change
maturity-label promotion
tag creation or crates.io publish
new tests, new examples, new public items
```

## 5. Implementation order

Execute in this order.

1. **Step 0**, above — already resolved (not required); no action needed, just don't skip recording it.
2. **compatibility.md rewrite** (§6 below) — do this before the sweep, because the sweep's replacement
   wording must be consistent with the new compatibility statement.
3. **The 19-site pre-1.0 / 0.x sweep** (§7 below), including the `migration.md` reconciliation.
4. **The current-family version-string retarget** (§9.1 below) — 29 strings across 14 files,
   `0.38.0`/`0.38.x` -> `1.0.0`/`1.0.x`. Do this after the sweep so both passes over the same
   files happen in a predictable order and neither undoes the other's edit.
5. **public-api-snapshot.md** — retarget the header from "current v0.38 release family" to 1.0.
6. **CHANGELOG `[1.0.0]`** (§8 below).
7. **Version bump** (§9 below).
8. **Tracking** — ROADMAP Status/history row, `rfcs/README.md`, `rfcs/handoffs/README.md`, RFC-076 status.
9. **One formatting pass**, then the full gate set (§11).

Do not interleave. A single formatting pass at the end keeps the diff reviewable.

## 6. compatibility.md — the 1.0 statement

Replace the forward-looking "v1.0 requirements" checklist (it becomes stale the moment 1.0 ships) with a
statement structured as:

```text
What SemVer guarantees from 1.0.0
  root exports: Tensor, MattenError, DataFormat, MattenLimits, SliceBuilder,
    Element / NumericPolicy under feature "dynamic", and IntoSliceRange /
    SliceConvert / SliceSpecRepr — the #[doc(hidden)] sealed slice-plumbing
    items, covered per RFC-076 §4.2 (revised)
  the MattenError variant set
  the panic/Result boundary zones
  the canonical serde object form {"shape":[...],"data":[...]} (RFC-075 §3.2)
  documented feature-flag behavior

What is explicitly NOT covered
  the rank-1/rank-2 nested-array convenience input forms
  CSV, which remains ingestion rather than canonical serialization
  Serialize's dynamic-tensor error behavior

MSRV policy       1.85 at release; how MSRV changes will be versioned
Lock-step policy  RFC-030 unchanged; RFC-075 §3.1 local-tool-only justification still in force
Maturity          matten-mlprep and matten-data ship at production-ready candidate
```

**The doc(hidden) inclusion must be written, not implied.** RFC-076 §4.2 (revised) covers
`IntoSliceRange`/`SliceConvert`/`SliceSpecRepr` under SemVer despite their `#[doc(hidden)]` marker — state
this explicitly so a reader does not assume "hidden" means "uncovered." A user who today writes
`fn f<R: matten::IntoSliceRange>(r: R)` now has the same stability guarantee as any other public item.

Keep `compatibility.md`'s existing per-family history section intact; it legitimately carries historical
version references and the release-docs guard tolerates it.

## 7. The pre-1.0 / 0.x sweep — 19 sites, 5 families

Five families need different treatment. **Do not find-and-replace.**

| # | Site | Family | Replacement guidance |
|---|---|---|---|
| 1 | `crates/matten/README.md:28` | status banner | "active pre-1.0 development" → a 1.0 status statement; keep the honest "numeric core is strong" framing |
| 2 | `crates/matten-ndarray/README.md:10` | status banner | drop "pre-1.0, so pin the minor version"; state SemVer applies |
| 3 | `crates/matten-mlprep/README.md:9` | status banner | same; keep the candidate label |
| 4 | `crates/matten-data/README.md:9` | status banner | same; keep the candidate label |
| 5 | `crates/matten-ndarray/README.md:102` | SemVer policy | currently says a `0.x` minor **may break** — false after 1.0; state the SemVer guarantee |
| 6 | `crates/matten-mlprep/README.md:106` | SemVer policy | same |
| 7 | `crates/matten-data/README.md:132` | SemVer policy | same |
| 8 | `crates/matten-ndarray/README.md:106` | dependency note | "broad pre-1.0 core requirement" → whatever the 1.0.0 manifests actually express |
| 9 | `crates/matten-mlprep/README.md:110` | dependency note | same |
| 10 | `crates/matten-data/README.md:133` | dependency note | same |
| 11 | `docs/src/reference/compatibility.md:68` | dependency note | same; must agree with §6 |
| 12 | `crates/matten-data/README.md:60` | **maturity (mixed)** | **highest risk.** Reads "production-ready candidate. The table-to-Tensor API is mostly stable but…". Drop the pre-1.0 hedge, **keep `production-ready candidate` exactly** — RFC-076 §5 and RFC-067 require the label to survive |
| 13 | `README.md:27` | root crate table | drop `0.38.x family` → `1.0.x family`; drop `stable (v0.x)` → a maturity word that doesn't restate a version line (e.g. `stable`) |
| 14 | `docs/src/reference/compatibility.md:50` | standalone compatibility-policy section | `## v0.x compatibility` heading is **superseded**, not retargeted, by the new §6 statement — remove or fold into it |
| 15 | `docs/src/reference/compatibility.md:52` | standalone compatibility-policy section | "`matten` is on the `v0.x` line... Breaking changes are allowed..." — this claim is **false** at 1.0 and must not survive next to the new SemVer promise |
| 16 | `docs/src/reference/migration.md:129` | standalone compatibility-policy section (reconcile, do not retarget) | `## Compatibility promise (v0.x)` duplicates `compatibility.md`'s subject in a second, independent location. **Replace this section with a short pointer** stating the compatibility promise lives in `compatibility.md`, linking there, rather than maintaining a second independently-worded promise that can drift |
| 17 | `docs/src/reference/migration.md:131` | standalone compatibility-policy section (reconcile, do not retarget) | "During `v0.x`, API changes are allowed but minimised..." plus the closing "`v1.0.0` requires explicit maintainer confirmation" line — both fall away once site 16's section is replaced by the pointer; this *is* the 1.0 release |
| 18 | `docs/src/contributing/release-checklist.md:188` | standalone compatibility-policy section | "During v0.x, patch releases (0.13.x) should not introduce new public API..." is stale process guidance once the project is past `0.x`. Rewrite for SemVer-major release planning (a breaking change requires a major version bump, not a `0.x` minor) |
| 19 | `docs/src/examples/data.md:176` | **maturity (mixed)**, same family as site 12 | Near-verbatim duplicate of site 12: "production-ready candidate. The table-to-Tensor API is mostly stable but pre-1.0; pin the release explicitly." Drop *"mostly stable but pre-1.0; pin the release explicitly"*; **keep `Production-ready candidate` exactly**. Note: §9.1's retarget independently changes `0.38.x family` → `1.0.x family` on this same line — that edit and this sweep edit are both required; neither substitutes for the other |

`docs/src/reference/public-api-snapshot.md`'s "current v0.38 release family" header retarget is handled by
§5 step 5 above, not counted in this table's 19 — it is a single-line header retarget, not a
compatibility-policy statement, and doing it twice risks two people touching the same line differently.

Site 12 remains the one most likely to be damaged: a careless edit either deletes the candidate label
(silently promoting the crate, violating RFC-067) or leaves a pre-1.0 hedge attached to a 1.0 release.

Sites 16-17 (`migration.md`) are the second-highest risk: deleting the section without adding the pointer
leaves no compatibility-promise context there at all; rewriting it in place instead of pointing at
`compatibility.md` recreates the two-promises defect this fix exists to close.

After the sweep, run the **same** grep the RFC's §6.2 verification uses — do not use a narrower one:

```bash
grep -rn "pre-1.0\|pre 1.0\|0\.x" README.md crates/*/README.md docs/src/
```

Account for **every** remaining hit as either deliberate history or an accepted false positive — for
example a third-party crate version string (`candle_core = "0.x"` in a migration playbook) or a guard
script's own description of the pattern it checks for. Zero unexplained hits.

## 8. CHANGELOG `[1.0.0]`

Structure it around the fact that this is a *commitment*, not a feature release:

```text
## [1.0.0] - <date>

<lead: 1.0.0 is a compatibility commitment. No public API, runtime, dependency,
 feature-flag, or MSRV change from 0.38.0.>

### Changed
- Version 0.38.0 -> 1.0.0. The public surface is now committed under SemVer;
  see docs/src/reference/compatibility.md for exactly what is and is not covered.
- matten-mlprep ships at production-ready candidate (train_test_split is
  ordered-only; train_test_split_seeded remains planned, RFC-024 §6).
- matten-data ships at production-ready candidate (CSV-only ingestion; not a
  dataframe engine; no streaming).

### Version
- Release bump 0.38.0 -> 1.0.0. Lock-step family versioning (RFC-030) still applies.
```

**RFC-067 requires both candidate labels be stated explicitly** so no wording silently promotes either
crate. Do not soften them.

This release is **not** local-tool-only, so the RFC-075 §3.1 justification line does **not** apply here.

## 9. Version bump

```text
Cargo.toml [workspace.package]  version = "0.38.0" -> "1.0.0"
Cargo.lock                       matten, matten-ndarray, matten-mlprep, matten-data -> 1.0.0
crates/matten/src/lib.rs         //! ```toml install pin -> matten = { version = "1.0.0", ... }
edition = "2024"                 unchanged
rust-version = "1.85"            unchanged
```

Let Cargo update the lock (`cargo check --workspace`); do not hand-edit it. Then verify:

```bash
cargo metadata --format-version 1 --no-deps    # all four at 1.0.0
cargo check --workspace                        # proves the lock is in sync
git diff --name-only -- 'crates/*/src/'        # must print ONLY crates/matten/src/lib.rs
```

That last command is the cheapest defect detector in this whole slice.

### 9.1 Current-family version-string retarget (implementation-order step 4)

Beyond the three files above, every current-family `0.38.0` / `0.38.x` string in tracked
user-facing docs retargets to `1.0.0` / `1.0.x`. Measured (`git grep -c -E "0\.38\.(0|x)"`):

```text
README.md                                                 10
crates/matten/README.md                                    1
crates/matten-data/README.md                                1
crates/matten-mlprep/README.md                              3
crates/matten-ndarray/README.md                             3
crates/matten/src/lib.rs                                    1  (covered above, listed for completeness)
docs/src/contributing/architecture.md                       1
docs/src/contributing/release-checklist.md                  1
docs/src/examples/data.md                                   3
docs/src/quick-start.md                                     1
docs/src/reference/boundary.md                              1
docs/src/reference/compatibility.md                         1
docs/src/reference/dynamic.md                               1
docs/src/reference/public-api-snapshot.md                   1
                                                    total   29
```

29 strings across 14 files (matching RFC-076 §7.1 exactly). The family-suffix shape changes
(`0.38.x family` → `1.0.x family`), not just the patch number. `compatibility.md`'s per-family
history section is exempt — this retarget touches only the live install-pin/current-family
lines. `docs/src/examples/data.md:176` needs **both** this retarget (the `0.38.x family` part)
and the sweep's site 19 (the `pre-1.0` hedge) — see §7's site 19 row and pitfall 1.

After this step: `git grep -nE "0\.38\.(0|x)"` must return only historical hits (`CHANGELOG.md`,
`ROADMAP.md`, `rfcs/`).

## 10. Invariants — any movement is a defect

```text
local-tool anchors (this release touches no tools/ code):
  help                        1,613  0daaf8e57e0cc4471baa30d6b05bdef76efb265b665aa0ad3fd51e0415286930
  fixed-demo Markdown           404  bdb6014f637455ed235af7eedcda0872b9161f76e362661bbbbe3fe8247e4c22
  fixed-demo JSON               952  6491d3856293572e80f0388be6002703178336447f24afb330087c82ad680fac
  input success JSON          3,176  84ec3f794c5ccf225bcf5fe88aa1f3d2043179492d776940fe5206c14cae7767
  input conversion-error JSON 3,077  f7c7125819e88635e21ab6c4a4769aee0f3a4ba3dc0e16dbfc20c1c82f267751

69 matten-report tests, by name
module-boundary normal + --self-test green
500-line ceiling green
ROADMAP header == latest history row (guard-enforced)
MSRV 1.85
root exports unchanged (7 visible + 3 doc(hidden))
maturity labels unchanged
```

## 11. Required verification

```bash
cargo fmt --all --check
cargo fmt --manifest-path tools/matten-report/Cargo.toml --check
cargo fmt --manifest-path tools/matten-migrate/Cargo.toml --check
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings
cargo clippy --all-targets --no-default-features --features dynamic -- -D warnings
cargo test --all-targets
cargo test --doc --all-features
# full feature matrix — release-checklist.md §2 (every profile)
cargo +1.85.0 build && cargo +1.85.0 test --all-features
# cargo public-api intentionally NOT run — see §3 (maintainer decision, 2026-07-28)
cargo package --workspace
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

Plus the full report-tool smoke matrix from the release checklist.

## 12. What the review request must report

State all of these proactively:

```text
[ ] confirmation cargo public-api was intentionally not run, citing RFC-076 §4.1's recorded
    maintainer decision and rationale -- not presented as a skipped or forgotten step
[ ] confirmation the three doc(hidden) items are listed under "what SemVer guarantees" in
    compatibility.md, per the revised §4.2 decision
[ ] `git diff --name-only -- 'crates/*/src/'` output (expected: exactly one file)
[ ] cargo metadata single-version 1.0.0 + cargo check --workspace clean
[ ] all 19 sweep sites, with site 12's and site 19's candidate labels both shown
    intact and the migration.md reconciliation (sites 16-17) resolved by pointer, not rewrite
[ ] all 29 current-family version strings across 14 files retargeted; grep confirms
    only historical hits remain (CHANGELOG.md, ROADMAP.md, rfcs/)
[ ] grep for residual pre-1.0 / 0.x hits (RFC's stronger form), each accounted for
[ ] CHANGELOG states both candidate labels explicitly
[ ] the five local-tool anchors and the 69-test count, unchanged
[ ] the complete §11 gate set, with results
[ ] confirmation no tag was created and nothing was published
```

## 13. Known pitfalls

1. **Site 12 / site 19 damage** (§7) — silently promoting `matten-data` by deleting its candidate
   label at either `crates/matten-data/README.md:60` or the near-verbatim duplicate at
   `docs/src/examples/data.md:176`. A line can require **both** a sweep edit (drop the pre-1.0
   hedge) and a retarget edit (§9.1's `0.38.x family` → `1.0.x family`) — applying only the
   retarget leaves the pre-1.0 claim standing in a `1.0.0` release; applying only the sweep leaves
   a stale family string. `data.md:176` is the worked example.
2. **Hand-editing `Cargo.lock`** — always regenerate via `cargo check --workspace`.
3. **Extra `.rs` churn** — an "improvement" made in passing during the sweep. Check with the
   `git diff --name-only -- 'crates/*/src/'` command; expected output is exactly one path.
4. **Presenting the cargo-public-api decision as a skip or a forgotten step rather than a recorded
   decision** — it is intentionally not run (§3, RFC-076 §4.1); say so explicitly with the
   rationale in the implementation review request, not silently.
5. **Committing `docs/book`**. It is generated; keep it out.
6. **Applying the RFC-075 §3.1 justification line** — it does not apply to this release (§8).
7. **ROADMAP header bumped without a history row** — now guard-caught, but check anyway.
8. **Doing the current-family retarget (§9.1) before the sweep (§7)**, or interleaving them —
   do the sweep first, then the retarget, so a single file is not touched by two passes out of order.
9. **Rewriting `migration.md`'s compatibility section in place instead of pointing at
   `compatibility.md`** — recreates the two-compatibility-promises defect (sites 16-17).
10. **Using the narrower `pre-1.0\|pre 1.0` grep instead of `pre-1.0\|pre 1.0\|0\.x`** — the
    narrower form misses the six `0.x` sites this handoff was revised to add; always use the RFC's
    stronger form (§7).

## 14. Acceptance criteria

RFC-076 §10 governs. This handoff adds no criteria of its own; it only sequences them.

## 15. Review stop

Acceptance of the implementation makes the release-prep commit a commit point. **Tag and publish remain a
separate maintainer-authorized step (RFC-076 §12).** Note that publishing is irreversible: crates.io permits
yanking within a limited window, but a yank does not retract a compatibility promise already consumed
downstream. The implementation review is the last cheap point to reconsider.

**Publish order, when Unit 3 is authorized:** `matten` first (all three companions depend on it via
`workspace = true`); then `matten-ndarray`, `matten-mlprep`, `matten-data`, which are mutually independent
and may go in any order.

---

**Reviewer note:** RFC-076 and this handoff were drafted by the party who has acted as independent auditor
on the RFC-069→075 line. Both should receive review from a different reviewer or an explicitly fresh pass.
