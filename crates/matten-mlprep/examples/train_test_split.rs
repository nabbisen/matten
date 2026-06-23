//! # Companion example: deterministic train/test split (matten-mlprep)
//!
//! Run: cargo run -p matten-mlprep --example mlprep_train_test_split
//!
//! ## What this shows
//! Splitting a `[samples, features]` matrix into train and test parts by an
//! ordered fraction.
//!
//! ## Teaching points
//! - rows are samples; the split keeps the first fraction as train, the rest as test;
//! - it is deterministic and ordered — no shuffling, no hidden randomness, no seed;
//! - shuffling, if wanted, is the caller's responsibility before splitting.

use matten::Tensor;
use matten_mlprep::train_test_split;

fn main() {
    // 5 samples, 2 features: rows 0..=4.
    let x = Tensor::new((0..10).map(|v| v as f64).collect(), &[5, 2]);
    let (train, test) = train_test_split(&x, 0.6).expect("valid split"); // 3 / 2
    println!("train {:?}: {:?}", train.shape(), train.as_slice());
    println!("test  {:?}: {:?}", test.shape(), test.as_slice());

    // Deterministic ordered split: first 3 rows -> train, last 2 rows -> test.
    assert_eq!(train.shape(), &[3, 2]);
    assert_eq!(test.shape(), &[2, 2]);
    assert_eq!(train.as_slice(), &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
    assert_eq!(test.as_slice(), &[6.0, 7.0, 8.0, 9.0]);
    println!("train_test_split: OK");
}
