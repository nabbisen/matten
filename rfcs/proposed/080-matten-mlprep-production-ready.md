# RFC-080: Promote `matten-mlprep` to Production-Ready

**Status:** Proposed; maturity-label decision only, no code or release change
**Target:** Post-`0.39.0` maturity decision on the `0.x` line; release family undecided
**Theme:** Take the promotion review RFC-058 deferred, now that RFC-058 §5.1's Option B is satisfied
**Depends on:** RFC-024, RFC-057, RFC-058, RFC-067, RFC-077
**Related:** RFC-022, RFC-030, RFC-059, RFC-074, RFC-076

---

## 1. Summary

Promote `matten-mlprep` from **production-ready candidate** to **production-ready**.

This is the "separate future review" RFC-058 explicitly deferred in 2026-06-27. It changes a maturity label
and the documentation that carries it. **No code, no API, no version, no release.**

The reason it is takeable now is precise rather than general: RFC-058 §5.1 recorded three exit criteria for
this exact promotion, and **RFC-077 satisfied Option B**.

## 2. Why now — RFC-058 wrote the exit criteria, and one is met

RFC-058's architect ruling deferred full production-ready with a single stated cause:

> **Architect ruling (2026-06-27): production-ready candidate. Full production-ready is deferred** — the
> ordered-only split is a real functional caveat, acceptable for candidate … but not yet for a broad
> production-ready recommendation.

§5.1 then recorded three ways out:

```text
Option A  keep ordered-only and justify it as intentional scope
Option B  add shuffled / seeded split first, via a separate feature RFC
          "(it raises API and RNG/dependency-policy questions)"
Option C  stay candidate until the split story is settled  [the path then taken]
```

**RFC-077 is Option B, executed to its description.** It was a separate feature RFC; it raised exactly the
API and RNG/dependency-policy questions §5.1 anticipated; and it resolved them (RFC-024 §6's pre-specified
signature, a dependency-free SplitMix64, a reproducibility contract locked by a mutation-proven test). It
shipped in `0.39.0`.

So the deferral's stated cause is gone. This RFC asks whether anything *else* stands in the way.

## 3. Audit against the production-ready bar

RFC-057 §3 defines the bar used for `matten-ndarray`'s promotion. Applying the same one:

