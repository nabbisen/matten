# RFC-129: Result-Zone Arithmetic

**Status:** **Implemented** 2026-09-03 in commit *"Add try_add/sub/mul/div (RFC-129) and apply the
boundary-only limit model (RFC-132)"* (`46fb4b6`), landed together with RFC-132 Stage 2 as §13
required, reviewed and approved after one required correction (in RFC-132's half). **Unreleased** —
rides `0.47.0`. Handoff: `rfcs/handoffs/129-result-zone-arithmetic-handoff.md`.
**Target:** `crates/matten/src/ops/`, `docs/src/reference/`, `docs/src/reference/compatibility.md`
**Theme:** Give the four operators most likely to exceed the budget a recoverable twin
**Related:** RFC-094 §4.1 (why this cannot ride a patch), RFC-127, RFC-018, RFC-001 §10

---

> **AMENDED by RFC-132's Stage-1 decision, 2026-09-01.** The owner chose the **boundary-only** limit
> model: limits bound allocations sized by a caller-supplied value, **not** allocations sized by data
> already in memory. **Arithmetic is the second kind, so the budget no longer applies to it.**
>
> ```text
> WAS  "a [2000,1000] + [2000,1000] case returns Err rather than panicking"
> NOW  that case returns Ok — there is no budget check on arithmetic
> ```
>
> **This RFC still stands.** `try_add`'s durable jobs are **broadcast incompatibility** and **dynamic
> tensors**; only its budget justification is superseded. §2's motivating example, E3, §9's fourth
> criterion and §10 are corrected below and marked.
>
> **It now lands together with RFC-132 Stage 2 in `0.47.0`.** Landing it alone would ship a test
> asserting behaviour RFC-132 then removes.

## 1. Summary

```text
Add try_add / try_sub / try_mul / try_div, returning Result.
Keep Add/Sub/Mul/Div delegating to them and panicking on Err — the exact
clip/try_clip pattern already in the crate.
```

**This is new public API, so it is a MINOR** — `0.47.0`. RFC-094 §4.1 excludes new public API from a
patch, which is the only reason it did not ride RFC-127's `0.46.2`.

## 2. The gap, and it is the most likely one to be hit

RFC-001 §10 draws the crate's central contract: a panic zone for convenience, a Result zone for
recoverable failure, and **a `try_` twin for nearly every convenience API**. The four arithmetic
operators are the exception, and they are the operations most likely to need one.

```text
default max_elements = 1_048_576      a 1024x1024 matrix
a [2000,1000] tensor:
    constructs fine
    .abs() works
    .sum_axis(0) works
    try_concatenate correctly returns Err
    &big + &big   ->  PANIC, and there is no try_add anywhere in the API
```

> **Superseded in part by RFC-132.** Under the boundary-only model this panic is *removed* rather than
> converted to an `Err` — `&big + &big` simply works. What survives as this RFC's justification is the
> rest of the Result-zone gap: **broadcast incompatibility and dynamic tensors still need a
> recoverable twin**, and today they have none.

**The external audit rates this the highest-likelihood finding in its report** — above the Critical —
because it needs no hostile input at all. An ordinary user with ordinary data reaches it.

> Because `Add`/`Sub`/`Mul`/`Div` are `std::ops` traits, they **cannot** return `Result`. The operator
> must panic; that is not the defect. The defect is that **no recoverable alternative exists**, so a
> caller who wants to handle the failure has nowhere to go — in the one library whose stated
> proposition is legible, recoverable failure.

## 3. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | No `try_add`/`try_sub`/`try_mul`/`try_div` anywhere | `grep 'pub fn try_add\|try_sub\|try_mul\|try_div'` across `crates/matten/src` → nothing |
| E2 | The operators panic through `BroadcastCtx::new` | `crates/matten/src/ops/broadcast.rs:85-93` |
| E3 | The default element budget is 1 048 576 | `crates/matten/src/limits.rs` |
| E4 | `clip`/`try_clip` is the established twin pattern in this crate | `crates/matten/src/elementwise.rs:100-130` |
| E5 | Nearly every other convenience API already has a `try_` twin | `compatibility.md`; `dot`/`try_dot`, `matmul`/`try_matmul`, `trace`/`try_trace`, `outer`/`try_outer` |
| E6 | RFC-115 recorded this gap and did not act on it | RFC-115 — *"no `try_add`/`try_broadcast` exists"* |

E6 is worth stating plainly: **this project noticed the gap once already and moved on.** The external
audit found it independently and rated it High.

## 4. The shape of the change

Follow `clip`/`try_clip` exactly (E4) — do not invent a second pattern.

```text
try_add(&self, other: &Tensor) -> Result<Tensor, MattenError>     and sub/mul/div
impl Add for &Tensor  ->  self.try_add(other).unwrap_or_else(|e| panic!(...))
```

```text
THE OPERATOR'S PANIC MESSAGE MUST NOT CHANGE.
Its exact text is asserted by existing tests and quoted in the book. Preserving
it is what makes this purely additive. Verify by running the existing tests
UNMODIFIED — editing one is a signal the message moved.
```

**Errors, not new variants.** Broadcast incompatibility is `MattenError::Broadcast`; budget excess is
`MattenError::Allocation`. Both exist. **Do not add an error variant** — `MattenError` is
`#[non_exhaustive]`, so adding one is not breaking, but a new variant for an existing condition
fragments the error model this crate is praised for.

### 4.1 Dynamic tensors

Every numeric entry point guards dynamic storage. `try_*` must too, returning
`MattenError::Unsupported` — matching `try_dot`/`try_matmul`, not panicking.

## 5. Documentation

```text
compatibility.md          a row per new method, Supported, citing this RFC
the arithmetic reference  state that the operators panic and name the try_ twin
public-api-snapshot.md    FOUR new public items — this is the RFC-109 case, not
                          the RFC-103 case. Omitting them IS the defect here.
```

**Note the inversion from RFC-121**, which required *no* snapshot row because nothing public changed.
Here four items are added, so the row is required. Carrying RFC-121's instruction forward would be
the error.

## 6. Why this cannot ride a patch

```text
RFC-094 §4.1  excluded: any new public API
```

Unambiguous. `0.46.2` (RFC-127) is a patch and cannot carry this. **The consequence is that the
highest-likelihood finding in the audit ships second**, and that ordering is release mechanics, not a
judgement about risk.

**The lever that changes it is authorizing `0.47.0` sooner** — RFC-094 §4.2(c), the owner asks. This
RFC does not assume that authorization.

### 6.1 Running in parallel with RFC-127

The two may proceed together — their targets are disjoint. **One trap:** the operators live in
`crates/matten/src/ops/tensor_ops.rs`, while one of RFC-127's unguarded allocation sites is
`crates/matten/src/tensor/ops.rs`. **Two different files with near-identical paths.** Confirm which
you are editing before you edit it; the audit's own text uses both spellings.

## 7. Scope

### Out of scope — a diff touching these is a defect

```text
changing any operator's panic MESSAGE          §4
a new MattenError variant                      §4
try_broadcast, or a public broadcast API       larger; not proposed here
threading MattenLimits through arithmetic      the audit's long-term item; today
                                               a caller cannot raise the budget
                                               for these ops at all, and that is
                                               a separate design question
RFC-127's fixes                                ship first, as 0.46.2
the version bump                               the 0.47.0 release RFC owns it
```

## 8. Risks

```text
R1  Changing the operators' panic text. Breaks asserted tests and the book (§4).
R2  Adding an error variant for a condition MattenError already covers (§4).
R3  Forgetting the dynamic guard, so try_add panics where try_dot returns
    Unsupported (§4.1).
R4  Omitting the public-api-snapshot rows — RFC-121's instruction carried
    forward where RFC-109's applies (§5).
R5  Implementing try_* by duplicating the broadcast logic rather than inverting
    it — the operator should delegate to try_, not the reverse.
R6  Shipping this in a patch (§6).
```

## 9. Acceptance criteria

```text
[ ] try_add / try_sub / try_mul / try_div exist, returning Result
[ ] each returns Err where the operator panics — same condition, same message text
    in the Err's Display where applicable
[ ] the operators delegate to them; every pre-existing arithmetic test passes
    UNMODIFIED
[ ] a [2000,1000] + [2000,1000] case returns Ok  (AMENDED by RFC-132 — was
    "returns Err rather than panicking"; the budget no longer applies here)
[ ] dynamic tensors return Unsupported, not a panic
[ ] no new MattenError variant
[ ] compatibility.md and the arithmetic reference updated
[ ] public-api-snapshot.md gains four items
[ ] nine guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
[ ] cargo test --workspace; both feature profiles
[ ] no version bump, tag, or publish — the 0.47.0 release RFC owns those
```

## 10. What this does not fix

```text
- (RESOLVED elsewhere) the budget question. RFC-132 answered it: the budget no
  longer applies to arithmetic at all, so there is nothing to raise. This bullet
  is kept, struck, as the record of what was open when this RFC was written.
- the other Result-zone gaps, if any. This RFC closes the four the audit named;
  if you find a fifth convenience API without a twin, REPORT it rather than
  adding it here.
```
