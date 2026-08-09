# Developer Handoff — RFC-112: `matten-mlprep` Panics on Zero Rows

**From:** High-capability model. **Date:** 2026-08-09.
**Design authority:** `rfcs/accepted/112-mlprep-axis-panic-regression.md`
**Base:** `main` @ `a316a6f`.
**Sequencing:** **after RFC-111.** The two touch disjoint files, but do them in order rather than
interleaved. **RFC-112 must land before any release carrying RFC-110.**

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. A pure addition in the handoff you can simply omit may be
> omitted and reported prominently; anything else, ask first.

---

## 1. Task title

Convert three panicking axis-reduction calls in `matten-mlprep` to their `try_` forms so a zero-row
input returns `Err` instead of panicking.

## 2. You found this; here is what it turned out to be

Your Checkpoint 2 reported it as a stale audit premise. It is more than that: **RFC-110 introduced
this panic**, and it is on `main` now. Proven with two detached worktrees —

```text
before RFC-110   standardize_columns(sliced-empty)   Err(..)
at RFC-110       standardize_columns(sliced-empty)   PANIC
```

RFC-111 is not implicated; remove it entirely and the panic remains. **Your judgment not to fix it
under RFC-111's Non-goals was right**, and pausing rather than working around it is why this is being
caught before release rather than after.

## 3. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | `standardize_columns` calls the **panicking** `x.mean_axis(0)` | `crates/matten-mlprep/src/scale.rs:44` |
| E2 | `minmax_scale_columns` calls **two** panicking forms | `scale.rs:92` (`min_axis`), `scale.rs:93` (`max_axis`) |
| E3 | Both functions already return `Result<Tensor, MattenMlprepError>` | `scale.rs:39`, and the `minmax` signature — so `?` works with no signature change |
| E4 | `MattenMlprepError::Matten(matten::MattenError)` exists | `crates/matten-mlprep/src/error.rs:41` — **no new variant needed** |
| E5 | `add_bias_column` is unaffected | probed: `Ok([0,4])` on a sliced-empty input; it performs no axis reduction |
| E6 | `train_test_split` rejects empty early and deliberately | `EmptySplit`, unrelated to this |
| E7 | `matten-report`'s call sites are not exposed | fixed non-empty demo data; workspace-excluded, `publish = false` |
| E8 | The reachable fixture | `Tensor::new(vec![1.,2.,3.,4.,5.,6.], &[2,3]).slice().range(0..0).all().build().unwrap()` → `[0,3]` |

Re-derive before editing. **Report any discrepancy first, including one that shrinks the task.**

## 4. Required implementation

```text
scale.rs:44   let means = x.mean_axis(0);
           -> let means = x.try_mean_axis(0).map_err(MattenMlprepError::Matten)?;

scale.rs:92   let mins = x.min_axis(0);
           -> let mins  = x.try_min_axis(0).map_err(MattenMlprepError::Matten)?;

scale.rs:93   let maxs = x.max_axis(0);
           -> let maxs  = x.try_max_axis(0).map_err(MattenMlprepError::Matten)?;
```

**Three call sites, not two.** `minmax_scale_columns` has both a min and a max (E2) — converting one
and not the other leaves half the panic in place, and a test that only exercises `min` would still
pass.

The following `let mins = mins.as_slice();` re-binding lines stay as they are; only the producing
expression changes.

## 5. What this RFC does NOT decide

**Do not make zero rows return `Ok(empty)`.** That is a real design question, it is deferred to the
owner, and RFC-112 §5 records it. `Err` is the correct minimal fix: it removes the panic and changes
nothing else.

If while working you form a view on the semantics, **write it in the review request** rather than
implementing it.

## 6. Required tests

```text
T1  standardize_columns on a sliced-empty tensor returns Err — NOT a panic
T2  minmax_scale_columns likewise, and the test must be able to distinguish which
    of the two call sites fired. A fixture that only ever trips `min` cannot show
    that `max` was converted — assert the error, then also convert-and-verify, or
    add a second case.
T3  the error carries core's message through MattenMlprepError::Matten
T4  NON-EMPTY behaviour byte-identical — the existing mlprep suite passes UNMODIFIED.
    Editing an existing test is a signal the change reached non-empty input.
T5  add_bias_column on the same fixture still returns Ok([0,4]) (E5) — asserted,
    since it is the neighbouring function most likely to be assumed rather than checked
T6  train_test_split unchanged (E6)
```

**Fixtures must be sliced-empty (E8)** — that is the reachable path, and it is how a user gets here
from a header-only CSV through `matten-data`.

## 7. Acceptance criteria

```text
[ ] all THREE call sites converted (§4)
[ ] zero-row input returns Err from both functions; no panic
[ ] no new error variant; MattenMlprepError::Matten used (E4)
[ ] no signature change (E3)
[ ] non-empty behaviour byte-identical; existing suite unmodified
[ ] add_bias_column and train_test_split re-asserted unchanged
[ ] core matten NOT touched — RFC-110's behaviour is correct
[ ] both feature profiles; cargo test --workspace
[ ] all eight guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
[ ] no version bump, tag, or publish
```

## 8. Risks

```text
R1  Converting two of three call sites (§4). The likeliest defect here.
R2  Implementing the deferred semantics — Ok(empty) instead of Err (§5).
R3  Changing core instead of the caller. RFC-110 is right; the caller is wrong.
R4  A constructed rather than sliced fixture — cannot exist pre-RFC-111 and is not
    the reachable path anyway.
R5  The ZeroVariance guard: with zero rows `std` is NaN, so `if std == 0.0` does not
    fire. Harmless — the output has no slots — and NOT this RFC's to change. If it
    looks wrong to you, report it.
```

## 9. Required evidence

For T1/T2, state what each produced **before** your change, so the regression is demonstrated rather
than asserted. For T4, confirm the existing suite passed unmodified rather than "should be
unaffected".

## 10. Required review-request format

Write to:
`.git-exclude/review-request/RFC-112/matten-rfc112-mlprep-axis-panic-regression-implementation-review-request-v0.1.md`

Include files changed with line counts, the §3 verification with any discrepancy, §9's evidence,
guard/clippy/test output, any view you formed on §5's deferred semantics, deviations with reasoning,
and anything you want answered at review.
