# Slicing

`matten` provides two slicing APIs. The builder is the canonical form; `slice_str`
is a NumPy-like convenience. Both return owned tensors and never produce view
lifetimes.

## Builder API (canonical)

```rust
use matten::Tensor;

let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);

// One method call per axis; finish with .build()
let row  = t.slice().index(0).all().build()?;     // shape [3]
let top2 = t.slice().range(0..2).all().build()?;  // shape [2, 3]
let col1 = t.slice().all().index(1).build()?;     // shape [2]
```

Builder methods:

| Method | Meaning |
|---|---|
| `.all()` | all elements along this axis (`:`), axis kept |
| `.index(n)` | single element, **axis removed** from output shape |
| `.range(0..2)` | half-open range, axis kept |
| `.range(1..)` | from index 1 to end |
| `.range(..3)` | from start to index 3 (exclusive) |
| `.range(..)` | entire axis (same as `.all()`) |
| `.range(0..=2)` | inclusive range → converted to `0..3` |
| `.build()` | validate and materialise, returns `Result<Tensor, MattenError>` |

Index semantics follow NumPy: `index(n)` removes the axis, collapsing one
dimension. `range` keeps it.

```rust
// Shape [2, 3]: index one axis
let scalar_result = t.slice().index(0).index(1).build()?;
assert!(scalar_result.is_scalar());  // both axes indexed out → shape []
```

## `slice_str` (convenience)

```rust
let row  = t.slice_str("0, :")?;      // first row
let top2 = t.slice_str("0:2, :")?;   // first two rows
let step = t.slice_str("::2")?;      // every other element in a 1-D tensor
```

Grammar:

| Pattern | Meaning |
|---|---|
| `:` | all (`All`) |
| `n` | single index (`Index(n)`) |
| `start:end` | half-open range |
| `start:` | from start to axis end |
| `:end` | from axis start to end |
| `start:end:step` | stepped range |

Whitespace around tokens is ignored: `"0:2, :"` and `" 0:2 , : "` are
equivalent.

`slice_str` **always returns `Result`** and never panics on malformed input.
It rejects specs longer than 512 bytes.

## Builder vs `slice_str`

The builder is the primary API because it is type-checked at the call site.
`slice_str` is useful for exploratory work and tutorials where NumPy-familiar
syntax is more readable.

```rust
// These produce the same tensor
let a = t.slice().range(0..2).all().build()?;
let b = t.slice_str("0:2, :")?;
assert_eq!(a, b);
```

When in doubt, use the builder — it gives better error messages and is
documented in examples as canonical.

## Numeric Tensor ownership

Every slice result is a **new contiguous owned tensor**. No borrowed view of
the source tensor is returned. This means slicing always allocates, but the
API is lifetime-free and safe to pass across function boundaries without
lifetime annotation.

## Error handling

`build()` and `slice_str()` both return `MattenError::Slice` on:

- number of specs ≠ tensor rank;
- index out of bounds;
- range start > end or end > dimension;
- `slice_str` parse error (carries the original spec string).

```rust
let err = t.slice().all().build().unwrap_err(); // too few specs for rank-2
assert!(matches!(err, MattenError::Slice { .. }));
```
