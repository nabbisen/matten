# Developer Handoff — RFC-103: `0.44.0` Dynamic Slicing Release

**From:** High-capability model. **Date:** 2026-08-08.
**Design authority:** `rfcs/accepted/103-0440-dynamic-slicing-release.md`
**Base:** `main` @ `32b48c8`, clean tree.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. If the disagreement is a pure addition in the handoff that
> you can simply omit, omitting it and reporting prominently is acceptable; anything else, ask first.
> (This rule exists because the RFC-102 handoff lacked it and required a `CHANGELOG.md` entry its RFC
> never listed. You were right to omit it and to say so.)

---

## 1. Task title

Prepare the `0.44.0` release: lock-step version bump, 38 live pin retargets, one CHANGELOG entry, and
two content rewrites. **No tag, no publish.**

## 2. What inverts from the last release you did

You have `0.43.0`/RFC-101 to pattern-match against, and **three things flip**:

```text
0.43.0 was Added-only        ->  0.44.0 is CHANGED-ONLY. No Added section, and no
                                 empty Added heading. RFC-101 forbade the mirror image.
0.43.0 changed no behaviour  ->  0.44.0's entire content IS a behaviour change. The
                                 phrase "No existing behavior changed" must not survive
                                 anywhere describing this release.
0.43.0's snapshot needed a   ->  0.44.0's needs a SENTENCE and NO new row. RFC-102 adds
CONTENT update                   no public item; signatures are identical.
```

The trap is muscle memory from four days ago. Every instinct from `0.43.0` is inverted here.

## 3. Evidence for every factual claim

| # | Claim | Established by |
|---|---|---|
| E1 | Family is at `0.43.0`, lock-step, four crates inherit | `Cargo.toml:42`; `crates/*/Cargo.toml:7` = `version.workspace = true` |
| E2 | `0\.43\b` matches **59 lines** in tracked `md/toml/rs/yml` | `git ls-files` → filter ext → exclude `Cargo.lock` by exact path → `grep -c` per file |
| E3 | **21** are historical, in 5 record files | `rfcs/done/101-…` 11, `rfcs/handoffs/101-…` 4, `ROADMAP.md` 3, `CHANGELOG.md` 2, `rfcs/README.md` 1 |
| E4 | **38** are live pins across 17 files | E2 − E3; every one inspected, none a narration |
| E5 | `introduction.md:19-25` claims *"No existing behavior changed"* | `sed -n '15,28p' docs/src/introduction.md` |
| E6 | `public-api-snapshot.md` records the RFC-088 precedent to follow | `sed -n '1,14p' docs/src/reference/public-api-snapshot.md` |
| E7 | `0.42.0` had `Added`+`Changed`+`Version`; `0.43.0` had `Added`+`Version` | `awk '/^## \[0\.4[23]\.0\]/{p=1} /^## \[0\.41/{p=0} p && /^#{2,3} /' CHANGELOG.md` |
| E8 | Only `crates/matten/src/lib.rs:67` carries a pin in `.rs` | E4's file list |

**Re-derive E2–E4 yourself before editing anything.** My first measurement was wrong twice — see
RFC-103 §5.0 — and both errors under-counted. If your number differs from 59/21/38, your method or
mine is broken and that is worth more than the retarget.

## 4. The 21 that must not move

This is the check RFC-101's §5 table failed, in the same shape: a grep sees a token, not a tense.

```text
rfcs/done/101-0430-core-surface-release.md      11   RFC-101 IS the 0.43.0 release
rfcs/handoffs/101-0430-…-handoff.md              4   its handoff
ROADMAP.md                                       3   history rows 3.87.0/3.89.0/3.90.0
CHANGELOG.md                                     2   the [0.43.0] entry
rfcs/README.md                                   1   the RFC-101 Done row
```

Retargeting any of these asserts that a past release happened during *this* one. **Assert they are
unchanged** in your review request — `git diff --stat` naming any of these five files is a defect.

Note `docs/src/contributing/release-checklist.md:15` (*"for example `0.43.0`"*) **is live** — it
illustrates the current tag and RFC-101 retargeted it for exactly that reason. It is not in the list
above.

## 5. Required implementation

