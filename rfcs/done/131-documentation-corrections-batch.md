# RFC-131: Documentation Corrections, Batched

**Status:** **Implemented** 2026-09-03 in commit *"Correct the audit's documentation findings, split
by what a release can carry (RFC-131)"* (`3513f67`), reviewed and approved with **no corrections**.
The book half (`quick-start`, `troubleshooting`, `SUMMARY`) reaches zero published packages and is
live on push; the five rustdoc corrections are packaged and ride `0.47.0` with RFC-128/129/132,
introducing no new release decision. Handoff:
`rfcs/handoffs/131-documentation-corrections-batch-handoff.md`.
**Target:** `docs/src/**`, and rustdoc in `crates/matten/src/**`
**Theme:** The audit's nineteen documentation findings, sorted by what a release can carry
**Related:** RFC-119 (the template for this), RFC-120 (which decides the split), RFC-127, RFC-094

---

## 1. Summary

```text
BOOK (docs/src/**)  reaches no package -> NO RELEASE, lands immediately
  D-17  quick-start's only install line is `default-features = false`
  D-19  no troubleshooting page anywhere in a 60-page book

RUSTDOC (crates/**) packaged content -> patch content under RFC-120 -> rides a patch
  D-5   convert.rs states a panic that does not happen
  D-7   slice.rs:386-387 states a guarantee the code does not provide
  D-8   slice.rs:332-333 — handled by RFC-127, verify only
  D-12  two contradictory comments on one constant
  D-13  shape.rs:18-19 describes the opposite of the code
  D-14  non-reflexive PartialEq undocumented
  D-6   the crate-root safety promise — becomes TRUE when RFC-127 lands; VERIFY,
        do not edit
```

**The split is not cosmetic.** It is RFC-120's rule, and it determines which half needs a release.

## 2. Why batched, and why this shape

RFC-119 corrected four false published statements in one slice and that worked. This is the same
technique applied to the external audit's documentation findings — with one change: **RFC-119 mixed
book and rustdoc corrections, which meant the whole thing needed a release.** Splitting by packaging
lets the book half land today.

```text
docs/src/**        cargo package --list -> 0 of 5 packages   -> no release
crates/**/*.rs     packaged                                   -> patch (RFC-120)
```

**Derive that split yourself** with `cargo package --list` rather than trusting this table.

## 3. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | `docs/src/quick-start.md`'s only install snippet is `default-features = false` | direct read — verified at review |
| E2 | That profile has no `serde`, `json` or `csv`, so `from_csv` does not exist on it | `crates/matten/Cargo.toml` `[features]` |
| E3 | No troubleshooting or FAQ page exists | `docs/src/SUMMARY.md` |
| E4 | `docs/src/**` reaches zero published packages | `cargo package --list` per crate |
| E5 | D-6's crate-root promise is false **today**, and RFC-127 makes it true | RFC-127 §2 probes |
| E6 | D-8's false comment is deleted by RFC-127 Change C | RFC-127 §6 |

## 4. Change A — the quick-start (D-17, the one that matters most)

**This is the first thing a new user copies**, and it hands them the lean profile.

```text
NOW    the page's ONLY install line is `default-features = false`
       a newcomer copies it, then `Tensor::from_csv` does not exist,
       and nothing on the page points at features
```

```text
DO     default install first — plain `matten = "0.4x"` — reaching a real result
DO     show the lean profile LAST, labelled as the opt-in it is
DO     reach an actual printed output; the page currently does not
DO NOT turn it into a tutorial. It is a quick start; the tutorial exists.
```

**The version string in any new install line is a live pin** and will be retargeted by every future
release RFC. Use the current family form the other pages use — do not invent a new one.

## 5. Change B — troubleshooting (D-19)

A symptom → cause → fix page, seeded from the error messages users actually hit.

```text
"no method named from_csv"          -> the feature is off (ties to Change A)
"matten shape error ..."            -> shape/data mismatch, and how to read it
"matten broadcast error ..."        -> the right-alignment rule
"matten unsupported error ..."      -> a dynamic tensor reached a numeric API;
                                       call try_numeric() first
allocation errors                   -> the element budget and MattenLimits
```

