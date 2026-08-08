# RFC-108: Two Empty-Tensor Defects — Stage 1

**Status:** **Implemented** 2026-08-08 in commit *"Fix mm_mul zero-column panic and add
Tensor::is_empty() (RFC-108)"*, reviewed and approved **after corrections** — this RFC's own §4 scope
was unsatisfiable and is amended inline below. Unreleased; the release decision is open under RFC-094,
and the panic this fixes is live in published `0.44.0`. Handoff:
`rfcs/handoffs/108-empty-tensor-defects-stage-1-handoff.md`.
**Target:** core `matten`; one bug fix plus one additive method — a minor release when it ships
**Theme:** Fix what is wrong today, independent of the zero-sized policy decision
**Related:** RFC-003 §7.4, RFC-099, RFC-102, RFC-105, RFC-106

---

## 1. Summary

Two fixes surfaced by RFC-106's audit. **Neither depends on the zero-sized-dimension policy
decision** — both are correct under accept, prevent, and document alike, and one is a live panic in
published `0.44.0`.

```text
1. mm_mul panics on a zero-column product, escaping try_dot's Result
2. is_empty() is absent, declined on a premise that is false
```

## 2. Defect 1 — `try_dot` panics, and its `Result` cannot express it

```text
[2,3] × [3,0]   try_dot     PANIC: "chunk size must be non-zero"
                try_matmul  PANIC: same
                dot         PANIC: same
```

Reproduced against the built library, twice independently (RFC-106 §2.10; confirmed at review with a
corrected fixture). **This is a raw `slice::chunks_mut` panic, not a `MattenError`.** RFC-099 added
`try_dot` so *"a caller can handle a failure instead of only catching a panic"* — this reaches a
caller who did exactly that.

### 2.1 The cause is one line

```rust
// math.rs:706-707
let mut out = vec![0.0f64; m * p];
for (i, row) in out.chunks_mut(p).enumerate() {   // panics for ANY slice when p == 0
```

`chunks_mut(0)` panics regardless of slice length. The neighbouring cases are already right:

```text
n == 0  (contraction dim)  -> `for k in 0..0` never runs, acc stays 0.0 -> correct all-zero result
m == 0  (no rows)          -> out is empty, chunks_mut(p>0) yields nothing -> correct empty result
p == 0  (no columns)       -> PANIC
```

**`math.rs:707` is the only `chunks`/`chunks_mut` call site in core**, so the blast radius is exactly
this one line.

### 2.2 Why this is mechanical, not a design question

A matrix product with zero output columns has one answer: an empty `[m, 0]`. Nothing is undecided,
and `mm_mul` already computes `n == 0` correctly by the same reasoning.

### 2.3 It is not fixed by any policy option

The crash has **no relationship to `checked_shape_len`**. Relaxing the constructor guard leaves it
exactly as it is. It needs its own fix regardless of what RFC-106's larger question resolves to —
which is why it is here and not there.

## 3. Defect 2 — `is_empty()` is absent on a false premise

Two sites decline it, both citing the same claim:

```text
compatibility.md   "Not planned" — "the shape model rejects zero-sized dimensions in
                   every form (`[0]`, `[0, 3]` ...)"
tensor.rs:67-70    #[allow(clippy::len_without_is_empty)] — "zero-sized dims are rejected
                   and a scalar has len()==1, so it would always be false. Deferred to a
                   future zero-sized-tensor RFC."
```

**The premise is false.** `len() == 0` is reachable today, via the same slice every RFC-105 and
RFC-106 fixture uses. And `tensor.rs:67` names *"a future zero-sized-tensor RFC"* — RFC-106 is that
RFC, and it has now run.

The implementation needs no judgment call under any policy option:

```rust
pub fn is_empty(&self) -> bool { self.len() == 0 }
```

Leaving the current text standing is the *records that lie* anti-pattern RFC-000 names. **Both sites
must be corrected together** — fixing one and leaving the other reproduces the exact split that made
this hard to see.

## 4. Scope

### In scope

```text
math.rs mm_mul: guard p == 0, returning the empty [m, 0] result
Tensor::is_empty(), and removing the now-unnecessary allow at tensor.rs:70
tensor.rs:67's comment, and compatibility.md's is_empty() row
tests per §6
```

### Out of scope — a diff touching these is a defect

```text
checked_shape_len, or whether zero-sized dimensions become constructible (RFC-106 Stage 3)
the five axis reductions (RFC-106 Stage 2)
Display on an empty tensor (RFC-106 §2.14)
the serde round-trip (RFC-106 Finding A — resolves under Stage 3, not here)
matten-ndarray's ZeroSizedAxis boundary (Stage 3)

AMENDED 2026-08-08 at review: this list previously forbade "any companion-crate
change". That was UNSATISFIABLE. Adding Tensor::is_empty() makes clippy::len_zero
fire on every `tensor.len() == 0` in the workspace, and matten-stats has two
(histogram.rs:73, quantile.rs:40) — so the CI gate
`cargo clippy --workspace --all-targets --all-features -- -D warnings` FAILS
unless they change. Verified by toggling: 0 errors without this RFC's diff, 4 with.
Those two lines become `x.is_empty()` and are IN scope. Nothing else in any
companion crate changes. The scope error was mine, not the implementer's.
CHANGELOG.md — the release RFC writes it
```

## 5. Risks

```text
R1  FIXING THE SYMPTOM. Special-casing chunks_mut is not the fix; returning the
    correct empty result is. The output shape must still be [m, 0], not [0, 0]
    or an error.
R2  m == 0 AND n == 0 MUST STAY CORRECT. Both work today. A guard written for
    p == 0 that also changes them is a regression — test all three separately,
    plus combinations.
R3  is_empty() ON A SCALAR. A rank-0 tensor has len() == 1, so is_empty() is
    false. That is correct and is the one case the old comment got right; keep
    it true and test it.
R4  REMOVING THE ALLOW. Once is_empty() exists, clippy's len_without_is_empty is
    satisfied and the allow at tensor.rs:70 should go. If clippy still complains,
    something is wrong with the signature — do not re-add the allow to silence it.
```

## 6. Acceptance criteria

```text
[x] try_dot / try_matmul / dot on [m,n] x [n,0] return Ok with shape [m, 0], no panic
[x] n == 0 and m == 0 still behave exactly as today — asserted separately (R2)
[x] the panicking `dot` form also returns, rather than panicking
[x] is_empty() returns true for a sliced-empty tensor, false for a scalar (R3)
[x] the allow at tensor.rs:70 is removed and clippy passes without it (R4)
[x] tensor.rs:67's comment and compatibility.md's is_empty() row both corrected
[x] no change to checked_shape_len, the axis reductions, or any other operation
[x] both feature profiles; cargo test --workspace
[x] all eight guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[x] clippy clean under RUSTFLAGS="-D warnings"; cargo fmt --check
[x] no version bump, tag, or publish
```

## 7. Release note

`is_empty()` is **Added**; the `mm_mul` fix is **Changed** (a panic becomes a result). Recorded here
for the release RFC; not written into `CHANGELOG.md` by this work.

`0.44.0` is published and carries the panic. Whether that warrants an expedited release is the
owner's call under RFC-094 — this RFC does not assume one.

## 8. Non-goals

```text
the zero-sized-dimension policy (RFC-106 Stage 3)
axis reductions (Stage 2)
any companion-crate change
```
