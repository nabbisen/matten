# RFC-038: Core Numeric Comfort APIs

**Status:** Implemented (v0.20.9–v0.20.12) — elementwise (v0.20.9), selection (v0.20.10), creation (v0.20.11), and shape (v0.20.12) bands shipped.  
**Target Release:** v0.20.x or v0.21.0  
**Related:** RFC-019, RFC-020, RFC-039, RFC-040, RFC-041  
**Scope:** Small NumPy-inspired APIs for core `matten::Tensor`

---

## 1. Summary

This RFC proposes a small set of core APIs to make `matten` more comfortable for common PoC numerical work.

The goal is not to clone NumPy.

The goal is:

```text
small familiar Tensor conveniences
  + no heavy dependencies
  + clear shape behavior
  + easy documentation
  + Sedan-first philosophy preserved
```

Proposed v0.20/v0.21 core set:

```rust
Tensor::linspace(start, end, count)
Tensor::eye(n)

tensor.clip(min, max)
tensor.abs()
tensor.sqrt()
tensor.exp()
tensor.ln()

tensor.argmin()
tensor.argmax()

tensor.squeeze()
tensor.expand_dims(axis)
```

---

## 2. Motivation

`matten` already supports creation, shape operations, arithmetic, reductions, and matrix basics. Users doing small numerical PoCs still expect familiar helpers:

- create evenly spaced samples;
- create identity matrices;
- apply common elementwise math;
- clip values;
- locate min/max index;
- add/remove singleton axes.

These are low-risk and high-value if kept small.

---

## 3. Inclusion Rule

A core comfort API may be added only if:

```text
[ ] it operates directly on Tensor
[ ] it has no heavy dependency
[ ] it is teachable in a few lines
[ ] it has obvious shape behavior
[ ] it does not create a new domain model
[ ] it does not require dtype/generic Tensor<T>
[ ] it does not imply SciPy/Pandas completeness
```

---

## 4. Proposed APIs

### 4.1 `linspace`

```rust
impl Tensor {
    pub fn linspace(start: f64, end: f64, count: usize) -> Tensor;
    pub fn try_linspace(start: f64, end: f64, count: usize) -> Result<Tensor, MattenError>;
}
```

Behavior:

- output shape `[count]`;
- includes both `start` and `end` when `count >= 2`;
- `count == 0` is invalid because core rejects zero-sized dimensions;
- `count == 1` returns `[start]`.

### 4.2 `eye`

```rust
impl Tensor {
    pub fn eye(n: usize) -> Tensor;
    pub fn try_eye(n: usize) -> Result<Tensor, MattenError>;
}
```

Behavior:

- shape `[n, n]`;
- diagonal = 1.0;
- off-diagonal = 0.0;
- `n == 0` invalid.

Optional future:

```rust
eye_rect(rows, cols)
```

Do not include rectangular eye in the first set unless needed.

### 4.3 Elementwise Math

```rust
impl Tensor {
    pub fn abs(&self) -> Tensor;
    pub fn sqrt(&self) -> Tensor;
    pub fn exp(&self) -> Tensor;
    pub fn ln(&self) -> Tensor;
    pub fn clip(&self, min: f64, max: f64) -> Tensor;
    pub fn try_clip(&self, min: f64, max: f64) -> Result<Tensor, MattenError>;
}
```

Behavior:

- shape preserved;
- data transformed elementwise;
- normal `f64` NaN/Inf behavior;
- `clip(min, max)` panics if `min > max`;
- `try_clip(min, max)` returns `Err(MattenError::InvalidArgument { operation: "clip", argument: "min/max", message: "min must be <= max" })` if `min > max`.

### 4.4 `argmin` / `argmax`

```rust
impl Tensor {
    pub fn argmin(&self) -> usize;
    pub fn argmax(&self) -> usize;
    pub fn try_argmin(&self) -> Result<usize, MattenError>;
    pub fn try_argmax(&self) -> Result<usize, MattenError>;
}
```

Behavior:

- returns flat row-major index;
- tie returns first occurrence;
- if any NaN is present, `try_argmin`/`try_argmax` return
  `Err(MattenError::InvalidArgument { operation: "argmin", argument: "self", message: "argmin is undefined for tensors containing NaN" })`
  (and the analogous message for `argmax`), and the panicking forms panic with the
  same context. See the NaN house policy in §5.1.

### 4.5 `squeeze`

```rust
impl Tensor {
    pub fn squeeze(&self) -> Tensor;
}
```

