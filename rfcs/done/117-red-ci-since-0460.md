# RFC-117: CI Has Been Red Since Before `0.46.0` Was Tagged

**Status:** **Implemented** 2026-08-09 in commit *"Fix red CI: invert two stale header_only
assertions, add missing CHANGELOG bullet, and a ninth guard (RFC-117)"* (`942caff`), reviewed and
approved with **no corrections**, and pushed. No release — no `crates/` change. Handoff:
`rfcs/handoffs/117-red-ci-since-0460-handoff.md`.
**Target:** `tools/matten-report/tests/`, `CHANGELOG.md`, `scripts/`
**Theme:** Fix the break, document what actually shipped, and close the gap that hid both
**Related:** RFC-111, RFC-114, rule 002 §8

---

## 1. Summary

```text
A. process-boundary.sh's header_only cases assert behaviour RFC-111 deliberately
   changed. Invert them.
B. The [0.46.0] CHANGELOG entry omits matten-data entirely, while 0.46.0 changed
   matten-data's published behaviour. Add the missing bullet.
C. Nothing local runs the workspace-excluded tools' shell tests. Close that gap.
```

**No `crates/` change, so no release** — the fix is a test, a record, and a guard.

## 2. What happened, and it is the reviewer's failure

**CI has been red since the RFC-114 preparation push**, three runs before the one that surfaced it:

```text
31287004992  success   Record the 0.45.0 release        <- last green
31301270368  FAILURE   Close RFC-114 preparation
31301275565  FAILURE
31301318598  FAILURE   Record the 0.46.0 release
31313280917  FAILURE   Prune two shipped rows           <- reported by the owner
```

All four carry the **same** failure: `process-boundary: FAIL: header_only_absent: expected a nonzero
exit`.

**`0.46.0` was tagged and published on a commit whose CI was already red.** I authorized the push,
the tag and the publish, verifying eight guards, clippy, `cargo test --workspace` and the sparse
index — and never looked at the workflow status on the commit I had just pushed.

Rule 002 §8 exists for precisely this and says *"prove it in the environment it runs in."* I wrote
that rule after the `check-doc-code.sh`/`RUSTFLAGS` incident and did not apply it at the release gate.

## 3. Root cause — RFC-111, and why local verification could not see it

`header_only_absent` runs `matten-report` against a header-only CSV and asserts a **nonzero exit**.
That held because `NumericTable::to_tensor()` wrapped core's rejection of a zero-sized shape.

**RFC-111 removed that rejection**, so a 0-row table now yields an empty tensor and the tool succeeds.
RFC-111's own review *did* catch the sibling case — it inverted
`zero_row_table_cannot_become_tensor` in `matten-data` — but nothing pointed at the shell test in a
workspace-excluded tool.

### 3.1 The gap, measured

```text
scripts/*.sh                 do NOT run the tools' shell tests — grep returns nothing
Cargo.toml `exclude`         tools/ is out of the workspace, so `cargo test --workspace`
                             never reaches them
.github/workflows/test.yaml  lines 203-205 are the ONLY thing that runs them
```

**Every local gate this project uses is blind to `tools/*/tests/*.sh`.** Both RFC-111's
implementation and its review ran the full local suite and passed.

## 4. Change A — invert the two `header_only` cases

`process-boundary.sh` has two: `header_only_absent` (§187) and `header_only_existing` (§199). Both
assert a process error. The behaviour they assert was deliberately changed and the tests are now
wrong — not the behaviour.

**Do not "fix" `matten-report` to error again.** RFC-111 accepted zero-sized dimensions on purpose,
and `matten-data`'s own doc comment had anticipated this exact case.

`module-boundaries.sh` passes and is untouched.

## 5. Change B — `0.46.0` shipped an undocumented behaviour change

The `[0.46.0]` entry names core `matten`, `matten-ndarray` and `matten-mlprep`. It **does not mention
`matten-data` at all** — verified, zero occurrences.

But `0.46.0` changed `matten-data`'s published behaviour: `NumericTable::to_tensor()` on a header-only
CSV returned `Err` in `0.45.0` and returns `Ok` with an empty tensor in `0.46.0`. A user reading the
entry to decide whether to upgrade cannot see it.

**Add the bullet to the existing `[0.46.0]` entry.** This is correcting the record to match what
shipped, not rewriting history — the behaviour *is* in `0.46.0`. Say plainly what changed and that it
follows from the zero-sized acceptance.

## 6. Change C — close the gap

A ninth guard that runs the workspace-excluded tools' own test suites, so a local run sees what CI
sees.

```text
scripts/check-tool-tests.sh   runs tools/matten-report/tests/process-boundary.sh,
                              module-boundaries.sh, and module-boundaries.sh --self-test
```

**Derive the list from `test.yaml`, do not hand-copy it** — a hand-copied list is the next thing to
drift, and this RFC exists because of a gap between what CI runs and what anything local runs. If the
guard cannot derive it, say so and hand-copy with a comment naming the risk.

**Prove the guard can fail** (rule 002 §4): revert one `header_only` assertion, confirm the guard
rejects it, restore, confirm it passes.

## 7. Scope

### Out of scope — a diff touching these is a defect

```text
any crate, any .rs file — the behaviour is correct and stays
matten-report's own source — only its tests are wrong
CHANGELOG entries other than [0.46.0]'s missing bullet
a version bump, tag, or publish
```

## 8. Risks

```text
R1  "Fixing" matten-report to error again, restoring red-adjacent behaviour that
    RFC-111 deliberately removed (§4).
R2  Rewriting the [0.46.0] entry rather than adding the missing bullet. The entry
    is otherwise accurate and was reviewed.
R3  A hand-copied guard list that drifts from test.yaml (§6) — the same class of
    gap this RFC closes.
R4  Only fixing header_only_absent. There are TWO cases; the script stops at the
    first failure, so the second is untested until the first passes.
```

## 9. Acceptance criteria

```text
[x] process-boundary.sh passes; BOTH header_only cases inverted (R4)
[x] module-boundaries.sh and its --self-test still pass, unmodified
[x] matten-report's source unchanged — assert via git diff --stat
[x] the [0.46.0] entry gains a matten-data bullet; nothing else in CHANGELOG changes
[x] scripts/check-tool-tests.sh exists, runs the same set as test.yaml, and is
    PROVEN to fail on a reverted assertion
[x] all nine guards pass
[x] no crate, no .rs file touched
[x] no version bump, tag, or publish
```

## 10. What this does not fix

`0.46.0` is published and its crates are correct — the regression is in a test, not in shipped code.
No yank is warranted. **The undocumented change (§5) reached users, and the CHANGELOG correction is
the only remedy available**, since the behaviour itself is intended.
