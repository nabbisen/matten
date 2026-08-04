# RFC-098: Batched Matrix Multiplication

**Status:** **Superseded by RFC-099** (2026-08-04), before acceptance. The owner asked whether
this recommendation was oriented to safety or to functionality; the honest answer was
functionality, and the split follows from that.

This RFC bundled a **robustness fix to shipped API** — `matmul`/`dot` panicking with no `Result`
form, alone among 41 `try_*` functions — with a **new capability** nobody is currently missing.
Bundled that way, declining the capability would also decline the fix, which is backwards. §4 of
this RFC is therefore taken up by RFC-099 on its own, and batched matmul returns to ROADMAP §3.1
as a candidate needing a positive argument rather than the expired one it had.

Preserved rather than deleted: §3's family-car argument, §5's semantics comparison and §9's scope
lock are the substance any future batched-matmul RFC has to engage with, including the admission
that §3 was written to persuade
**Target:** core `matten`; a public API addition, so a minor release when it ships
**Theme:** The largest remaining gap in core's numeric surface, and the first library capability since RFC-090
**Related:** RFC-002, RFC-005, RFC-010, RFC-018, RFC-020, RFC-041, RFC-047, RFC-055, RFC-056, RFC-094, ROADMAP §1.1

---

## 1. Summary

Let `matmul` and `dot` accept rank > 2, treating leading dimensions as a batch.

This RFC also has to decide something it did not set out to: **whether `try_matmul` arrives with it**.
§4 argues it must.

## 2. Why now — the deferral's stated basis has expired

Two records defer this, and both give a reason that no longer holds:

```text
compatibility.md:130   "Batched matmul (rank > 2) | Deferred | RFC-010 scope: [m,n]×[n,p] maximum."
compatibility.md:140   "Higher-rank batched matmul is out of scope for 0.1.0."
```

The scope named is **`0.1.0`**. The family is at `0.42.0`. That is not an argument for doing it — an
expired reason is not a reason to act — but it does mean the deferral is currently resting on nothing,
and should either be re-argued or discharged rather than left to accumulate.

What core does today, in `matmul_dispatch`:

```text
(1,1) vv_dot   (2,1) mv_mul   (1,2) vm_mul   (2,2) mm_mul
_     -> panic!("unsupported rank combination ...")
```

## 3. Does this belong in a "family car"? — argued, not assumed

ROADMAP §1.1 says `matten` is for education, learning, PoC and prototyping, and RFC-047 declined
ML-framework scope. Batched matmul is the operation where those two pull against each other, so the
question deserves a real answer rather than an appeal to usefulness.

**For.** A rank-3 tensor is where a learner first meets the idea that a shape can carry meaning
beyond rows and columns — a batch of matrices, a stack of images, a set of samples. Being unable to
multiply them forces the learner to loop and re-stack, which teaches the wrong lesson about what a
tensor library is for. It is also arithmetic, not machinery: no autograd, no devices, no graph.

**Against, and this is the real risk.** Batched operations are the doorway to framework scope. The
next requests are batched `transpose`, batched reductions, then broadcasting rules everywhere, then
something that looks like a framework nobody decided to build. RFC-047's line held because core has
had no batched anything.

**Recommendation: accept, with the doorway named.** §9 states that this RFC authorises batched
*matmul* and nothing else batched, and that any further batched operation needs its own RFC arguing
against this section. That is the same shape as RFC-093 §6's lock, which has since held through two
amendments.

## 4. `try_matmul` is not optional here

`matmul` and `dot` **panic** on an unsupported rank combination, and there is no Result form. That is
an anomaly: core has **41** `try_*` functions, and RFC-055 and RFC-056 built the result-form family
for exactly this reason.

It is also already anticipated. RFC-010 §167:

> If later `try_matmul` is introduced, it returns `Result`.

