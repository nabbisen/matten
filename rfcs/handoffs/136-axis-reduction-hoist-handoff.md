# RFC-136 Developer Handoff — The Axis-Reduction Hoist

**RFC:** `rfcs/accepted/136-axis-reduction-hoist.md`
**Status:** Accepted 2026-09-03 by the owner. Unblocked — RFC-133 has landed (`0f42a8f`).
**Target:** `crates/matten/src/math.rs` — `axis_reduce` (:520) and `nan_axis_reduce` (:463)
**Authority:** where this document and the RFC disagree, **the RFC wins.**

---

## 1. The one-sentence version

Stop recomputing where every element belongs. Two functions, the same hoist, **14–111× faster and
bit-identical** — with the bit-identity derivable before you write anything, not discovered after.

## 2. Read this before you start: the last two rounds' lesson

RFC-133's design document told you results would change in the last bits. It was wrong, you proved
it wrong, and the RFC was amended. RFC-132's site list would have reintroduced a Critical, and you
caught that before writing code.

**This handoff's factual claims deserve the same treatment.** The measurements in §5 are mine and I
re-derived them from the current tree, but derive them again. The one place I would look hardest is
§7's safety argument, because it is the one that depends on a policy decision made in a different
function.

## 3. Change A — `axis_reduce` (`math.rs:520`)

The current loop allocates four `Vec`s per input element (`flat_to_coord` twice, `out_coord`,
`coord_to_flat` again). Replace it with the decomposition `stats.rs:50-71` already uses:

```rust
let axis_len = src_shape[axis];
let outer: usize = src_shape[..axis].iter().product();
let inner: usize = src_shape[axis + 1..].iter().product();

for o in 0..outer {
    let base = o * axis_len * inner;
    let dst = o * inner;
    for a in 0..axis_len {
        let row = base + a * inner;
        for i in 0..inner {
            out_data[dst + i] = f(out_data[dst + i], t.data[row + i]);
        }
    }
}
```

**No allocation inside any loop.** `out_data` stays `vec![identity; out_len]`, so accumulator
initialisation is unchanged.

```text
KEEP  RFC-127's guard at :547 exactly where it is, comment intact. It guards a
      REAL case (sum_axis reaching here with a zero-length reduced axis whose
      surviving axes are unbounded) and §7 shows it is load-bearing.
DROP  the `if out_shape.is_empty() { 0 }` special case only if you have
      satisfied yourself it is subsumed -- for rank-1 input, outer = 1 and
      inner = 1, so dst = 0, which is the same answer. Verify, don't assume.
```

## 4. Change B — `nan_axis_reduce` (`math.rs:463`)

Same hoist. The only difference is that `has_nan[dst + i]` is updated alongside `out_data[dst + i]`,
preserving the existing split between "saw a NaN" and "accumulate the value".

**Also add the `check_shape` guard here** — see §7 for why, and read §7 before deciding you
disagree, because the obvious first reading of that code is wrong in both directions.

## 5. The numbers to beat, and to re-derive

Measured on `[256, 256, 16]` (1 048 576 elements), release profile:

```text
axis 0   current 26.12ms   hoisted  322.39µs    81.0x   bit_identical
axis 1   current 26.40ms   hoisted  238.14µs   110.9x   bit_identical
axis 2   current 26.54ms   hoisted    1.84ms    14.4x   bit_identical

whole-tensor sum() on the same data: 399.36µs
  -- so an axis reduction currently costs ~65x a full one, which independently
     reproduces the external audit's P-1 on our hardware
```

**14× on axis 2 is the honest result, not a defect.** `inner = 16` gives a short inner loop with
proportionally more outer-loop overhead. Do not chase it with tiling or a small-`inner` special
case: that would break §6's guarantee for a fraction of the win, and it is explicitly out of scope.

**Report your numbers even if they differ from mine.** A disagreement is a finding, not a failure.

## 6. Why bit-identity holds — and what would break it

For one output cell, the contributing values are at `base + a*inner + i` for `a = 0..axis_len`.

```text
current:      iterates src_flat in order, so those contributions arrive in
              ASCENDING a -- a*inner is the only term varying between them
replacement:  the `a` loop runs 0..axis_len for each (o, i). Same cell, same
              values, ASCENDING a
```

Same operands, same sequence, same starting `identity`. **This holds for any `f`, not just float
addition** — which matters, because `min`/`max` are not associative in the presence of `NaN`, and a
float-associativity argument would not have covered them.

