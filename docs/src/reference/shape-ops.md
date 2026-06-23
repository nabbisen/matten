# Shape operations

All shape-transformation methods return **new independent owned tensors**.
Phase 1 copies data internally; no view lifetime is ever exposed.

## Reshape

```rust
let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);

// Panic zone
let r = t.reshape(&[3, 2]);      // shape [3, 2], same flat order

// Result zone
let r = t.try_reshape(&[3, 2])?; // MattenError::Shape on mismatch
```

Only the element count matters — reshape never fails because of memory layout.
Flat data order (row-major) is preserved unchanged.

```rust
// Any compatible shape works
let flat  = t.reshape(&[6]);        // [6]
let col   = t.reshape(&[6, 1]);     // [6, 1]
let cube  = t.reshape(&[1, 2, 3]);  // [1, 2, 3]
```

Panic message on mismatch:

```text
matten shape error in reshape: cannot reshape tensor with 6 elements
    from shape [2, 3] into shape [4, 2] requiring 8 elements
```

## Flatten

```rust
let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
let f = t.flatten();   // shape [4]

// A scalar becomes shape [1]
let s = Tensor::scalar(7.0).flatten();  // shape [1]
```

## Transpose

`transpose()` reverses the axis order. `t()` is an alias.

```rust
// 2-D: swap rows and columns
let m  = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
let mt = m.transpose();
// shape [3, 2], data [1.0, 4.0, 2.0, 5.0, 3.0, 6.0]

// Higher rank: axes are fully reversed
// [d0, d1, d2] → [d2, d1, d0]
let t3  = Tensor::new((1..=24).map(|x| x as f64).collect(), &[2, 3, 4]);
let t3t = t3.transpose();  // shape [4, 3, 2]
```

Transposing twice is the identity:

```rust
assert_eq!(t.transpose().transpose(), t);
```

Transposing a scalar panics — there are no axes to reverse.

## Swap axes

```rust
let t = Tensor::new((1..=24).map(|x| x as f64).collect(), &[2, 3, 4]);
let s = t.swap_axes(0, 2);  // shape [4, 3, 2]
```

Swapping an axis with itself is a no-op. Out-of-range axes panic:

```text
matten shape error in swap_axes: axis 5 is out of range for rank-3 tensor
```

## Squeeze and expand_dims (RFC-038)

```rust
use matten::Tensor;

// squeeze: drop every length-1 axis (data order unchanged)
let t = Tensor::new(vec![1.0, 2.0, 3.0], &[1, 3, 1]);
let s = t.squeeze();           // shape [3]

// an all-ones shape squeezes to a scalar
let one = Tensor::new(vec![5.0], &[1, 1]).squeeze();  // shape []

// expand_dims: insert a length-1 axis at `axis` (0..=ndim)
let v = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
let row = v.expand_dims(0);    // [1, 3]
let col = v.expand_dims(1);    // [3, 1]

// Result zone: axis > ndim is an InvalidArgument
let r = v.try_expand_dims(axis)?;
```

`squeeze` removes all length-1 axes and never fails (a scalar stays a scalar).
`expand_dims` accepts `axis` in `0..=ndim`; an out-of-range axis panics, while
`try_expand_dims` returns `MattenError::InvalidArgument`. Both clone data and reject
dynamic tensors (call `try_numeric()` first).

## Element access

```rust
let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);

t.get(&[0, 1])  // Some(2.0)
t.get(&[5, 0])  // None — out of bounds
t.get(&[0])     // None — rank mismatch

// Scalar element
Tensor::scalar(99.0).get(&[])  // Some(99.0)
```

`get` returns `Option<f64>` and never panics. There is no mutable element
setter in Phase 1.

## Phase 1 ownership note

Every method above clones or physically reorders data into a fresh contiguous
buffer. This keeps the API lifetime-free and predictable, at the cost of
higher allocation than a view-based library. When this matters for large data,
migrate to `ndarray` or `nalgebra` using `tensor.into_vec()`.
