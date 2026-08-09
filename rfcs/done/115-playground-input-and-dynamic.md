# RFC-115: Playground Input Ergonomics, and a Dynamic Path

**Status:** **Implemented** 2026-08-09 in commit *"Fix playground input parsing and add a try_numeric
demo (RFC-115)"* (`301a600`), reviewed and approved with **no corrections**. Ships in no release.
**§3's wasm figure was corrected at review** — the +1.7% it cited measured the bare feature flag, not
the change; the real delta is ~+18%. The decision was unaffected. Handoff:
`rfcs/handoffs/115-playground-input-and-dynamic-handoff.md`.
**Target:** `tools/matten-playground` (workspace-excluded, `publish = false`) + the docs page
**Theme:** Make the page easier to *use*, then let it accept data it currently cannot
**Related:** RFC-093, RFC-095, RFC-102, RFC-111, RFC-113

---

## 1. Summary

Two parts, in this order:

```text
A. INPUT ERGONOMICS. A blank cell is silently dropped; a grid pasted across two
   lines fails. Both affect all four existing operations. No dynamic needed.
B. A DYNAMIC PATH. Enable the `dynamic` feature and add a `try_numeric` demo, so
   a learner can paste genuinely mixed data and see which cell fails and why.
```

**A is worth landing even if B slips.** It improves every operation the page already has.

## 2. Part A — the input path is lossy and it is not the user's fault

`parse_values` (`lib.rs:33-42`) and `parse_shape` (`:20-30`) both do:

```rust
s.split(',').map(str::trim).filter(|p| !p.is_empty()).map(parse)
```

Measured by mirroring that function exactly:

```text
"1,2,3,4,5,6"       -> [1,2,3,4,5,6]                     fine
"1,2,,4,5,6"        -> [1,2,4,5,6]   the blank VANISHES  <- then try_new reports
                                                            "5 elements into shape
                                                            [2,3] requiring 6",
                                                            never mentioning the blank
"1,2,x,4,5,6"       -> Error: "x" is not a number         fine, clear
"1, 2, 3\n4, 5, 6"  -> Error: "3\n4" is not a number      <- a grid pasted as two
                                                             rows, which is the most
                                                             natural way to type one
```

**Two defects, both in the input the page invites.** The page asks for a 2×3 grid; typing it the way a
grid looks fails, and leaving a cell out fails with a message about a different problem.

### 2.1 What to change

```text
1. SPLIT ON NEWLINES AS WELL AS COMMAS. A grid pasted across lines must work.
   Rows-as-lines is how a learner writes a matrix.
2. DO NOT SILENTLY DROP A BLANK CELL. Report it: which position, and that the
   count no longer matches. A dropped value producing a length error about a
   number the user never typed is the worst message on the page.
3. Trailing separators stay forgiving — "1,2,3," should still work. The filter
   exists for that reason; the fix is to distinguish a TRAILING empty from an
   INTERIOR one, not to remove forgiveness.
```

Point 3 is the whole subtlety: the current `filter` is not wrong, it is *too broad*.

## 3. Part B — the dynamic path, and why `try_numeric` rather than slicing

The wasm cost was the stated reason to defer, and it does not hold. Measured:

```text
dynamic OFF   160,621 bytes
dynamic ON    163,355 bytes
delta         +2,734  (+1.7%)
```

**The demo should be `try_numeric`, not slicing.** Slicing is the easier demo; `try_numeric` is the
one that teaches. It is the single gate in the ingest → clean → convert → compute lifecycle
(`data-model.md`), and *"why will my data not convert, and which cell is the problem"* is the question
a learner with a spreadsheet actually has. A dynamic slice mostly shows that `Text` survives being
selected — true, and not a question anyone asked.

```text
input   a grid that MAY contain text or blanks
show    the dynamic tensor as-is (Element per cell)
then    try_numeric() -> either the numeric tensor, or the real error naming the
        first offending cell
```

**This makes Part A's blank-cell fix compose**: with `dynamic`, a blank becomes `Element::None` and
the page can show it rather than reject it.

## 4. Scope

### In scope

```text
tools/matten-playground/src/lib.rs   parse_values/parse_shape (A); a dynamic entry
                                      point (B)
tools/matten-playground/Cargo.toml   features = ["dynamic"] (B only)
docs/theme/playground.js             a form for the new operation (B only)
docs/src/playground.md               help text for both
```

### Out of scope — a diff touching these is a defect

```text
core matten or any published crate
the four existing operations' OUTPUT format — RFC-095's grid contract
the broadcast duplication — still unavoidable, no try_add exists (RFC-113 §2)
mdbook-mermaid or any doc-toolchain change — that is RFC-116
CHANGELOG.md — this tool ships in no release (§7)
```

## 5. Risks

```text
R1  Part A changing the four existing operations' RESULTS. It must change only
    what the parser ACCEPTS and what it REPORTS, never a computed value.
R2  Removing trailing-separator forgiveness (§2.1 point 3). A regression for
    everyone, in service of a fix for one case.
R3  A panic. On wasm a panic traps the page with NO message (RFC-113 §3).
    Nothing added here may call a panicking form.
R4  Part B's demo becoming a slicing demo because it is easier to build (§3).
R5  Scope creep into RFC-095's output format.
```

## 6. Acceptance criteria

```text
[x] a grid pasted across newlines parses, for every operation
[x] an INTERIOR blank cell is reported, naming its position — not dropped
[x] a TRAILING separator still works ("1,2,3," is fine)
[x] no computed result changes for any currently-valid input — asserted
[x] Part B: a try_numeric demo, showing the dynamic tensor and then either the
    numeric result or the real error naming the offending cell
[x] no panicking core form called anywhere in the tool — grep, and state it
[x] the wasm module builds; the page works
[x] cargo test for the tool; clippy under RUSTFLAGS="-D warnings"
[x] core matten and every published crate untouched — assert via git diff --stat
[x] no version bump, tag, or publish
```

## 7. This produces no release

`tools/` is workspace-excluded and `publish = false`. Under RFC-094 a release is triggered by a change
under `crates/`, and this RFC touches none. **Neither this RFC nor RFC-116 will produce a release**,
which is worth stating plainly since both were described as release themes.

## 8. Non-goals

```text
dynamic slicing as the demo (§3)
mutation, is_empty, or any other core surface in the page
schema inference, CSV upload, or file input
RFC-095's grid rendering contract
```
