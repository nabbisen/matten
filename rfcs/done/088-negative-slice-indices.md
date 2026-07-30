# RFC-088: Negative Indices in `slice_str`

**Status:** Implemented — commit *"Add negative slice indices to slice_str (RFC-088)"*; reviewed
and approved 2026-07-30 after one should-fix (the inverted-range error message lost the caller's
written form once bounds could be negative). No public item changed, so `public-api-snapshot.md`
correctly did not move. **Unreleased**: the family stays at `0.40.0`
**Target:** Post-`0.40.0`, on the `0.x` line
**Theme:** Close RFC-008's `0.1.0` deferral of negative indices, for the reader who arrives from Python
**Depends on:** RFC-005, RFC-008, RFC-015, RFC-018
**Related:** RFC-002, RFC-039, RFC-087

---

## 1. Summary

Extend `slice_str`'s grammar so `index`, `start` and `end` accept an optional leading `-`, with
Python's convention: `-1` is the last element along that axis.

```text
t.slice_str("-1")        last element
t.slice_str("0:-1")      everything but the last
t.slice_str("-2:")       last two
t.slice_str("-1,:")      last row of a matrix
```

**`slice_str` only.** The builder is explicitly excluded, for a reason that is not laziness — see §4.

No new public type, no new `MattenError` variant, no dependency or MSRV change.

## 2. Why this, and why now

Second theme against §1.1's planning baseline, on the same criterion as RFC-087 — *what a learner
meets early and often* — and this one is met **earlier and more often than anything else on the
list**. `x[-1]` is among the first things anyone who has written Python reaches for. Today it is a
parse error:

```text
t.slice_str("-1")    ->  matten slice error for "-1": unrecognised slice component "-1"
t.slice_str("0:-1")  ->  matten slice error for "0:-1": expected integer, got "-1"
```

For a library whose stated purpose is education, learning, PoC and prototyping, that is a papercut at
exactly the wrong moment — the first ten minutes, when a reader is deciding whether the library thinks
like they do. The workaround (`len - 1`, computed by hand, per axis) is not hard; it is just constant.

RFC-008 §4 deferred this with *"No negative indices in `0.1.0`"* and §11 listed it under "Rejected in
`0.1.0`". That version is long past, and the deferral came with no criterion to revisit it.

## 3. Grammar change

Only three productions change, each gaining an optional sign:

```text
index      := "-"? digits
start      := "-"? digits
end        := "-"? digits
step       := nonzero_digits        UNCHANGED — positive only
```

Everything else in RFC-008 §11 is untouched: `:` for all, comma-separated axes, whitespace ignored,
ellipsis / newaxis / boolean masks / list indexes still rejected.

## 4. Why the builder is excluded

`SliceBuilder::index` takes `usize`, and `range` takes `R: IntoSliceRange` — a **sealed** trait
implemented only for the five standard range types over `usize`. A negative index cannot be expressed
through any of them.

Two ways to change that, both rejected:

- **Add `isize` range impls.** `impl IntoSliceRange for Range<isize>` would make `range(0..-1)` work —
  and would simultaneously make every *existing* `range(1..3)` call ambiguous, because an unsuffixed
  integer literal could then infer as either `usize` or `isize`. That is a **source-breaking change to
  working downstream code**, which RFC-015's compatibility policy does not permit for a convenience
  feature.
- **Add `index_back(n)` / `range_back(..)` methods.** Not breaking, but it grows the typed API with a
  second way to say the same thing, and the builder is the *programmatic* path where a caller already
  has `len` in hand and can write `len - 1` directly.

The string form is where the ergonomic gap actually bites, because that is the path a learner reaches
for when transcribing `x[-1]`. Fixing it there and leaving the typed API alone is the proportionate
answer, and it is recorded here so a future reader does not mistake the asymmetry for an oversight.

## 5. Semantics

**Resolution.** For a negative value `i` on an axis of size `dim`, the resolved index is `dim + i`.
Resolve first, then apply RFC-008 §12.2's existing bounds validation **unchanged**:

```text
Index(i)   requires resolved  0 <= i < dim
Range      requires resolved  start <= end <= dim
step       must be > 0
```

**Out of range errors — it does not clamp.** `slice_str("-10")` on an axis of size 3 is an error, and
so is `slice_str("-10:")`.

