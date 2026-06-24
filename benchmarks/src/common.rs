//! Shared, deterministic input generation for benchmark workloads.
//!
//! Inputs are pinned (no randomness, no I/O, no clock) so that benchmark setup is
//! reproducible and excluded from the timed body. Generators are cheap, pure
//! functions of their size arguments.

use matten::Tensor;

/// A deterministic 1-D tensor of length `n` with values `0.0..n`.
pub fn vector(n: usize) -> Tensor {
    Tensor::from_vec((0..n).map(|i| i as f64).collect())
}

/// A deterministic 1-D tensor of length `n` with a small bounded spread, useful
/// where large magnitudes would dominate (for example cosine similarity).
pub fn unit_spread_vector(n: usize) -> Tensor {
    Tensor::from_vec((0..n).map(|i| ((i % 7) as f64) + 1.0).collect())
}

/// A deterministic `[rows, cols]` matrix with values `0.0..rows*cols`.
pub fn matrix(rows: usize, cols: usize) -> Tensor {
    let data = (0..rows * cols).map(|i| i as f64).collect();
    Tensor::new(data, &[rows, cols])
}

/// A deterministic row-stochastic `[n, n]` matrix (each row sums to 1), suitable
/// for Markov / PageRank style power iterations.
pub fn row_stochastic(n: usize) -> Tensor {
    let mut data = vec![0.0; n * n];
    for r in 0..n {
        // Spread weight across the row in a fixed pattern, then normalize.
        let mut row_sum = 0.0;
        for c in 0..n {
            let w = ((r + c) % 5) as f64 + 1.0;
            data[r * n + c] = w;
            row_sum += w;
        }
        for c in 0..n {
            data[r * n + c] /= row_sum;
        }
    }
    Tensor::new(data, &[n, n])
}
