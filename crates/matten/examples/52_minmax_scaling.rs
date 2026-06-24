//! Min-max (0–1) scaling of each column.
//!
//! Run: cargo run --example 52_minmax_scaling

use matten::Tensor;

fn main() {
    let data = Tensor::new(
        vec![1.0, 10.0, 100.0, 2.0, 20.0, 200.0, 3.0, 30.0, 300.0],
        &[3, 3],
    );

    // Column min and max using axis reductions
    let col_min = data.min_axis(0);
    let col_max = data.max_axis(0);
    let range = &col_max - &col_min;

    // Broadcast: (data - min) / (max - min)
    let scaled = &(&data - &col_min) / &range;

    println!("col_min  = {:?}", col_min.as_slice());
    println!("col_max  = {:?}", col_max.as_slice());
    println!("scaled shape = {:?}", scaled.shape());

    // First row should be all 0.0, last row all 1.0
    let row0 = scaled.slice().index(0).all().build().unwrap();
    let row2 = scaled.slice().index(2).all().build().unwrap();
    assert!(row0.as_slice().iter().all(|&v| (v - 0.0).abs() < 1e-10));
    assert!(row2.as_slice().iter().all(|&v| (v - 1.0).abs() < 1e-10));
    println!("Min-max scaling: OK");
}
