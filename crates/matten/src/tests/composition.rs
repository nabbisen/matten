//! Tests for shape composition (`concatenate`/`stack`, RFC-039).
//!
//! Validates the design specification: borrowed-slice input, output shape and
//! row-major data order, the empty/rank/dimension/axis error policy, single-input
//! behavior, allocation limits, and dynamic rejection.

use crate::{MattenError, Tensor};

// ----- concatenate: happy paths -----

#[test]
fn concatenate_vectors_axis0() {
    let a = Tensor::from_vec(vec![1.0, 2.0]);
    let b = Tensor::from_vec(vec![3.0, 4.0, 5.0]);
    let c = Tensor::concatenate(&[&a, &b], 0);
    assert_eq!(c.shape(), &[5]);
    assert_eq!(c.as_slice(), &[1.0, 2.0, 3.0, 4.0, 5.0]);
}

#[test]
fn concatenate_matrices_axis0() {
    // [2,3] + [4,3] -> [6,3]; non-square so the joined axis is visible.
    let a = Tensor::new((1..=6).map(f64::from).collect(), &[2, 3]);
    let b = Tensor::new((7..=18).map(f64::from).collect(), &[4, 3]);
    let c = Tensor::concatenate(&[&a, &b], 0);
    assert_eq!(c.shape(), &[6, 3]);
    assert_eq!(
        c.as_slice(),
        &(1..=18).map(f64::from).collect::<Vec<_>>()[..]
    );
}

#[test]
fn concatenate_matrices_axis1() {
    // [2,3] + [2,5] -> [2,8].
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let b = Tensor::new(
        vec![10.0, 11.0, 12.0, 13.0, 14.0, 20.0, 21.0, 22.0, 23.0, 24.0],
        &[2, 5],
    );
    let c = Tensor::concatenate(&[&a, &b], 1);
    assert_eq!(c.shape(), &[2, 8]);
    assert_eq!(
        c.as_slice(),
        &[
            1.0, 2.0, 3.0, 10.0, 11.0, 12.0, 13.0, 14.0, // row 0
            4.0, 5.0, 6.0, 20.0, 21.0, 22.0, 23.0, 24.0, // row 1
        ]
    );
}

#[test]
fn concatenate_three_inputs() {
    let a = Tensor::from_vec(vec![1.0]);
    let b = Tensor::from_vec(vec![2.0, 3.0]);
    let c = Tensor::from_vec(vec![4.0, 5.0, 6.0]);
    let out = Tensor::concatenate(&[&a, &b, &c], 0);
    assert_eq!(out.shape(), &[6]);
    assert_eq!(out.as_slice(), &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
}

#[test]
fn concatenate_single_input_is_clone_equivalent() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let c = Tensor::concatenate(&[&a], 0);
    assert_eq!(c.shape(), a.shape());
    assert_eq!(c.as_slice(), a.as_slice());
}

// ----- concatenate: error policy -----

#[test]
fn concatenate_empty_is_invalid_argument() {
    let err = Tensor::try_concatenate(&[], 0).unwrap_err();
    assert!(matches!(
        err,
        MattenError::InvalidArgument {
            operation: "concatenate",
            argument: "tensors",
            ..
        }
    ));
}

#[test]
fn concatenate_rank_mismatch_is_shape() {
    let a = Tensor::from_vec(vec![1.0, 2.0]); // [2]
    let b = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]); // [2,2]
    let err = Tensor::try_concatenate(&[&a, &b], 0).unwrap_err();
    assert!(matches!(
        err,
        MattenError::Shape {
            operation: "concatenate",
            ..
        }
    ));
}

#[test]
fn concatenate_dimension_mismatch_is_shape() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let b = Tensor::new(vec![1.0, 2.0, 3.0], &[1, 3]); // axis-1 size differs
    let err = Tensor::try_concatenate(&[&a, &b], 0).unwrap_err();
    assert!(matches!(
        err,
        MattenError::Shape {
            operation: "concatenate",
            ..
        }
    ));
}

#[test]
fn concatenate_axis_out_of_range_is_shape() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    // valid axis range is 0..2; axis 2 is out of range.
    let err = Tensor::try_concatenate(&[&a], 2).unwrap_err();
    assert!(matches!(
        err,
        MattenError::Shape {
            operation: "concatenate",
            ..
        }
    ));
}

// ----- stack: happy paths (non-square [2,4] across axes 0,1,2) -----

