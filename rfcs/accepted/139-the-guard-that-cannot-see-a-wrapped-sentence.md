# RFC-139: The Guard That Cannot See a Wrapped Sentence

**Status:** **Accepted** 2026-11-03 by the owner. Handoff:
`rfcs/handoffs/139-wrapped-sentence-guard-handoff.md`. Touches zero published packages; no version
bump, tag, or publish.
**Target:** `scripts/check-release-docs.sh` (the stale-prior-family checks, `:370-404`)
**Theme:** A guard whose comment names a file it is structurally unable to read
**Related:** RFC-117 (guards that cannot pass vacuously), RFC-123 (the vacuous crate-table guard),
RFC-131 (the same failure mode as a near-miss), RFC-137 (where this was found), RFC-138 (the same
shape, different mechanism)

---

## 1. Summary

```text
DEFECT  check-release-docs.sh's three stale-version patterns are line-based.
        A sentence that WRAPS cannot match, so the check passes.
IMPACT  0.39.0 shipped to crates.io claiming to document the v0.38 API.
FIX     match against paragraph-joined logical lines instead of raw lines.
        Prototyped and proven (§5) -- it catches the 0.39.0 case the current
        guard passes.
```

## 2. The defect, and it is in the guard's own stated scope

`scripts/check-release-docs.sh:396` says what pattern (c) covers:

```text
# (c) "current [v]X.Y [release] family" prose (e.g. the public-API snapshot header, the
#     introduction page).
```

It cannot match the public-API snapshot header. The sentence wraps:

```text
docs/src/reference/public-api-snapshot.md
  line 3   This page lists every public item in `matten` at the current v0.48 release
  line 4   family. It serves as the baseline for tracking breaking changes toward v1.0.0
```

`grep -rInE 'current v?0\.[0-9]+( release)? family'` is line-based. `family` is on the next line, so
there is no line for the pattern to match — and no match means no error.

**Proven, not reasoned about:**

```text
$ sed -i '3s/v0.48/v0.39/' docs/src/reference/public-api-snapshot.md
$ bash scripts/check-release-docs.sh
All release documentation checks passed.
```

## 3. It has already shipped once

`git log -L3,3` on that file lists a `Prepare` commit for every release but one. The gap is real:

```text
at tag 0.39.0:   Cargo.toml  version = "0.39.0"
                 snapshot    "...at the current v0.38 release"
```

`0.39.0` went to crates.io with its public-API reference page claiming to describe the `v0.38` API.
The guard was green throughout, and the page had been wrong for a whole release cycle.

**Every other release got that line right by hand.** This is the project's third instance of the
same shape — RFC-123's crate-table check that matched zero rows, RFC-138's schedule that GitHub
switches off, and this — and the common thread is a check whose *stated* coverage exceeds its
*actual* coverage while reporting success.

> It is also the second appearance of the **line-based grep** failure mode specifically. The first
> was RFC-131's near-miss, where a spot-check `grep` for a quoted error string returned zero because
> the template spans two source lines, and was nearly filed as a finding. That incident produced the
> maxim *"a grep returning zero is a claim about the grep"* — and then the same mechanism was left
> live inside a guard.

## 4. Which patterns are exposed, measured rather than assumed

| | Pattern | Exposed? | Evidence |
|---|---|---|---|
| (a) | install pins `<crate> = "0.NN"` | **No** | a TOML pin is one line by construction; it cannot wrap |
| (b) | `X.Y.x family` labels | **In principle** | nothing wraps today — 8 matches line-based, 8 joined |
| (c) | `current [v]X.Y [release] family` prose | **Yes, actively** | the snapshot header, §2 |

**(b) is prophylactic and (c) is demonstrated.** Saying so matters: a fix justified by a defect that
exists in only one of three patterns should not be sold as repairing all three. (a) needs no change
at all, and the RFC does not pretend otherwise.

## 5. The fix, prototyped and proven

