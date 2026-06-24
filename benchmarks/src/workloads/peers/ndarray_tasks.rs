//! `ndarray` implementations of the comparable peer tasks (RFC-049 Phase 2).
//!
//! `ndarray`'s `Array1`/`Array2` are row-major like `matten`'s `Tensor`, so all six
//! tasks map directly. Each function mirrors the corresponding `matten` workload
//! (`workloads::core` / `workloads::scenarios`) at identical sizes.

use ndarray::{Array1, Array2};

/// Cosine similarity of two vectors — same problem as `scenarios::cosine_similarity`.
pub fn cosine_similarity(a: &Array1<f64>, b: &Array1<f64>) -> f64 {
    a.dot(b) / (a.dot(a).sqrt() * b.dot(b).sqrt())
}

/// Small dense matrix multiplication — same problem as `core::matmul`.
pub fn matmul(a: &Array2<f64>, b: &Array2<f64>) -> Array2<f64> {
    a.dot(b)
}

/// One Markov step `v · P` (row vector × matrix) — same problem as `scenarios::markov_step`.
pub fn markov_step(dist: &Array1<f64>, p: &Array2<f64>) -> Array1<f64> {
    dist.dot(p)
}

/// One PageRank step (matrix × vector, then damping) — same as `scenarios::pagerank_step`.
pub fn pagerank_step(m: &Array2<f64>, r: &Array1<f64>, damping: f64) -> Array1<f64> {
    let base = (1.0 - damping) / r.len() as f64;
    m.dot(r).mapv(|x| base + damping * x)
}

/// One linear-regression gradient-descent step — same as `scenarios::linreg_gd_step`.
pub fn linreg_gd_step(
    x: &Array2<f64>,
    xt: &Array2<f64>,
    theta: &Array1<f64>,
    y: &Array1<f64>,
    lr: f64,
) -> Array1<f64> {
    let pred = x.dot(theta);
    let residual = &pred - y;
    let grad = xt.dot(&residual);
    theta - &(&grad * lr)
}

/// One 1-D heat-equation step (operator × state) — same as `scenarios::heat_step`.
pub fn heat_step(operator: &Array2<f64>, u: &Array1<f64>) -> Array1<f64> {
    operator.dot(u)
}
