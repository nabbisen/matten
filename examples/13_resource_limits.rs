//! Demonstrating `MattenLimits` and boundary-safe constructors.
//!
//! Run: cargo run --example 13_resource_limits
//!
//! `matten` uses `MattenLimits` as the single source of truth for allocation
//! budgets. The `try_zeros`, `try_ones`, and `try_full` constructors return
//! `Result` instead of panicking, enabling safe use at data boundaries.

use matten::{MattenError, MattenLimits, Tensor};

fn main() {
    // ── Default limits ────────────────────────────────────────────────────
    let limits = MattenLimits::default();
    println!("Default max_dimensions: {}", limits.max_dimensions);
    println!(
        "Default max_elements:   {} (~{} MiB f64)",
        limits.max_elements,
        limits.max_elements * 8 / 1_048_576
    );

    // ── Boundary-safe constructors ────────────────────────────────────────
    let t = Tensor::try_zeros(&[100, 100]).unwrap();
    assert_eq!(t.shape(), &[100, 100]);
    assert_eq!(t.len(), 10_000);
    println!("try_zeros([100, 100]): OK, {} elements", t.len());

    let t = Tensor::try_ones(&[50, 20]).unwrap();
    assert_eq!(t.as_slice().iter().sum::<f64>(), 1000.0);
    println!("try_ones([50, 20]):    OK, sum = {}", t.sum());

    let t = Tensor::try_full(&[10, 10], -1.0).unwrap();
    assert_eq!(t.as_slice()[0], -1.0);
    println!("try_full([10, 10], -1.0): OK");

    // ── Custom limits ─────────────────────────────────────────────────────
    let tight = MattenLimits {
        max_elements: 10,
        ..MattenLimits::default()
    };
    let err = Tensor::try_zeros_with_limits(&[100], &tight).unwrap_err();
    assert!(matches!(err, MattenError::Allocation { .. }));
    println!("Custom limit (max=10): correctly rejected [100]");

    // ── Panicking variants respect limits too ─────────────────────────────
    // These delegate to try_zeros/try_ones/try_full internally:
    let _t = Tensor::zeros(&[10, 10]); // fine — 100 elements
    println!("zeros([10, 10]):        OK (panicking form, within limit)");

    println!("done.");
}
