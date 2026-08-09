# Reductions and matrix multiplication

`matten` provides whole-tensor reductions, axis reductions, and explicit
matrix/vector multiplication. `*` remains element-wise — matrix multiplication
always requires `matmul` or `dot`.

## Whole-tensor reductions

```rust
use matten::Tensor;

let v = Tensor::from_vec(vec![1.0, 2.0, 3.0, 4.0]);

v.sum();   // 10.0
v.mean();  // 2.5
v.min();   // 1.0
v.max();   // 4.0
# assert_eq!(v.sum(), 10.0);
# assert_eq!(v.mean(), 2.5);
# assert_eq!(v.min(), 1.0);
# assert_eq!(v.max(), 4.0);
```

All four return `f64`. `sum` and `mean` propagate `NaN` naturally (IEEE 754).
`min` and `max` return `NaN` if **any** element is `NaN` — this is deliberate
and documented (see below).

## NaN / Inf policy

| Operation | NaN behaviour |
|---|---|
| `sum` | propagates (`NaN + x = NaN`) |
| `mean` | propagates |
| `min` | returns `NaN` if any element is `NaN` |
| `max` | returns `NaN` if any element is `NaN` |
| `argmin` / `argmax` | **error/panic** if any element is `NaN` (an index is ill-defined) |

```rust
let t = Tensor::from_vec(vec![1.0, f64::NAN, 3.0]);
assert!(t.min().is_nan());
assert!(t.max().is_nan());
```

`Inf` is handled normally: it participates in comparisons as expected.

**Implementation note:** `min`/`max` detect `NaN` explicitly and
short-circuit. They do **not** use `f64::min`/`f64::max` (which silently
ignore `NaN`).

## Index reductions (argmin / argmax, RFC-038)

`argmin`/`argmax` return the **flat, row-major** index of the smallest/largest
element, with the **first occurrence** winning ties:

```rust
use matten::Tensor;
let t = Tensor::new(vec![2.0, 9.0, 3.0, 1.0, 0.0, 4.0], &[2, 3]);
assert_eq!(t.argmin(), 4); // the 0.0
assert_eq!(t.argmax(), 1); // the 9.0
```

Unlike the value reductions above, an index is ill-defined when any element is
`NaN`. These therefore follow the **selection** branch of the NaN policy:
`try_argmin`/`try_argmax` return `MattenError::InvalidArgument`, and the convenience
`argmin`/`argmax` panic with the same context. (On a dynamic tensor the `try_*` forms
return `MattenError::Unsupported`; call `try_numeric()` first.)

## Axis reductions

```rust
// [[1,2,3],[4,5,6]]
let m = Tensor::new(vec![1.0,2.0,3.0,4.0,5.0,6.0], &[2,3]);

m.sum_axis(0);   // column sums  -> shape [3]  -> [5,7,9]
m.sum_axis(1);   // row sums     -> shape [2]  -> [6,15]
m.mean_axis(0);  // column means -> shape [3]  -> [2.5,3.5,4.5]
m.mean_axis(1);  // row means    -> shape [2]  -> [2.0,5.0]
# assert_eq!(m.sum_axis(0).shape(), &[3]);
# assert_eq!(m.sum_axis(0).to_vec(), vec![5.0, 7.0, 9.0]);
# assert_eq!(m.sum_axis(1).shape(), &[2]);
# assert_eq!(m.sum_axis(1).to_vec(), vec![6.0, 15.0]);
# assert_eq!(m.mean_axis(0).shape(), &[3]);
# assert_eq!(m.mean_axis(0).to_vec(), vec![2.5, 3.5, 4.5]);
# assert_eq!(m.mean_axis(1).shape(), &[2]);
# assert_eq!(m.mean_axis(1).to_vec(), vec![2.0, 5.0]);
```

The reduced axis is removed from the output shape. Reducing a vector along its
only axis gives a scalar-shaped tensor.

Both panic with an actionable message if `axis >= ndim`.

**Empty reduced axis (RFC-110):** `mean_axis`/`try_mean_axis` error
(`MattenError::InvalidArgument`, or a panic carrying that message) when the
**reduced** axis has length 0 — the mean of nothing is undefined. `sum_axis` is
unaffected and returns the additive identity `0.0` per output slot, the same
boundary RFC-105 drew for whole-tensor `sum`. A zero-length axis that
*survives* the reduction (the axis you did **not** reduce) is a different case
entirely and still returns `Ok` with an empty result — no constructor accepts
a zero-sized shape, but slicing reaches one
(`t.slice().range(0..0).all().build()`).

