//! # Companion example: standardize feature columns (matten-mlprep)
//!
//! Run: cargo run -p matten-mlprep --example mlprep_standardize_columns
//!
//! ## What this shows
//! Standardizing each feature column of a `[samples, features]` matrix to zero
//! mean and unit standard deviation (the z-score).
//!
//! ## Teaching points
//! - rows are samples, columns are features;
//! - the transform is deterministic — no randomness and no fitted model state;
//! - `matten-mlprep` is small preprocessing, not an ML training framework.

use matten::Tensor;
use matten_mlprep::standardize_columns;

fn main() {
    // 3 samples, 2 features.
    let x = Tensor::new(vec![1.0, 10.0, 2.0, 20.0, 3.0, 30.0], &[3, 2]);
    let z = standardize_columns(&x).expect("two non-constant columns");
    println!("input  shape {:?}: {:?}", x.shape(), x.as_slice());
    println!("z-score      {:?}: {:?}", z.shape(), z.as_slice());

    // Shape is preserved and each standardized column has (near) zero mean.
    assert_eq!(z.shape(), x.shape());
    let zs = z.as_slice();
    let col0_mean = (zs[0] + zs[2] + zs[4]) / 3.0;
    let col1_mean = (zs[1] + zs[3] + zs[5]) / 3.0;
    assert!(col0_mean.abs() < 1e-9 && col1_mean.abs() < 1e-9);
    println!("standardize_columns: OK");
}
