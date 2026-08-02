# RFC-094: Release Cadence Policy

**Status:** Proposed
**Target:** Process policy; no version, no release
**Theme:** Replace per-release negotiation with a written trigger, and say what a patch release is for
**Amends:** RFC-015 §4 (release gates) — adds *when* to release; RFC-015 keeps owning *whether it is fit to*
**Related:** RFC-030 (lock-step versioning), RFC-086, RFC-089, RFC-091, ROADMAP §1.1, org §6.4

---

## 1. Summary

Write down when a release happens.

RFC-015 §4 says what must be true before releasing. RFC-030 says every crate shares one version.
Neither says *when*, so the decision has been taken per release, in conversation, each time. This RFC
supplies the missing rule: **patches ship immediately, minors batch, documentation never ships.**

## 2. What the absence of a policy actually produced

Not a hypothetical. Measured at `7cbb46a`:

```text
0.36.0  2026-07-17     0.40.0  2026-07-30
0.37.0  2026-07-21     0.41.0  2026-07-31
0.38.0  2026-07-28     0.42.0  2026-08-01
0.39.0  2026-07-28  <- same day as 0.38.0
```

Four minor releases in five days, two of them on one day.

**Under lock-step this republishes crates that did not change.** Counting real `src` changes:

```text
0.40.0   4 of 5 crates changed  ->  1 republished unchanged
0.41.0   1 of 5 crates changed  ->  4 republished unchanged
0.42.0   2 of 5 crates changed  ->  3 republished unchanged
```

**Eight no-change crate versions across three releases.** That is inherent to RFC-030 and not a
defect — but crates.io has no unpublish, only yank, so each one is permanent. It is a real per-release
cost and it argues for fewer, fuller releases rather than for abandoning lock-step.

**The process cost is comparable.** Each release requires roughly 37 version-string retargets across
17 files, a release RFC, a Developer Handoff, a review round, a CHANGELOG entry, a signed tag, a
publish, and a disposition commit. `0.42.0` paid all of that to ship one function.

**And RFC-091 §2 records the trigger failing outright.** Its own §6.4 trigger — *"a second theme
lands"* — had not fired; the release proceeded on an owner override of a recommendation to wait. A
trigger that is overridden the first time it is tested is not a policy, it is a preference being
renegotiated per release.

## 3. Patch releases have quietly stopped existing

```text
last patch release   0.28.5, 2026-06-28
minor releases since 14
```

Fifty-three patch tags exist historically; `0.28.x` alone had five. Since `0.28` there have been
fourteen minors and **zero** patches. Nothing decided this — the option simply stopped being reached
for, and every fix has ridden along with the next minor.

That is fine when a fix is not urgent and harmful when it is. §4 restores the distinction by saying
what each level is *for*, rather than leaving it to whichever release comes next.

## 4. Decision

### 4.1 Patch — `0.4x.y`

**Trigger: as soon as the fix is reviewed. No batching, no waiting for company.**

```text
contents: correctness fixes to already-published crate code, and nothing else
excluded: any new public API, any behaviour change that is not a bug fix
```

A user hitting a wrong answer should not wait for an unrelated feature to be ready. This is the only
release type with no batching rule, deliberately.

### 4.2 Minor — `0.4x.0`

**Trigger: any one of the following.**

```text
(a) two or more themes have landed unreleased            — the batching rule
(b) 28 days have passed with anything unreleased         — the anti-rot floor
(c) the owner asks                                       — §6.7, overrides (a) and (b)
```

(a) is the existing informal rule, now written down. (b) exists so finished work cannot sit
indefinitely — the *"releasing nothing of what was built"* failure org §6.4 names. (c) is recorded
because it is what actually happened at `0.42.0`; an owner override is legitimate and should be a
named path rather than a departure from policy.

**The 28 days is a judgement, not a derivation.** There is no user data to derive it from, and ROADMAP
§1.1 states adoption is not a success measure, so no external cadence pressure exists. It is set to be
slow enough that a release means something and fast enough that nothing rots. It should be changed by
amendment if it proves wrong, not quietly ignored.

### 4.3 No release at all

```text
documentation, guards, scripts, ROADMAP, rfcs/, CI, the book, and anything else
that does not reach crates.io NEVER triggers a release
```

Already the practice — it is what was concluded when a CI-guard fix was proposed as `0.42.1` and
declined on the grounds that `cargo package --list` showed the changed file in zero of the five
published packages. Written down here so the reasoning does not have to be re-derived.

**The test is mechanical, not editorial:** if `git diff --name-only <last-tag>..HEAD -- crates/` is
empty, there is nothing to release.

## 5. What this does not change

```text
RFC-015 §4 still owns the release GATES — whether a release is fit to go out
RFC-030 still owns lock-step versioning; this RFC does not reopen it
the owner still authorizes every tag and every publish, separately (org §6.7)
the release RFC + handoff + review sequence is unchanged
v1.0 timing remains the owner's alone and outside this policy entirely
```

## 6. Consequences worth stating plainly

```text
- releases become less frequent and individually larger
- a finished feature may wait up to 28 days, visibly, by design
- the release-readiness checkpoint at each RFC disposition becomes mechanical:
  count unreleased themes, check the date, check crates/ — rather than a judgement
  call re-argued each time
- the no-change republish count per release falls, because more of the five crates
  will have actually changed
```

## 7. Acceptance criteria

```text
[ ] the three triggers and the patch/minor/no-release split are recorded normatively
[ ] docs/src/contributing/release-checklist.md points at this policy for WHEN,
    keeping its own scope as WHETHER-fit
[ ] the org workflow document's §6.4 checkpoint cites these triggers instead of
    describing an ad-hoc judgement
[ ] RFC-015 carries a reciprocal amendment note, per RFC-000's convention
[ ] no code, API, version, or release change
```

## 8. Non-goals

```text
changing lock-step versioning (RFC-030) or introducing per-crate versions
a v1.0 schedule — RFC-076 stays deferred, and §6.7 keeps that timing with the owner
automating the release, tag, or publish
a fixed calendar cadence — (b) is a floor, not a schedule
```
