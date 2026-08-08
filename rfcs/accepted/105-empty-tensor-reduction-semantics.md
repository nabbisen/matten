# RFC-105: Empty-Tensor Reduction Semantics

**Status:** **Accepted** 2026-08-08 by the owner. Not yet implemented. Handoff:
`rfcs/handoffs/105-empty-tensor-reduction-semantics-handoff.md`.
**Target:** core `matten`; behaviour change to existing APIs, so a minor release when it ships
**Theme:** Make reductions correct on empty tensors, which are reachable today
**Related:** RFC-003 §7.4, RFC-055, RFC-099, RFC-106

---

## 1. Summary

Five reductions misbehave on empty tensors: two panic with a raw index error, three return sentinel
values. Empty tensors are reachable today. Fix them to match the two that are already correct.

**This RFC decides nothing about the shape model.** Whether zero-sized dimensions should be
constructible is RFC-106's question. These defects are wrong under every answer to it.

## 2. The defects, measured

An empty tensor is reachable without any constructor accepting one:

```rust
Tensor::new(vec![1.,2.,3.,4.,5.,6.], &[2,3]).slice().range(0..0).all().build().unwrap()
// -> shape [0, 3], len 0
```

Measured against the built library:

```text
try_argmin   PANIC: index out of bounds: the len is 0 but the index is 0    <- WORST
try_argmax   PANIC: index out of bounds: the len is 0 but the index is 0
try_mean     Ok(NaN)
try_min      Ok(inf)
try_max      Ok(-inf)
try_sum      Ok(-0.0)                                                        <- see §5
try_var      Err("variance is undefined for an empty tensor")                <- ALREADY CORRECT
try_std      Err("standard deviation is undefined for an empty tensor")      <- ALREADY CORRECT
```

The `argmin`/`argmax` panic is a **raw Rust slice panic**, not a matten error. That is an unhandled
edge case, not a designed rejection — and it defeats the `try_` form, which exists precisely so a
caller need not fear a panic.

`min` returning `inf` and `max` returning `-inf` is worse than a panic in one respect: **it is
silent**. A caller computing a range over a filtered-empty selection gets `inf`/`-inf` and no signal.

## 3. The fix is already written, twice

`try_var` (`stats.rs:118-128`) and `try_std` do exactly the right thing:

```rust
if self.data.is_empty() {
    return Err(MattenError::InvalidArgument {
        operation: "var",
        argument: "self",
        message: "variance is undefined for an empty tensor".to_string(),
    });
}
```

Apply the same shape to `try_mean`, `try_min`, `try_max`, `try_argmin`, `try_argmax`. The panicking
forms already delegate via `unwrap_or_else(|e| panic!("{e}"))`, so they inherit the message and start
panicking with a sentence instead of an index error.

**No new error variant, no new pattern, no API signature change.**

## 4. Why these five and not `sum`

```text
sum   of an empty set = 0.0, the additive identity. Mathematically defined,
      and what ndarray returns. NOT a defect. Keep it.

mean  = sum/n, undefined at n = 0. NaN is a floating-point artefact, not an answer.
min   } undefined on an empty set. inf/-inf are fold sentinels leaking out; they are
max   } the identity elements of min/max, which is exactly why they are wrong to return.
arg*  } there is no index to return. Currently indexes into an empty slice.
```

This is the same boundary `var`/`std` already drew.

## 5. One thing to verify, not a claim

`try_sum` returns **`-0.0`**, where Rust's `[].iter().sum::<f64>()` gives `0.0`. Observed, not
explained. It is harmless (`-0.0 == 0.0` compares true) and it is **not** part of §4's fix.

**Determine why before changing anything**, and if it is incidental, normalise to `0.0`. If it turns
out to be load-bearing somewhere, leave it and say so. Do not "fix" it blind.

## 6. Scope

### In scope

```text
try_mean, try_min, try_max, try_argmin, try_argmax: Err on empty, mirroring try_var
tests: each of the five, asserted on a SLICED-EMPTY tensor (the reachable path)
the panicking forms: assert they now carry the sentence, not an index panic
docs: the three FALSE claims listed in §7
§5's -0.0, investigated and reported
```

### Out of scope — a diff touching these is a defect

```text
whether zero-sized dimensions should be CONSTRUCTIBLE — that is RFC-106
sum's 0.0 result (§4)
try_var / try_std, which are already correct
any axis-wise reduction (var_axis, std_axis, sum_axis) — RFC-106 audits those
the ndarray bridge, matten-stats, matten-data
```

## 7. Documentation that is false today

Each of these justifies code by an invariant that does not hold, and each must be corrected:

```text
math.rs:~74      mean's doc: "Behaviour on an empty tensor is unspecified (zero-sized
                 dims are rejected by constructors in Phase 1)" -> they are rejected by
                 CONSTRUCTORS, but slicing produces one anyway.
selection.rs:17  "Callers guarantee `data` is non-empty (core rejects zero-sized
                 dimensions)" -> this comment is why argmin panics.
stats.rs:112     "zero-sized dimensions, so an empty tensor is not constructible in
                 practice" -> false, and the code it justifies returns inf.
```

`compatibility.md`'s `is_empty()` row also rests on *"the shape model rejects zero-sized dimensions in
every form"*. **Leave that row to RFC-106** — it is about the shape model, not about reductions.

## 8. Risks

```text
1. THE TESTS MUST USE A SLICED-EMPTY TENSOR. A test that constructs one directly
   cannot exist — constructors reject zero-sized dims. Building the fixture through
   slice().range(0..0) is the only reachable path, and it is the point.
2. BEHAVIOUR CHANGE, NOT A FIX, FOR CALLERS. Anyone relying on min() == inf for
   empty input gets an error instead. That is intended; it belongs in the CHANGELOG
   under Changed when released.
3. THE PANICKING FORMS MUST STILL PANIC. Do not convert min() to return Result —
   the pair convention (RFC-055/RFC-099) stands. Only the MESSAGE changes.
```

## 9. Acceptance criteria

```text
[ ] try_mean/try_min/try_max/try_argmin/try_argmax return Err on an empty tensor
[ ] each message names the operation and says the result is undefined for an empty
    tensor, mirroring try_var's wording
[ ] argmin()/argmax() panic with that sentence, NOT "index out of bounds" — asserted
    against the captured message
[ ] every fixture is a SLICED-EMPTY tensor, not a constructed one (risk 1)
[ ] try_sum still returns a zero; §5 investigated and reported either way
[ ] try_var/try_std unchanged — byte-identical
[ ] no signature change; the panicking/try_ pairs are preserved
[ ] all eight guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] §7's three code comments corrected; compatibility.md NOT touched (RFC-106 owns it)
[ ] no version bump, tag, or publish
```

## 10. Non-goals

```text
the zero-sized shape-model decision (RFC-106)
axis-wise reductions, the bridge, and the companion crates (RFC-106's audit)
is_empty() (RFC-106)
```
