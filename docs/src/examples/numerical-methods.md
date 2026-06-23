# Numerical methods

Small numerical-method examples that demonstrate how iterative and sampled-grid
algorithms look in `matten`. They use only the default numeric Tensor API (plus the
RFC-038 comfort APIs), small hard-coded inputs, and deterministic output.

These are teaching examples, not a SciPy replacement.

## Examples

### `35_linear_regression_gradient_descent.rs`

*Difficulty: Advanced-small.* Fits `y = w·x + b` by batch gradient descent on
mean-squared error. The data is stacked into a design matrix with a bias column, so
predictions are `X · θ` and the gradient is `(2/n)·Xᵀ·(ŷ - y)` — one `matmul` for
each, with `transpose` forming `Xᵀ` once. Converges to the true line `y = 2x + 1`.

```bash
cargo run --example 35_linear_regression_gradient_descent
```

Source: [`35_linear_regression_gradient_descent.rs`](https://github.com/nabbisen/matten/blob/main/crates/matten/examples/35_linear_regression_gradient_descent.rs)

### `36_heat_equation_1d.rs`

*Difficulty: Advanced-small.* Evolves the 1D heat equation on a rod with fixed-end
temperatures using the explicit (forward-Euler) finite-difference update. The stencil
is encoded as a tridiagonal matrix `A` (with identity rows at the boundaries), so each
time step is `u_next = A · u`. The profile converges to the steady-state straight line
between the boundary temperatures.

```bash
cargo run --example 36_heat_equation_1d
```

Source: [`36_heat_equation_1d.rs`](https://github.com/nabbisen/matten/blob/main/crates/matten/examples/36_heat_equation_1d.rs)

### `39_finite_difference_derivative.rs`

*Difficulty: Intermediate.* Approximates the derivative of `f(x) = x³` sampled on a
`linspace` grid using the central difference `(f(x+h) − f(x−h)) / (2h)`. The grid and
the function values are `Tensor`s (the latter via elementwise `&x * &x`). For a cubic
the central-difference error is exactly `h²`, so the example shows the approximation
quality directly. It is a numerical approximation, not symbolic differentiation.

```bash
cargo run --example 39_finite_difference_derivative
```

Source: [`39_finite_difference_derivative.rs`](https://github.com/nabbisen/matten/blob/main/crates/matten/examples/39_finite_difference_derivative.rs)

### `40_trapezoidal_integration.rs`

*Difficulty: Intermediate.* Approximates `∫₀¹ x² dx` with the composite trapezoidal
rule and compares against the known exact value `1/3`. The grid comes from `linspace`,
the values from elementwise squaring, and the running total from a `Tensor::sum`
reduction. It is a numerical approximation, not an integration library.

```bash
cargo run --example 40_trapezoidal_integration
```

Source: [`40_trapezoidal_integration.rs`](https://github.com/nabbisen/matten/blob/main/crates/matten/examples/40_trapezoidal_integration.rs)

## What this is not

These are single-file demonstrations of accepted APIs. They do not imply that
`matten` is an optimization library, a PDE/finite-element framework, or a SciPy
replacement.
