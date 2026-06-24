//! Scenario workloads (RFC-049 §9.2, architect Q4 scenario set).
//!
//! Five small, well-known computations drawn from the existing `matten` examples:
//! cosine similarity (ex 26), Markov chain step (ex 33), tiny PageRank step
//! (ex 34), linear-regression gradient-descent step (ex 35), and a 1-D heat
//! equation step (ex 36). Each is one update/step so the bench can repeat it.

use matten::Tensor;

/// Euclidean magnitude of a 1-D tensor.
fn magnitude(v: &Tensor) -> f64 {
    v.as_slice().iter().map(|x| x * x).sum::<f64>().sqrt()
}

/// Cosine similarity between two 1-D tensors (ex 26).
pub fn cosine_similarity(a: &Tensor, b: &Tensor) -> f64 {
    let dot = a.dot(b).as_slice()[0];
    dot / (magnitude(a) * magnitude(b))
}

/// One Markov-chain update `v_next = v · P` (ex 33), vector × matrix.
pub fn markov_step(dist: &Tensor, p: &Tensor) -> Tensor {
    dist.matmul(p)
}

/// One PageRank power-iteration step with damping (ex 34).
pub fn pagerank_step(m: &Tensor, r: &Tensor, damping: f64) -> Tensor {
    let n = r.as_slice().len();
    let mr = m.matmul(r); // [n, n] x [n] -> [n]
    let base = (1.0 - damping) / n as f64;
    let next: Vec<f64> = mr.as_slice().iter().map(|&x| base + damping * x).collect();
    Tensor::from_vec(next)
}

/// One gradient-descent step for `y = X·theta` least squares (ex 35).
///
/// `xt` is `Xᵀ`, formed once by the caller and reused.
pub fn linreg_gd_step(x: &Tensor, xt: &Tensor, theta: &Tensor, y: &[f64], lr: f64) -> Tensor {
    let pred = x.matmul(theta); // [n]
    let residual: Vec<f64> = pred.as_slice().iter().zip(y).map(|(p, t)| p - t).collect();
    let grad = xt.matmul(&Tensor::from_vec(residual)); // [features]
    let updated: Vec<f64> = theta
        .as_slice()
        .iter()
        .zip(grad.as_slice())
        .map(|(w, g)| w - lr * g)
        .collect();
    Tensor::from_vec(updated)
}

/// One explicit-Euler step of the 1-D heat equation `u_next = A · u` (ex 36),
/// where `operator` is the precomputed `[n, n]` update matrix.
pub fn heat_step(operator: &Tensor, u: &Tensor) -> Tensor {
    operator.matmul(u)
}
