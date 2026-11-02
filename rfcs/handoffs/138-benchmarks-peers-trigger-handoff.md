# RFC-138 Developer Handoff — Drop the Weekly Schedule, Keep the Check

**RFC:** `rfcs/accepted/138-the-schedule-that-stops-guarding.md`
**Status:** Accepted 2026-11-03 by the owner.
**Target:** `.github/workflows/benchmarks-peers.yml`, `docs/src/contributing/release-checklist.md`
**Authority:** where this document and the RFC disagree, **the RFC wins.**

---

## 1. Scope

Two small edits. **Do not change either `run:` step** — both are correct, including the one that
duplicates `test.yaml:28`.

```text
A  remove `schedule:` from benchmarks-peers.yml, leaving workflow_dispatch
B  add the dispatch to the release checklist, after the push, before the tag
```

## 2. Change A — the workflow

```yaml
on:
  workflow_dispatch:
```

Replace the `schedule:` block with a comment saying why it is gone, in your own words. The substance
to convey: GitHub disables a scheduled workflow after 60 days of repository inactivity, and a
disabled workflow **does not fail — it stops**, staying stopped once activity resumes, so the check
moved to an event that cannot lapse unobserved.

Also update the file's **header comment**, which currently ends:

```text
# compile-checks the opt-in `peers` bench, on manual dispatch and a weekly schedule.
```

That sentence becomes false the moment you make this change. Leaving it is the exact defect class
this project has corrected repeatedly — a statement that was true when written.

```text
DO NOT touch either run: step.
DO NOT add push: or pull_request:. The architect ruling (RFC-049 §B3/§5) keeps
       the heavy peer deps out of the normal flow, and a path-scoped trigger on
       benchmarks/** fires only when someone touches the directory whose
       buildability is the question.
DO NOT delete the workflow. workflow_dispatch is the mechanism Change B uses.
```

## 3. Prove the manual path works *before* removing the automatic one

```bash
gh workflow run benchmarks-peers.yml
gh run list --workflow benchmarks-peers.yml --limit 1
```

**Order matters.** In 18 weeks and 19 runs this workflow has been triggered by `schedule` every
single time and by `workflow_dispatch` **zero** times. The manual path is therefore unexercised, and
this RFC makes it the only path. Confirm it dispatches and goes green before you take the schedule
away — not after.

If the dispatch fails for a reason unrelated to this change (an upstream `ndarray`/`nalgebra` break,
or a new stable rustc), **stop and report it**. That is the schedule's job finally producing a
signal, and it would be a finding worth having rather than something to work around.

## 4. Change B — the release checklist

In `docs/src/contributing/release-checklist.md`, section `### Push, confirm CI green, then tag`
(`:257`), the numbered sequence is currently:

```text
1. Push main
2. Confirm CI is green on the commit just pushed
3. Tag
4. Publish
```

Insert the dispatch as a new **step 3**, renumbering Tag to 4 and Publish to 5. Keep the existing
steps' wording — including the `0.46.0` paragraph at the end, which is the record of why step 2
exists.

**Why after the push and not in "Before every release":** a `workflow_dispatch` runs against a ref,
so dispatching after the push tests the commit that is about to be tagged. That is RFC-118's own
reasoning applied to a second check, and placing it earlier would test whatever the working tree
happened to hold.

Say in the step what a reader needs to decide with: this compile-checks the opt-in `peers` harness
against freshly-resolved peer dependencies, it is **not** a speed gate, and a failure means the
developer harness no longer builds — it does **not** mean anything is wrong with the crates about to
be published.

## 5. What this check does and does not protect — get this right in the prose

Someone will read the new checklist step and need to know whether a red run blocks the release.

```text
IT GUARDS    the `peers` bench still compiles (workspace-excluded,
             publish = false, opt-in behind --features peers)
IT DOES NOT  guard dependency isolation. scripts/check-published-dependency-
             isolation.sh runs on EVERY push (test.yaml:28), and reads only
             `cargo tree` over the five published crates
```

So a red `benchmarks-peers` does not by itself stop a release the way a red CI run does. Write the
step so that distinction is plain, and say what to do instead: investigate, and decide — it is a
judgement call, not a gate.

## 6. Verify

```text
[ ] `schedule:` gone; `workflow_dispatch:` retained
[ ] the header comment no longer claims a weekly schedule
[ ] both run: steps byte-identical
[ ] no push:/pull_request: trigger added
[ ] a manual dispatch ran green BEFORE the schedule was removed (§3)
[ ] the checklist's step is in `### Push, confirm CI green, then tag`, as the
    new step 3, with Tag and Publish renumbered
[ ] the 0.46.0 paragraph and steps 1-2 wording are unchanged
[ ] nine guards; mdbook build clean (you edited a book page)
[ ] no crates/ path touched; zero published packages affected
[ ] no version bump, tag, or publish
```

`docs/src` is inside the guards' `USER_DOCS`, so run the nine guards even though this looks like a
prose edit.

## 7. One commit

```text
Dispatch the peers compile-check from the release sequence instead of a weekly schedule (RFC-138)
```

## 8. Review request

Write `.git-exclude/review-request/RFC-138/`. Include the dispatch run's URL or id and its
conclusion, the workflow diff, and the checklist diff. If the dispatch surfaced a real upstream
break, that is the headline, not a footnote.
