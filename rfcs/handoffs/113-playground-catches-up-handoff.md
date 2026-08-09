# Developer Handoff — RFC-113: The Playground Catches Up

**From:** High-capability model. **Date:** 2026-08-09.
**Design authority:** `rfcs/accepted/113-playground-catches-up.md`
**Base:** `main` @ `cb98985`, clean tree.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. A pure addition in the handoff you can simply omit may be
> omitted and reported prominently; anything else, ask first.

---

## 1. Task title

Replace `playground_matmul`'s hand-rolled shape validator with `try_matmul`, and test zero-sized
inputs across all four operations.

## 2. THE distinction this task turns on

The tool contains **two** reimplementations of core behaviour. **One is now avoidable and one is
not.** Deleting both, or keeping both, are each wrong.

```text
matmul     matmul_result_shape (lib.rs:266) duplicates core's rank/dimension rules
           AND its panic strings.
           -> AVOIDABLE. try_matmul has existed since RFC-099, shipped in 0.43.0.
           -> DELETE the duplicate.

broadcast  the `+` path (lib.rs:64-72) reproduces apply_binary's panic text.
           -> UNAVOIDABLE. There is NO try_add / try_broadcast on Tensor —
              verified: `grep -rn "pub fn try_add\|pub fn try_broadcast"` returns
              nothing.
           -> KEEP IT. Do not "make it consistent."
```

Adding a `try_add` to core would be a different RFC and is **out of scope**. If you think it should
exist, say so in the review request.

## 3. Why the duplication exists at all — read this before touching it

`lib.rs:259-263` records something the author verified and that constrains everything here:

> *The `wasm32-unknown-unknown` target cannot recover a panic's message — a panic inside
> `std::panic::catch_unwind`, compiled for this target, reaches the JS caller as a bare
> `RuntimeError: unreachable` trap with no payload.*

**A panic in this tool breaks the page with no error text.** That is why nothing may call a panicking
form. It is also the strongest argument for Change 1: `try_matmul` removes the only *avoidable* panic
path in the tool.

## 4. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | `playground_matmul` calls the panicking `a.matmul(&b)` | `tools/matten-playground/src/lib.rs:338` |
| E2 | Guarded by `matmul_result_shape` | `lib.rs:266`, with the comment *"Guaranteed not to panic"* |
| E3 | Three tests pin the duplicate against core's live panic | `lib.rs:407, 419, 431`, via `real_panic_message` at `:378` |
| E4 | `try_matmul`/`try_dot` exist | RFC-099, shipped `0.43.0` |
| E5 | **`lib.rs:257` claims *"`matmul`/`dot` have no `Result`-returning form"* — FALSE since RFC-099** | direct read |
| E6 | No `try_add`/`try_broadcast` exists | `grep -rn "pub fn try_add\|pub fn try_broadcast" crates/matten/src` → nothing |
| E7 | `playground_reshape` already uses `try_reshape` | `lib.rs:187` |
| E8 | `playground_axis_reduce` already uses `try_sum_axis`/`try_mean_axis` | `lib.rs:230-231` |
| E9 | Zero-sized input behaves correctly today, untested | probed: `[3,0]×[0,2] → [3,2]`, `[2,3]×[3,0] → [2,0]`, reshape/axis/broadcast all clean errors |
| E10 | The tool builds against core **by path**, `default-features = false` | `tools/matten-playground/Cargo.toml:18` — `dynamic` is OFF |

Re-derive before editing. **Report any discrepancy first, including one that shrinks the task.**

## 5. Change 1

```text
1. playground_matmul: replace the matmul_result_shape pre-check + a.matmul(&b)
   with try_matmul, rendering its error through the existing error_block.
2. DELETE matmul_result_shape entirely.
3. DELETE the three sync tests at lib.rs:407/419/431. They exist ONLY to keep the
   duplicate honest. Keeping them would pin a copy with no purpose.
4. Keep real_panic_message if the broadcast tests still use it; delete it if not.
5. E5's doc comment is FALSE. Rewrite it — the reason this function existed no
   longer holds.
```

**The rendered error text will change** — `try_matmul` returns a `MattenError`, and the hand-rolled
string was built to match a *panic payload*. Any test asserting the old text must be updated
deliberately and listed in the review request (RFC §7 R3).

## 6. Change 2 — zero-sized coverage

One test per operation, using shapes typeable into the form:

```text
matmul     [3,0] x [0,2]  -> Ok [3,2]      (n = 0, contraction)
matmul     [2,3] x [3,0]  -> Ok [2,0]      (p = 0, the RFC-108 case)
reshape    [2,3] -> [0,6] -> Err, element-count mismatch
axis_reduce [0,3] axis 0  -> Err, RFC-110's message
broadcast  [0,3] + [2,3]  -> Err, incompatible
```

These all pass today (E9). **The tests are the point, not the fix** — they make the next core change
to empty-shape behaviour fail here rather than on the page.

Add a line to `docs/src/playground.md` noting a zero-sized dimension is accepted. A reader will try it
precisely because it looks like it should fail.

## 7. Out of scope

```text
Change 3 (dynamic slicing) — RFC §6, a separate decision pending a wasm size
  measurement. `dynamic` is OFF in this tool (E10); enabling it grows the module
  every reader downloads.
the broadcast duplication (§2) — unavoidable
adding try_add to core — a different RFC
core matten or any published crate
RFC-095's grid output format
CHANGELOG.md — this tool is publish = false and ships in no release
```

## 8. Acceptance criteria

```text
[ ] playground_matmul uses try_matmul
[ ] matmul_result_shape and its three sync tests DELETED
[ ] E5's false doc comment corrected
[ ] the broadcast duplication KEPT and its doc still accurate (§2)
[ ] `grep -n '\.matmul(\|\.dot(' tools/matten-playground/src/` returns nothing
    outside tests — state the result
[ ] one zero-sized test per operation (§6)
[ ] every test asserting a changed error string listed explicitly
[ ] docs/src/playground.md mentions zero-sized shapes
[ ] the wasm module builds; cargo test for the tool passes
[ ] clippy under RUSTFLAGS="-D warnings"
[ ] core matten and every published crate untouched — assert via git diff --stat
[ ] no version bump, tag, or publish
```

## 9. Risks

```text
R1  Deleting the broadcast duplication for consistency (§2). It is load-bearing.
R2  Leaving a panicking call behind. Grep afterwards; a panic traps the page with
    no message (§3).
R3  Deleting the three sync tests but leaving real_panic_message unused — clippy
    under -D warnings will catch it, but decide deliberately.
R4  Assuming the error text is unchanged. It is not (§5).
R5  Enabling the `dynamic` feature. That is Change 3, out of scope (E10).
```

## 10. Required review-request format

Write to:
`.git-exclude/review-request/RFC-113/matten-rfc113-playground-catches-up-implementation-review-request-v0.1.md`

Include files changed with line counts, the §4 verification with any discrepancy, the before/after
error text for matmul, the list of tests whose assertions changed, the §8 grep result, build/test/
clippy output, any view on whether core should gain a `try_add`, deviations with reasoning, and
anything you want answered at review.
