# RFC-079: `0.39.0` Pre-v1 Feature Release

**Status:** Reviewed (GO, no conditions after one correction, applied), committed, and **released
as `0.39.0`** — tagged and published to crates.io. Scope was narrowed by owner decision (§3) to
release RFC-077 only, deferring `matten-stats`'s first publication pending an external read of
RFC-078 §4.1 — but the actual publish action, taken outside this project's assistant session,
included `matten-stats` anyway (no `publish` key ever mechanically enforced the deferral). See the
`0.39.0` post-release alignment handoff for the correction: RFC-079 §3's deferral decision itself
is recorded accurately as of the time it was made; it is the crates.io outcome that diverged from
it, not this document being wrong when written.
**Target:** `0.39.0` on the `0.x` line — released, tagged, and published (all five crates,
`matten-stats` included)
**Theme:** Release RFC-077's seeded split as the family's first consumer-visible release since
`0.31.0`. `matten-stats` (RFC-078) shipped alongside it at `0.39.0`, Experimental maturity, despite
this RFC's own §3 decision to defer it
**Depends on:** RFC-030, RFC-067, RFC-075, RFC-077, RFC-078
**Related:** RFC-024, RFC-040, RFC-058, RFC-074, RFC-076

---

## 1. Summary

Prepare `0.39.0` as a normal `0.x` minor release. Two features are implemented, reviewed, and closed
on the `0.38.x` line, but only one is published this cycle:

```text
RFC-077  matten_mlprep::train_test_split_seeded   (e9b87fd, closed ea8fd23)   -- PUBLISHED in 0.39.0
RFC-078  the matten-stats companion crate         (3ab3864, closed 7fb9c7c)   -- NOT published; see SS3
```

`matten-stats` stays a workspace member and moves to `0.39.0` in lock-step (RFC-030) like every other
crate — that is unconditional and does not depend on publish status. What is deferred is the one
irreversible act: `cargo publish -p matten-stats`, the crate's first-ever appearance on crates.io.

Scope:

```text
bump the lock-step family version 0.38.0 -> 0.39.0 across FIVE crates (all workspace members,
  published or not)
retarget 33 current-family version strings across 15 files, including matten-stats's own README
add a CHANGELOG [0.39.0] entry for RFC-077 only
```

**This RFC does not tag or publish.** Acceptance authorizes preparing the release commit; tagging `0.39.0`
and publishing remain a separate owner-authorized step (§10), and per the ROADMAP `3.22.0` standing rule
the version bump itself requires the owner's explicit confirmation — obtained; see §3.

**This is not the v1.0 release.** RFC-076 remains proposed and deferred. Everything `0.x` — the `pre-1.0`
wording, the "no compatibility promise" posture, `cargo public-api`'s optional status — stays exactly as it
is. This RFC deliberately touches none of it.

## 2. Motivation

Two things make this release worth cutting now rather than accumulating further:

1. **It is the first consumer-visible release since `0.31.0`.** RFC-074's MD-2 finding was that eight
   consecutive releases changed no published crate while all effort went into a local tool. This release
   ends that run with two genuine additions to published crates — and consequently RFC-075 §3.1's
   local-tool-only CHANGELOG justification does **not** apply here, for the first time since the rule was
   written.
2. **Both features are complete and idle.** Each was implemented, independently reviewed (GO, no
   conditions), and closed. Holding them unreleased delivers nothing and lets the gap between repository
   and crates.io widen.

## 3. The one consequential decision: publishing `matten-stats` — RESOLVED: deferred

`crates/matten-stats/Cargo.toml` carries no `publish` key, so it defaults to publishable, and doing so
would be the only irreversible act available in this release. RFC-078 §4.1's decision to diverge from
core's population statistics (sample `ddof = 1` for `covariance`/`correlation`) had been proposed, argued,
and reviewed by the same party throughout every prior round — no external reader had examined the
*judgment* (the mathematics and documentation were separately verified and are correct; the judgment call
itself was not).

**Owner decision:** obtain that external read from someone outside this project's assistant session before
`matten-stats` first publishes, rather than either publishing it now with the risk accepted on the record,
or delaying this entire release. Consequently:

```text
matten-stats is NOT published in 0.39.0
its Cargo.toml/Cargo.lock entry still moves to 0.39.0 in lock-step (RFC-030) — version numbers
  are not a publish record, and nothing is lost by the crate carrying a version it was never
  actually pushed at
RFC-077's train_test_split_seeded ships alone in 0.39.0 -- the family's first consumer-visible
  release since 0.31.0 still happens, on schedule
matten-stats publishes in a LATER release, once the external read lands, whatever that read
  concludes (it may also change the ddof = 1 choice itself before first publication -- Experimental
  on a 0.x line means that is still open)
```

