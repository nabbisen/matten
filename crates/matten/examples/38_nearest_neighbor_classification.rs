//! # Example: Nearest-Neighbor Classification
//!
//! Run: cargo run --example 38_nearest_neighbor_classification
//!
//! ## Problem
//! Given a small set of labeled training points, classify a new query point by the
//! label of its single closest training point (1-nearest-neighbor).
//!
//! ## Math idea
//! Compute the squared Euclidean distance from the query to every training point and
//! take the label of the minimum (a 1-NN classifier — no training step, no fitted
//! parameters).
//!
//! ## Tensor representation
//! The training set is a `[samples, features]` `Tensor`; labels are a parallel slice.
//! A query is a length-`features` slice.
//!
//! ## What this demonstrates
//! - using a `Tensor` as a labeled `[samples, features]` data matrix;
//! - a nearest-point search via `Tensor::argmin` (RFC-038);
//! - composing `Tensor` row access with plain Rust arithmetic.
//!
//! ## Expected output
//! ```text
//! query [1.5, 1.5] -> class 0
//! query [8.5, 8.5] -> class 1
//! Nearest-neighbor classification: OK
//! ```
//!
//! This is an algorithm demonstration, not an ML framework.

use matten::Tensor;

/// Squared Euclidean distance between two equal-length points.
fn sq_dist(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| (x - y) * (x - y)).sum()
}

/// Label of the single nearest training point (1-NN), via `Tensor::argmin` over
/// the per-training-point distances.
fn classify(query: &[f64], train: &Tensor, labels: &[u8]) -> u8 {
    let dim = train.shape()[1];
    let data = train.as_slice();
    let dists: Vec<f64> = (0..train.shape()[0])
        .map(|i| sq_dist(query, &data[i * dim..(i + 1) * dim]))
        .collect();
    labels[Tensor::from_vec(dists).argmin()]
}

fn main() {
    // Labeled training set: 4 points (rows), 2 features (columns).
    let train = Tensor::new(
        vec![
            1.0, 1.0, //
            2.0, 2.0, //
            8.0, 8.0, //
            9.0, 9.0, //
        ],
        &[4, 2],
    );
    let labels = [0u8, 0, 1, 1];

    let queries = [[1.5, 1.5], [8.5, 8.5]];
    for q in &queries {
        let label = classify(q, &train, &labels);
        println!("query [{:.1}, {:.1}] -> class {label}", q[0], q[1]);
    }

    assert_eq!(classify(&[1.5, 1.5], &train, &labels), 0);
    assert_eq!(classify(&[8.5, 8.5], &train, &labels), 1);
    println!("Nearest-neighbor classification: OK");
}