> **AMENDED 2026-11-03 — this section was incomplete, and the gap was found in implementation
> review, not here. The original text below is kept verbatim.**
>
> "Run the existing patterns against that stream" is true of the **match** stage and false of the
> **exclusion** stage. Each check is a pair: match the pattern, then drop occurrences of the
> *current* family. Widening the unit of text from a line to a paragraph widens the exclusion
> exactly as much as the match, so **a paragraph that mentions the current family anywhere
> immunizes every stale reference inside it** — including the public-API snapshot's own 2,406-character
> header paragraph, the one file this RFC exists to protect. Two cases that the old line-based guard
> caught and the first implementation did not were reproduced end-to-end.
>
> The fix is to exclude by **occurrence**: strip every current-family match out of the text first,
> then apply the (byte-identical) stale pattern to what remains. §5.1's first requirement stands as
> written for the three match patterns; it does **not** license leaving the exclusion stage
> unchanged.
>
> Two consequences followed, both now required:
>
> - Matching moved into `awk`'s dynamic-regex engine, so the script must **assert that the installed
>   awk honours interval expressions** (`.{0,2}` in pattern (b)) before relying on them. An awk that
>   does not compiles the pattern and then matches nothing, for every input — a silently vacuous
>   guard, which is the RFC-117 class arriving through this RFC's own fix. Checked up front with
>   `exit 1`, on the model of the `CURRENT_MINOR` precondition at `:424`.
> - Printing a whole joined paragraph is not an error message. The failure output must name the file,
>   the line, and the offending phrase.
>
> §5.1's last requirement is also wrong as written: the file list does **not** cover "the same
> extensions `grep -r` reached". `grep -rI` reads every text file under `docs/src`; `.md` and `.rs`
> is deliberately **narrower**, excluding the playground's generated wasm-bindgen output. The
> narrowing is correct — awk cannot usefully read a `.wasm` — but it must be stated as a narrowing,
> not as an equivalence. A comment claiming coverage its code does not provide is the defect this
> RFC was written to close.
>
> §5.2's CI requirement is sound but under-specified: **a proof covers a pattern, not a check.**
> Pattern (c) has no interval expression and pattern (b) does, so a CI proof using (c) establishes
> nothing about (b).
>
> Verification note: a before/after match-**set** comparison on a clean tree cannot detect any of the
> above, because neither side holds a stale reference. Mutation testing is required — one case per
> shape, including a stale reference sharing a paragraph with a current one.
>
> **Closed.** All of the above landed in *Exclude stale version references per occurrence, not per
> paragraph, so a paragraph cannot immunize itself (RFC-139)*. The interval question was settled by
> evidence rather than inference: a masked-(b) mutation pushed to a throwaway branch went red in CI
> with `crates/matten-ndarray/README.md:102: 0.47.x\` family`, and the assertion did **not** fire —
> so CI's awk does honour intervals, and the guard says so on every run from now on.


Join each Markdown paragraph into one logical line, tagged with the paragraph's true start line, and
run the existing patterns against that stream.

```awk
function flush() { if (buf != "") { print fname ":" start ":" buf; buf="" } }
FNR==1                 { flush(); fname=FILENAME }
/^[[:space:]]*$/       { flush(); next }
                       { if (buf=="") start=FNR; buf = (buf=="" ? $0 : buf " " $0) }
END                    { flush() }
```

`FNR==1` flushes the *previous* file's pending buffer before `fname` is reassigned, which is the one
subtle part: `END` fires once for the whole run, not per file, so without that line the last
paragraph of every file but the last would be dropped.

**Verified against the real tree:**

```text
1. current tree (v0.48, correct)
     new check: PASS

2. snapshot made stale (v0.39), exactly as 0.39.0 shipped
     CURRENT guard: PASS   <-- blind
     NEW check:     FAIL   docs/src/reference/public-api-snapshot.md:3:This page lists...

3. restored
     clean
```

The error message keeps a usable location — the paragraph's starting line, 3, which is where a
person edits.

### 5.1 Requirements on the implementation

