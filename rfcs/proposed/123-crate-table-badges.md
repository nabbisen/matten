# RFC-123: Move the Per-Crate Badges into the Crate Table

**Status:** Proposed
**Target:** `README.md`, `scripts/check-release-docs.sh`
**Theme:** Make an incomplete crate list structurally visible instead of merely unlikely
**Related:** RFC-030 (lock-step versioning), RFC-117 (guards that cannot pass vacuously), RFC-121
(sequencing), rule 002 §4

---

## 1. Summary

```text
A  move the four crates.io and four docs.rs badges out of the flat block at the
   top and into the Crates table, one row per crate
B  add matten-stats' two badges — MISSING for eight releases (§2)
C  drop the Version column: five identical hand-maintained strings, made
   redundant by the sentence directly beneath the table
D  replace the guard that column leaves behind, which would otherwise pass
   vacuously, with one that checks the invariant that actually just failed
```

**No `crates/` change, so no release.** The root `README.md` reaches zero published packages.

## 2. This starts from a live defect, not a preference

`matten-stats` has **no crates.io badge and no docs.rs badge**, while the other four crates have both.

```text
docs.rs badges     matten, matten-data, matten-mlprep, matten-ndarray
crates.io badges   matten, matten-data, matten-mlprep, matten-ndarray
published crates   ...and matten-stats
```

It has been published since `0.39.0` — **eight releases** — and it *is* in the crate table. Only the
badge block forgot it.

**The cause is structural and it is the whole argument for this change.** A flat list of eleven badges
has no property that says "there should be five of each." Nothing about it is wrong-looking when one
is missing. A table with one row per crate does have that property: every crate has a row, and an
empty cell in a row is visible at a glance.

This is RFC-117's principle applied to a document — **make the defect impossible to miss rather than
trusting nobody will make it.** The audit that opened this sequence found four statements that went
false because nothing pointed back at them; this is the same shape, in a list.

## 3. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | Four crates have docs.rs badges; four have crates.io badges; **five crates are published** | `grep -oE 'docsrs/[a-z-]+' README.md`, `grep -oE 'crates/v/[a-z-]+' README.md`, `ls crates/` |
| E2 | `matten-stats` has neither | direct read — zero matches for it in any badge line |
| E3 | It has been published for **eight releases**, since `0.39.0` | `git tag -l` from `0.39.0`; RFC-078 added the crate |
| E4 | The crate table already has **five** rows, one per crate — the omission is only in the badge block | `grep -cE '^\| \[`matten' README.md` → 5; `ls -d crates/*/` → 5 |
| E5 | The `Version` column holds `0.46.x family` **five times**, identical | direct read, `README.md:29-33` |
| E6 | The same fact is stated in **prose immediately below the table**: *"All crates share one **family version** (RFC-030): matching numbers mean a matched, compatible set."* | `README.md:34-36` |
| E7 | Those five cells are retargeted at every **minor** release | RFC-121 §4.3 — the five `0.46.x` references in `README.md` |
| E8 | `check-release-docs.sh` guards that column against bare patch versions | `scripts/check-release-docs.sh:430-437` |
| E9 | Root `README.md` reaches **zero** of the five published packages | `cargo package --list` per crate |

**Re-derive E1 and E4 before editing.** If the badge counts are not 4/4/5, the premise has changed.

## 4. Change A and B — the table

Repo-level badges stay at the top; per-crate badges move into the table.

```text
STAYS AT THE TOP    license, CI Test, CI Docs   — these describe the repository,
                    not any one crate, and have no row to live in
MOVES INTO A ROW    crates.io and docs.rs, per crate
ADDED               matten-stats' two, closing §2
```

```text
| Crate | crates.io | docs.rs | Status | What it is |
```

**Every row gets both badges. A row without them is the defect this RFC exists to prevent** — do not
leave one blank on the grounds that a crate is newer or smaller.

## 5. Change C — drop the Version column

E5 and E6 together are the argument: the column repeats one value five times, and **the sentence
directly beneath the table already states it better** — including the part the column cannot express,
that maturity is the Status column and not the version.

```text
GAINED   five hand-maintained cells removed from every minor release's retarget
         (E7), replaced by a live crates.io badge that cannot go stale
LOST     nothing. E6's prose survives untouched and says more than the column did.
```

