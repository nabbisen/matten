# RFC-081 `Experimental` Crates in a v1.0 Family: Implementation Handoff

**Project:** `matten`
**Related RFC:** RFC-081 (design authority)
**Document kind:** Policy-application handoff
**Status:** Drafted for review; implementation unauthorized until RFC-081 and this handoff are accepted
**Date:** 2026-07-28

---

## 1. Purpose

Apply RFC-081's decision and refresh `rfcs/proposed/076-v1-release-preparation.md`'s stale inventory, as
one reviewable slice.

**Documentation only.** No code, no version, no release, no maturity change.

## 2. Preconditions

```text
RFC-081 and this handoff accepted
RFC-080 committed (91496bd) — matten-mlprep is production-ready
0.39.0 post-release alignment committed (1da1e3f) — matten-stats recorded as published
working tree clean; version stays 0.39.0
```

## 3. What this slice is, and is not

```text
IS      applying an accepted policy to a proposed, unexecuted RFC's inventory
IS      recording a rule that binds any FUTURE v1.0 release RFC

IS NOT  promoting, demoting, or relabelling any crate
IS NOT  deciding which exit matten-stats takes (RFC-081 §10 defers that)
IS NOT  executing, authorizing, or scheduling a v1.0 release
IS NOT  editing RFC-067 — its candidate rule is untouched
```

RFC-076 stays in `rfcs/proposed/`. It is being **corrected, not accepted, executed, or closed**.

## 4. Edit RFC-076 — by derivation, not by a fixed list

A prior draft of this section enumerated five sites by manual inspection. Review found it missed at
least seven more, including one (the per-crate `matten-mlprep` checklist block) whose *reasoning*, not
just its wording, had become void. Enumeration by one author reading the document once is the wrong
instrument — a maturity change touches every place that describes the family's current shape, not just
the places a first pass happens to notice. **Do this by sweep and classification instead:**

```bash
grep -nE "four|five|all .* crates|per-crate" rfcs/proposed/076-v1-release-preparation.md
grep -nE "candidate|Experimental|production-ready|maturity" rfcs/proposed/076-v1-release-preparation.md
grep -n "matten-stats" rfcs/proposed/076-v1-release-preparation.md          # expect zero before this slice
grep -nE "package --workspace|publish order|publish in dependency" rfcs/proposed/076-v1-release-preparation.md
```

Classify every hit as **current-status claim** (edit), **historical narrative** (leave), or **generic
policy / unrelated false-positive** (leave). Record the full classified list in the review request,
including the leave-alone hits, so a reader sees they were considered rather than missed again.

Running that sweep now, against this base commit, produces this classification — the actual answer,
not just the method:

### 4.1 Sites requiring an edit

