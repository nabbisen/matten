//! Pairwise Euclidean distances between rows using broadcasting and matmul.
//!
//! Run: cargo run --example pairwise_distance
//!
//! Uses the identity: ||a-b||² = ||a||² + ||b||² - 2·aᵀb

use matten::Tensor;

fn pairwise_euclidean(points: &Tensor) -> Tensor {
    let n = points.shape()[0];

    // Row-wise squared norms: shape [n]
    let sq = points * points; // wait — need &ref; fix below
    let _ = sq;
    let sq = &(points.clone()) * points;
    let row_sq_norms = sq.sum_axis(1); // [n]

    // Gram matrix: G[i,j] = points[i] · points[j], shape [n,n]
    let gram = points.matmul(&points.transpose()); // [n,n]

    // dist²[i,j] = sq_norm[i] + sq_norm[j] - 2·G[i,j]
    // Broadcast [n] as column [n,1] + row [1,n]
    let col = row_sq_norms.reshape(&[n, 1]);
    let row = row_sq_norms.reshape(&[1, n]);
    let dist_sq = &(&col + &row) - &(&gram * 2.0);

    // Clamp small negatives from floating-point and take sqrt element-wise
    let clamped: Vec<f64> = dist_sq
        .as_slice()
        .iter()
        .map(|&v| if v < 0.0 { 0.0 } else { v })
        .collect();
    let clamped_t = Tensor::new(clamped, &[n, n]);

    // sqrt element-wise using scalar-on-left trick: each element via map
    let dists: Vec<f64> = clamped_t.as_slice().iter().map(|v| v.sqrt()).collect();
    Tensor::new(dists, &[n, n])
}

fn main() {
    let points = Tensor::new(vec![0.0, 0.0, 3.0, 4.0, 6.0, 0.0], &[3, 2]);

    let dists = pairwise_euclidean(&points);
    println!("pairwise distances:");
    for i in 0..3 {
        let row = dists.slice().index(i).all().build().unwrap();
        println!("  row {i}: {:?}", row.as_slice());
    }

    // distance from (0,0) to (3,4) should be 5
    assert!((dists.get(&[0, 1]).unwrap() - 5.0).abs() < 1e-9);
    // distance from (3,4) to (6,0) should be 5
    assert!((dists.get(&[1, 2]).unwrap() - 5.0).abs() < 1e-9);
    println!("Pairwise distances: OK");
}
