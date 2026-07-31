# Shape composition

Shape composition joins, repeats, or grids tensors. `matten` provides six functions
on the numeric `Tensor` only, across two themes:

- [`concatenate`](#concatenate) — join along an **existing** axis (RFC-039).
- [`stack`](#stack) — join along a **new** axis (RFC-039).
- [`repeat` / `repeat_axis`](#repeat--repeat_axis) — repeat each **element**
  (RFC-087).
- [`tile`](#tile) — repeat the **whole tensor** (RFC-087).
- [`meshgrid`](#meshgrid) — build coordinate grids for evaluating `f(x, y)`
  (RFC-087).

Each has a panicking convenience form and a non-panicking `try_*` form. All reject
dynamic tensors — convert with `try_numeric()` first — and check the output
allocation against `MattenLimits` before copying any data.

## concatenate

```rust
Tensor::concatenate(tensors: &[&Tensor], axis: usize) -> Tensor
Tensor::try_concatenate(tensors: &[&Tensor], axis: usize) -> Result<Tensor, MattenError>
```

All inputs must have the **same rank** and the **same size on every axis except**
`axis`. The output `axis` size is the sum of the inputs' `axis` sizes; all other
axes are unchanged. `axis` must be in `0..rank`.

```text
[2, 3] ++ [4, 3]  along axis 0  ->  [6, 3]
[2, 3] ++ [2, 5]  along axis 1  ->  [2, 8]
```

`concatenate` extends an axis that already exists:

```text
axis 0 concatenate: add more rows

[ a a a ]      [ b b b ]      [ a a a ]
[ a a a ]  ++  [ b b b ]  ->  [ a a a ]   shape [4, 3]
                              [ b b b ]
                              [ b b b ]

axis 1 concatenate: add more columns

[ a a a ]      [ b b ]      [ a a a b b ]
[ a a a ]  ++  [ b b ]  ->  [ a a a b b ]   shape [2, 5]
```

A single-element list returns a clone of that tensor (after validating the axis and
dynamic status).

## stack

```rust
Tensor::stack(tensors: &[&Tensor], axis: usize) -> Tensor
Tensor::try_stack(tensors: &[&Tensor], axis: usize) -> Result<Tensor, MattenError>
```

All inputs must have **identical shapes**. A new axis of size *n* (the number of
inputs) is inserted at position `axis`, so the output rank is the input rank plus
one. `axis` may be `0..=rank`.

```text
three [2, 4] tensors stacked at axis 0  ->  [3, 2, 4]
three [2, 4] tensors stacked at axis 1  ->  [2, 3, 4]
three [2, 4] tensors stacked at axis 2  ->  [2, 4, 3]
```

`stack` creates a new axis whose size is the number of inputs:

```text
two vectors shape [3]

a = [1 2 3]
b = [4 5 6]

stack([a, b], axis 0) -> shape [2, 3]

[ 1 2 3 ]
[ 4 5 6 ]

stack([a, b], axis 1) -> shape [3, 2]

[ 1 4 ]
[ 2 5 ]
[ 3 6 ]
```

The short rule:

```text
concatenate: existing axis gets longer
stack:       new axis appears
```

A single-element list inserts a length-1 axis (the analogue of `expand_dims`).

## `repeat` / `repeat_axis`

```rust
Tensor::repeat(&self, n: usize) -> Tensor
Tensor::try_repeat(&self, n: usize) -> Result<Tensor, MattenError>

Tensor::repeat_axis(&self, n: usize, axis: usize) -> Tensor
Tensor::try_repeat_axis(&self, n: usize, axis: usize) -> Result<Tensor, MattenError>
```

`repeat` repeats each **element** `n` times, flattening the result to rank 1:

```text
[1, 2, 3].repeat(2)  ->  [1, 1, 2, 2, 3, 3]
```

`repeat_axis` repeats each element `n` times along `axis`, preserving rank:

```text
[[1, 2], [3, 4]].repeat_axis(2, 0)  ->  [[1, 2], [1, 2], [3, 4], [3, 4]]
```

A rank-0 scalar `.repeat(n)` produces a rank-1 tensor of length `n`; `repeat_axis`
on a rank-0 scalar is a `Shape` error (there is no axis to repeat along). `n = 0` is
also a `Shape` error — the shape model has no representation for a zero-sized
dimension, so this is explicit rather than an empty tensor.

**`repeat` is explicit allocation, unlike broadcasting**, which is implicit and
materializes nothing: `[1, 2, 3] * 2` and `[1, 2, 3].repeat(2)` differ for exactly
that reason — the first never allocates a doubled-length tensor, the second always
does.

## `tile`

```rust
Tensor::tile(&self, reps: &[usize]) -> Tensor
Tensor::try_tile(&self, reps: &[usize]) -> Result<Tensor, MattenError>
```

`tile` repeats the **whole tensor**, one repetition factor per axis:

```text
[1, 2, 3].tile(&[2])         ->  [1, 2, 3, 1, 2, 3]
[[1, 2]].tile(&[2, 1])        ->  [[1, 2], [1, 2]]
```

**`repeat` repeats elements; `tile` repeats the whole tensor** — the single most
confused pair in this area:

```text
[1, 2, 3].repeat(2)   -> [1, 1, 2, 2, 3, 3]   (each element, in place)
[1, 2, 3].tile(&[2])  -> [1, 2, 3, 1, 2, 3]   (the whole tensor, twice)
```

If `reps` is **shorter** than the input's rank, it is padded with leading `1`s
(NumPy-compatible). If `reps` is **longer** than the rank, this is an explicit
`Shape` error naming both lengths — NumPy would silently promote the tensor's rank
instead, which `matten` treats as the surprising direction, not the safe one: the
result would have more dimensions than the input, with no obvious place for a
caller to look. This is a deliberate, one-directional divergence from NumPy (see
below). `reps` must be non-empty and every entry nonzero.

## `meshgrid`

```rust
Tensor::meshgrid(x: &Tensor, y: &Tensor) -> (Tensor, Tensor)
Tensor::try_meshgrid(x: &Tensor, y: &Tensor) -> Result<(Tensor, Tensor), MattenError>
```

Builds the two coordinate grids for evaluating a function of two variables over a
grid. `x` and `y` must both be rank-1 (a rank-2 input is a `Shape` error, never
silently flattened). For `x` of length `m` and `y` of length `n`, both outputs have
shape `[n, m]`, using NumPy's **`xy`** indexing:

```text
out_x[i][j] == x[j]     (each row is a full copy of x)
out_y[i][j] == y[i]     (each row is constant, equal to y[i])
```

```text
x = [1, 2, 3]        (len 3)
y = [10, 20]         (len 2)

meshgrid(x, y) -> both outputs shape [2, 3]

out_x = [[1, 2, 3], [1, 2, 3]]
out_y = [[10, 10, 10], [20, 20, 20]]
```

`xy` is used deliberately, matching NumPy's default, even though the alternative
`ij` convention (`out[i][j] == (x[i], y[j])`) can feel like the more natural matrix
reading. **When `x` and `y` have equal length, `xy` and `ij` differ only by a
transpose** — an invisible mistake with no shape error to catch it — so this matches
the ecosystem instead of diverging on an axis a caller cannot see. A reader who
specifically wants `ij` gets it by transposing both outputs.

## The `tile`/`meshgrid` divergence principle

`tile`'s rank-promotion rejection and `meshgrid`'s NumPy-matching `xy` indexing look
like opposite choices — one matches the ecosystem, one does not — but both follow
the same rule:

```text
MATCH the ecosystem when a divergence would be SILENT — wrong numbers, or a
      wrong shape the caller cannot see.                (meshgrid's indexing)

DIVERGE where the ecosystem's own behaviour is itself implicit, the divergence
      surfaces as an explicit error, and the error teaches.   (tile's rank promotion)
```

This is not a license to diverge generally — it is consistent with `matten`'s
standing preference for explicit over silent behaviour.

## Errors

| Condition | `try_*` returns |
|---|---|
| empty input list (`concatenate`/`stack`) | `InvalidArgument { argument: "tensors" }` |
| any dynamic input | `Unsupported` (convert with `try_numeric()` first) |
| rank / dimension / shape mismatch | `Shape` |
| axis out of range (`0..rank` for `concatenate`, `0..=rank` for `stack`/`repeat_axis`) | `Shape` |
| `n = 0` (`repeat`/`repeat_axis`), empty or zero-containing `reps` (`tile`) | `Shape` |
| `reps` longer than rank (`tile`), non-rank-1 input (`meshgrid`) | `Shape` |
| `repeat_axis` on a rank-0 scalar | `Shape` |
| result exceeds the allocation limit | `Allocation` |

The convenience forms panic with the same message the `try_*` forms would return.

## Allocation safety

The output shape is checked against [`MattenLimits`](./compatibility.md) before any
data is copied, so an oversized result fails with `Allocation` (or `Shape` when the
stacked rank would exceed the dimension limit) rather than attempting a huge
allocation. `repeat`, `tile`, and `meshgrid` all multiply sizes and can overflow
trivially, so every output size is computed with a checked product before
allocating — never a bare `*`.

## Example

See [`14_concatenate_stack.rs`](https://github.com/nabbisen/matten/blob/main/crates/matten/examples/14_concatenate_stack.rs)
for `concatenate`/`stack`, and
[`58_repeat_tile_meshgrid.rs`](https://github.com/nabbisen/matten/blob/main/crates/matten/examples/58_repeat_tile_meshgrid.rs)
for `repeat`/`tile`/`meshgrid`, for runnable walkthroughs.
