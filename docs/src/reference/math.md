# Reductions and matrix multiplication

`matten` provides whole-tensor reductions, axis reductions, and explicit
matrix/vector multiplication. `*` remains element-wise — matrix multiplication
always requires `matmul` or `dot`.

## Whole-tensor reductions

```rust
use matten::Tensor;

let v = Tensor::from_vec(vec![1.0, 2.0, 3.0, 4.0]);

v.sum()   // 10.0
v.mean()  // 2.5
v.min()   // 1.0
v.max()   // 4.0
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

m.sum_axis(0)   // column sums  -> shape [3]  -> [5,7,9]
m.sum_axis(1)   // row sums     -> shape [2]  -> [6,15]
m.mean_axis(0)  // column means -> shape [3]  -> [2.5,3.5,4.5]
m.mean_axis(1)  // row means    -> shape [2]  -> [2.0,5.0]
```

The reduced axis is removed from the output shape. Reducing a vector along its
only axis gives a scalar-shaped tensor.

Both panic with an actionable message if `axis >= ndim`.

## Vector dot product

```rust
let a = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
let b = Tensor::from_vec(vec![4.0, 5.0, 6.0]);

let d = a.dot(&b);
assert!(d.is_scalar());
assert_eq!(d.as_slice(), &[32.0]); // 1*4 + 2*5 + 3*6
```

`dot` on two vectors `[n]` and `[n]` returns a **scalar tensor** (shape `[]`).

## Matrix multiplication

`matmul` is an alias for `dot`. Use whichever reads more clearly.

| Left shape | Right shape | Result shape |
|---|---|---|
| `[n]` | `[n]` | `[]` scalar |
| `[m, n]` | `[n]` | `[m]` |
| `[n]` | `[n, p]` | `[p]` |
| `[m, n]` | `[n, p]` | `[m, p]` |

```rust
let a = Tensor::new(vec![1.0,2.0,3.0,4.0], &[2,2]);
let b = Tensor::new(vec![5.0,6.0,7.0,8.0], &[2,2]);

let c = a.matmul(&b);
// [[19,22],[43,50]]
assert_eq!(c.as_slice(), &[19.0, 22.0, 43.0, 50.0]);
```

Incompatible shapes panic with an actionable message including both shapes.
Batched matmul (rank > 2) is out of scope for the numeric core.


## Axis reductions (min and max)

`min_axis` and `max_axis` reduce along an axis, removing it from the output
shape, and propagate `NaN` the same way `min` and `max` do.

```rust
use matten::Tensor;

// [[3,1,4],[1,5,9]]
let m = Tensor::new(vec![3.0,1.0,4.0,1.0,5.0,9.0], &[2,3]);

m.min_axis(0)  // column minimums -> shape [3] -> [1.0, 1.0, 4.0]
m.max_axis(0)  // column maximums -> shape [3] -> [3.0, 5.0, 9.0]
m.min_axis(1)  // row minimums   -> shape [2] -> [1.0, 1.0]
m.max_axis(1)  // row maximums   -> shape [2] -> [4.0, 9.0]
```

NaN propagation: if any element along the reduced axis is `NaN`, the output
for that position is `NaN`.

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

```rust
let flat: Vec<f64> = tensor.into_vec();
// hand off to your preferred crate
```

## See also

For the three linalg-adjacent helpers `norm`, `trace`, and `outer` — and the list
of advanced linear algebra that is intentionally out of core scope — see
[Linear algebra (core-lite)](./linalg.md).
