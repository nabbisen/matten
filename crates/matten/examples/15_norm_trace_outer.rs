//! Linalg core-lite: `norm`, `trace`, and `outer` (RFC-041).
//!
//! Run: cargo run --example 15_norm_trace_outer
//!
//! Core matten provides small linalg-adjacent helpers, not a linear algebra
//! backend. `norm` is the L2/Frobenius norm over all elements; `trace` sums the
//! diagonal of a rank-2 tensor (rectangular via min(rows, cols)); `outer` is the
//! rank-1 x rank-1 outer product. For inverse/determinant/eigen/SVD and friends,
//! use a specialized crate such as nalgebra or ndarray-linalg.

use matten::Tensor;

fn main() {
    // norm: L2 / Frobenius over all elements.
    let v = Tensor::from_vec(vec![3.0, 4.0]);
    println!("norm([3, 4])          = {}", v.norm());
    assert_eq!(v.norm(), 5.0);

    let m = Tensor::new(vec![1.0, 2.0, 2.0, 4.0], &[2, 2]);
    println!("Frobenius norm (2x2)  = {}", m.norm());

    // trace: diagonal sum of a rank-2 tensor (rectangular allowed).
    let sq = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    println!("trace(2x2)            = {}", sq.trace());
    assert_eq!(sq.trace(), 5.0);

    let rect = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    println!("trace(2x3, min diag)  = {}", rect.trace());
    assert_eq!(rect.trace(), 6.0); // self[0,0] + self[1,1] = 1 + 5

    // outer: rank-1 x rank-1 -> [m, n].
    let a = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    let b = Tensor::from_vec(vec![4.0, 5.0]);
    let o = a.outer(&b);
    println!("outer([3] x [2])      = shape {:?}", o.shape());
    assert_eq!(o.shape(), &[3, 2]);
    assert_eq!(o.as_slice(), &[4.0, 5.0, 8.0, 10.0, 12.0, 15.0]);

    // try_* forms return Result; e.g. trace on a non-rank-2 tensor is Err.
    assert!(Tensor::from_vec(vec![1.0, 2.0]).try_trace().is_err());

    println!("Norm/trace/outer: OK");
}
