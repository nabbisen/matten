# RFC-086: `0.40.0` — Feature and Maturity Release

**Status:** `proposed/` by folder (not yet implemented); **reviewed and accepted 2026-07-30** —
implementation authorized under the handoff. Release preparation only; tag and publish remain a
separate authorized step, and tagging is blocked until §3's precondition is resolved. §10's
release-readiness proposal is **not** adopted by this acceptance — it is a separate owner decision
**Target:** `0.40.0`, on the `0.x` line
**Theme:** Ship the accumulated user-facing work of RFC-082 through RFC-085, and close the release-cadence
gap that let it accumulate unnoticed
**Depends on:** RFC-030, RFC-064, RFC-067, RFC-075, RFC-079, RFC-082, RFC-083, RFC-084, RFC-085
**Related:** RFC-076, RFC-078, RFC-080, RFC-081

---

## 1. Summary

Bump the lock-step family `0.39.0` → `0.40.0` and prepare the release of four completed RFCs. Two are
user-facing features; three are maturity promotions that have never been visible to users.

**No tag, no publish.** This RFC prepares a release; executing it is a separate authorized step
(§8), and the owner has one **blocking precondition** to resolve first (§3).

## 2. Motivation — and the process failure that produced it

`0.39.0` shipped 2026-07-28. Since then six commits have touched published crates:

```text
matten-data     CsvBatchReader                                   RFC-082   new public type
matten-stats    covariance_population, skewness, kurtosis        RFC-083   three new public functions
matten-mlprep   -> production-ready                              RFC-080   maturity
matten-stats    -> production-ready candidate                    RFC-084   maturity
matten-data     -> production-ready                              RFC-085   maturity
```

None of it is installable. A user on `0.39.0` cannot call any of the four new functions, and crates.io
still advertises `matten-stats` as `Experimental` — a label three RFCs have since superseded.

**This accumulated because nothing asked.** Each RFC correctly said "no release, no version bump" for
its own slice; no step ever asked whether the *accumulation* warranted one. This project has now failed
release cadence in both directions: `rfcs/README.md` records a post-`0.38.0` assessment finding **eight
consecutive releases (`0.31.0` → `0.38.0`) with zero published-crate change**, and this is the mirror
image — real content, no release. The common cause is that releases have no trigger; they happen when
someone remembers. §10 proposes the fix.

## 3. BLOCKING PRECONDITION — the `0.38.0` and `0.39.0` tags are orphaned

The `filter-branch` pass that stripped `Co-Authored-By` trailers rewrote every commit but **did not
rewrite the tags**. Measured:

```text
0.39.0 -> 557ffcb   ORPHANED   correct target: 041c115  "Prepare 0.39.0 RFC-079 pre-v1 feature release"
0.38.0 -> 6450dbe   ORPHANED   correct target: 16356bd  "Prepare 0.38.0 RFC-073 input-mode JSON release"
97 other tags       ON BRANCH
```

Both orphaned tags are **published releases**, and `origin` carries the same two tags at the same
orphaned commits. The link between each published artifact and its source is severed for exactly the
two versions users are running.

**Tagging `0.40.0` before fixing this makes it worse**, not neutral: it adds a third release to a tag
sequence where the two most recent entries do not resolve to branch history, and it obscures the
breakage behind a working newest tag.

**This is the owner's action, not mine or the implementer's** — high-model §12 and mid-model §13 both
prohibit tagging. Recommended sequence:

```text
1. re-tag 0.38.0 -> 16356bd and 0.39.0 -> 041c115, forcing the update on origin
2. verify: every tag resolves to an ancestor of main (99/99)
3. only then execute this release
```

RFC-086 may be reviewed, accepted, and implemented before that happens — the precondition blocks
**tagging**, not preparation. But it must be resolved before `0.40.0` is tagged.

## 4. Release content

### 4.1 Features (user-visible, new)

```text
matten-data   CsvBatchReader::{open, next_batch}  behind the off-by-default `streaming`
              feature (RFC-082). Row-count-bounded batched CSV reading; no new dependency.
              Two documented divergences from Table::from_csv_path on malformed input
              (RFC-082 §4.3) -- the CHANGELOG must not imply exact parity.

matten-stats  covariance_population (population, ddof = 0), skewness, kurtosis
              (both uncorrected g1/g2; kurtosis is EXCESS/Fisher, so a normal
              distribution scores 0.0, not 3.0) -- RFC-083. Three functions to six.
              MattenStatsError gains no variant. matten-data's MattenDataError gains
              InvalidBatchSize (additive, #[non_exhaustive], non-breaking).
```

### 4.2 Maturity promotions (first public visibility)

```text
matten-mlprep  production-ready candidate -> production-ready   RFC-080
matten-stats   Experimental -> production-ready candidate       RFC-084 (discharges RFC-081 §3 Exit A)
matten-data    production-ready candidate -> production-ready   RFC-085 (closes RFC-059 §6)
```

**RFC-067 requires each label be named explicitly** — no wording may imply a silent promotion. All
three must appear in the CHANGELOG by name, with `matten-stats`'s stated as *candidate*, not
production-ready.

Resulting family: `matten` stable, `matten-ndarray` / `matten-mlprep` / `matten-data` production-ready,
`matten-stats` production-ready candidate.

## 5. Scope

### In scope

