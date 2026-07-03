//! # Companion example: visual standardization summary (matten-mlprep)
//!
//! Run: cargo run -p matten-mlprep --example mlprep_visual_standardize_summary
//!
//! ## What this shows
//! A compact before/after summary for `standardize_columns`: per-column mean,
//! per-column standard deviation, and the fact that the shape is unchanged.
//!
//! ## Teaching points
//! - rows are samples, columns are features;
//! - standardization changes values, not shape;
//! - the report is explanatory only: no model-quality or data-quality judgment.

use matten::Tensor;
use matten_mlprep::standardize_columns;

fn format_values(values: &[f64]) -> String {
    let values = values
        .iter()
        .map(|&value| {
            let stable = if value.abs() < 0.0005 { 0.0 } else { value };
            format!("{stable:.3}")
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{values}]")
}

fn print_stats(label: &str, t: &Tensor) {
    let mean = t.mean_axis(0);
    let std = t.std_axis(0);
    println!("{label:<13} mean={}", format_values(mean.as_slice()));
    println!("{:<13} std={}", "", format_values(std.as_slice()));
}

fn main() {
    let input = Tensor::new(vec![8.0, 80.0, 10.0, 100.0, 12.0, 120.0], &[3, 2]);

    println!("== Standardize columns ==");
    println!("input shape   {:?}", input.shape());
    print_stats("before", &input);

    let standardized = standardize_columns(&input).expect("two non-constant columns");
    println!("after shape   {:?}", standardized.shape());
    print_stats("after", &standardized);
    println!("meaning       standardize_columns changes values, not shape.");

    assert_eq!(standardized.shape(), input.shape());
    let after_mean = standardized.mean_axis(0);
    let after_std = standardized.std_axis(0);
    for &value in after_mean.as_slice() {
        assert!(value.abs() < 1e-9);
    }
    for &value in after_std.as_slice() {
        assert!((value - 1.0).abs() < 1e-9);
    }

    println!("mlprep_visual_standardize_summary: OK");
}
