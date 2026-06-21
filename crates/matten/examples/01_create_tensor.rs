//! Creating tensors: `new`, `zeros`, `ones`, `full`, `scalar`, `from_vec`,
//! `arange`, and nested rows.
//!
//! Run: cargo run --example 01_create_tensor

use matten::Tensor;

fn main() {
    // From data + shape
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    println!("new:         {t:?}");

    // Fill constructors
    println!("zeros [3]:   {:?}", Tensor::zeros(&[3]));
    println!("ones  [2,2]: {:?}", Tensor::ones(&[2, 2]));
    println!("full  [3]:   {:?}", Tensor::full(&[3], -1.0));
    println!("scalar:      {:?}", Tensor::scalar(42.0));

    // From flat vec
    println!(
        "from_vec:    {:?}",
        Tensor::from_vec(vec![10.0, 20.0, 30.0])
    );

    // Range
    println!("arange:      {:?}", Tensor::arange(0.0, 5.0, 1.0));

    // From nested rows (convenient for trusted literals)
    let rows: Tensor = vec![vec![1.0, 2.0], vec![3.0, 4.0]].into();
    println!("from rows:   {rows:?}");

    // Boundary-safe construction
    let result = Tensor::try_new(vec![1.0, 2.0], &[3]);
    println!("try_new err: {}", result.unwrap_err());
}
