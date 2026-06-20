# RFC-018: Shape, Allocation, and Resource Safety Limits

**Status:** Implemented (v0.14.0)  
**Target:** v0.14.x  
**Theme:** Safety and availability  
**Depends on:** RFC-001, RFC-003, RFC-004, RFC-006, RFC-009  
**Related handoff:** `018-shape-allocation-and-resource-safety-limits-handoff.md`

## 1. Summary

This RFC defines resource-safety limits for shape calculations, allocation-heavy constructors, broadcasting, range generation, and parser boundaries.

`matten` is not a security library, but it may be used inside web services, CSV/JSON pipelines, and PoC data tools. Therefore, it must prevent accidental or malicious resource exhaustion at obvious boundaries.

## 2. Goals

- Prevent shape product overflow.
- Prevent catastrophic allocation attempts.
- Bound `arange` length.
- Bound broadcasting output size.
- Provide user-configurable resource limits.
- Keep default behavior simple.
- Keep internal math ergonomic while making boundary APIs safe.

## 3. Non-goals

- No streaming execution engine.
- No out-of-core tensor storage.
- No GPU memory management.
- No sparse tensor model.
- No full sandbox against hostile inputs.

## 4. External design

### 4.1 Limit model

Introduce a lightweight limit object:

```rust
pub struct MattenLimits {
    pub max_dimensions: usize,
    pub max_elements: usize,
    pub max_parse_bytes: usize,
}
```

Default limits should be generous for PoCs but not unbounded.

Suggested defaults:

```text
max_dimensions = 8
max_elements = 100_000_000
max_parse_bytes = 128 * 1024 * 1024
```

Exact defaults may be tuned after benchmarking.

### 4.2 Try APIs

Boundary-safe APIs should use checked limits:

```rust
Tensor::try_new(data, shape)
Tensor::try_zeros(shape)
Tensor::try_ones(shape)
Tensor::try_full(shape, value)
Tensor::try_arange(start, stop, step)
Tensor::try_broadcast_shape(left, right)
```

Panicking APIs may delegate to `try_*` and panic with a readable message.

## 5. Data model

### 5.1 Shape product

All shape product calculations must use checked multiplication:

```rust
fn checked_element_count(shape: &[usize]) -> Result<usize, MattenError>;
```

Zero-sized dimensions remain unsupported unless a future RFC changes the shape model.

### 5.2 Allocation budget

Before allocating a vector, the required element count must be checked against limits.

For numeric tensors:

```text
requested bytes roughly = element_count * size_of::<f64>()
```

For dynamic tensors, element count alone is insufficient, but it remains a useful first guard.

## 6. Data lifecycle

Resource checks occur at these lifecycle points:

```text
construction
reshape
broadcast planning
arange generation
JSON/CSV parsing
dynamic conversion
```

No operation should attempt a large allocation before checking shape/product limits.

## 7. Events and observable behavior

Resource failures return:

```rust
MattenError::Allocation {
    requested_elements,
    message,
}
```

for `try_*` APIs.

Panicking APIs should panic with:

```text
matten allocation error in <operation>: ...
```

## 8. Store access

No new store is introduced.

The RFC ensures store allocation is checked before writing:

- `Vec<f64>`;
- dynamic element vector;
- output buffer for broadcasting;
- output buffer for reductions with axes;
- parser intermediate buffers where applicable.

## 9. Public API proposal

### 9.1 Limits type

```rust
#[derive(Debug, Clone, Copy)]
pub struct MattenLimits {
    pub max_dimensions: usize,
    pub max_elements: usize,
    pub max_parse_bytes: usize,
}

impl Default for MattenLimits {
    fn default() -> Self;
}
```

**Single source of truth (v0.13.3 kickoff review).** The codebase already
defines `MAX_NDIM` (= 8) and `ARANGE_MAX_ELEMENTS` (= `1 << 20`). RFC-018 MUST
NOT create a parallel limit system. Instead, `MattenLimits` becomes the single
policy object and the existing constants become its defaults:

