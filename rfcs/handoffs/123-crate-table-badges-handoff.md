# Developer Handoff — RFC-123: Move the Per-Crate Badges into the Crate Table

**From:** High-capability model. **Date:** 2026-08-28.
**Design authority:** `rfcs/accepted/123-crate-table-badges.md`
**Base:** to be taken **after** `0.46.1` is tagged and published — see §1.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. A pure addition in the handoff you can simply omit may be
> omitted and reported prominently; anything else, ask first.

---

> **AMENDED 2026-09-01 — most of this task is already done.** The owner performed Changes A, B and C
> directly. **Your task is now Change D only: replace the guard.**
>
> ```text
> DO NOT move badges. DO NOT edit the crate table. DO NOT remove the Version
> column. All of that is finished, and redoing it would revert the owner's layout.
> ```
>
> The guard is now **provably** vacuous — its pattern matches **0** rows in `README.md` and can never
> match again, which is precisely the condition §6 was written to prevent. Read §6 and §6.1; ignore
> §4 and §5.
>
> **The blocker below is also lifted** — `0.46.1` shipped on 2026-08-31.

## 1. ~~DO NOT START THIS YET~~ — unblocked; the reasoning is kept as record

~~**This task is blocked, and starting it early breaks work already in flight.**~~ **LIFTED — `0.46.1` shipped 2026-08-31.** The original reasoning is kept below as the record of why it was held.

```text
RFC-121 (0.46.1) is accepted and in progress. Its acceptance criteria include:

    "all 13 '0.46.x' references UNCHANGED — asserted explicitly, by diff"

FIVE of those thirteen are README.md's Version cells — exactly what Change C
deletes. Deleting them while RFC-121 is being implemented makes its 21/13
measurement wrong and fails a criterion on a change its implementer never made.
```

```text
START ONLY WHEN   0.46.1 is tagged AND published
IF UNSURE         ask. Do not infer it from a green CI run or a merged commit.
```

Finish RFC-121 first. This one is not urgent — the defect it fixes is eight releases old.

## 2. Task title

Move the eight per-crate badges into the Crates table, add `matten-stats`' two missing ones, delete
the Version column, and replace the guard that column leaves behind.

**No `crates/` change. No release.**

## 3. What is actually wrong today

`matten-stats` has **no crates.io badge and no docs.rs badge**, though it has been published since
`0.39.0` — eight releases — and already has a row in the crate table.

```text
docs.rs badges     matten, matten-data, matten-mlprep, matten-ndarray   (4)
crates.io badges   matten, matten-data, matten-mlprep, matten-ndarray   (4)
published crates   ...and matten-stats                                  (5)
```

**Re-derive this before editing.** If the counts are not 4/4/5, the premise has changed and that is
worth more than the edit.

> The point of this RFC is not tidiness. A flat badge block has no property that says *"there should
> be five of each"*, so nothing looks wrong when one is missing. A row-per-crate table does. You are
> converting a defect that is easy to make into one that is hard to miss.

## 4. Change A/B — the table

```text
STAYS AT THE TOP   license, CI Test, CI Docs — repo-level, no row to live in
MOVES INTO A ROW   crates.io and docs.rs, one pair per crate
ADDED              matten-stats' two, closing §3
```

```text
| Crate | crates.io | docs.rs | Status | What it is |
```

Reuse the existing badge URLs verbatim — they are correct and already absolute. `matten-stats`' two
follow the same shields.io pattern as its four siblings.

**Every row gets both badges.** A blank cell is the defect being fixed.

## 5. Change C — delete the Version column

It holds `0.46.x family` five times. The paragraph **directly beneath the table** already says it, and
says more:

```text
"All crates share one **family version** (RFC-030): matching numbers mean a
 matched, compatible set. A crate's **maturity is the Status column**, not its
 version number — a crate may sit at the shared family version and still be `beta`."
```

```text
DO NOT touch that paragraph. Not a word. After Change C it carries the family
fact ALONE, which is what makes deleting the column safe.
```

## 6. Change D — the guard. This is the part with teeth

`scripts/check-release-docs.sh:430-437` checks the crate table's version cell for bare patch
versions. **Once the column is gone that grep can never match — it would pass on every tree forever.**

```text
DELETE the old check. Do not leave it. A green check that cannot fail reads as
coverage and is worse than no check — RFC-117's ninth guard was written with
exactly this instruction ("error loudly rather than pass vacuously").
```

Replace it with a guard on the invariant that actually failed:

```text
for every directory under crates/
    assert README.md's crate table has a row naming it
    assert that row contains a crates.io badge for it
    assert that row contains a docs.rs badge for it
on failure: ERROR naming THE CRATE and WHICH badge is missing
```

**Derive the crate list from `crates/` on disk, not from a hand-written list.** A hand-copied list is
the next thing to fall out of step, which is the whole defect being fixed.

### 6.1 Prove it can fail — required, rule 002 §4

```text
1. with the change applied and the guard passing, DELETE one badge
2. run the guard; confirm it FAILS and names that crate and that badge
3. restore the badge; confirm it passes again
4. report all three outputs in the review request
```

A guard accepted without this is exactly what this RFC is replacing.

## 7. Out of scope

```text
any crates/** file                    — no release
the family-version paragraph          — byte-identical (§5)
license and CI badges                 — they stay at the top (§4)
Status values, "What it is" text      — unchanged
docs/src/** and the book              — not this table
RFC-121 / 0.46.1                      — and this waits for it (§1)
```

## 8. Acceptance criteria

```text
[ ] 0.46.1 tagged and published BEFORE this starts (§1)
[ ] all five crates carry BOTH badges in their row; matten-stats specifically
[ ] license and CI badges still at the top, outside the table
[ ] Version column gone; the family paragraph byte-identical
[ ] the old crate-table version check REMOVED, not left passing vacuously
[ ] the new guard derives its crate list from crates/ on disk
[ ] the new guard PROVEN able to fail (§6.1) — all three outputs reported
[ ] the failure message names the crate and the missing badge
[ ] git diff touches no crates/** path; cargo package --list unchanged
[ ] ten guards pass; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] no version bump, tag, or publish
```

## 9. Risks

```text
R1  STARTING BEFORE 0.46.1 (§1). Highest consequence, easiest to trip.
R2  Leaving the old guard in place after its column is gone (§6).
R3  Adding the new guard without proving it can fail (§6.1).
R4  Editing the family-version paragraph (§5).
R5  Moving license or CI badges into the table (§4).
R6  A hand-written crate list in the guard instead of reading crates/ (§6).
R7  A cramped five-column table. If "What it is" becomes unreadable, SAY SO
    and propose a shape — do not ship it cramped and do not silently drop
    a column to make room.
```

## 10. Required review-request format

Write to:
`.git-exclude/review-request/RFC-123/matten-rfc123-crate-table-badges-implementation-review-request-v0.1.md`

Include files changed with line counts, your re-derived 4/4/5 counts with any discrepancy, §6.1's
three outputs, confirmation that `0.46.1` had shipped before you started, deviations with reasoning,
and anything you want answered at review.
