# RFC-041: Linear Algebra Boundary — Core Lite vs External Crates

**Status:** Implemented (v0.21.1) — `norm` + `trace` + `outer` shipped in core; advanced linalg/decomposition/BLAS/sparse rejected (§5).
**Target Release:** v0.21+ decision  
**Related:** RFC-010, RFC-025, RFC-038  
**Scope:** Boundary for linear algebra APIs in core and companions

---

## 1. Summary

This RFC defines the linear algebra boundary for `matten`.

`matten` may provide a very small set of obvious linear algebra helpers, but it must not become a linear algebra backend.

Recommended direction:

```text
Core may contain:
  dot
  matmul
  norm
  trace
  outer, if accepted

External/specialized crates should own:
  inverse
  determinant
  eigenvalues
  SVD
  QR
  Cholesky
  sparse
  BLAS/LAPACK integration
```

---

## 2. Motivation

Mathematical users expect some linear algebra. But serious linear algebra quickly requires:

- numerical stability decisions;
- decomposition algorithms;
- BLAS/LAPACK;
- sparse formats;
- dtype control;
- performance expectations.

That conflicts with `matten`'s family-car philosophy.

---

## 3. Current Position

Core already supports or has planned support for:

```text
dot
matmul
transpose
```

These are appropriate because they are foundational and easy to explain.

The question is how far to go.

---

## 4. Allowed Core-Lite Candidates

### 4.1 `norm`

```rust
impl Tensor {
    pub fn norm(&self) -> f64;
}
```

Default:

```text
L2 norm over all elements
```

Optional future:

```rust
norm_l1
norm_l2
norm_inf
norm_axis
```

Recommendation:

```text
Add only L2 norm first if accepted.
```

### 4.2 `trace`

```rust
impl Tensor {
    pub fn trace(&self) -> f64;
}
```

Behavior:

- rank-2 only;
- sum diagonal;
- rectangular matrices allowed using min(rows, cols), or reject non-square.

Recommendation:

```text
Allow rectangular trace over min(rows, cols), matching common array behavior, if documented.
```

### 4.3 `outer`

```rust
impl Tensor {
    pub fn outer(&self, rhs: &Tensor) -> Tensor;
}
```

Behavior:

- both inputs rank-1;
- output shape `[left.len(), right.len()]`.

`outer` is simple and useful, but may wait until core comfort APIs are stable.

---

## 5. Rejected from Core

Core should not implement:

```text
inverse
determinant
solve
least_squares
eigenvalues
eigenvectors
SVD
QR
LU
Cholesky
sparse matrices
BLAS/LAPACK backend
```

Rationale:

- too much numerical policy;
- too much performance expectation;
- better handled by `nalgebra`, `ndarray-linalg`, or other crates;
- would change project identity.

---

## 6. Companion Options

### 6.1 `matten-nalgebra`

A future bridge crate may help users move from `matten::Tensor` to `nalgebra` structures.

Requires future RFC.

### 6.2 `matten-linalg-lite`

A future companion could provide small educational / PoC helpers, but this is risky.

Only consider if:

- it stays tiny;
- it does not depend on BLAS;
- it does not imply production numerical linear algebra;
- it has clear non-goals.

Recommendation:

```text
Prefer bridge-to-specialized-crate over owning linalg algorithms.
```

---

## 7. Numerical Reliability

If any linalg-like API is added, docs must state:

```text
matten prioritizes PoC ergonomics, not numerical linear algebra performance or stability leadership.
```

For serious workloads, recommend specialized crates.

---

## 8. Acceptance Criteria

```text
[ ] decide whether norm/trace/outer enter core
[ ] document exact shape behavior
[ ] reject advanced decomposition APIs from core
[ ] no BLAS/LAPACK dependency in core
[ ] no sparse support in core
[ ] future nalgebra/candle bridge work requires separate RFC
```

---

## 9. Non-goals

- No SciPy linalg clone.
- No production solver package.
- No GPU acceleration.
- No sparse algebra.
- No dtype-generic linalg.

---

## Architect Rulings — v0.21 Boundary Review (2026-06-23)

All questions accepted. `norm` + `trace` + `outer` are authorized for core (target
**v0.21.1**); serious linalg/decomposition/BLAS/sparse rejected from core.

**Q8 — Accept all three (option a):** `norm(&self) -> f64`, `trace(&self) -> f64`,
`outer(&self, other: &Tensor) -> Tensor`, with `try_*` forms if consistent with the
current panic/Result split.

**Q9 — `norm` = L2/Frobenius over all elements only:** `sqrt(sum(x_i^2))` (Frobenius
for matrices). Do not add `norm_l1`/`norm_inf`/`norm_axis`/`norm_with_order` in the
first cut. NaN propagates (any NaN element → NaN). No special scaling/overflow
algorithm guaranteed in the first cut (document this).

**Q10 — `trace` rectangular via `min(rows, cols)` (option a):** rank-2 only; sum
`self[i, i]` for `i in 0..min(rows, cols)`. Invalid rank → `try_trace` returns
`MattenError::Shape`, `trace` panics with a clear message. Rustdoc must state the
rectangular behavior explicitly.

**Q11 — Confirm core rejection:** `inverse`, `determinant`, `solve`, eigen*, SVD,
QR, LU, Cholesky, sparse formats, BLAS/LAPACK dependencies are rejected from core;
`matten-nalgebra` / `matten-ndarray-linalg` bridges require separate future RFCs.
Required docs wording: "Core `matten` provides small linalg-adjacent helpers, not a
linear algebra backend."

**`outer`:** rank-1 × rank-1 → `[m, n]`; `MattenLimits` allocation check; reject
non-rank-1 input.

**Required tests** (architect checklist): norm `[3,4]` = 5; norm matrix uses all
elements; norm NaN propagates; trace square; trace rectangular rows<cols and
rows>cols; trace rank≠2→Shape; outer `[m]×[n]`→`[m,n]` with correct values; outer
rejects non-rank-1; dynamic rejection where applicable; `MattenLimits` failure for
outer.
