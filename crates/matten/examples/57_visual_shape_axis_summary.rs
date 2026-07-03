//! Visual shape and axis summary for a few core tensor operations.
//!
//! Run: cargo run --example 57_visual_shape_axis_summary
//!
//! This example prints short, deterministic summaries: inputs, operation,
//! output shape, and small output values. It is intentionally not a full
//! tutorial; the mdBook reference pages carry the larger diagrams.

use matten::Tensor;

fn print_tensor_line(label: &str, t: &Tensor) {
    println!(
        "{label:<16} shape={:?} values={:?}",
        t.shape(),
        t.as_slice()
    );
}

fn main() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let b = Tensor::new(vec![10.0, 20.0, 30.0], &[3]);

    println!("== Broadcasting ==");
    print_tensor_line("input A", &a);
    print_tensor_line("input b", &b);
    let broadcast = &a + &b;
    print_tensor_line("A + b", &broadcast);
    println!("meaning         b repeats across rows");
    assert_eq!(broadcast.shape(), &[2, 3]);
    assert_eq!(broadcast.as_slice(), &[11.0, 22.0, 33.0, 14.0, 25.0, 36.0]);

    println!();
    println!("== Reshape ==");
    let reshaped = a.reshape(&[3, 2]);
    print_tensor_line("[2, 3] input", &a);
    print_tensor_line("[3, 2] view", &reshaped);
    println!("meaning         row-major values stay in the same order");
    assert_eq!(reshaped.shape(), &[3, 2]);
    assert_eq!(reshaped.as_slice(), a.as_slice());

    println!();
    println!("== Axis reductions ==");
    let col_means = a.mean_axis(0);
    let row_means = a.mean_axis(1);
    println!(
        "mean_axis(0)    collapse rows, keep columns -> shape {:?}, values {:?}",
        col_means.shape(),
        col_means.as_slice()
    );
    println!(
        "mean_axis(1)    collapse columns, keep rows -> shape {:?}, values {:?}",
        row_means.shape(),
        row_means.as_slice()
    );
    assert_eq!(col_means.shape(), &[3]);
    assert_eq!(col_means.as_slice(), &[2.5, 3.5, 4.5]);
    assert_eq!(row_means.shape(), &[2]);
    assert_eq!(row_means.as_slice(), &[2.0, 5.0]);

    println!();
    println!("== Matrix multiplication ==");
    let left = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let right = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[3, 2]);
    let product = left.matmul(&right);
    print_tensor_line("left", &left);
    print_tensor_line("right", &right);
    print_tensor_line("left.matmul", &product);
    println!("meaning         [2, 3] x [3, 2] -> [2, 2]");
    assert_eq!(product.shape(), &[2, 2]);
    assert_eq!(product.as_slice(), &[22.0, 28.0, 49.0, 64.0]);

    println!();
    println!("57_visual_shape_axis_summary: OK");
}
