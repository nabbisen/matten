# RFC-056: Result-Form Axis Reductions — `try_sum_axis` / `try_mean_axis` / `try_min_axis` / `try_max_axis`

**Status:** Implemented (v0.24.0); architect-accepted (review 2026-06-27)
**Target Release:** v0.24.0 (additive, minor bump)
**Related:** RFC-040 (statistics core: `var_axis`/`std_axis` + their `try_` forms), RFC-019 (reductions), RFC-031 (dynamic rejection)
**Scope:** Give every axis reduction in core a non-panicking `Result` form, matching the `try_var_axis`/`try_std_axis` precedent

---

## 1. Summary

Core `matten` has six axis reductions. Two of them — `var_axis` and `std_axis` — already
expose a non-panicking `try_` form (`try_var_axis`, `try_std_axis`). The other four —
`sum_axis`, `mean_axis`, `min_axis`, `max_axis` — are panic-only.

This RFC adds the four missing forms:

```text
try_sum_axis(&self, axis) -> Result<Tensor, MattenError>
try_mean_axis(&self, axis) -> Result<Tensor, MattenError>
try_min_axis(&self, axis) -> Result<Tensor, MattenError>
try_max_axis(&self, axis) -> Result<Tensor, MattenError>
```

It is purely additive: no signature changes, no behavior change to the existing panic forms,
no new error variant, no new dependency, f64-only core.

## 2. Motivation

Axis reductions fail on a **real, common runtime condition**: an out-of-bounds axis index.
Today that condition is only reachable as a panic:

```rust
let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
let axis = user_supplied_axis();      // e.g. parsed from CLI / config
let reduced = m.sum_axis(axis);       // panics if axis >= 2 — no way to recover
```

The sibling statistics reductions already model exactly this failure as a recoverable
`Result`:

```rust
let v = m.try_var_axis(axis)?;        // MattenError::Shape if axis >= rank
```

So the surface is inconsistent: a caller who computes an axis from untrusted input can recover
from a bad axis on `var_axis` but must pre-validate or risk a panic on `sum_axis`. Unlike the
scalar value reductions (see RFC-055), the axis failure mode here is **not** the dynamic-tensor
case routed through `try_numeric()` — it is an ordinary shape/index error that any reduction
can hit at runtime. That makes the `try_` form independently useful, not merely a uniformity
nicety.

## 3. Current surface

| Axis reduction | Panic form | `try_` form | Panics on |
|---|---|---|---|
| sum | `sum_axis` | — (missing) | `axis >= rank`; dynamic tensor |
| mean | `mean_axis` | — (missing) | `axis >= rank`; dynamic tensor |
| min | `min_axis` | — (missing) | `axis >= rank`; dynamic tensor |
| max | `max_axis` | — (missing) | `axis >= rank`; dynamic tensor |
| var | `var_axis` | `try_var_axis` ✓ | `axis >= rank` → `Shape`; dynamic → `Unsupported` |
| std | `std_axis` | `try_std_axis` ✓ | `axis >= rank` → `Shape`; dynamic → `Unsupported` |

The axis-bounds check is also currently scattered: `mean_axis` carries its own inline
`panic!`, while `sum_axis`/`min_axis`/`max_axis` rely on the bounds check inside the
`axis_reduce` / `nan_axis_reduce` helpers. Routing all four through `try_` engines centralises
that check in one place per family.

## 4. Proposed design

### 4.1 Error contract (reuses existing variants)

```text
axis >= self.ndim()  ->  MattenError::Shape          (mirrors try_var_axis)
dynamic tensor        ->  MattenError::Unsupported     (mirrors try_var_axis; dynamic feature only)
otherwise             ->  Ok(Tensor)  with the reduced axis removed (no keepdims)
```

No new `MattenError` variant is introduced. NaN propagation, the "remove the reduced axis"
output-shape rule, and population semantics elsewhere are all unchanged.

### 4.2 Implementation pattern — `try_` is the engine

Follow the established `var_axis`/`std_axis` shape: the `try_` form is the real
implementation; the panic form becomes a thin wrapper.

```rust
pub fn try_sum_axis(&self, axis: usize) -> Result<Tensor, MattenError> { /* checks + reduce */ }

#[must_use]
pub fn sum_axis(&self, axis: usize) -> Tensor {
    self.try_sum_axis(axis).unwrap_or_else(|e| panic!("{e}"))
}
```

This guarantees the two forms never diverge and matches how `var_axis = try_var_axis(...).unwrap_or_else(...)`
is already written.

**Decision point — panic-message text.** Routing the panic forms through the `try_` engine
makes their panic text come from the `MattenError` `Display` (exactly as `var_axis` already
does), which differs textually from the current hand-written messages in `mean_axis` /
`axis_reduce`. The author recommends accepting that alignment (one consistent panic-message
source across all axis reductions). If the architect prefers byte-stable panic messages, the
fallback is to add the four `try_` forms **alongside** the existing panic forms without
rewriting their internals — slightly more duplication, identical public behavior.

## 5. Acceptance criteria

- `try_sum_axis`, `try_mean_axis`, `try_min_axis`, `try_max_axis` exist, each returning
  `Result<Tensor, MattenError>`.
- Each returns `MattenError::Shape` for `axis >= rank` and (under the `dynamic` feature)
  `MattenError::Unsupported` for a dynamic tensor.
- On the success path each is numerically identical to its panic form (same values, same
  output shape, same NaN propagation).
- The panic forms still panic on the same conditions.
- No new `MattenError` variant; no new dependency; core stays f64-only; MSRV unchanged.
- Rustdoc for each panic form cross-links its `try_` form (and vice versa), matching the
  `var_axis`/`try_var_axis` docs.

## 6. Non-goals

- No new reductions (no `prod_axis`, `median_axis`, `argmin_axis`, etc.).
- No `keepdims` option.
- No change to NaN policy, population/ddof semantics, or output-shape rules.
- No performance work; the axis-reduction cost question stays gated on the ROADMAP
  regression-visibility anchor (evidence-driven only).
- No companion crate.
- Scalar value reductions (`try_sum`/`try_mean`/`try_min`/`try_max`/`try_norm`) are handled
  separately in RFC-055; this RFC stands alone and does not depend on it.

## 7. Testing

Design-spec-driven (per project testing guidelines):

- Ok path per method on a rank-2 tensor; assert values/shape equal the panic form's output.
- `try_*_axis(rank)` and a larger axis both return `Err(MattenError::Shape)`.
- Under the `dynamic` feature: a dynamic tensor returns `Err(MattenError::Unsupported)`; a
  numeric tensor succeeds.
- Each panic form still panics for `axis >= rank` (`#[should_panic]`).
- Doctests on each `try_` form showing one Ok and one `is_err()` case (as `try_var_axis` does).
- Full feature matrix (lean / dynamic / all-features) and MSRV 1.85.

## 8. Relationship to RFC-055

RFC-055 (scalar value reductions) and this RFC together complete the reduction-family
consistency theme for v0.24. They are independent: this one addresses an ordinary shape-error
recovery gap and is the stronger, self-contained case; RFC-055 addresses dynamic-tensor
uniformity and revisits a prior `norm` ruling. The architect may accept, defer, or reject each
independently; if both are accepted they can ship in the same v0.24.0 minor or be staged.
