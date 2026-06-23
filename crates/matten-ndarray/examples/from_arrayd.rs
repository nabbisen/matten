//! # Companion example: `ArrayD` -> `Tensor` (matten-ndarray)
//!
//! Run: cargo run -p matten-ndarray --example from_arrayd
//!
//! ## What this shows
//! Converting an `ndarray::ArrayD<f64>` into a `matten::Tensor`, including a
//! non-standard-layout (transposed) input.
//!
//! ## Teaching points
//! - the conversion **copies** data into the `Tensor` (no zero-copy / borrow);
//! - logical shape and element order are preserved, even for a transposed
//!   (non-contiguous) input;
//! - the dependency direction is one-way: `matten-ndarray` depends on `matten`
//!   and `ndarray`, while core `matten` depends on neither.

use matten_ndarray::from_arrayd;
use ndarray::{ArrayD, IxDyn};

fn main() {
    let arr = ArrayD::from_shape_vec(IxDyn(&[2, 3]), vec![1., 2., 3., 4., 5., 6.]).unwrap();

    let t = from_arrayd(arr.clone()).expect("contiguous converts");
    println!(
        "from contiguous: shape {:?} data {:?}",
        t.shape(),
        t.as_slice()
    );

    // Transposed input is non-standard layout; conversion preserves logical order.
    let tt = from_arrayd(arr.t().to_owned()).expect("transposed converts");
    println!(
        "from transposed: shape {:?} data {:?}",
        tt.shape(),
        tt.as_slice()
    );
    assert_eq!(tt.shape(), &[3, 2]);
    assert_eq!(tt.as_slice(), &[1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
    println!("ok");
}
