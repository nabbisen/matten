# RFC-138: The Schedule That Stops Guarding

**Status:** **Accepted** 2026-11-03 by the owner. Handoff:
`rfcs/handoffs/138-benchmarks-peers-trigger-handoff.md`. Touches zero published packages; no version
bump, tag, or publish.
**Target:** `.github/workflows/benchmarks-peers.yml`, the release sequence
**Theme:** A weekly guard GitHub will switch off silently, protecting something cheaper than it looks
**Related:** RFC-049 Phase 2 (the architect ruling that created this workflow), RFC-117 (guards that
cannot pass vacuously), RFC-118 (the release sequence this joins), RFC-139 (the same failure shape,
different mechanism)

---

## 1. Summary

```text
DROP   the weekly `schedule:` from benchmarks-peers.yml
KEEP   `workflow_dispatch:` -- unchanged, and still the only way it has ever
       been run by a human (zero times in 18 weeks)
ADD    dispatching it to the release sequence, where the question matters
```

**The trigger is not the defect. The silence is.** GitHub disables a scheduled workflow after 60
days of repository inactivity, and a disabled workflow does not fail — it stops. No red X, no
further notice, and it stays stopped after the repository becomes active again.

## 2. Why this is an RFC and not a one-line edit

Because the thing being removed is a **guard**, and this project has now found three instances of the
same shape: a check whose stated coverage and actual coverage differ, passing confidently in
between.

```text
RFC-123   the crate-table guard matched zero rows and could never fail again
RFC-137   the stale-version patterns cannot see the file their comment names,
          which is how 0.39.0 shipped claiming to document the v0.38 API
this RFC  a scheduled guard that GitHub switches off without failing
```

A workflow that silently stops running is the purest form of it: not a pattern that matches nothing,
but a check that is not evaluated at all. Deciding to accept that deliberately is a different act
from letting it happen, and the difference belongs in writing.

## 3. What the schedule uniquely catches — the case for keeping it

Stated first and fairly, because it is real and no push-trigger replaces it.

```text
benchmarks/Cargo.lock is GIT-IGNORED (RFC-049 Q2), so the peer dependencies
    resolve FRESH on every run
dtolnay/rust-toolchain@stable FLOATS, so the compiler moves too
the `peers` bench is NOT built in ordinary CI, by architect ruling (§B3/§5):
    the peer deps are heavy and must not slow the normal flow
```

So the weekly run detects a new `ndarray`/`nalgebra` release, or a new stable rustc, breaking the
`peers` harness **with nothing changed in this repository**. A `push`/`pull_request` trigger cannot
see that, because there is no push when an upstream crate publishes. A path-scoped trigger on
`benchmarks/**` is worse still: it fires only when someone touches the very directory whose
buildability is in question, which could be months.

**This is the strongest argument against this RFC, and it is why §5 frames the change as a
deliberate downgrade rather than a free win.**

## 4. What it does *not* guard — the fact that decides the question

The workflow has two steps, and the second is already run on **every push**:

```text
benchmarks-peers.yml:24   bash scripts/check-published-dependency-isolation.sh
test.yaml:28              bash scripts/check-published-dependency-isolation.sh
```

That script's only inputs are `cargo tree -p <crate> --all-features --edges normal,build` over the
five published crates. It reads nothing from the benchmark build, the `peers` feature, build output,
or any lockfile — verified by reading it, not inferred from its name.

> The workflow's own comment calls this step *"Re-assert peer deps never reach the published
> crates"*, and **"re-assert" is honest** — it says plainly that this is a repeat. An earlier draft
> of this analysis described the comment as implying the workflow was load-bearing for isolation.
> That was unfair to the comment, and the correction matters: **there is nothing to fix in step 2.**

So whatever happens to the schedule, **isolation protection is untouched**. What the schedule alone
protects is a compile-check of the `peers` bench — and that harness is:

```text
workspace-excluded (Cargo.toml exclude, RFC-049)
publish = false
opt-in behind --features peers
```

Breakage there costs a developer who opts in a compile error. It reaches no published crate and no
user.

## 5. The track record

```text
19 runs     2026-06-29 -> 2026-11-02 (18 weeks)
19 success,  0 failure
19 schedule, 0 workflow_dispatch
~45s per run
```

**It has never fired, and nobody has ever dispatched it manually.**

This does not prove the guard is worthless — a quiet guard may simply have had nothing to catch, and
RFC-128 made exactly this argument about properties that pass. What it does establish is that the
cost of a cheaper design is low, and that the project has 18 weeks of evidence that peer breakage is
not a frequent event.