| Signal | Evidence | Verdict |
|---|---|---|
| Mature docs | Crate README with a `## Public API` block (added closing RFC-066 NF-1), five examples, per-function doc comments with `# Errors` sections and doctests | **Met** |
| Stable API | Zero functional churn `0.31.0` → `0.38.0` (RFC-074's measurement). The only change since is RFC-077's *additive* `train_test_split_seeded` | **Met** |
| Compatibility and MSRV policy | Documented in the crate README's SemVer section; MSRV `1.85` shared family-wide | **Met** |
| Clear release notes | Present for every release touching the crate, including `[0.39.0]` | **Met** |
| No hidden dependency surprises | Exactly one dependency: `matten`. No third-party crates. RFC-077 deliberately hand-rolled SplitMix64 rather than pull `rand` | **Met** |

And the candidate-level signals it must continue to hold:

| Signal | Evidence |
|---|---|
| Strong tests | 27+ test fns in `tests/preprocessing.rs`, including RFC-077's eight seeded-split tests and a mutation-proven locked-permutation contract test |
| Examples in CI | Five `mlprep_*` examples, run by the release checklist |
| Clear error types | `MattenMlprepError` with six documented variants, `Display` + `std::error::Error` |
| Documented compatibility policy | Crate README SemVer section |
| No known P0/P1 issues | None recorded |

## 4. The one substantive question: is the split story actually settled?

The promotion turns on this, so it should be argued rather than asserted.

**What RFC-077 delivered:** `train_test_split` (ordered, deterministic) and `train_test_split_seeded`
(shuffled, seeded, reproducible). Both documented, both tested, the reproducibility of the second locked by
a test that fails if the PRNG constants or shuffle direction change.

**What a user can now do:** ordered splits for time-series or pre-shuffled data; reproducible random splits
for everything else. That covers the ordinary cases the crate's scope claims to serve.

**What is still absent:** stratified splits, grouped splits, and time-series-aware splits. These are
genuinely not provided — but they are **scope exclusions, not gaps**. RFC-024 defines the crate as
*transparent, deterministic preprocessing helpers*, explicitly not ML-workflow management, and RFC-077 §9
listed stratified/grouped/time-series as non-goals. A production-ready label means "recommend as a normal
dependency **for its documented scope**" (RFC-058's own phrasing), not "covers every splitting strategy."

**Assessment: settled.** The caveat RFC-058 named was the *absence of any shuffled option*, and that is
closed. Remaining absences are declared scope boundaries, which is exactly the condition RFC-067 §"candidate
reason" distinguishes from hidden churn.

## 5. Ladder discipline

RFC-058 §2 argued that `matten-ndarray` "spent real time at candidate before production-ready (RFC-057)"
and that advancing one rung at a time keeps the ladder meaningful. `matten-mlprep` has held candidate since
RFC-058 (2026-06-27) across releases `0.31.0` → `0.39.0` — comparable tenure, and with a functional
improvement during it rather than mere elapsed time. The ladder is respected.

## 6. Scope

### In scope

```text
the maturity label: production-ready candidate -> production-ready, at every site that states it
crates/matten-mlprep/README.md status banner
README.md (root) crate table row
docs/src/reference/compatibility.md
docs/src/examples/{companions,data,index}.md
docs/src/contributing/release-checklist.md
ROADMAP.md and rfcs/README.md tracking
```

### Out of scope

```text
any code, API, test, or example change
matten-data's label — it is a separate crate with a separate caveat (CSV-only,
  non-dataframe scope); this RFC says nothing about it
matten-stats's Experimental label
stratified / grouped / time-series splits
version bump, CHANGELOG, release prep, tag, publish
RFC-076 updates
```

## 7. What this does NOT claim

Stating this explicitly, because a maturity promotion is easy to over-read:

```text
it does not promise stratified/grouped/time-series splitting
it does not change any API or behaviour
it does not make the family v1.0-ready
it does not affect matten-data, which remains candidate for its own reasons
it does not imply matten-stats is near promotion — that crate is Experimental
  with three-week-old APIs and an unreviewed policy divergence
```

## 8. Compatibility

| Dimension | Impact |
|---|---|
| Public Rust API | **None** |
| Runtime behavior | None |
| Dependencies / features / MSRV | None |
| Version | None — no bump |
| Maturity label | `matten-mlprep`: candidate → **production-ready** |
| RFC-067 family maturity table | One fewer candidate. `matten-data` (candidate) and `matten-stats` (Experimental) remain |

**Consequence for a future v1.0:** RFC-067's MD-1 resolution permits candidate-labelled crates in a 1.0
family. This promotion reduces how much that provision must carry, but does not remove the need for it —
`matten-data` is still a candidate, and `matten-stats` raises the separate, unanswered question of whether a
1.0 family may include an `Experimental` crate (RFC-079 §9).

## 9. Acceptance criteria

```text
[ ] every site stating matten-mlprep's maturity says production-ready
[ ] no site accidentally promotes matten-data or matten-stats
[ ] no code, API, test, example, dependency, MSRV, or version change
[ ] the RFC-058 §5.1 Option B basis is recorded in the RFC and tracking, so a
    future reader sees why the deferral ended rather than that it simply lapsed
[ ] no CHANGELOG entry (this is not a release; the label ships with whatever
    release comes next)
[ ] guards pass, including the ROADMAP header/history parity check
```

## 10. Non-goals

```text
[ ] promoting matten-data or matten-stats
[ ] any code change
[ ] any release, version bump, tag, or publish
[ ] resolving the Experimental-in-a-1.0-family question
[ ] adding stratified/grouped/time-series splits
```

## 11. Follow-up

The label change ships with whatever release comes next; it does not justify a release on its own. If the
next release is `0.40.0`, its CHANGELOG should state the promotion explicitly — RFC-067 requires that no
wording imply a *silent* maturity change, and that requirement cuts both ways.
