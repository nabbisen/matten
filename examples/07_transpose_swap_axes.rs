//! Transpose and axis swapping: owned, lifetime-free, row-major result.
//!
//! Run: cargo run --example 07_transpose_swap_axes

use matten::Tensor;

fn main() {
    // 2-D transpose: swap rows and columns
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    println!("original  {m:?}");
    let mt = m.transpose();
    println!("transposed {mt:?}");
    assert_eq!(mt.shape(), &[3, 2]);

    // t() is an alias
    assert_eq!(m.t(), mt);

    // Transpose twice is identity
    assert_eq!(m.transpose().transpose(), m);
    println!("transpose twice == identity: OK");

    // swap_axes on rank-3
    let r3 = Tensor::new((1..=24).map(|x| x as f64).collect(), &[2, 3, 4]);
    let s = r3.swap_axes(0, 2);
    println!("rank-3 swap_axes(0,2) shape: {:?}", s.shape()); // [4,3,2]
}
