//! # Example: Graph Path Counting
//!
//! Run: cargo run --example 32_graph_path_counting
//!
//! ## Problem
//! Count the number of *walks* of a given length between nodes of a directed
//! graph.
//!
//! ## Math idea
//! If `A` is the adjacency matrix of a graph (`A[i, j] = 1` when there is an edge
//! `i -> j`), then
//! ```text
//! (A^k)[i, j] = number of walks of length k from node i to node j.
//! ```
//! A *walk* may repeat nodes and edges; it is not the same as a *simple path*,
//! which may not.
//!
//! ## Tensor representation
//! The adjacency matrix is a 2-D `Tensor` of shape `[n, n]`. Each extra power is
//! one `Tensor::matmul`; entries are read with `Tensor::get`.
//!
//! ## What this demonstrates
//! - representing a graph as an adjacency `Tensor`;
//! - matrix powers via repeated `Tensor::matmul`;
//! - interpreting the entries of `A^k`.
//!
//! ## Expected output
//! ```text
//! A^2:
//!   [0, 0, 1, 2]
//!   [0, 0, 0, 1]
//!   [0, 0, 0, 0]
//!   [0, 0, 0, 0]
//! A^3:
//!   [0, 0, 0, 1]
//!   [0, 0, 0, 0]
//!   [0, 0, 0, 0]
//!   [0, 0, 0, 0]
//! walks of length 2 from 0 to 3 = 2
//! walks of length 3 from 0 to 3 = 1
//! Graph path counting: OK
//! ```

use matten::Tensor;

/// Returns `a` raised to the `k`-th matrix power via repeated `matmul`.
///
/// `a` must be a square 2-D tensor and `k >= 1`.
fn matrix_power(a: &Tensor, k: u32) -> Tensor {
    let mut acc = a.clone();
    for _ in 1..k {
        acc = acc.matmul(a);
    }
    acc
}

/// Pretty-print a small integer-valued matrix.
fn print_matrix(label: &str, m: &Tensor) {
    let (rows, cols) = (m.shape()[0], m.shape()[1]);
    println!("{label}:");
    for i in 0..rows {
        let row: Vec<String> = (0..cols)
            .map(|j| format!("{:.0}", m.get(&[i, j]).expect("index in bounds")))
            .collect();
        println!("  [{}]", row.join(", "));
    }
}

fn main() {
    // Directed graph on 4 nodes:
    //   0 -> 1, 0 -> 2, 1 -> 2, 1 -> 3, 2 -> 3
    //
    //        to: 0  1  2  3
    //   from 0 [ 0  1  1  0 ]
    //        1 [ 0  0  1  1 ]
    //        2 [ 0  0  0  1 ]
    //        3 [ 0  0  0  0 ]
    let a = Tensor::new(
        vec![
            0.0, 1.0, 1.0, 0.0, //
            0.0, 0.0, 1.0, 1.0, //
            0.0, 0.0, 0.0, 1.0, //
            0.0, 0.0, 0.0, 0.0, //
        ],
        &[4, 4],
    );

    let a2 = matrix_power(&a, 2);
    let a3 = matrix_power(&a, 3);

    print_matrix("A^2", &a2);
    print_matrix("A^3", &a3);

    // From node 0 to node 3: two length-2 walks (0->1->3, 0->2->3)
    // and one length-3 walk (0->1->2->3).
    let walks2 = a2.get(&[0, 3]).expect("index in bounds");
    let walks3 = a3.get(&[0, 3]).expect("index in bounds");
    println!("walks of length 2 from 0 to 3 = {walks2:.0}");
    println!("walks of length 3 from 0 to 3 = {walks3:.0}");

    assert_eq!(walks2, 2.0);
    assert_eq!(walks3, 1.0);
    println!("Graph path counting: OK");
}
