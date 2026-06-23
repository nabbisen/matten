//! # Example: Tiny PageRank
//!
//! Run: cargo run --example 34_tiny_pagerank
//!
//! ## Problem
//! Rank the nodes of a tiny directed graph by importance, where a node is
//! important if important nodes link to it — the core idea behind PageRank.
//!
//! ## Math idea
//! Build a column-stochastic link matrix `M` (`M[i, j] = 1/outdeg(j)` when `j -> i`).
//! With damping `d` and `N` nodes, power-iterate
//! ```text
//! r_next[i] = (1 - d)/N + d * (M · r)[i]
//! ```
//! until `r` settles. The `(1 - d)/N` term is the uniform "teleport" that keeps the
//! walk from getting stuck.
//!
//! ## Tensor representation
//! `M` is an N×N `Tensor`; the rank vector `r` is a length-N `Tensor`. The matrix
//! product `M · r` is one `Tensor::matmul` (`[m, n] × [n] -> [m]`); the damping and
//! teleport are ordinary Rust over `as_slice`.
//!
//! ## What this demonstrates
//! - matrix × vector multiplication via `Tensor::matmul`;
//! - power iteration (repeated multiply-and-renormalize);
//! - composing `Tensor` math with plain Rust arithmetic.
//!
//! ## Expected output
//! ```text
//! PageRank after 50 iterations (d = 0.85):
//!   node 0: 0.3725
//!   node 1: 0.1958
//!   node 2: 0.3941
//!   node 3: 0.0375
//! highest-ranked node: 2
//! Tiny PageRank: OK
//! ```

use matten::Tensor;

/// One PageRank power-iteration step.
fn pagerank_step(m: &Tensor, r: &Tensor, damping: f64, n: usize) -> Tensor {
    let mr = m.matmul(r); // [n, n] × [n] -> [n]
    let base = (1.0 - damping) / n as f64;
    let next: Vec<f64> = mr.as_slice().iter().map(|&x| base + damping * x).collect();
    Tensor::from_vec(next)
}

fn main() {
    // Directed graph on 4 nodes:
    //   0 -> 1, 0 -> 2, 1 -> 2, 2 -> 0, 3 -> 2
    // Column-stochastic link matrix M (M[i, j] = share of j's rank flowing to i):
    //          from: 0    1   2   3
    //   to 0 [ 0     0    1   0  ]
    //   to 1 [ 0.5   0    0   0  ]
    //   to 2 [ 0.5   1    0   1  ]
    //   to 3 [ 0     0    0   0  ]
    let m = Tensor::new(
        vec![
            0.0, 0.0, 1.0, 0.0, //
            0.5, 0.0, 0.0, 0.0, //
            0.5, 1.0, 0.0, 1.0, //
            0.0, 0.0, 0.0, 0.0, //
        ],
        &[4, 4],
    );

    let n = 4;
    let damping = 0.85;
    let mut r = Tensor::from_vec(vec![0.25, 0.25, 0.25, 0.25]);
    for _ in 0..50 {
        r = pagerank_step(&m, &r, damping, n);
    }

    let ranks = r.as_slice();
    println!("PageRank after 50 iterations (d = {damping}):");
    for (i, &rank) in ranks.iter().enumerate() {
        println!("  node {i}: {rank:.4}");
    }

    // The best-connected node should win; the link-less node 3 only gets teleport.
    let top = ranks
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(i, _)| i)
        .unwrap();
    println!("highest-ranked node: {top}");

    let total: f64 = ranks.iter().sum();
    assert!((total - 1.0).abs() < 1e-9, "ranks must sum to 1");
    assert_eq!(top, 2, "node 2 receives the most links");
    println!("Tiny PageRank: OK");
}
