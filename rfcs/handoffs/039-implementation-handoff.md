# RFC-039 Developer Handoff: Shape Composition API Boundary

**Project:** `matten`  
**RFC:** RFC-039  
**Handoff Kind:** Design + Limited Implementation Handoff  
**Implementation Level:** Implement only if scope remains clear  
**Status:** Draft handoff for developer review  
**Applies To:** v0.20+ planning and implementation sequence  

---

## 0. Handoff Summary

This document turns RFC-039 into developer-executable work. It is not a replacement for the RFC. The RFC remains the design authority; this handoff translates it into implementation phases, PR boundaries, checks, and acceptance criteria.

## 1. Implementation Handoff

RFC-039 should first decide whether to implement only:

```rust
try_concatenate
concatenate
try_stack
stack
```

Do not implement `repeat`, `tile`, or `meshgrid` in the first PR.

---

## 2. Internal Design

### 2.1 Functions

Recommended signatures:

```rust
pub fn try_concatenate(tensors: &[&Tensor], axis: usize) -> Result<Tensor, MattenError>;
pub fn concatenate(tensors: &[&Tensor], axis: usize) -> Tensor;

pub fn try_stack(tensors: &[&Tensor], axis: usize) -> Result<Tensor, MattenError>;
pub fn stack(tensors: &[&Tensor], axis: usize) -> Tensor;
```

### 2.2 Concatenate algorithm

1. Validate non-empty input.
2. Reject dynamic tensors.
3. Validate same rank.
4. Validate axis in range.
5. Validate all dimensions except axis match.
6. Compute output shape with checked addition/product.
7. Allocate output.
8. Copy data in row-major logical order.

### 2.3 Stack algorithm

1. Validate non-empty input.
2. Reject dynamic tensors.
3. Validate all shapes equal.
4. Validate axis in `0..=ndim`.
5. Insert new dimension of input count.
6. Allocate output.
7. Copy data into stacked row-major order.

### 2.4 Limits

Use `MattenLimits`.

---

## 3. Task Breakdown / PR Plan

### PR-039-1: Shape validation helpers

Implement internal helpers:

```text
validate_concatenate_shapes
validate_stack_shapes
```

Acceptance:

```text
[ ] no allocation in validation except output shape
[ ] clear errors
[ ] tested independently if possible
```

### PR-039-2: Concatenate

Implement try and panic forms.

Acceptance:

```text
[ ] axis 0 and axis 1 examples pass
[ ] rank mismatch tested
[ ] dimension mismatch tested
[ ] empty input tested
[ ] dynamic rejected
```

### PR-039-3: Stack

Implement try and panic forms.

Acceptance:

```text
[ ] axis 0..=ndim tested
[ ] shape mismatch tested
[ ] empty input tested
[ ] dynamic rejected
```

### PR-039-4: Docs/examples

Add examples but keep repeat/tile/meshgrid deferred.

Acceptance:

```text
[ ] examples compile
[ ] docs say repeat/tile/meshgrid deferred
```

---

## 4. Acceptance / QA Checklist

```text
[ ] borrowed input API accepted
[ ] output shape correct
[ ] output data order correct
[ ] empty input returns error
[ ] axis errors actionable
[ ] allocation limit enforced
[ ] dynamic tensors rejected
[ ] repeat/tile/meshgrid not implemented
```

CI:

```bash
cargo fmt --all --check
cargo clippy -p matten --all-targets --all-features -- -D warnings
cargo test -p matten --all-targets --all-features
```

---

## 5. Internal Test Matrix

Concatenate:

```text
[2] + [3] axis 0 -> [5]
[2,3] + [4,3] axis 0 -> [6,3]
[2,3] + [2,5] axis 1 -> [2,8]
rank mismatch -> Err
dimension mismatch -> Err
```

Stack:

```text
two [3] axis 0 -> [2,3]
two [3] axis 1 -> [3,2]
two [2,3] axis 0 -> [2,2,3]
two [2,3] axis 1 -> [2,2,3]
two [2,3] axis 2 -> [2,3,2]
```

Verify axis examples carefully against implementation.

---

## 6. Do Not Implement

- repeat;
- tile;
- meshgrid;
- lazy views;
- zero-copy stack;
- ragged stack;
- dtype promotion.
