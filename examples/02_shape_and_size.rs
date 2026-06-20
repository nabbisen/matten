//! Shape inspection: `shape`, `ndim`, `len`, `is_scalar`, `is_vector`,
//! `is_matrix`, and element access with `get`.
//!
//! Run: cargo run --example 02_shape_and_size

use matten::Tensor;

fn main() {
    let scalar = Tensor::scalar(1.0);
    let vec1d = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    let mat = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let rank3 = Tensor::zeros(&[2, 3, 4]);

    for (name, t) in [
        ("scalar", &scalar),
        ("vector", &vec1d),
        ("matrix", &mat),
        ("rank-3", &rank3),
    ] {
        println!(
            "{name:6}: shape={:?}  ndim={}  len={}  is_scalar={}  is_vector={}  is_matrix={}",
            t.shape(),
            t.ndim(),
            t.len(),
            t.is_scalar(),
            t.is_vector(),
            t.is_matrix()
        );
    }

    // Safe element access: Option<f64>, never panics
    println!("\nmat.get([0,1]) = {:?}", mat.get(&[0, 1])); // Some(2.0)
    println!("mat.get([5,0]) = {:?}", mat.get(&[5, 0])); // None
}
