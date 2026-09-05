# RFC-136: The Axis-Reduction Hoist

**Status:** **Implemented** 2026-09-05 in commit *"Hoist the per-element coordinate arithmetic out of
both axis reductions (RFC-136)"* (`3db17b2`), reviewed and approved. **Unreleased** — the second
theme of `0.48.0`, satisfying RFC-094 §4.2's two-theme trigger alongside RFC-133. **Bit-identical**
across ranks 1-4, every axis, with NaN scattered and an all-NaN case, verified at review against an
independent reconstruction of the pre-change algorithm.

> **§2's measured figures are narrowed by the review — see §2.1.** The 81× / 111× / 14× reported
> below are real, but so are the implementer's 199.6× / 109.8× / 16.6× on the *same machine*. The
> ratio is not a stable number; **the floor is ~10×**, and that is what the changelog may claim.

Handoff: `rfcs/handoffs/136-axis-reduction-hoist-handoff.md`.
**Target:** `crates/matten/src/math.rs` (`axis_reduce`, `nan_axis_reduce`)
**Theme:** A 14–111× win with no numeric change at all — and the harness saw this one two years ago
**Related:** RFC-133 (the sibling optimisation, and the contrast), RFC-049 (the harness that already
measures this), RFC-094 §4.1 / RFC-120 (why it is still a minor), external audit P-1

---

## 1. Summary

```text
A  hoist axis_reduce's per-element coordinate arithmetic          measured 14-111x
B  the same hoist in nan_axis_reduce (min_axis / max_axis)        same defect, same fix
```

**This is a pure optimisation.** Results are **bit-identical by construction** — not merely to
`1e-6`, and not, as with RFC-133, bit-identical as a fact discovered after the design claimed
otherwise. Here the reasoning is available up front and is given in §5.

**It is still a minor** (§9), for the same reason RFC-133 turned out to be: a performance
improvement is not a correctness fix, so it cannot be patch content.

## 2. Evidence — measured here, not inherited

Unlike RFC-133, whose 10× was the external auditor's number until Change A checked it, every figure
below was produced on this project's own hardware against the current tree.

| # | Claim | Established by |
|---|---|---|
| E1 | Current `sum_axis` on a 1 048 576-element `[256,256,16]` tensor: **~26 ms on every axis** | direct measurement |
| E2 | The `outer`/`axis_len`/`inner` form: **322 µs / 238 µs / 1.84 ms** for axes 0/1/2 | same probe |
| E3 | Speedup **81× / 111× / 14×** by axis; the spread is real (§4.1) | E1 ÷ E2 |
| E4 | Output is **bit-identical** on all three axes | `to_bits()` comparison |
| E5 | Whole-tensor `sum()` on the same data: **399 µs** — so an axis reduction costs **~65×** a full one | same probe |
| E6 | The cost is **four heap allocations per element**, not "recomputed strides" | direct read (§3) |
| E7 | `stats.rs:50-71` already contains the target form | direct read |
| E8 | All four callers `reject_dynamic` before reaching `axis_reduce`, so the data is contiguous | `math.rs:279,326,383,441` |

**E5 independently reproduces the external audit's P-1** (~65× vs whole reductions) on different
hardware, which is worth stating because RFC-133's E2 also reproduced. The auditor's performance
numbers have now been checked twice and held twice.

### 2.1 The ratio is unstable — amended 2026-09-05 at review

The figures in §2 are honest measurements and do not reproduce. Neither do the implementer's. The
cause was found at review and is specific to what this RFC removes:

```text
old algorithm, axis 0, same binary, same machine:
    cold (first run ever)   75.39 ms
    warm (best of 5)        38.35 ms     <- 1.97x from allocator warm-up alone
and the WARM baseline itself moved 14.96 ms -> 38.64 ms between invocations
```

The old path's cost is dominated by **four million heap allocations**, making it unusually sensitive
to allocator state and machine load. The hoisted path is not — every run agrees to within a few
percent. So the *numerator* is stable and the *denominator* is not, which is why 81×, 110× and 199×
are all true of the same change.

**Consequence for §9's changelog wording:** claim the **floor**, never a headline multiplier. Axis 2
measured 10.1× / 14.4× / 14.6× / 16.6× across four runs, so *"an order of magnitude or more, with no
numeric change"* holds under every methodology tried. A floor is the right shape for a performance
claim regardless: it is what a user is entitled to rely on.

> The implementer attributed the gap to CPU generation and compiler version. Both runs were on the
> same `AMD Ryzen 9 9950X`, one rustc patch release apart — so that explanation was wrong, and the
> methodological one replaces it. **A ratio is a claim about two numbers, and the unstable one here
> is a baseline nobody will ever run again.**

## 3. What the cost actually is

The audit called this *"strides recomputed per element"*. That undersells it. Per element of the
**input** tensor, `axis_reduce` performs:

```text
flat_to_coord(src_flat, src_shape)
    strides_for_shape(shape)      -> Vec alloc      (1)
    coord = vec![0; ndim]         -> Vec alloc      (2)
out_coord: Vec<usize> = ...collect()  -> Vec alloc  (3)
coord_to_flat(&out_coord, &out_shape)
    strides_for_shape(out_shape)  -> Vec alloc      (4)
```

**Four heap allocations and two full stride computations for every scalar added.** On the
1 048 576-element case that is over four million allocations to perform one million additions. The
arithmetic is not the cost; the bookkeeping around it is.

This also explains E3's spread better than "cache effects" would: the overhead is per **input**
element and therefore near-constant across axes (E1: ~26 ms regardless), while the *replacement*'s
cost depends on the inner-loop length. Axis 2 has `inner = 16`, a short inner loop with more
outer-loop overhead, so it gains least. The win is large wherever the reduction is large.

## 4. Change A — `axis_reduce`

Replace the per-element coordinate round-trip with the decomposition `stats.rs` already uses:

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

No allocation inside any loop. `out_data` is already `vec![identity; out_len]`, so the accumulator
initialisation is unchanged.

**Keep RFC-127's `check_shape(&out_shape, operation)?` exactly where it is** (`math.rs:547`) and
keep its comment. It guards a real case — `sum_axis` legitimately reaches this function with a
zero-length *reduced* axis, whose surviving axes are not bounded by that zero — and §6 shows that
guard is load-bearing in a way that is easy to miss.

### 4.1 Do not "fix" the axis-2 case

14× is the honest result for a short inner loop, not a defect in the transform. Blocking, tiling, or
special-casing small `inner` to chase it would break the bit-identity guarantee (§5) for a fraction
of the win. **Out of scope, and a diff attempting it is a defect.**

## 5. Why this is bit-identical *by construction*

For a fixed output cell, the contributing input elements are the `axis_len` values at
`base + a*inner + i` for `a = 0..axis_len`.

```text
current:      iterates src_flat 0..len in order. For a given output cell those
              contributions are visited in ASCENDING a, because a*inner is the
              only term that varies between them.

replacement:  the `a` loop runs 0..axis_len for each (o, i). Same cell, same
              values, ASCENDING a.
```

Same values, same order, same starting `identity`. The accumulator `f` is applied to the same
operands in the same sequence, so the result is identical **for any `f` whatsoever** — not only for
float addition. That is a stronger guarantee than a float-associativity argument, and it covers
`min`/`max` (non-associative in the presence of `NaN`) as well as `sum`.

E4 confirms it empirically on all three axes; the argument above is why it is not luck.

> **The contrast with RFC-133 is the point.** There, the design document asserted the results would
> change and was wrong; bit-identity was *discovered* at implementation. Here it is derivable before
> a line is written, because the transform does not reorder anything — it only stops recomputing
> where each element belongs. Both are "loop restructuring"; only one of them ever needed an
> empirical check to know what it did. **Deriving the property first is the cheaper habit**, and
> RFC-133 §5.1's amendment is the record of what it costs not to.

## 6. Change B — `nan_axis_reduce`, and a coupling worth writing down

`nan_axis_reduce` (`math.rs:463`, serving `min_axis`/`max_axis`) has the identical per-element
pattern and gets the identical hoist, with `has_nan[dst]` updated alongside `out_data[dst]`.

**One asymmetry must be understood before touching it.** `axis_reduce` computes its output length
through RFC-127's guard; `nan_axis_reduce` computes it as a bare `out_shape.iter().product()`, with
no guard at all:

```text
axis_reduce      :547  let out_len = MattenLimits::default().check_shape(&out_shape, operation)?;
nan_axis_reduce  :483  let out_len: usize = if out_shape.is_empty() { 1 }
                       else { out_shape.iter().product() };
```

That looks like the RFC-127 defect surviving in a second function. **It is not, and I verified it
rather than assuming either way.** A tensor of shape `[1_000_000, 0, 1_000_000]` is valid and empty
(RFC-111), and removing its zero axis leaves an output of 10¹² elements — an 8 TB allocation. Down
the `sum_axis` path RFC-127's guard catches exactly that:

```text
sum_axis(1) -> caught: requested 1000000000000 elements, exceeding the limit
min_axis(1) -> caught: minimum is undefined for a reduced axis of length 0
```

`min_axis`/`max_axis` reject a zero-length reduced axis **upstream** (RFC-110), so `axis_len ≥ 1`
always holds by the time `nan_axis_reduce` runs, and `out_len ≤ t.data.len()` follows — already
bounded by the input tensor's own validation. The missing guard is genuinely unreachable.

**But it is unreachable for a reason that lives in a different function.** If `min_axis` ever
adopted `sum_axis`'s empty-axis policy — returning the identity instead of an error, which is a
plausible future harmonisation, and RFC-110 shows this project does revisit exactly these
semantics — `nan_axis_reduce` would become allocator-abortable the same day, with nothing in its own
body to say so.