This is not a defect this RFC introduces; it is the owner directly exercising the choice §3 originally
posed, and choosing neither of the two options this RFC anticipated (external read now vs. accept the risk
now) but a third: defer the publish until the read exists, obtained by a human outside this session rather
than by any further self-review inside it.

## 4. Scope

### In scope

```text
version bump 0.38.0 -> 0.39.0 (Cargo.toml + Cargo.lock, all five crates, including
  the unpublished matten-stats -- lock-step versioning is unconditional)
crates/matten/src/lib.rs install-pin doc-comment string
33 current-family version-string retargets across 15 files (§5)
CHANGELOG.md [0.39.0] entry for RFC-077 only (§6)
ROADMAP.md release-table row, Status, and history row
rfcs/README.md tracking
```

### Out of scope — a diff touching these is a defect

```text
any crates/*/src/*.rs change except the lib.rs doc-comment install pin
any public API change, addition, or removal
any dependency, feature, edition, or MSRV change
maturity promotion (matten-stats stays Experimental; matten-mlprep and
  matten-data stay production-ready candidate)
any pre-1.0 wording change — this is a 0.x release, that wording stays correct
compatibility.md's v1.0 requirements section — untouched
cargo public-api — remains optional per RFC-076 §4.1's maintainer decision
tag creation or crates.io publishing (including matten-stats's first publish -- SS3)
docs/src/contributing/release-checklist.md's fifth-crate/publish-order teaching --
  deferred to whichever release actually first publishes matten-stats (SS7)
RFC-076 updates (noted in §9, not performed here)
```

## 5. Version-string retarget — 33 strings across 15 files

Measured at the accepted base:

```text
README.md                                   11
crates/matten-stats/README.md                3
crates/matten-mlprep/README.md               3
crates/matten-ndarray/README.md              3
docs/src/examples/data.md                    3
crates/matten-data/README.md                 1
crates/matten/README.md                      1
crates/matten/src/lib.rs                     1
docs/src/contributing/architecture.md        1
docs/src/contributing/release-checklist.md   1
docs/src/quick-start.md                      1
docs/src/reference/boundary.md               1
docs/src/reference/compatibility.md          1
docs/src/reference/dynamic.md                1
docs/src/reference/public-api-snapshot.md    1
```

`0.38.0` → `0.39.0` and `0.38.x family` → `0.39.x family`. `crates/matten-stats/README.md` is new to this
list — it did not exist at the `0.38.0` release.

Three files legitimately retain `0.38` references after a correct retarget and are exempt:

```text
docs/src/reference/compatibility.md   per-family history section retains older versions
docs/design/v1-readiness-audit.md     dated audit report; "eight releases (0.31.0 -> 0.38.0)" is
                                       the measurement that motivated this pre-v1 feature line —
                                       rewriting it would corrupt that finding
scripts/check-release-docs.sh         a code comment records the 0.38.0 incident that motivated
                                       the ROADMAP header/history-row parity guard
```

**Unlike RFC-076, no `pre-1.0` sweep is involved.** That wording remains accurate on a `0.x` release and
must not be touched.

## 6. CHANGELOG

### 6.1 Structure

A `## [0.39.0]` entry recording one additive feature, no breaking change, and no local-tool-only
justification (§2 — this release changes a published crate). `matten-stats` is **not** mentioned —
CHANGELOG documents what ships to crates.io consumers, and it does not ship this release.

```text
### Added
- matten-mlprep: train_test_split_seeded(x, train_ratio, seed) — reproducible
  shuffled split using a dependency-free SplitMix64 PRNG (RFC-024 §6). The
  existing ordered train_test_split is unchanged.

### Version
- Release bump 0.38.0 -> 0.39.0.
```

### 6.2 Required statements

```text
name train_test_split_seeded and its one-line behavior
no "Changed" section claiming the family grows to five crates — it does not,
  from a crates.io consumer's perspective, until matten-stats actually publishes
```

**Do not mention `matten-stats` in this CHANGELOG entry at all** — not as Added, not as deferred,
not as a forward-looking note. A `[0.39.0]` entry describes `0.39.0`'s shipped contents; a crate that
does not ship has no place in it, and adding one invites exactly the "shipped by implication" outcome
§3 was written to prevent. Its eventual first-publish CHANGELOG entry belongs to whatever release
actually does that, once the external read lands.

## 7. Release-checklist — no change this release

`docs/src/contributing/release-checklist.md`'s `matten-stats`/fifth-crate/publish-ordering teaching is
**deferred entirely** to whichever release actually first publishes `matten-stats` (§3). Updating it now
would describe a publish step this release does not take, which is worse than leaving the gap: a stale
"five crates" instruction that fires before the crate is ever actually published. `cargo package
--workspace` already exercises all five crates today regardless of this document; packaging is not
publishing.

