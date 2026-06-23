//! # Example: Trapezoidal Integration
//!
//! Run: cargo run --example 40_trapezoidal_integration
//!
//! ## Problem
//! Approximate the area under a curve (a definite integral) from sampled function
//! values, and compare against the known exact value.
//!
//! ## Math idea
//! The composite trapezoidal rule approximates `∫ f` by summing trapezoids between
//! adjacent samples:
//! `∫ ≈ h · (f₀/2 + f₁ + … + f_{n-1} + fₙ/2)`,
//! which equals `h · (Σ f − (f₀ + fₙ)/2)`. Here `f(x) = x²` on `[0, 1]`, whose exact
//! integral is `1/3`.
//!
//! ## Tensor representation
//! The sample grid is a 1-D `Tensor` from `linspace`; the function values are a
//! `Tensor` computed elementwise, and the running total uses a `Tensor` reduction.
//!
//! ## What this demonstrates
//! - `Tensor::linspace` for an evenly spaced grid;
//! - elementwise squaring (`&x * &x`) to evaluate `f`;
//! - a whole-tensor `sum` reduction in the trapezoidal formula.
//!
//! ## Expected output
//! ```text
//! trapezoidal estimate: 0.335000
//! exact integral (1/3): 0.333333
//! abs error:            0.001667
//! Trapezoidal integration: OK
//! ```
//!
//! This is a numerical approximation, not an integration library.

use matten::Tensor;

fn main() {
    // Evenly spaced sample grid on [0, 1].
    let x = Tensor::linspace(0.0, 1.0, 11);
    let xs = x.as_slice();
    let h = xs[1] - xs[0];

    // f(x) = x^2, evaluated with elementwise multiplication.
    let f = &x * &x;
    let fs = f.as_slice();
    let n = fs.len();

    // Composite trapezoidal rule: h * (Σ f - (f_first + f_last)/2).
    let estimate = h * (f.sum() - (fs[0] + fs[n - 1]) / 2.0);
    let exact = 1.0 / 3.0; // ∫_0^1 x^2 dx = 1/3
    let err = (estimate - exact).abs();

    println!("trapezoidal estimate: {estimate:.6}");
    println!("exact integral (1/3): {exact:.6}");
    println!("abs error:            {err:.6}");

    assert!(err < 0.01);
    println!("Trapezoidal integration: OK");
}
