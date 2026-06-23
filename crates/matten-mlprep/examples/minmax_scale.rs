//! # Companion example: min-max scale feature columns (matten-mlprep)
//!
//! Run: cargo run -p matten-mlprep --example mlprep_minmax_scale
//!
//! ## What this shows
//! Scaling each feature column of a `[samples, features]` matrix into `[0, 1]`.
//!
//! ## Teaching points
//! - rows are samples, columns are features;
//! - per column, the minimum maps to 0 and the maximum to 1;
//! - deterministic, with no fitted model state.

use matten::Tensor;
use matten_mlprep::minmax_scale_columns;

fn main() {
    // 3 samples, 2 features.
    let x = Tensor::new(vec![0.0, 100.0, 5.0, 150.0, 10.0, 200.0], &[3, 2]);
    let s = minmax_scale_columns(&x).expect("two non-constant columns");
    println!("input  shape {:?}: {:?}", x.shape(), x.as_slice());
    println!("scaled shape {:?}: {:?}", s.shape(), s.as_slice());

    // Per column: min -> 0.0, max -> 1.0 (column 0 = [0,5,10], column 1 = [100,150,200]).
    assert_eq!(s.shape(), x.shape());
    let ss = s.as_slice();
    assert_eq!(ss[0], 0.0); // col 0 min
    assert_eq!(ss[4], 1.0); // col 0 max
    assert_eq!(ss[1], 0.0); // col 1 min
    assert_eq!(ss[5], 1.0); // col 1 max
    println!("minmax_scale: OK");
}