**Extending the panicking API alone would widen the anomaly rather than fix it.** Today a caller
passing rank-3 gets a panic naming four supported combinations. After a batch-only change, a caller
passing *mismatched batch dimensions* would get a new panic on a new axis, still with no Result form
anywhere — more panic surface, in the one core operation that lacks the escape hatch every other
operation has.

So `try_matmul` and `try_dot` ship in the same RFC. This makes the change larger and is the main
reason to reject it if you are going to.

## 5. Semantics — the decision to get right

Three options, in increasing power:

```text
(a) STRICT       leading dims must be equal
                 [b,m,n] x [b,n,p] -> [b,m,p]

(b) STRICT + SHARED RIGHT   (a), plus a rank-2 right operand applied to every batch
                 [b,m,n] x [n,p]   -> [b,m,p]

(c) FULL BROADCASTING       NumPy semantics over all leading dims
                 [1,m,n] x [b,n,p] -> [b,m,p], and every other broadcast case
```

**Recommendation: (b).** It covers the two things people actually do — a batch against a batch, and a
batch against one shared matrix — and the rule fits in a sentence: *leading dimensions must match,
except that a rank-2 right operand is reused for every batch.*

(c) is rejected for a specific reason rather than for size: full broadcasting makes shape errors
harder to explain, and RFC-020's whole diagnostics standard exists because this project treats a
confusing error as a defect. A rule a learner cannot restate is one they cannot debug.

(a) is defensible but leaves `[b,m,n] × [n,p]` — the shared-weight-matrix case — needing a manual
`repeat`, which is exactly the re-stacking §3 objects to.

## 6. Allocation, and what the error says

A batched result is `batch × m × p`. RFC-018's limits apply unchanged, and the existing
`MattenLimits` path must be used rather than a new check.

Error text follows RFC-020. The batch-mismatch message must name **which** axis disagrees and both
shapes — not "shapes are incompatible", which is the class of message RFC-020 was written against.

## 7. Scope

### In scope

```text
matmul / dot accepting rank > 2 under §5(b)
try_matmul / try_dot returning Result<Tensor, MattenError>  (§4)
rank-2 and below behaviour UNCHANGED, byte-for-byte
docs: reference/math.md, compatibility.md's deferral row, public-api-snapshot.md
```

### Out of scope — a diff touching these is a defect

```text
any other batched operation (§3, §9)
full broadcasting (§5c)
changing the four existing rank combinations' results or their panic text
matten-stats, matten-mlprep, matten-data
```

## 8. Release consequence

This is new public API, so under RFC-094 §4.2 it is a **minor**, and the first trigger to fire since
`0.42.0`. It would carry the two changes already accumulated — the grid example and the core README
playground link.

## 9. The scope lock

**This RFC authorises batched `matmul` and nothing else batched.** Batched transpose, batched
reductions, batched slicing, and broadcasting beyond §5(b) are each out of scope regardless of how
naturally they follow, and each needs its own RFC that argues against this section by name.

The reason is in §3: this is the doorway to a scope RFC-047 declined, and a doorway is easiest to
hold at the frame.

## 10. Acceptance criteria

```text
[ ] rank > 2 works per §5(b); leading dims equal, or a rank-2 right operand
[ ] rank <= 2 results and panic text are UNCHANGED — asserted, not assumed
[ ] try_matmul / try_dot return Result and never panic on shape grounds
[ ] batch-mismatch errors name the disagreeing axis and both shapes (§6)
[ ] MattenLimits enforced via the existing path, with a test at the boundary
[ ] compatibility.md's deferral row is removed, not left contradicting the code
[ ] public-api-snapshot.md lists the four new/changed entries
[ ] all eight guards pass; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] no tag, no publish; the version bump is a separate release decision (§8)
```

## 11. Non-goals

```text
autograd, devices, graphs, or anything else RFC-047 declined
einsum, tensordot, or general contraction
making matten competitive with a tensor framework — §1.1 is unchanged
```
