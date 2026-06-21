//! Standardize each feature column to zero mean and unit std.
//! Run: cargo run -p matten-mlprep --example mlprep_standardize_columns
use matten::Tensor;
use matten_mlprep::standardize_columns;

fn main() {
    let x = Tensor::new(vec![1.0, 10.0, 2.0, 20.0, 3.0, 30.0], &[3, 2]);
    let z = standardize_columns(&x).expect("two non-constant columns");
    println!("input  shape {:?}: {:?}", x.shape(), x.as_slice());
    println!("z-score      {:?}: {:?}", z.shape(), z.as_slice());
}
