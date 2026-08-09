# Developer Handoff — RFC-118: Confirm CI Green Before Tagging

**From:** High-capability model. **Date:** 2026-08-09.
**Design authority:** `rfcs/accepted/118-confirm-ci-before-tagging.md`
**Base:** `main` @ `71b3347`, clean tree.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. A pure addition in the handoff you can simply omit may be
> omitted and reported prominently; anything else, ask first.

---

## 1. Task title

Add a blocking CI-confirmation step to the release sequence in
`docs/src/contributing/release-checklist.md`, and correct §1's guard list from eight scripts to nine.

**Documentation only. No crate, no `.rs`, no script.**

## 2. Why this exists

`0.46.0` was tagged and published across **four consecutive red CI runs**. Every local gate passed —
nine checks, clippy, both feature profiles, a publish dry-run, sparse-index verification — and the
workflow result on the commit just pushed was never looked at, though it already existed.

**This is the reviewer's failure, not a process nobody could follow.** But the honest finding is that
the step does not exist: the checklist has no CI reference at all (§3). RFC-117 closed the local
blindness with a ninth guard; nothing closes this.

## 3. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | Four consecutive red runs spanned the tag and publish | `gh run list` — `31301270368`, `31301275565`, `31301318598`, `31313280917`; last green before them `31287004992` |
| E2 | The checklist has **zero** CI-checking steps | `grep -inE "github actions\|workflow\|\bCI\b" docs/src/contributing/release-checklist.md` → 5 hits, all about MSRV ranges and future gating |
| E3 | §1 lists **eight** `scripts/*.sh` invocations | `docs/src/contributing/release-checklist.md`, section *1. Source verification* |
| E4 | There are **nine** scripts | `ls scripts/*.sh \| wc -l` → 9 |
| E5 | The missing one is `check-tool-tests.sh` | RFC-117; and it is the guard that would have caught `0.46.0`'s break |
| E6 | Tag format is already documented, the sequence is not | line 15 — *"Release tags use bare SemVer with no `v` prefix"*; no push/tag/publish ordering anywhere |
| E7 | Publishing is documented in detail | the *Publishing: one workspace command* section, including sparse-index verification |

**Re-derive E3 and E4 yourself.** Do not take nine from this handoff — count them. Figures quoted
from memory in this project have been wrong repeatedly today, including mine in the RFC this handoff
accompanies.

## 4. Change A — the sequence

Add a step. The ordering is forced, and the reason for each constraint should be written down, not
just the order:

```text
1. push main         CI runs on push; it cannot report on an unpushed commit
2. CONFIRM CI GREEN  on the PUSHED COMMIT — the new step
3. tag               push must precede this: a tag on a commit absent from the
                     remote is the orphaned-tag defect repaired for 0.38.0/0.39.0
4. publish
```

Three things the wording must carry, each corresponding to a way the step could fail to work:

```text
BLOCKING      "a red run STOPS the release" — not "investigate", not "check".
              The failure being fixed is a reviewer who saw no reason to look;
              advisory wording reproduces it exactly.
THE PUSHED    "CI is green" was TRUE of an older commit throughout the 0.46.0
COMMIT        incident. Name the commit, not the branch.
EXECUTABLE    give a command — `gh run list --limit 5`, or the Actions tab.
              A step with no way to perform it is a step that gets skipped.
```

**Where to put it** is your judgment. It is a *sequence* step, so it may not belong inside
*Before every release*'s numbered source-verification list — the publishing section is closer to it
in time. Pick a placement and say why in the review request.

## 5. Change B — the guard list

§1 lists eight (E3). Add the ninth. **Derive the list, do not copy mine** — E4's command is the
authority, and if your count differs from nine, that discrepancy is worth more than the edit.

Match the existing format: one `bash scripts/…` line with a trailing comment naming the RFC or the
invariant it guards.

## 6. Out of scope

```text
any crate, any .rs file, any script
the publishing section's CONTENT — correct and detailed, leave it
RFC-094's cadence policy — this is sequence, not timing
automating the check (a release script, branch protection) — larger, separate
```

## 7. Acceptance criteria

```text
[ ] the sequence push -> confirm CI green -> tag -> publish is documented
[ ] the CI step is BLOCKING in its wording, names the PUSHED COMMIT, and gives a
    command (§4)
[ ] the push-before-tag reason is stated (orphaned tags)
[ ] §1's guard list matches `ls scripts/*.sh` — derived, and the count reported
[ ] placement chosen and justified in the review request
[ ] no crate, no .rs, no script — assert via git diff --stat
[ ] nine guards; check-doc-code.sh under RUSTFLAGS="-D warnings"; mdbook build clean
[ ] no version bump, tag, or publish
```

## 8. Risks

```text
R1  Advisory wording (§4). The single most likely defect, and it silently
    preserves the failure.
R2  Omitting "the pushed commit" (§4).
R3  Copying nine from this handoff instead of deriving it (§5).
R4  Editing the publishing section's existing content.
R5  Drifting into automation. Out of scope.
```

## 9. Required review-request format

Write to:
`.git-exclude/review-request/RFC-118/matten-rfc118-confirm-ci-before-tagging-implementation-review-request-v0.1.md`

Include files changed with line counts, the §3 verification with any discrepancy, **your derived
guard count**, where you placed the step and why, the exact wording of the blocking sentence, guard
and `mdbook build` output, deviations with reasoning, and anything you want answered at review.
