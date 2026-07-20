# Linear algebra (core-lite)

> Core `matten` provides small linalg-adjacent helpers, not a linear algebra
> backend. `matten` prioritizes PoC ergonomics, not numerical linear algebra
> performance or stability leadership.

`matten` offers exactly three linalg-adjacent helpers (RFC-041), alongside the
`dot`/`matmul` already in [Reductions and matrix multiplication](./math.md):

- [`norm`](#norm) — L2 / Frobenius norm over all elements.
- [`trace`](#trace) — diagonal sum of a rank-2 tensor.
- [`outer`](#outer) — rank-1 × rank-1 outer product.

## norm

```rust
Tensor::norm(&self) -> f64
Tensor::try_norm(&self) -> Result<f64, MattenError>
```

The L2 / Frobenius norm over **all** elements: `sqrt(sum(x_i^2))`. It works at any
rank — for a matrix this is the Frobenius norm. `NaN` propagates (any `NaN` element
yields `NaN`). No overflow-avoidance scaling is applied, so extreme magnitudes may
overflow to infinity.

`try_norm` returns `MattenError::Unsupported` on a dynamic tensor; `norm` panics
in that case. Convert with `try_numeric()` first when working from dynamic data.

```text
norm([3, 4])          = 5            // sqrt(9 + 16)
norm([[1, 2], [2, 4]]) = 5           // Frobenius: sqrt(1 + 4 + 4 + 16)
```

## trace

```rust
Tensor::trace(&self) -> f64
Tensor::try_trace(&self) -> Result<f64, MattenError>
```

The sum of the diagonal of a **rank-2** tensor. Rectangular matrices are allowed:
the trace sums `self[i, i]` for `i in 0..min(rows, cols)`.

```text
trace([[1, 2], [3, 4]])             = 5   // 1 + 4
trace([[1, 2, 3], [4, 5, 6]])       = 6   // min(2,3)=2 -> self[0,0] + self[1,1]
```

`try_trace` returns `MattenError::Shape` if the tensor is not rank-2, or
`MattenError::Unsupported` on a dynamic tensor; `trace` panics in those cases.

## outer

```rust
Tensor::outer(&self, other: &Tensor) -> Tensor
Tensor::try_outer(&self, other: &Tensor) -> Result<Tensor, MattenError>
```

The outer product of two **rank-1** tensors: `out[i, j] = self[i] * other[j]`, with
shape `[self.len(), other.len()]`. The output is checked against
[`MattenLimits`](./compatibility.md) before allocation.

```text
[1, 2, 3] ⊗ [4, 5]  ->  [[4, 5], [8, 10], [12, 15]]   // shape [3, 2]
```

`try_outer` returns `MattenError::Shape` if either input is not rank-1,
`MattenError::Unsupported` on a dynamic tensor, or `MattenError::Allocation` if the
result exceeds the limit; `outer` panics in those cases.

## Out of scope for core

The following are intentionally **not** in core `matten` (RFC-041 §5):

```text
inverse        determinant     solve          least_squares
eigenvalues    eigenvectors    SVD            QR
LU             Cholesky        sparse         BLAS / LAPACK
```

For serious numerical linear algebra, use a specialized crate such as `nalgebra`
or `ndarray-linalg`. A future `matten-nalgebra` / `matten-ndarray-linalg` bridge
would require its own RFC.

## Example

See [`15_norm_trace_outer.rs`](https://github.com/nabbisen/matten/blob/main/crates/matten/examples/15_norm_trace_outer.rs)
for a runnable walkthrough.
