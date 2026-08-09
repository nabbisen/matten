# RFC-110: Empty-Axis Reduction Semantics — Stage 2

**Status:** **Implemented** 2026-08-09 in commit *"Error on empty reduced axis in
mean/min/max/var/std_axis (RFC-110)"* (`2b90592`), reviewed and approved with **no corrections**. Unreleased; **Changed**,
not Added. This closes the last known-wrong behaviour in the empty-tensor family. Handoff:
`rfcs/handoffs/110-empty-axis-reduction-semantics-handoff.md`.
**Target:** core `matten`; behaviour change to existing APIs — a minor release when it ships
**Theme:** Finish RFC-105's fix on the axis-wise siblings it deliberately excluded
**Related:** RFC-105, RFC-106 (Stage 2), RFC-108

---

## 1. Summary

Five axis reductions leak sentinel values when the reduced axis has length zero. Extend RFC-105's
answer to them verbatim.

**This is the last known-wrong behaviour in the empty-tensor family.** It is a confirmation, not an
open design question — §3 explains why.

## 2. The defects, measured

```text
input [0,3], reducing axis 0 (length 0):

  sum_axis  = [0.0, 0.0, 0.0]      <- CORRECT: the additive identity, per RFC-105's sum
  mean_axis = [NaN, NaN, NaN]
  min_axis  = [inf, inf, inf]
  max_axis  = [-inf, -inf, -inf]
  var_axis  = [NaN, NaN, NaN]
  std_axis  = [NaN, NaN, NaN]
```

Measured against the built library at `0.45.0`. These are exactly the values RFC-105 removed from the
whole-tensor forms — `inf` and `-inf` are fold identities never overwritten by an empty loop, and
`NaN` is `0.0 / 0`.

**RFC-105 excluded these deliberately**, saying *"any axis-wise reduction … RFC-106 audits those."*
RFC-106 audited them and classified all five SEMANTIC. This RFC resolves that.

## 3. Why this is a confirmation, not a design question

RFC-106's audit offered one argument for keeping `NaN`:

> *some callers may prefer a `NaN`-filled or empty-shaped result to avoid an `Err` interrupting a
> batch of otherwise-valid axis slices*

**That batch cannot exist.** A reduced axis has a single length. If it is zero, **every** output slot
reduces over zero elements — `axis_reduce` removes the reduced axis and each output element iterates
`src_shape[axis]` times, uniformly. There is no partial case where some slices are valid and others
are not. The measurement above shows it: all three slots, all sentinel.

With that gone, nothing argues against the consistent answer. The owner's stated preference — *clean,
intuitive, safe* — points one way, and so does the crate's own precedent:

```text
clean      one rule for whole-tensor and axis forms, not the same question answered
           two ways in one crate
intuitive  the mean of nothing is undefined; inf is a fold artifact, not an answer
safe       a sentinel is SILENT. min_axis returning inf looks like data.
```

## 4. The fix

Identical in shape to RFC-105's, which was itself copied from `try_var`:

```text
try_mean_axis, try_min_axis, try_max_axis, try_var_axis, try_std_axis:
    if the reduced axis has length 0 -> Err(MattenError::InvalidArgument{..}),
    worded as RFC-105 worded the whole-tensor forms
```

The panicking forms inherit the message through their existing `unwrap_or_else` delegation. **No new
error variant, no signature change, no new pattern.**

**`sum_axis` is not touched.** `0.0` per output slot is the additive identity and is correct — the
same boundary RFC-105 drew for whole-tensor `sum`.

### 4.1 The distinction that must not be blurred

```text
ZERO-LENGTH REDUCED axis    [0,3].mean_axis(0)  -> nothing to reduce  -> Err
ZERO-LENGTH SURVIVING axis  [0,3].mean_axis(1)  -> shape [0], no work -> Ok, empty
```

The second is already correct and must stay correct. Only the *reduced* axis's length matters.

## 5. Scope

### In scope

```text
the five try_*_axis forms, and the messages their panicking forms inherit
tests per §7, on sliced-empty fixtures
docs wherever these five document their empty behaviour
```

### Out of scope — a diff touching these is a defect

```text
sum_axis (§4)
the whole-tensor forms — RFC-105 settled them
RFC-106 Stage 3: checked_shape_len, the serde and ndarray round-trips, Display
any companion crate
CHANGELOG.md — the release RFC writes it
```

## 6. Risks

```text
1. THE SURVIVING-AXIS CASE. [0,3].mean_axis(1) must stay Ok with shape [0]. A guard
   written on "is the tensor empty" rather than "is the reduced axis empty" breaks
   it. Test both axes of the same tensor.
2. THE FIXTURE MUST BE SLICED-EMPTY. No constructor accepts a zero-sized shape.
   A fixture with a non-zero reduced axis makes every new test pass vacuously.
3. NON-SQUARE. [0,3] and [3,0] behave differently per axis. Test both orientations,
   or a transposed bug stays invisible.
4. sum_axis DRIFT. It is correct today. If a shared helper is changed rather than
   the five call sites, sum_axis moves with them.
```

## 7. Acceptance criteria

```text
[x] the five try_*_axis forms return Err when the REDUCED axis has length 0
[x] each message mirrors RFC-105's wording for its whole-tensor sibling
[x] the panicking forms carry that message, asserted against the captured text
[x] [0,3].mean_axis(1) and [3,0].mean_axis(0) still return Ok with an empty result
    — the surviving-axis case, both orientations (risks 1, 3)
[x] sum_axis unchanged on every case above — asserted, not assumed
[x] whole-tensor forms unchanged; existing suite passes unmodified
[x] both feature profiles; cargo test --workspace
[x] all eight guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[x] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
[x] no version bump, tag, or publish
```

## 8. Release note

**Changed**, not Added — observable for anyone relying on the sentinels. Recorded for the release
RFC; not written into `CHANGELOG.md` here.

## 9. Non-goals

```text
RFC-106 Stage 3 in any part
sum_axis's identity
Display on an empty tensor (Stage 3's sixth SEMANTIC row)
```
