# RFC-010: Reductions, Basic Statistics, and Matrix Multiplication

> RFC status: Implemented (0.7.0)
> Project: `matten`  
> Target audience: library implementers, maintainers, reviewers  
> Design level: external design first; internal design where needed  
> Last updated: 2026-06-18

## 1. Summary

This RFC defines the first non-element-wise mathematical operations: whole-tensor reductions, optional axis reductions, vector dot product, and matrix multiplication. These operations remain simple, explicit, and Phase 1 `f64`-oriented. `*` remains element-wise; matrix multiplication uses `matmul` or `dot`.

## 2. Motivation

A tensor library is not useful with only element-wise operations. Users need `sum`, `mean`, and matrix multiplication for basic algorithms. At the same time, `matten` should not become a full linear algebra library competing with `nalgebra` or BLAS-backed crates.

## 3. Goals

- Add whole-tensor reductions.
- Define NaN/Inf behavior.
- Add explicit `dot` and `matmul`.
- Keep algorithms straightforward and readable.
- Avoid advanced linear algebra scope creep.

## 4. Non-goals

- No LU/QR/SVD/eigen decomposition.
- No BLAS/LAPACK dependency.
- No GPU acceleration.
- No automatic differentiation.
- No optimized tensor contraction API.

## 5. Cargo Features

These APIs belong to the default numeric feature set.

Under `dynamic`, numeric-compatible behavior may be added only after RFC-011 defines coercion. Non-coercible values must not be silently ignored.

## 6. Data Model

All Phase 1 operations read `f64` values from row-major storage.

Reduction outputs:

- whole-tensor scalar reductions return `f64`;
- axis reductions return `Tensor`;
- vector dot returns scalar tensor or `f64`? This RFC chooses `Tensor` for shape consistency in `dot`, while allowing a named scalar helper later.

Recommended:

```rust
pub fn dot(&self, rhs: &Tensor) -> Tensor;
pub fn matmul(&self, rhs: &Tensor) -> Tensor;
```

For vector dot, result is `Tensor::scalar(value)`.

## 7. Data Lifecycle

### 7.1 Whole reductions

1. Iterate through flat storage.
2. Accumulate result.
3. Return scalar `f64`.

### 7.2 Axis reductions

1. Validate axis.
2. Compute output shape by removing axis.
3. Allocate result initialized to reduction identity.
4. Map source coordinates to result coordinates.
5. Accumulate.

### 7.3 Matmul

1. Validate operand ranks and dimensions.
2. Allocate result.
3. Use straightforward nested loops.
4. Return owned result.

## 8. Events

| Event | Required behavior |
|---|---|
| reduction requested | validate empty behavior if applicable |
| axis reduction requested | validate axis bounds |
| matmul requested | validate rank/dimension compatibility |
| result allocated | checked product |

No public event bus.

## 9. Store Access

Only in-memory tensor data is read. New result tensors are allocated for tensor outputs.

## 10. Public API

```rust
impl Tensor {
    pub fn sum(&self) -> f64;
    pub fn mean(&self) -> f64;
    pub fn min(&self) -> f64;
    pub fn max(&self) -> f64;

    pub fn sum_axis(&self, axis: usize) -> Tensor;
    pub fn mean_axis(&self, axis: usize) -> Tensor;

    pub fn dot(&self, rhs: &Tensor) -> Tensor;
    pub fn matmul(&self, rhs: &Tensor) -> Tensor;
}
```

Axis reductions may be deferred if `0.1.0` is too large, but whole reductions and at least `matmul` should be available before RC if mathematical examples need them.

## 11. Operation Semantics

### 11.1 NaN and Inf

Phase 1 follows Rust/IEEE 754 `f64` behavior.

- `sum` propagates `NaN` naturally.
- `mean` is `sum / len`.
- `min`/`max` behavior around `NaN` must be explicitly documented. This RFC recommends using `f64::min`/`f64::max` semantics only after confirming desired behavior; alternatively, panic or return `NaN` if any NaN exists.

Decision: for predictability, `min` and `max` should return `NaN` if any element is NaN. This is easy to document for beginners.

Implementer warning: do NOT use `iter().fold(f64::INFINITY, f64::min)` or `fold(f64::NEG_INFINITY, f64::max)`. `f64::min` / `f64::max` silently ignore `NaN` and would violate this rule. Detect `NaN` explicitly (e.g. short-circuit to `NaN` if any element is `NaN`).

### 11.2 Matmul cases

Minimum cases:

| Left | Right | Result |
|---|---|---|
| `[n]` | `[n]` | `[]` scalar tensor |
| `[m, n]` | `[n]` | `[m]` |
| `[m, n]` | `[n, p]` | `[m, p]` |
| `[n]` | `[n, p]` | `[p]` |

Higher-rank batched matmul is out of scope for `0.1.0`.

## 12. Internal Design

### 12.1 Matmul helper

```rust
pub(crate) fn matmul_2d(a: &Tensor, b: &Tensor) -> Result<Tensor, MattenError>;
```

Use plain loops first:

```text
for i in 0..m
  for j in 0..p
    acc = 0
    for k in 0..n
      acc += a[i,k] * b[k,j]
```

### 12.2 Axis reduction helper

A coordinate-mapping implementation is acceptable. Optimization can come later.

## 13. Error Handling

- Invalid matmul shapes may panic in `matmul`/`dot`.
- If later `try_matmul` is introduced, it returns `Result`.
- Axis out of bounds in `sum_axis`/`mean_axis` may panic with operation and axis.

Example:

```text
matten shape error in matmul: left shape [2, 3] cannot multiply right shape [4, 2]; expected left columns 3 to equal right rows 4
```

## 14. Testing

- reductions on simple vectors and matrices;
- NaN tests;
- matmul vector-vector, matrix-vector, vector-matrix, matrix-matrix;
- invalid matmul panic;
- axis reduction shape and values;
- golden tests against NumPy for selected cases.

## 15. Acceptance Criteria

- Basic mathematical examples can be written without external crates.
- `*` remains element-wise.
- `matmul` is explicit and documented.
- NaN behavior is documented.
- No advanced linear algebra APIs are introduced prematurely.
