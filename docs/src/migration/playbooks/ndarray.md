# Migrating to `ndarray`

[`ndarray`](https://docs.rs/ndarray) is the general Rust N-D array crate: strided arrays,
views, broadcasting, and (with a BLAS backend) fast matrix multiplication. It is the natural
first production target when a dense numeric workload outgrows `matten`. A dedicated bridge
crate, **`matten-ndarray`**, provides a contract-backed conversion in both directions.

## Choose this target when

- You have general **N-D numeric arrays** and need production-grade array operations.
- **Dense `matmul`** or **axis reductions** are a measured hot path at scale.
- You want strided views, advanced indexing, or a BLAS backend.

## Do not choose this target when

- The work is small or not on a hot path — staying with `matten` is simpler.
- You fundamentally need **linear-algebra results** (LU/QR/SVD, solvers, eigenvalues) →
  prefer [`nalgebra`](./nalgebra.md).
- The real need is **tabular** (group-by/join/pivot) → Polars/Pandas, not an array crate.

## Concept mapping

| `matten` | `ndarray` |
|---|---|
| `Tensor` (row-major `f64`) | `ArrayD<f64>` / `Array1`/`Array2` |
| `Tensor::new(data, &[r, c])` | `Array2::from_shape_vec((r, c), data)` |
| `.matmul(&b)` | `a.dot(&b)` |
| `.sum_axis(i)` / `.mean_axis(i)` | `a.sum_axis(Axis(i))` / `a.mean_axis(Axis(i))` |
| elementwise `&a + &b` | `&a + &b` |
| `.reshape(&[..])` | ndarray's current reshape APIs (e.g. `to_shape` / `into_shape_with_order`, per ownership/layout) |
| `.shape()` | `.shape()` / `.dim()` |

## Example migrations

These map directly from the shipped examples:

- `22_matrix_multiplication` → `ndarray` when the matrices are large or the multiply is hot.
- `27_axis_reductions` → `ndarray`; axis reductions are exactly where `matten`'s internal
  baseline flagged the widest internal cost, so this is a strong candidate to move.
- `35_linear_regression_gradient_descent` → `ndarray` (or `nalgebra`) once the GD loop runs
  on real-sized design matrices.
- `50_rowwise_scoring` → `ndarray` if rows get large, otherwise **stay with `matten`**.

## Conversion path

The clean path is the **`matten-ndarray` bridge**, which copies, is numeric-only, rejects
dynamic tensors, and preserves logical row-major order:

```rust
use matten::Tensor;
use matten_ndarray::{to_arrayd, from_arrayd};

let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);

// matten Tensor -> ndarray ArrayD<f64> (copies)
let arr = to_arrayd(&t)?;

// ... heavy ndarray work here (dot, BLAS matmul, axis reductions, views) ...

// ndarray ArrayD<f64> -> matten Tensor (copies)
let back = from_arrayd(arr)?;
# Ok::<(), matten_ndarray::MattenNdarrayError>(())
```

If you are not using the bridge crate, the manual path via flat data also works (see the
[reference page](../../reference/migration.md)): `t.into_vec()` / `t.shape()` into
`ArrayD::from_shape_vec(shape, flat)`.

## Common pitfalls

- **Convert once.** Both directions copy — do it at the boundary, not inside the hot loop.
- **Dynamic tensors are rejected.** Make the tensor numeric (`try_numeric()`) before
  converting; the bridge returns a `DynamicTensor` error rather than guessing.
- **Reshape APIs moved.** Prefer ndarray's current reshape APIs over the deprecated
  `into_shape`; check the ndarray version `matten-ndarray` pins before copying snippets.

## Performance / positioning notes

In the accepted RFC-049 Rust peer comparison (task-scoped, small fixed sizes, single
machine — not a ranking), dense `matmul` and matrix–vector tasks showed the widest gap to
`ndarray` (roughly an order of magnitude at those sizes), while a lighter vector task was
competitive. The practical reading: if dense `matmul`, matrix–vector, or axis-reduction
kernels are your measured hot paths, moving *those* to `ndarray` is where the benefit is
concentrated. This is positioning, not a claim that either library is "better" in general.

## Minimal checklist

- [ ] The hot path is a dense array kernel you have actually measured.
- [ ] You convert once at the boundary, not per iteration.
- [ ] The tensor is numeric (no dynamic elements) before conversion.
- [ ] You kept `matten` for construction/ingestion/glue where it was already fine.
