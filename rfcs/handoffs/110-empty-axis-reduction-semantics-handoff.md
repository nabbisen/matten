# Developer Handoff — RFC-110: Empty-Axis Reduction Semantics (Stage 2)

**From:** High-capability model. **Date:** 2026-08-09.
**Design authority:** `rfcs/accepted/110-empty-axis-reduction-semantics.md`
**Base:** `main` @ `a90bdcd`, clean tree, family at `0.45.0`.
**Sequencing:** **before RFC-111 (Stage 3).** Stage 3 makes empty tensors easy to construct; these
sentinels must be gone first, or a rare defect becomes a common one.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. A pure addition in the handoff you can simply omit may be
> omitted and reported prominently; anything else, ask first.

---

## 1. Task title

Make `try_mean_axis`, `try_min_axis`, `try_max_axis`, `try_var_axis`, `try_std_axis` return `Err`
when the **reduced** axis has length zero. `sum_axis` is not touched.

## 2. This is RFC-105 again, on the siblings it excluded

RFC-105 fixed the whole-tensor forms and said explicitly *"any axis-wise reduction … RFC-106 audits
those."* The audit ran; these five were the finding. **The fix is the same shape as RFC-105's, which
was itself copied from `try_var`.** You have two working precedents in-crate; invent nothing.

## 3. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | On `[0,3]` reducing axis 0: `mean_axis` → `[NaN,NaN,NaN]`, `min_axis` → `[inf,…]`, `max_axis` → `[-inf,…]`, `var_axis`/`std_axis` → `[NaN,…]` | probed against the built library at `0.45.0` |
| E2 | `sum_axis` → `[0.0, 0.0, 0.0]` — the additive identity, **correct** | same probe |
| E3 | The five entry points | `math.rs:315` (`try_mean_axis`), `math.rs:359` (`try_min_axis`), `math.rs:402` (`try_max_axis`), `stats.rs:203` (`try_var_axis`), `stats.rs:237` (`try_std_axis`) |
| E4 | The helpers that leak the identity | `math.rs:478` `axis_reduce`, `math.rs:416` `nan_axis_reduce`, `stats.rs:35` `variance_axis_impl` |
| E5 | The reduced-axis length is `t.shape()[axis]` | `math.rs:491-498` — `axis_reduce` derives `out_shape` by removing that axis |
| E6 | RFC-105's precedent | `stats.rs` `try_var` — `Err(MattenError::InvalidArgument{ operation, argument: "self", message: "… is undefined for an empty tensor" })` |
| E7 | The reachable fixture | `Tensor::new(vec![1.,2.,3.,4.,5.,6.], &[2,3]).slice().range(0..0).all().build().unwrap()` → `[0,3]` |

Re-derive before editing. **Report any discrepancy first, including one that shrinks the task.**

## 4. Required implementation

Guard **at the five entry points** (E3), after the existing dynamic rejection, before any
computation — exactly where RFC-105 placed its guards.

```text
if self.shape()[axis] == 0 { return Err(InvalidArgument{ .. }) }
```

Word each message as RFC-105 worded its whole-tensor sibling, adapted to name the axis.

**Do not put the guard in the shared helpers** (E4). `axis_reduce` also serves `sum_axis`, which is
correct and must stay correct (RFC §6 risk 4). Guarding the helper moves `sum_axis` with them.

**Validate `axis` first.** `axis_reduce` panics on an out-of-range axis today; `self.shape()[axis]`
would panic with a raw index error instead. Preserve the existing message — check the axis bound
before reading the length, or read it in a way that cannot panic.

## 5. The distinction the whole task turns on

```text
REDUCED axis length 0     [0,3].mean_axis(0)   nothing to reduce      -> Err
SURVIVING axis length 0   [0,3].mean_axis(1)   shape [0], no work     -> Ok, empty
```

A guard written on *"is the tensor empty"* rather than *"is the reduced axis empty"* breaks the
second. Both are reachable from the same fixture; both must be tested.

## 6. Required tests

```text
T1  each of the five returns Err on a zero-length REDUCED axis
T2  each message names the operation and mirrors RFC-105's wording
T3  the panicking forms carry that message — asserted against the captured text,
    not merely "it panics"
T4  SURVIVING-axis case still Ok with an empty result, BOTH orientations:
      [0,3].mean_axis(1)  and  [3,0].mean_axis(0)
    A transposed bug is invisible if only one orientation is tested.
T5  sum_axis unchanged on every case in T1 and T4 — asserted, not assumed
T6  whole-tensor forms unchanged (RFC-105's tests pass unmodified)
T7  out-of-range axis still panics with its EXISTING message, not an index panic
```

**Every fixture must be sliced-empty (E7).** No constructor accepts a zero-sized shape; a fixture
whose reduced axis is non-zero makes every new test pass vacuously.

## 7. Required documentation

Wherever these five document their behaviour, state the empty-reduced-axis case. Mirror how RFC-105
documented the whole-tensor forms.

```text
DO NOT TOUCH: sum_axis's documented behaviour — it is unchanged
DO NOT TOUCH: CHANGELOG.md — the release RFC writes it
DO NOT TOUCH: checked_shape_len, serde, Display, the ndarray bridge — all RFC-111
```

## 8. Acceptance criteria

```text
[ ] the five return Err when the reduced axis has length 0
[ ] messages mirror RFC-105's wording; panicking forms carry them (asserted)
[ ] surviving-axis case Ok in both orientations (T4)
[ ] sum_axis unchanged everywhere (T5)
[ ] out-of-range axis keeps its existing panic message (T7)
[ ] guards at the five entry points, NOT in the shared helpers
[ ] whole-tensor forms and the existing suite unmodified
[ ] both feature profiles; cargo test --workspace
[ ] all eight guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
    — the CI form. Do not scope it to -p matten.
[ ] no version bump, tag, or publish
```

## 9. Risks

```text
R1  Guarding the shared helper -> sum_axis moves too (§4).
R2  "Is the tensor empty" instead of "is the reduced axis empty" -> breaks T4.
R3  self.shape()[axis] panicking with a raw index error on an out-of-range axis,
    replacing the existing message (§4). T7.
R4  A fixture whose reduced axis is non-zero -> every new test passes vacuously.
R5  Scope creep into Stage 3. Display, serde, the bridge and checked_shape_len are
    all RFC-111. If you find another defect, report it — do not fix it here.
```

## 10. Required review-request format

Write to:
`.git-exclude/review-request/RFC-110/matten-rfc110-empty-axis-reduction-semantics-implementation-review-request-v0.1.md`

Include files changed with line counts, the §3 verification with any discrepancy, the T3 captured
messages, guard/clippy/test output, deviations with reasoning, and anything you want answered at
review.
