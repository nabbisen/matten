# RFC-118: Confirm CI Green Before Tagging

**Status:** **Implemented** 2026-08-09 in commit *"Confirm CI green before tagging, and count the
ninth guard (RFC-118)"* (`3e298df`), reviewed and approved after one correction — the step was first
placed under *Additional gates for minor releases*, where a patch release would have been entitled to
skip it. No release; documentation only. Handoff:
`rfcs/handoffs/118-confirm-ci-before-tagging-handoff.md`.
**Target:** `docs/src/contributing/release-checklist.md`
**Theme:** Add the missing step between *push* and *tag*
**Related:** RFC-094, RFC-117, rule 002 §8

---

## 1. Summary

```text
A. Add a step to the release sequence: push -> CONFIRM CI GREEN -> tag -> publish.
B. Section 1 lists EIGHT guard scripts. There are nine.
```

Documentation only. No `crates/` change, so no release.

## 2. Why — `0.46.0` was tagged and published across four red runs

```text
31287004992  success   Record the 0.45.0 release     last green
31301270368  FAILURE   Close RFC-114 preparation
31301275565  FAILURE
31301318598  FAILURE   Record the 0.46.0 release     <- tagged and published here
31313280917  FAILURE   Prune two shipped rows        <- the owner spotted it
```

Every local gate passed. Nine checks, clippy, both feature profiles, a publish dry-run, and
verification against the sparse index — and the workflow result on the commit just pushed was never
looked at, though it already existed by then.

**RFC-117 fixed the break and closed the local blindness.** It did nothing about this: no script can
stop a person tagging without reading a result. The checklist can.

## 3. The gap, measured

```text
grep -inE "github actions|workflow|\bCI\b" release-checklist.md
  -> five hits, ALL about MSRV ranges and future gating.
  -> ZERO steps that check a workflow result before or during a release.
```

The page covers source verification, the feature matrix, examples, MSRV, a public-API audit, a
documentation truth pass, the CHANGELOG, the version bump, and publishing — in detail, including the
sparse-index verification and why the JSON API cannot be used. **It has no step that looks at CI.**

## 4. Change A — the sequence

Add an explicit step. The ordering is forced and worth stating with its reason:

```text
1. push main            CI runs on push. It cannot report on an unpushed commit.
2. CONFIRM CI GREEN     on the pushed commit specifically — not "CI was green
                        recently", not the previous run.
3. tag                  push must already have happened: a tag pointing at a commit
                        absent from the remote is the orphaned-tag defect this
                        project repaired once, for 0.38.0/0.39.0.
4. publish
```

**Step 2 is the new one.** Steps 1, 3 and 4 already happen; only 3 and 4 are written down.

State how to check, so the step is executable rather than aspirational:

```bash
gh run list --limit 5          # or the Actions tab
```

And state the failure mode plainly: **a red run at step 2 stops the release.** Not "investigate and
proceed" — stop. `0.46.0` is the case that shows why: the failure was real, it was a stale test
rather than broken code, and it *still* meant a tagged release carried a red build for anyone who
looked.

## 5. Change B — the guard count

Section 1 lists eight `scripts/*.sh` invocations. `check-tool-tests.sh` (RFC-117) makes nine, and it
is the one that would have caught `0.46.0`'s break. Add it.

**Check the count rather than trusting this RFC** — `ls scripts/*.sh | wc -l` is the authority, and
this RFC's own figure is the kind that has been wrong repeatedly in this project.

## 6. Scope

### Out of scope — a diff touching these is a defect

```text
any crate, any .rs file, any script
the publishing section's content — it is correct and detailed
RFC-094's cadence policy — this is about sequence, not about when to release
automating the check — a workflow-status gate is a separate, larger change
```

## 7. Risks

```text
R1  Writing the step as advisory ("check CI") rather than blocking ("a red run
    stops the release"). The failure being corrected is a reviewer who saw no
    reason to look; softer wording reproduces it.
R2  Adding the step but not saying it applies to the PUSHED COMMIT. "CI is green"
    was true of an older commit throughout the 0.46.0 incident.
R3  Trusting §5's count of nine rather than deriving it.
R4  Turning this into a workflow-automation change. Out of scope.
```

## 8. Acceptance criteria

```text
[x] the release sequence documents push -> confirm CI green -> tag -> publish
[x] the CI step names the PUSHED COMMIT and says a red run STOPS the release
[x] the ordering reason for push-before-tag is stated (orphaned tags)
[x] an executable way to check is given
[x] section 1's guard list matches `ls scripts/*.sh`, derived not copied
[x] no crate, no .rs, no script touched — assert via git diff --stat
[x] nine guards; check-doc-code.sh under RUSTFLAGS="-D warnings"; mdbook build clean
[x] no version bump, tag, or publish
```

## 9. What this does not claim

A checklist step is weaker than a guard. It works only if read. **That is the honest limit** — the
mechanical alternative is a release script or a branch protection rule that refuses to tag on a red
commit, and neither is in scope here.

This RFC makes the omission visible to the next person. It does not make it impossible.
