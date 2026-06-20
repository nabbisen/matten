# RFC-006: Broadcasting and Element-Wise Operators

> RFC status: Implemented (0.3.0)
> Project: `matten`  
> Target audience: library implementers, maintainers, reviewers  
> Design level: external design first; internal design where needed  
> Last updated: 2026-06-18

## 1. Summary

This RFC defines NumPy-style right-aligned broadcasting and element-wise arithmetic operators for Phase 1 `f64` tensors. The `*` operator remains element-wise; matrix multiplication is explicit and covered in RFC-010.

## 2. Motivation

Broadcasting is one of the main reasons NumPy feels productive. `matten` should let users write `&a + &b` naturally without manually reshaping common vector and scalar operands. At the same time, broadcasting bugs can silently corrupt results. Therefore this RFC defines exact compatibility rules and testing obligations.

## 3. Goals

- Implement `Add`, `Sub`, `Mul`, `Div`, and `Neg` for borrowed tensors.
- Support scalar operations with `f64` where Rust coherence permits.
- Define right-aligned broadcasting.
- Allocate a new result tensor for each operation.
- Preserve operands unchanged.
- Keep implementation internal and non-generic in Phase 1.

## 4. Non-goals

- No BLAS integration.
- No matrix product via `*`.
- No lazy expression graph.
- No public broadcast iterator type.
- No custom dtype arithmetic in Phase 1.

## 5. Cargo Features

| Feature | Behavior |
|---|---|
| default | `f64` element-wise ops and broadcasting. |
| `dynamic` | May reuse shape rules; value coercion governed by RFC-011. |

`dynamic` must not change Phase 1 examples unless explicitly documented.

## 6. Data Model

Broadcasting uses only shapes and logical element access.

```rust
left: Tensor(shape = L, data = Vec<f64>)
right: Tensor(shape = R, data = Vec<f64>)
result: Tensor(shape = broadcast(L, R), data = Vec<f64>)
```

Result data is row-major in result shape.

## 7. Broadcasting Rules

Shapes are compatible when aligned from the right and each dimension pair satisfies one of:

- equal dimensions;
- one dimension is `1`;
- one side has no dimension because it has lower rank.

Examples:

| Left | Right | Result |
|---|---|---|
| `[]` | `[2, 3]` | `[2, 3]` |
| `[4]` | `[3, 4]` | `[3, 4]` |
| `[3, 1]` | `[1, 4]` | `[3, 4]` |
| `[2, 3]` | `[2]` | incompatible |

## 8. Data Lifecycle

1. Operator receives borrowed operands.
2. Shapes are checked for broadcast compatibility.
3. Result shape is computed.
4. Result vector is allocated.
5. For each result coordinate, source coordinates are mapped to left/right using broadcast rules.
6. Operation is applied.
7. New owned result tensor is returned.

No operand is mutated or consumed.

## 9. Events

Conceptual events:

| Event | Required behavior |
|---|---|
| broadcast requested | compute result shape or fail |
| incompatible dimension found | panic in operator with shapes and operation |
| result allocation requested | use checked length |
| element operation executed | produce `f64` following IEEE 754 |

No public event system is introduced.

## 10. Store Access

Only in-memory tensor storage is read. A new in-memory result store is allocated. Operators do not read or write external stores.

## 11. Public API

```rust
impl std::ops::Add for &Tensor {
    type Output = Tensor;
    fn add(self, rhs: &Tensor) -> Tensor;
}

impl std::ops::Sub for &Tensor { type Output = Tensor; /* ... */ }
impl std::ops::Mul for &Tensor { type Output = Tensor; /* ... */ }
impl std::ops::Div for &Tensor { type Output = Tensor; /* ... */ }
impl std::ops::Neg for &Tensor { type Output = Tensor; /* ... */ }
```

Scalar forms:

```rust
impl std::ops::Add<f64> for &Tensor { type Output = Tensor; }
impl std::ops::Sub<f64> for &Tensor { type Output = Tensor; }
impl std::ops::Mul<f64> for &Tensor { type Output = Tensor; }
impl std::ops::Div<f64> for &Tensor { type Output = Tensor; }

// scalar-on-left forms are legal: `&Tensor` is a local type in the trait's
// parameter position; only a generic blanket impl would violate coherence
impl std::ops::Add<&Tensor> for f64 { type Output = Tensor; }
impl std::ops::Sub<&Tensor> for f64 { type Output = Tensor; }
impl std::ops::Mul<&Tensor> for f64 { type Output = Tensor; }
impl std::ops::Div<&Tensor> for f64 { type Output = Tensor; }
```

Reverse scalar forms such as `1.0 + &tensor` ARE implemented for `0.1.0`. The earlier claim that orphan rules forbid them was incorrect: a concrete `impl Add<&Tensor> for f64` is permitted because `&Tensor` is a local type in the trait's parameter position. Only a generic blanket `impl<T> Add<&Tensor> for T` would violate coherence. All four ops are provided in both directions.

## 12. Internal Design

### 12.1 Broadcast shape helper

```rust
pub(crate) fn broadcast_shape(left: &[usize], right: &[usize]) -> Result<Vec<usize>, MattenError>;
```

### 12.2 Index mapping

For each result flat index:

1. Convert flat index to result multidimensional coordinate.
2. Map each coordinate to left/right coordinate:
   - if operand dimension is missing, use scalar coordinate;
   - if operand dimension is `1`, use `0`;
   - otherwise use result coordinate.
3. Convert operand coordinate to flat row-major index.

Optimization may avoid allocating coordinates per element, but correctness comes first.

### 12.3 Avoid expanded intermediates

Implementation should not materialize a broadcast-expanded copy of an operand before applying the operation. It should write directly to the result vector.

## 13. Error and Panic Behavior

Operator shape mismatch may panic:

```text
matten broadcast error in add: shapes [2, 3] and [2] are not compatible
```

Potential allocation overflow during result shape calculation should panic in operators but must be produced by checked helpers internally.

Division by zero follows IEEE 754 `f64` behavior. It does not produce `MattenError` in Phase 1.

The internal `broadcast_shape` helper returns `MattenError::Broadcast { left, right }` on incompatible shapes; operators panic-format that error. Broadcasting failure is never folded into `Shape`.

## 14. Testing

- Same-shape ops.
- Scalar broadcasting.
- Vector-to-matrix broadcasting.
- 3D broadcasting.
- Incompatible shapes.
- Panic message includes operation and both shapes.
- Division by zero behavior documented/tested.
- Property tests comparing selected cases to NumPy-generated golden outputs.

## 15. Acceptance Criteria

- `&a + &b` works for same and compatible shapes.
- Result shape is correct.
- Result data is row-major.
- Operators do not mutate operands.
- Incompatible shapes panic with helpful message.
- `*` remains element-wise.
