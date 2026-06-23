//! # Example: 1D Heat Equation
//!
//! Run: cargo run --example 36_heat_equation_1d
//!
//! ## Problem
//! A thin rod has its two ends held at fixed temperatures and starts cold inside.
//! How does the temperature profile evolve, and where does it settle?
//!
//! ## Math idea
//! The explicit (forward-Euler) finite-difference update of the 1D heat equation is
//! `u_i ← u_i + r·(u_{i-1} - 2·u_i + u_{i+1})`. Collecting one full sweep into a
//! tridiagonal matrix `A` (with identity rows at the fixed boundaries) makes each
//! time step a single `u_next = A · u`. The stability condition is `r ≤ 0.5`.
//!
//! ## Tensor representation
//! `A` is an N×N `Tensor`; the rod state `u` is length-N. Each time step is one
//! `Tensor::matmul` (`[n, n] × [n] -> [n]`).
//!
//! ## What this demonstrates
//! - encoding a stencil as a matrix and iterating with `Tensor::matmul`;
//! - fixed boundary conditions via identity rows;
//! - convergence to the steady-state (linear) profile between the boundaries.
//!
//! ## Expected output
//! ```text
//! step    u(rod)
//!    1   [0.00, 0.00, 0.00, 25.00, 100.00]
//!   10   [0.00, 16.26, 37.61, 66.22, 100.00]
//!   50   [0.00, 24.98, 49.98, 74.98, 100.00]
//!  300   [0.00, 25.00, 50.00, 75.00, 100.00]
//! Heat equation 1D: OK
//! ```

use matten::Tensor;

/// One explicit heat-equation step: `u_next = A · u`.
fn step(a: &Tensor, u: &Tensor) -> Tensor {
    a.matmul(u)
}

fn main() {
    // 5 nodes along the rod. Boundaries are fixed (left = 0, right = 100); the
    // interior starts cold. Diffusion number r = 0.25 satisfies r <= 0.5 (stable).
    let r = 0.25;
    let d = 1.0 - 2.0 * r;

    // Boundary rows are identity (hold the value); interior rows are [r, 1-2r, r].
    let a = Tensor::new(
        vec![
            1.0, 0.0, 0.0, 0.0, 0.0, //
            r, d, r, 0.0, 0.0, //
            0.0, r, d, r, 0.0, //
            0.0, 0.0, r, d, r, //
            0.0, 0.0, 0.0, 0.0, 1.0, //
        ],
        &[5, 5],
    );

    let mut u = Tensor::from_vec(vec![0.0, 0.0, 0.0, 0.0, 100.0]);

    println!("step    u(rod)");
    for s in 1..=300 {
        u = step(&a, &u);
        if matches!(s, 1 | 10 | 50 | 300) {
            let row: Vec<String> = u.as_slice().iter().map(|v| format!("{v:.2}")).collect();
            println!("{s:>4}   [{}]", row.join(", "));
        }
    }

    // Steady state is the linear profile between the two boundary temperatures.
    let expected = [0.0, 25.0, 50.0, 75.0, 100.0];
    for (got, want) in u.as_slice().iter().zip(&expected) {
        assert!((got - want).abs() < 0.1);
    }
    println!("Heat equation 1D: OK");
}
