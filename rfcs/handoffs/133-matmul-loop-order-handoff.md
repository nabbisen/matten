# RFC-133 Developer Handoff — Matmul Loop Order, and a Benchmark That Can See It

**RFC:** `rfcs/accepted/133-matmul-loop-order-and-a-benchmark-that-can-see-it.md`
**Status:** Accepted 2026-09-03 by the owner. **Unblocked** — RFC-127 shipped in `0.46.2`.
**Target:** `crates/matten/src/math.rs`, `benchmarks/benches/core.rs`
**Authority:** where this document and the RFC disagree, **the RFC wins.**

---

## 1. The one-sentence version

Reorder two loops in `mm_mul` for a measured ~10× win — but **add the benchmark case that can see it
first**, because a performance claim this project cannot reproduce on its own harness is not a claim
it should make.

## 2. Order is not negotiable

```text
Change A   add a 512x512 case to benchmarks/benches/core.rs      <- FIRST
           run it, RECORD the baseline
Change B   interchange mm_mul's loops                            <- SECOND
           run it again, REPORT before/after
```

Doing B first destroys the only chance to measure A's baseline on this machine. If you find yourself
with a faster `mm_mul` and no recorded "before", the measurement is gone and cannot be recovered
without reverting.

## 3. The number is the audit's, not ours — verify it

The RFC's E2 (170 ms → 16 ms) comes from the external architect's measurement on **their** machine.
It has never been reproduced here. Change A exists precisely to check it.

**Report what you actually measure, whatever it is.** If it is ~10×, say so. If it is 1.2×, say that
too and stop — the RFC states plainly it is not worth doing for a small win, and a truthful negative
result is a complete and successful outcome for this task. Do not tune the benchmark until it agrees
with the audit.

## 4. Change A — the benchmark case

`benchmarks/benches/core.rs:14` currently has `let side = 64;` — 4096 elements, comfortably
cache-resident, which is why the harness cannot see this improvement today.

```text
ADD     a 512x512 matmul case
KEEP    the existing 64 case -- it measures a different (cache-resident) regime,
        and deleting it silently discards comparability with every past run
```

**This is not a regression gate and must not become one.** RFC-049's Phase 4 (thresholds, hard gates,
CI failure conditions) is explicitly **not authorized**, and `benchmarks/README.md:5` still says the
harness exists *"not to claim matten is faster"*. You are adding an instrument, not a verdict.

## 5. Change B — the interchange

Here is the function as it stands **after** RFC-127, at `crates/matten/src/math.rs:731`:

```rust
for (i, row) in out.chunks_mut(p).enumerate() {
    for (j, slot) in row.iter_mut().enumerate() {
        let mut acc = 0.0f64;
        for k in 0..n {
            acc += a.data[i * n + k] * b.data[k * p + j];
        }
        *slot = acc;
    }
}
```

The inner loop strides through `b` by `p` per step — one cache line per multiply at 512×512. The
interchange makes the innermost access contiguous in both operands, accumulating into `out` in place
(it is already zero-initialised):

```rust
for (i, row) in out.chunks_mut(p).enumerate() {
    for k in 0..n {
        let aik = a.data[i * n + k];
        let b_row = &b.data[k * p..k * p + p];
        for (slot, &bkj) in row.iter_mut().zip(b_row) {
            *slot += aik * bkj;
        }
    }
}
```

**Change the loop order and nothing else.** Two guards live in this function and both must survive
byte-identical in behaviour:

```text
RFC-108   the `p == 0` early return -- chunks_mut(0) panics regardless of length
RFC-127   the check_shape(&[m, p]) allocation guard at :739
```

A rewrite is how those get silently undone, and the RFC names that as R2.

### 5.1 One trap that looks like a free win — do not take it

The `i-k-j` shape invites skipping zero multipliers:

```rust
if aik == 0.0 { continue; }   // DO NOT
```

It is not equivalent. `0.0 * inf` is `NaN` and `0.0 * NaN` is `NaN`; skipping turns both into a
silent `+0.0`. That is a real behaviour change on exactly the inputs nobody tests, and it is outside
this RFC's scope besides.

### 5.2 The float consequence, which you must not bury

Summation order changes, so results change in the last bits. This is **a `Changed` entry and a
minor** — not a bug fix, not a silent improvement. Word the changelog so a reader understands their
numbers may differ in the final digits.

**Every test asserting exact float equality on a matmul result must be listed** with the reason it
changed. Do not quietly relax an assertion; a test that was pinning a value for a reason deserves to
have that reason re-stated, not erased. Check `n == 0` and `p == 0` still produce the same empty
results — both paths change shape under the interchange even though neither changes value.

## 6. Verify equivalence, don't assume it

```text
[ ] random [512,512] x [512,512] agrees with the pre-change implementation to ~1e-6
[ ] the m/n/p == 0 cases still return the same empty matrices (RFC-108's guard)
[ ] RFC-127's guard still rejects [1048576,1] x [1,1048576] -- it is a catchable
    error, NOT an allocator abort; the whole point of 0.46.2
```

Keep a copy of the old implementation around long enough to diff outputs against it. Reasoning that
the arithmetic is identical is not the same as observing that it is.

## 7. Out of scope — a diff touching these is a defect

```text
P-1, the 65x axis-reduction hoist        its own RFC (RFC-133 §6). Different KIND
                                         of change: bit-identical results, so it
                                         is a pure optimisation and deserves its
                                         own review, not a ride on this one
Phase 4 gates / thresholds               explicitly unauthorized (§2 of the RFC)
parallelism, rayon                       the audit says do single-threaded first
any mm_mul change beyond loop order      §5
the version bump                         0.48.0's release RFC will own it
```

## 8. Release context — do not act on it, just know it

This is the **first theme of `0.48.0`**. Under RFC-094 §4.2 a minor needs two themes, 28 days, or the
owner asking, so landing this starts that clock; the same release will carry RFC-130's Change C.
**Land the code and stop.** No version bump, no tag, no publish — each is a separate owner
authorization at the time.

## 9. Risks, restated as things that have actually gone wrong before

```text
R1  Change B before Change A -- the baseline is unrecoverable
R2  Restructuring mm_mul and undoing RFC-108's or RFC-127's guard (§5)
R3  Shipping in a patch. Float output changes (§5.2)
R4  Editing exact-equality tests without listing them (§5.2)
R5  Drifting into Phase 4 because the benchmark file is already open (§4)
R6  Treating the 10x as established -- it is the audit's number (§3)
R7  Taking the `aik == 0.0` shortcut (§5.1)
```

## 10. Definition of done

```text
[ ] 512x512 case added FIRST; baseline recorded before Change B
[ ] before/after reported honestly -- including if it is not ~10x
[ ] the side = 64 case retained
[ ] mm_mul changed in loop ORDER only; both guards intact and proven so
[ ] agreement to ~1e-6 with the previous implementation on a random case
[ ] every exact-equality test that changed is listed with its reason
[ ] no public API or signature change
[ ] no Phase 4 gate, threshold, or CI failure condition
[ ] nine guards; cargo test --workspace; both feature profiles
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
[ ] no version bump, tag, or publish
```

## 11. Review request

Write `.git-exclude/review-request/RFC-133/`. Include **the measured numbers** — before and after,
with the machine and profile — the equivalence check's method and result, and the list of tests you
changed with reasons. If the win did not reproduce, that document is still the deliverable and the
finding is the result.
