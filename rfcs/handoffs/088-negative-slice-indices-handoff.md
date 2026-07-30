# RFC-088 Negative Slice Indices: Implementation Handoff

**Project:** `matten`
**Governing RFC:** `rfcs/proposed/088-negative-slice-indices.md`
**Document kind:** Detailed implementation handoff
**Status:** Proposed; inherits RFC-088's state. Not authorized for implementation until accepted
**Date:** 2026-07-30

---

## 1. Purpose

Let `slice_str` accept a leading `-` on `index`, `start` and `end`, resolving `-1` to the last element
along that axis.

**No public item is added or changed.** This is a parser and validation change behind an existing
signature — so unlike RFC-087, `public-api-snapshot.md` does **not** move. If your diff touches it,
something has gone wrong.

No release, no version bump. Version stays `0.40.0`.

## 2. Where the work is

Everything is in **`crates/matten/src/slice.rs`**:

```text
parse_slice_str(spec)        :312   splits on ',' and dispatches per axis
parse_axis_spec(part, full)  :329   the per-axis parser
  :339  "unrecognised slice component {part:?}"    <- what "-1" hits today
  :350  "expected integer, got {s:?}"              <- what "0:-1" hits today
```

Those two messages are your map: the first is the bare-index path, the second the range path. Both
must learn the sign; nothing else in the file parses integers.

## 3. Design — resolve, then validate

Keep the two concerns separate. **Parse to a signed value, resolve against `dim`, then feed the
existing bounds validation unchanged.**

```text
parse    "-2"  ->  signed -2                (parser's job)
resolve  -2 with dim=5  ->  3               (new, tiny)
validate 3 against 0 <= i < dim             (RFC-008 §12.2, UNCHANGED)
```

**Do not** loosen or duplicate the bounds rules. `Index` still needs `0 <= i < dim`; `Range` still
needs `start <= end <= dim`; `step` still must be `> 0`. Resolution happens before them, not instead
of them.

Note that resolution needs `dim`, which the parser does not have — parsing is per-axis-string, and
axis sizes are known later. Expect to carry the sign through the `SliceSpec` and resolve at validation
time, rather than resolving inside the parser. Choose whichever shape fits the existing code; the
requirement is that the two steps stay distinguishable.

## 4. Semantics to implement

```text
-1 is the last element:      resolved = dim + i   for i < 0

"-1"       on [1,2,3]  ->  3.0        (index; the axis is dropped, as any index is)
"0:-1"     on [1,2,3]  ->  [1.0, 2.0]
"-2:"      on [1,2,3]  ->  [2.0, 3.0]
":-1"      on [1,2,3]  ->  [1.0, 2.0]
"-3:-1"    on [1,2,3]  ->  [1.0, 2.0]
"-3"       on [1,2,3]  ->  1.0        (n == dim resolves to 0, valid)
"-4"       on [1,2,3]  ->  ERROR      (n == dim+1)
"-0"       on [1,2,3]  ->  1.0        (parses as 0; no special case)
```

### 4.1 Out of range is an error, not a clamp

`"-10"` and `"-10:"` on an axis of size 3 both **error**. Python clamps the range form; matten does
not, because matten already errors on positive out-of-range — `"0:100"` on size 3 is an error today —
and one spec string must not be validated by two different rules depending on its sign.

**Error messages must contain both the written form and what it resolved to**, or the reader cannot
tell what happened:

```text
matten slice error: index -10 (resolves to -7) is out of range for axis 0 with size 3
```

## 5. Required tests

