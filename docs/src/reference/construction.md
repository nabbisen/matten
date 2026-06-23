# Construction and conversion

All `matten` construction produces an owned, contiguous, row-major `Vec<f64>`
paired with a validated shape. Fields are private; users interact only through
methods.

## Core constructors

```rust
// From data + shape (panic zone)
let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);

// From data + shape (Result zone)
let t = Tensor::try_new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2])?;

// 1-D from flat vector
let t = Tensor::from_vec(vec![1.0, 2.0, 3.0]);   // shape [3]
```

`new` panics on mismatch; `try_new` returns `MattenError::Shape` or
`MattenError::Allocation`.

## Fill constructors

```rust
let z = Tensor::zeros(&[3, 4]);       // all 0.0, shape [3, 4]
let o = Tensor::ones(&[3, 4]);        // all 1.0
let f = Tensor::full(&[3, 4], -1.0); // all -1.0
let s = Tensor::scalar(42.0);         // shape [], len 1
```

All fill constructors validate the shape before allocating — a bad shape panics
with an actionable message.

## Range constructor

```rust
// Half-open, step > 0: [0.0, 1.0, 2.0, 3.0, 4.0]
let r = Tensor::arange(0.0, 5.0, 1.0);

// Negative step: [3.0, 2.0, 1.0]
let r = Tensor::arange(3.0, 0.0, -1.0);

// Result zone (step or bounds from user input)
let r = Tensor::try_arange(start, end, step)?;
```

`arange` rejects zero or non-finite step, non-finite bounds, and a computed
element count above the allocation limit (`2²⁸`).

## Evenly spaced values and identity (RFC-038)

```rust
// `count` evenly spaced values, inclusive of both endpoints:
let xs = Tensor::linspace(0.0, 1.0, 5);   // [0.0, 0.25, 0.5, 0.75, 1.0]
let one = Tensor::linspace(2.0, 9.0, 1);  // [2.0]

// n × n identity matrix:
let i3 = Tensor::eye(3);                   // 1.0 on the diagonal, 0.0 elsewhere

// Result zone:
let xs = Tensor::try_linspace(start, end, count)?;
let i = Tensor::try_eye(n)?;
```

`linspace` includes both endpoints when `count >= 2`, returns `[start]` when
`count == 1`, and rejects `count == 0`. `eye` produces shape `[n, n]` and rejects
`n == 0`. Both are budget-checked like the fill constructors (oversized results
yield `MattenError::Allocation`).

## Shape model

Shapes are runtime `Vec<usize>`. There is no const-generic or type-level
shape arithmetic.

| Shape | Meaning |
|---|---|
| `[]` | scalar — `len() == 1`, `is_scalar() == true` |
| `[n]` | 1-D vector — `is_vector() == true` |
| `[rows, cols]` | 2-D matrix — `is_matrix() == true` |
| `[d0, …, d7]` | up to rank 8 |

Rules enforced on every constructor:

- Zero-sized dimensions are rejected (deferred to a future RFC).
- Rank may not exceed 8.
- Shape product is computed with checked arithmetic; overflow returns
  `MattenError::Allocation`.

## Nested row construction

```rust
// Panic zone (convenience for trusted literals)
let t: Tensor = vec![vec![1.0, 2.0], vec![3.0, 4.0]].into();

// Result zone (ragged rows return Err)
let t = Tensor::try_from_rows(vec![vec![1.0, 2.0], vec![3.0, 4.0]])?;
```

`From<Vec<Vec<f64>>>` panics on ragged rows with an actionable message.
`try_from_rows` returns `MattenError::Shape` with the ragged-row detail.

## Inspection

```rust
t.shape()      // &[usize]  — no allocation
t.ndim()       // usize     — shape().len()
t.len()        // usize     — element count
t.is_scalar()  // bool      — ndim() == 0
t.is_vector()  // bool      — ndim() == 1
t.is_matrix()  // bool      — ndim() == 2
t.as_slice()   // &[f64]    — flat row-major view
```

## Conversion out

```rust
let v: Vec<f64>        = t.to_vec();       // clone
let v: Vec<f64>        = t.into_vec();     // move, no copy
let v: Vec<f64>        = Vec::from(&t);    // borrow-clone
let v: Vec<f64>        = t.into();         // consuming From
let rows: Vec<Vec<f64>> = t.try_into()?;   // fails for non-rank-2
```

## Migration to faster libraries

When a PoC moves to a performance-sensitive path, hand the flat data to a
specialised crate:

```rust
let flat: Vec<f64> = tensor.into_vec(); // zero-copy move
// pass `flat` to ndarray, nalgebra, candle, etc.
```
