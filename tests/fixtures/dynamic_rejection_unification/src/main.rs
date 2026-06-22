//! Regression fixture for RFC-031 — feature-unification dynamic rejection.
//!
//! Core `matten` is compiled with `dynamic`; companion crates use their
//! default features (their own `dynamic` mirror is NOT enabled). Before
//! v0.19.1 a dynamic `Tensor` would reach numeric accessors and panic because
//! the companion guards were behind `#[cfg(feature = "dynamic")]`. After
//! v0.19.1 each guard calls `Tensor::is_dynamic()` unconditionally, so a
//! dynamic `Tensor` always returns `Err(DynamicTensor)`.
//!
//! Run with: `cargo run` from this directory, or invoke via the CI script.

use matten::{Element, Tensor};
use matten_ndarray::{MattenNdarrayError, to_arrayd};
use matten_mlprep::{MattenMlprepError, standardize_columns};

fn make_dynamic() -> Tensor {
    Tensor::from_elements(
        vec![
            Element::Float(1.0),
            Element::Float(2.0),
            Element::Float(3.0),
            Element::Float(4.0),
        ],
        &[2, 2],
    )
}

fn assert_no_panic<F: FnOnce() + std::panic::UnwindSafe>(label: &str, f: F) {
    let result = std::panic::catch_unwind(f);
    assert!(result.is_ok(), "PANIC in {label} — RFC-031 regression!");
}

fn main() {
    // Confirm we have a dynamic tensor (requires core/dynamic to be ON).
    let t = make_dynamic();
    assert!(t.is_dynamic(), "is_dynamic() must return true for a dynamic tensor");

    // --- matten-ndarray ---
    assert_no_panic("to_arrayd", || {
        let r = to_arrayd(&make_dynamic());
        assert!(
            matches!(r, Err(MattenNdarrayError::DynamicTensor)),
            "expected Err(DynamicTensor) from to_arrayd, got: {r:?}"
        );
    });

    // --- matten-mlprep ---
    assert_no_panic("standardize_columns", || {
        let r = standardize_columns(&make_dynamic());
        assert!(
            matches!(r, Err(MattenMlprepError::DynamicTensor)),
            "expected Err(DynamicTensor) from standardize_columns, got: {r:?}"
        );
    });

    // Numeric tensor: is_dynamic() must return false, conversions must succeed.
    let numeric = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    assert!(!numeric.is_dynamic(), "numeric tensor must not be dynamic");
    to_arrayd(&numeric).expect("numeric tensor must convert without error");

    println!("RFC-031 regression fixture: all assertions passed.");
}