## 8. Compatibility

| Dimension | Impact |
|---|---|
| Public Rust API | **Additive** — one new function published (`train_test_split_seeded`). `matten-stats` exists in the workspace at `0.39.0` but ships no public API this release, since it is not published |
| Runtime behavior | None for existing APIs |
| Feature flags | None |
| Dependencies | None new in any existing crate; `matten-stats` depends only on `matten` |
| MSRV | None (`1.85`) |
| Maturity labels | No change; `matten-stats` remains Experimental and unpublished |
| Version | `0.38.0` → `0.39.0`, for all five workspace crates including the unpublished `matten-stats` |
| Compatibility promise | **Unchanged** — still `0.x`, still no SemVer guarantee |

## 9. Consequence for RFC-076 (flagged, not resolved)

RFC-076's v1.0 release-prep specification assumes a **four**-crate family and is already factually stale
today, regardless of when `matten-stats` actually publishes — the crate exists in the workspace now. It
needs, before it is ever executed:

```text
cargo package --workspace covering five crates
a fifth row in the RFC-067 family maturity table
Cargo.lock bump scope of five package entries
publish order including matten-stats, whenever it does first publish
```

There is also a question RFC-067 never answered and RFC-076 must not assume: **may a v1.0 family include a
crate at `Experimental`?** RFC-067 resolved the *candidate*-label question (MD-1); `Experimental` is a
different rung. This RFC does not resolve it — it records that RFC-076 cannot be executed until someone
does.

## 10. Acceptance criteria

```text
[ ] Cargo.toml + Cargo.lock at 0.39.0 for all five crates (including matten-stats); cargo metadata confirms
[ ] the only .rs change is the lib.rs install-pin doc comment
[ ] all 33 version strings retargeted, including crates/matten-stats/README.md; git grep for
    0.38 returns only the named historical exemptions (SS5)
[ ] no pre-1.0 wording touched (this is a 0.x release)
[ ] CHANGELOG's [0.39.0] entry names only train_test_split_seeded; no matten-stats mention at all
[ ] docs/src/contributing/release-checklist.md is untouched by this RFC (SS7)
[ ] cargo package --workspace packages five crates (packaging, not publishing)
[ ] all six guard scripts, full feature matrix, MSRV, and report-tool anchors pass
[ ] no tag, no publish (matten-stats included), no API/dependency/MSRV/maturity change
```

## 11. Non-goals

```text
[ ] tagging or publishing anything, matten-stats specifically included
[ ] any v1.0 activity — RFC-076 stays proposed and deferred
[ ] promoting any crate's maturity label
[ ] changing the ddof decision (that is RFC-078's; still open until the external read lands, SS3)
[ ] resolving RFC-067's Experimental-in-a-1.0-family question (§9)
[ ] touching pre-1.0 wording or compatibility.md's v1.0 section
[ ] teaching the release checklist about matten-stats (deferred to its first-publish release, SS7)
```

## 12. Follow-up

After this RFC is accepted, implemented, reviewed, and committed, release execution remains a separate
owner-authorized step:

```text
1. tag 0.39.0 — bare SemVer, no v prefix
2. publish matten first, then matten-ndarray, matten-mlprep, matten-data
   (matten-stats is explicitly EXCLUDED from this publish list -- SS3)
3. post-release status alignment commit with its own ROADMAP history row
```

**`matten-stats` publishes in a later release**, once the owner has obtained an external read of RFC-078
§4.1 outside this project's assistant session. Nothing about *this* release is irreversible in that sense —
every crate it does publish is a routine additive `0.x` change to an already-published crate.

## 13. Post-release correction — `matten-stats` published anyway

`0.39.0` was tagged and published outside this project's assistant session. Verified directly against
crates.io (not `cargo publish --dry-run`, which is the wrong instrument for checking state that was
never withheld by tooling): `matten-stats` shows exactly one version, `0.39.0`, alongside `matten` and
the other three crates. `crates/matten-stats/Cargo.toml` never carried a `publish = false` key — §3
named that as the mechanism-free state that made the deferral a discipline decision, not an
enforced one, and the actual publish action did not honor it.

**§3's decision itself is unchanged and was correctly recorded at the time it was made.** What changed
is the crates.io outcome, not this RFC's account of the decision. The consequence: the crate name
`matten-stats` is now permanently claimed, the `ddof = 1` policy has shipped without the external read
§3 called for, and that read now informs whether a *future* change to `covariance`/`correlation` is
warranted — it is no longer a gate on first publication, because first publication already happened.
See the `0.39.0` post-release alignment handoff and ROADMAP's corresponding new history row (added
after `3.25.0`, which is left untouched as an accurate record of the decision as it stood).
