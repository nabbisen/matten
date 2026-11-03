# RFC-139 Developer Handoff — Make the Stale-Version Checks See Wrapped Sentences

**RFC:** `rfcs/done/139-the-guard-that-cannot-see-a-wrapped-sentence.md`
**Status:** Accepted 2026-11-03 by the owner.
**Target:** `scripts/check-release-docs.sh`, the stale-prior-family checks at `:370-404`
**Authority:** where this document and the RFC disagree, **the RFC wins.**

---

## 1. The one-sentence version

The three stale-version checks are line-based, so a sentence that wraps cannot match and the guard
passes. **Change what they read, not what they match.**

## 2. See the defect yourself first

```bash
sed -i '3s/v0\.48/v0.39/' docs/src/reference/public-api-snapshot.md
bash scripts/check-release-docs.sh          # -> passes. It is blind.
git checkout docs/src/reference/public-api-snapshot.md   # file is clean, so this is safe
```

The sentence the guard's own comment (`:396`) names as a target spans lines 3-4:

```text
line 3   ...at the current v0.48 release
line 4   family. It serves as the baseline...
```

**This already shipped.** `0.39.0` went to crates.io with that page claiming to describe the `v0.38`
API, and the guard was green the whole time.

## 3. The fix

> **AMENDED 2026-11-03 — three requirements were missing from this document and one was wrong.
> Everything below is kept verbatim; read this block first.**
>
> **1. Exclude by occurrence, not by paragraph.** §4's "KEEP the three regexes byte-identical. This
> is a change of INPUT" is correct about the three **match** patterns and silent about the
> **exclusion** stage, which also reads the widened text. A paragraph mentioning the current family
> anywhere therefore immunized every stale reference inside it — proven on the public-API snapshot's
> own 2,406-character header paragraph and on the ndarray README's Compatibility list, both of which
> the old line-based guard caught. Strip current-family occurrences out of the text first, then apply
> the stale pattern to what remains.
>
> **2. Assert the awk honours interval expressions.** Matching now runs through awk's dynamic-regex
> engine. Pattern (b) contains `.{0,2}`; an awk that does not honour intervals compiles it and then
> matches nothing, forever — a silently vacuous guard. Assert it up front with `exit 1`, on the model
> of the `CURRENT_MINOR` precondition at `:424`, and prove the assertion can fail before trusting it.
>
> **3. Make the failure output readable.** Printing a matched paragraph prints up to thousands of
> characters. Report `file:line: <offending phrase>`.
>
> **Wrong as written:** §4's "must reach the same files `grep -r` reached -- that means .md AND .rs".
> `grep -rI` reads every text file under `docs/src`; `.md` and `.rs` is deliberately **narrower**,
> excluding the playground's generated wasm-bindgen output. The narrowing is right; stating it as an
> equivalence is not, and a comment claiming coverage its code lacks is this RFC's own defect class.
>
> **§5's set comparison is necessary but not sufficient.** It compares two implementations on a tree
> with no stale reference, so it cannot see a lost detection. Add mutation testing: plain, wrapped,
> **masked** (stale sharing a paragraph with current), last-paragraph-no-trailing-newline, and clean.
>
> **§6's CI proof covers a pattern, not a check.** (c) has no interval expression; (b) does. Prove
> (b) in CI separately.
>
> **Ruled 2026-11-03, after review:** `report_stale` reporting only the first offender per paragraph
> stays as-is. All ten current subjects sit in ten distinct paragraphs, so even at a bump — when every
> one goes stale at once — it costs zero extra passes, and a `while`/`match` loop would risk a hang in
> a release gate. Record the cost in a comment. If revisited, use `gsub`, which cannot spin.
>
> **All of the above is done**, in *Exclude stale version references per occurrence, not per
> paragraph, so a paragraph cannot immunize itself (RFC-139)*. Three throwaway-branch CI proofs back
> it: the wrapped shape, the masked-(c) shape, and the masked-(b) shape — the last of which also
> proved CI's awk honours intervals, since the assertion did not fire. This document is now a record,
> not a task.


Build a stream of paragraph-joined logical lines, each tagged with the paragraph's true starting line
number, and run the **existing patterns, unchanged** against it.

