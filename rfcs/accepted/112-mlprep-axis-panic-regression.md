# RFC-112: `matten-mlprep` Panics on Zero Rows — an RFC-110 Regression

**Status:** **Accepted** 2026-08-09 by the owner. Not yet implemented. **Sequenced after RFC-111**,
which is in flight; the two touch disjoint files. Handoff:
`rfcs/handoffs/112-mlprep-axis-panic-regression-handoff.md`.
**Target:** `matten-mlprep`; a defect fix — minor release when it ships
**Theme:** Stop a published function panicking on a reachable input
**Related:** RFC-099, RFC-105, RFC-108, RFC-110, RFC-111

---

## 1. Summary

`standardize_columns` and `minmax_scale_columns` **panic** on a tensor with zero rows. Use the `try_`
forms and return an error instead.

**This is a regression introduced by RFC-110**, which I authored, reviewed, and approved with no
corrections. It is on `main` now and unreleased.

## 2. Release blocker

```text
RFC-112 MUST land before any release carrying RFC-110.
```

RFC-110 is unreleased. That is the only reason this is a near-miss rather than a repeat of `0.44.0`,
which shipped `try_dot`'s panic. Releasing RFC-110 without this would ship a regression introduced
while fixing a different one.

## 3. Measured, in two worktrees

```text
before RFC-110 (f055bc1~2)
  standardize_columns(sliced-empty)   Err("matten rejected the result: zero-sized
                                          dimensions are not supported")

at RFC-110 (2b90592)
  standardize_columns(sliced-empty)   PANIC "mean is undefined for a reduced axis
                                             of length 0 (axis 0)"
  minmax_scale_columns(sliced-empty)  PANIC "minimum is undefined for a reduced
                                             axis of length 0 (axis 0)"
```

The fixture is built by **slicing**, which has produced empty tensors since RFC-008. **RFC-111 is not
implicated** — remove it entirely and the panic remains.

### 3.1 How it happens

RFC-110 changed `mean_axis`/`min_axis`/`max_axis` to error on a zero-length reduced axis instead of
leaking `NaN`/`inf`. Correct for a direct caller. But `matten-mlprep` calls the **panicking** forms:

```text
crates/matten-mlprep/src/scale.rs:44   x.mean_axis(0)    standardize_columns
crates/matten-mlprep/src/scale.rs:92   x.min_axis(0)     minmax_scale_columns
crates/matten-mlprep/src/scale.rs:93   x.max_axis(0)
```

Before RFC-110 the sentinel made that safe. After it, the panic reaches a published API.

### 3.2 Reachable from ordinary use

A CSV with a header row and zero data rows flows `matten-data` → `to_tensor()` → either scaling
function. No slicing required by the user.

## 4. Scope

### In scope

```text
crates/matten-mlprep/src/scale.rs   the three call sites -> try_ forms
tests: zero-row input returns Err, does NOT panic
```

### Out of scope — a diff touching these is a defect

```text
whether standardize_columns SHOULD return Ok(empty) on zero rows (§5)
core matten — RFC-110's behaviour is correct and stays
add_bias_column — verified unaffected, Ok([0,4]), no axis reduction
train_test_split — rejects empty early and deliberately (EmptySplit)
matten-report — fixed non-empty demo data, workspace-excluded, publish = false
RFC-111's changes
CHANGELOG.md — the release RFC writes it
```

## 5. The fix is mechanical; the semantics are not, and are deferred

```text
MECHANICAL, this RFC
  x.mean_axis(0)  ->  x.try_mean_axis(0).map_err(MattenMlprepError::Matten)?
  same for min_axis / max_axis
  MattenMlprepError::Matten already exists (error.rs:41) — no new variant.
  Panic -> Err. That is the whole regression.

DESIGN, deferred
  Should zero rows yield Ok(empty) instead of Err? Defensible; RFC-106's original
  instinct. Not what makes this urgent, and it needs an owner decision.
```

**A published function that panics on a reachable input is a defect under either answer.** That is
the argument RFC-108 made for `mm_mul` and it holds unchanged.

## 6. Risks

```text
R1  "Fixing" the semantics while here — returning Ok(empty) — is the deferred
    decision (§5), not this RFC's. Err is the correct minimal fix.
R2  Changing core instead. RFC-110's behaviour is right; the caller is wrong.
R3  Missing that minmax_scale_columns has TWO call sites (min and max), not one.
R4  The ZeroVariance guard. With zero rows, `std` is NaN and `if std == 0.0` does
    not fire — harmless today because the output has no slots, but do not "fix"
    it here; report it if it looks wrong.
```

## 7. Acceptance criteria

```text
[ ] standardize_columns on a zero-row tensor returns Err, does NOT panic
[ ] minmax_scale_columns likewise — both call sites converted
[ ] the error carries core's message via MattenMlprepError::Matten
[ ] non-empty behaviour byte-identical; the existing mlprep suite passes unmodified
[ ] add_bias_column and train_test_split unchanged and re-asserted
[ ] fixtures are SLICED-empty — the reachable path
[ ] both feature profiles; cargo test --workspace
[ ] all eight guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
[ ] no version bump, tag, or publish
```

## 8. Release note

**Changed** — a panic becomes an error. Recorded for the release RFC; not written into
`CHANGELOG.md` here. **The release RFC must state that RFC-110 and RFC-112 ship together**, since
RFC-110 alone is a regression.

## 9. Non-goals

```text
the zero-rows semantics decision (§5)
any core change
RFC-111, which is independent and in progress
```
