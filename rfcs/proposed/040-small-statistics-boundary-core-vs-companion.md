# RFC-040: Small Statistics Boundary — Core vs Companion

**Status:** Proposed (accepted for implementation — v0.21 boundary review, 2026-06-23; see Architect Rulings below). Target v0.21.2.
**Target Release:** v0.21+ decision  
**Related:** RFC-019, RFC-038  
**Scope:** Decide where variance, standard deviation, covariance, correlation, quantile, and histogram belong

---

## 1. Summary

This RFC defines the boundary for statistics APIs.

The core question:

```text
Which statistics are universal enough for core matten,
and which belong in a future matten-stats companion?
```

Recommended initial direction:

```text
Core may accept small obvious reductions:
  var, std, maybe median only after policy is clear.

A future matten-stats companion should own:
  covariance, correlation, quantile, percentile, histogram,
  and anything with significant statistical policy.
```

---

## 2. Motivation

Users expect basic statistics in numerical arrays. But statistics APIs have policy traps:

- population vs sample variance;
- `ddof`;
- NaN handling;
- empty tensor behavior;
- axis behavior;
- quantile interpolation;
- histogram bin policy;
- weighted variants.

Putting too much in core risks growing `matten` into a statistics library.

---

## 3. Candidate APIs

### 3.1 Possible Core

```rust
impl Tensor {
    pub fn var(&self) -> f64;
    pub fn std(&self) -> f64;

    pub fn var_axis(&self, axis: usize) -> Tensor;
    pub fn std_axis(&self, axis: usize) -> Tensor;
}
```

Only if policy is clear.

### 3.2 Possible Companion

```rust
matten_stats::cov(&x)
matten_stats::corrcoef(&x)
matten_stats::quantile(&x, q)
matten_stats::percentile(&x, p)
matten_stats::histogram(&x, bins)
matten_stats::zscore(&x)
```

Some `zscore` may overlap with `matten-mlprep`; avoid duplication.

---

## 4. Core Inclusion Rule

A statistic may enter core only if:

```text
[ ] it is common in beginner/intermediate numerical workflows
[ ] it can be explained without statistical domain discussion
[ ] policy fits in rustdoc
[ ] it has no heavy dependency
[ ] axis semantics match existing reduction APIs
[ ] NaN behavior is documented
```

`mean` already qualifies.

`var` and `std` may qualify, but only after ddof policy.

---

## 5. Recommended Variance Policy

If core adds variance:

```rust
pub fn var(&self) -> f64;        // population variance, ddof = 0
pub fn std(&self) -> f64;        // sqrt(var)
pub fn var_sample(&self) -> f64; // optional later, ddof = 1
```

Alternative:

```rust
pub fn var_with_ddof(&self, ddof: usize) -> Result<f64, MattenError>;
```

Recommendation:

```text
Do not add ddof-heavy API to core first.
If core adds var/std, define them as population variance and document it clearly.
```

---

## 6. NaN Policy

Options:

1. Propagate NaN.
2. Ignore NaN.
3. Error on NaN.
4. Provide nan-aware variants.

Recommendation for core:

```text
Propagate NaN for simple reductions.
Do not silently ignore NaN.
```

This is **deliberate house policy** (architect ruling, RFC-033–042 review Q6), and
is consistent with RFC-038 §5.1: ordinary `f64` reductions such as `var`/`std`
follow `f64` propagation (NaN input produces NaN output), whereas selection/
index-producing APIs (e.g. `argmin`/`argmax`) instead return `Err`/panic because
NaN makes their result ill-defined. Reductions can preserve `f64` semantics; index
selection cannot. Document the propagation behavior on each reduction.

Future companion may provide:

```rust
nanmean
nanstd
```

if needed.

---

## 7. Axis Policy

Axis statistics should follow existing axis-reduction shape behavior:

```text
reduce axis -> remove that axis
```

No `keepdims` in first version unless a separate shape RFC accepts it.

---

## 8. Histogram / Quantile

Do not put histogram or quantile in core initially.

Reasons:

- bin selection policy;
- interpolation policy;
- edge inclusion policy;
- sorting/allocation cost;
- statistical expectations.

These belong in `matten-stats` if accepted.

---

## 9. `matten-stats` Companion Criteria

Create `matten-stats` only if:

- at least three APIs are clearly useful;
- policy choices are documented;
- examples are small;
- it does not overlap confusingly with `matten-mlprep`;
- it does not introduce heavy dependencies.

Initial maturity would be:

```text
Experimental
```

---

## 10. Acceptance Criteria

```text
[ ] decide whether var/std enter core
[ ] define population/sample policy
[ ] define NaN policy
[ ] define axis behavior
[ ] defer quantile/histogram unless companion accepted
[ ] prevent overlap with matten-mlprep
[ ] no heavy dependencies
```

---

## 11. Non-goals

- No full SciPy stats clone.
- No probability distribution library.
- No statistical tests.
- No regression models.
- No time series.

---

## Architect Rulings — v0.21 Boundary Review (2026-06-23)

All questions accepted. `var`/`std` and `var_axis`/`std_axis` are authorized for core
(target **v0.21.2**), population variance only; quantile/histogram/cov/corr/z-score
deferred; no `matten-stats` companion yet.

**Q3 — Add `var(&self) -> f64` and `std(&self) -> f64`** (option a). Provide `try_*`
forms if the existing reduction family uses them or if empty/dynamic cases need
non-panicking behavior; otherwise preserve the existing panic-zone convention and
document clearly.

**Q4 — Population variance only (ddof = 0):** `var = sum((x_i - mean)^2) / n`,
`std = sqrt(var)`. Rustdoc must state "population variance, not sample variance" and
include the formula note. Do **not** add `var_sample`/`std_sample`/`var_with_ddof`/
`std_with_ddof` in the first cut.

**Q5 — Add `var_axis(axis)` / `std_axis(axis)`** (+ `try_*` if consistent). Axis
reductions remove the reduced axis; no `keepdims` in the first cut (e.g. `[2,3]`
axis 0 → `[3]`, axis 1 → `[2]`).

**Q6 — Defer** quantile, percentile, histogram, covariance, correlation, z-score from
core (future `matten-stats` candidates).

**Q7 — Do not scaffold `matten-stats` yet:** create it only after ≥3 clearly-useful,
well-scoped APIs are accepted. No empty/speculative companion.

**Additional statistics constraints (architect):**

- **Empty-tensor policy:** `try_var`/`try_std` on an empty tensor →
  `Err(MattenError::InvalidArgument { operation, argument: "self", .. })`; the panic
  forms panic with a clear message. (`Tensor` already forbids zero-sized dimensions —
  document this and add tests; an empty tensor is not constructible.)
- **NaN policy:** `var`/`std` propagate NaN; `var_axis`/`std_axis` propagate NaN
  within the reduced slice. No `nanvar`/`nanstd` in core.
- **Numerical algorithm:** use a **two-pass** variance (mean first, then squared
  deviations). Avoid the naive one-pass `E[x^2] - E[x]^2` if it risks avoidable
  cancellation. Compensated algorithms are not required in the first cut, but do not
  choose the weakest formulation when a simple two-pass one is available.

**Required tests** (architect checklist): var/std for a simple vector; population
variance documented + tested; singleton variance = 0.0; NaN propagates; empty-tensor
behavior matches policy; `var_axis`/`std_axis` axes 0 & 1; invalid axis → Shape;
dynamic rejection if applicable. Suggested values: `[1,2,3,4]` → mean 2.5, population
variance 1.25, std `sqrt(1.25)`.