```rust
impl Default for MattenLimits {
    fn default() -> Self {
        Self {
            max_dimensions: MAX_NDIM,
            max_elements: ARANGE_MAX_ELEMENTS,
            max_parse_bytes: DEFAULT_MAX_PARSE_BYTES, // new constant, see §9.4
        }
    }
}
```

After this RFC lands, `arange`/`try_arange` and the existing constructors MUST
route their element-count check through `MattenLimits::default().max_elements`
(or a shared helper using it) rather than reading `ARANGE_MAX_ELEMENTS`
directly. The old constant remains only as the literal that seeds the default,
so there is exactly one effective policy.

### 9.2 Limit-aware constructors

```rust
impl Tensor {
    pub fn try_new_with_limits(
        data: Vec<f64>,
        shape: &[usize],
        limits: MattenLimits,
    ) -> Result<Tensor, MattenError>;

    pub fn try_zeros_with_limits(
        shape: &[usize],
        limits: MattenLimits,
    ) -> Result<Tensor, MattenError>;
}
```

The simpler **`try_zeros` / `try_ones` / `try_full` are net-new public APIs**,
not cleanup of existing ones — only `try_new` and `try_arange` exist in v0.13.2.
They should be planned and changelogged as scope additions:

```rust
impl Tensor {
    pub fn try_zeros(shape: &[usize]) -> Result<Tensor, MattenError>;
    pub fn try_ones(shape: &[usize]) -> Result<Tensor, MattenError>;
    pub fn try_full(shape: &[usize], value: f64) -> Result<Tensor, MattenError>;
}
```

Panicking convenience APIs (`zeros`, `ones`, `full`) remain and delegate to the
checked internals.

### 9.4 New constant

```rust
// Default parser input ceiling; tune after benchmarking.
pub(crate) const DEFAULT_MAX_PARSE_BYTES: usize = 128 * 1024 * 1024;
```

### 9.3 Parser limits

```rust
impl Tensor {
    pub fn from_json_with_limits(input: &str, limits: MattenLimits) -> Result<Tensor, MattenError>;
    pub fn from_csv_with_limits(input: &str, limits: MattenLimits) -> Result<Tensor, MattenError>;
}
```

If this feels too broad for v0.14, document the design and implement constructor/broadcast/arange checks first.

## 10. Cargo feature impact

No new feature is required.

Parser limit APIs depend on existing `json` / `csv` features.

## 11. Internal design

### 11.1 Central helper module

Add:

```text
src/limits.rs
```

With helpers:

```rust
checked_shape_len(shape, limits)
checked_allocation_len(operation, requested_elements, limits)
checked_arange_len(start, stop, step, limits)
checked_broadcast_len(left, right, limits)
```

### 11.2 Broadcasting safety

Broadcast shape calculation must be separate from allocation.

Lifecycle:

```text
validate broadcast compatibility
compute result shape
check result element count
allocate output
compute
```

### 11.3 arange safety

`arange` must reject:

- zero step;
- non-finite start/stop/step;
- length overflow;
- length beyond max elements.

## 12. Examples

Modify examples only if needed.

Add a small boundary example:

```text
examples/13_resource_limits.rs
```

It should show:

```rust
let limits = MattenLimits { max_elements: 10, ..Default::default() };
let result = Tensor::try_zeros_with_limits(&[100], limits);
assert!(result.is_err());
```

## 13. Acceptance criteria

- Shape product overflow is impossible in accepted constructors.
- `arange` cannot attempt catastrophic allocation.
- Broadcasting checks output size before allocation.
- `MattenError::Allocation` is used consistently.
- Limit-aware API does not complicate quickstart.
- Existing examples remain simple.

## 14. QA checklist

- [ ] Shape overflow tests
- [ ] Max dimension tests
- [ ] Max element tests
- [ ] arange zero-step/non-finite/huge-length tests
- [ ] broadcasting huge-output test
- [ ] parser huge-input smoke tests if implemented
- [ ] Panic wrappers tested with `catch_unwind`

## 15. Open questions

1. What default `max_elements` is appropriate for a PoC crate?
2. Should limits be global, per-call only, or both?
3. Should parser byte limits be part of core limits or feature-specific parser config?
