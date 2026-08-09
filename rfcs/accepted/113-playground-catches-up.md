# RFC-113: The Playground Catches Up

**Status:** **Accepted** 2026-08-09 by the owner. Not yet implemented. Handoff:
`rfcs/handoffs/113-playground-catches-up-handoff.md`.
**Target:** `tools/matten-playground` (workspace-excluded, `publish = false`) + the docs page
**Theme:** Remove a duplicated rule, and cover inputs the library newly accepts
**Related:** RFC-093, RFC-095, RFC-099, RFC-102, RFC-108, RFC-110, RFC-111

---

## 1. Summary

Three changes, in order of value:

```text
1. playground_matmul calls the PANICKING matmul behind a hand-rolled copy of core's
   shape rules. Call try_matmul and delete the copy.
2. Zero-sized shapes are typeable into the form for the first time (RFC-111). Cover
   them across all four operations.
3. Optionally add dynamic slicing (RFC-102) as a fifth operation — with a real cost
   (§6) that makes it a separate decision.
```

## 2. Why now

The playground depends on core **by path**, not by version, so it already tracks development with
nothing to bump. It has been absorbing behaviour changes silently:

```text
axis_reduce [0,3] axis 0   ->  "mean is undefined for a reduced axis of length 0 (axis 0)"
```

That is RFC-110's error, live in the playground, with nobody having touched the tool.

And **RFC-111 made zero-sized shapes typeable.** A user can now enter `3,0` in the form. Measured
against the current tree, all four operations handle it without panicking — but that is luck earned
by RFC-108, not by anything in the playground.

## 3. Change 1 — delete the duplicated rule

`tools/matten-playground/src/lib.rs:338` calls the **panicking** `a.matmul(&b)`, guarded by
`matmul_result_shape` (`:266`) with the comment *"Guaranteed not to panic: the same rank/dimension
rules were just checked."*

That guard is a reimplementation of core's matmul shape rules **and its error strings**, down to
hardcoding `const OP: &str = "dot"` because *"matmul() delegates to dot()"*.

**A panic in wasm aborts the module** rather than showing an error, so the guard is load-bearing. It
is kept honest by three tests that capture core's live panic payload via `real_panic_message(|| …)`
and compare — which is good practice, and also an admission that the duplication is a standing sync
risk.

**`try_matmul` has existed since RFC-099**, shipped in `0.43.0`. Call it, render its error, and
delete `matmul_result_shape` and its three sync tests entirely. Less code, no divergence risk, and
nothing can panic.

### 3.1 Check the same pattern elsewhere

`playground_axis_reduce` already uses `try_sum_axis`/`try_mean_axis`. Confirm `playground_reshape`
and `playground_broadcast` do the same, and convert any that do not. **The rule for this tool is that
it never calls a panicking form.**

## 4. Change 2 — cover zero-sized inputs

Current behaviour, measured:

```text
matmul  [3,0] x [0,2]   Ok, shape [3,2]      (n = 0 contraction)
matmul  [2,3] x [3,0]   Ok, shape [2,0]      (p = 0 — the RFC-108 case)
reshape [2,3] -> [0,6]  Err, element-count mismatch — correct
axisred [0,3] axis 0    Err, RFC-110's message — correct
broadcast [0,3]+[2,3]   Err, incompatible — correct
```

Nothing is wrong. **Nothing is tested either.** Add a case per operation so the next core change to
empty-shape behaviour surfaces here rather than in the page.

The page's own help text should say a zero-sized dimension is now accepted — it is the kind of thing
a reader will try precisely because it looks like it should fail.

## 5. Scope

### In scope

```text
tools/matten-playground/src/lib.rs   Change 1, Change 2's tests
docs/src/playground.md               a line on zero-sized shapes
```

### Out of scope — a diff touching these is a defect

```text
core matten, or any published crate
the four operations' output FORMAT — RFC-095's grid contract is unchanged
book.toml, the workflow, or the wasm build
CHANGELOG.md — the playground is publish = false and ships in no release
```

## 6. Change 3 — dynamic slicing, and the cost that makes it separate

RFC-102's dynamic slicing is the best remaining fit for this tool: it is about **shape and
structure**, which is the playground's whole remit, and it benefits from being poked at.

**But it is not free.** The playground currently declares:

```text
matten = { path = "../../crates/matten", default-features = false }
```

`dynamic` is **off**. Adding this operation means enabling it, which pulls the `Element` machinery
into the wasm module that every reader of the page downloads. That is a real cost for one demo, and
it is a different kind of decision from Changes 1 and 2.

**Recommendation: land Changes 1 and 2 first; decide Change 3 separately once the wasm size delta is
measured.** Do not bundle a payload increase with a duplication removal.

### 6.1 What it would not change

RFC-093 §6's scope lock is untouched by any of this. Slicing shows *which positions were selected* —
representation, not visualization. No value is encoded as magnitude.

## 7. Risks

```text
R1  Deleting matmul_result_shape but leaving a caller of the panicking matmul.
    Grep for `.matmul(`/`.dot(` in the tool afterwards; expect zero.
R2  The three sync tests are deleted, not merely bypassed. They exist ONLY to keep
    the duplicate honest; keeping them alongside try_matmul would pin a copy that
    no longer has a purpose.
R3  try_matmul's error string may differ from the hand-rolled one. That is the
    point — but the page's output changes, so any test asserting the old text must
    be updated deliberately and listed.
R4  Change 3's wasm size. Measure before deciding, not after.
```

## 8. Acceptance criteria

```text
[ ] playground_matmul uses try_matmul; matmul_result_shape and its three sync
    tests are DELETED
[ ] no panicking core form is called anywhere in the tool — grep, and state it
[ ] all four operations have a zero-sized-input test
[ ] any test asserting a changed error string is listed explicitly
[ ] docs/src/playground.md mentions zero-sized shapes
[ ] the wasm module still builds; the page still works
[ ] cargo test for the tool; clippy under RUSTFLAGS="-D warnings"
[ ] core matten and every published crate untouched
[ ] no version bump, tag, or publish — this tool ships in no release
```

## 9. Non-goals

```text
Change 3 (§6) — a separate decision after the size measurement
mutation (RFC-104) — the playground is stateless one-shot; mutation needs
  before/after state to mean anything
is_empty() — too small for its own form; it belongs in whatever an empty
  result already prints
any change to RFC-095's grid rendering contract
```
