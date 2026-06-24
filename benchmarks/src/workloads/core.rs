//! Core micro-workloads (RFC-049 §9.1, architect Q4 core set).
//!
//! Each function does the smallest meaningful unit of work over pre-built inputs.
//! Setup (input construction) lives in `crate::common` and in the bench harness,
//! not here, so the timed body measures the operation rather than allocation.

use matten::Tensor;

/// Construction: build an `[n]` tensor from owned data.
pub fn construction(n: usize) -> Tensor {
    Tensor::from_vec((0..n).map(|i| i as f64).collect())
}

/// Reshape then flatten a square matrix (shape juggling, no arithmetic).
pub fn reshape_flatten(square: &Tensor, side: usize) -> Tensor {
    square.reshape(&[side, side]).flatten()
}

/// Elementwise addition with broadcasting disabled (same shapes).
pub fn elementwise_add(a: &Tensor, b: &Tensor) -> Tensor {
    a + b
}

/// Elementwise multiplication (same shapes).
pub fn elementwise_mul(a: &Tensor, b: &Tensor) -> Tensor {
    a * b
}

/// Broadcasting: add a row vector `[cols]` to every row of an `[rows, cols]`
/// matrix.
pub fn broadcasting(matrix: &Tensor, row: &Tensor) -> Tensor {
    matrix + row
}

/// Whole-tensor reductions.
pub fn sum_mean(t: &Tensor) -> (f64, f64) {
    (t.sum(), t.mean())
}

/// Axis reductions over axis 0.
pub fn sum_mean_axis(t: &Tensor) -> (Tensor, Tensor) {
    (t.sum_axis(0), t.mean_axis(0))
}

/// Matrix multiplication `[m, k] x [k, n] -> [m, n]`.
pub fn matmul(a: &Tensor, b: &Tensor) -> Tensor {
    a.matmul(b)
}

/// Slicing: take the first `k` rows of a 2-D tensor via the builder.
pub fn slice_rows(t: &Tensor, k: usize) -> Tensor {
    t.slice()
        .range(0..k)
        .all()
        .build()
        .expect("valid non-dynamic row slice")
}

/// Dynamic ingestion: convert a fully-numeric dynamic tensor to a numeric one.
/// Behind the optional `dynamic` feature so Phase 1 stays minimal by default.
#[cfg(feature = "dynamic")]
pub fn dynamic_try_numeric(elements: &[matten::Element], shape: &[usize]) -> Tensor {
    let dynamic = Tensor::from_elements(elements.to_vec(), shape);
    dynamic
        .try_numeric()
        .expect("all-numeric dynamic input converts")
}