#[test]
fn stack_vectors_axis0_and_axis1() {
    let a = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    let b = Tensor::from_vec(vec![4.0, 5.0, 6.0]);

    let s0 = Tensor::stack(&[&a, &b], 0);
    assert_eq!(s0.shape(), &[2, 3]);
    assert_eq!(s0.as_slice(), &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

    let s1 = Tensor::stack(&[&a, &b], 1);
    assert_eq!(s1.shape(), &[3, 2]);
    assert_eq!(s1.as_slice(), &[1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
}

#[test]
fn stack_matrices_axis0() {
    let inputs: Vec<Tensor> = (0..3)
        .map(|k| Tensor::new((0..8).map(|i| f64::from(k * 8 + i)).collect(), &[2, 4]))
        .collect();
    let refs: Vec<&Tensor> = inputs.iter().collect();
    let s = Tensor::stack(&refs, 0);
    assert_eq!(s.shape(), &[3, 2, 4]);
    // axis 0 is contiguous tensor-major: t0 block, then t1, then t2.
    assert_eq!(
        s.as_slice(),
        &(0..24).map(f64::from).collect::<Vec<_>>()[..]
    );
}

#[test]
fn stack_matrices_axis1() {
    let t0 = Tensor::new((0..8).map(f64::from).collect(), &[2, 4]);
    let t1 = Tensor::new((100..108).map(f64::from).collect(), &[2, 4]);
    let s = Tensor::stack(&[&t0, &t1], 1);
    assert_eq!(s.shape(), &[2, 2, 4]);
    // out[i, k, j] = t_k[i, j]
    assert_eq!(
        s.as_slice(),
        &[
            0.0, 1.0, 2.0, 3.0, // i=0, k=0 (t0 row0)
            100.0, 101.0, 102.0, 103.0, // i=0, k=1 (t1 row0)
            4.0, 5.0, 6.0, 7.0, // i=1, k=0 (t0 row1)
            104.0, 105.0, 106.0, 107.0, // i=1, k=1 (t1 row1)
        ]
    );
}

#[test]
fn stack_matrices_axis2() {
    let t0 = Tensor::new((0..8).map(f64::from).collect(), &[2, 4]);
    let t1 = Tensor::new((100..108).map(f64::from).collect(), &[2, 4]);
    let s = Tensor::stack(&[&t0, &t1], 2);
    assert_eq!(s.shape(), &[2, 4, 2]);
    // out[i, j, k] = t_k[i, j]; innermost axis selects the tensor.
    assert_eq!(
        s.as_slice(),
        &[
            0.0, 100.0, 1.0, 101.0, 2.0, 102.0, 3.0, 103.0, // i=0
            4.0, 104.0, 5.0, 105.0, 6.0, 106.0, 7.0, 107.0, // i=1
        ]
    );
}

#[test]
fn stack_single_input_inserts_axis() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let s0 = Tensor::stack(&[&a], 0);
    assert_eq!(s0.shape(), &[1, 2, 2]);
    assert_eq!(s0.as_slice(), a.as_slice());

    let s2 = Tensor::stack(&[&a], 2);
    assert_eq!(s2.shape(), &[2, 2, 1]);
    assert_eq!(s2.as_slice(), a.as_slice());
}

// ----- stack: error policy -----

#[test]
fn stack_empty_is_invalid_argument() {
    let err = Tensor::try_stack(&[], 0).unwrap_err();
    assert!(matches!(
        err,
        MattenError::InvalidArgument {
            operation: "stack",
            argument: "tensors",
            ..
        }
    ));
}

#[test]
fn stack_shape_mismatch_is_shape() {
    let a = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    let b = Tensor::from_vec(vec![4.0, 5.0]); // different shape
    let err = Tensor::try_stack(&[&a, &b], 0).unwrap_err();
    assert!(matches!(
        err,
        MattenError::Shape {
            operation: "stack",
            ..
        }
    ));
}

#[test]
fn stack_axis_out_of_range_is_shape() {
    let a = Tensor::from_vec(vec![1.0, 2.0, 3.0]); // rank 1, valid 0..=1
    let err = Tensor::try_stack(&[&a], 2).unwrap_err();
    assert!(matches!(
        err,
        MattenError::Shape {
            operation: "stack",
            ..
        }
    ));
}

#[test]
fn stack_max_axis_equals_rank_is_allowed() {
    let a = Tensor::from_vec(vec![1.0, 2.0]); // rank 1
    let s = Tensor::stack(&[&a], 1); // axis == rank is the upper bound
    assert_eq!(s.shape(), &[2, 1]);
}

// ----- allocation limits -----

#[test]
fn stack_respects_dimension_limit() {
    // Stacking adds a rank. Build a tensor at the max rank so the stacked output
    // (rank + 1) trips the dimension limit -> Shape, never a silent huge alloc.
    let shape = vec![1usize; 8]; // MAX_NDIM default is 8
    let a = Tensor::new(vec![1.0], &shape);
    let err = Tensor::try_stack(&[&a], 0).unwrap_err();
    assert!(matches!(
        err,
        MattenError::Shape { .. } | MattenError::Allocation { .. }
    ));
}

// ----- dynamic rejection (try_* must Err, never panic) -----

#[cfg(feature = "dynamic")]
#[test]
fn concatenate_and_stack_reject_dynamic() {
    use crate::dynamic::Element;
    let numeric = Tensor::from_vec(vec![1.0, 2.0]);
    let dynamic = Tensor::from_elements(vec![Element::Float(1.0), Element::Float(2.0)], &[2]);
    assert!(dynamic.is_dynamic());

    let c = Tensor::try_concatenate(&[&numeric, &dynamic], 0).unwrap_err();
    assert!(matches!(
        c,
        MattenError::Unsupported {
            operation: "concatenate",
            ..
        }
    ));

    let s = Tensor::try_stack(&[&dynamic], 0).unwrap_err();
    assert!(matches!(
        s,
        MattenError::Unsupported {
            operation: "stack",
            ..
        }
    ));
}
