//! # Companion example: seeded, shuffled train/test split (matten-mlprep, RFC-077)
//!
//! Run: cargo run -p matten-mlprep --example mlprep_train_test_split_seeded
//!
//! ## What this shows
//! Splitting a `[samples, features]` matrix into train and test parts by a
//! seeded, shuffled partition — unlike [`train_test_split`], which is ordered.
//!
//! ## Teaching points
//! - `n_train = floor(n_rows * train_ratio)`, identical to the ordered split;
//! - row order is determined by a Fisher-Yates shuffle seeded from `seed`;
//! - the same `(x, train_ratio, seed)` always reproduces the same split.

use matten::Tensor;
use matten_mlprep::train_test_split_seeded;

fn main() {
    let x = Tensor::new(vec![10.0, 20.0, 30.0, 40.0, 50.0], &[5, 1]);
    let (train, test) = train_test_split_seeded(&x, 0.6, 7).expect("valid split"); // 3 / 2
    println!("train {:?}: {:?}", train.shape(), train.as_slice());
    println!("test  {:?}: {:?}", test.shape(), test.as_slice());

    assert_eq!(train.shape(), &[3, 1]);
    assert_eq!(test.shape(), &[2, 1]);

    // Re-running with the same seed reproduces the exact same split.
    let (train2, test2) = train_test_split_seeded(&x, 0.6, 7).expect("valid split");
    assert_eq!(train.as_slice(), train2.as_slice());
    assert_eq!(test.as_slice(), test2.as_slice());
    println!("same seed -> reproduced split: OK");

    // A different seed shuffles differently.
    let (train3, _) = train_test_split_seeded(&x, 0.6, 8).expect("valid split");
    println!(
        "different seed -> train {:?}: {:?}",
        train3.shape(),
        train3.as_slice()
    );

    println!("train_test_split_seeded: OK");
}
