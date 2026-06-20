# RFC-007: Reshape, Axis Operations, and Indexing

> RFC status: Implemented (0.4.0)
> Project: `matten`  
> Target audience: library implementers, maintainers, reviewers  
> Design level: external design first; internal design where needed  
> Last updated: 2026-06-18

## 1. Summary

This RFC defines `reshape`, `try_reshape`, `flatten`, `transpose`, `t`, `swap_axes`, and basic element access. In Phase 1, all returned tensors are owned and lifetime-free. Operations may copy/materialize internally to preserve contiguous row-major storage and avoid exposing view lifetimes.

## 2. Motivation

Array work requires frequent shape transformation. Rust libraries often expose view and stride complexity for performance. `matten` deliberately hides that complexity in Phase 1 by returning owned tensors and copying where necessary.

## 3. Goals

- Provide common shape transformations.
- Preserve row-major logical order.
- Avoid public view/lifetime types.
- Define axis bounds behavior.
- Define indexing helper behavior.
- Keep Phase 1 implementation simple and correct.

## 4. Non-goals

- No lazy views in Phase 1.
- No mutation-through-view.
- No advanced indexing masks.
- No negative axes in `0.1.0`.
- No stride-exposing public API.

## 5. Cargo Features

| Feature | Behavior |
|---|---|
| default | Owned materializing transformations. |
| `dynamic` | May share storage and update view metadata if RFC-012 approves. |

Public method names should remain stable across features where possible.

## 6. Data Model

Phase 1 input/output:

```rust
Tensor { data: Vec<f64>, shape: Vec<usize> }
```

All outputs are independent owned tensors. `reshape` may clone data without reordering. `transpose` and `swap_axes` generally reorder data into row-major storage for the new shape.

## 7. Data Lifecycle

### 7.1 Reshape

1. Validate product(new_shape) equals `self.len()`.
2. Clone data in row-major flat order.
3. Store new shape.
4. Return new tensor.

### 7.2 Flatten

1. Clone data.
2. Set shape to `[self.len()]`.

### 7.3 Transpose

For rank 2, transpose matrix axes. For rank >2, default `transpose()` reverses all axes unless a later RFC narrows it. This RFC recommends:

```text
shape [d0, d1, d2] -> [d2, d1, d0]
```

`t()` is an alias for `transpose()`.

### 7.4 Swap axes

1. Validate axis indices.
2. Produce new shape with axes swapped.
3. Reorder data so output is row-major under new shape.

## 8. Events

| Event | Required behavior |
|---|---|
| reshape requested | validate product equality |
| axis operation requested | validate axis bounds |
| output materialized | produce independent Phase 1 tensor |
| flat index requested | validate bounds for Result-zone or panic for direct indexing |

No public callbacks.

## 9. Store Access

Operations read from the source tensor's private storage and write a new private storage vector. Phase 1 outputs must not alias source storage.

Public data access may include:

```rust
pub fn get(&self, indices: &[usize]) -> Option<f64>;
pub fn get_flat(&self, index: usize) -> Option<f64>;
```

Returning `f64` by value is fine for Phase 1. Mutable access is deferred.

## 10. Public API

```rust
impl Tensor {
    pub fn reshape(&self, new_shape: &[usize]) -> Tensor;
    pub fn try_reshape(&self, new_shape: &[usize]) -> Result<Tensor, MattenError>;
    pub fn flatten(&self) -> Tensor;
    pub fn transpose(&self) -> Tensor;
    pub fn t(&self) -> Tensor;
    pub fn swap_axes(&self, axis1: usize, axis2: usize) -> Tensor;

    pub fn get(&self, indices: &[usize]) -> Option<f64>;
    pub fn get_flat(&self, index: usize) -> Option<f64>;
}
```

`get` returns `None` for rank mismatch or out-of-bounds indices. It is not a parsing boundary, but `Option` is more ergonomic than panicking for element lookup.

## 11. Internal Design

### 11.1 Coordinate helpers

```rust
pub(crate) fn strides_for(shape: &[usize]) -> Vec<usize>;
pub(crate) fn coord_to_flat(coord: &[usize], shape: &[usize]) -> Option<usize>;
pub(crate) fn flat_to_coord(flat: usize, shape: &[usize]) -> Vec<usize>;
```

### 11.2 Axis reorder

A generic axis-permutation helper can power both `transpose` and `swap_axes`:

```rust
pub(crate) fn permute_axes(t: &Tensor, permutation: &[usize]) -> Result<Tensor, MattenError>;
```

For each source flat index:

1. Convert to source coordinate.
2. Permute coordinate.
3. Convert to destination flat index.
4. Write value.

This is not the fastest method, but it is simple and correct. Later optimization may improve internals without changing API.

## 12. Error and Panic Behavior

- `reshape` panics on product mismatch.
- `try_reshape` returns `Err`.
- `swap_axes` panics on axis out of bounds.
- `get` returns `None` rather than panic.

Example panic:

```text
matten shape error in reshape: cannot reshape tensor with 6 elements from shape [2, 3] into shape [4, 2] requiring 8 elements
```

## 13. Testing

- reshape preserves flat order;
- invalid reshape panic and `try_reshape` error;
- flatten preserves data;
- transpose 2D expected output;
- transpose higher-rank expected coordinate mapping;
- swap axes expected mapping;
- output independence in Phase 1;
- `get` rank mismatch and bounds cases.

## 14. Acceptance Criteria

- Shape transforms never expose lifetimes.
- Phase 1 transformed tensors are independent.
- Axis operations preserve logical values correctly.
- `get` is safe and non-panicking.
- Panic messages include operation, source shape, and target/axis context.
