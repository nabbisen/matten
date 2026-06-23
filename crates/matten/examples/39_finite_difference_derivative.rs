//! # Example: Finite-Difference Derivative
//!
//! Run: cargo run --example 39_finite_difference_derivative
//!
//! ## Problem
//! Approximate the derivative of a function sampled on a grid, without any symbolic
//! math — just function values and arithmetic.
//!
//! ## Math idea
//! The central difference `(f(x+h) - f(x-h)) / (2h)` approximates `f'(x)` with an
//! error that shrinks like `h²` (second-order accurate). Here `f(x) = x³`, whose
//! exact derivative is `3x²`; for a cubic the central-difference error is exactly
//! `h²`, which makes the approximation easy to see.
//!
//! ## Tensor representation
//! The sample grid is a 1-D `Tensor` built with `linspace`; the function values are
//! another `Tensor` computed with elementwise multiplication.
//!
//! ## What this demonstrates
//! - `Tensor::linspace` for an evenly spaced grid;
//! - elementwise `Tensor` arithmetic (`&x * &x`) to evaluate `f`;
//! - a central-difference stencil over the sampled values.
//!
//! ## Expected output
//! ```text
//! h = 0.25
//!    x    f'approx   f'exact
//! 0.25     0.2500    0.1875
//! 0.50     0.8125    0.7500
//! 0.75     1.7500    1.6875
//! 1.00     3.0625    3.0000
//! 1.25     4.7500    4.6875
//! 1.50     6.8125    6.7500
//! 1.75     9.2500    9.1875
//! max abs error = 0.0625 (= h^2 = 0.0625)
//! Finite-difference derivative: OK
//! ```
//!
//! This is a numerical approximation, not symbolic differentiation.

use matten::Tensor;

fn main() {
    // Evenly spaced sample grid on [0, 2].
    let x = Tensor::linspace(0.0, 2.0, 9);
    let xs = x.as_slice();
    let h = xs[1] - xs[0];

    // f(x) = x^3, evaluated with elementwise Tensor multiplication.
    let x2 = &x * &x;
    let f = &x2 * &x;
    let fs = f.as_slice();

    println!("h = {h}");
    println!("   x    f'approx   f'exact");
    let mut max_err = 0.0f64;
    for i in 1..xs.len() - 1 {
        let approx = (fs[i + 1] - fs[i - 1]) / (2.0 * h);
        let exact = 3.0 * xs[i] * xs[i]; // d/dx x^3 = 3x^2
        let err = (approx - exact).abs();
        max_err = max_err.max(err);
        println!("{:4.2}     {:6.4}    {:6.4}", xs[i], approx, exact);
        // Central difference of x^3 has error exactly h^2.
        assert!((approx - (exact + h * h)).abs() < 1e-9);
    }
    println!("max abs error = {max_err:.4} (= h^2 = {:.4})", h * h);
    assert!((max_err - h * h).abs() < 1e-9);
    println!("Finite-difference derivative: OK");
}
