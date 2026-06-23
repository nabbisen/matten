//! # Example: K-Means (small)
//!
//! Run: cargo run --example 37_kmeans_small
//!
//! ## Problem
//! Group a handful of 2-D points into `k` clusters by repeatedly assigning each
//! point to the nearest cluster center and moving each center to the mean of its
//! points (Lloyd's algorithm).
//!
//! ## Math idea
//! Start from fixed initial centroids (no random seeding, so the run is
//! deterministic). Each iteration: assign every point to its nearest centroid by
//! squared Euclidean distance, then recompute each centroid as the mean of its
//! assigned points. Repeat until assignments stop changing.
//!
//! ## Tensor representation
//! The dataset is a `[points, features]` `Tensor`; each row is one point. Centroids
//! are kept as small vectors that are recomputed each pass.
//!
//! ## What this demonstrates
//! - using a `Tensor` as a `[samples, features]` data matrix;
//! - a nearest-centroid assignment (a local argmin — `matten` has no `argmin` yet);
//! - composing `Tensor` row access with plain Rust arithmetic.
//!
//! ## Expected output
//! ```text
//! assignments: [0, 0, 0, 1, 1, 1]
//! centroid 0: [1.5000, 1.5000]
//! centroid 1: [8.0000, 7.8333]
//! K-means (small): OK
//! ```
//!
//! This is an algorithm demonstration, not an ML framework: `k`, the initial
//! centroids, and the iteration count are all fixed and explicit.

use matten::Tensor;

/// Squared Euclidean distance between two equal-length points.
fn sq_dist(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| (x - y) * (x - y)).sum()
}

/// Index of the nearest centroid to `point` (a local argmin).
fn nearest(point: &[f64], centroids: &[Vec<f64>]) -> usize {
    let mut best = 0;
    let mut best_d = f64::INFINITY;
    for (i, c) in centroids.iter().enumerate() {
        let d = sq_dist(point, c);
        if d < best_d {
            best_d = d;
            best = i;
        }
    }
    best
}

fn main() {
    // 6 points (rows), 2 features (columns): two obvious clusters.
    let points = Tensor::new(
        vec![
            1.0, 1.0, //
            1.5, 2.0, //
            2.0, 1.5, //
            8.0, 8.0, //
            8.5, 7.5, //
            7.5, 8.0, //
        ],
        &[6, 2],
    );
    let n = points.shape()[0];
    let dim = points.shape()[1];
    let data = points.as_slice();
    let row = |i: usize| &data[i * dim..(i + 1) * dim];

    // Fixed (deterministic) initial centroids — no random seeding.
    let mut centroids = vec![vec![0.0, 0.0], vec![10.0, 10.0]];
    let mut assignments = vec![0usize; n];

    for _ in 0..5 {
        // Assign each point to its nearest centroid.
        for (i, a) in assignments.iter_mut().enumerate() {
            *a = nearest(row(i), &centroids);
        }
        // Recompute each centroid as the mean of its assigned points.
        for (c, centroid) in centroids.iter_mut().enumerate() {
            let mut sum = vec![0.0; dim];
            let mut count = 0.0;
            for (i, &a) in assignments.iter().enumerate() {
                if a == c {
                    for (d, s) in sum.iter_mut().enumerate() {
                        *s += row(i)[d];
                    }
                    count += 1.0;
                }
            }
            if count > 0.0 {
                for (d, slot) in centroid.iter_mut().enumerate() {
                    *slot = sum[d] / count;
                }
            }
        }
    }

    println!("assignments: {assignments:?}");
    for (c, centroid) in centroids.iter().enumerate() {
        println!("centroid {c}: [{:.4}, {:.4}]", centroid[0], centroid[1]);
    }

    assert_eq!(assignments, vec![0, 0, 0, 1, 1, 1]);
    assert!((centroids[0][0] - 1.5).abs() < 1e-9 && (centroids[0][1] - 1.5).abs() < 1e-9);
    assert!((centroids[1][0] - 8.0).abs() < 1e-9 && (centroids[1][1] - 23.5 / 3.0).abs() < 1e-9);
    println!("K-means (small): OK");
}