## 6. The change

```yaml
on:
  workflow_dispatch:
  # schedule: removed -- see RFC-138. GitHub disables a scheduled workflow after
  # 60 days of repository inactivity, and a disabled workflow does not fail, it
  # stops: no red X, and it stays stopped once activity resumes. This check is
  # therefore dispatched from the release sequence (RFC-118), where a human reads
  # the result, instead of relying on a trigger that can lapse unobserved.
```

Both steps stay exactly as they are.

**And the release sequence gains one step.** The operational file is
`docs/src/contributing/release-checklist.md` — not RFC-118, which is the *design record* for
confirming CI before tagging and needs no amendment for a different check.

> **Corrected on acceptance.** §8 R5 originally said the step must land "in RFC-118's sequence".
> RFC-118 is a `done/` design RFC; releases are actually run from the book's release checklist. The
> risk R5 names is real — a guarantee that lives only in an RFC does not exist — but it named the
> wrong file, which would have produced exactly the outcome it warns about.

It belongs in `### Push, confirm CI green, then tag` (`:257`), **after the push and before the
tag** — not in "Before every release" — for RFC-118's own reason: a dispatch runs against a ref, so
dispatching after the push tests *the commit that is about to be tagged* rather than whatever the
working tree happened to hold.

A release is the moment the question is actually live — the project is about to publish, and wants
to know the peer harness still builds and isolation still holds. It is an event that cannot be
switched off, and it puts a person in front of the answer.

### 6.1 What is deliberately given up

```text
Between releases, an upstream ndarray/nalgebra or rustc break goes undetected.
```

**Stated plainly rather than buried.** The project is trading automatic detection of a
developer-tool regression for a guarantee that cannot lapse silently. That trade is only defensible
because of §4 — the user-facing half runs on every push — and it should be revisited if the `peers`
harness ever becomes load-bearing for something published.

## 7. Alternatives considered and rejected

```text
A  Click GitHub's "prevent disabling" link.
   Rejected: it parks the invariant in repository SETTINGS rather than in
   version control. This project has refused that elsewhere, and a reviewer
   reading the repo could not tell whether it was still set.

B  Keep the schedule and add a push trigger.
   Rejected: the schedule still lapses silently, so the defect survives. The
   push trigger also reintroduces heavy peer deps into the normal flow, which
   the architect ruling (§B3/§5) specifically avoided.

C  Lengthen the cron to monthly.
   Rejected on a misreading worth recording: the auto-disable is driven by
   REPOSITORY inactivity, not by cron frequency. A monthly schedule is
   disabled on exactly the same clock.

D  Add a guard that detects "benchmarks-peers has not run in N days".
   Rejected as more machinery than the thing it protects. It also needs the
   GitHub API from inside a guard script, which no existing guard does.
```

## 8. Risks

```text
R1  Reading this as "the peers check is unimportant". It is a real check; the
    claim is narrower -- it protects a developer tool, not a published crate.
R2  Dropping the workflow entirely. workflow_dispatch MUST stay; it is the
    mechanism the release sequence will use.
R3  Touching either step. Both are correct, including the redundant one (§4).
R4  Adding a push or pull_request trigger (§7 B).
R5  Letting the release-sequence step live only in this RFC. It must land in
    `docs/src/contributing/release-checklist.md`, which is where releases are
    actually run from, or the guarantee this RFC claims does not exist.
    (This risk originally named RFC-118 -- see §6's correction.)
```

## 9. Acceptance criteria

```text
[ ] `schedule:` removed from benchmarks-peers.yml, with the reason in a comment
[ ] `workflow_dispatch:` retained
[ ] both run steps unchanged
[ ] `gh workflow run benchmarks-peers.yml` dispatches and passes, proving the
    manual path works before the automatic one is removed
[ ] `docs/src/contributing/release-checklist.md` records the dispatch step, in
    `### Push, confirm CI green, then tag`, after the push and before the tag
[ ] nine guards; no crates/ path touched; zero published packages affected
[ ] no version bump, tag, or publish
```

## 10. What this does not do

```text
- fix RFC-139's line-based guard patterns. Same failure shape, unrelated fix,
  its own RFC.
- make the peers harness part of ordinary CI. The architect ruling stands.
- authorize RFC-049 Phase 4 gates. Still unauthorized, still untouched.
- change what is published. benchmarks/ reaches zero published packages, so
  this RFC can land in any release or none.
```