```text
version bump 0.39.0 -> 0.40.0 (Cargo.toml + Cargo.lock, all five crates -- lock-step is unconditional)
36 current-family version-string retargets across 16 files (§6)
CHANGELOG.md [0.40.0] entry (§7), including the four-crate preamble fix (§7.2)
ROADMAP.md release-table row, Status, history row, header bump
rfcs/README.md tracking
```

### Out of scope — a diff touching these is a defect

```text
any crates/*/src/*.rs change except crates/matten/src/lib.rs's install-pin doc comment
any public API change, addition, or removal -- the surface ships exactly as RFC-085 left it
any dependency, feature, edition, or MSRV change
any further maturity promotion -- the three in §4.2 already happened; this release
  publishes them, it does not perform them
any pre-1.0 / 0.x wording change -- this is a 0.x release, that wording stays correct
RFC-076, and compatibility.md's v1.0 requirements section
cargo public-api -- optional per RFC-076 §4.1's maintainer decision
tag creation, crates.io publishing, and the §3 tag repair
```

## 6. Version-string retarget — 36 strings across 16 files

Measured at `2f00edf`:

```text
README.md                                     11
docs/src/examples/data.md                      4
crates/matten-ndarray/README.md                3
crates/matten-mlprep/README.md                 3
crates/matten-stats/README.md                  3
crates/matten-data/README.md                   2
Cargo.toml                                     1
crates/matten/README.md                        1
crates/matten/src/lib.rs                       1
docs/src/quick-start.md                        1
docs/src/contributing/architecture.md          1
docs/src/contributing/release-checklist.md     1
docs/src/reference/boundary.md                 1
docs/src/reference/compatibility.md            1
docs/src/reference/dynamic.md                  1
docs/src/reference/public-api-snapshot.md      1
```

**132 further occurrences are historical and must NOT change**: `CHANGELOG.md` (4), `ROADMAP.md` (13),
`rfcs/**` (115). Those record what was true at their release. RFC-079's review caught an over-broad
retarget doing exactly this damage; do not repeat it.

The implementer must re-measure rather than trust these figures — the count moves with every commit.

## 7. CHANGELOG

### 7.1 Required content

```text
Added    — CsvBatchReader (feature-gated, RFC-082); covariance_population, skewness,
           kurtosis (RFC-083). State kurtosis is EXCESS. State the streaming feature
           is off by default.
Changed  — MattenDataError::InvalidBatchSize added (additive, non-breaking).
Maturity — all three promotions of §4.2, each named explicitly (RFC-067), with
           matten-stats stated as production-ready CANDIDATE.
Version  — lock-step family bump, all five crates.
```

Do **not** claim `CsvBatchReader` is equivalent to `Table::from_csv_path`: RFC-082 §4.3 records two
accepted divergences on malformed input.

### 7.2 Fix the preamble — it still says four crates

`CHANGELOG.md`'s header reads *"each entry applies to the whole family — core `matten`,
`matten-ndarray`, `matten-mlprep`, and `matten-data`."* `matten-stats` has been published since
`0.39.0` and is missing. That is a live inaccuracy in a user-facing document; fix it here.

This is an exception to §5's "no CHANGELOG history change" instinct — the preamble is *current*
description, not a historical entry. No released entry is touched.

## 8. Release execution — separate and authorized

```text
1. resolve §3's tag precondition                              OWNER
2. tag 0.40.0 -- bare SemVer, no v prefix                      OWNER
3. publish in dependency order: matten first, then
   matten-ndarray, matten-mlprep, matten-data, matten-stats    OWNER
4. post-release status alignment commit                        normal RFC flow
```

`matten` must be published first (release-checklist §"publish ordering"). Companion dry-runs may fail
before core is visible on crates.io; that is a sequencing caveat, not a dependency-policy failure.

## 9. Acceptance criteria

```text
[ ] version 0.40.0 in Cargo.toml and Cargo.lock, all five crates, verified by cargo metadata
[ ] the 36 live strings retargeted, count RE-MEASURED not assumed
[ ] zero change to CHANGELOG released entries, ROADMAP history rows, or rfcs/**
[ ] CHANGELOG [0.40.0] entry per §7.1, and the §7.2 preamble fix
[ ] all three maturity promotions named explicitly, matten-stats as CANDIDATE (RFC-067)
[ ] the only .rs change is crates/matten/src/lib.rs's install-pin doc comment
[ ] full gate set: fmt, clippy, workspace tests, doctests, MSRV, mdbook, all guards
[ ] no tag, no publish, no API change
```

## 10. Follow-up — close the cadence gap

The accumulation in §2 happened because no step asked. Proposed: fold a **release-readiness check**
into the §6.4 RFC-disposition checkpoint the high-capability model already runs at every RFC close —
an existing hook, so it costs nothing:

```text
1. is there unreleased published-crate change since the last tag?
2. is any of it user-facing?
3. does the accumulation justify a release yet?
```

A "no" to 3 is a fine answer; an *unasked* question is not. This converts "someone remembers" into
"asked every time", which is what both past failures needed.

Recorded here as a proposal; adopting it is the owner's call and would be a small amendment to the
role documents rather than part of this release.

## 11. Non-goals

```text
v1.0 preparation or execution -- RFC-076 stays deferred; v1.0 is not currently wanted
any new feature, API change, or maturity promotion
the §3 tag repair -- owner action, prerequisite to tagging, not part of this preparation
resolving any RFC-082 §5 or RFC-083 §6 deferral
```
