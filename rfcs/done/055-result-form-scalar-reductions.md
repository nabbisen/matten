# RFC-055: Result-Form Scalar Reductions — `try_sum` / `try_mean` / `try_min` / `try_max` / `try_norm`

**Status:** Implemented (v0.24.0); architect-accepted (review 2026-06-27)
**Target Release:** v0.24.0 (additive, minor bump)
**Related:** RFC-040 (statistics core: `var`/`std` + their `try_` forms), RFC-041 (linear-algebra core-lite: `norm`), RFC-019 (reductions), RFC-031 (dynamic rejection)
**Scope:** Decide whether the scalar value reductions should gain non-panicking `Result` forms for surface uniformity, and if so, define their contract

---

## 1. Summary

Core `matten` exposes scalar value reductions that return `f64`. Two families differ in shape:

- `var` / `std` provide a non-panicking `try_var` / `try_std` (the `try_` form is the engine;
  the panic form is a thin `unwrap_or_else(panic)` wrapper).
- `sum`, `mean`, `min`, `max`, and `norm` are **panic-only** — no `try_` form.

This RFC proposes adding `try_sum`, `try_mean`, `try_min`, `try_max`, `try_norm` so the whole
scalar-reduction family is uniform. It is additive (no signature or behavior change to the
existing panic forms), introduces no new error variant or dependency, and keeps the core
f64-only.

This is the more *contested* half of the v0.24 reduction-consistency theme, and the RFC is
written to surface that honestly rather than assume the addition.

## 2. The tension this RFC must resolve

Unlike the axis reductions (RFC-056), the scalar value reductions have **exactly one reachable
failure mode**: the tensor is dynamic (only when the `dynamic` feature is on), which today
panics with a "call `try_numeric()` first" message. There is no empty-tensor failure —
constructors reject zero-sized dimensions, so length is always ≥ 1 — and `sum`/`min`/`max`/
`norm` of any valid tensor is total (`NaN` simply propagates; it is not an error).

`norm`'s own rustdoc records the prior decision explicitly:

> "…this matches the other value reductions (`sum`, `mean`), which have no `try_*` form."

i.e. the v0.21 boundary review (the `norm` panic-only ruling) deliberately chose **not** to give
scalar value reductions a `try_` form, on the grounds that their only failure is the
dynamic-tensor case, which the API already routes through `try_numeric()` up front.

So there is a genuine design conflict:

```text
Uniformity argument (for adding):
  var/std already have try_ forms guarding the same dynamic-tensor failure.
  A caller handling a possibly-dynamic tensor can stay on the Result path for
  var() but is forced to panic-or-pre-convert for sum()/min()/max()/norm().
  That asymmetry is surprising and undercuts the "Result-form reductions" surface.

Honest-API argument (against adding):
  A try_sum() whose only Err is "this tensor is dynamic" is largely redundant with
  try_numeric(): convert once, then every reduction is total. Adding five thin
  wrappers widens the surface for a failure mode the design already steers around.
```

This RFC recommends **adding** them (uniformity wins, given var/std set the precedent), but the
decision is the architect's, and §6 offers narrower fallbacks.

## 3. Current surface

| Scalar reduction | Panic form | `try_` form | Only reachable failure |
|---|---|---|---|
| sum | `sum` | — (missing) | dynamic tensor |
| mean | `mean` | — (missing) | dynamic tensor |
| min | `min` | — (missing) | dynamic tensor |
| max | `max` | — (missing) | dynamic tensor |
| norm | `norm` | — (missing; panic-only per prior ruling) | dynamic tensor |
| var | `var` | `try_var` ✓ | dynamic → `Unsupported` (+ defensive unreachable empty → `InvalidArgument`) |
| std | `std` | `try_std` ✓ | dynamic → `Unsupported` (+ defensive unreachable empty → `InvalidArgument`) |

## 4. Proposed design

### 4.1 Error contract (reuses existing variants)

