# Bridge conversion contracts

A **bridge crate** converts a `matten::Tensor` to and from a specific external type (for
example `matten-ndarray` ↔ `ndarray::ArrayD<f64>`). Because a conversion can silently lose or
reshape data if its rules are vague, every bridge crate documents a **conversion contract**:
a fixed set of dimensions that say exactly what the conversion does. This page gives the
template and the filled-in contract for the reference bridge, `matten-ndarray`.

## The contract template

Every bridge contract documents these dimensions:

| Dimension | What it states |
|---|---|
| Source type | The `matten` side of the conversion. |
| Target type | The external type. |
| Direction | One-way or bidirectional, and the function names. |
| Copy / view behavior | Whether data is copied or shared (zero-copy). |
| Shape / rank policy | How shape is preserved and any rank limits. |
| Memory-order policy | Row-major vs column-major, and how non-standard layouts are handled. |
| Dynamic-tensor policy | What happens to dynamic (non-numeric/missing-capable) tensors. |
| NaN policy | Whether `NaN`/`inf` are passed through or treated specially. |
| Missing-value policy | How missing/None values are handled (if reachable at all). |
| Integer / text / bool policy | How non-`f64` element kinds are handled (if reachable). |
| Error behavior | `Result` vs panic, and the error type/variants. |
| Performance caveat | The cost the caller must plan around. |
| Examples | Runnable conversion snippets. |

Two rules are constant across all bridges (see [bridge-crate policy](./bridge-crate-policy.md)):
conversions **return `Result` and never panic** on rejected input, and a bridge crate
**does not re-export** core `Tensor`.

## Reference contract: `matten-ndarray`

`matten-ndarray` converts between `matten::Tensor` and `ndarray::ArrayD<f64>`.

| Dimension | `matten-ndarray` |
|---|---|
| Source / target type | `matten::Tensor` ↔ `ndarray::ArrayD<f64>` |
| Direction | Bidirectional: `to_arrayd(&Tensor)`, `from_arrayd(ArrayD<f64>)` |
| Copy / view | **Copies both directions.** No zero-copy is claimed. |
| Shape / rank | Shape is preserved exactly. Rank is bounded by core `matten`; an over-rank array is rejected via the core validation error. |
| Memory order | **Row-major logical order both ways.** `from_arrayd` reads *logical* order, so a transposed/sliced/non-standard-layout `ArrayD` converts correctly instead of being silently transposed. |
| Dynamic-tensor policy | **Rejected.** `to_arrayd` on a dynamic tensor returns `DynamicTensor` (a `Result`, not a panic). The guard is unconditional — it does not depend on the `dynamic` feature being enabled. |
| NaN policy | Passed through as ordinary `f64` values; no special handling. |
| Missing-value policy | Not reachable: only numeric tensors convert, and dynamic tensors (which can carry missing values) are rejected first. |
| Integer / text / bool policy | Not reachable: `matten`'s numeric `Tensor` is `f64` only; non-numeric element kinds live in the rejected dynamic model. |
| Error behavior | Returns `Result<_, MattenNdarrayError>`; never panics. Variants: `DynamicTensor`, `ZeroSizedAxis(shape)` (core has no zero-length axes), `NdarrayShape(..)` (ndarray shape mismatch), `Matten(MattenError)` (wraps a core validation error). |
| Performance caveat | Both directions allocate and copy. Convert **once** at the boundary, not inside a hot loop. |
| Examples | See below. |

### Examples

```rust
use matten::Tensor;
use matten_ndarray::{to_arrayd, from_arrayd};

// Tensor -> ArrayD<f64> (copies; row-major)
let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
let arr = to_arrayd(&t)?;
assert_eq!(arr[[1, 0]], 3.0);

// ArrayD<f64> -> Tensor preserves *logical* order, even for a transposed array
let back = from_arrayd(arr.t().to_owned())?; // logical shape [2, 2], transposed
# Ok::<(), matten_ndarray::MattenNdarrayError>(())
```

A dynamic tensor is rejected rather than guessed:

```rust,ignore
// to_arrayd(&dynamic_tensor)  ->  Err(MattenNdarrayError::DynamicTensor)
// Resolve to a numeric tensor first (e.g. try_numeric()), then convert.
```

## Error-category note

The generic error categories sketched in RFC-051 (`UnsupportedTensorKind`, `UnsupportedRank`,
…) are **illustrative for future bridges, not a required enum schema**. `matten-ndarray`'s
existing variants (`DynamicTensor` / `ZeroSizedAxis` / `NdarrayShape` / `Matten`) document its
contract clearly and are compliant as-is; a bridge need not rename or expand its error enum
to match the sketch.
