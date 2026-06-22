# RFC-041 Developer Handoff: Linear Algebra Boundary — Core Lite vs External Crates

**Project:** `matten`  
**RFC:** RFC-041  
**Handoff Kind:** Boundary / Optional Implementation Handoff  
**Implementation Level:** Decision first; minimal helpers only  
**Status:** Draft handoff for developer review  
**Applies To:** v0.20+ planning and implementation sequence  

---

## 0. Handoff Summary

This document turns RFC-041 into developer-executable work. It is not a replacement for the RFC. The RFC remains the design authority; this handoff translates it into implementation phases, PR boundaries, checks, and acceptance criteria.

## 1. Implementation Handoff

RFC-041 sets the linear algebra boundary.

Possible small core helpers, if accepted:

```rust
norm()
trace()
outer()
```

Rejected from core:

```text
inverse
determinant
solve
eigenvalues
SVD
QR
LU
Cholesky
sparse
BLAS/LAPACK
```

This RFC should mostly produce documentation and explicit non-goals unless maintainers approve the small helpers.

---

## 2. Internal Design

### 2.1 `norm`

Default:

```text
L2 norm over all elements
sqrt(sum(x_i^2))
```

No axis variants in first implementation.

### 2.2 `trace`

Policy options:

```text
A) rank-2 only, rectangular allowed over min(rows, cols)
B) rank-2 only, square required
```

Recommendation: A, because it is simple and useful if documented.

### 2.3 `outer`

Rules:

```text
rank-1 inputs only
output shape [left.len(), right.len()]
```

### 2.4 Dynamic behavior

Reject dynamic tensors.

---

## 3. Task Breakdown / PR Plan

### PR-041-1: Boundary documentation

- Add RFC.
- Update docs with linalg boundary.
- Point serious users to specialized crates.

Acceptance:

```text
[ ] advanced linalg rejected from core
[ ] no dependency added
```

### PR-041-2: Optional `norm`

Implement only if approved.

Acceptance:

```text
[ ] L2 norm correct
[ ] scalar/vector/matrix tested
[ ] NaN/Inf behavior documented
```

### PR-041-3: Optional `trace`

Implement only if approved.

Acceptance:

```text
[ ] rank-2 only
[ ] rectangular policy tested
[ ] invalid rank tested
```

### PR-041-4: Optional `outer`

Implement only if approved.

Acceptance:

```text
[ ] rank-1 only
[ ] output shape correct
[ ] output values correct
```

---

## 4. Acceptance / QA Checklist

### Boundary QA

```text
[ ] no inverse/determinant/eigen/SVD/QR/Cholesky
[ ] no BLAS/LAPACK dependency
[ ] no sparse matrix support
[ ] docs recommend specialized crates
```

### Helper QA, if implemented

```text
[ ] norm tested
[ ] trace tested
[ ] outer tested
[ ] dynamic rejected
[ ] errors actionable
[ ] examples compile
```

### CI

```bash
cargo fmt --all --check
cargo clippy -p matten --all-targets --all-features -- -D warnings
cargo test -p matten --all-targets --all-features
bash scripts/check-core-dependency-boundary.sh
```

---

## 5. Do Not Implement

- serious linalg algorithms;
- GPU/device support;
- BLAS backend;
- dtype-generic linalg;
- matrix decomposition;
- sparse formats.
