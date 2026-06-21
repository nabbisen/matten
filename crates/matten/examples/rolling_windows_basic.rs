//! Rolling window operations: sum and max over overlapping slices.
//!
//! Run: cargo run --example rolling_windows_basic

use matten::Tensor;

fn rolling_sum(t: &Tensor, window: usize) -> Vec<f64> {
    (0..=(t.len() - window))
        .map(|i| t.slice().range(i..i + window).build().unwrap().sum())
        .collect()
}

fn rolling_max(t: &Tensor, window: usize) -> Vec<f64> {
    (0..=(t.len() - window))
        .map(|i| t.slice().range(i..i + window).build().unwrap().max())
        .collect()
}

fn main() {
    let series = Tensor::from_vec(vec![3.0, 1.0, 4.0, 1.0, 5.0, 9.0, 2.0, 6.0]);
    let w = 3;

    let sums = rolling_sum(&series, w);
    let maxes = rolling_max(&series, w);

    println!("series       = {:?}", series.as_slice());
    println!("rolling sum  = {:?}", sums);
    println!("rolling max  = {:?}", maxes);

    assert_eq!(sums[0], 8.0); // 3+1+4
    assert_eq!(maxes[0], 4.0); // max(3,1,4)
    println!("Rolling windows: OK");
}
