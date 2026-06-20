# RFC-004: Construction and Conversion APIs

> RFC status: Implemented (0.2.0)
> Project: `matten`  
> Target audience: library implementers, maintainers, reviewers  
> Design level: external design first; internal design where needed  
> Last updated: 2026-06-18

## 1. Summary

This RFC defines Phase 1 tensor constructors and standard conversions: `new`, `try_new`, `from_vec`, `zeros`, `ones`, `full`, `scalar`, `arange`, `From<Vec<f64>>`, `From<Vec<Vec<f64>>>`, and conversion back to flat and nested vectors. Constructors must preserve the DX-first panic style for trusted local code while exposing `try_*` forms for boundary-like or user-driven construction.

## 2. Motivation

The first experience with `matten` must be simple. Users should be able to create a tensor from a flat vector and a shape in one line, or from nested vectors when writing matrix examples. At the same time, application code needs safe constructors when shapes originate from user input.

## 3. Goals

- Define construction APIs for Phase 1.
- Define ownership and copying behavior.
- Define nested-vector shape inference.
- Define conversion behavior and failure cases.
- Keep conversions narrow and obvious.
- Avoid generic dtype constructors in Phase 1.

## 4. Non-goals

- No arbitrary nested ragged data support.
- No dtype inference beyond `f64` in Phase 1.
- No ndarray/nalgebra/candle interop in this RFC.
- No random-number constructor unless a later feature RFC approves `rand`.
- No file parsing; that belongs to RFC-009.

## 5. Cargo Features

| Feature | Effect |
|---|---|
| default | All numeric constructors and vector conversions. |
| `serde` | No change to constructors; serialization covered by RFC-009. |
| `dynamic` | May add `Element` constructors in RFC-011. |
| future `rand` | May add `random` or `randn` constructors. |

## 6. Data Model

Phase 1 constructor target:

```rust
pub struct Tensor {
    data: Vec<f64>,
    shape: Vec<usize>,
}
```

The constructor invariant is:

```text
data.len() == checked_product(shape)
```

The data vector is stored in row-major order.

## 7. Data Lifecycle

### 7.1 `new`

1. Caller transfers ownership of `Vec<f64>`.
2. Shape is validated using RFC-003.
3. Data length is compared to shape product.
4. On success, `Tensor` owns the vector without additional copying.
5. On failure, the convenience constructor panics with an actionable message.

### 7.2 `try_new`

Same as `new`, but returns `Result<Tensor, MattenError>`.

### 7.3 Fill constructors

1. Shape is validated.
2. Product length is calculated.
3. A new vector of that length is allocated and filled.

### 7.4 Nested conversion

`Vec<Vec<f64>>` is accepted only when rectangular. Ragged input is rejected or panics depending on API form.

## 8. Events

Conceptual events:

| Event | Required behavior |
|---|---|
| constructor receives shape | validate product and rank |
| constructor receives nested rows | validate rectangularity |
| fill constructor allocates | allocate exactly product elements |
| conversion consumes tensor | no extra copy for flat `Vec<f64>` when consuming |
| borrowed conversion requested | copy into owned output |

No public event callbacks are introduced.

## 9. Store Access

Construction writes to in-memory storage only. External stores are out of scope.

Public accessors related to conversion:

```rust
pub fn as_slice(&self) -> &[f64];
pub fn to_vec(&self) -> Vec<f64>;
pub fn into_vec(self) -> Vec<f64>;
```

`as_slice()` exposes Phase 1 contiguous storage as read-only. It must not expose mutable access in `0.1.0` unless mutation APIs are defined later.

## 10. Public API

