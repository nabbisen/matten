//! NumPy-style right-aligned broadcasting.
//!
//! Run: cargo run --example 06_broadcasting
//!
//! Rules: shapes are compatible when, aligning from the right, each dimension
//! pair is equal, one is 1, or one side is missing (treated as 1).

use matten::Tensor;

fn main() {
    // Scalar [] + matrix [2,2] -> [2,2]
    let scalar = Tensor::scalar(10.0);
    let mat = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    println!("scalar + mat = {:?}", &scalar + &mat);

    // Row vector [3] + matrix [2,3] -> [2,3]  (bias addition)
    let matrix = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let bias = Tensor::new(vec![10.0, 20.0, 30.0], &[3]);
    println!("matrix + bias = {:?}", &matrix + &bias);

    // Column [3,1] + row [1,4] -> [3,4]  (outer product pattern)
    let col = Tensor::new(vec![1.0, 2.0, 3.0], &[3, 1]);
    let row = Tensor::new(vec![10.0, 20.0, 30.0, 40.0], &[1, 4]);
    let result = &col + &row;
    println!("col + row shape = {:?}", result.shape()); // [3, 4]
    println!("col + row data  = {:?}", result.as_slice());
}
