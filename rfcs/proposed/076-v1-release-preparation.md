# RFC-076: v1.0 Release Preparation

**Status:** Proposed; release-preparation specification only, no release executed
**Target:** `1.0.0` release preparation; tag and publish remain a separate authorized step
**Theme:** Execute every prerequisite RFC-074 and RFC-075 left for a release-prep unit, and
specify the `1.0.0` compatibility commitment itself
**Depends on:** RFC-002, RFC-015, RFC-022, RFC-030, RFC-057, RFC-058, RFC-059, RFC-066, RFC-067, RFC-074, RFC-075
**Related:** RFC-009, RFC-024, RFC-037, RFC-040, RFC-041, RFC-042, RFC-071

---

> **Version-base drift note (added by the RFC-081 inventory refresh, 2026-07-28).** This RFC was written
> against `0.38.0`; the current line is `0.39.0` (two releases have shipped since: RFC-079, RFC-080).
> Version-specific figures below (the 19-site pre-1.0 sweep, the 29-string retarget count and file list)
> are **not re-measured here** — they must be re-measured when this RFC is next revised for execution,
> or they will be stale again by the time that revision runs. This refresh corrects only the family-size
> and maturity-label assumptions (§6 below), not the version-string figures.

## 1. Summary

RFC-074 found the published family **conditionally ready** for `1.0.0`; RFC-075 resolved MD-2, declared
the serde canonical format stable, and recorded the RFC-067 family maturity table. Both explicitly deferred
release preparation to a separate unit. This RFC is that unit.

It specifies exactly what the `1.0.0` release-preparation change will contain:

```text
decide not to run cargo public-api for this release, recorded explicitly with rationale (§4.1)
cover the three #[doc(hidden)] slice-plumbing items under the 1.0 promise (§4.2, revised)
reproduce the RFC-067 family maturity table in full, as RFC-067 requires
replace compatibility.md's "v1.0 requirements" list with an actual 1.0 compatibility statement
sweep every pre-1.0 / 0.x compatibility-policy wording site (19 sites across 9 files, §6.2)
retarget every current-family version string, 0.38.0 / 0.38.x -> 1.0.0 / 1.0.x
  (29 strings across 14 files, §7)
bump the lock-step family version 0.38.0 -> 1.0.0
add CHANGELOG release notes stating each companion's actual maturity label explicitly
  (matten-mlprep is no longer a candidate — RFC-080 — and matten-stats ships at
  production-ready candidate, having taken Exit A via RFC-084)
run the complete gate set RFC-074's documentation-only re-audit did not run
```

**This RFC does not tag, publish, or release.** Acceptance authorizes preparing the release commit;
tagging `1.0.0` and publishing to crates.io remain a separate maintainer-authorized step (§12).

## 2. Motivation

The `1.0.0` decision has been made deliberately, not by drift: RFC-074 recommended two explicit paths, the
owner chose Path B, and RFC-075 closed the policy prerequisites. What remains is mechanical but
consequential, and it differs from every prior release in one respect that deserves stating plainly:

**Every release so far has been effectively reversible.** Under `0.x`, SemVer promises nothing, and this
project has used that freedom — nineteen `pre-1.0`/`0.x` compatibility-policy statements across nine
files say so explicitly (§6.2). `1.0.0` is not reversible. Once published, the root API surface, the
`MattenError` variant set, the panic/Result boundary, the canonical serde form, and the feature-flag
contract are committed under SemVer.

The evidence that this is safe is unusually strong: `git diff 0.31.0..HEAD -- 'crates/*/src/'` touches one
file, and every changed line is a doc-comment version string. Eight consecutive releases of zero functional
change is a stability record most projects cannot demonstrate mechanically. But that evidence supports the
*decision*; it does not substitute for the *verification*, which RFC-074 deliberately did not perform
because it was a documentation-only audit. This RFC performs it.

