# RFC-139 Developer Handoff — Make the Stale-Version Checks See Wrapped Sentences

**RFC:** `rfcs/accepted/139-the-guard-that-cannot-see-a-wrapped-sentence.md`
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
