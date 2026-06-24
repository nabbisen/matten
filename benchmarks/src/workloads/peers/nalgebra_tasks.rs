//! `nalgebra` implementations of the comparable peer tasks (RFC-049 Phase 2).
//!
//! `nalgebra` is a (small-)dense linear-algebra crate, so it is a natural peer for the
//! matrix/vector tasks. `DMatrix` is column-major; matrices are built from row-major
//! source data via `DMatrix::from_row_slice` in the bench, so the logical problem
//! matches `matten`/`ndarray`. Each task notes its representation.

use nalgebra::{DMatrix, DVector};

/// Cosine similarity of two vectors — dot / (‖a‖·‖b‖). Directly comparable.
pub fn cosine_similarity(a: &DVector<f64>, b: &DVector<f64>) -> f64 {
    a.dot(b) / (a.norm() * b.norm())
}

/// Small dense matrix multiplication. Directly comparable.
pub fn matmul(a: &DMatrix<f64>, b: &DMatrix<f64>) -> DMatrix<f64> {
    a * b
}

/// One Markov step `v · P`. Represented as a row vector times a matrix
/// (`distᵀ · P`), transposed back to a column vector — the same small matrix/vector
/// product as the other implementations.
pub fn markov_step(dist: &DVector<f64>, p: &DMatrix<f64>) -> DVector<f64> {
    (dist.transpose() * p).transpose()
}

/// One PageRank step: matrix × vector, then damping. Small matrix/vector product.
pub fn pagerank_step(m: &DMatrix<f64>, r: &DVector<f64>, damping: f64) -> DVector<f64> {
    let base = (1.0 - damping) / r.len() as f64;
    (m * r).map(|x| base + damping * x)
}

/// One linear-regression gradient-descent step. The design matrix and its transpose
/// are small dense matrices; the step is matrix/vector products plus a scaled update.
pub fn linreg_gd_step(
    x: &DMatrix<f64>,
    xt: &DMatrix<f64>,
    theta: &DVector<f64>,
    y: &DVector<f64>,
    lr: f64,
) -> DVector<f64> {
    let pred = x * theta;
    let residual = &pred - y;
    let grad = xt * &residual;
    theta - grad * lr
}

/// One 1-D heat-equation step (operator × state) — a small matrix/vector product.
pub fn heat_step(operator: &DMatrix<f64>, u: &DVector<f64>) -> DVector<f64> {
    operator * u
}
