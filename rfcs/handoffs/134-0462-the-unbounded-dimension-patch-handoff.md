# Developer Handoff — RFC-134: `0.46.2`, The Unbounded-Dimension Patch

**From:** High-capability model. **Date:** 2026-09-01.
**Design authority:** `rfcs/accepted/134-0462-the-unbounded-dimension-patch.md`
**Base:** `main` @ the RFC-134 acceptance commit, clean tree, family at `0.46.1`.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. A pure addition in the handoff you can simply omit may be
> omitted and reported prominently; anything else, ask first.

---

## 1. Task title

Prepare `0.46.2`: lock-step bump, **21** live pin retargets across **14** files, one CHANGELOG entry.
**No tag, no publish.**

## 2. Why this one is urgent

RFC-127 closed an **uncatchable process abort reachable from a 36-byte JSON document**. It is fixed in
git and **still live on crates.io in every published version**. Until this ships, the repository is
correct and the users are not.

That is the only thing about this task that differs from `0.46.1` — the mechanics below are nearly
identical, and the traps are the same ones.

## 3. Re-derive the counts. They moved once already this month

```text
METHOD (RFC-109's, which has now held three times)
  git ls-files -> keep .md .toml .rs .yml .yaml
    -> exclude Cargo.lock BY EXACT PATH
    -> exclude rfcs/, ROADMAP.md, CHANGELOG.md
    -> grep -o, counting "0\.46\.1" and "0\.46\.x" SEPARATELY

EXPECTED
  21  exact "0.46.1" across 14 files      the retarget set
   8  "0.46.x" family refs                ALREADY CORRECT — do not move
   3  "0.46.0" strings                    deliberately preserved (§5)
```

**The family count is 8, not the 13 you may remember from `0.46.1`.** The owner's crate-table change
replaced five `0.46.x family` strings with badges. If you derive 13, you are counting `rfcs/` or
`ROADMAP.md`.

## 4. The 21 to move

```text
Cargo.toml 1, README.md 6, crates/matten/README.md 1, crates/matten-data/README.md 1,
crates/matten-mlprep/README.md 1, crates/matten-ndarray/README.md 1,
crates/matten-stats/README.md 1, crates/matten/src/lib.rs 1,
docs/src/contributing/architecture.md 1, docs/src/examples/data.md 3,
docs/src/quick-start.md 1, docs/src/reference/boundary.md 1,
docs/src/reference/compatibility.md 1, docs/src/reference/dynamic.md 1
```

`crates/matten/src/lib.rs` is the **only** `.rs` file you may touch, and only its install-pin doc
comment. RFC-127 shipped every other `.rs` change already.

## 5. The three that must NOT move

```text
release-checklist.md:15    a tag-FORMAT example; 0.46.0 is still a real tag
release-checklist.md:276   RFC-118's INCIDENT RECORD — "0.46.0 was tagged and
                           published across four consecutive red CI runs".
                           Retargeting it would claim that of THIS release.
introduction.md:34         a pointer to the [0.46.0] CHANGELOG HEADING, which
                           keeps its name forever
```

Both files therefore drop out of the retarget set entirely — which is why 16 candidate files become
14.

## 6. CHANGELOG `[0.46.2]` — three sections, and one of them is `Security`

```text
### Security     the uncatchable abort from untrusted JSON/CSV
### Fixed        the slice wrong-answer, the corrupt try_matmul, Tensor::new's docs
### Version      0.46.1 -> 0.46.2, lock-step
```

**Do not fold Security into Fixed.** The project has used a `### Security` heading four times before,
and people scan changelogs for that word. This one was a remote DoS.

RFC §6 gives the wording. **Four falsehoods to avoid**, each specific:

```text
1. do NOT say zero-sized dimensions are restricted. RFC-111 STANDS and its tests
   pass unmodified. The bound is PER-DIMENSION.
2. do NOT call the slice fix a behaviour change for ordinary callers. Valid
   indices are identical; only >= 2^63 changes, and only from wrong to Err.
3. do NOT claim general hardening. RFC-132 decides the limit MODEL and has not
   shipped.
4. do NOT imply earlier CHANGELOG entries were wrong.
```

Assert **no removed line** in `CHANGELOG.md` — not a fixed occurrence count. Expect the new entry to
*add* `0.46.1` occurrences in its own Version line; that is correct.

## 7. What must NOT change

```text
public-api-snapshot.md   NO new row. RFC-127 added ZERO public items — verified
                         by grep over its whole diff. This is RFC-103's rule.
                         RFC-129 will be the opposite case; do not confuse them.
introduction.md          no rewrite; no new content to describe
rfcs/**, ROADMAP.md      records
any other .rs            RFC-127 shipped them
```

## 8. Acceptance criteria

```text
[ ] cargo metadata shows 0.46.2 for all five crates
[ ] 21 pins retargeted across 14 files; §5's three exclusions untouched
[ ] all 8 "0.46.x" references UNCHANGED — proven by diff, not by count alone
[ ] Cargo.lock regenerated and committed
[ ] CHANGELOG [0.46.2]: Security + Fixed + Version, none empty, no §6 falsehood
[ ] public-api-snapshot.md and introduction.md UNCHANGED
[ ] CHANGELOG.md has no removed line
[ ] no .rs diff except lib.rs's install-pin doc comment
[ ] nine guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
[ ] cargo test --workspace; both feature profiles
[ ] NO tag, NO publish
```

## 9. Risks

```text
R1  Retargeting the 8 "0.46.x" refs. The recurring trap — every release before
    0.46.1 was a MINOR that moved both forms.
R2  Retargeting §5's exclusions, especially release-checklist.md:276.
R3  Adding a public-api-snapshot row (§7).
R4  Saying zero-sized dimensions are now restricted (§6).
R5  Folding Security into Fixed (§6).
R6  Deriving 13 family refs from memory instead of 8 (§3).
R7  Tagging or publishing. Neither is authorized; RFC-118's confirm-CI-green
    step sits between the push and the tag.
```

## 10. Required evidence

```text
- your derived 21 / 8 / 3, with the method, and any discrepancy FIRST
- git diff proving no "0.46.x" string moved
- git diff proving the three exclusions are untouched
- cargo metadata showing 0.46.2 across all five crates
- confirmation that public-api-snapshot.md and introduction.md have empty diffs
- guard, test and clippy output
```

## 11. Required review-request format

Write to:
`.git-exclude/review-request/RFC-134/matten-rfc134-0462-patch-implementation-review-request-v0.1.md`

Include files changed with line counts, §10's evidence, deviations with reasoning, and anything you
want answered at review.