What would break it: reordering the `a` loop, splitting the reduction (tiling, blocking, a parallel
reduction over `a`), or accumulating into a temporary in a different order. **If you find yourself
changing the `a` sequence for any reason, stop** — that is a behaviour change, not an optimisation,
and it needs its own RFC.

## 7. The `nan_axis_reduce` guard — the subtle part

The two functions compute their output length differently:

```text
axis_reduce      :547  let out_len = MattenLimits::default().check_shape(&out_shape, operation)?;
nan_axis_reduce  :483  let out_len: usize = if out_shape.is_empty() { 1 }
                       else { out_shape.iter().product() };
```

**First reading: "RFC-127's defect survived into a second function."** That is wrong. I built the
case to check it — `[1_000_000, 0, 1_000_000]` is a valid empty tensor (RFC-111), and removing its
zero axis leaves 10¹² elements, an 8 TB allocation:

```text
sum_axis(1) -> caught: requested 1000000000000 elements, exceeding the limit
min_axis(1) -> caught: minimum is undefined for a reduced axis of length 0
```

`min_axis`/`max_axis` reject a zero-length reduced axis **upstream** (RFC-110), so `axis_len ≥ 1`
always holds here and `out_len ≤ t.data.len()` follows — already bounded by the input tensor's own
validation. **Not reachable.**

**Second reading: "so the guard is unnecessary."** Also wrong, and this is the one that matters. It
is unreachable because of a policy enforced in a *different function*. If `min_axis` ever adopted
`sum_axis`'s empty-axis behaviour — returning the identity rather than erroring, a plausible
harmonisation, and RFC-110 shows this project does revisit exactly these semantics — then
`nan_axis_reduce` becomes allocator-abortable that same day, with nothing in its own body to warn
whoever made the change.

**So: add the guard.** One call, and it makes the bound local to the code that allocates. That was
RFC-127's whole lesson.

## 8. Out of scope — a diff touching these is a defect

```text
tiling / blocking / small-inner special cases   §6, breaks the guarantee
parallelism, rayon                              single-threaded work first
Phase 4 gates or thresholds                     still unauthorized
new benchmark cases                             core/sum_mean_axis already
                                                covers this -- see RFC-136 §7
empty-axis SEMANTICS in min_axis/max_axis       §7's safety rests on the
                                                current policy; changing it is
                                                RFC-110 territory
the version bump                                0.48.0's release RFC owns it
```

## 9. Verify, don't assume

```text
[ ] bit-identical to the previous implementation on every axis of a rank-3
    tensor, and on rank-1 and rank-2 -- compare to_bits(), not approximate
    equality, since the claim is exactness
[ ] the rank-1 case (out_shape empty) returns what it did before
[ ] zero-length REDUCED axis: sum_axis still returns the identity, min_axis
    still errors -- both paths, both unchanged
[ ] zero-length SURVIVING axis still behaves as before
[ ] RFC-127's guard in axis_reduce still fires on [1_000_000, 0, 1_000_000]
[ ] contiguity assumption still valid: all four callers reject_dynamic first
    (math.rs:279, 326, 383, 441) -- confirm, since the hoist depends on it
```

The contiguity check matters more than it looks. The hoisted form indexes `t.data` by computed
offsets, which is only correct for a contiguous row-major buffer. It holds today because every
caller rejects dynamic tensors first; if that ever stopped being true the hoist would read the wrong
elements silently, with no panic to signal it.

## 10. Definition of done

```text
[ ] axis_reduce hoisted; zero allocations inside any loop
[ ] nan_axis_reduce hoisted, has_nan updated alongside out_data
[ ] nan_axis_reduce gains the check_shape guard (§7)
[ ] RFC-127's guard in axis_reduce unmoved, comment intact
[ ] all of §9 verified and reported
[ ] before/after timings reported per axis
[ ] no public API or signature change
[ ] no new benchmark case, no Phase 4 gate
[ ] nine guards; cargo test --workspace; both feature profiles
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
[ ] no version bump, tag, or publish
```

## 11. Review request

Write `.git-exclude/review-request/RFC-136/`. Include the per-axis timings, the method you used for
the bit-identity comparison, and the zero-length-axis results for **both** paths. If you conclude
§7's guard is unnecessary, say so with the reasoning — disagreeing with this document on a point it
has argued at length is exactly the kind of pushback the last three rounds have been right to
produce.
