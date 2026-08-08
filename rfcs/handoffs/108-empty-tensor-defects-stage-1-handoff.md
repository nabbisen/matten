# Developer Handoff — RFC-108: Two Empty-Tensor Defects, Stage 1

**From:** High-capability model. **Date:** 2026-08-08.
**Design authority:** `rfcs/accepted/108-empty-tensor-defects-stage-1.md`
**Base:** `main` @ `07af3c1`, clean tree, family at `0.44.0`.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. A pure addition in the handoff you can simply omit may be
> omitted and reported prominently; anything else, ask first.

---

## 1. Task title

Fix `mm_mul`'s zero-column panic, and add `Tensor::is_empty()`. Two independent fixes; neither
depends on the zero-sized-dimension policy question.

## 2. Why these two, and why now

One is **a live panic in published `0.44.0`** that escapes a `Result` API built specifically to
prevent that. The other is a method declined twice on a claim that is false.

Both came out of RFC-106's audit, which found them by **running** operations rather than reading
them. That is worth carrying into this task: the panic was invisible in source review.

## 3. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | `[2,3] × [3,0]` panics `"chunk size must be non-zero"` via `try_dot`, `try_matmul`, and `dot` | probed against the built library; RFC-106 §2.10, reconfirmed at review |
| E2 | The cause is `out.chunks_mut(p)` with `p == 0` | `crates/matten/src/math.rs:707` |
| E3 | `math.rs:707` is the **only** `chunks`/`chunks_mut` call site in core | `grep -rn "chunks_mut\|chunks(" crates/matten/src` |
| E4 | `n == 0` already works — `for k in 0..0` leaves `acc = 0.0` | `math.rs:710-712`; probed `[2,0] × [0,3]` → all-zero `[2,3]` |
| E5 | `m == 0` already works — `out` empty, `chunks_mut(p>0)` yields nothing | `math.rs:706-707`; probed |
| E6 | `#[allow(clippy::len_without_is_empty)]` sits at `tensor.rs:70` | direct read |
| E7 | Its justification at `tensor.rs:67-69` cites *"a future zero-sized-tensor RFC"* | direct read — RFC-106 **is** that RFC, and it has run |
| E8 | `compatibility.md`'s `is_empty()` row declines it on *"the shape model rejects zero-sized dimensions in every form"* | direct read — false; slicing reaches `[0,3]` |
| E9 | The reachable empty fixture | `Tensor::new(vec![1.,2.,3.,4.,5.,6.], &[2,3]).slice().range(0..0).all().build().unwrap()` |

Re-derive before editing. **Report any discrepancy first, including one that shrinks the task.**

## 4. Required implementation

### 4.1 `mm_mul`

Return the correct empty result rather than special-casing the panic away. The output shape is
`[m, 0]` — **not** `[0, 0]`, and **not** an error.

```text
guard p == 0 before the chunks_mut loop, returning Ok(Tensor { data: vec![], shape: vec![m, 0] })
```

**Do not restructure the loop.** `n == 0` and `m == 0` are correct today (E4, E5) and must stay
byte-identical in behaviour. A rewrite that "handles all the empty cases uniformly" is how those two
get broken.

### 4.2 `is_empty()`

```rust
pub fn is_empty(&self) -> bool { self.len() == 0 }
```

Then **remove** the `#[allow(clippy::len_without_is_empty)]` at `tensor.rs:70`. If clippy still
complains after adding the method, something is wrong with the signature — **do not re-add the allow
to silence it** (RFC §5 R4).

## 5. Required tests

```text
T1  try_dot / try_matmul / dot on [2,3] x [3,0] -> Ok, shape [m, 0], NO panic.
    Assert the shape, not merely that it returns.
T2  n == 0: [2,0] x [0,3] -> Ok, shape [2,3], all zeros. Unchanged from today.
T3  m == 0: [0,3] x [3,2] -> Ok, shape [0,2]. Unchanged from today.
T4  combinations: [0,3] x [3,0] -> Ok, shape [0,0]. Both dims zero at once.
T5  the panicking `dot` form returns for all of T1-T4 rather than panicking
T6  is_empty() true on E9's fixture; FALSE on a scalar (len()==1) and on any
    ordinary tensor
T7  every pre-existing dot/matmul test passes UNMODIFIED. Editing one is a signal
    the fix reached non-empty behaviour.
```

T2 and T3 are not redundant with T1: they are the cases that already work, and the fix must not
touch them. Assert them explicitly so a regression is visible.

## 6. Required documentation

```text
tensor.rs:67-69      the comment justifying the removed allow — delete or rewrite; it
                     cites a premise that is false and an RFC that has now run (E7)
compatibility.md     the is_empty() row: Not planned -> Supported. Its stated reason
                     is false (E8); rewrite the row, do not edit around it
is_empty()'s own doc note that a rank-0 scalar has len() == 1 and is therefore NOT empty
```

**Both sites must change together.** Fixing one and leaving the other reproduces the split that made
this invisible for as long as it was.

```text
DO NOT TOUCH: CHANGELOG.md — the release RFC writes it.
DO NOT TOUCH: checked_shape_len, the five axis reductions, Display, serde, or any
              companion crate. All are RFC-106 Stage 2/3 (RFC §4).
```

## 7. Acceptance criteria

```text
[ ] T1-T7 present and passing
[ ] output shape for p == 0 is [m, 0]; for both-zero, [0, 0]
[ ] n == 0 and m == 0 behaviour byte-identical to today
[ ] is_empty() true on a sliced-empty tensor, false on a scalar
[ ] the allow at tensor.rs:70 removed; clippy clean WITHOUT it
[ ] tensor.rs:67-69 and compatibility.md's row both corrected
[ ] no change to checked_shape_len, axis reductions, Display, serde, or companions
[ ] both feature profiles; cargo test --workspace
[ ] all eight guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] clippy clean under RUSTFLAGS="-D warnings"; cargo fmt --check
[ ] no version bump, tag, or publish
```

## 8. Risks

```text
R1  Uniform rewrite of mm_mul's empty handling breaks n==0 or m==0 (§4.1). T2/T3.
R2  Returning [0,0] or an Err for p==0. The mathematically correct answer is [m,0].
R3  Re-adding the clippy allow to silence a complaint instead of fixing the cause.
R4  Correcting one is_empty() site and not the other (§6).
R5  Scope creep into Stage 2/3. Every neighbouring empty-tensor defect the audit
    found is deliberately NOT in this RFC. If you find a new one, report it —
    do not fix it here.
```

## 9. Required evidence

For T1, give the captured result and shape — and state what it produced *before* your fix, so the
defect is demonstrated rather than asserted. For R3, confirm clippy passes with the allow removed
rather than merely absent from output.

## 10. Required review-request format

Write to:
`.git-exclude/review-request/RFC-108/matten-rfc108-empty-tensor-defects-stage-1-implementation-review-request-v0.1.md`

Include files changed with line counts, the §3 verification with any discrepancy, §9's evidence,
guard and test output, deviations with reasoning, and anything you want answered at review.