Read an axis reduction as "collapse that axis and keep the others":

```text
input shape [2, 3]
axes         0  1

axis 0 = rows      axis 1 remains, output shape [3]
axis 1 = columns   axis 0 remains, output shape [2]
```

For a `[2, 3]` matrix:

```text
            columns / axis 1
             0   1   2
rows 0     [ 1   2   3 ]
axis 0     [ 4   5   6 ]

mean_axis(0): collapse rows, keep columns
             [ (1+4)/2  (2+5)/2  (3+6)/2 ]
          -> [   2.5      3.5      4.5   ]   shape [3]

mean_axis(1): collapse columns, keep rows
             [ (1+2+3)/3  (4+5+6)/3 ]
          -> [     2.0        5.0   ]         shape [2]
```

## Vector dot product

```rust
let a = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
let b = Tensor::from_vec(vec![4.0, 5.0, 6.0]);

let d = a.dot(&b);
assert!(d.is_scalar());
assert_eq!(d.as_slice(), &[32.0]); // 1*4 + 2*5 + 3*6
```

`dot` on two vectors `[n]` and `[n]` returns a **scalar tensor** (shape `[]`).

`try_dot` returns `Result<Tensor, MattenError>` instead of panicking: `MattenError::Shape`
on incompatible shapes or an unsupported rank combination, `MattenError::Unsupported` on a
dynamic tensor (call `try_numeric()` on each operand first). `dot` delegates to `try_dot`
and panics with the same message on error.

## Matrix multiplication

`matmul` is an alias for `dot`, including its `try_matmul`/`try_dot` non-panicking form.
Use whichever reads more clearly.

| Left shape | Right shape | Result shape |
|---|---|---|
| `[n]` | `[n]` | `[]` scalar |
| `[m, n]` | `[n]` | `[m]` |
| `[n]` | `[n, p]` | `[p]` |
| `[m, n]` | `[n, p]` | `[m, p]` |

Shape flow for the common matrix-matrix case:

```text
left shape       right shape       result shape
[m, n]       x   [n, p]        ->  [m, p]
    ^             ^
    |             |
    shared inner dimension must match
```

Each output cell is one row from the left dotted with one column from the right:

```text
left [2, 3]          right [3, 2]           result [2, 2]

[ a b c ]            [ x y ]                [ ax+bz+cu   ay+bw+cv ]
[ d e f ]       x    [ z w ]          ->    [ dx+ez+fu   dy+ew+fv ]
                     [ u v ]
```

```rust
let a = Tensor::new(vec![1.0,2.0,3.0,4.0], &[2,2]);
let b = Tensor::new(vec![5.0,6.0,7.0,8.0], &[2,2]);

let c = a.matmul(&b);
// [[19,22],[43,50]]
assert_eq!(c.as_slice(), &[19.0, 22.0, 43.0, 50.0]);
```

Incompatible shapes panic with an actionable message including both shapes, or with
`try_matmul`, return `Err(MattenError::Shape { .. })` with the same message.
Batched matmul (rank > 2) is out of scope for the numeric core.


## Axis reductions (min and max)

`min_axis` and `max_axis` reduce along an axis, removing it from the output
shape, and propagate `NaN` the same way `min` and `max` do.

```rust
use matten::Tensor;

// [[3,1,4],[1,5,9]]
let m = Tensor::new(vec![3.0,1.0,4.0,1.0,5.0,9.0], &[2,3]);

m.min_axis(0);  // column minimums -> shape [3] -> [1.0, 1.0, 4.0]
m.max_axis(0);  // column maximums -> shape [3] -> [3.0, 5.0, 9.0]
m.min_axis(1);  // row minimums   -> shape [2] -> [1.0, 1.0]
m.max_axis(1);  // row maximums   -> shape [2] -> [4.0, 9.0]
# assert_eq!(m.min_axis(0).to_vec(), vec![1.0, 1.0, 4.0]);
# assert_eq!(m.max_axis(0).to_vec(), vec![3.0, 5.0, 9.0]);
# assert_eq!(m.min_axis(1).to_vec(), vec![1.0, 1.0]);
# assert_eq!(m.max_axis(1).to_vec(), vec![4.0, 9.0]);
```

NaN propagation: if any element along the reduced axis is `NaN`, the output
for that position is `NaN`.

**Empty reduced axis (RFC-110):** `min_axis`/`try_min_axis` and
`max_axis`/`try_max_axis` error (`MattenError::InvalidArgument`, or a panic
carrying that message) when the **reduced** axis has length 0, rather than
returning `f64::INFINITY`/`f64::NEG_INFINITY` — those are fold identities, not
answers. A zero-length axis that *survives* the reduction still returns `Ok`
with an empty result.

