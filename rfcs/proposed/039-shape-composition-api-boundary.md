# RFC-039: Shape Composition API Boundary

**Status:** Proposed  
**Target Release:** v0.21+  
**Related:** RFC-038  
**Scope:** Boundary for stack, concatenate, repeat, tile, meshgrid, and related shape APIs

---

## 1. Summary

This RFC defines which shape-composition APIs may enter core `matten`, which need more design, and which should be deferred.

Unlike `squeeze` and `expand_dims`, shape composition APIs can introduce complex semantics and shape errors. They require a boundary RFC before implementation.

Candidate APIs:

```rust
Tensor::stack(...)
Tensor::concatenate(...)
tensor.repeat(...)
tensor.tile(...)
Tensor::meshgrid(...)
```

Recommended decision:

```text
Do not bundle these into RFC-038.
Design them separately and implement only the smallest subset first.
```

---

## 2. Motivation

Users familiar with NumPy often expect shape-composition helpers. They are useful, but they are also a source of confusion:

- axis semantics;
- rank promotion;
- broadcasting expectations;
- memory allocation size;
- empty inputs;
- singleton axes;
- dynamic tensor behavior.

`matten` should add them only where the behavior is obvious and teachable.

---

## 3. Classification

### 3.1 Lower-risk

```rust
concatenate(tensors, axis)
stack(tensors, axis)
```

These are common and useful.

### 3.2 Medium-risk

```rust
repeat(axis, count)
tile(repeats)
```

These can produce large allocations and need limits.

### 3.3 Higher-risk

```rust
meshgrid(...)
```

This can confuse users and produce large tensors quickly.

Recommendation:

```text
Implement concatenate/stack first if accepted.
Defer repeat/tile/meshgrid until allocation and examples are clear.
```

---

## 4. Proposed API Shape

Possible associated functions:

```rust
impl Tensor {
    pub fn try_concatenate(tensors: &[&Tensor], axis: usize) -> Result<Tensor, MattenError>;
    pub fn concatenate(tensors: &[&Tensor], axis: usize) -> Tensor;

    pub fn try_stack(tensors: &[&Tensor], axis: usize) -> Result<Tensor, MattenError>;
    pub fn stack(tensors: &[&Tensor], axis: usize) -> Tensor;
}
```

Recommendation:

```text
Prefer borrowed form.
Avoid forcing users to clone just to pass inputs.
```

---

## 5. Semantics

### 5.1 Concatenate

`concatenate` joins tensors along an existing axis.

Rules:

- input list must be non-empty;
- all tensors must have same rank;
- all dimensions except target axis must match;
- output axis size is sum of input axis sizes;
- axis must be valid;
- output is owned row-major Tensor.

Example:

```text
[2, 3] + [4, 3] along axis 0 -> [6, 3]
[2, 3] + [2, 5] along axis 1 -> [2, 8]
```

### 5.2 Stack

`stack` joins tensors along a new axis.

Rules:

- input list must be non-empty;
- all tensors must have identical shape;
- axis may be `0..=ndim`;
- output rank is input rank + 1;
- new axis size is number of tensors.

Examples:

```text
three [2, 4] tensors stacked at axis 0 -> [3, 2, 4]
three [2, 4] tensors stacked at axis 1 -> [2, 3, 4]
three [2, 4] tensors stacked at axis 2 -> [2, 4, 3]
```

Non-square shapes are used deliberately so the new-axis position is visible; avoid
examples where different axes coincidentally produce the same shape, since that
weakens teaching value and makes implementation review harder.

---

## 6. Allocation Safety

All shape composition APIs must use `MattenLimits`.

Required:

- checked product;
- allocation budget check;
- `try_*` API returns `MattenError::Allocation`;
- convenience API may panic with actionable message.

---

## 7. Dynamic Behavior

Initial recommendation:

```text
Reject dynamic tensors for stack/concatenate unless a future RFC explicitly supports them.
```

Reason:

- dynamic storage/on-ramp semantics should not grow into computation semantics;
- users can convert to numeric first.

---

## 8. Deferred APIs

### 8.1 `repeat`

Needs decisions:

- repeat whole tensor or repeat along axis?
- scalar repeat semantics?
- output size limit?
- relation to broadcasting?

### 8.2 `tile`

Needs decisions:

- repeat vector shape length;
- rank padding semantics;
- allocation guard.

### 8.3 `meshgrid`

Needs decisions:

- indexing style (`xy` vs `ij`);
- output type (`Vec<Tensor>`? tuple?);
- allocation risk;
- beginner docs.

Do not implement these until separate acceptance.

---

## 9. Acceptance Criteria

```text
[ ] concatenate and stack semantics are fully specified
[ ] examples are verified
[ ] empty input behavior defined
[ ] axis behavior defined
[ ] allocation checks use MattenLimits
[ ] dynamic behavior defined
[ ] repeat/tile/meshgrid remain deferred unless separately approved
```

---

## 10. Non-goals

- No lazy views.
- No zero-copy stacking.
- No ragged arrays.
- No dtype promotion.
- No dataframe-style concatenation.
