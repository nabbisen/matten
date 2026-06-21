//! Scale each feature column into [0, 1].
//! Run: cargo run -p matten-mlprep --example mlprep_minmax_scale
use matten::Tensor;
use matten_mlprep::minmax_scale_columns;

fn main() {
    let x = Tensor::new(vec![0.0, 100.0, 5.0, 150.0, 10.0, 200.0], &[3, 2]);
    let s = minmax_scale_columns(&x).expect("two non-constant columns");
    println!("input  {:?}", x.as_slice());
    println!("scaled {:?}", s.as_slice());
}