```text
REQUIRED: add the check_shape guard to nan_axis_reduce as well.
```

Not because it is reachable today, but because a bound that costs one call and depends for its
safety on a policy decision made elsewhere should not be left implicit. Alternatively a comment
would do — but the guard is cheaper than the comment is reliable, and RFC-127's whole lesson was
that an unbounded allocation is worth guarding at the site that allocates.

## 7. Benchmarks — the harness already sees this one

**No new benchmark case is required, and that is itself the finding.** `core/sum_mean_axis` has
existed since RFC-049 and both baseline reports flagged it, in these words:

> *"The clearest signal is again `sum_mean_axis` at ~1.30 ms — roughly ~400× the whole-tensor
> `sum_mean` … the natural first place to look if axis-reduction cost ever matters. Positioning /
> regression-visibility information, **not a defect**."*
> — `benchmarks/reports/internal-baseline-v0.2.md:109`

The harness worked. It measured the slowest core path, named it the clearest signal, said *"again"*
because v0.1 had said the same, and even predicted the correct response. Then the observation was
classified as *not a defect* — which was defensible, since `matten` is DX-first and not a
performance crate — and **nothing converted it into a decision for roughly twenty releases**, until
an external auditor reported the same number as a finding.

```text
RFC-133   the harness was BLIND (side = 64 is cache-resident) -> add a case
RFC-136   the harness SAW IT, twice, and said so -> nothing happened
```

Coverage was never the problem here. What is missing is a step that turns a measurement into a
disposition. **That gap is not fixed by this RFC** — it is a process question, recorded in §12 as a
candidate rather than smuggled into a code change.

## 8. Scope

### Out of scope — a diff touching these is a defect

```text
Phase 4 gates / thresholds            still unauthorized (RFC-049, RFC-133 §2)
parallelism, rayon                    single-threaded work first
tiling / blocking / small-inner cases  §4.1 -- breaks §5's guarantee
any semantic change to empty-axis behaviour   §6 depends on the CURRENT policy;
                                      changing it is RFC-110 territory, not this
the version bump                      0.48.0's release RFC owns it
new benchmark cases                   §7 -- the coverage already exists
```

## 9. Release classification

**A minor, and the reasoning is now settled precedent.** RFC-094 §4.1 as amended by RFC-120 confines
patches to *"correctness fixes to already-published crate content."* Nothing here is a correctness
fix — the results were always right, merely expensive to obtain. So it cannot ride a patch.

The changelog should claim, as RFC-133's will:

```text
no numeric change; axis reductions 14-111x faster depending on axis
```

**This is the second theme of `0.48.0`**, which under RFC-094 §4.2 satisfies the two-theme trigger
alongside RFC-133 — and that release also carries RFC-130's Change C, which missed `0.47.0`'s
window.

## 10. Risks

```text
R1  Changing accumulation order while restructuring, forfeiting §5's guarantee.
    The `a` loop must remain the per-cell sequence.
R2  Dropping or moving RFC-127's guard in axis_reduce (§4).
R3  Assuming nan_axis_reduce's missing guard is a live vulnerability -- it is
    not (§6) -- or assuming it is therefore unnecessary. Both readings are wrong.
R4  Chasing the axis-2 case with tiling (§4.1).
R5  Shipping in a patch (§9).
R6  Touching empty-axis semantics, which §6's safety argument depends on.
R7  Assuming contiguity without checking. It holds only because all four
    callers reject dynamic tensors first (E8) -- verify that still holds.
```

## 11. Acceptance criteria

```text
[ ] axis_reduce hoisted; no allocation inside any loop
[ ] nan_axis_reduce hoisted the same way, has_nan updated alongside out_data
[ ] nan_axis_reduce gains the check_shape guard (§6)
[ ] RFC-127's guard in axis_reduce unmoved, comment intact
[ ] output bit-identical to the previous implementation on every axis of a
    rank-3 tensor, and on rank-1 and rank-2 cases
[ ] the zero-length-reduced-axis cases still behave exactly as before, both
    the sum_axis path (identity) and the min_axis path (error)
[ ] before/after timings reported per axis
[ ] no public API or signature change
[ ] no new benchmark case, no Phase 4 gate
[ ] nine guards; cargo test --workspace; both feature profiles
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
[ ] no version bump, tag, or publish
```

## 12. What this does not fix

```text
- the harness's inability to DETECT a regression. Still Phase 4, still
  unauthorized (RFC-133 said this too, and it remains true).
- the gap §7 identifies: a measurement recorded as "not a defect" has no
  route to becoming a decision. Two baseline reports named this the clearest
  signal in the suite and it took an external audit to act on it. Worth a
  process RFC; deliberately NOT bundled into a code change.
- matten's positioning. Speed is still not the proposition. A DX-first crate
  may nonetheless decline to be 65x slower than necessary at no cost in
  clarity, which is the whole argument for doing this.
```