**PRECONDITION (RFC-081 §3, added by the RFC-081 inventory refresh): DISCHARGED (RFC-084, 2026-07-29).**
No crate in the lock-step family may be labelled `Experimental` at `1.0.0`. `matten-stats` was the only
such crate; it has since taken **Exit A** — promoted to production-ready candidate by its own promotion
RFC (RFC-084) — so this precondition no longer blocks RFC-076's execution. This does **not** mean RFC-076
is now authorized to proceed: it remains deferred for the separate, unrelated reason that **v1.0 is not
currently wanted** (owner decision, 2026-07-29). The family-size and maturity-label corrections elsewhere
in this refresh (§5-§7) still need re-verification against the now-six-function `matten-stats` surface and
the current version line whenever this RFC is next revised for execution.

## 3. Scope

### In scope

```text
recording the decision not to run cargo public-api for this release, with rationale (§4.1)
an explicit decision on #[doc(hidden)] slice plumbing under the 1.0 promise — covered (§4.2)
the 1.0 compatibility statement in docs/src/reference/compatibility.md
the pre-1.0 wording sweep across all identified sites
retarget every current-family version string (0.38.0 / 0.38.x -> 1.0.0 / 1.0.x)
  across the 14 affected files (§7)
the RFC-067 family maturity table, reproduced in full
version bump 0.39.0 -> 1.0.0 (Cargo.toml + Cargo.lock, all five crates)
CHANGELOG.md 1.0.0 release notes
docs/src/reference/public-api-snapshot.md retarget to 1.0
ROADMAP.md and RFC index tracking
the complete verification gate set (§8)
```

### Out of scope

```text
tag creation
crates.io publishing
any public API change, addition, or removal
any dependency change
MSRV change
companion maturity promotion (matten-mlprep and matten-data were both promoted to
  production-ready separately, by RFC-080 and RFC-085, prior to this RFC)
any tools/matten-report or tools/matten-migrate change
any backlog theme from RFC-070's remaining-themes table
new features of any kind
running cargo public-api (§4.1, revised: decided not required for this release)
sealing, renaming, or removing the #[doc(hidden)] slice-plumbing items themselves
  (§4.2 decides their compatibility status only)
```

**A release-prep diff that changes behavior is a defect.** The only expected `.rs` change is the install-pin
string inside the `//!` doc comment in `crates/matten/src/lib.rs` — the invariant that has held for eight
consecutive releases.

## 4. Prerequisite Closure

### 4.1 `cargo public-api` (revised — maintainer decision, 2026-07-28: not run for this release)

NF-2 was closed as *documentation* by the RFC-074 review's H0: `docs/src/contributing/release-checklist.md`
§5 names the step as a manual pre-release option. This RFC originally required `cargo public-api` to be run
for real, for the first time in the project's history, before `1.0.0`. The implementation handoff's Step 0
found it unavailable in the working environment (`cargo-public-api` not installed; no nightly toolchain
present, which `cargo public-api` requires for rustdoc JSON output) and, per this RFC's own instruction not
to skip the step silently, reported the blocker rather than working around it.

**Decision: do not run `cargo public-api` as a prerequisite for this release.** The maintainer judged it
not required, given the verification already in place:

```text
the public API root-export allowlist is enforced automatically on every commit by
  scripts/check-release-docs.sh (grep-based, checked against crates/matten/src/lib.rs)
docs/src/reference/public-api-snapshot.md is maintained by hand and cross-checked against
  source at every release; RFC-074's re-audit independently re-verified it item by item
zero functional churn in any published crate's src/ since 0.31.0 (RFC-074), so there is no
  unreviewed recent change for a snapshot tool to catch that manual review has not already seen
adding a nightly-toolchain dependency for a one-time check is a real environmental cost this
  project does not otherwise carry (CI and the documented MSRV are both stable-only)
```