Behavior:

- removes all axes with dimension `1`;
- scalar remains scalar;
- `[1, 3, 1] -> [3]`;
- `[1, 1] -> []`.

Optional future:

```rust
squeeze_axis(axis)
```

### 4.6 `expand_dims`

```rust
impl Tensor {
    pub fn expand_dims(&self, axis: usize) -> Tensor;
    pub fn try_expand_dims(&self, axis: usize) -> Result<Tensor, MattenError>;
}
```

Behavior:

- inserts dimension `1`;
- axis may be from `0..=ndim`;
- `[3]`, axis 0 -> `[1, 3]`;
- `[3]`, axis 1 -> `[3, 1]`.

---

## 5. Dynamic Behavior

These APIs are numeric APIs.

If called on a dynamic tensor:

- panic-zone methods may panic with `Unsupported`;
- try methods return `MattenError::Unsupported`.

They must not silently coerce dynamic tensors.

Users should call:

```rust
dynamic.try_numeric()?
```

first.

### 5.1 NaN house policy (architect ruling, RFC-033–042 review Q6)

```text
- APIs whose result is undefined or misleading in the presence of NaN return
  Err in their try_* forms and panic in their convenience forms.
- APIs that are ordinary f64 reductions follow f64 propagation behavior,
  provided the behavior is documented.
```

Application in this RFC:

```text
argmin / argmax:
  NaN makes "index of minimum/maximum" ill-defined.
  try_argmin / try_argmax return Err(MattenError::InvalidArgument { ... }).
  argmin / argmax panic with the same message.
```

Ordinary `f64` reductions (e.g. `var`/`std` in RFC-040) follow `f64` propagation:
NaN input produces NaN output. This split is deliberate — selection/index-producing
APIs require an ordering decision that NaN breaks, while numeric reductions can
preserve `f64` NaN propagation.

### 5.2 Error variant for invalid arguments (architect ruling, RFC-033–042 review Q9)

Invalid local numeric arguments use a dedicated variant rather than `Unsupported`:

```rust
#[non_exhaustive]
pub enum MattenError {
    // ... existing variants ...
    InvalidArgument {
        operation: &'static str,
        argument: &'static str,
        message: String,
    },
}
```

Distinction:

```text
Unsupported       -> operation is not supported for this tensor kind/feature/mode
                     (e.g. dynamic tensor passed to a numeric-only API)
InvalidArgument   -> operation is supported, but the supplied argument/value is invalid
                     (e.g. clip with min > max; argmin on a tensor containing NaN)
```

Since `MattenError` is `#[non_exhaustive]`, adding this variant is non-breaking. All
comfort APIs in this RFC use this single error path for invalid-argument failures.

### 5.3 Module placement (architect ruling, RFC-033–042 review Q8)

These methods are added in new modules rather than appended to a near-threshold
file (`math.rs` is already close to the 300-ELOC "consider splitting" line):

```text
creation.rs       linspace, eye
elementwise.rs    abs, sqrt, exp, ln, clip
shape_ops.rs      squeeze, expand_dims
selection.rs      argmin, argmax   (or an existing reductions module if a close fit)
```

Follow existing module naming where a close equivalent already exists, but do not
push a near-threshold file over the limit.

---

## 6. Documentation Requirements

Each API needs:

- rustdoc example;
- shape behavior;
- NaN/Inf behavior where relevant;
- panic vs Result behavior;
- dynamic behavior.

Examples must stay simple.

---

## 7. Tests

Required tests:

```text
linspace count 0/1/2/N
eye valid/zero/large limit
clip normal/min>max/NaN
abs/sqrt/exp/ln shape preservation
argmin/argmax ties
argmin/argmax NaN policy
squeeze scalar/vector/matrix/N-D
expand_dims axis bounds
dynamic unsupported behavior, if dynamic enabled
```

---

## 8. Non-goals

- No dtype system.
- No generic Tensor<T>.
- No ufunc framework.
- No broadcasting function wrappers beyond existing operators.
- No SciPy functions.
- No dataframe operations.
- No heavy dependencies.
- No random number generation in this RFC.

---

## 9. Acceptance Criteria

```text
[ ] API set remains small
[ ] no heavy dependency added
[ ] docs fit the Sedan-first philosophy
[ ] dynamic tensors are not silently coerced
[ ] NaN/Inf policy documented
[ ] shape behavior tested
[ ] examples compile in CI
```
