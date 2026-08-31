# Developer Handoff — RFC-131: Documentation Corrections, Batched

**From:** High-capability model. **Date:** 2026-09-01.
**Design authority:** `rfcs/proposed/131-documentation-corrections-batch.md`
**Base:** **after RFC-127 ships** — two of these findings are resolved by it. See §2.

> **PENDING ACCEPTANCE.** RFC-131 is in `proposed/`. **Do not start** until the owner accepts it.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. A pure addition in the handoff you can simply omit may be
> omitted and reported prominently; anything else, ask first.

---

## 1. Task title

Correct the external audit's documentation findings, **split by what a release can carry**.

```text
BOOK (docs/src/**)    reaches no package -> no release -> lands immediately
RUSTDOC (crates/**)   packaged -> patch content (RFC-120) -> rides RFC-127's patch
```

**Derive that split yourself** with `cargo package --list`. It decides the whole shape of the task.

## 2. Two of these are verification, not editing

```text
D-6  the crate-root promise that the Result zone "never panics on ordinary
     invalid input" is FALSE today and TRUE once RFC-127 lands.
     -> VERIFY after RFC-127. DO NOT EDIT THE SENTENCE.
        The prose states the intent correctly; the CODE was what was wrong.

D-8  slice.rs:332-333's false comment is deleted by RFC-127 Change C.
     -> VERIFY it is gone. If RFC-127 missed it, REPORT that — do not delete it
        here, because that would hide an incomplete fix.
```

## 3. Change A — the quick-start, and it is the one that matters most

**This is the first thing a new user copies.** Today its only install line is the lean profile.

```text
NOW    `matten = { version = "...", default-features = false }`
       -> no serde, no json, no csv
       -> the newcomer copies it, tries Tensor::from_csv, gets
          "no method named from_csv", and nothing on the page mentions features
```

```text
DO     lead with the DEFAULT install, plain, reaching a printed result
DO     show the lean profile LAST, labelled as the opt-in it is
DO     actually reach output — the page currently does not
DO NOT turn it into a tutorial. The tutorial exists. This is a quick start.
```

**Any install line you write is a live version pin** and will be retargeted by every future release
RFC. Use the same family form the other pages use — do not invent a new one, and do not hard-code a
patch version where the other pages use a family.

## 4. Change B — the troubleshooting page

Symptom → cause → fix, seeded from the messages users actually see.

```text
"no method named from_csv"       -> the feature is off (ties to Change A)
"matten shape error ..."          -> shape/data mismatch, how to read it
"matten broadcast error ..."      -> the right-alignment rule
"matten unsupported error ..."    -> a dynamic tensor hit a numeric API;
                                     call try_numeric() first
allocation errors                 -> the element budget and MattenLimits
```

```text
COPY EVERY ERROR STRING VERBATIM FROM THE SOURCE, and verify by grepping the
code for each one you write.

A troubleshooting page whose strings do not match what the user sees is worse
than none: they search for their exact message and find nothing.
```

Add it to `SUMMARY.md` so it is reachable.

## 5. Change C — the rustdoc corrections

```text
D-5   convert.rs:23-25   documents a panic that does not happen
D-7   slice.rs:386-387   documents a guarantee the code does not provide
D-12  two contradictory comments on one constant
D-13  shape.rs:18-19     describes the opposite of the code
D-14  document that PartialEq is non-reflexive for NaN
```

**Rewrite the claim; do not annotate around it.** RFC-119's review had to correct that twice.

For **D-12**, decide which of the two comments is true, delete the other, and **say in the review
request which you kept and why.** Do not keep both with a note reconciling them.

### 5.1 D-14 is documentation only — do NOT change the code

`a == a` being `false` when `a` contains `NaN` is **correct IEEE-754**, and matches `ndarray` and
NumPy. The only defect is that it is undocumented.

```text
"Fixing" this would break IEEE semantics and diverge from the whole ecosystem.
Document it. Do not touch PartialEq.
```

## 6. Sequencing and the release question

```text
RFC-127 ships       -> makes D-6 true, removes D-8
Change A + B         book only, no release, land immediately
Change C             rustdoc; rides RFC-127's patch if it has not shipped yet,
                     otherwise the next one
```

**Change C must not trigger a second patch of its own.** RFC-094 §4.1's "no batching" means a fix must
not *wait* — it does not forbid two fixes in one patch. If `0.46.2` is still being prepared, fold
Change C into it and say so.

## 7. Out of scope

```text
any behavioural code change              statements only
"fixing" non-reflexive PartialEq         §5.1
editing D-6's crate-root sentence        §2
deleting D-8's comment yourself          §2 — report if RFC-127 missed it
the tutorial arc, linking rfcs/README from the book, migration expansion
                                         medium-term, not here
D-15 the "every platform" claim          needs an OWNER DECISION (add CI
                                         platforms, or soften the claim), not a
                                         documentation edit
CHANGELOG, version, pins                 the release RFC owns them
```

## 8. Risks

```text
R1  Editing D-6's sentence instead of verifying it (§2).
R2  "Fixing" PartialEq into incorrectness (§5.1).
R3  Letting Change C trigger a second patch (§6).
R4  Troubleshooting strings that do not match the code (§4).
R5  Turning the quick-start into a tutorial (§3).
R6  Annotating around a false claim rather than rewriting it (§5).
R7  Hard-coding a patch version in a new install line (§3).
```

## 9. Acceptance criteria

```text
[ ] quick-start leads with the DEFAULT install and reaches a printed result;
    lean profile last and labelled
[ ] troubleshooting page exists, in SUMMARY.md, every error string verified by
    grep against the source
[ ] D-5, D-7, D-12, D-13, D-14 corrected; for D-12, which comment was kept and why
[ ] D-6 verified true after RFC-127, sentence UNEDITED
[ ] D-8 verified deleted by RFC-127; if not, reported
[ ] no behavioural code change — asserted by diff
[ ] the book half and rustdoc half identified separately, cargo package --list
    as the evidence
[ ] nine guards; check-doc-code.sh under RUSTFLAGS="-D warnings"; mdbook build clean
[ ] no version bump, tag, or publish
```

## 10. Required review-request format

Write to:
`.git-exclude/review-request/RFC-131/matten-rfc131-documentation-corrections-batch-implementation-review-request-v0.1.md`

Include the packaging split with its evidence, the D-12 decision and reasoning, the D-6/D-8
verification results, the grep proving the troubleshooting strings match the source, guard and mdbook
output, and deviations with reasoning.
