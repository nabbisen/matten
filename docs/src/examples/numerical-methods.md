# Numerical methods

Small numerical-method examples that demonstrate how iterative algorithms look in
`matten` — an optimizer and a PDE solver, each reduced to repeated `Tensor::matmul`.
They use only the default Phase-1 numeric API, small hard-coded inputs, and
deterministic output.

These are teaching examples, not a SciPy replacement. Two further numerical examples
(`39_finite_difference_derivative`, `40_trapezoidal_integration`) are deferred until
the core comfort APIs of RFC-038 land.

## Examples

### `35_linear_regression_gradient_descent.rs`

*Difficulty: Advanced-small.* Fits `y = w·x + b` by batch gradient descent on
mean-squared error. The data is stacked into a design matrix with a bias column, so
predictions are `X · θ` and the gradient is `(2/n)·Xᵀ·(ŷ - y)` — one `matmul` for
each, with `transpose` forming `Xᵀ` once. Converges to the true line `y = 2x + 1`.

```bash
cargo run --example 35_linear_regression_gradient_descent
```

### `36_heat_equation_1d.rs`

*Difficulty: Advanced-small.* Evolves the 1D heat equation on a rod with fixed-end
temperatures using the explicit (forward-Euler) finite-difference update. The stencil
is encoded as a tridiagonal matrix `A` (with identity rows at the boundaries), so each
time step is `u_next = A · u`. The profile converges to the steady-state straight line
between the boundary temperatures.

```bash
cargo run --example 36_heat_equation_1d
```

## What this is not

These are single-file demonstrations of accepted APIs. They do not imply that
`matten` is an optimization library, a PDE/finite-element framework, or a SciPy
replacement.