This **diverges from Python**, which errors for a bare index but silently clamps slice bounds
(`a[-10:]` on a 3-element list returns the whole list). The divergence is deliberate, and it follows
RFC-087 §6's boundary rule rather than inventing a new one:

```text
matten already errors on positive out-of-range: "0:100" on size 3 is an error today,
not a clamp. Negative indices must behave the same way, or the same spec string would
be validated by two different rules depending on its sign.

The divergence from Python is VISIBLE — the caller gets an error naming the axis and
size — so it falls on the "diverge where it surfaces and teaches" side of RFC-087 §6,
not the "match the ecosystem because the mistake would be silent" side.
```

**Error messages must show both forms.** A reader who wrote `-10` needs to see what it resolved to:

```text
matten slice error: index -10 (resolves to -7) is out of range for axis 0 with size 3
```

**`-0`** parses as `0`. No special case; it is simply an odd way to write zero.

## 6. What this does not change

```text
the builder                       §4
negative STEP (reversal)          a separate feature; step stays positive-only.
                                  "::-1" remains a parse error, and this RFC does not
                                  make reversal look almost-available
ellipsis, newaxis, boolean masks, list indexes    still rejected (RFC-008 §11)
bounds-validation RULES           unchanged; only the values fed into them are resolved first
any existing valid spec           see §7
```

## 7. Compatibility

**Additive in practice.** Every string that parses today parses identically tomorrow. The strings whose
behaviour changes are exactly those that are **parse errors today** — a leading `-` was never valid —
so no working program can observe a difference.

This is worth stating precisely rather than waving at: the change is not "additive" in the strict sense
that *nothing* behaves differently. Input that previously errored now succeeds. A program that
deliberately relied on `"-1"` being rejected would change behaviour. That program is not one this
project needs to protect.

No public type, signature, error variant, feature, dependency, edition or MSRV change.

## 8. A pre-existing inconsistency this makes easier to hit — flagged, not resolved

Slicing can already produce a zero-sized dimension, which the constructor rejects:

```text
Tensor::new(vec![1.,2.,3.], &[3]).slice_str("0:0")   ->  Ok, shape [0]
Tensor::zeros(&[0])                                  ->  Err, "zero-sized dimensions are
                                                          not supported in the current
                                                          matten shape model"
```

So the shape model is inconsistent about whether zero-sized dimensions exist, and `slice_str` is the
door through which they enter. **This RFC does not introduce that**, but negative indices make it
easier to reach by accident — `"0:-3"` on an axis of size 3 resolves to `"0:0"`, and a learner writing
`"0:-3"` on a short axis will land on it without meaning to.

Resolving it needs its own RFC and a decision about which side gives way: either slicing should reject
empty results, or the shape model should admit zero-sized dimensions. Both are larger than this
change. Recorded here so the connection is on the record when someone picks it up.

## 9. Scope

```text
IN    the slice_str parser: optional leading '-' on index/start/end
      negative resolution before the existing bounds validation
      error messages showing the written form AND the resolved index
      crates/matten/src/... slice tests
      docs/src/reference/slicing.md — grammar table and examples
      the RFC-008 grammar's "Rejected in 0.1.0" list is HISTORY; do not edit it

OUT   the builder (§4); negative step / reversal; ellipsis, newaxis, masks, list indexes;
      the §8 zero-sized-dimension question; any change to bounds-validation rules;
      public-api-snapshot.md — no public item is added or changed (§7)
      version bump, CHANGELOG, tag, publish
```

## 10. Acceptance criteria

```text
[ ] "-1", "-2", "0:-1", "-2:", ":-1", "-3:-1" all correct on a known vector
[ ] negative indices work on EVERY axis of a rank-2 tensor, not just axis 0
[ ] mixed signs in one spec ("0:-1,-1") correct
[ ] "-n" where n == dim resolves to 0 and succeeds; n == dim+1 errors
[ ] out-of-range negatives ERROR rather than clamp, for both index and range forms
[ ] the error message contains both the written form and the resolved index
[ ] "::-1" still a parse error — reversal is not smuggled in
[ ] every spec valid before this change still parses to the identical result
[ ] full gate set; no version bump, CHANGELOG, tag, or publish
```

## 11. Non-goals

```text
negative step / reversed slicing
builder support
resolving §8's zero-sized-dimension inconsistency
NumPy-compatible clamping of out-of-range slice bounds (§5)
```
