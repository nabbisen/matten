# Developer Handoff — RFC-121: `0.46.1`, The Published-Surface Patch

**From:** High-capability model. **Date:** 2026-08-28.
**Design authority:** `rfcs/accepted/121-0461-published-surface-patch.md`
**Base:** `main` @ `bf4fa1a`, clean tree, family at `0.46.0`.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. A pure addition in the handoff you can simply omit may be
> omitted and reported prominently; anything else, ask first.

---

## 1. Task title

Prepare `0.46.1`: lock-step bump, **21** live pin retargets across **14** files, one CHANGELOG entry.
**No tag, no publish.**

## 2. What makes this task different — read before anything else

**This is a PATCH. You have three release preparations to pattern-match against and all three are
minors.** The instinct they trained is wrong here in one specific, high-consequence way:

```text
minor 0.45.0 -> 0.46.0 :  "0.45.0" -> "0.46.0"   AND   "0.45.x" -> "0.46.x"
patch 0.46.0 -> 0.46.1 :  "0.46.0" -> "0.46.1"   BUT   "0.46.x" STAYS
```

> **`0.46.1` is in the `0.46.x` family.** All **13** `0.46.x` references are *already correct*.
> Touching one is a defect. This is R1, and it is the single most likely way to get this release
> wrong — not because it is subtle, but because the last three releases required exactly the
> opposite.

It is also the project's **first patch release since `0.28.5` (2026-06-28)** and the first ever under
RFC-094 — whose §3 recorded that patches had *"quietly stopped existing."*

## 3. Evidence — re-derive before editing

**Do not take 24/21/13 from this handoff.** Derive them. RFC-119's E13 was wrong in my document and
the implementer caught it; assume the same is possible here. If your figures differ, that discrepancy
is worth more than the retarget — report it first.

```text
METHOD (RFC-109's, which has held up twice)
  git ls-files
    -> keep .md .toml .rs .yml .yaml
    -> exclude Cargo.lock BY EXACT PATH (not by pattern — an unanchored
       filter drops any LINE mentioning it, which is how this went wrong once)
    -> exclude rfcs/, ROADMAP.md, CHANGELOG.md   (records)
    -> grep -o per file, counting "0\.46\.0" and "0\.46\.x" SEPARATELY

EXPECTED
  24  exact "0.46.0" in live files
   3  of those must NOT move (§5)
  21  to retarget, across 14 files
  13  "0.46.x" family references, all already correct
```

## 4. The 21 to move

```text
Cargo.toml                                  1   the workspace version itself
README.md                                   6
crates/matten/README.md                     1
crates/matten-data/README.md                1
crates/matten-mlprep/README.md              1
crates/matten-ndarray/README.md             1
crates/matten-stats/README.md               1
crates/matten/src/lib.rs                    1   the install-pin doc comment
docs/src/contributing/architecture.md       1
docs/src/examples/data.md                   3
docs/src/quick-start.md                     1
docs/src/reference/boundary.md              1
docs/src/reference/compatibility.md         1
docs/src/reference/dynamic.md               1
```

`crates/matten/src/lib.rs` is the **only** `.rs` file this task may touch, and only its install-pin
doc comment. RFC-119 shipped every other `.rs` change already.

## 5. The 3 that must NOT move — why a blind sweep is wrong

```text
docs/src/contributing/release-checklist.md:276
    "`0.46.0` was tagged and published across four consecutive red CI runs"
    RFC-118's INCIDENT RECORD. Retargeting it asserts 0.46.1 shipped on red
    CI. It did not. This is R2 and it rewrites history.

docs/src/introduction.md:34
    "see the `[0.46.0]` CHANGELOG entry"
    A reference to a CHANGELOG HEADING, which keeps its name forever.

docs/src/contributing/release-checklist.md:15
    "Release tags use bare SemVer ... for example `0.46.0`."
    A tag-FORMAT example; 0.46.0 is still a real tag. Leave it.
    This one is a judgement, not a rule — if you disagree, say so.
```

