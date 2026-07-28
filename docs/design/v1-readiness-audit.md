# v1.0 Readiness Audit

**Project:** `matten`
**Related RFC:** RFC-066 (original audit, v0.31.0); re-audited under RFC-074
**Document kind:** Readiness audit report
**Status:** Re-audited (0.38.0); conditionally ready on technical grounds, blocked on an
explicit maintainer decision (MD-2). Post-audit: NF-1 and NF-2 closed (H0, see
[Post-Audit Update](#post-audit-update-nf-1-nf-2-closed)).
**Scope:** Audit report only; no v1.0 release authorization

---

## Summary

This is a re-audit of the original RFC-066 v1.0 readiness audit, conducted under
RFC-074 after eight releases (`0.31.0` -> `0.38.0`) with no published-crate
change. It supersedes the RFC-066 findings below while preserving them for
traceability, per RFC-074 goal #1.

**Every RFC-066 finding remains resolved or unchanged, with no regression:**

```text
BF-1 (public API snapshot dynamic-serde inconsistency): remains remediated.
MD-1 (lock-step v1.0 with candidate companions): remains resolved by RFC-067.
NF-1 (matten-data and matten-ndarray lack an explicit Public API README block): closed post-audit (H0).
NF-2 (cargo public-api not wired as a gate): closed post-audit (H0) as a manual release-checklist step.
```

**On narrow technical grounds, the public surface is more provably stable now
than at the original audit**, not less: `git diff 0.31.0..HEAD -- crates/*/src/`
touches exactly one file (`crates/matten/src/lib.rs`), and every changed line
is a version string inside a `//!` doc comment. Zero functional change reached
any published crate across eight consecutive releases. The public API snapshot,
error model, and boundary docs are internally consistent and match source.

**But this audit finds a new, unresolved maintainer-decision finding (MD-2)
that the original audit did not have to consider**, because it is a direct
consequence of the eight-release gap this re-audit was triggered by:

```text
MD-2: The published family has had zero functional change across eight
      releases while all engineering effort went into a private,
      unpublished local tool (tools/matten-report). RFC-071's own accepted
      rationale for releasing private-tool milestones as lock-step
      checkpoints included an explicit reconsideration trigger — "if the
      publish churn outweighs the value of a public project checkpoint" —
      that has not fired despite eight consecutive occasions to fire it.
      No v1.0 release RFC has been drafted in the ~7 releases since RFC-066
      opened that possibility. This audit cannot decide, on the maintainer's
      behalf, whether the project's real direction is "heading toward 1.0"
      or "0.x indefinitely with occasional local-tool-only checkpoints" —
      but it can no longer treat that question as implicitly answered by
      inaction, because inaction has now produced eight releases of
      evidence pointing toward the latter.
```

**Verdict: conditionally ready.** The technical/API axis clears every gate
RFC-066 set. The audit does not recommend starting v1.0 release preparation,
because doing so without first resolving MD-2 would silently choose "heading
toward 1.0" as the answer by default, the same way lock-step versioning's
reconsideration trigger silently went unfired eight times. See
[Recommendation](#recommendation).

This report does not authorize a v1.0 release.

## Decision Boundary

This report is evidence for a later maintainer decision. It does not authorize:

```text
v1.0 release
version bump
tag
publish
API change
dependency change
companion promotion
new public tooling crate
RFC-030 lock-step versioning policy change
```

If a later decision proposes any of those actions, it needs a separate release
RFC, release-policy decision, or accepted implementation slice.

## RFC-066 Findings Re-Verified, Item By Item

| RFC-066 finding | Original disposition | Re-audit status |
|---|---|---|
| BF-1 — public API snapshot self-contradicted on dynamic serde behavior | Remediated | **Unchanged, remains remediated.** `public-api-snapshot.md`'s dynamic-behaviour table still says `Serialize` returns a serde error; `crates/matten/src/ser.rs` behavior is unchanged (zero source churn since `0.31.0`). |
| MD-1 — can a lock-step v1.0 family include candidate-labeled companions? | Resolved by RFC-067 | **Unchanged, remains resolved.** RFC-067's policy (candidate label is not an automatic v1.0 blocker, but a future v1.0 RFC must include the RFC-067 family maturity table) has not been revisited or contradicted. |
| NF-1 — `matten-data` README lacks an explicit Public API block | Non-blocking | As found at audit time: open. `crates/matten-data/README.md` sections were: Overview, Not a dataframe library, Relationship to core `dynamic`, Status and scope, Dependency style, Compatibility — no `## Public API` heading. `matten-mlprep/README.md` had one (`## Public API`, line 71); `matten-data` and `matten-ndarray` did not. **Closed post-audit; see [Post-Audit Update](#post-audit-update-nf-1-nf-2-closed).** |
| NF-2 — `cargo public-api` not wired as a gate | Non-blocking | As found at audit time: open. No reference to `cargo public-api` or `public-api` tooling existed in `scripts/`, `.github/workflows/`, or the release checklist beyond the existing manual `grep -n "^pub use"` spot-check. **Closed post-audit; see [Post-Audit Update](#post-audit-update-nf-1-nf-2-closed).** |

No RFC-066 finding regressed. Two non-blocking findings (NF-1, NF-2) simply
never got picked up, which is consistent with this audit's broader finding
that engineering effort went entirely into `tools/matten-report` for eight
releases.

## Public API Review

Reviewed inputs:

```text
docs/src/reference/public-api-snapshot.md
docs/src/reference/compatibility.md
crates/matten/src/lib.rs
crates/matten-ndarray/src/lib.rs
crates/matten-mlprep/src/lib.rs
crates/matten-data/src/lib.rs
crate READMEs
```

Independently re-verified for this re-audit:

```text
git diff --stat 0.31.0..HEAD -- crates/
  -> crates/matten-data/README.md    | 2 +-
     crates/matten-mlprep/README.md  | 6 +++---
     crates/matten-ndarray/README.md | 6 +++---
     crates/matten/README.md         | 2 +-
     crates/matten/src/lib.rs        | 2 +-
     5 files changed, 9 insertions(+), 9 deletions(-)

git diff 0.31.0..HEAD -- crates/matten/src/lib.rs
  -> one hunk, both changed lines are inside a ```toml //! doc-comment
     snippet showing the install-pin version string (0.31.0 -> 0.38.0)
```

The core crate root still matches the documented root surface exactly
(confirmed both by direct `grep -n "^pub use"` and by
`scripts/check-release-docs.sh`'s automated root-export allowlist check,
currently green):

```text
Tensor
MattenError
DataFormat
MattenLimits
SliceBuilder
Element                 # feature = "dynamic"
NumericPolicy           # feature = "dynamic"
```

Hidden compiler-visibility plumbing is unchanged:

```text
IntoSliceRange          # #[doc(hidden)]
SliceConvert            # #[doc(hidden)]
SliceSpecRepr           # #[doc(hidden)]
```

Companion surfaces (`matten-ndarray`, `matten-mlprep`, `matten-data`) are
unchanged from the original audit's inventory — re-confirmed by the same
zero-source-churn evidence above.

`docs/src/reference/public-api-snapshot.md` states "the public API did not
change for the 0.38.0 local-tool JSON release," which is accurate and is one
instance of a claim that has now been accurate eight releases running.

**Conclusion: no regression from the original audit; the API surface is more
strongly evidenced as stable than it was at `0.31.0`, precisely because it has
not moved.**

## Panic/Result Boundary Review

Reviewed inputs unchanged from the original audit
(`compatibility.md`, `error-model.md`, `boundary.md`, `public-api-snapshot.md`,
`dynamic.md`). Source review confirms zero churn in `crates/matten/src/`
outside the single doc-comment line already noted, so the two-zone
panic/Result policy, `MattenError` variant set, and dynamic-tensor guard
behavior are unchanged and remain stable enough for v1.0 discussion.

## Serde/Format Review

Reviewed inputs unchanged from the original audit. The canonical JSON object
form (`{"shape":[...],"data":[...]}`), the nested-array convenience forms, and
CSV's framing as ingestion (not canonical serialization) are unchanged.
`crates/matten/src/ser.rs` has zero churn since `0.31.0`. Stable.

## Deferred Mathematics And Streaming Scope (RFC-074 §5.2)

This section did not exist in the original RFC-066 audit; RFC-074 added it
because eleven backlog items sit behind the v1.0 question and each needs an
explicit blocker/non-blocker classification rather than an implicit one.

Reviewed inputs:

```text
rfcs/done/040-small-statistics-boundary-core-vs-companion.md
rfcs/done/041-linear-algebra-boundary-core-lite-vs-external-crates.md
rfcs/done/026-large-csv-and-streaming-data-policy.md
rfcs/done/037-deferred-streaming-and-large-csv-policy.md
```

**Broader statistics** (covariance, correlation, quantile, percentile,
histogram, z-score): RFC-040 §8's own wording is a deferral with a directed
home, not a permanent rejection — "Do not put histogram or quantile in core
**initially**," with `matten-stats` accepting them "**if accepted**," and
RFC-040's status line reads "quantile/histogram/cov/corr/z-score
**deferred**." Core placement is settled *against* for now; the companion
question remains explicitly open, unlike RFC-041 below. Either way, this sits
outside the current documented contract and adding it later — in core or in
`matten-stats` — is additive, not breaking. **Not a v1 blocker.**

**Broader linear algebra** (inverse, determinant, decomposition, BLAS/sparse):
RFC-041 §5 explicitly rejects these from core with a permanent rationale
("too much numerical policy... would change project identity"), directing
users to `nalgebra`/`ndarray-linalg` instead. **Settled, not a v1 blocker.**

**Streaming / large CSV**: RFC-037 defers this with explicit, unmet reopening
criteria (batch model, schema-drift policy, malformed-row policy, memory
budget, sync-vs-async, crate placement — none answered). Unlike linalg, this
is a genuine "not yet designed" gap rather than a rejected scope, and it is
even less settled than the deferred-but-directed stats question above (no
companion crate name or acceptance bar exists yet for streaming). But
`matten-data`'s current documented scope (in-memory rectangular CSV,
explicitly "not a dataframe engine") does not promise streaming, so v1.0 does
not need to resolve it first — reopening streaming later is additive, not a
breaking change to the current contract. **Not a v1 blocker**, provided the
current scope statement stays in the docs unchanged at v1.0.

**Conclusion: none of the three deferred-mathematics/streaming themes block
v1.0.** Linalg is settled-rejected; stats is settled-against-core-for-now but
deferred-not-rejected toward a companion; streaming is genuinely open but
additive. All three sit outside the current documented contract, and none
requires resolution before v1.0. This narrows the v1.0 question to the
API/boundary/versioning axes actually covered elsewhere in this report.

## Companion Maturity Review (RFC-074 §5.3)

Reviewed inputs:

```text
rfcs/done/030-workspace-versioning-model-lockstep.md
rfcs/done/057-promote-matten-ndarray-production-ready.md
rfcs/done/058-promote-matten-mlprep-production-ready-candidate.md
rfcs/done/059-promote-matten-data-production-ready-candidate.md
rfcs/done/067-v1-family-maturity-policy.md
README.md
crates/*/README.md
```

Current maturity ladder, unchanged since the original audit:

```text
matten          stable (v0.x)
matten-ndarray production-ready
matten-mlprep  production-ready candidate
matten-data    production-ready candidate
```

**No new evidence justifies promoting `matten-mlprep` or `matten-data` to
production-ready.** RFC-058 §5.1 explicitly recorded full-production-ready
exit criteria as "recorded, not required here" and deferred the decision to "a
*separate future review*" (RFC-058, RFC-059 §"Architect ruling: full
production-ready is deferred"). Since zero source lines changed in either
crate since `0.31.0`, the specific caveats those RFCs cited at promotion time
— `matten-mlprep`'s ordered-only `train_test_split` (no shuffle/seed) and
`matten-data`'s CSV-only, non-dataframe scope — are unchanged and still apply.
Promotion requires new work (e.g., a seeded-split option) or a fresh
maintainer review accepting the current limitation as permanently acceptable,
neither of which has happened.

**Recommendation: hold both companions at `production-ready candidate`.**
RFC-067's resolution of MD-1 already covers this case — a v1.0 family may
include them at their current labels if a future v1.0 release RFC states so
explicitly with the RFC-067 family maturity table. This audit does not
recommend spending the promotion review as a v1.0 prerequisite; it recommends
carrying the candidate label into any future v1.0 family exactly as RFC-067
already anticipated.

## Lock-Step Versioning Assessment (RFC-074 §5.4 — first-class, not a footnote)

Reviewed inputs:

```text
rfcs/done/030-workspace-versioning-model-lockstep.md
rfcs/done/071-private-fixed-demo-json-report-artifacts.md
CHANGELOG.md (0.31.0 through 0.38.0 entries)
```

RFC-030's rationale for lock-step versioning is that the crates "only ship
together" and users benefit from "the simplest possible compatibility
contract." That rationale is unaffected by this finding — the crates still
ship together, and a matched version number is still the simplest contract
*when a release changes something*.

The problem is narrower and specific: **RFC-071 §6 itself named the failure
mode and the exact condition under which the project should stop, and that
condition has now been met without triggering the reconsideration:**

> "reconsider this release model before future private-tool-only milestones
> if the publish churn outweighs the value of a public project checkpoint"
> — RFC-071 §6, accepted as the `0.37.0` release's business decision.

Measured evidence that the condition has been met:

```text
Releases 0.32.0 through 0.38.0 (seven releases after RFC-071's own release,
counting 0.37.0 itself as the eighth data point in the 0.31.0-0.38.0 span):
  every one shipped a "no public API / dependency / runtime / MSRV /
  maturity change" CHANGELOG disclaimer (31 occurrences of "No public API"
  across CHANGELOG.md);
  the entire published-crate diff across all eight is nine doc-comment
  version-string insertions and nine deletions.
```

**This is a maintainer-decision finding, not a source defect** — identical in
character to MD-1. It is recorded as **MD-2**:

```text
MD-2: Should RFC-030 lock-step versioning continue to release
      publish-a-family-checkpoint-regardless-of-content for private-tool-only
      milestones, or should the RFC-071 reconsideration trigger now fire —
      e.g. by requiring an explicit per-release justification, tracking
      local-tool milestones on a separate unpublished coordinate, or some
      other policy the maintainer prefers?
```

This audit does not resolve MD-2. Resolving it requires a maintainer decision
and, if the answer changes policy, a separate RFC amending or superseding
RFC-030/RFC-071 — the same pattern RFC-067 used to resolve MD-1 without this
audit deciding company policy unilaterally.

**MD-2 is independent of the v1.0 question and does not block it**: a v1.0
release RFC could proceed under the current lock-step policy unchanged. But it
does bear on the "is a 1.0 commitment appropriate" question below, because a
versioning scheme that has been signaling false progress for eight releases
should not gain a *bigger* promise (1.0's compatibility commitment) layered on
top of it without the maintainer having consciously decided to keep it as-is.

## The 1.0 Commitment Itself (RFC-074 §5.5)

Reviewed inputs:

```text
rfcs/done/066-v1-readiness-audit-and-release-decision-gate.md
rfcs/done/067-v1-family-maturity-policy.md
docs/src/reference/compatibility.md (v1.0 requirements section)
```

`compatibility.md`'s v1.0 requirements section lists: public API review
complete, `cargo public-api` snapshot approved, panic/Result split finalized,
serde canonical format declared stable, limitations/non-goals documented, and
the RFC-067 family maturity table included if any crate remains candidate-
labeled.

Against that list:

```text
public API review complete             -> substantively yes (re-verified above)
cargo public-api snapshot approved      -> tooling now documented as a manual
                                            release-checklist step (NF-2
                                            closed post-audit), but no
                                            snapshot has actually been taken
                                            or approved yet -> still NOT DONE
                                            as a v1.0 gate item
panic/Result split finalized            -> yes, unchanged and stable
serde canonical format declared stable  -> the JSON object form is de facto
                                            stable (zero churn) but has never
                                            been explicitly "declared" stable
                                            as its own decision
limitations/non-goals documented        -> yes (RFC-040/041/037 boundaries)
RFC-067 family maturity table           -> not yet drafted (it lives in the
                                            not-yet-written v1.0 release RFC)
```

Three of six gates are met outright; the other three (`cargo public-api`,
explicit serde-stability declaration, the RFC-067 table) are not defects — they
are work items that were always deferred to "whenever a v1.0 release RFC gets
drafted." No one has drafted one in the ~7 releases since RFC-066 opened that
door.

**Recommendation: do not treat "not ready" as a permanent default answer, and
do not silently default into "must be heading toward 1.0" either.** The
honest reading of eight releases of zero published-crate change is that the
project's *de facto* behavior has been "0.x, with occasional lock-step
checkpoints for local-tool work" — not "en route to 1.0." That may be exactly
the right choice; family-car positioning (README.md, `philosophy.md`) never
promised a 1.0 timeline. But it should be a **conscious** choice, matching
this audit's headline finding, not an emergent one.

This audit recommends the maintainer pick one of two paths explicitly (see
[Recommendation](#recommendation)) rather than leaving the question open for
a ninth consecutive release.

## Release-Gate Review

Reviewed inputs unchanged from the original audit. Re-assessed against the
same v1.0.0 gate:

| Gate | Assessment |
|---|---|
| stable core public API | **Stronger than at the original audit.** Zero churn across eight releases is direct evidence, not just documentation review. `cargo public-api` tooling is now documented (NF-2 closed); actually running and approving a snapshot is still the one missing formal step. |
| clear dynamic on-ramp story | Unchanged; still yes. |
| strong, scoped examples | Unchanged; still yes. |
| reliable diagnostics | Unchanged; still yes. |
| documented companion-crate boundary | Unchanged; still yes, and RFC-030/RFC-067 now additionally cover the lock-step/maturity-table question explicitly. |
| clean feature matrix | Unchanged; the release-checklist commands remain the required gate, not converted into an additional audit-owned check. |

## Blocking Findings

None. No unresolved blocking source/doc mismatch exists, matching the
original audit's conclusion after BF-1 remediation, re-confirmed with zero
source churn since.

## Maintainer-Decision Findings

### Resolved MD-1: Lock-step v1.0 with candidate-labeled companions

Unchanged from the original audit. Resolved by RFC-067: candidate-labeled
companions are not automatic v1.0 blockers, but a future v1.0 release RFC must
include the RFC-067 family maturity table and decide inclusion explicitly.

### New MD-2: Lock-step versioning's unfired reconsideration trigger

See [Lock-Step Versioning Assessment](#lock-step-versioning-assessment-rfc-074-54--first-class-not-a-footnote)
above. Not resolved by this audit; requires an explicit maintainer decision
and, if policy changes, a separate RFC.

## Non-Blocking Findings

### NF-1: `matten-data` snapshot-equivalent docs are less explicit (CLOSED post-audit)

As found at audit time (unchanged from the original audit):
`crates/matten-data/README.md` lacked an explicit `## Public API` block
comparable to `matten-mlprep`'s. The re-audit corrected the scope of this
finding: `crates/matten-ndarray/README.md` also lacked one — the asymmetry
was one-of-four (only `matten-mlprep` had the block), not the
`matten-data`-only gap the original audit's prose suggested, even though its
own finding table already listed both crates correctly.

**Closed.** See [Post-Audit Update](#post-audit-update-nf-1-nf-2-closed).

### NF-2: `cargo public-api` remains a future snapshot step (CLOSED post-audit)

As found at audit time (unchanged from the original audit): not wired as a
project dependency or gate.

**Closed.** See [Post-Audit Update](#post-audit-update-nf-1-nf-2-closed).

## Verification Record

Observed during this re-audit:

```text
cargo fmt --all --check                                      passed
bash scripts/check-core-dependency-boundary.sh                passed
bash scripts/check-published-dependency-isolation.sh          passed
bash scripts/check-matten-data-scope.sh                        passed
bash scripts/check-benchmark-dependency-sync.sh                passed
bash scripts/check-streaming-scope.sh                          passed
bash scripts/check-release-docs.sh                             passed
git diff --check                                               passed
git diff --stat 0.31.0..HEAD -- crates/                        independently reproduced
git diff --name-only 0.31.0..HEAD -- 'crates/*/src/*'          independently reproduced
git diff 0.31.0..HEAD -- crates/matten/src/lib.rs               independently reproduced
grep -c "No public API" CHANGELOG.md                            independently reproduced (31)
grep -n "^pub use" crates/matten/src/lib.rs                     independently reproduced
grep -rn "production-ready" README.md crates/*/README.md        independently reproduced
```

Not run in this re-audit, matching the original audit's scope boundary:

```text
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings
cargo test --all-targets
cargo test --doc --all-features
full feature-matrix test set from docs/src/contributing/release-checklist.md
MSRV build/test commands
```

Reason: this slice updates the audit report and design-doc index entry only.
It changes no Rust source, public API, dependencies, feature flags,
manifests, generated artifacts, or release versions. The omitted gates remain
required before any actual v1.0 release-prep decision.

## Recommendation

Current recommendation:

```text
Not authorized for v1.0 release preparation.
Technical readiness (public API, error model, boundary, deferred-scope
  clarity) clears every gate this audit and RFC-066 set.
Process readiness is blocked on an explicit maintainer decision (MD-2) that
  this audit cannot make: is the project heading toward 1.0, or is a
  documented "0.x indefinitely" stance the more honest position given eight
  releases of evidence pointing that way?
```

Two explicit next paths, ranked by how directly they resolve MD-2 rather than
leaving it open for a ninth release:

```text
Path A — adopt "0.x indefinitely" explicitly.
  Update docs/src/reference/compatibility.md's v1.0 section to state this
  is a deliberate position, not an oversight. Close RFC-066 and RFC-074 as
  resolved-without-1.0. Revisit RFC-030/RFC-071's lock-step release-churn
  question (MD-2) on its own merits, independent of any 1.0 timeline.
  Lowest effort; matches eight releases of observed behavior; requires the
  maintainer to actually choose this rather than let it remain implicit.

Path B — pursue v1.0 deliberately.
  NF-1 and NF-2 are now closed post-audit (see Post-Audit Update below), so
  a v1.0 release RFC's remaining prerequisites are: declare the JSON
  canonical serde format explicitly stable, include the RFC-067 family
  maturity table deciding matten-mlprep/matten-data's candidate-label
  inclusion, actually run and approve a cargo public-api snapshot, and
  resolve MD-2 (a versioning-policy statement, even if the answer is "keep
  RFC-030 unchanged, decided consciously").
```

This audit does not choose between Path A and Path B; that choice is MD-2's
resolution and belongs to the maintainer. Whichever is chosen, this report's
prerequisite list (serde-stability declaration, RFC-067 table, an approved
cargo public-api snapshot, MD-2 resolution) is what a Path B v1.0 release RFC
would need to close before release preparation could begin.

## Post-Audit Update: NF-1, NF-2 closed

After this re-audit and its review (`matten-rfc074-v1-readiness-reaudit-review-v0.1.md`,
GO), the two non-blocking findings were closed as the review's H0 common
first step (applicable to either path):

```text
NF-1: added a `## Public API` block to crates/matten-data/README.md and
      crates/matten-ndarray/README.md, mirroring crates/matten-mlprep/README.md's.
NF-2: added `cargo public-api` as a documented manual pre-release step in
      docs/src/contributing/release-checklist.md, reconciled against
      docs/src/reference/public-api-snapshot.md and each crate's README
      Public API block. Not wired into CI; that remains a separate,
      explicit future decision (toolchain pinning, nightly requirements).
```

This closes the tooling/documentation gap only. It does not mean a
`cargo public-api` snapshot has actually been taken and approved yet — that
remains a real Path B prerequisite, to be run before an actual v1.0
release-prep RFC, not before this audit-adjacent cleanup.

No v1.0 release is authorized by this report.