```text
[ ] "-1", "-2", "-3" on a 3-element vector: exact values
[ ] "0:-1", ":-1", "-2:", "-3:-1": exact values
[ ] NEGATIVES ON EVERY AXIS of a rank-2 tensor -- "-1,:" and ":,-1" and "-1,-1".
    An implementation that resolves against axis 0's size for every axis passes a
    rank-1 test suite completely. This is the analogue of RFC-087's unequal-length
    meshgrid test: use a tensor whose two dimensions DIFFER, e.g. [3, 2], so a
    wrong-axis resolution produces a wrong value or an error rather than passing
[ ] mixed signs in one spec: "0:-1,-1"
[ ] "-3" (n == dim) succeeds and resolves to 0; "-4" errors
[ ] "-10" and "-10:" both error -- NOT clamped to the whole axis
[ ] the error message contains the written form AND the resolved index
[ ] "::-1" is still a parse error (reversal not smuggled in)
[ ] "-0" behaves as "0"
[ ] REGRESSION: every spec that was valid before still parses to the identical
    result -- ":", "0", "0:2", "1:", ":2", "0:10:2" and the existing suite
```

## 6. The test most likely to be got wrong

**Rank-2 with unequal dimensions.** If resolution uses the wrong axis's size, a square-tensor test
still passes. Use `[3, 2]`:

```text
m = [[1,2],[3,4],[5,6]]     shape [3, 2]

"-1,:"   -> [5.0, 6.0]      last ROW      (axis 0, size 3)
":,-1"   -> [2.0, 4.0, 6.0] last COLUMN   (axis 1, size 2)
"-1,-1"  -> 6.0             scalar
```

If `":,-1"` returns the last row, or errors with "size 3", resolution is reading the wrong axis.

## 7. Documentation

```text
crates/matten/src/... slice_str's doc comment: the new forms, with an example
docs/src/reference/slicing.md   the `slice_str` section (line ~42): grammar and
                                examples; state that out-of-range does NOT clamp,
                                and that the builder does not take negatives (RFC-088 §4)
```

**Do not edit `rfcs/done/008-...md`.** Its "Rejected in `0.1.0`" list is a correct record of what
RFC-008 decided; RFC-088 supersedes it in effect, not by rewriting it.

**Do not touch `public-api-snapshot.md`** (§1).

## 8. Verification

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings
cargo test --all-targets
cargo test --doc --all-features
cargo +1.85.0 build && cargo +1.85.0 test --all-features
bash scripts/check-release-docs.sh
bash scripts/check-core-dependency-boundary.sh
bash scripts/check-published-dependency-isolation.sh
bash scripts/check-benchmark-dependency-sync.sh
bash scripts/check-streaming-scope.sh
bash scripts/check-matten-data-scope.sh
mdbook build docs && rm -rf docs/book
git diff --check
```

Scope confirmation:

```bash
git diff --name-only -- crates/matten-data crates/matten-mlprep crates/matten-ndarray crates/matten-stats
# expect EMPTY
git diff --name-only -- docs/src/reference/public-api-snapshot.md
# expect EMPTY -- no public item changed
git diff --name-only -- rfcs/done
# expect EMPTY
grep -m1 '^version' Cargo.toml     # still 0.40.0
```

## 9. Known pitfalls

1. **Resolving against the wrong axis's size** (§6). The `[3, 2]` test exists for this.
2. **Clamping instead of erroring** (§4.1). Python clamps; matten must not.
3. **Loosening the bounds rules** instead of resolving before them (§3).
4. **Accidentally accepting `"::-1"`.** A sign on `step` must stay a parse error — the grammar change
   is on `index`/`start`/`end` only.
5. **Error messages showing only the resolved value**, leaving the reader unable to connect it to what
   they wrote (§4.1).
6. **Touching `public-api-snapshot.md`** — nothing public changed.
7. **Editing RFC-008's "Rejected" list** — that is history (§7).
8. **Adding builder support.** Out of scope, and RFC-088 §4 explains why adding `isize` range impls
   would be source-breaking for existing callers.

## 10. What the review request must report

```text
[ ] the rank-2 unequal-dimension test with its three specs and exact values (§6)
[ ] the out-of-range tests showing an ERROR, with the message text quoted
[ ] "::-1" still rejected
[ ] the regression evidence: the pre-existing slice test suite passing unchanged
[ ] confirmation public-api-snapshot.md, rfcs/done/ and the companion crates are
    absent from the diff
[ ] full gate set incl. MSRV and mdbook; version still 0.40.0
```

## 11. Review stop

Acceptance makes this a commit point. It authorizes no release, version bump, tag, or publish.