```text
KEEP the three patterns byte-identical. This RFC changes WHAT THEY READ, not
     what they match. Rewriting a regex at the same time would make a
     behaviour change indistinguishable from the fix.
KEEP the dynamic CURRENT_MINOR derivation (:377). Its comment records that a
     hardcoded value was missed at the 0.23.0 bump, "which is exactly how
     stale 0.22 pins shipped".
KEEP (a) line-based, or convert it too -- either is acceptable, but if it is
     converted, say so and show it still matches the same pins.
MIND  USER_DOCS contains a DIRECTORY (docs/src) as well as files. The current
     check relies on `grep -r`; a file list must be expanded, and must cover
     the same extensions `grep -r` reached (.md and .rs).
```

### 5.2 Prove it can fail, in the environment it runs in

Rule 002 §4 and §8. The proof in §5 was run locally; the guard also runs in CI.

```text
[ ] make the snapshot header stale, push to a branch, and observe CI RED
```

A guard fixed only on a developer's machine is the same class of claim this RFC exists to close.

## 6. Why not the alternatives

```text
A  `grep -z` / `pcregrep -M` / `rg -U` for multiline matching.
   Rejected: -z treats the whole file as one record, so line numbers are lost
   from the error message; pcregrep and rg are new CI dependencies, and no
   existing guard needs either.

B  Assert the checked sentences never wrap.
   Rejected: it makes prose formatting load-bearing for a guard. The next
   person to reflow a paragraph breaks the check, and nothing tells them.

C  Rewrite the patterns to tolerate a newline (e.g. `[[:space:]]*`).
   Rejected: grep cannot match across lines however the pattern is written.
   This is the attractive wrong fix and is recorded as R2 below.
```

## 7. Risks

```text
R1  Rewriting the three patterns while changing their input (§5.1). The fix
    must be a change of INPUT only.
R2  Trying to solve it with a cleverer regex (§6 C). grep is line-based; no
    pattern escapes that.
R3  Losing docs/src's recursive coverage, or silently narrowing the file
    extensions reached (§5.1).
R4  Dropping the last paragraph of each file -- the awk `FNR==1` flush (§5).
    A test file with no trailing blank line catches this.
R5  Proving it only locally (§5.2).
R6  Retargeting any 0.4x string while testing. CHANGELOG, ROADMAP and rfcs/
    are history and sit outside USER_DOCS for that reason.
```

## 8. Acceptance criteria

```text
[ ] the three patterns' regexes are unchanged
[ ] (c) matches a wrapped sentence; the snapshot header is covered
[ ] a deliberately stale snapshot header FAILS the guard, with file and line
[ ] the clean tree still passes
[ ] no coverage LOST while gaining some: capture the current checks' matched
    lines before the change and after, and diff the two SETS. Do not verify
    this against a count quoted in an RFC -- today the patterns match 23
    install pins and 8 family labels over USER_DOCS, but this document has
    already had that number wrong once (25, carried over from the 0.48.0
    retarget report, which counted a different scope), and RFC-127 §8 shipped
    "~31" against an actual 47. The before/after sets cannot be wrong that way
[ ] the last paragraph of a file with no trailing newline is still checked
[ ] docs/src is still reached recursively, same extensions
[ ] proven RED in CI on a branch (§5.2), not only locally
[ ] nine guards; no crates/ path touched; zero published packages affected
[ ] no version bump, tag, or publish
```

## 9. What this does not fix

```text
- the other guards. This RFC covers check-release-docs.sh's three stale-version
  patterns only. Whether any other guard greps for something that can wrap is
  a real question and a separate audit; it is not answered here, and claiming
  otherwise would repeat this RFC's own defect one level up.
- RFC-138's scheduled workflow. Same shape, different mechanism.
- the eleven "Unreleased" rows in rfcs/README.md's Done table, which are stale
  statements of a different kind -- some genuinely stale, some permanently
  true -- and need per-row judgement rather than a pattern.
```
