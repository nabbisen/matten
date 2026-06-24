//! Shape composition: joining tensors with `concatenate` and `stack` (RFC-039).
//!
//! Run: cargo run --example 14_concatenate_stack
//!
//! `concatenate` joins tensors along an existing axis (same rank, matching sizes on
//! every non-concatenation axis). `stack` joins equally shaped tensors along a new
//! axis, so the output rank grows by one. Both take a borrowed slice `&[&Tensor]`
//! and reject dynamic tensors; the `try_*` forms return a `Result`.

use matten::Tensor;

fn main() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let b = Tensor::new(vec![5.0, 6.0, 7.0, 8.0], &[2, 2]);

    // concatenate joins an existing axis.
    let cat0 = Tensor::concatenate(&[&a, &b], 0);
    println!("concatenate axis 0 -> shape {:?}", cat0.shape());
    assert_eq!(cat0.shape(), &[4, 2]);

    let cat1 = Tensor::concatenate(&[&a, &b], 1);
    println!("concatenate axis 1 -> shape {:?}", cat1.shape());
    assert_eq!(cat1.shape(), &[2, 4]);

    // stack adds a new axis (rank + 1). The new axis position selects the tensor.
    let st0 = Tensor::stack(&[&a, &b], 0);
    println!("stack axis 0      -> shape {:?}", st0.shape());
    assert_eq!(st0.shape(), &[2, 2, 2]);

    let st2 = Tensor::stack(&[&a, &b], 2);
    println!("stack axis 2      -> shape {:?}", st2.shape());
    assert_eq!(st2.shape(), &[2, 2, 2]);
    assert_eq!(st2.as_slice(), &[1.0, 5.0, 2.0, 6.0, 3.0, 7.0, 4.0, 8.0]);

    // try_* forms return Result instead of panicking; e.g. a rank mismatch is Err.
    let v = Tensor::from_vec(vec![1.0, 2.0]);
    assert!(Tensor::try_concatenate(&[&a, &v], 0).is_err());

    println!("Concatenate/stack: OK");
}
