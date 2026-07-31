//! # Companion example: histogram bin-selection policy (RFC-090)
//!
//! Run: cargo run -p matten-stats --example stats_histogram
//!
//! ## What this shows
//! - `bins` is a required argument — there is no automatic bin-count rule
//!   (not Sturges, not Freedman-Diaconis, not `"auto"`);
//! - the last bin is closed at the top, so the maximum value is never
//!   silently dropped from the counts;
//! - a constant input errors (`ZeroVariance`) instead of NumPy's silently
//!   widened `(v - 0.5, v + 0.5)` range.

use matten::Tensor;
use matten_stats::{MattenStatsError, histogram};

fn main() {
    let x = Tensor::new(vec![0.0, 1.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0], &[8]);

    let h = histogram(&x, 5).expect("valid input");
    println!("edges  = {:?}", h.edges);
    println!("counts = {:?}", h.counts);
    assert_eq!(h.edges, vec![0.0, 2.0, 4.0, 6.0, 8.0, 10.0]);
    assert_eq!(h.counts, vec![2, 2, 1, 1, 2]);

    // Nothing is dropped: the sum of counts always equals the input length,
    // including the maximum, which lands in the closed last bin.
    assert_eq!(h.counts.iter().sum::<usize>(), x.len());

    // A constant input errors rather than inventing a range.
    let constant = Tensor::new(vec![5.0, 5.0, 5.0], &[3]);
    let err = histogram(&constant, 4).unwrap_err();
    assert!(matches!(err, MattenStatsError::ZeroVariance));
    println!("constant input: {err} (not a silently widened range)");

    println!("stats_histogram: OK");
}
