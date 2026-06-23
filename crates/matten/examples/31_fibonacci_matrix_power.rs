//! # Example: Fibonacci by Matrix Power
//!
//! Run: cargo run --example 31_fibonacci_matrix_power
//!
//! ## Problem
//! Compute Fibonacci numbers using matrix multiplication instead of the usual
//! additive recurrence.
//!
//! ## Math idea
//! With `Q = [[1, 1], [1, 0]]`, raising `Q` to the n-th power gives
//! ```text
//! Q^n = [[F(n+1), F(n)],
//!        [F(n),   F(n-1)]]
//! ```
//! so `F(n)` is the top-right entry of `Q^n` (with `F(0) = 0`, `F(1) = 1`).
//!
//! ## Tensor representation
//! `Q` and the running product are 2×2 `Tensor`s. Each power step is a single
//! `Tensor::matmul` call; the result entry is read with `Tensor::get(&[0, 1])`.
//!
//! ## What this demonstrates
//! - 2×2 matrix construction;
//! - repeated matrix multiplication via `Tensor::matmul` (note: `*` is
//!   element-wise, never a matrix product);
//! - reading a single matrix element with `Tensor::get`.
//!
//! ## Expected output
//! ```text
//! F(1..=10) via matrix power = 1 1 2 3 5 8 13 21 34 55
//! Fibonacci by matrix power: OK
//! ```
//!
//! Note: this is a demonstration of the matrix identity, not a big-integer
//! Fibonacci routine — `f64` stays exact only for modest `n`.

use matten::Tensor;

/// `n`-th Fibonacci number via the matrix identity `Q^n`, with `F(0) = 0`.
fn fib(n: u32) -> u64 {
    let q = Tensor::new(vec![1.0, 1.0, 1.0, 0.0], &[2, 2]);
    // Start from the 2×2 identity so that `fib(0)` returns 0.
    let mut acc = Tensor::new(vec![1.0, 0.0, 0.0, 1.0], &[2, 2]);
    for _ in 0..n {
        acc = acc.matmul(&q);
    }
    // F(n) is the top-right entry of Q^n.
    acc.get(&[0, 1]).expect("index in bounds") as u64
}

fn main() {
    let expected = [1u64, 1, 2, 3, 5, 8, 13, 21, 34, 55];

    print!("F(1..=10) via matrix power =");
    for n in 1..=10u32 {
        let f = fib(n);
        print!(" {f}");
        assert_eq!(f, expected[(n - 1) as usize]);
    }
    println!();

    assert_eq!(fib(0), 0);
    println!("Fibonacci by matrix power: OK");
}