```text
1. Cargo.toml:42 -> 0.44.0. Build so Cargo.lock regenerates. Commit the lock.
2. Retarget the 38 live pins. Both forms occur and both must move:
     "0.43.0"  exact pins
     "0.43.x"  family references (README table, crate README status lines)
3. crates/matten/src/lib.rs:67 — the ONLY .rs edit permitted, an install-pin
   doc comment. Any other .rs change is a defect.
4. CHANGELOG [0.44.0]: Changed + Version only (§6).
5. introduction.md content rewrite (§7).
6. public-api-snapshot.md: one sentence, no row (§8).
```

## 6. CHANGELOG `[0.44.0]`

`Changed` and `Version` only. **No `Added`. No empty `Added` heading.**

The `Changed` entry must say what actually changed: `slice()` and `slice_str()` accepted only numeric
tensors and returned `MattenError::Unsupported` for dynamic ones; they now accept dynamic tensors and
return a dynamic tensor.

**Four claims would be publishable falsehoods** (RFC-103 §6.1):

```text
- calling it an ADDITION. Nothing was added; an error was removed from existing methods.
- mentioning shared storage WITHOUT its retention cost. A slice retains its source's
  ENTIRE allocation (RFC-102 §8.1). compatibility.md and slicing.md ship both halves
  together; the CHANGELOG must not ship only the flattering half.
- "no existing behavior changed" — 0.43.0's boilerplate, exactly inverted here.
- any suggestion numeric slicing changed. It did not: 35 pre-existing numeric slice
  tests pass unmodified.
```

## 7. `introduction.md` — rewrite, do not renumber

E5's paragraph describes RFC-099/RFC-100 and ends *"No existing behavior changed."* Under `0.44.0`
that sentence is **false on the documentation's front page**. Rewrite the paragraph for RFC-102.

Accuracy points, all verified during the RFC-102 review and easy to get wrong:

```text
- the slice GRAMMAR is unchanged — same specs, same rank rules, same error messages
- every NUMERIC result is unchanged
- a dynamic slice SHARES storage, and therefore RETAINS the source's whole allocation
- it removes an error; it does not add a method
```

## 8. `public-api-snapshot.md` — a sentence, and no row

Its head paragraph's *"most recently changed in RFC-099 … and RFC-100"* **stays true**; RFC-102 adds
no public item and changes no signature. Do not move that claim.

Add a sentence following the **RFC-088 precedent already on that page** (E6), which records a
behaviour change behind an existing method as *"changed no public item … not a new row here."*
RFC-102 is the same shape. **Adding a row would be a defect** — it would imply a surface change that
did not happen.

## 9. Acceptance criteria

```text
[ ] cargo metadata shows 0.44.0 for all five crates
[ ] 38 live pins retargeted; zero 0.43 remaining outside the 21 record occurrences
[ ] the five record files are UNCHANGED — asserted, via git diff --stat
[ ] no .rs diff except crates/matten/src/lib.rs's doc comment
[ ] CHANGELOG [0.44.0]: Changed + Version only; no empty Added heading
[ ] "No existing behavior changed" appears nowhere describing 0.44.0
[ ] public-api-snapshot.md: sentence added, no new row, claim not moved
[ ] ROADMAP.md and rfcs/** untouched
[ ] cargo test --workspace; both feature profiles build
[ ] eight guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] clippy clean under RUSTFLAGS="-D warnings"; cargo fmt --check
[ ] NO tag, NO publish
```

## 10. Compatibility, security, risks

No dependency, feature, edition, MSRV, or maturity-label change. `#![forbid(unsafe_code)]` untouched.
The released behaviour change is RFC-102's, already reviewed and approved.

```text
R1  Retargeting a record file — silently rewrites history. §4 exists for this.
R2  Added-section muscle memory from 0.43.0, four days old. §2 exists for this.
R3  Shipping the sharing benefit without the retention cost — the exact omission
    RFC-102's review required correcting in the docs. Do not reintroduce it here.
R4  A stray .rs edit. Only lib.rs:67, and only its doc comment.
```

## 11. Required evidence

For E2–E4, give the command and the numbers you got, not the word "verified". For §4, paste
`git diff --stat` and let it show the five record files absent. For §6 and §7, quote the text you
wrote so the claims can be checked against RFC-102's behaviour rather than against this handoff.

## 12. Required review-request format

Write to:
`.git-exclude/review-request/RFC-103/matten-rfc103-0440-dynamic-slicing-release-implementation-review-request-v0.1.md`

Include files changed with line counts, the §3 verification with any discrepancy, §11's evidence,
guard and test output, deviations with reasoning, and anything you want answered at review.
