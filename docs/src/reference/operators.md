# Operators and broadcasting

`matten` implements element-wise arithmetic for borrowed tensors with
NumPy-style right-aligned broadcasting. All results are new owned tensors;
operands are never mutated.

## Element-wise operators

```rust
use matten::Tensor;

let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
let b = Tensor::full(&[2, 2], 10.0);

let c = &a + &b;  // [11.0, 12.0, 13.0, 14.0]
let d = &a - &b;  // [-9.0, -8.0, -7.0, -6.0]
let e = &a * &b;  // [10.0, 20.0, 30.0, 40.0]  ← element-wise, not matmul
let f = &a / &b;  // [0.1,  0.2,  0.3,  0.4]
let g = -&a;       // [-1.0, -2.0, -3.0, -4.0]
```

**`*` is always element-wise.** Matrix multiplication is explicit via `matmul` / `dot`.

## Scalar operators

All eight scalar forms are supported:

```rust
let t = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);

// tensor on left
let r = &t + 10.0;   // [11.0, 12.0, 13.0]
let r = &t * 2.0;    // [2.0, 4.0, 6.0]

// scalar on left
let r = 10.0 + &t;   // [11.0, 12.0, 13.0]
let r = 2.0 * &t;    // [2.0, 4.0, 6.0]
```

## Broadcasting rules

Shapes are compatible when aligned from the right and each dimension pair
satisfies one of:

- dimensions are equal;
- one dimension is `1` (it broadcasts to match the other);
- one operand has fewer dimensions (the missing leading axes are treated as `1`).

| Left | Right | Result |
|---|---|---|
| `[]` | `[3, 4]` | `[3, 4]` — scalar broadcasts everywhere |
| `[4]` | `[3, 4]` | `[3, 4]` — row vector broadcasts across rows |
| `[3, 1]` | `[1, 4]` | `[3, 4]` — outer product pattern |
| `[2, 3]` | `[2]` | **incompatible** — panics |

Read broadcasting from the trailing axis leftward:

```text
matrix:  [2, 3]
bias:       [3]
          -----
result:  [2, 3]

axis -1: 3 matches 3
axis -2: bias has no axis, so it behaves like 1 and repeats over 2 rows
```

```rust
// bias addition: add a [3] bias vector to every row of a [2, 3] matrix
let matrix = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
let bias   = Tensor::new(vec![10.0, 20.0, 30.0], &[3]);
let result = &matrix + &bias;
// [[11.0, 22.0, 33.0],
//  [14.0, 25.0, 36.0]]
```

The data meaning is "repeat the smaller shape where it has a missing axis or a
dimension of `1`":

```text
matrix [2, 3]       bias [3]         result [2, 3]

[ 1  2  3 ]       [10 20 30]       [11 22 33]
[ 4  5  6 ]   +   [10 20 30]   =   [14 25 36]
```

Two one-length axes can expand in different directions:

```text
left [3, 1]        right [1, 4]       result [3, 4]

[1]                [10 20 30 40]      [11 21 31 41]
[2]          +                         [12 22 32 42]
[3]                                    [13 23 33 43]
```

## Incompatible shapes

Incompatible shapes panic in operator code with an actionable message:

```text
matten broadcast error in add: shapes [2, 3] and [2] are not compatible
```

## IEEE 754 semantics

`matten` does not intercept `NaN` or `inf`:

- Division by zero produces `inf`, `-inf`, or `NaN` per IEEE 754.
- `NaN` propagates through all arithmetic.
- No silent sanitisation.

## No intermediate copies

The broadcast implementation maps result coordinates directly to source
element indices using zero-stride tricks. No expanded broadcast copies of
the operands are allocated.

## Elementwise comfort math (RFC-038)

Beyond the operators above, `Tensor` provides a few familiar elementwise
transforms. Each preserves shape, follows ordinary `f64` NaN/Inf behavior, and
panics on dynamic tensors (call `try_numeric()` first):

| Method | Effect |
|---|---|
| `abs()` | absolute value |
| `sqrt()` | square root (negative → `NaN`) |
| `exp()` | `e^x` |
| `ln()` | natural log (`ln(0.0)` → `-inf`, negative → `NaN`) |
| `clip(min, max)` | clamp each element into `[min, max]` |

```rust
use matten::Tensor;
let t = Tensor::from_vec(vec![-5.0, 0.5, 9.0]);
assert_eq!(t.clip(0.0, 1.0).as_slice(), &[0.0, 0.5, 1.0]);
```

`clip` panics if `min > max`; `try_clip(min, max)` returns
`MattenError::InvalidArgument` instead (or `MattenError::Unsupported` on a dynamic
tensor).
