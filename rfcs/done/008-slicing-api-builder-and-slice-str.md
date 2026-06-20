# RFC-008: Slicing API: Builder and `slice_str`

> RFC status: Implemented (0.4.0)
> Project: `matten`  
> Target audience: library implementers, maintainers, reviewers  
> Design level: external design first; internal design where needed  
> Last updated: 2026-06-18

## 1. Summary

This RFC defines `matten` slicing. The canonical slicing API is a Rust-native builder. `slice_str` is a secondary convenience API for NumPy-like exploratory workflows and must return `Result<Tensor, MattenError>`. Phase 1 slices return owned materialized tensors. Phase 2 may return view-like tensors backed by shared storage if RFC-012 approves.

## 2. Motivation

Slicing is necessary for useful tensor manipulation, but it is also a parser and indexing risk. A builder-first API preserves Rust clarity and avoids making a string parser the core array model. `slice_str` remains useful for tutorial and NumPy-familiar workflows, but it must be bounded and recoverable.

## 3. Goals

- Define a builder-first slicing API.
- Define supported slice components.
- Define Phase 1 materialization semantics.
- Define `slice_str` minimum grammar.
- Ensure malformed string slices return `Result::Err`, not panic.
- Keep public slicing API lifetime-free.

## 4. Non-goals

- No public `slice!` macro in initial release.
- No boolean masks.
- No fancy indexing with arbitrary index arrays.
- No negative indices in `0.1.0`.
- No mutation-through-slice in Phase 1.
- No lazy Phase 1 views.

## 5. Cargo Features

| Feature | Behavior |
|---|---|
| default | Builder and optional bounded `slice_str`, materialized output. |
| `dynamic` | Same external syntax; may return shared-storage tensor internally. |

`slice_str` should not require heavy parser dependencies.

## 6. Data Model

Internal slice specification:

```rust
pub(crate) enum SliceItem {
    All,
    Index(usize),
    Range { start: Option<usize>, end: Option<usize>, step: usize },
}

pub struct SliceBuilder<'a> { /* lifetime internal to builder only */ }
```

The builder may carry a reference lifetime internally, but the returned `Tensor` is owned and has no public lifetime parameter.

For Phase 1, slicing removes indexed axes or keeps them? This RFC chooses NumPy-like integer index semantics:

- `Index(n)` selects one element along an axis and removes that axis from the output shape.
- `Range` and `All` keep the axis.

Example:

```text
shape [2, 3], spec [Index(0), All] -> shape [3]
shape [2, 3], spec [Range(0..1), All] -> shape [1, 3]
```

## 7. Data Lifecycle

### 7.1 Builder slicing

1. User starts `tensor.slice()`.
2. User appends one item per axis.
3. `build()` validates rank and bounds.
4. Output shape is calculated.
5. Output values are materialized in row-major order.
6. Owned `Tensor` is returned.

### 7.2 String slicing

1. User calls `slice_str(spec)`.
2. Parser validates length and grammar.
3. Parser converts to internal `SliceItem` list.
4. Same build path as builder is used.

`slice_str` must be a thin wrapper over the builder execution model.

## 8. Events

| Event | Required behavior |
|---|---|
| slice builder created | no allocation beyond builder metadata |
| slice item added | record item in order |
| build called | validate full rank and bounds |
| string parser starts | enforce bounded length |
| parser fails | return `MattenError::Slice` or parse equivalent |
| output materialized | Phase 1 copies selected elements |

No public event system.

## 9. Store Access

Slicing reads private tensor storage and writes new tensor storage in Phase 1. It does not touch external stores.

Phase 2 may instead store view metadata and share storage; this is internal and must preserve the same output values.

## 10. Public API

```rust
impl Tensor {
    pub fn slice(&self) -> SliceBuilder<'_>;
    pub fn slice_str(&self, spec: &str) -> Result<Tensor, MattenError>;
}

impl<'a> SliceBuilder<'a> {
    pub fn all(self) -> Self;
    pub fn index(self, index: usize) -> Self;
    pub fn range<R>(self, range: R) -> Self
    where
        R: IntoSliceRange;

    pub fn build(self) -> Result<Tensor, MattenError>;
}
```

The exact `IntoSliceRange` design is internal or sealed. Public docs should emphasize examples rather than trait details.

Example:

```rust
let first_row = t.slice()
    .index(0)
    .all()
    .build()?;

let first_two_rows = t.slice()
    .range(0..2)
    .all()
    .build()?;
```

## 11. `slice_str` Grammar

Minimum grammar:

```text
spec       := axis_spec ("," axis_spec)*
axis_spec  := ws? (":" | index | range) ws?
index      := digits
range      := start? ":" end? (":" step)?
start      := digits
end        := digits
step       := nonzero_digits
```

Supported examples:

| Spec | Meaning |
|---|---|
| `:` | all for one axis |
| `0` | index 0 |
| `0:2` | start inclusive, end exclusive |
| `1:` | from index 1 to axis end |
| `:2` | from axis start to index 2 exclusive |
| `0:10:2` | stepped range |

Whitespace around tokens is ignored.

Rejected in `0.1.0`:

- negative indices;
- ellipsis `...`;
- newaxis/None insertion;
- boolean masks;
- list indexes such as `[0,2]`.

## 12. Internal Design

### 12.1 Shared execution

Both builder and string slicing must call the same internal executor:

```rust
pub(crate) fn execute_slice(tensor: &Tensor, spec: &[SliceItem]) -> Result<Tensor, MattenError>;
```

### 12.2 Bounds validation

- number of slice items must equal rank, unless a future RFC defines missing axes as `All`;
- `Index(i)` requires `i < dim`;
- range start/end must satisfy `start <= end <= dim`;
- step must be greater than zero.

### 12.3 Output shape calculation

For each axis:

- `All` contributes original dimension;
- `Index(_)` contributes no dimension;
- `Range` contributes computed length.

If all axes are indexed, output shape is `[]` scalar.

## 13. Error Handling

Builder `build()` returns `Result` because slicing is often driven by user choices.

`slice_str` always returns `Result` and must include the original spec in error messages.

Example:

```text
matten slice parse error: invalid slice spec "0::" at axis 0; expected start:end[:step]
```

## 14. Testing

- builder all/index/range/range-from/range-to;
- scalar output by indexing all axes;
- bounds errors;
- rank mismatch;
- `slice_str` malformed inputs;
- `slice_str` whitespace handling;
- step slicing if implemented;
- output independence in Phase 1;
- fuzz target for `slice_str` parser.

## 15. Acceptance Criteria

- Builder slicing is usable and documented as canonical.
- `slice_str` is secondary and safe if implemented.
- Malformed string slicing never panics.
- Phase 1 output is owned and independent.
- No public slicing macro is added without a later RFC.
