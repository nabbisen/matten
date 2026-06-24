# Shape composition

Shape composition joins several tensors into one. `matten` provides two functions
(RFC-039), both on the numeric `Tensor` only:

- [`concatenate`](#concatenate) — join along an **existing** axis.
- [`stack`](#stack) — join along a **new** axis.

Each has a panicking convenience form and a non-panicking `try_*` form. Both take a
borrowed slice `&[&Tensor]`, so callers never have to clone inputs just to pass them.
Dynamic tensors are rejected — convert with `try_numeric()` first.

`repeat`, `tile`, and `meshgrid` are intentionally **deferred** (see RFC-039 §8):
they need a separate indexing and allocation policy, and are not part of the API.

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

A single-element list inserts a length-1 axis (the analogue of `expand_dims`).

## Errors

Both functions follow the same error policy:

| Condition | `try_*` returns |
|---|---|
| empty input list | `InvalidArgument { argument: "tensors" }` |
| any dynamic input | `Unsupported` (convert with `try_numeric()` first) |
| rank / dimension / shape mismatch | `Shape` |
| axis out of range (`0..rank` for `concatenate`, `0..=rank` for `stack`) | `Shape` |
| result exceeds the allocation limit | `Allocation` |

The convenience forms (`concatenate`, `stack`) panic with the same message the
`try_*` forms would return.

## Allocation safety

The output shape is checked against [`MattenLimits`](./compatibility.md) before any
data is copied, so an oversized result fails with `Allocation` (or `Shape` when the
stacked rank would exceed the dimension limit) rather than attempting a huge
allocation.

## Example

See [`14_concatenate_stack.rs`](https://github.com/nabbisen/matten/blob/main/crates/matten/examples/14_concatenate_stack.rs)
for a runnable walkthrough.
