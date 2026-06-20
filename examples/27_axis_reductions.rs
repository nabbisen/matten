//! Axis-based reductions: `sum_axis`, `mean_axis`, `min_axis`, `max_axis`.
//!
//! Run: cargo run --example 27_axis_reductions
//!
//! Reducing along an axis removes that axis from the output shape.
//! NaN propagates: if any element along a reduced axis is NaN, the
//! output cell for that slice is NaN.

use matten::Tensor;

fn main() {
    // Shape [3, 4]: 3 rows, 4 columns
    let m = Tensor::new(
        vec![
            1.0, 2.0, 3.0, 4.0, // row 0
            5.0, 6.0, 7.0, 8.0, // row 1
            9.0, 10.0, 11.0, 12.0, // row 2
        ],
        &[3, 4],
    );

    // Reduce rows → column sums: shape [4]
    let col_sums = m.sum_axis(0);
    assert_eq!(col_sums.shape(), &[4]);
    assert_eq!(col_sums.as_slice(), &[15.0, 18.0, 21.0, 24.0]);
    println!("col sums  = {:?}", col_sums.as_slice());

    // Reduce columns → row means: shape [3]
    let row_means = m.mean_axis(1);
    assert_eq!(row_means.shape(), &[3]);
    assert_eq!(row_means.as_slice(), &[2.5, 6.5, 10.5]);
    println!("row means = {:?}", row_means.as_slice());

    // Column-wise min and max
    let col_min = m.min_axis(0);
    let col_max = m.max_axis(0);
    assert_eq!(col_min.as_slice(), &[1.0, 2.0, 3.0, 4.0]);
    assert_eq!(col_max.as_slice(), &[9.0, 10.0, 11.0, 12.0]);
    println!("col min   = {:?}", col_min.as_slice());
    println!("col max   = {:?}", col_max.as_slice());

    // NaN propagation: one NaN contaminates its column's min
    let with_nan = Tensor::new(vec![1.0, f64::NAN, 3.0, 5.0, 6.0, 7.0], &[2, 3]);
    let nan_col_min = with_nan.min_axis(0);
    assert!(nan_col_min.as_slice()[1].is_nan()); // column 1 had a NaN
    assert_eq!(nan_col_min.as_slice()[0], 1.0); // column 0 clean
    println!("NaN propagation confirmed");

    println!("done.");
}
