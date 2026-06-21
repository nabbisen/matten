//! Row-wise weighted scoring: multiply each row by a weight vector.
//!
//! Run: cargo run --example rowwise_scoring

use matten::Tensor;

fn main() {
    // 3 items × 4 features
    let features = Tensor::new(
        vec![0.8, 0.6, 0.9, 0.7, 0.4, 0.9, 0.5, 0.8, 0.7, 0.7, 0.8, 0.6],
        &[3, 4],
    );

    // Importance weights per feature: shape [4], broadcast across rows
    let weights = Tensor::new(vec![0.3, 0.2, 0.4, 0.1], &[4]);

    // Weighted feature matrix: [3,4]
    let weighted = &features * &weights;

    // Score per item: sum across features (axis 1) -> shape [3]
    let scores = weighted.sum_axis(1);
    println!("scores = {:?}", scores.as_slice());

    // Best item (highest score)
    let best_score = scores.max();
    println!("best score = {best_score:.3}");

    assert_eq!(scores.shape(), &[3]);
    println!("Row-wise scoring: OK");
}
