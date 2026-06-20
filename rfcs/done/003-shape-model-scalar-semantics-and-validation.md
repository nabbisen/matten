# RFC-003: Shape Model, Scalar Semantics, and Validation

> RFC status: Implemented (0.1.0)
> Project: `matten`  
> Target audience: library implementers, maintainers, reviewers  
> Design level: external design first; internal design where needed  
> Last updated: 2026-06-18

## 1. Summary

This RFC defines the shape model for `matten`: runtime `Vec<usize>` shapes, row-major logical order, scalar tensors as shape `[]`, checked shape product calculation, and validation rules for rank and element count. Shape semantics are the foundation for construction, reshape, broadcasting, slicing, reductions, and serialization.

## 2. Motivation

`matten` must be easy to use without type-level shapes. At the same time, it must avoid silent overflow and confusing errors. A runtime shape model gives NumPy-like flexibility while preserving enough invariant checking to avoid invalid internal states.

## 3. Goals

- Define shape as runtime `Vec<usize>`.
- Define scalar shape as `[]` with exactly one element.
- Define row-major order.
- Require checked multiplication for element counts.
- Decide initial rank policy.
- Define zero-sized dimension policy for `0.1.x`.

## 4. Non-goals

- No const-generic dimension arithmetic.
- No compile-time shape validation.
- No named axes.
- No ragged tensor representation.
- No sparse tensor model.

## 5. Cargo Features

The shape model applies to all feature sets.

- Default Phase 1 uses `Vec<f64>` plus shape.
- `dynamic` uses dynamic storage plus shape and possible view metadata.
- Shape semantics must not change across features.

## 6. Data Model

```rust
pub struct Tensor {
    data: Vec<f64>,
    shape: Vec<usize>,
}
```

Phase 2 may replace `data` with another storage representation, but the externally observed shape model is the same.

Recommended internal helper:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Shape {
    dims: Vec<usize>,
}
```

`Shape` may remain internal. The public API returns `&[usize]` rather than exposing a public shape type.

## 7. Shape Rules

### 7.1 Scalar

`shape == []` means a scalar tensor.

```rust
let x = Tensor::scalar(3.0);
assert_eq!(x.shape(), &[]);
assert_eq!(x.len(), 1);
assert!(x.is_scalar());
```

### 7.2 Vector and matrix

- `[n]` is a one-dimensional vector with `n` elements.
- `[m, n]` is a two-dimensional row-major matrix.

### 7.3 Higher rank

Ranks greater than 2 are supported at runtime using `Vec<usize>`. Public APIs must not require special type parameters per rank.

### 7.4 Zero-sized dimensions

For `0.1.0`, zero-sized dimensions should be rejected by constructors unless a later decision explicitly supports empty tensors.

Reasoning:

- empty tensors complicate `mean`, `min`, `max`, JSON shape inference, and scalar broadcasting;
- `matten` is a PoC-oriented crate, not an exact NumPy compatibility layer;
- support can be added later with an explicit empty-tensor RFC.

## 8. Data Lifecycle

Shape lifecycle:

1. Caller provides shape.
2. Internal validator checks rank, zero dimensions, and product overflow.
3. Constructor verifies product equals data length or allocates exactly product elements.
4. Transformations create a new shape and validate element count or axis bounds.
5. Result tensor stores a valid shape only.

Invalid shapes must never be stored in a public `Tensor` value.

## 9. Events

Conceptual validation events:

| Event | Trigger | Required result |
|---|---|---|
| `shape_received` | constructor/reshape/broadcast/slice | checked before use |
| `rank_checked` | all shape APIs | reject if rank exceeds accepted limit |
| `product_checked` | allocation/reshape | no overflow |
| `axis_checked` | transpose/swap/reduction/slice | out-of-bounds rejected or panic per API zone |

No public event bus is introduced.

## 10. Store Access

Shape is private storage. Public access:

```rust
pub fn shape(&self) -> &[usize];
pub fn ndim(&self) -> usize;
pub fn len(&self) -> usize;
pub fn is_scalar(&self) -> bool;
pub fn is_vector(&self) -> bool; // ndim() == 1
pub fn is_matrix(&self) -> bool; // ndim() == 2
```

`shape()` returns a shared slice, not a mutable reference. Users cannot mutate shape without going through validation.

## 11. Public API

```rust
impl Tensor {
    pub fn shape(&self) -> &[usize];
    pub fn ndim(&self) -> usize;
    pub fn len(&self) -> usize;
    pub fn is_scalar(&self) -> bool;
    pub fn is_vector(&self) -> bool;
    pub fn is_matrix(&self) -> bool;
}
```

Expected behavior:

- `ndim() == shape().len()`.
- `len()` is the checked logical element count stored or derived safely.
- `is_empty()` is **not** part of the `0.1.0` API: zero-sized dims are rejected and a scalar has `len() == 1`, so it would always be false. It is deferred to a future zero-sized-tensor RFC.
- `is_scalar()` is true exactly when shape is empty.
- `is_vector()` / `is_matrix()` are non-allocating conveniences: `ndim() == 1` and `ndim() == 2` respectively.

## 12. Internal Design

### 12.1 Checked product

```rust
pub(crate) fn checked_shape_len(shape: &[usize]) -> Result<usize, MattenError> {
    let mut len = 1usize;
    for &dim in shape {
        if dim == 0 {
            return Err(MattenError::shape("zero-sized dimensions are not supported in matten 0.1"));
        }
        len = len.checked_mul(dim)
            .ok_or_else(|| MattenError::shape("shape product overflow"))?;
    }
    Ok(len)
}
```

The real implementation should use structured error fields rather than string-only errors.

### 12.2 Rank limit

Recommended initial rank limit: 8.

Rationale:

- protects against accidental extremely nested JSON and parser abuse;
- covers common PoC usage;
- can be relaxed later because shape is already stored as `Vec<usize>`.

Boundary APIs must enforce rank limits. Internal panic-zone APIs may panic on invalid rank if called directly.

### 12.3 Row-major flattening

For shape `[d0, d1, ..., dk]`, row-major index mapping is:

```text
flat = i0 * stride0 + i1 * stride1 + ... + ik * stridek
stride_j = product(dims[j+1..])
```

Phase 1 tensors store elements in row-major order.

## 13. Error Handling

- Constructor mismatch may panic in `Tensor::new` but must use checked product first.
- `try_new` must return `MattenError::Shape` or equivalent.
- Axis errors in Result-zone APIs return `Err`.
- Axis errors in panic-zone APIs may panic with operation and axis included.

## 14. Testing

- scalar shape tests;
- vector/matrix/higher-rank product tests;
- product overflow tests;
- zero-dimension rejection tests;
- rank-limit tests;
- row-major indexing tests;
- property tests comparing flatten/unflatten index round trips.

## 15. Acceptance Criteria

- Shape `[]` is accepted as scalar and length 1.
- Zero-sized dimensions are either rejected or explicitly changed by later RFC.
- Checked product is used everywhere before allocation.
- Shape is immutable through public API.
- Row-major order is documented and tested.
