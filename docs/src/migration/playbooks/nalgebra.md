# Migrating to `nalgebra`

[`nalgebra`](https://docs.rs/nalgebra) is the dense linear-algebra crate: statically- and
dynamically-sized vectors and matrices, plus decompositions (LU, QR, SVD), solvers, and
eigenvalues. It is the right target when your workload is fundamentally **small/mid dense
linear algebra**, especially when you need results `matten` intentionally does not provide.

There is **no `matten-nalgebra` bridge crate today** — conversion is manual (a few lines) and
a dedicated bridge is only a documented future direction, not a commitment.

## Choose this target when

- You need **decompositions or solvers**: LU, QR, SVD, eigenvalues, linear systems.
- Your data is naturally **small/mid dense vectors and matrices**.
- You want a typed linear-algebra API rather than general N-D arrays.

## Do not choose this target when

- You need general **N-D arrays** or BLAS-backed bulk array ops → prefer
  [`ndarray`](./ndarray.md).
- The work is small or not hot → **stay with `matten`**.
- The real need is **tabular** or **ML** → Polars/Pandas or Candle.

## Concept mapping

| `matten` | `nalgebra` |
|---|---|
| `Tensor` of shape `[n]` | `DVector<f64>` |
| `Tensor` of shape `[r, c]` (row-major) | `DMatrix<f64>` (column-major — see pitfalls) |
| `.matmul(&b)` | `&a * &b` |
| matrix–vector | `&m * &v` |
| `.dot(&b)` (vectors) | `a.dot(&b)` |
| `.transpose()` | `.transpose()` |
| decompositions (not in `matten`) | `.lu()`, `.qr()`, `.svd(..)`, `.symmetric_eigen()`, … |

## Example migrations

- `20_dot_product` / `21_matrix_vector_product` → `nalgebra` `DVector`/`DMatrix` operations.
- `22_matrix_multiplication` → `nalgebra` `&a * &b` (or `ndarray` for general N-D).
- `31_fibonacci_matrix_power` → `nalgebra` matrix powers.
- `35_linear_regression_gradient_descent` → `nalgebra` when you want a typed matrix/vector
  formulation (or want to switch to a closed-form solve via a decomposition).

## Conversion path

Manual, via flat row-major data. **`DMatrix` is column-major**, so build it from a row-major
slice with `from_row_slice`, which reads the source in row-major order:

```rust
use matten::Tensor;
use nalgebra::{DMatrix, DVector};

// vector
let v = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
let dv = DVector::from_vec(v.into_vec());

// matrix (row-major source -> from_row_slice keeps the logical layout)
let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
let shape = m.shape().to_vec();
let dm = DMatrix::from_row_slice(shape[0], shape[1], &m.into_vec());

// ... decompositions / solvers / matrix algebra here ...
```

To return to `matten`, read the matrix back in row-major order (transpose as needed) and
rebuild with `Tensor::new(data, &shape)`.

## Common pitfalls

- **Column-major trap.** Do not feed a row-major flat `Vec` to a column-major constructor as
  if it were column-major — use `from_row_slice` or transpose deliberately, or you will
  silently transpose your data.
- **Convert once** at the boundary; conversions copy.
- **Make dynamic tensors numeric first** (`try_numeric()`).

## Performance / positioning notes

In the accepted RFC-049 Rust peer comparison (task-scoped, small fixed sizes, single
machine — not a ranking), `nalgebra` had lower overhead than `matten` on dense `matmul` and
matrix–vector kernels, while a lighter vector task was competitive. The value of `nalgebra`,
though, is usually **capability** rather than raw speed: decompositions and solvers that
`matten` does not implement at all. If you need those, the migration is about *what you can
compute*, not just how fast.

## Minimal checklist

- [ ] You need dense linear algebra or a decomposition/solver `matten` does not provide.
- [ ] You build `DMatrix` with `from_row_slice` (or transpose deliberately).
- [ ] You convert once at the boundary; the tensor is numeric first.
- [ ] You kept `matten` for the parts where it was already a good fit.