**Seed it from real messages, copied verbatim from the code.** A troubleshooting page whose messages
do not match what the user sees is worse than none — the user searches for their exact string and
finds nothing.

## 6. Change C — the rustdoc corrections

Each is a false or contradictory statement in packaged content. **Treat them exactly as RFC-119
treated its four**: rewrite the claim, do not annotate around it.

```text
D-5   convert.rs:23-25   documents a panic that does not happen
D-7   slice.rs:386-387   documents a guarantee the code does not provide
D-12  two contradictory comments on one constant — decide which is true, delete
      the other, and say in the review request which you kept and why
D-13  shape.rs:18-19     describes the opposite of the code
D-14  document that PartialEq is non-reflexive for NaN
```

### 6.1 D-14 is documentation, NOT a code change

`a == a` being `false` when `a` contains `NaN` is **correct IEEE-754**, and matches `ndarray` and
NumPy. **Do not "fix" it.** The only defect is that it is undocumented.

### 6.2 D-6 and D-8 are verification tasks, not edits

```text
D-6  the crate-root promise that the Result zone "never panics on ordinary
     invalid input" is FALSE today and TRUE once RFC-127 lands.
     VERIFY it after RFC-127. Do not edit the sentence — it states the intent
     correctly and the code is what was wrong.
D-8  slice.rs:332-333's false comment is deleted by RFC-127 Change C.
     VERIFY it is gone. If RFC-127 missed it, report that.
```

## 7. Sequencing

```text
RFC-127 first — it makes D-6 true and removes D-8
then  Change A + Change B      book only, no release, land immediately
then  Change C                 rustdoc, rides the next patch after RFC-127's
```

**Change C must not trigger its own release.** If `0.46.2` (RFC-127) has not yet shipped when Change C
is ready, fold it into that release rather than preparing a second patch — RFC-094 §4.1's "no
batching" is about not making a fix *wait*, not about forbidding two fixes in one patch.

## 8. Scope

### Out of scope — a diff touching these is a defect

```text
any behavioural code change            this RFC changes statements only
"fixing" non-reflexive PartialEq       §6.1
editing D-6's crate-root sentence      §6.2
RFC-127's fixes                        its own RFC
the tutorial expansion, rfcs/README link from the book, migration pages
                                       the audit's medium-term items; not here
CHANGELOG, version, pins               the release RFC owns them
```

## 9. Risks

```text
R1  Editing D-6's sentence instead of verifying it. The prose is right; the code
    was wrong (§6.2).
R2  "Fixing" PartialEq into incorrectness (§6.1).
R3  Letting Change C trigger a second patch instead of riding RFC-127's (§7).
R4  A troubleshooting page whose error strings do not match the code (§5).
R5  Turning the quick-start into a tutorial (§4).
R6  Annotating around a false rustdoc claim rather than rewriting it — the
    failure RFC-119's review corrected twice.
```

## 10. Acceptance criteria

```text
[ ] quick-start leads with the DEFAULT install and reaches a printed result;
    the lean profile appears last, labelled
[ ] troubleshooting page exists, reachable from SUMMARY.md, every error string
    copied verbatim from the code — verified by grep against the source
[ ] D-5, D-7, D-12, D-13, D-14 corrected; for D-12, which comment was kept and why
[ ] D-6 VERIFIED true after RFC-127, sentence unedited
[ ] D-8 VERIFIED deleted by RFC-127; if not, reported
[ ] no behavioural code change — asserted by diff
[ ] the book half and the rustdoc half identified separately, with
    cargo package --list as the evidence
[ ] nine guards; check-doc-code.sh under RUSTFLAGS="-D warnings"; mdbook build clean
[ ] no version bump, tag, or publish
```

## 11. What this does not fix

```text
- the remaining medium-term documentation items: the tutorial arc, linking
  rfcs/README.md from the book, the migration-guide expansion
- D-15, the "on every platform" reproducibility claim that ubuntu-only CI cannot
  check. That needs a DECISION — add platforms, or soften the claim — and it is
  the owner's, not a documentation edit
- a guard that can read a published claim, which is what would have prevented
  this batch and RFC-119's before it
```
