//! Whole-tensor and axis reductions: `sum`, `mean`, `sum_axis`, `mean_axis`.
//!
//! Run: cargo run --example 23_sum_mean

use matten::Tensor;

fn main() {
    let v = Tensor::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    println!("v.sum()  = {}", v.sum()); // 15.0
    println!("v.mean() = {}", v.mean()); // 3.0

    // Axis reductions on a matrix
    // [[1,2,3],[4,5,6]]
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    println!("column sums   (axis 0) = {:?}", m.sum_axis(0).as_slice()); // [5,7,9]
    println!("row sums      (axis 1) = {:?}", m.sum_axis(1).as_slice()); // [6,15]
    println!("column means  (axis 0) = {:?}", m.mean_axis(0).as_slice()); // [2.5,3.5,4.5]
    println!("row means     (axis 1) = {:?}", m.mean_axis(1).as_slice()); // [2.0,5.0]

    assert_eq!(m.sum_axis(0).as_slice(), &[5.0, 7.0, 9.0]);
    assert_eq!(m.mean_axis(1).as_slice(), &[2.0, 5.0]);
    println!("Assertions passed: OK");
}