This does not retract NF-2's original concern that manual review can miss something a tool would catch —
that risk is accepted here, not argued away. `cargo public-api` remains available to adopt as an ongoing
gate for a future release if the maintainer later wants tool-verified snapshots; nothing in this decision
forecloses that. `docs/src/contributing/release-checklist.md` §5's `cargo public-api` step is retained as
optional guidance, not renamed or removed, since a future release may still choose to use it.

Because the three `#[doc(hidden)]` slice-plumbing items are now **covered** by the `1.0.0` promise (§4.2,
revised), the specific risk `cargo public-api` was expected to surface at this release — those three items
appearing in a snapshot diff — is moot: covering them does not depend on first enumerating them by tool, since
their identity is already known and stated explicitly in §4.2 and `compatibility.md`.

### 4.2 Decision: `#[doc(hidden)]` slice plumbing under the 1.0 promise

`crates/matten/src/lib.rs:113-114` re-exports three items behind `#[doc(hidden)]`:

```rust
#[doc(hidden)]
pub use crate::slice::{IntoSliceRange, SliceConvert, SliceSpecRepr};
```

Established facts, verified in source:

```text
they are already sealed — SliceConvert: sealed::Sealed, IntoSliceRange: SliceConvert,
  with sealed::Sealed implemented only for the five std range types
  (Range, RangeFrom, RangeTo, RangeFull, RangeInclusive)
downstream crates therefore cannot implement them
they must remain pub: IntoSliceRange appears in a real user-facing generic bound,
  SliceBuilder::range<R: IntoSliceRange>(r) — the pub-ness is a visibility-chain
  requirement, not an oversight
docs/src/reference/compatibility.md already describes them as
  "hidden implementation plumbing" and "a visibility-chain artefact"
zero churn since 0.31.0
```

**Decision (revised — maintainer decision, 2026-07-28): cover the three items under the `1.0.0` SemVer
compatibility promise, stated explicitly in `compatibility.md` rather than left to inference.**

This RFC originally proposed excluding the three items. Both review rounds verified the exclusion
argument's underlying facts as correct but were explicit that they could not independently adjudicate the
judgment call itself — "the one irreversible decision in the release," per the rereview — since author and
reviewer were the same party throughout. No independent read of the exclusion argument was available. The
maintainer chose the conservative fallback both reviews named as the default absent that independent read:
**cover, not exclude.**

Rationale: zero churn since `0.31.0` means covering costs nothing today, and widening a promise later is
always possible while narrowing one after `1.0.0` is not. The `#[doc(hidden)]`/sealed-trait facts remain
true and are unaffected by this reversal — they simply no longer justify an exclusion, since the asymmetry
between the two options (excluding is riskier if wrong; including is riskier only in a mild long-term
sense — a permanently frozen trio of internal trait names) favors the option that does not require getting
an unreviewed judgment call right on the first, irreversible try.

**Alternative considered and rejected:** excluding the three items, as originally drafted. It would have
preserved more freedom to refactor slice internals, and the underlying facts (sealed traits, no possible
downstream implementation, existing "plumbing" documentation) still support it as a defensible position —
but it was rejected here specifically because it was never reviewed by anyone who did not also write the
argument for it, and it is the one decision in this release that cannot be revisited after publication.

## 5. RFC-067 Family Maturity Table

RFC-067 line 76 requires the **v1.0 release RFC** to *include* this table. RFC-075 §3.3 recorded it so the
answers were settled in advance; this RFC reproduces it in full, as required — citation alone would satisfy
RFC-067 by pointer rather than by content.

