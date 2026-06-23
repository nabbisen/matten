# RFC-046: Numerical Methods and Scientific Toy Examples

**Status:** Proposed  
**Target Release:** v0.21.0, partially v0.20.x if APIs already exist  
**Related:** RFC-038, RFC-039, RFC-041  
**Scope:** Medium examples for numerical methods and scientific computation

---

## 1. Summary

This RFC adds “so-so complicated” examples that make `matten` feel useful for small scientific/numerical experiments.

Initial examples:

```text
examples/35_linear_regression_gradient_descent.rs
examples/36_heat_equation_1d.rs
```

Optional after RFC-038 comfort APIs:

```text
examples/39_finite_difference_derivative.rs
examples/40_trapezoidal_integration.rs
```

---

## 2. Motivation

Users often evaluate numeric libraries by asking:

```text
Can I fit a line?
Can I simulate a small system?
Can I express an iterative numerical method?
```

These examples demonstrate real numerical workflows without turning `matten` into SciPy.

---

## 3. Example 35: Linear Regression by Gradient Descent

### Problem

Fit a line:

```text
y = a*x + b
```

to a small set of points.

### Math idea

Use a design matrix:

```text
X = [x, 1]
w = [a, b]
prediction = Xw
loss = mean((prediction - y)^2)
```

Update weights by gradient descent.

### APIs demonstrated

- matrix construction;
- matrix-vector multiplication;
- subtraction;
- mean / reduction;
- scalar updates;
- iterative algorithm.

### Implementation note

Do not add an optimizer abstraction.

Keep the example local:

```text
for step in 0..N {
    ...
}
```

### Acceptance

```text
[ ] small hard-coded dataset
[ ] fixed learning rate
[ ] fixed iteration count
[ ] deterministic final parameters
[ ] comments explain this is not an ML framework
```

---

## 4. Example 36: 1D Heat Equation

### Problem

Simulate heat diffusion along a 1D rod.

Update rule:

```text
next[i] = current[i] + alpha * (current[i-1] - 2*current[i] + current[i+1])
```

### APIs demonstrated

- vector as grid;
- iteration;
- indexing/slicing;
- boundary conditions;
- stencil update.

### Implementation note

Use a simple local loop if slicing APIs are not ergonomic.

### Acceptance

```text
[ ] small grid
[ ] fixed boundary policy
[ ] prints initial and final temperatures
[ ] no PDE framework claim
```

---

## 5. Optional Example 39: Finite Difference Derivative (deferred until RFC-038)

### Requires

Recommended after RFC-038:

```text
Tensor::linspace
elementwise operations
```

### Problem

Approximate derivative of a sampled function.

### Acceptance

```text
[ ] clearly marked as numerical approximation
[ ] no symbolic math claim
[ ] small vector output
```

---

## 6. Optional Example 40: Trapezoidal Integration (deferred until RFC-038)

### Requires

Recommended after RFC-038:

```text
Tensor::linspace
elementwise operations
reductions
```

### Problem

Approximate area under a curve.

### Acceptance

```text
[ ] simple function, e.g. x^2 on [0, 1]
[ ] compares approximate result with known exact value
[ ] no integration-library claim
```

---

## 7. Documentation Requirements

Add docs page:

```text
docs/src/examples/numerical-methods.md
```

It should state:

```text
These examples demonstrate small numerical methods, not a SciPy replacement.
```

---

## 8. QA Checklist

```text
[ ] examples compile
[ ] deterministic outputs
[ ] no external data
[ ] no random seed unless fixed
[ ] no hidden dependency
[ ] docs explain approximation limits
```

CI:

```bash
cargo check -p matten --examples --all-features
cargo run -p matten --example 35_linear_regression_gradient_descent
cargo run -p matten --example 36_heat_equation_1d
```

Optional examples should be added only when dependent APIs are accepted.

---

## 9. Non-goals

- No optimizer API.
- No autograd.
- No neural network framework.
- No SciPy integration.
- No PDE solver framework.
- No plotting.