```text
dynamic tensor  ->  MattenError::Unsupported   (dynamic feature only; mirrors try_var)
otherwise       ->  Ok(f64)                     (NaN propagates as a value, never an error)
```

No new `MattenError` variant. The author recommends **omitting** the defensive
empty-tensor guard that `try_var`/`try_std` carry: for `sum`/`min`/`max`/`norm` the empty
result is well-defined-if-degenerate, `mean`'s empty behavior is already documented as
unspecified, and an empty tensor is unconstructible regardless — so an `InvalidArgument` branch
here would be dead code. (Flagged as a minor, reviewable divergence from `try_var`/`try_std`.)

### 4.2 Implementation pattern — `try_` is the engine

Mirror `var`/`std`: the `try_` form holds the dynamic-rejection check (reusing the same helper
`try_var` uses, not the panic-form `require_numeric`), and the panic form delegates:

```rust
pub fn try_sum(&self) -> Result<f64, MattenError> { reject_dynamic(self, "sum")?; Ok(/* … */) }

#[must_use]
pub fn sum(&self) -> f64 { self.try_sum().unwrap_or_else(|e| panic!("{e}")) }
```

**Decision point — panic-message text.** As in RFC-056, routing the panic forms through the
`try_` engine sources their panic text from `MattenError` `Display` (as `var`/`std` already do),
changing it from the current hand-written `require_numeric` / `norm` messages. Recommended:
accept the alignment. Fallback: add the `try_` forms alongside, leaving the panic forms'
internals untouched.

### 4.3 The `norm` revisit (explicit)

Adding `try_norm` **reverses** the documented v0.21 decision that `norm` stays panic-only. The
RFC asks the architect to confirm that reversal. If the architect prefers to keep `norm`
panic-only, `try_norm` can be dropped from the set without affecting `try_sum`/`try_mean`/
`try_min`/`try_max` (and the `norm` rustdoc would be updated to stop implying `sum`/`mean` will
never have `try_` forms).

## 5. Acceptance criteria

- `try_sum`, `try_mean`, `try_min`, `try_max`, `try_norm` exist (subject to the `norm` ruling
  in §4.3), each returning `Result<f64, MattenError>`.
- Each returns `MattenError::Unsupported` for a dynamic tensor (under the `dynamic` feature) and
  is otherwise total, numerically identical to its panic form (including NaN propagation).
- The panic forms still panic on a dynamic tensor.
- No new `MattenError` variant; no new dependency; core stays f64-only; MSRV unchanged.
- `norm`'s rustdoc is corrected to reflect whatever the architect rules.
- Each panic form and its `try_` form cross-link in rustdoc.

## 6. Fallback scopes (for the architect)

In decreasing size, if full uniformity is not desired:

1. **Full set** (recommended): all five `try_` forms.
2. **Without `norm`**: `try_sum`/`try_mean`/`try_min`/`try_max`; keep `norm` panic-only.
3. **Defer entirely**: keep scalar value reductions panic-only, document the dynamic-tensor
   convention via `try_numeric()`, and rely on RFC-056 (axis) alone for the v0.24 theme. In this
   case the ROADMAP's tracked "Result-form reductions" item is formally closed as
   *considered-and-declined* rather than left open.

## 7. Non-goals

- No new reductions (`prod`, `median`, …).
- No change to NaN policy, population/ddof semantics, or any reduction's value.
- No performance work.
- No companion crate.
- Axis reductions are RFC-056; this RFC does not depend on it.

## 8. Testing

- Ok path per method; assert equality with the panic form.
- Under the `dynamic` feature: a dynamic tensor returns `Err(MattenError::Unsupported)`; a
  numeric tensor succeeds.
- Each panic form still panics on a dynamic tensor (`#[should_panic]`, dynamic feature).
- Doctests showing one Ok case per `try_` form.
- Full feature matrix (lean / dynamic / all-features) and MSRV 1.85.
