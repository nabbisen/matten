# RFC-099: Result-Form `try_matmul` and `try_dot`

**Status:** Proposed
**Target:** core `matten`; a public API addition, so a minor release when it ships
**Theme:** Close the last panic-only hole in core's numeric surface — robustness, not capability
**Supersedes:** RFC-098 (in part; its batched-matmul half returns to ROADMAP §3.1)
**Related:** RFC-005, RFC-010 §167, RFC-018, RFC-020, RFC-055, RFC-056, RFC-095

---

## 1. Summary

Add `try_matmul` and `try_dot` returning `Result<Tensor, MattenError>`, and make the existing
`matmul` / `dot` delegate to them.

**No new capability.** The same rank combinations are supported, the same shapes are rejected, and
every existing message stays byte-identical. What changes is that a caller can *handle* a failure
instead of only catching a panic.

## 2. Reciprocal note — what this takes over from RFC-098

RFC-098 proposed batched matmul and, as a dependency, these Result forms. The owner asked whether
that recommendation was oriented to safety or functionality. It was functionality: the theme was
chosen because core had gained no capability since RFC-090, and the robustness fix was recruited to
support it.

Bundled that way, **declining the capability would also decline the fix** — so the fix is taken up
alone here, and batched matmul returns to §3.1 needing a positive argument.

Doing them in this order is also better for safety than the reverse, not merely tidier: if batched
matmul is ever accepted, its new shape-error surface returns `Result` from the first line, instead of
adding another panic to the one operation that lacks an escape hatch.

## 3. The gap, measured

```text
core `try_*` functions          41
core operations that panic on shape/type failure with NO Result form      dot, matmul
```

Two out of forty-three. RFC-055 and RFC-056 built the result-form family deliberately, and this pair
was left behind — not by decision but by sequence. RFC-010 §167 already wrote down the intended
shape:

> If later `try_matmul` is introduced, it returns `Result`.

This RFC does exactly that and nothing more.

## 4. The established pattern, which this follows exactly

```rust
pub fn try_sum(&self) -> Result<f64, MattenError> { reject_dynamic(self, "sum")?; … }
pub fn sum(&self) -> f64 { self.try_sum().unwrap_or_else(|e| panic!("{e}")) }
```

Logic and validation move into the `try_` form; the panicking form becomes a one-line delegation.

## 5. The one real hazard, and it is concrete

`panic!("{e}")` prints the error's `Display`. So a mechanical refactor changes the panic text unless
the error is constructed to reproduce it exactly. **For `dot` this is not hypothetical** — its
dynamic guard is bespoke and differs from the shared helper. Captured from a real panic:

```text
actual today                 matten unsupported error in dot/matmul: not supported on dynamic
                             tensors; call try_numeric() on each operand first

reject_dynamic would give    matten unsupported error in dot: dot is not supported on dynamic
                             tensors; call try_numeric() first
```

Different operation name, different message. **The existing text must be preserved**, and not merely
for compatibility — it is *better*: `dot/matmul` tells the caller both entry points share one guard,
and "on each operand first" is the correct advice for a binary operation.

So `try_dot` must construct `MattenError::Unsupported` with the bespoke operation and message. It must
**not** call `reject_dynamic`, which is the obvious move and the wrong one.

The four shape-failure panics in `matmul_dispatch` carry the same requirement.

## 6. The verifier already exists

The panic strings are already asserted live, by work landed days ago:

```text
tools/matten-playground   RFC-095 C1 added catch_unwind tests that compare the playground's
                          reproduced text against matten's REAL panic payload, every run
57_visual_shape_axis_summary   asserts its rendered output, run in CI
```

Any drift in the shape-failure messages fails those immediately. This RFC does not need a new guard
for its central hazard; it needs the implementer to know the net is there and to run it.

**The dynamic-guard message is the exception** — nothing currently asserts it, which is why §5 quotes
it verbatim from a captured panic. A test pinning it belongs in this change.

## 7. Scope

### In scope

```text
try_dot, try_matmul -> Result<Tensor, MattenError>
dot, matmul delegate via unwrap_or_else(|e| panic!("{e}"))
every existing message byte-identical, including the bespoke dynamic guard (§5)
docs: reference/math.md, public-api-snapshot.md
```

### Out of scope — a diff touching these is a defect

```text
batched matmul, or ANY change to which rank combinations are supported
new error variants — MattenError is #[non_exhaustive] but needs nothing new here
try_ forms for other operations; the other 41 already exist
performance work of any kind
```

## 8. Why this is low risk, stated so it can be checked rather than believed

```text
no new semantics       the same inputs succeed and fail as before
no new error variants  existing Shape / Unsupported carry it
no allocation change   RFC-018 limits are untouched
additive API           two new functions; nothing removed or renamed
already-specified      RFC-010 §167 fixed the shape years ago
already-verified       §6's live tests cover the main hazard
```

The residual risk is precisely one thing: **changing a message while believing you have not.** §5 and
§6 exist for it.

## 9. Acceptance criteria

```text
[ ] try_dot / try_matmul return Result and never panic on shape or dynamic grounds
[ ] dot / matmul delegate, and their panic text is BYTE-IDENTICAL — asserted against
    captured strings, not eyeballed
[ ] the bespoke dynamic-guard message is preserved and now pinned by a test (§5, §6)
[ ] reject_dynamic is NOT used for dot/matmul (§5)
[ ] no change to which rank combinations succeed
[ ] the playground's live panic tests and example 57 still pass unmodified
[ ] public-api-snapshot.md lists both new functions
[ ] all eight guards pass; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] no tag, no publish; the version bump is a separate decision under RFC-094
```

## 10. Non-goals

```text
batched matmul — ROADMAP §3.1, needing a positive argument (RFC-098)
changing any existing message "while we are in there"
deprecating the panicking forms; they stay, as RFC-055's do
```
