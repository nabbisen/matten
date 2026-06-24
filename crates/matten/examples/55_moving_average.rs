//! Simple moving average using slice windows.
//!
//! Run: cargo run --example 55_moving_average

use matten::Tensor;

fn main() {
    let series = Tensor::from_vec(vec![1.0, 3.0, 5.0, 7.0, 9.0, 11.0, 13.0]);
    let window = 3usize;
    let n = series.len();

    // Compute moving averages for windows [0..3], [1..4], ...
    let mut avgs = Vec::new();
    for start in 0..=(n - window) {
        let w = series.slice().range(start..start + window).build().unwrap();
        avgs.push(w.mean());
    }

    println!("series = {:?}", series.as_slice());
    println!("3-pt moving avg = {:?}", avgs);
    assert_eq!(avgs.len(), n - window + 1);
    assert!((avgs[0] - 3.0).abs() < 1e-10); // mean(1,3,5) = 3
    assert!((avgs[1] - 5.0).abs() < 1e-10); // mean(3,5,7) = 5
    println!("Moving average: OK");
}
