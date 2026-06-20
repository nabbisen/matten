# RFC-019: Axis Reductions and Small Matrix Statistics

**Status:** Implemented (v0.14.0 (core); v0.15.0 (examples))  
**Target:** v0.15.x  
**Theme:** Sedan math maturity  
**Depends on:** RFC-003, RFC-006, RFC-010, RFC-018  
**Related handoff:** `019-axis-reductions-and-small-matrix-statistics-handoff.md`

## 1. Summary

This RFC adds axis-based reductions and small statistics helpers to the numeric `Tensor` API.

The current core supports whole-tensor reductions and basic matrix operations. The next natural Sedan-like extension is to compute along rows or columns without forcing users into complex iterator code.

This RFC is intentionally modest. It does not introduce advanced linear algebra, dataframe aggregation, or ML training.

## 2. Goals

- Add ergonomic row/column/axis reductions.
- Keep API NumPy-familiar but Rust-simple.
- Support common PoC statistics.
- Preserve dynamic guard model.
- Avoid dataframe semantics.

## 3. Non-goals

- No grouped aggregation.
- No column names.
- No covariance/correlation matrix in this RFC unless implemented as an example.
- No regression, decomposition, inverse, determinant, eigen, QR, SVD, or Cholesky.
- No dynamic reductions.

## 4. External design

### 4.1 Axis reductions

Proposed API:

```rust
impl Tensor {
    pub fn sum_axis(&self, axis: usize) -> Tensor;
    pub fn mean_axis(&self, axis: usize) -> Tensor;
    pub fn min_axis(&self, axis: usize) -> Tensor;
    pub fn max_axis(&self, axis: usize) -> Tensor;
}
```

For a tensor with shape `[2, 3]`:

```text
sum_axis(0) -> shape [3]  // reduce rows
sum_axis(1) -> shape [2]  // reduce columns
```

### 4.2 Keepdims deferred

NumPy-style `keepdims` is useful but can wait.

Future API:

```rust
sum_axis_keepdims(axis)
```

not in this RFC unless implementation is trivial.

### 4.3 NaN policy

Use the existing NaN policy from RFC-010:

- `sum` propagates normal IEEE behavior;
- `mean` propagates NaN if included;
- `min` / `max` must return NaN if any reduced element is NaN.

Do not use `f64::min` / `f64::max` fold if it ignores NaN.

## 5. Data model

Only numeric tensors are supported.

Output tensor shape is input shape with the target axis removed.

If input is scalar:

- `sum_axis(0)` is invalid because scalar has no axis;
- use whole-tensor `sum()` instead.

If input is 1D:

```text
shape [N], axis 0 -> scalar shape []
```

## 6. Data lifecycle

```text
input numeric Tensor
  -> validate axis
  -> compute output shape
  -> allocate output
  -> reduce values
  -> output numeric Tensor
```

Dynamic tensors reject at validation.

## 7. Events and observable behavior

Panics for internal math misuse:

- invalid axis;
- dynamic tensor input.

No `Result` API is required unless a future boundary-safe math mode is added.

Panic format:

```text
matten shape error in sum_axis: axis 2 out of bounds for shape [2, 3]
```

## 8. Store access

Reduction reads numeric storage and writes new numeric storage.

No views or borrowed outputs.

## 9. Public API proposal

```rust
impl Tensor {
    pub fn sum_axis(&self, axis: usize) -> Tensor;
    pub fn mean_axis(&self, axis: usize) -> Tensor;
    pub fn min_axis(&self, axis: usize) -> Tensor;
    pub fn max_axis(&self, axis: usize) -> Tensor;
}
```

Optional if scope allows:

```rust
pub fn var_axis(&self, axis: usize) -> Tensor;
pub fn std_axis(&self, axis: usize) -> Tensor;
```

Variance/stddev may be deferred if whole-tensor variance is still being matured.

## 10. Cargo feature impact

No new feature.

Axis reductions are default numeric core.

## 11. Internal design

### 11.1 Index strategy

For simplicity, implement using row-major flat indexing and shape strides.

Helper:

```rust
fn reduce_axis<F>(tensor: &Tensor, axis: usize, init: f64, f: F) -> Tensor
where
    F: Fn(f64, f64) -> f64;
```

But `min`/`max` may need special NaN handling.

### 11.2 Output shape

```rust
let mut out_shape = self.shape().to_vec();
out_shape.remove(axis);
```

If `out_shape` is empty, output is scalar shape `[]`.

### 11.3 Performance

Naive loops are acceptable. The project prioritizes DX and correctness over benchmark optimization.

## 12. Examples

Required examples:

```text
examples/27_axis_reductions.rs
examples/28_column_mean.rs
examples/29_row_scores.rs
```

Example concept:

```rust
let x = Tensor::from(vec![
    vec![1.0, 2.0, 3.0],
    vec![4.0, 5.0, 6.0],
]);

let column_means = x.mean_axis(0);
let row_sums = x.sum_axis(1);
```

## 13. Acceptance criteria

- Axis reductions work for 1D, 2D, and 3D tensors.
- Scalar axis misuse panics clearly.
- Invalid axis panics clearly.
- Dynamic tensors reject clearly.
- NaN behavior matches documented policy.
- Examples are simple and formula-like.

## 14. QA checklist

- [ ] 1D axis reduction tests
- [ ] 2D axis 0 and axis 1 tests
- [ ] 3D axis tests
- [ ] scalar invalid-axis tests
- [ ] invalid axis tests
- [ ] NaN min/max tests
- [ ] dynamic guard tests
- [ ] example smoke tests

## 15. Open questions

1. Should variance/stddev axis methods be included in v0.15 or deferred?
2. Should axis be allowed as negative index in future? Current answer: no.
3. Should `keepdims` be supported later?
