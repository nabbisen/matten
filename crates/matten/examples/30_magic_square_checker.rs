//! # Example: Magic Square Checker
//!
//! Run: cargo run --example 30_magic_square_checker
//!
//! ## Problem
//! A *magic square* is an n×n grid of numbers whose every row, every column, and
//! both main diagonals add up to the same total (the *magic constant*). Given a
//! square matrix, decide whether it is magic.
//!
//! ## Math idea
//! Compute the target sum from the first row, then verify that all n rows, all n
//! columns, and the two diagonals share that sum.
//!
//! ## Tensor representation
//! The square is a 2-D `Tensor` of shape `[n, n]`. Cells are read with
//! `Tensor::get(&[row, col])`, which returns `Option<f64>` (`None` if out of bounds).
//!
//! ## What this demonstrates
//! - 2-D `Tensor` construction with `Tensor::new(data, &[n, n])`;
//! - shape inspection via `Tensor::shape`;
//! - element access via `Tensor::get`;
//! - row / column / diagonal traversal with ordinary Rust iterators.
//!
//! ## Expected output
//! ```text
//! Lo Shu is magic: true   (magic sum = 15)
//! Sequence is magic: false
//! Magic square checker: OK
//! ```

use matten::Tensor;

/// Returns `Some(magic_sum)` if `m` is an n×n magic square, otherwise `None`.
///
/// The cell values here are small integers, so the row/column/diagonal sums are
/// exact in `f64` and can be compared directly.
fn magic_sum(m: &Tensor) -> Option<f64> {
    let shape = m.shape();
    if shape.len() != 2 || shape[0] != shape[1] || shape[0] == 0 {
        return None;
    }
    let n = shape[0];
    let at = |i: usize, j: usize| m.get(&[i, j]).expect("index in bounds");

    // Target is the sum of the first row.
    let target: f64 = (0..n).map(|j| at(0, j)).sum();

    // Every row must match.
    for i in 0..n {
        let row: f64 = (0..n).map(|j| at(i, j)).sum();
        if row != target {
            return None;
        }
    }
    // Every column must match.
    for j in 0..n {
        let col: f64 = (0..n).map(|i| at(i, j)).sum();
        if col != target {
            return None;
        }
    }
    // Both diagonals must match.
    let main_diag: f64 = (0..n).map(|i| at(i, i)).sum();
    let anti_diag: f64 = (0..n).map(|i| at(i, n - 1 - i)).sum();
    if main_diag != target || anti_diag != target {
        return None;
    }

    Some(target)
}

fn main() {
    // Lo Shu — the classic 3×3 magic square (magic constant 15).
    let lo_shu = Tensor::new(vec![8.0, 1.0, 6.0, 3.0, 5.0, 7.0, 4.0, 9.0, 2.0], &[3, 3]);
    // 1..9 in order: rows match but columns and diagonals do not.
    let sequence = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0], &[3, 3]);

    match magic_sum(&lo_shu) {
        Some(s) => println!("Lo Shu is magic: true   (magic sum = {s})"),
        None => println!("Lo Shu is magic: false"),
    }
    match magic_sum(&sequence) {
        Some(s) => println!("Sequence is magic: true (magic sum = {s})"),
        None => println!("Sequence is magic: false"),
    }

    assert_eq!(magic_sum(&lo_shu), Some(15.0));
    assert_eq!(magic_sum(&sequence), None);
    println!("Magic square checker: OK");
}