| # | Line(s) | Says now | Change to |
|---|---|---|---|
| 1 | 29 | *"add CHANGELOG release notes stating each candidate label explicitly"* | matten-mlprep is no longer a candidate; state each companion's **actual** maturity label, and require `matten-stats`'s Experimental status (and which RFC-081 exit it took) to be named too |
| 2 | 66 | *"version bump 0.38.0 -> 1.0.0 (Cargo.toml + Cargo.lock, all four crates)"* | five crates; note the base is now `0.39.0`, not `0.38.0` (§6 below) |
| 3 | 81 | Out of Scope: *"companion maturity promotion (matten-mlprep and matten-data remain candidates)"* | `matten-mlprep` is production-ready (RFC-080, prior to and independent of this RFC); only `matten-data` remains a candidate under this fence |
| 4 | 187 | maturity table row: `matten-mlprep … production-ready candidate (RFC-058)` | **production-ready (RFC-080)**; caveat column cleared — RFC-077 closed the ordered-split caveat that justified it |
| 5 | after 188 | table ends at `matten-data` | **add a `matten-stats` row**: Experimental (RFC-078), published `0.39.0`, caveat *"unsettled surface (RFC-040 §9); the `ddof = 1` divergence from core shipped in `0.39.0` without external review — changing it now would break adopters"* (not "unreviewed externally" alone — that reads as a still-pending pre-publication gate, which it no longer is), inclusion **blocked pending RFC-081 §3 Exit A or B** |
| 6 | 193-198 | Per-crate RFC-067 checklist, `matten-mlprep` block: *"candidate label an acceptable documented caveat? yes — … the ordered-split limit"* / *"separate promotion RFC required? no — RFC-058 deferred …"* | **Rewrite as reasoning, not word-swap.** This block argues for admitting `matten-mlprep` *as a candidate* — an argument RFC-080 already acted on. Replace with: `matten-mlprep` was promoted to production-ready by RFC-080 before this RFC; RFC-067's candidate conditions no longer apply to it; it is unconditionally includable. Add a parallel `matten-stats` block: Experimental; RFC-067's candidate conditions do not apply (they govern candidates, not Experimental crates); inclusion is blocked by RFC-081 §3 until Exit A or B |
| 7 | 207-208 | *"Neither companion is promoted by this RFC. Both enter the `1.0.0` family at `production-ready candidate`…"* | False on both counts now: `matten-mlprep` was promoted (separately, before this RFC — not "by this RFC," but the sentence's premise that it's still a candidate is wrong), and a five-crate family does not have "both" companions at one label. Rewrite: `matten-data` enters at candidate (unchanged by this RFC); `matten-mlprep` enters at production-ready (promoted by RFC-080, prior to this RFC); `matten-stats` may not enter while Experimental (RFC-081) |
| 8 | 227 | *"lock-step family policy: all four crates share the version (RFC-030)…"* | five crates |
| 9 | 229 | `compatibility.md` content spec: *"maturity labels: matten-mlprep and matten-data ship at candidate label…"* | `matten-mlprep` ships at production-ready; `matten-data` at candidate; the spec must also require stating `matten-stats`'s resolved status (it cannot still be Experimental if this RFC is executed — RFC-081 §3) |
| 10 | 287 | CHANGELOG entry spec: *"matten-mlprep and matten-data ship at production-ready candidate, named explicitly…"* | only `matten-data` ships at candidate; `matten-mlprep`'s production-ready label predates this release and needs no re-announcement; `matten-stats`'s resolved exit must be named |
| 11 | 296 | `Cargo.lock: matten, matten-ndarray, matten-mlprep, matten-data -> 1.0.0` | add `matten-stats` |
| 12 | 352 | `cargo package --workspace` (gate list, no count stated) | add an inline note: must package **five** crates |
| 13 | 379 | compatibility table: *"Maturity labels \| None — both companions remain `production-ready candidate`"* | wrong twice over: `matten-mlprep` is promoted, and "both companions" undercounts a five-crate family. Restate per-crate: `matten-mlprep` production-ready (unchanged by this release), `matten-data` candidate (unchanged), `matten-stats` per its resolved RFC-081 exit |
| 14 | 398 | acceptance criterion: *"CHANGELOG names both candidate labels explicitly"* | "both" is wrong; must also require naming `matten-stats`'s status |
| 15 | 413 | Non-goals: *"promoting matten-mlprep or matten-data"* | `matten-mlprep`'s promotion already happened, separately, before this RFC — narrow to *"promoting matten-data"*; do not leave a non-goal that reads as still-open for something already settled |
| 16 | 429 | *"publish in dependency order: matten, then matten-ndarray, matten-mlprep, matten-data"* | add `matten-stats` **conditionally**: it joins this list only if Exit A (promotion) was taken and it remains in the lock-step family; if Exit B, it is excluded and the list stays four long for a different reason than today's staleness |
| 17 | 204 | `matten-data`'s per-crate checklist: *"separate promotion RFC required before v1.0? no — same reasoning as matten-mlprep"* | **A cross-reference orphaned by site 6, not a stale claim on its own** — line 204 was accurate until site 6 rewrites the `matten-mlprep` block it points at. After that edit, "same reasoning as matten-mlprep" refers to reasoning that no longer exists (the mlprep block will say it was promoted and RFC-067's candidate conditions no longer apply to it — not a reason `matten-data` needs no promotion RFC). Inline `matten-data`'s own reason instead: *"no — RFC-059 deferred full-production-ready to a separate later review"* |

