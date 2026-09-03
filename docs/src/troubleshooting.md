# Troubleshooting

Symptom → cause → fix, for the messages you actually hit. Every message below is copied verbatim
from the source (verified by grep against `crates/matten/src`) — if what you're seeing doesn't
match one of these exactly, it isn't the same problem.

## "no associated function or constant named `from_csv` found for struct `Tensor`"

```text
error[E0599]: no associated function or constant named `from_csv` found for struct `Tensor`
```

The `csv` (or `json`, for `from_json`) feature is off. This happens on the lean profile:

```toml
matten = { version = "0.47.0", default-features = false }
```

Either drop `default-features = false` to get the default profile (`serde` + `json` + `csv`), or
opt back in explicitly:

```toml
matten = { version = "0.47.0", features = ["csv"] }
```

See [Quick start](./quick-start.md) and [Cargo features](./reference/boundary.md#cargo-features).

## "matten shape error in ..."

```text
matten shape error in try_new: data length 3 does not match shape [2, 2], which requires 4 elements
```

The data you passed doesn't have exactly `shape.iter().product()` elements. Recompute the shape
from the data's actual length, or recompute the data from the shape you intend — the message names
both numbers so you can tell which one is wrong. See
[Data model and lifecycle](./reference/data-model.md).

## "matten broadcast error in ..."

```text
matten broadcast error in add: shapes [2, 3] and [2] are not compatible
```

The two shapes can't broadcast against each other. Shapes are compared right-aligned: dimensions
must be equal, or one of them must be `1`. A trailing `[2]` only broadcasts against another axis of
size `2` or `1` at the *same* right-aligned position — `[2, 3]` and `[2]` line up `3` against `2`,
which is neither equal nor `1`. See [Operators and broadcasting](./reference/operators.md#broadcasting-rules)
for the full alignment rule, and use `try_add`/`try_sub`/`try_mul`/`try_div` if you'd rather get a
`Result` back than have this panic.

## "matten unsupported error in ..."

```text
matten unsupported error in clip: clip is not supported on dynamic tensors; call try_numeric() first
```

A numeric-only API (arithmetic, elementwise math, `matmul`, reductions, ...) was called on a
**dynamic** tensor (the `dynamic` feature's heterogeneous `Element` engine). Convert first:

```rust,ignore
let numeric = dynamic_tensor.try_numeric()?;
```

See [Dynamic feature (Element model)](./reference/dynamic.md).

## "matten allocation error: ..."

```text
matten allocation error: try_zeros requested 4000000 elements, exceeding the limit of 1048576
(MattenLimits::max_elements); use smaller shapes (requested 4000000 elements)
```

The requested shape exceeds [`MattenLimits::max_elements`](./reference/compatibility.md) (default
1,048,576 — about 8 MiB of `f64`). This is a safety limit against runaway allocations from
caller-supplied shapes; it does **not** apply to ordinary operations on tensors already in memory
(arithmetic, reductions, slicing). If your shape is legitimately larger, pass a custom
[`MattenLimits`](./reference/compatibility.md) to the `_with_limits` form of the constructor you're
calling (`try_zeros_with_limits`, `try_ones_with_limits`, `try_full_with_limits`).
