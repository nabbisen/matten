//! # Companion example: linear-interpolation quantile (matten-stats, RFC-078)
//!
//! Run: cargo run -p matten-stats --example stats_quantile
//!
//! ## What this shows
//! `quantile(x, q)` interpolates linearly between the two nearest ranks of
//! the sorted sample; input order does not matter and the input is never
//! mutated.
//!
//! ## Teaching points
//! - `q = 0.0` is the minimum, `q = 1.0` is the maximum;
//! - `q = 0.5` on an even-length input interpolates between the two middle values.

use matten::Tensor;
use matten_stats::quantile;

fn main() {
    // Deliberately unsorted; quantile must not depend on input order.
    let x = Tensor::new(vec![30.0, 10.0, 40.0, 20.0], &[4]);

    let min = quantile(&x, 0.0).expect("valid input");
    let max = quantile(&x, 1.0).expect("valid input");
    let median = quantile(&x, 0.5).expect("valid input");

    println!("min = {min}, max = {max}, median = {median}");
    assert_eq!(min, 10.0);
    assert_eq!(max, 40.0);
    assert_eq!(median, 25.0); // interpolated midpoint of 20 and 30

    println!("quantile: OK");
}