```awk
function flush() { if (buf != "") { print fname ":" start ":" buf; buf="" } }
FNR==1                 { flush(); fname=FILENAME }
/^[[:space:]]*$/       { flush(); next }
                       { if (buf=="") start=FNR; buf = (buf=="" ? $0 : buf " " $0) }
END                    { flush() }
```

**The `FNR==1` line is the subtle part.** `END` fires once for the whole run, not per file, so
without flushing on each new file's first line — using the *stored* `fname`, before reassigning
it — the final paragraph of every file except the last is silently dropped. A dropped paragraph is a
new blind spot, which would make this fix an instance of the defect it is closing.

This was prototyped and verified before the RFC was written:

```text
1. clean tree (v0.48)                       new check: PASS
2. snapshot stale (v0.39)                   CURRENT guard: PASS  <- blind
                                            NEW check:     FAIL
                                            docs/src/reference/public-api-snapshot.md:3:...
3. restored                                 clean
```

## 4. Hard requirements

```text
KEEP the three regexes byte-identical. This is a change of INPUT. If you
     rewrite a pattern at the same time, a behaviour change becomes
     indistinguishable from the fix, and neither can be reviewed.
KEEP the CURRENT_MINOR derivation at :377 exactly as it is. Its comment
     records that a hardcoded minor was missed at the 0.23.0 bump, "which is
     exactly how stale 0.22 pins shipped".
MIND USER_DOCS mixes files and a DIRECTORY (docs/src). The current checks rely
     on `grep -r` to walk it. A file list must be expanded explicitly, and
     must reach the same files `grep -r` reached -- that means .md AND .rs
     (the five crates' lib.rs files are in USER_DOCS for a reason).
```

Pattern (a), the install pins, is a TOML line by construction and cannot wrap. Converting it is
**optional**: either leave it line-based or convert it with the others, but say which you did.
Pattern (b) does not wrap anywhere today — converting it is prophylactic, and that is fine; just do
not claim it fixed an active defect.

## 5. Do not verify coverage against a count

Capture the matched lines **before** your change and **after**, and diff the two sets. The "after"
set must be a superset: nothing previously caught may be lost.

**Do not check this against a number quoted in an RFC.** RFC-139 itself had this wrong once — it
said 25 install pins, carried over from the `0.48.0` retarget report, which counted a different
scope; the guard's own pattern over `USER_DOCS` matches 23. RFC-127 §8 shipped "~31" against an
actual 47. A before/after set comparison cannot be wrong in that way; a count can.

## 6. Prove it fails in the environment it runs in

Rule 002 §4 and §8. The proof in §3 was local, and this guard also runs in CI.

```text
[ ] make the snapshot header stale on a throwaway branch, push it, and observe
    the CI run go RED
[ ] delete the branch afterwards
```

A guard proven only on a developer's machine is the same shape of claim this RFC exists to close.

## 7. Verify

```text
[ ] the three regexes are unchanged
[ ] a stale snapshot header FAILS, naming the file and a usable line number
[ ] the clean tree still passes
[ ] before/after matched-line SETS compared; nothing lost (§5)
[ ] the last paragraph of a file with NO trailing blank line is still checked
    -- construct such a file and confirm (§3's FNR==1 hazard)
[ ] docs/src still walked recursively, .md and .rs both reached
[ ] proven RED in CI on a branch, not only locally (§6)
[ ] no crates/ path touched; zero published packages affected
[ ] no version bump, tag, or publish
```

## 8. Out of scope

```text
other guards        whether any OTHER guard greps for something that can wrap
                    is a real question and a separate audit. Do not widen this
                    task into it; claiming this RFC fixed "the guards" would
                    repeat its own defect one level up.
the regexes         §4
RFC-138             the scheduled workflow. Same shape, unrelated fix.
the 11 "Unreleased" rows in rfcs/README.md -- stale statements of a different
                    kind, needing per-row judgement, not a pattern.
```

## 9. One commit

```text
Match the stale-version checks against paragraph-joined lines so a wrapped sentence cannot hide (RFC-139)
```

## 10. Review request

Write `.git-exclude/review-request/RFC-139/`. Include the before/after matched-line set comparison,
the local failure proof, the **CI** failure proof (run URL or id), and your trailing-paragraph test.
If you found that converting pattern (a) changed what it matches, say so — that would be a finding
about this document, and the last several rounds have each produced one.
