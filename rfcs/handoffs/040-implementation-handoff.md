# RFC-040 Developer Handoff: Small Statistics Boundary — Core vs Companion

**Project:** `matten`  
**RFC:** RFC-040  
**Handoff Kind:** Boundary / Optional Implementation Handoff  
**Implementation Level:** Decision first; possible small core implementation  
**Status:** Draft handoff for developer review  
**Applies To:** v0.20+ planning and implementation sequence  

---

## 0. Handoff Summary

This document turns RFC-040 into developer-executable work. It is not a replacement for the RFC. The RFC remains the design authority; this handoff translates it into implementation phases, PR boundaries, checks, and acceptance criteria.

## 1. Implementation Handoff

RFC-040 is primarily a boundary decision.

Recommended near-term implementation, if accepted:

```rust
Tensor::var()
Tensor::std()
Tensor::var_axis(axis)
Tensor::std_axis(axis)
```

with **population variance** (`ddof = 0`) only.

Do not implement covariance, correlation, quantile, percentile, histogram, or statistical tests in core.

---

## 2. Internal Design

### 2.1 Variance policy

Core default:

```text
population variance
ddof = 0
NaN propagates
axis reduction removes axis
```

Formula:

```text
mean = sum(x) / n
var = sum((x - mean)^2) / n
std = sqrt(var)
```

For numerical stability, a two-pass implementation is acceptable for clarity.

### 2.2 Empty tensors

Core currently rejects zero-sized dimensions, so empty behavior should be unreachable in normal tensors. If future zero-size support appears, this must be revisited.

### 2.3 Dynamic behavior

Reject dynamic tensors unless converted with `try_numeric()`.

---

## 3. Task Breakdown / PR Plan

### PR-040-1: Boundary docs only

- Add RFC.
- Update roadmap.
- Add docs saying stats beyond var/std are deferred.

Acceptance:

```text
[ ] quantile/histogram/cov/corr not in core
[ ] future matten-stats criteria documented
```

### PR-040-2: Optional core var/std

Only after maintainers approve.

Implement:

```rust
var
std
```

Acceptance:

```text
[ ] population variance documented
[ ] NaN propagation tested
[ ] examples added
```

### PR-040-3: Optional axis var/std

Implement:

```rust
var_axis
std_axis
```

Acceptance:

```text
[ ] axis behavior matches mean_axis
[ ] invalid axis tested
[ ] shape after reduction tested
```

### PR-040-4: `matten-stats` decision note

If maintainers want more stats, create a future RFC for `matten-stats`.

Acceptance:

```text
[ ] no companion created without separate approval
```

---

## 4. Acceptance / QA Checklist

### Boundary QA

```text
[ ] no SciPy stats clone
[ ] no probability distributions
[ ] no statistical tests
[ ] no regression models
[ ] no quantile/histogram in core without separate approval
```

### Implementation QA, if var/std accepted

```text
[ ] population variance correct
[ ] std = sqrt(var)
[ ] axis variants remove axis
[ ] NaN behavior documented and tested
[ ] dynamic tensors rejected
[ ] examples compile
```

### CI

```bash
cargo fmt --all --check
cargo clippy -p matten --all-targets --all-features -- -D warnings
cargo test -p matten --all-targets --all-features
```

---

## 5. Do Not Implement

- `ddof` general API in first PR;
- sample variance unless explicitly approved;
- quantile;
- histogram;
- covariance;
- correlation;
- distributions;
- statistical tests;
- time series.
