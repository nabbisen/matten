//! Deterministic ordered train/test split.
//! Run: cargo run -p matten-mlprep --example mlprep_train_test_split
use matten::Tensor;
use matten_mlprep::train_test_split;

fn main() {
    // 5 samples, 2 features.
    let x = Tensor::new((0..10).map(|v| v as f64).collect(), &[5, 2]);
    let (train, test) = train_test_split(&x, 0.6).expect("valid split"); // 3 / 2
    println!("train {:?}: {:?}", train.shape(), train.as_slice());
    println!("test  {:?}: {:?}", test.shape(), test.as_slice());
}
