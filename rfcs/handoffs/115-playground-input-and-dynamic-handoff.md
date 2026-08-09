# Developer Handoff — RFC-115: Playground Input Ergonomics, and a Dynamic Path

**From:** High-capability model. **Date:** 2026-08-09.
**Design authority:** `rfcs/accepted/115-playground-input-and-dynamic.md`
**Base:** `main` @ `b3200e2`, clean tree.
**Sequencing:** Part A first, and it stands alone. Part B may follow in the same commit or a second.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. A pure addition in the handoff you can simply omit may be
> omitted and reported prominently; anything else, ask first.

---

## 1. Task title

**A.** Fix the playground's input parser: accept newlines as separators, and stop silently dropping an
interior blank cell. **B.** Enable `dynamic` and add a `try_numeric` demo.

## 2. Part A is the part a user feels

Every operation on the page shares one parser. Two of its behaviours are wrong for the input the page
itself invites:

```text
"1,2,,4,5,6"        -> [1,2,4,5,6]      the blank VANISHES, then try_new reports
                                        "5 elements into shape [2,3] requiring 6"
                                        — a message about a number nobody typed
"1, 2, 3\n4, 5, 6"  -> Error: "3\n4" is not a number
                                        — a 2×3 grid typed the way a grid looks
```

**Part A alone improves all four existing operations.** Land it even if Part B slips.

## 3. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | `parse_values` at `lib.rs:33-42`, `parse_shape` at `:20-30` — both `split(',')` → `trim` → `filter(!is_empty)` → parse | direct read |
| E2 | The `filter` drops **interior** blanks as well as trailing ones | mirrored the function exactly: `"1,2,,4,5,6"` → 5 values |
| E3 | Only `,` is a separator, so a newline lands inside a token | `"1, 2, 3\n4, 5, 6"` → `"3\n4"` fails to parse |
| E4 | `build_tensor` (`lib.rs:47-51`) calls `Tensor::try_new`, so the length error comes from core and is accurate — about the wrong thing | direct read |
| E5 | Enabling `dynamic` costs **+2,734 bytes on 160,621 (+1.7%)** | built the wasm both ways and compared |
| E6 | The tool is `crate-type = ["cdylib"]` | `tools/matten-playground/Cargo.toml:11-12` — it cannot be linked as a test dependency; test in-crate |
| E7 | Each form is wired by id: `pg-<op>-<field>`, `pg-<op>-run`, `pg-<op>-output` | `docs/theme/playground.js` `val`/`wireButton`/`showResult`; `docs/src/playground.md` defines the ids |
| E8 | On wasm a panic reaches JS as a bare `RuntimeError: unreachable`, **no payload** | recorded in `lib.rs`, verified by the tool's original author |

Re-derive before editing. **Report any discrepancy first, including one that shrinks the task.**

## 4. Part A — what to change, and the one subtlety

```text
1. SEPARATORS: split on newlines as well as commas. A grid pasted as rows must work.
2. INTERIOR BLANK: report it — which position, and that the count no longer matches.
3. TRAILING SEPARATOR: "1,2,3," must STILL work.
```

**Point 3 is the whole difficulty.** The current `filter` is not wrong, it is **too broad**: it exists
so a trailing comma is forgiving, and it swallows interior blanks as a side effect. Distinguish
*trailing* from *interior*; do not delete the forgiveness.

A useful way to think about it: split, then trim, then drop empties **only from the end** — anything
empty that still has a non-empty token after it is an interior blank and must be reported.

**`parse_shape` has the same shape and the same bug.** Fix both, or state why only one needed it.

## 5. Part B — the demo is `try_numeric`, not slicing

```text
Cargo.toml    features = ["dynamic"]   (E5: +1.7%)
lib.rs        a new entry point taking shape + values, building a DYNAMIC tensor,
              showing it, then calling try_numeric()
playground.js a form wired like the existing four (E7)
playground.md the form's markup and help text
```

**Show both halves**: the dynamic tensor as-is (so a blank appears as `None` and text as `Text`),
**then** `try_numeric()`'s outcome — either the numeric tensor or the real error naming the first
offending cell.

**Do not build a slicing demo.** Slicing is easier and teaches less; `try_numeric` is the single gate
in the lifecycle and answers the question a learner with a spreadsheet actually has (RFC §3).

With `dynamic` on, Part A's interior blank becomes `Element::None` **on this form only** — the four
numeric forms still report it as an error, because they build numeric tensors. Say so in the help
text; a reader who sees a blank accepted on one form and rejected on another will otherwise think one
of them is broken.

## 6. Required tests

```text
T1  newline-separated rows parse, for values AND shape
T2  an interior blank is REPORTED, naming its position — not dropped
T3  a trailing separator still works: "1,2,3," and "2,3,"
T4  every currently-valid input produces a BYTE-IDENTICAL result — the four
    existing operations' outputs must not change (RFC §5 R1)
T5  Part B: mixed input shows Element-per-cell, then try_numeric's outcome;
    a text cell produces the real error naming that cell
T6  no panicking core form anywhere — grep, and state the result (E8)
```

T4 is the one that matters most. **The parser may change what it ACCEPTS and what it REPORTS, never
what it COMPUTES.**

## 7. Acceptance criteria

```text
[ ] newlines accepted as separators, values and shape
[ ] interior blank reported with its position; trailing separator still forgiving
[ ] no computed result changes for any currently-valid input — asserted (T4)
[ ] Part B: a try_numeric demo showing the dynamic tensor and then the outcome
[ ] the help text explains why a blank is accepted on one form and not the others
[ ] no panicking core form called anywhere in the tool
[ ] the wasm module builds; the page works; the size delta is stated
[ ] cargo test for the tool; clippy under RUSTFLAGS="-D warnings"; fmt clean
[ ] core matten and every published crate untouched — assert via git diff --stat
[ ] no version bump, tag, or publish
```

## 8. Risks

```text
R1  Removing trailing-separator forgiveness (§4 point 3). A regression for
    everyone, to fix one case.
R2  Changing a computed result (T4). The parser's job is acceptance and
    reporting only.
R3  Building a slicing demo because it is easier (§5).
R4  A panic. On wasm it traps the page with no message (E8).
R5  Fixing parse_values and forgetting parse_shape (§4).
R6  Scope creep into RFC-095's output format, or into RFC-116's toolchain work.
```

## 9. Required evidence

For T2 and T3, quote the exact input and the exact output. For T4, state how you established
byte-identity rather than asserting it. For the wasm size, give both numbers.

## 10. Required review-request format

Write to:
`.git-exclude/review-request/RFC-115/matten-rfc115-playground-input-and-dynamic-implementation-review-request-v0.1.md`

Include files changed with line counts, the §3 verification with any discrepancy, §9's evidence,
build/test/clippy output, deviations with reasoning, and anything you want answered at review.
