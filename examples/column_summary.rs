//! Per-column summary statistics: mean, min, max, std dev.
//!
//! Run: cargo run --example column_summary

use matten::Tensor;

fn main() {
    let data = Tensor::new(
        vec![
            2.0, 5.0, 10.0, 4.0, 7.0, 20.0, 6.0, 3.0, 15.0, 8.0, 11.0, 25.0, 10.0, 9.0, 30.0,
        ],
        &[5, 3],
    );

    let means = data.mean_axis(0);
    let mins = data.min_axis(0);
    let maxes = data.max_axis(0);
    let centred = &data - &means;
    let variances = (&centred * &centred).mean_axis(0);
    let stds: Vec<f64> = variances.as_slice().iter().map(|v| v.sqrt()).collect();
    let stds_t = Tensor::new(stds, &[3]);

    println!("col means = {:?}", means.as_slice());
    println!("col mins  = {:?}", mins.as_slice());
    println!("col maxes = {:?}", maxes.as_slice());
    println!("col stds  = {:?}", stds_t.as_slice());

    assert_eq!(means.shape(), &[3]);
    println!("Column summary: OK");
}
