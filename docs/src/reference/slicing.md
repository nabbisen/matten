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

```rust,ignore
// Shape [2, 3]: index one axis
let scalar_result = t.slice().index(0).index(1).build()?;
assert!(scalar_result.is_scalar());  // both axes indexed out → shape []
```

## `slice_str` (convenience)

```rust,ignore
let row  = t.slice_str("0, :")?;      // first row
let top2 = t.slice_str("0:2, :")?;   // first two rows
let step = t.slice_str("::2")?;      // every other element in a 1-D tensor
let last = t.slice_str("-1, :")?;    // last row (RFC-088)
```

Grammar:

| Pattern | Meaning |
|---|---|
| `:` | all (`All`) |
| `n` or `-n` | single index (`Index(n)`); a leading `-` counts from the end |
| `start:end` | half-open range; either bound may have a leading `-` |
| `start:` | from start to axis end |
| `:end` | from axis start to end |
| `start:end:step` | stepped range (`step` is always positive; a leading `-` on `step` is a parse error) |

Whitespace around tokens is ignored: `"0:2, :"` and `" 0:2 , : "` are
equivalent.

`slice_str` **always returns `Result`** and never panics on malformed input.
It rejects specs longer than 512 bytes.

### Negative indices (RFC-088)

`index`, `start`, and `end` accept an optional leading `-`, matching Python's
convention: `-1` is the last element along that axis, `-2` the second to
last, and so on. A negative value is resolved as `dim + i` before the usual
bounds check.

```rust
let t = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);
assert_eq!(t.slice_str("-1")?.as_slice(), &[3.0]);       // last element
assert_eq!(t.slice_str("0:-1")?.as_slice(), &[1.0, 2.0]); // everything but the last
assert_eq!(t.slice_str("-2:")?.as_slice(), &[2.0, 3.0]);  // last two
```

**Out-of-range negatives error; they do not clamp.** `slice_str("-10")` or
`slice_str("-10:")` on an axis of size 3 is an error — unlike Python, which
clamps a negative slice bound silently (`a[-10:]` on a 3-element list returns
the whole list). `matten` already errors on positive out-of-range values
(`"0:100"` on size 3 errors too), so a spec string is not validated by two
different rules depending on its sign. The error message names both the
written form and what it resolved to, e.g. `index -10 (resolves to -7) is
out of range for axis 0 with size 3`.

**The builder does not accept negative indices.** `SliceBuilder::index` and
`.range()` take `usize` only; a caller with `len` in hand writes `len - 1`
directly. Adding signed range support to the builder would make every
existing `range(1..3)` call ambiguous between `usize` and `isize` inference —
a source-breaking change RFC-088 declines to make for a convenience feature.

**Negative step ("reversal") is not implemented.** `step` stays positive-only;
`"::-1"` remains a parse error, not a reversed slice.

## Builder vs `slice_str`

The builder is the primary API because it is type-checked at the call site.
`slice_str` is useful for exploratory work and tutorials where NumPy-familiar
syntax is more readable.

```rust,ignore
// These produce the same tensor
let a = t.slice().range(0..2).all().build()?;
let b = t.slice_str("0:2, :")?;
assert_eq!(a, b);
```

When in doubt, use the builder — it gives better error messages and is
documented in examples as canonical.

## Numeric Tensor ownership

Every slice of a **numeric** tensor is a **new contiguous owned tensor**. No
borrowed view of the source tensor is returned. This means slicing always
allocates and copies the selected `f64`s, but the API is lifetime-free and
safe to pass across function boundaries without lifetime annotation.

## Slicing dynamic tensors (RFC-102, `#[cfg(feature = "dynamic")]`)

`slice()` and `slice_str()` also work on dynamic tensors, returning a dynamic
tensor (`is_dynamic() == true`). The grammar, rank rules, and error messages
are identical to the numeric case — slicing selects *positions*; it does not
interpret `Element` values, so `Text`, `None`, and `Bool` survive a slice
unchanged alongside `Int`/`Float`.

**Ownership differs from the numeric case above:** a dynamic slice shares
storage with its source (`Arc::clone`, RFC-012's copy-on-write model) rather
than copying elements. Slicing a slice composes through the existing view
instead of nesting, so an arbitrarily long chain of slices still shares one
underlying allocation.

**That sharing has a cost: a slice keeps its source's entire allocation
alive for as long as the slice itself lives — even after the source tensor
is dropped.** A one-element slice of a 100,000-element tensor retains all
100,000 elements in memory, not just the one selected. If you need to
release the rest, materialize the slice into its own storage explicitly:

```rust
# #[cfg(feature = "dynamic")] {
use matten::{Element, Tensor};

let t = Tensor::from_elements((0..6).map(Element::Int).collect(), &[2, 3]);
let row = t.slice().index(0).all().build().unwrap();
let released = Tensor::from_elements(row.to_elements(), row.shape());
# let _ = released;
# }
```

```rust
# #[cfg(feature = "dynamic")] {
use matten::{Element, Tensor};

let t = Tensor::from_elements((0..6).map(Element::Int).collect(), &[2, 3]);
let row = t.slice().index(0).all().build().unwrap();
assert!(row.is_dynamic());
assert_eq!(row.get_element(&[1]), Some(Element::Int(1)));
# }
```

## Error handling

`build()` and `slice_str()` both return `MattenError::Slice` on:

- number of specs ≠ tensor rank;
- index out of bounds;
- range start > end or end > dimension;
- `slice_str` parse error (carries the original spec string).

```rust,ignore
let err = t.slice().all().build().unwrap_err(); // too few specs for rank-2
assert!(matches!(err, MattenError::Slice { .. }));
```
