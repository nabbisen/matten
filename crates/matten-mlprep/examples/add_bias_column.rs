//! Prepend a constant bias (intercept) column.
//! Run: cargo run -p matten-mlprep --example mlprep_add_bias_column
use matten::Tensor;
use matten_mlprep::add_bias_column;

fn main() {
    let x = Tensor::new(vec![2.0, 3.0, 4.0, 5.0], &[2, 2]);
    let b = add_bias_column(&x).expect("rank-2 input");
    println!("input  {:?} {:?}", x.shape(), x.as_slice());
    println!("biased {:?} {:?}", b.shape(), b.as_slice()); // [2,3], col 0 = 1.0
}