### 4.2 Sites checked and confirmed to need NO edit

Record these explicitly — considered, not overlooked:

```text
Line 15    "recorded the RFC-067 family maturity table" -- historical (RFC-075's action)
Line 23    "reproduce the RFC-067 family maturity table in full" -- generic scope description;
           the table itself is fixed at its real location (site 4/5 above)
Line 44    API-surface irreversibility statement -- unrelated to crate count/maturity
Line 113   "zero functional churn ... since 0.31.0" -- stable-API evidence, unaffected by
           the maturity relabel; the 0.31.0 baseline is covered by the general version-drift
           note (SS6), not fixed individually
Line 143   "five std range types" -- false positive from the sweep pattern, unrelated subject
Line 186   matten-ndarray's table row -- correct and unaffected
Line 188   matten-data's table row -- correct; matten-data is still a candidate
Lines 195, 202   matten-data's own per-crate checklist entries -- still correct, unaffected
Lines 233-256    The pre-1.0/0.x sweep's "five phrasing families" spec, including the
           matten-data-specific "mixed maturity" sites (crates/matten-data/README.md:60,
           docs/src/examples/data.md:176) -- these describe matten-data (still candidate,
           unaffected) and belong to the already-scoped 19-site retarget sweep this slice
           does not redo (SS6 below)
Line 286   "no ... change from 0.38.0" -- version-baseline drift, covered by the general
           note (SS6), not fixed individually
Line 355   check-published-dependency-isolation.sh gate mention -- no crate count implied
Line 394   "each of the five phrasing families" -- the pre-1.0 sweep's own category count,
           unrelated to crate count
Line 404   Non-goal "no maturity promotion" -- still accurate for RFC-076 itself; it does not
           promote anyone (RFC-080 already did, separately)
Line 411   Non-goal "publishing to crates.io" -- accurate, unaffected
```

**Do not** rewrite RFC-076's §4.1 (`cargo public-api` not required) or §4.2 (`#[doc(hidden)]` covered).
Those are accepted decisions and remain correct.

## 5. Add the RFC-081 precondition to RFC-076

RFC-076 must not be executable while an `Experimental` crate sits in the family. Add to its §2 or §3:

```text
PRECONDITION (RFC-081 §3): no crate in the lock-step family may be labelled
Experimental at 1.0.0. matten-stats currently is. Before this RFC can be
executed, matten-stats must take Exit A (promotion via its own RFC) or Exit B
(removal from the lock-step family via an RFC amending RFC-030), and this RFC
must record which.
```

This is the point of the slice: without it, RFC-076 remains executable-looking while resting on an
unanswered question.

## 6. Note the version-base drift

RFC-076 was written against `0.38.0` and says so throughout. Two releases have happened. **Do not
retarget every version string** — that would be re-preparing the release, which is out of scope. Fix only
site 1 (the scope line), and add a one-line note near the top:

```text
Written against 0.38.0; the current line is 0.39.0. Version-specific figures
(string counts, file lists) must be re-measured when this RFC is next revised
for execution — they are not re-measured here.
```

An implementer who "helpfully" re-measures RFC-076's 33-string retarget list is doing the *next* slice's
work, and would produce numbers stale again by the time it runs.

## 7. Tracking

```text
rfcs/README.md   add RFC-081 to the proposed table; on acceptance move to done.
                 Update the v1.0-readiness theme row: RFC-076 corrected and
                 blocked on RFC-081 §3, not merely deferred.
ROADMAP.md       Status sentence + history row recording the decision and the
                 refresh. The history row must state the RULE (no Experimental
                 crate in a 1.0 family) — a future reader should find the policy
                 without opening RFC-081.
```

