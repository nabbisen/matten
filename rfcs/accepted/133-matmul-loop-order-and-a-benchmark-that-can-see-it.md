# RFC-133: Matmul Loop Order, and a Benchmark That Can See It

**Status:** **Accepted** 2026-09-03 by the owner. Handoff:
`rfcs/handoffs/133-matmul-loop-order-handoff.md`. RFC-127's collision is resolved — it shipped in
`0.46.2` — so this is unblocked. **No version bump, tag, or publish is authorized by this
acceptance**; each is a separate owner authorization at the time (RFC-094 §5).
**Target:** `crates/matten/src/math.rs`, `benchmarks/benches/core.rs`
**Theme:** A 10× win from reordering two loops — and first, a measurement that can observe it
**Related:** RFC-049 (benchmark scope, expanded here with owner authorization), RFC-127 (must ship
first — file collision), external audit P-2/P-3

---

## 1. Summary

```text
A  add a 512x512 case to the benchmark harness       FIRST, so the change is measurable
B  interchange mm_mul's loops i-j-k -> i-k-j          measured 170ms -> 16ms
```

**Order matters and is not negotiable:** the benchmark case lands *before* the optimisation, or the
project has no way to show the improvement it is claiming.

**This is a minor** — Change B alters floating-point summation order, so results shift in the last
bits. That is a `Changed` entry, not a patch.

## 2. Scope authorization — this expands RFC-049, deliberately

RFC-049 closed with Phase 4 — regression thresholds and hard gates — *"designed but **not
authorized**"*, and `benchmarks/README.md:5` states the harness exists **"not to claim matten is
faster"**.

```text
AUTHORIZED by the owner 2026-09-01:  add a 512x512 case
NOT AUTHORIZED, explicitly:          Phase 4 hard gates / regression thresholds
```

**The harness's purpose is unchanged.** A 512×512 case does not turn it into a performance-regression
gate; it lets the project observe its own change. The external audit framed the current `side = 64`
as a failure of the harness (its P-3). It is not — the harness was never a regression gate. It simply
cannot see this particular improvement, which is a reason to add a case, not a reproach.

## 3. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | `benchmarks/benches/core.rs:14` uses `let side = 64;` — 4096 elements, cache-resident | direct read |
| E2 | The audit measured 512×512 at **170 ms vs 16 ms** for the identical algorithm reordered, agreeing to 1e-6 | external audit P-2 — **not independently re-measured**; Change A exists to verify it |
| E3 | The inner loop strides through `b` by `p` per step | `crates/matten/src/math.rs` `mm_mul` |
| E4 | The harness disclaims speed claims | `benchmarks/README.md:5` |
| E5 | RFC-049 Phase 4 is unauthorized | RFC-049 |

**E2 is the audit's number, not mine.** Change A's whole job is to make it checkable before Change B
is trusted.

## 4. Change A — the benchmark case, first

```text
add a 512x512 matmul case alongside the existing side = 64
KEEP the 64 case — it measures a different regime and removing it loses history
```

**Run it and record the baseline before touching `mm_mul`.** If the measured gap is not roughly E2's
10×, that is a finding: either the audit's number does not reproduce here, or the machine differs
materially. **Report the number either way** — this RFC is not worth doing for a 1.2× win.

## 5. Change B — the loop interchange

```text
i-j-k  ->  i-k-j
```

Identical arithmetic, identical result to within float associativity, different memory access order.
Nothing about the public API, the signature, or the shape rules changes.

```text
DO NOT restructure mm_mul beyond the loop order. RFC-108 fixed a zero-column
panic in this function and RFC-127 adds an allocation guard to it; a rewrite is
how those get silently undone.
DO keep the p == 0 guard (RFC-108) and whatever RFC-127 adds, byte-identical
in behaviour.
```

### 5.1 The float consequence, stated plainly

Summation order changes, so results change in the last bits. That is:

```text
a `Changed` CHANGELOG entry, worded so a reader understands their numbers may
differ in the final digits — NOT a bug fix, NOT a silent improvement
a MINOR, not a patch
```

**Existing tests that assert exact float equality on matmul results will need review.** Any you
change is part of this change's surface — list them.