```rust
impl Tensor {
    pub fn new(data: Vec<f64>, shape: &[usize]) -> Tensor;
    pub fn try_new(data: Vec<f64>, shape: &[usize]) -> Result<Tensor, MattenError>;

    pub fn from_vec(data: Vec<f64>) -> Tensor;
    pub fn zeros(shape: &[usize]) -> Tensor;
    pub fn ones(shape: &[usize]) -> Tensor;
    pub fn full(shape: &[usize], value: f64) -> Tensor;
    pub fn scalar(value: f64) -> Tensor;

    pub fn arange(start: f64, end: f64, step: f64) -> Tensor;
    pub fn try_arange(start: f64, end: f64, step: f64) -> Result<Tensor, MattenError>;

    pub fn as_slice(&self) -> &[f64];
    pub fn to_vec(&self) -> Vec<f64>;
    pub fn into_vec(self) -> Vec<f64>;
}

impl From<Vec<f64>> for Tensor;
impl From<Tensor> for Vec<f64>;
impl From<&Tensor> for Vec<f64>;

impl From<Vec<Vec<f64>>> for Tensor;
impl TryFrom<Tensor> for Vec<Vec<f64>>;
```

### 10.1 `arange`

`arange(start, end, step)` follows half-open semantics.

- `step == 0.0` panics (`arange`) / returns `Err` (`try_arange`).
- non-finite `start`, `end`, or `step` is rejected (panic / `Err`).
- If `step > 0.0`, values continue while `x < end`.
- If `step < 0.0`, values continue while `x > end`.
- the computed element count is validated with checked arithmetic before allocation; an excessive count panics (`arange`) or returns `MattenError::Allocation` (`try_arange`).
- Floating-point accumulation behavior must be documented as approximate.

## 11. Internal Design

### 11.1 Avoid duplicate validation logic

All constructors should call a shared validator.

```rust
pub(crate) fn validate_data_shape(data_len: usize, shape: &[usize]) -> Result<(), MattenError>;
```

`new` should be a thin panic wrapper around `try_new`.

### 11.2 Nested vector validation

```rust
fn flatten_rectangular(rows: Vec<Vec<f64>>) -> Result<(Vec<f64>, Vec<usize>), MattenError> {
    let row_count = rows.len();
    let col_count = rows.first().map(|r| r.len()).unwrap_or(0);
    // zero-row behavior follows zero-sized tensor policy; likely reject in 0.1.
    // validate every row.len() == col_count.
    // flatten in row-major order.
}
```

If zero-sized dimensions are rejected, empty outer vectors are not accepted in `From<Vec<Vec<f64>>>`; panic wrapper may panic.

### 11.3 Conversion to nested vector

`TryFrom<Tensor> for Vec<Vec<f64>>` succeeds only when `tensor.ndim() == 2`. It consumes the tensor and can move values without cloning. For borrowed nested output, use a named method later if needed.

## 12. Error Handling

- `Tensor::new` panics on length mismatch.
- `Tensor::try_new` returns `MattenError`.
- `From<Vec<Vec<f64>>>` may panic on ragged input because `From` cannot fail.
- Prefer `TryFrom<Vec<Vec<f64>>>` if users need recoverable ragged-input behavior; due to trait overlap concerns, this may be a named method:

```rust
pub fn try_from_rows(rows: Vec<Vec<f64>>) -> Result<Tensor, MattenError>;
```

This RFC recommends adding `try_from_rows` to avoid overloading `From` with panic semantics in application boundaries.

## 13. Testing

- `new` success/failure tests.
- `try_new` error tests.
- scalar constructor tests.
- fill constructor length tests.
- `arange` positive/negative/zero-step tests.
- rectangular nested vector tests.
- ragged nested vector tests.
- consuming flat conversion must avoid copy where possible.

## 14. Acceptance Criteria

- A beginner can create common tensors in one line.
- `new` panic messages are clear.
- `try_new` exists before boundary APIs rely on user-provided shapes.
- `From<Vec<f64>>` creates shape `[len]`.
- `From<Vec<Vec<f64>>>` creates shape `[rows, cols]` for rectangular input.
- Flat conversion back to `Vec<f64>` is implemented.