## 6. CHANGELOG `[0.46.1]`

Two sections, both non-empty: **`### Fixed`** and **`### Version`**. None of the last four releases
used `Fixed`; it is right here and `Added`/`Changed` are not — nothing was added and no behaviour
changed.

RFC §6 gives the four bullets. **Four claims would be publishable falsehoods:**

```text
- calling any of this a BEHAVIOUR change. The example's behaviour changed;
  no API's did.
- implying matmul or dot gained anything. They lost a bug in a demo.
- saying dynamic reshape or arithmetic became available. Only SLICING was
  mis-described; both of those remain guarded.
- attributing z-score to matten-stats. It lives in matten-mlprep
  (`standardize_columns`). Getting this wrong here would be the third time
  in this sequence — it was RFC-119's required review correction.
```

## 7. What must NOT change — two of these invert RFC-109

```text
public-api-snapshot.md   NO new row, no changed claim. RFC-119 added, removed
                         and changed ZERO public items. RFC-109 required four
                         rows and called omitting them the defect; HERE,
                         ADDING one is the defect. Follow RFC-103, not RFC-109.

introduction.md          NO content rewrite. Every minor rewrote it because
                         it had new content to describe. This has none.
                         Line 19's "the current 0.46 release family" stays true.

rfcs/**, ROADMAP.md      Records. Assert unchanged.

CHANGELOG.md             Assert NO REMOVED LINE — not a fixed occurrence count.
                         Expect [0.46.1] to ADD "0.46.0" occurrences in its own
                         Version line; that is correct. A fixed count is the
                         wrong invariant (RFC-103's review corrected me on this).

any other .rs, any example, .github/**   RFC-119 shipped them.
```

## 8. Acceptance criteria

```text
[ ] cargo metadata shows 0.46.1 for all five crates
[ ] 21 pins retargeted across 14 files; the 3 exclusions untouched
[ ] all 13 "0.46.x" references UNCHANGED — asserted explicitly, by diff
[ ] Cargo.lock regenerated and committed
[ ] CHANGELOG [0.46.1]: Fixed + Version, neither empty, no §6 falsehood
[ ] public-api-snapshot.md and introduction.md UNCHANGED
[ ] rfcs/**, ROADMAP.md unchanged; CHANGELOG.md has no removed line
[ ] no .rs diff except crates/matten/src/lib.rs's doc comment
[ ] nine guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
    — the CI form. Do NOT scope it to -p matten.
[ ] cargo test --workspace; both feature profiles build
[ ] NO tag, NO publish
```

## 9. Risks

```text
R1  Retargeting the 13 "0.46.x" family references (§2). The likeliest defect.
R2  Retargeting release-checklist.md:276, rewriting RFC-118's record (§5).
R3  Adding a public-api-snapshot row — RFC-109's instruction carried forward
    where RFC-103's applies (§7).
R4  Describing anything here as a behaviour change (§6).
R5  Asserting a fixed CHANGELOG occurrence count (§7).
R6  Touching a .rs file other than lib.rs's doc comment.
R7  Tagging or publishing. Neither is authorized — each is a separate owner
    decision at the time, and RFC-118's confirm-CI-green step sits between
    the push and the tag.
```

## 10. Required evidence

```text
- your derived 24 / 21 / 13, with the method, and any discrepancy FIRST
- `git diff` proof that no "0.46.x" string moved
- `git diff` proof that the 3 exclusions are untouched
- cargo metadata output showing 0.46.1 across all five crates
- confirmation that public-api-snapshot.md and introduction.md have empty diffs
- guard, test and clippy output
```

## 11. Required review-request format

Write to:
`.git-exclude/review-request/RFC-121/matten-rfc121-0461-published-surface-patch-implementation-review-request-v0.1.md`

Include files changed with line counts, the §3 re-derivation with any discrepancy, §10's evidence,
deviations with reasoning, and anything you want answered at review.
