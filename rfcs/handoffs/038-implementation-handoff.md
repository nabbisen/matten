# RFC-038 Developer Handoff: Core Numeric Comfort APIs

**Project:** `matten`  
**RFC:** RFC-038  
**Handoff Kind:** Implementation Handoff  
**Implementation Level:** Core implementation with careful API/tests  
**Status:** Draft handoff for developer review  
**Applies To:** v0.20+ planning and implementation sequence  

---

## 0. Handoff Summary

This document turns RFC-038 into developer-executable work. It is not a replacement for the RFC. The RFC remains the design authority; this handoff translates it into implementation phases, PR boundaries, checks, and acceptance criteria.

## 1. Implementation Handoff

RFC-038 adds small core `Tensor` convenience APIs inspired by NumPy.

Target API set:

```rust
Tensor::linspace(start, end, count)
Tensor::try_linspace(start, end, count)

Tensor::eye(n)
Tensor::try_eye(n)

tensor.clip(min, max)
tensor.try_clip(min, max)

tensor.abs()
tensor.sqrt()
tensor.exp()
tensor.ln()

tensor.argmin()
tensor.argmax()
tensor.try_argmin()
tensor.try_argmax()

tensor.squeeze()
tensor.expand_dims(axis)
tensor.try_expand_dims(axis)
```

This is core `matten` work, not a companion.

---

## 2. Internal Design

### 2.1 Module placement

Recommended:

```text
crates/matten/src/
  tensor.rs
  creation.rs or constructors.rs
  math.rs or elementwise.rs
  shape_ops.rs
  reductions.rs
```

Use existing project structure if different.

### 2.2 Error behavior

For each API with invalid input, provide:

```text
panicking convenience form
try_* Result form
```

Examples:

```rust
clip(min, max)      // panic if min > max
try_clip(min, max)  // Result error if min > max
```

Invalid-argument failures use one pinned variant across all comfort APIs
(RFC-038 §5.2, architect Q9):

```rust
MattenError::InvalidArgument { operation: &'static str, argument: &'static str, message: String }
```

Use `InvalidArgument` for invalid local numeric arguments (e.g. `clip` with
`min > max`, `argmin` on a NaN-containing tensor). Do **not** use `Unsupported` for
these — `Unsupported` is reserved for unsupported tensor kind/feature/mode (e.g. a
dynamic tensor passed to a numeric-only API). Adding the variant is non-breaking
(`MattenError` is `#[non_exhaustive]`).

### 2.3 Dynamic behavior

If core is compiled with `dynamic`, numeric comfort APIs must reject dynamic tensors.

Recommended helper:

```rust
self.ensure_numeric("clip")?;
```

or equivalent.

Do not silently call `try_numeric()`.

### 2.4 Limits

Use `MattenLimits` for:

- `linspace(count)`;
- `eye(n)` product `n * n`;
- `expand_dims` rank check.

---

## 3. Task Breakdown / PR Plan

### PR-038-1: Constructors `linspace` and `eye`

Implement:

```rust
try_linspace
linspace
try_eye
eye
```

Acceptance:

```text
[ ] count == 0 handled
[ ] count == 1 behavior documented
[ ] n == 0 handled
[ ] allocation limit checked
[ ] examples added
```

### PR-038-2: Elementwise functions

Implement:

```rust
abs
sqrt
exp
ln
clip
try_clip
```

Acceptance:

```text
[ ] shape preserved
[ ] f64 NaN/Inf behavior documented
[ ] min > max tested for clip
[ ] dynamic unsupported tested where applicable
```

### PR-038-3: Argmin / argmax

Implement:

```rust
try_argmin
try_argmax
argmin
argmax
```

Policy:

```text
return flat row-major index
tie -> first occurrence
NaN -> Err / panic
```

Acceptance:

```text
[ ] tie behavior tested
[ ] NaN behavior tested
[ ] flat index documented
```

### PR-038-4: Squeeze / expand_dims

Implement:

```rust
squeeze
try_expand_dims
expand_dims
```

Acceptance:

```text
[ ] scalar behavior tested
[ ] [1, 3, 1] -> [3]
[ ] [1, 1] -> []
[ ] expand axis 0..=ndim valid
[ ] out-of-range axis tested
[ ] rank limit checked
```

### PR-038-5: Documentation and examples

Add examples:

```text
examples/core_comfort_00_linspace_eye.rs
examples/core_comfort_01_elementwise.rs
examples/core_comfort_02_argmin_argmax.rs
examples/core_comfort_03_squeeze_expand.rs
```

Acceptance:

```text
[ ] examples compile
[ ] no SciPy/Pandas language
[ ] docs say these are convenience APIs
```

---

## 4. Acceptance / QA Checklist

### Functional QA

```text
[ ] linspace exact endpoints for count >= 2
[ ] linspace count 1 documented
[ ] eye diagonal/off-diagonal correct
[ ] elementwise shape preserved
[ ] clip bounds correct
[ ] argmin/argmax row-major index
[ ] squeeze removes singleton axes
[ ] expand_dims inserts singleton axis
```

### Error QA

```text
[ ] try_* APIs return MattenError
[ ] panicking APIs have actionable messages
[ ] NaN argmin/argmax policy enforced
[ ] dynamic tensors rejected
[ ] limits checked before allocation
```

### CI

```bash
cargo fmt --all --check
cargo clippy -p matten --all-targets --all-features -- -D warnings
cargo test -p matten --all-targets --all-features
cargo test -p matten --doc --all-features
cargo check -p matten --examples --all-features
```

### Public API QA

```text
[ ] no generic Tensor<T>
[ ] no dtype system
[ ] no heavy dependency
[ ] public API snapshot updated
```

---

## 5. Do Not Include

- random number generation;
- ufunc framework;
- var/std;
- stack/concatenate;
- quantile/histogram;
- inverse/determinant;
- dataframe operations.
