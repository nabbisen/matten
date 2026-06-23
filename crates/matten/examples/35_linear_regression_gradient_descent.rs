//! # Example: Linear Regression by Gradient Descent
//!
//! Run: cargo run --example 35_linear_regression_gradient_descent
//!
//! ## Problem
//! Fit a straight line `y = w*x + b` to a few points by minimizing mean-squared
//! error, using batch gradient descent rather than a closed-form solution.
//!
//! ## Math idea
//! Stack the data into a design matrix `X` (with a leading bias column) and a target
//! vector `y`, so the model is `ŷ = X · θ` with `θ = [b, w]`. The MSE gradient is
//! `(2/n) · Xᵀ · (ŷ - y)`, and each step takes `θ ← θ - lr · gradient`.
//!
//! ## Tensor representation
//! `X` is a `[samples, 2]` `Tensor`; `θ` is length-2. Predictions are one
//! `Tensor::matmul` (`[n, 2] × [2] -> [n]`) and the gradient is another
//! (`Xᵀ` is `[2, n]`, so `[2, n] × [n] -> [2]`). The per-step update is plain Rust.
//!
//! ## What this demonstrates
//! - matrix × vector multiplication via `Tensor::matmul`;
//! - `Tensor::transpose` to form `Xᵀ` once and reuse it;
//! - an iterative optimizer composing `Tensor` math with ordinary arithmetic.
//!
//! ## Expected output
//! ```text
//! fitted: y = 2.0000*x + 1.0000
//! target: y = 2*x + 1
//! Linear regression (gradient descent): OK
//! ```

use matten::Tensor;

/// One batch gradient-descent step on the MSE of a linear model `ŷ = X · θ`.
fn gd_step(x: &Tensor, xt: &Tensor, theta: &Tensor, y: &[f64], lr: f64) -> Tensor {
    let n = y.len() as f64;
    let pred = x.matmul(theta); // [n]
    let residual: Vec<f64> = pred.as_slice().iter().zip(y).map(|(p, t)| p - t).collect();
    let grad = xt.matmul(&Tensor::from_vec(residual)); // [features]
    let updated: Vec<f64> = theta
        .as_slice()
        .iter()
        .zip(grad.as_slice())
        .map(|(w, g)| w - lr * (2.0 / n) * g)
        .collect();
    Tensor::from_vec(updated)
}

fn main() {
    // Data generated from the true line y = 2x + 1.
    // Design matrix X carries a leading bias column, so theta = [b, w].
    let x = Tensor::new(
        vec![
            1.0, 0.0, //
            1.0, 1.0, //
            1.0, 2.0, //
            1.0, 3.0, //
            1.0, 4.0, //
        ],
        &[5, 2],
    );
    let y = [1.0, 3.0, 5.0, 7.0, 9.0];
    let xt = x.transpose(); // [2, 5], formed once and reused each step

    let mut theta = Tensor::from_vec(vec![0.0, 0.0]); // [b, w]
    let lr = 0.05;
    for _ in 0..2000 {
        theta = gd_step(&x, &xt, &theta, &y, lr);
    }

    let p = theta.as_slice();
    println!("fitted: y = {:.4}*x + {:.4}", p[1], p[0]);
    println!("target: y = 2*x + 1");

    assert!((p[0] - 1.0).abs() < 0.05, "intercept ~ 1");
    assert!((p[1] - 2.0).abs() < 0.05, "slope ~ 2");
    println!("Linear regression (gradient descent): OK");
}