**Do not delete or reword E6's paragraph.** It is what makes dropping the column safe, and it now
carries the fact alone.

## 6. Change D — the guard, which must not be left passing vacuously

`check-release-docs.sh:430-437` checks that the table's version cell is not a bare patch version.
**Delete the column and that grep can never match again** — it would pass forever, on every tree,
proving nothing.

That is precisely the failure mode RFC-117 named when it required the ninth guard to *"error loudly
if it derives zero commands rather than passing vacuously."* Leaving it is worse than deleting it,
because a green check that cannot fail reads as coverage.

**Replace it with a guard on the invariant that actually failed (§2):**

```text
for every directory under crates/
    there is a row in README.md's crate table naming it
    that row contains a crates.io badge for it
    that row contains a docs.rs badge for it
otherwise ERROR, naming the crate and what is missing
```

This would have caught `matten-stats` eight releases ago, and it catches the next crate added to the
family on the day it is added.

**Prove it can fail** (rule 002 §4): delete one badge, confirm the guard errors and names the crate,
restore it. Report both outputs. A guard accepted without that proof is the thing this change is
replacing.

## 7. Sequencing — this must land AFTER `0.46.1`

**Hard constraint, not a preference.**

RFC-121 is accepted and in flight. Its measurement is 21 pins across 14 files, and it explicitly
**asserts that the five `0.46.x` family references in `README.md` are unchanged** (RFC-121 §4.3) —
those five are exactly the Version cells Change C deletes.

```text
implementing this BEFORE 0.46.1  -> RFC-121's 21/13 figures become wrong while
                                    its implementer is working from them, and its
                                    "assert no 0.46.x moved" criterion fails on a
                                    change it never made
```

Do not start Change C until `0.46.1` is tagged and published.

## 8. Scope

### Out of scope — a diff touching these is a defect

```text
any crates/** file                      — no release; the badge block is root-only
E6's family-version paragraph            — it must survive verbatim (§5)
the license and CI badges                — they stay at the top (§4)
the Status column or any Status value    — maturity labels are RFC-084/085's
the "What it is" descriptions            — unchanged
docs/src/** and the book                 — the book has its own crate pages
RFC-121 and the 0.46.1 preparation       — untouched, and this waits for it (§7)
```

## 9. Risks

```text
R1  Starting before 0.46.1, invalidating RFC-121's pin measurement (§7).
    The highest-consequence risk here and the easiest to trip.
R2  Leaving the old guard in place after its column is gone — a green check
    that cannot fail (§6).
R3  Adding the new guard without proving it can fail (rule 002 §4).
R4  Deleting or rewording E6's paragraph, which now carries the family fact
    alone (§5).
R5  Moving the license or CI badges into the table. They are repo-level and
    have no row.
R6  Leaving a row without both badges — the defect being fixed (§4).
R7  Widening the table until "What it is" becomes unreadable. If five columns
    crowd it, say so and propose a shape rather than shipping a cramped table.
```

## 10. Acceptance criteria

```text
[ ] all five crates have BOTH a crates.io and a docs.rs badge, in their row
[ ] matten-stats specifically — the defect in §2 — is closed
[ ] license and CI badges remain at the top, outside the table
[ ] the Version column is gone; E6's paragraph is byte-identical
[ ] the old crate-table version check is REMOVED, not left passing vacuously
[ ] the new guard exists, and is PROVEN able to fail by deleting one badge —
    both outputs reported
[ ] the new guard names the crate and what is missing, not just "failed"
[ ] git diff touches no crates/** path; cargo package --list unchanged
[ ] nine guards (ten after this) pass; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] 0.46.1 is already tagged and published before this starts (§7)
[ ] no version bump, tag, or publish
```

## 11. What this does not fix

```text
- the ROADMAP Status block, still describing 0.41.0            (audit F5)
- the v1.0 readiness audit, nine releases stale                (audit F6)
- SECURITY.md, awaiting a disclosure contact                   (audit F10)
- the three tools' unsafe policy                               (audit F11)
- mechanically blocking a tag on red CI                        (RFC-118 §9)
```

And it does not give the project a guard that can read a *claim*. This one checks that a row exists
and carries two links — structure, not truth. The four false statements the audit found would still
pass it.
