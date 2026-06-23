//! # Companion example: prepend a bias/intercept column (matten-mlprep)
//!
//! Run: cargo run -p matten-mlprep --example mlprep_add_bias_column
//!
//! ## What this shows
//! Prepending a constant `1.0` column to a `[samples, features]` matrix — the
//! usual way to give a linear model an intercept term.
//!
//! ## Teaching points
//! - rows are samples; a new leading column of ones is added;
//! - the feature count grows by exactly one; the sample count is unchanged;
//! - deterministic, with no model state.

use matten::Tensor;
use matten_mlprep::add_bias_column;

fn main() {
    // 2 samples, 2 features.
    let x = Tensor::new(vec![2.0, 3.0, 4.0, 5.0], &[2, 2]);
    let b = add_bias_column(&x).expect("rank-2 input");
    println!("input  {:?} {:?}", x.shape(), x.as_slice());
    println!("biased {:?} {:?}", b.shape(), b.as_slice()); // [2,3], col 0 = 1.0

    // New shape [2, 3]; the first column of every row is 1.0.
    assert_eq!(b.shape(), &[2, 3]);
    let bs = b.as_slice();
    assert_eq!(bs[0], 1.0);
    assert_eq!(bs[3], 1.0);
    println!("add_bias_column: OK");
}