## 8. Verification

```bash
bash scripts/check-release-docs.sh     # includes ROADMAP header/history parity
git diff --check
git diff --name-only                   # expect ONLY .md files
git status --porcelain                 # no Cargo.toml, Cargo.lock, .rs, scripts/
grep -rn "Experimental" README.md crates/*/README.md docs/src/reference/compatibility.md
# matten-stats's label must be UNCHANGED — this slice relabels nothing

# re-run §4's four sweeps post-edit; every remaining hit must match SS4.2's shape
# (historical / generic / matten-data's own unaffected sites), not a new miss
grep -nE "four|five|all .* crates|per-crate" rfcs/proposed/076-v1-release-preparation.md
grep -nE "candidate|Experimental|production-ready|maturity" rfcs/proposed/076-v1-release-preparation.md
grep -n "matten-stats" rfcs/proposed/076-v1-release-preparation.md
grep -nE "package --workspace|publish order|publish in dependency" rfcs/proposed/076-v1-release-preparation.md
```

No cargo gates are required: nothing compiles differently. State that explicitly in the review request
rather than silently omitting them.

## 9. What the review request must report

```text
[ ] the seventeen RFC-076 edits from §4.1, with before/after for each
[ ] the §4.2 leave-alone list, confirming each was checked and why it needs no edit
[ ] confirmation line 193-198's matten-mlprep block was rewritten as reasoning, not word-swapped
[ ] the added RFC-081 precondition, quoted
[ ] the four sweep commands from §4 re-run post-edit, with remaining hits accounted for
[ ] confirmation matten-stats's Experimental label is unchanged everywhere
[ ] confirmation RFC-076 §4.1 and §4.2 were NOT touched
[ ] confirmation RFC-076 stayed in proposed/ — not accepted, executed, or closed
[ ] ROADMAP history row stating the rule itself
[ ] git diff --name-only showing documentation only
```

## 10. Known pitfalls

1. **Promoting `matten-stats` to satisfy the rule.** The rule creates an obligation, not a promotion.
   Which exit it takes is a separate decision on the crate's own evidence (RFC-081 §10).
2. **Re-measuring RFC-076's version-string figures.** Out of scope (§6); they belong to whichever slice
   actually executes it.
3. **Editing RFC-076 §4.1 or §4.2.** Accepted and still correct.
4. **Moving RFC-076 to `done/`.** It is being corrected, not completed.
5. **Treating this as v1.0 work.** It adds a precondition to a v1.0 release; it does not advance one.
6. **Touching RFC-067.** Its candidate rule is deliberately untouched (RFC-081 §3.1).
7. **Changing any maturity label.** Nothing is promoted or demoted by this slice.
8. **Word-swapping a reasoning block instead of rewriting it.** Site 6 (lines 193-198) is not a
   label to substitute — it is an argument for admitting `matten-mlprep` as a candidate, an argument
   RFC-080 already mooted. Swapping "candidate" for "production-ready" inside that reasoning would
   produce a paragraph arguing for something that already happened, which is worse than leaving it
   wrong: it looks deliberate.
9. **Trusting this list without re-sweeping.** It has been short three times running (RFC-079's
   retarget, RFC-080's label sites, this handoff's first draft). Re-run §4's four sweeps after
   editing and confirm every remaining hit is accounted for in §4.2's shape, not just absent.
10. **Missing a cross-reference orphaned by another edit in the same slice.** Site 17 (line 204)
    was accurate on its own and correctly fell into the leave-alone bucket on a first sweep — it
    only becomes wrong once site 6 rewrites the block it points at ("same reasoning as
    matten-mlprep"). A sweep for stale *claims* cannot find this class of defect, because the line
    is not stale until another edit in the same commit makes it so. When a slice rewrites a block,
    check what else in the document cross-references it, not just what the sweep patterns match.

## 11. Review stop

Acceptance makes this a commit point. It authorizes no release, version change, maturity change, or
`matten-stats` exit decision.