| Crate | Version at release | Maturity label | Public API stability | Candidate caveat | v1.0 family inclusion |
|---|---|---|---|---|---|
| `matten` | `1.0.0` | stable | Stable; zero functional churn `0.31.0`→`0.38.0` (RFC-074) | none | **Include** |
| `matten-ndarray` | `1.0.0` | production-ready (RFC-057) | Stable; zero churn | none | **Include** |
| `matten-mlprep` | `1.0.0` | production-ready (RFC-080) | Stable; zero churn | none — RFC-077 added `train_test_split_seeded`, closing the ordered-only-split caveat; RFC-080 promoted the crate on that basis, prior to this RFC | **Include** |
| `matten-data` | `1.0.0` | production-ready (RFC-085) | Stable; zero churn | none — RFC-085 closed RFC-059 §6's deferred full-production review; scope stays CSV-only, not a dataframe engine (RFC-042 scope lock, CI-enforced by `scripts/check-matten-data-scope.sh`; RFC-037) | **Include** |
| `matten-stats` | `1.0.0` | production-ready candidate (RFC-084) | Published `0.39.0`; surface settled at six functions (RFC-083) | Estimator conventions per function, stated explicitly and verified by execution (RFC-083 §4.1); no external statistical review obtained — candidate label carries that, not a hidden gap | **Include, at candidate label** — RFC-081 §3's precondition is discharged: `matten-stats` took Exit A (RFC-084), prior to this RFC |

Per-crate RFC-067 checklist (lines 87-91), and its `matten-stats` counterpart under RFC-081 §3:

```text
matten-mlprep
  RFC-067 candidate conditions apply?             no — promoted to production-ready by
                                                   RFC-080, prior to this RFC. RFC-067's
                                                   conditions govern candidate-labelled
                                                   crates; matten-mlprep is no longer one
  separate promotion RFC required before v1.0?    already happened (RFC-080) — not by
                                                   this RFC, and not required again

matten-data
  RFC-067 candidate conditions apply?             no — promoted to production-ready by
                                                   RFC-085, prior to this RFC. RFC-067's
                                                   conditions govern candidate-labelled
                                                   crates; matten-data is no longer one
  separate promotion RFC required before v1.0?    already happened (RFC-085) — not by
                                                   this RFC, and not required again

matten-stats
  RFC-067 candidate conditions apply?             yes — promoted to production-ready candidate
                                                   by RFC-084, prior to this RFC (was Experimental,
                                                   which RFC-067 never addressed; now resolved)
  candidate label an acceptable documented caveat? yes — no usage history, no external ddof
                                                    review; both are what the candidate label
                                                    exists to carry, per RFC-084 §3
  separate promotion RFC required before v1.0?     already happened (RFC-084) — not by
                                                    this RFC, and not required again
```