## 6. What about P-1, the 65× axis reduction?

The audit also reports axis reductions ~65× slower than whole reductions, from strides recomputed per
element (`math.rs` `axis_reduce`).

**It is NOT in this RFC, and the distinction is worth stating:**

```text
P-2 (this RFC)   changes summation ORDER -> float results change -> `Changed`
P-1 (not here)   hoists stride computation -> accumulation order UNCHANGED
                 -> results bit-identical -> a pure optimisation
```

They are different kinds of change and deserve separate decisions. P-1 is arguably the easier sell —
no behaviour change at all — and `stats.rs:50-71` already contains the faster shape to copy.

**Recommend it as a follow-up RFC.** Not folded in, because "these two both make things faster" is not
a good enough reason to give them one review.

## 7. Scope

### Out of scope — a diff touching these is a defect

```text
Phase 4 hard gates / regression thresholds   explicitly NOT authorized (§2)
P-1, the axis-reduction hoist                 §6, its own RFC
any mm_mul change beyond loop order           §5
parallelism / rayon                           the audit itself says to do the
                                              10-65x single-threaded work first
RFC-127's allocation guard in math.rs         ships first; do not duplicate or
                                              revert it
the version bump                              the 0.47.0 release RFC owns it
```

## 8. Sequencing — RFC-127 first, and this one collides

**RFC-127 also edits `math.rs`.** This RFC must land after it, or the two conflict in the same
function.

```text
RFC-127 -> 0.46.2   (math.rs: allocation guards)          SHIPPED 2026-09-03
then this           -> rides 0.47.0 with RFC-129/RFC-132  SUPERSEDED, see below
```

> **Amended 2026-09-03 on acceptance.** Both halves of the sequencing above are now history:
> RFC-127 shipped in `0.46.2` and `0.47.0` was tagged and published the same day, carrying RFC-128,
> RFC-129, RFC-131 and RFC-132. **The collision this section existed to prevent is therefore gone,
> and `math.rs` is at its post-RFC-127 shape** — which is what §5's "keep RFC-127's guard
> byte-identical" now refers to concretely, rather than to an unlanded change.
>
> **This RFC now rides `0.48.0`, and is the first theme in it.** Under RFC-094 §4.2 that has a
> consequence worth recording: a minor needs two themes, 28 days, or the owner asking, so this RFC
> is one of the two triggers for the release that will also carry **RFC-130's Change C** (the
> manifest `homepage` key), which missed `0.47.0`'s window.

## 9. Risks

```text
R1  Doing Change B before Change A, so the improvement is unmeasured (§4).
R2  Restructuring mm_mul and silently undoing RFC-108's p == 0 guard or
    RFC-127's allocation guard (§5).
R3  Shipping in a patch. Summation order changes float output (§5.1).
R4  Editing exact-equality tests without listing them (§5.1).
R5  Drifting into Phase 4 gates because the benchmark is already open (§2).
R6  Treating E2's 10x as established. It is the audit's number; verify it (§4).
R7  Landing before RFC-127 (§8).
```

## 10. Acceptance criteria

```text
[ ] the 512x512 case added FIRST, and the baseline recorded before Change B
[ ] the measured before/after reported — if it is not roughly 10x, say so
[ ] the side = 64 case retained
[ ] mm_mul changed in loop order ONLY; RFC-108's and RFC-127's guards intact
[ ] results agree with the previous implementation to ~1e-6 on a random case
[ ] every exact-equality test that needed changing is listed with its reason
[ ] no public API change; no signature change
[ ] no Phase 4 gate, threshold, or CI failure condition added
[ ] nine guards; cargo test --workspace; both feature profiles
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
[ ] no version bump, tag, or publish
```

## 11. What this does not fix

```text
- P-1's 65x axis reductions (§6)
- the benchmark harness's inability to DETECT a regression — that is Phase 4
  and it stays unauthorized. This RFC makes one improvement visible; it does not
  make future regressions visible, and nobody should read it as having done so.
- matten's positioning. Speed is still explicitly not the proposition, and a 10x
  win on one operation does not change that.
```
