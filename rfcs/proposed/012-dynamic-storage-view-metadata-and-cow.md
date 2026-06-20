# RFC-012: Dynamic Storage, View Metadata, and Copy-on-Write

> RFC status: Proposed  
> Project: `matten`  
> Target audience: library implementers, maintainers, reviewers  
> Design level: external design first; internal design where needed  
> Last updated: 2026-06-18

## 1. Summary

This RFC defines the Phase 2 dynamic storage architecture. Dynamic tensors may use shared storage and view metadata so slicing and reshaping large mixed datasets avoid immediate deep copies. Mutation must use copy-on-write (CoW) so modifying one tensor does not affect another tensor sharing the same storage.

## 2. Motivation

Phase 1 intentionally deep-copies to hide lifetimes. That is acceptable for small numerical PoCs but not ideal for messy business data with text and nulls. Phase 2 exists to handle larger and messier data more efficiently while preserving the single `Tensor` public type.

## 3. Goals

- Define dynamic storage with shared ownership.
- Define view metadata for slicing/reshape.
- Define materialization and mutation lifecycle.
- Preserve public no-lifetime API.
- Avoid reference cycles.
- Require memory tests before release.

## 4. Non-goals

- No lazy expression graph.
- No query optimizer.
- No borrowed external-memory views.
- No mutation-through-view public API in first dynamic release unless separately approved.
- No sparse storage.

## 5. Cargo Features

This RFC applies only under:

```toml
features = ["dynamic"]
```

Default Phase 1 may remain `Vec<f64>` and owned/materialized. If a unified internal storage enum is used, default build must not pay dynamic complexity cost.

## 6. Data Model

Recommended dynamic internal model:

```rust
#[cfg(feature = "dynamic")]
pub(crate) struct DynamicTensor {
    storage: std::sync::Arc<Vec<Element>>,
    shape: Vec<usize>,
    view: ViewMap,
}

#[cfg(feature = "dynamic")]
pub(crate) enum ViewMap {
    Contiguous,
    Indexed {
        offset: usize,
        strides: Vec<usize>,
        // optional explicit index map if slicing is irregular
    },
    MaterializedIndex(Vec<usize>),
}
```

This is a recommended shape, not a forced implementation. A simpler first implementation may use `Arc<Vec<Element>>` plus explicit index mapping if correctness is clearer.

## 7. Data Lifecycle

### 7.1 Construction

1. Boundary/native input creates owned `Vec<Element>`.
2. Tensor wraps it in `Arc`.
3. View is `Contiguous`.

### 7.2 Reshape

If element count matches:

- update shape metadata;
- keep same `Arc`;
- update view if needed;
- no element clone.

### 7.3 Slice

- create new tensor sharing same `Arc`;
- set shape and view mapping;
- no full clone unless slice cannot be represented cheaply and materialization is explicitly chosen.

### 7.4 Read

- logical index maps through view to storage index;
- return borrowed or cloned element according to public API.

### 7.5 Mutation

If mutation APIs are introduced:

1. Check `Arc::get_mut`.
2. If uniquely owned and contiguous, mutate directly.
3. If shared or view-mapped, materialize logical values into a new `Vec<Element>`.
4. Mutate new storage.
5. Reset view to `Contiguous`.

## 8. Events

Conceptual lifecycle events:

| Event | Required behavior |
|---|---|
| dynamic tensor constructed | store in `Arc`, contiguous view |
| reshape view created | no clone if possible |
| slice view created | no full clone for representable slices |
| materialization triggered | allocate logical data in row-major order |
| mutation requested | CoW if shared or non-contiguous |
| drop occurs | no reference cycle; memory reclaimed by Arc |

A future diagnostics feature may expose materialization counts, but not in this RFC.

## 9. Store Access

Dynamic storage is private. Public users should not see `Arc`, strides, offsets, or index maps.

Potential dynamic accessors:

```rust
#[cfg(feature = "dynamic")]
impl Tensor {
    pub fn get_element(&self, indices: &[usize]) -> Option<&Element>; // if lifetime ergonomics acceptable
    pub fn get_element_cloned(&self, indices: &[usize]) -> Option<Element>;
}
```

Because borrowed element access exposes lifetimes in return types but not user-written annotations, it may be acceptable. For beginner docs, prefer cloned/value access examples.

## 10. Public API

This RFC does not require exposing storage details. It requires that existing APIs continue to work:

```rust
let view = tensor.slice_str("0:100, :")?;
let reshaped = view.try_reshape(&[1000])?;
```

Dynamic-specific APIs may include:

```rust
#[cfg(feature = "dynamic")]
impl Tensor {
    pub fn materialize(&self) -> Tensor;
    pub fn is_materialized(&self) -> bool; // optional diagnostics
}
```

`materialize` may be useful for users who want predictable memory/layout before exporting. It should remain optional.

## 11. Internal Design

### 11.1 View mapping strategy

Start simple. Two viable first implementations:

#### Option A: Stride view

Good for reshape, transpose, regular slicing.

Pros:

- compact metadata;
- avoids per-element index map.

Cons:

- more complex indexing correctness.

#### Option B: Explicit index map

Good for simplicity.

Pros:

- easy to implement slicing correctness;
- works for irregular future cases.

Cons:

- index map can be large;
- may undermine memory goals.

Recommendation: use stride view for contiguous/regular views and reserve explicit index maps for cases that cannot be represented otherwise. Do not implement irregular advanced indexing in first dynamic release.

### 11.2 Materialization

Materialization must write data in logical row-major order for the tensor's current shape. After materialization, `view = Contiguous`.

### 11.3 CoW correctness

Mutation must never modify another tensor's observed values.

Test pattern:

```rust
let a = dynamic_tensor.slice_str("0:10, :")?;
let b = a.clone();
a.set_element(&[0, 0], Element::Float(99.0))?;
assert_ne!(a.get_element_cloned(&[0, 0]), b.get_element_cloned(&[0, 0]));
```

Exact mutation API may be deferred, but internal CoW should be designed for it.

## 12. Error Handling

- View creation from valid shape/slice should return `Result` if user input is involved.
- Materialization allocation failure or limit violation returns `Err` in Result-zone APIs.
- Internal logic must not panic due to invalid view metadata created by public APIs.

## 13. Testing

- reshape does not clone storage when possible;
- slicing does not clone full storage when representable;
- materialization preserves values;
- clones share storage until mutation;
- mutation isolates values;
- no reference cycles;
- memory stress tests for repeated views;
- text-heavy and null-heavy datasets from RFC-011.

## 14. Acceptance Criteria

- Dynamic slicing can avoid immediate deep copy.
- Public API still uses `Tensor`.
- Users do not need to know `Arc` or view metadata.
- CoW mutation isolation is defined and testable.
- Memory trade-offs are documented before `0.2.0` beta.