**`matten-data` enters at production-ready**, promoted separately by RFC-085 before this RFC
(closing RFC-059 §6's deferred full-production review) — no re-announcement of a promotion is
needed, only a correct label. **`matten-mlprep` enters at production-ready**, promoted separately
by RFC-080 before this RFC — no re-announcement of a promotion is needed, only a correct label.
**`matten-stats` enters at production-ready candidate**, promoted separately by RFC-084 before this
RFC (Exit A under RFC-081 §3) — no re-announcement of a promotion is needed, only a correct label.

## 6. Documentation Changes

### 6.1 The 1.0 compatibility statement

Replace `docs/src/reference/compatibility.md`'s "v1.0 requirements" list — a forward-looking checklist that
becomes stale the moment 1.0 ships — with an actual compatibility statement covering:

```text
what SemVer now guarantees: root exports (including the three #[doc(hidden)]
  slice-plumbing items IntoSliceRange/SliceConvert/SliceSpecRepr, covered per
  §4.2), the MattenError variant set, panic/Result boundary zones, the
  canonical serde object form (RFC-075 §3.2), and documented feature-flag
  behavior
what is explicitly NOT covered: the nested-array convenience input forms, CSV
  as ingestion rather than canonical serialization, and dynamic-tensor
  Serialize error behavior (RFC-075 §3.2)
MSRV policy: 1.85 at release; how MSRV changes will be versioned
lock-step family policy: all five crates share the version (RFC-030), and the
  RFC-075 §3.1 local-tool-only justification requirement remains in force
maturity labels: matten-mlprep ships at production-ready (promoted separately by
  RFC-080); matten-data ships at production-ready (promoted separately by
  RFC-085, closing RFC-059 §6's deferred full-production review); matten-stats
  ships at production-ready candidate (promoted separately by RFC-084, Exit A
  under RFC-081 §3), with the caveats in §5
```

### 6.2 The `pre-1.0` / `0.x` sweep — 19 sites across 9 files

This is a sweep, not a find-and-replace: **five** distinct phrasing families need different
treatment, one site (`matten-data/README.md:60`) mixes maturity wording with pre-1.0 wording and
must be edited without disturbing the candidate label §5 preserves, and one pair of sites
(`compatibility.md`/`migration.md`) currently states **two separate compatibility promises for
the same project** that must be reconciled into one, not just retargeted.

| Family | Sites |
|---|---|
| Status banner / blockquote | `crates/matten/README.md:28`, `crates/matten-ndarray/README.md:10`, `crates/matten-mlprep/README.md:9`, `crates/matten-data/README.md:9` |
| SemVer policy section | `crates/matten-ndarray/README.md:102`, `crates/matten-mlprep/README.md:106`, `crates/matten-data/README.md:132` |
| Dependency-requirement note | `crates/matten-ndarray/README.md:106`, `crates/matten-mlprep/README.md:110`, `crates/matten-data/README.md:133`, `docs/src/reference/compatibility.md:68` |
| Maturity statement (mixed) | `crates/matten-data/README.md:60`, `docs/src/examples/data.md:176` — both read *"production-ready candidate. The table-to-Tensor API is mostly stable but…"* (near-verbatim duplicate wording; the retarget in §7.1 changes `0.38.x family` → `1.0.x family` on `data.md:176` but does not remove the `pre-1.0` hedge — both edits are required on that line, neither substitutes for the other) |
| Root crate table | `README.md:27` — `0.38.x family` **and** `stable (v0.x)` in the same table cell |
| Standalone compatibility-policy section | `docs/src/reference/compatibility.md:50` (`## v0.x compatibility` heading), `docs/src/reference/compatibility.md:52` ("breaking changes are allowed" body), `docs/src/reference/migration.md:129` (`## Compatibility promise (v0.x)` heading), `docs/src/reference/migration.md:131` (duplicate compatibility claim, reconcile per §6.2 below — replace with a pointer to `compatibility.md`, do not retarget), `docs/src/contributing/release-checklist.md:188` (stale "During v0.x" process guidance) |

The SemVer-policy sites currently say a `0.x` minor bump may contain breaking changes — after 1.0 that is
false and must state the SemVer guarantee instead. The dependency-requirement sites describe a "broad
pre-1.0 core requirement"; the replacement must match whatever requirement the `1.0.0` manifests actually
express. The mixed maturity site must keep `production-ready candidate` intact (§5) while dropping the
pre-1.0 stability hedge. Also retarget `docs/src/reference/public-api-snapshot.md`'s "current v0.38 release
family" header, and `README.md:27`'s `stable (v0.x)` maturity cell.

**The `compatibility.md` / `migration.md` reconciliation is a decision, not a substitution.**
`compatibility.md`'s `## v0.x compatibility` section is superseded outright by the new §6.1
statement (rewrite the section, do not retarget its prose — its "breaking changes are allowed"
claim becomes false at 1.0). `migration.md`'s `## Compatibility promise (v0.x)` section
duplicates the same subject in a second, independent location; shipping 1.0 with two documents
each making their own compatibility claim is the actual defect. **Decision: `migration.md`'s
section is replaced with a short pointer to `compatibility.md` as the single canonical
compatibility statement**, rather than maintained as a second, independently-worded promise that
can drift out of sync on a future release. `release-checklist.md:188`'s "During v0.x, patch
releases..." guidance becomes stale process text at 1.0 and must be rewritten for the SemVer-major
release-planning reality (breaking changes require a major bump, not a `0.x` minor).

After the sweep, run the **same** command against both the RFC's post-sweep verification and the
implementation handoff's — they must not diverge:

```bash
grep -rn "pre-1.0\|pre 1.0\|0\.x" README.md crates/*/README.md docs/src/
```

Account for every remaining hit as intentional history or an accepted false positive (e.g. a
third-party crate version like `candle_core = "0.x"`, or a guard script's description of the
string pattern it checks for — neither is this project's own compatibility wording).

### 6.3 CHANGELOG

A `## [1.0.0]` entry that states:

```text
this is a compatibility commitment, not a feature release
no public API, runtime, dependency, feature-flag, or MSRV change from 0.38.0
matten-data's production-ready label predates this release (RFC-085, closing
  RFC-059 §6's deferred full-production review) and needs no re-announcement;
  matten-mlprep's production-ready label predates this release (RFC-080) and
  needs no re-announcement; matten-stats's production-ready candidate label
  likewise predates this release (RFC-084, Exit A under RFC-081 §3) and needs
  no re-announcement, named explicitly with its own caveat (RFC-067)
what the 1.0 promise covers and what it excludes, pointing at compatibility.md
```

## 7. Version And Metadata

```text
Cargo.toml [workspace.package] version = "0.38.0" -> "1.0.0"
Cargo.lock: matten, matten-ndarray, matten-mlprep, matten-data, matten-stats -> 1.0.0
crates/matten/src/lib.rs: install-pin doc-comment string -> "1.0.0"
edition 2024, rust-version 1.85: unchanged
```

### 7.1 Current-family version-string retarget

Beyond `Cargo.toml`/`Cargo.lock`/the `lib.rs` doc pin, every current-family
`0.38.0` / `0.38.x` string in tracked user-facing docs must retarget to
`1.0.0` / `1.0.x`. Measured now (`git grep -c -E "0\.38\.(0|x)"`):

```text
README.md                                                 10
crates/matten/README.md                                    1
crates/matten-data/README.md                                1
crates/matten-mlprep/README.md                              3
crates/matten-ndarray/README.md                             3
crates/matten/src/lib.rs                                    1
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

29 strings across 14 files. The family-suffix shape changes
(`0.38.x family` -> `1.0.x family`), not just the patch number.
`docs/src/reference/compatibility.md`'s per-family history section
legitimately retains historical `0.3x` references — this retarget touches
only the live install-pin/current-family lines, not that history.

Verify with `cargo metadata --format-version 1 --no-deps` (single family version) and
`cargo check --workspace` (lock in sync, not hand-edited). After the retarget,
`git grep -nE "0\.38\.(0|x)"` must return only historical hits (`CHANGELOG.md`,
`ROADMAP.md`, `rfcs/`).

## 8. Required Verification

The full set RFC-074's documentation-only re-audit explicitly did not run. All must pass and be reported:

```bash
cargo fmt --all --check
cargo fmt --manifest-path tools/matten-report/Cargo.toml --check
cargo fmt --manifest-path tools/matten-migrate/Cargo.toml --check
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings
cargo clippy --all-targets --no-default-features --features dynamic -- -D warnings
cargo test --all-targets
cargo test --doc --all-features
# complete feature matrix — docs/src/contributing/release-checklist.md §2
cargo +1.85.0 build && cargo +1.85.0 test --all-features
# cargo public-api intentionally NOT run — see §4.1 (maintainer decision, 2026-07-28)
cargo package --workspace       # must package FIVE crates
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

Local-tool anchors must remain unchanged (help `1,613`/`0daaf8e5…`, Markdown `404`/`bdb6014f…`, fixed JSON
`952`/`6491d385…`, input success `3,176`/`84ec3f79…`, input error `3,077`/`f7c71258…`), and the tool suite
must remain at 69 tests. This release touches no tool code; any anchor movement is a defect.

## 9. Compatibility

| Dimension | Impact |
|---|---|
| Public Rust API | **None** — no item added, removed, or changed |
| Runtime behavior | None |
| Feature flags | None |
| Dependencies | None |
| MSRV | None (`1.85`) |
| Maturity labels | None from this release. `matten-mlprep` production-ready (unchanged by this release — RFC-080 already promoted it); `matten-data` production-ready (unchanged by this release — RFC-085 already promoted it); `matten-stats` production-ready candidate (unchanged by this release — RFC-084 already promoted it) |
| Version | `0.38.0` → `1.0.0` |
| **Compatibility promise** | **Changes fundamentally.** `0.x` promised nothing; `1.0.0` commits the covered surface under SemVer (§6.1) |

The last row is the entire point of this release and the only thing about it that is irreversible.

## 10. Acceptance Criteria

```text
[ ] the decision not to run cargo public-api (§4.1) is recorded with rationale, not silently
    dropped from the original scope
[ ] the #[doc(hidden)] decision (§4.2, revised: covered) is implemented in compatibility.md,
    listed under what SemVer guarantees, not what it excludes
[ ] the RFC-067 family maturity table is reproduced in full in this RFC (§5)
[ ] compatibility.md states what 1.0 covers and what it excludes
[ ] all 19 pre-1.0 / 0.x sites swept, each of the five phrasing families handled appropriately
[ ] migration.md's compatibility-promise section points at compatibility.md rather than
    stating a second, independent compatibility claim
[ ] all 29 current-family version strings across 14 files retarget to 1.0.0 / 1.0.x
[ ] CHANGELOG names matten-data's production-ready label (RFC-085) and matten-stats's candidate label (RFC-084) explicitly
[ ] cargo metadata shows a single family version 1.0.0 with the lock in sync
[ ] the only .rs change is the install-pin doc comment
[ ] the complete §8 gate set passes and is reported
[ ] local-tool anchors and the 69-test count are unchanged
[ ] ROADMAP header equals the latest history row (guard-enforced)
[ ] no tag, no publish, no API change, no dependency change, no maturity promotion
```

## 11. Non-goals

```text
[ ] tagging 1.0.0
[ ] publishing to crates.io
[ ] any public API addition, removal, or change
[ ] promoting matten-data or matten-mlprep (both already happened, separately, via RFC-085 and RFC-080)
[ ] changing MSRV, edition, features, or dependencies
[ ] sealing, renaming, or removing the #[doc(hidden)] slice plumbing
    (§4.2 decides its compatibility status only; the items themselves are untouched)
[ ] any tools/ change
[ ] reopening streaming, broader stats, broader linalg, or bridge scope
[ ] revisiting RFC-030 lock-step versioning beyond RFC-075 §3.1
```

## 12. Follow-up Work

After this RFC is accepted, implemented, reviewed, and committed, **Unit 3** — release execution — remains
a separate maintainer-authorized step:

```text
1. tag 1.0.0 — bare SemVer, no v prefix
2. publish in dependency order: matten, then the companions (matten-ndarray, matten-mlprep,
   matten-data, and matten-stats — the latter took Exit A via RFC-084 and remains in the
   lock-step family, RFC-081 §3)
3. post-release status alignment commit: RFC-076 status, ROADMAP Status line and
   release-table row, each with its own history row
```

Note for whoever executes Unit 3: unlike every prior release in this project, **step 2 is irreversible**.
crates.io does not permit unpublishing beyond a 72-hour yank window, and a yank does not undo a
compatibility promise already consumed by downstream users. The review of this RFC's implementation is the
last point at which reconsidering is cheap.

---

**Reviewer note:** this RFC was drafted by the same party who has acted as independent auditor on the
RFC-069→075 line. It should receive review from a different reviewer, or an explicitly fresh pass, since
the usual independence between author and auditor does not hold here.
