//! # Example: Markov Chain Weather Model
//!
//! Run: cargo run --example 33_markov_chain_weather
//!
//! ## Problem
//! A simple weather model has two states, Sunny and Rainy. Given today's weather
//! and a table of day-to-day transition probabilities, what is the chance of each
//! state several days from now — and where does it settle?
//!
//! ## Math idea
//! With a row-vector distribution `v` and a row-stochastic transition matrix `P`
//! (each row sums to 1), one day's update is `v_next = v · P`. Iterating drives `v`
//! toward the stationary distribution `π` that satisfies `π = π · P`.
//!
//! ## Tensor representation
//! `P` is a 2×2 `Tensor`; the distribution is a length-2 `Tensor`. Each step is a
//! single `Tensor::matmul` in the `[n] × [n, p] -> [p]` (vector × matrix) form.
//!
//! ## What this demonstrates
//! - vector × matrix multiplication via `Tensor::matmul`;
//! - repeated application to model a process over time;
//! - convergence toward a stationary distribution.
//!
//! ## Expected output
//! ```text
//! day  P(Sunny)  P(Rainy)
//!   1   0.9000    0.1000
//!   2   0.8600    0.1400
//!   5   0.8350    0.1650
//!  10   0.8334    0.1666
//! stationary ≈ 0.8333 0.1667
//! Markov chain weather: OK
//! ```

use matten::Tensor;

/// One Markov step: `next = current · P` (row-vector convention).
fn step(dist: &Tensor, p: &Tensor) -> Tensor {
    dist.matmul(p)
}

fn main() {
    // Row = today's state, column = tomorrow's state; each row sums to 1.
    //            Sunny  Rainy
    //   Sunny  [ 0.9    0.1 ]
    //   Rainy  [ 0.5    0.5 ]
    let p = Tensor::new(vec![0.9, 0.1, 0.5, 0.5], &[2, 2]);

    // Start certain that today is Sunny.
    let mut dist = Tensor::from_vec(vec![1.0, 0.0]);

    println!("day  P(Sunny)  P(Rainy)");
    for day in 1..=10 {
        dist = step(&dist, &p);
        if matches!(day, 1 | 2 | 5 | 10) {
            let d = dist.as_slice();
            println!("{day:>3}   {:.4}    {:.4}", d[0], d[1]);
        }
    }

    // Stationary distribution solves π = π·P, giving π = [5/6, 1/6].
    let stationary = [5.0 / 6.0, 1.0 / 6.0];
    println!("stationary ≈ {:.4} {:.4}", stationary[0], stationary[1]);

    let d = dist.as_slice();
    assert!((d[0] - stationary[0]).abs() < 1e-3);
    assert!((d[1] - stationary[1]).abs() < 1e-3);
    assert!((d[0] + d[1] - 1.0).abs() < 1e-12);
    println!("Markov chain weather: OK");
}