## `*` is always element-wise

```rust
let a = Tensor::new(vec![1.0,2.0,3.0,4.0], &[2,2]);
let b = Tensor::new(vec![5.0,6.0,7.0,8.0], &[2,2]);

let elem = &a * &b;        // [5, 12, 21, 32]  ← element-wise
let mat  = a.matmul(&b);   // [19, 22, 43, 50] ← matrix product
```

`matten` never overloads `*` for matrix multiplication. If you need the matrix
product, always call `matmul` or `dot` explicitly.

## Performance note

matmul uses plain nested loops — correct and readable, but not
cache-optimised. For large matrices, migrate the flat data to `ndarray` or
`nalgebra`:

```rust,ignore
let flat: Vec<f64> = tensor.into_vec();
// hand off to your preferred crate
```

## Display / formatting (RFC-100)

`Tensor` implements `Display` (`{}`) for a human-facing rendering, distinct from the
single-line `Debug` (`{:?}`) used for diagnostics:

```rust
use matten::Tensor;

let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
assert_eq!(t.to_string(), "1.0 2.0 3.0\n4.0 5.0 6.0");
assert_eq!(
    format!("{t:?}"),
    "Tensor(shape=[2, 3], data=[1.0, 2.0, 3.0, 4.0, 5.0, 6.0])"
);
```

Rank 0 is the bare scalar; rank 1 is one right-aligned row; rank 2 is a right-aligned grid,
per-column widths, no brackets and no commas — matching what this project already renders
elsewhere rather than `ndarray`'s `[[1, 2], [3, 4]]` syntax:

```rust
use matten::Tensor;

assert_eq!(Tensor::scalar(3.5).to_string(), "3.5");
assert_eq!(Tensor::new(vec![1.0, 2.0, 3.0], &[3]).to_string(), "1.0 2.0 3.0");
```

Every cell uses `{:?}` (Debug) formatting, not bare `Display` — `matten`'s only element type
is `f64`, and bare `Display` drops the `.0` on whole numbers, which would make a grid of
floats read as one of integers:

```rust
use matten::Tensor;
// Deliberately diverges from ndarray, which prints "1" here, not "1.0".
assert_eq!(Tensor::new(vec![1.0, 2.0], &[2]).to_string(), "1.0 2.0");
```

Rank > 2 has no honest 2-D arrangement, so it falls back to the flat form used before this
RFC existed:

```rust
use matten::Tensor;
let t = Tensor::new((1..=8).map(|x| x as f64).collect(), &[2, 2, 2]);
assert_eq!(
    t.to_string(),
    "shape=[2, 2, 2] values=[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]"
);
```

A rank-1 row truncates past 12 values, and a rank-2 grid truncates past 12 columns, so
`Display` on a huge tensor cannot flood a terminal. `{:#}` (the alternate flag) disables
truncation:

```rust
use matten::Tensor;
let row: Vec<f64> = (1..=13).map(|x| x as f64).collect();
let t = Tensor::new(row, &[13]);
assert!(t.to_string().ends_with("... 1 more values"));
assert!(!format!("{t:#}").contains("more values"));
```

On a dynamic tensor (`#[cfg(feature = "dynamic")]`), each cell renders via `Element`'s own
`Display` in the same grid, **except `Float`**, which uses `{:?}` on the inner `f64` instead
so a whole-number float stays visibly distinct from an `Int` of the same value — a dynamic
tensor exists precisely to carry mixed types in one grid, and `Element`'s own `Display`
alone would render `Float(2.0)` and `Int(2)` identically as `2`:

```rust
use matten::{Element, Tensor};

let t = Tensor::from_elements(
    vec![Element::Float(2.0), Element::Int(2), Element::Float(1.5), Element::Int(7)],
    &[2, 2],
);
assert_eq!(t.to_string(), "2.0 2\n1.5 7");
```

`Debug` is unchanged by this RFC — it stays the single-line, truncated-at-8 diagnostic form
RFC-020 defined, and is still the better choice for logs. `Display` is for a human looking at
the data.

## See also

For the three linalg-adjacent helpers `norm`, `trace`, and `outer` — and the list
of advanced linear algebra that is intentionally out of core scope — see
[Linear algebra (core-lite)](./linalg.md).

For population variance and standard deviation — `var`, `std`, `var_axis`,
`std_axis` — see [Statistics (core-lite)](./stats.md).
