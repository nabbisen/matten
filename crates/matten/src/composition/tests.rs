//! Tests for shape composition (`concatenate`/`stack`, RFC-039;
//! `repeat`/`tile`/`meshgrid`, RFC-087).
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

// ===== repeat / repeat_axis / tile / meshgrid (RFC-087) =====

// ----- repeat vs tile: the exact-value pair that prevents the classic swap -----

#[test]
fn repeat_repeats_each_element() {
    let a = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    let r = a.repeat(2);
    assert_eq!(r.shape(), &[6]);
    assert_eq!(r.as_slice(), &[1.0, 1.0, 2.0, 2.0, 3.0, 3.0]);
}

#[test]
fn tile_repeats_the_whole_tensor() {
    let a = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    let t = a.tile(&[2]);
    assert_eq!(t.shape(), &[6]);
    assert_eq!(t.as_slice(), &[1.0, 2.0, 3.0, 1.0, 2.0, 3.0]);
}

// ----- repeat: rank-2 flattens; repeat_axis preserves rank -----

#[test]
fn repeat_on_rank2_input_flattens_to_rank1() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let r = a.repeat(2);
    assert_eq!(r.shape(), &[8]);
    assert_eq!(r.as_slice(), &[1.0, 1.0, 2.0, 2.0, 3.0, 3.0, 4.0, 4.0]);
}

#[test]
fn repeat_axis0_and_axis1_of_the_same_matrix() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);

    let r0 = a.repeat_axis(2, 0);
    assert_eq!(r0.shape(), &[4, 2]);
    assert_eq!(r0.as_slice(), &[1.0, 2.0, 1.0, 2.0, 3.0, 4.0, 3.0, 4.0]);

    let r1 = a.repeat_axis(2, 1);
    assert_eq!(r1.shape(), &[2, 4]);
    assert_eq!(r1.as_slice(), &[1.0, 1.0, 2.0, 2.0, 3.0, 3.0, 4.0, 4.0]);
}

#[test]
fn repeat_axis_on_rank0_scalar_is_shape_error() {
    let s = Tensor::scalar(3.0);
    let err = Tensor::try_repeat_axis(&s, 2, 0).unwrap_err();
    assert!(matches!(
        err,
        MattenError::Shape {
            operation: "repeat_axis",
            ..
        }
    ));
}

#[test]
fn repeat_on_rank0_scalar_produces_rank1() {
    let s = Tensor::scalar(7.0);
    let r = s.repeat(3);
    assert_eq!(r.shape(), &[3]);
    assert_eq!(r.as_slice(), &[7.0, 7.0, 7.0]);
}

// ----- tile: reps shorter than rank prepends 1s; longer than rank errors -----

#[test]
fn tile_reps_shorter_than_rank_prepends_ones() {
    // [[1, 2]] shape [1, 2]; reps = [2] pads to [1, 2] -> out shape [1, 4].
    let a = Tensor::new(vec![1.0, 2.0], &[1, 2]);
    let t = a.tile(&[2]);
    assert_eq!(t.shape(), &[1, 4]);
    assert_eq!(t.as_slice(), &[1.0, 2.0, 1.0, 2.0]);
}

#[test]
fn tile_reps_matching_rank_two_by_two() {
    // RFC-087 §4 example: tile(&[2, 1]) on [[1, 2]] -> [[1, 2], [1, 2]].
    let a = Tensor::new(vec![1.0, 2.0], &[1, 2]);
    let t = a.tile(&[2, 1]);
    assert_eq!(t.shape(), &[2, 2]);
    assert_eq!(t.as_slice(), &[1.0, 2.0, 1.0, 2.0]);
}

#[test]
fn tile_reps_longer_than_rank_is_shape_error_naming_both_lengths() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]); // rank 2
    let err = Tensor::try_tile(&a, &[1, 1, 1]).unwrap_err(); // reps len 3
    match err {
        MattenError::Shape { operation, message } => {
            assert_eq!(operation, "tile");
            assert!(
                message.contains('3'),
                "message should name reps length 3: {message}"
            );
            assert!(
                message.contains('2'),
                "message should name rank 2: {message}"
            );
        }
        other => panic!("expected MattenError::Shape, got {other:?}"),
    }
}

// ----- repeat / repeat_axis / tile: n = 0, empty reps, rep = 0 -----
//
// RFC-111 (T8): n == 0 / rep == 0 used to be rejected by each function's own
// dedicated guard, independent of checked_shape_len. That guard is gone; these
// three now assert the inverse -- an empty, not-an-error result.

#[test]
fn repeat_n_zero_is_empty_not_an_error() {
    let a = Tensor::from_vec(vec![1.0, 2.0]);
    let r = Tensor::try_repeat(&a, 0).unwrap();
    assert_eq!(r.shape(), &[0]);
    assert!(r.is_empty());
}

#[test]
fn repeat_axis_n_zero_is_empty_not_an_error() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let r = Tensor::try_repeat_axis(&a, 0, 0).unwrap();
    assert_eq!(r.shape(), &[0, 2]);
    assert!(r.is_empty());
}

#[test]
fn repeat_axis_out_of_range_is_shape_error() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]); // rank 2, valid 0..2
    let err = Tensor::try_repeat_axis(&a, 2, 2).unwrap_err();
    assert!(matches!(
        err,
        MattenError::Shape {
            operation: "repeat_axis",
            ..
        }
    ));
}

#[test]
fn tile_empty_reps_is_shape_error() {
    let a = Tensor::from_vec(vec![1.0, 2.0]);
    let err = Tensor::try_tile(&a, &[]).unwrap_err();
    assert!(matches!(
        err,
        MattenError::Shape {
            operation: "tile",
            ..
        }
    ));
}

#[test]
fn tile_rep_zero_is_empty_not_an_error() {
    // RFC-111 (T8): tile's own `reps.contains(&0)` guard is gone.
    let a = Tensor::from_vec(vec![1.0, 2.0]);
    let r = Tensor::try_tile(&a, &[0]).unwrap();
    assert_eq!(r.shape(), &[0]);
    assert!(r.is_empty());
}

// ----- repeat / repeat_axis / tile with an EMPTY SOURCE tensor (RFC-111 SS3) -----
//
// The risk named at review: these three now can also receive an already-empty
// source (not just an empty n/reps parameter). tile's coordinate loop divides
// by each input dimension (`c % dim`); a zero dim must never be reached by
// that division. Both non-empty fixtures are checked (both orientations).

fn empty_0x3() -> Tensor {
    Tensor::new(vec![1., 2., 3., 4., 5., 6.], &[2, 3])
        .slice()
        .range(0..0)
        .all()
        .build()
        .unwrap()
}

fn empty_3x0() -> Tensor {
    Tensor::new(vec![1., 2., 3.], &[3, 1])
        .slice()
        .all()
        .range(0..0)
        .build()
        .unwrap()
}

#[test]
fn repeat_on_empty_source() {
    let a = empty_0x3();
    let r = a.try_repeat(2).unwrap();
    assert_eq!(r.shape(), &[0]);
    assert!(r.is_empty());
}

#[test]
fn repeat_axis_on_empty_source() {
    let a = empty_0x3();
    let r = a.try_repeat_axis(2, 0).unwrap();
    assert_eq!(r.shape(), &[0, 3]);
    assert!(r.is_empty());
}

#[test]
fn tile_on_empty_source_both_orientations() {
    // No panic from `c % dim`: the coordinate loop only runs when the output
    // total is non-zero, and it never is here.
    let a = empty_0x3();
    let r = a.try_tile(&[2, 2]).unwrap();
    assert_eq!(r.shape(), &[0, 6]);
    assert!(r.is_empty());

    let b = empty_3x0();
    let r = b.try_tile(&[2, 2]).unwrap();
    assert_eq!(r.shape(), &[6, 0]);
    assert!(r.is_empty());
}

// ----- meshgrid -----

#[test]
fn meshgrid_with_unequal_input_lengths_pins_xy_indexing() {
    // The mandatory test (RFC-087 §5): equal-length inputs cannot distinguish
    // xy from ij indexing (they differ only by a transpose, same shape either
    // way). Unequal lengths make a transposed (ij) implementation fail on shape
    // alone.
    let x = Tensor::from_vec(vec![1.0, 2.0, 3.0]); // len 3
    let y = Tensor::from_vec(vec![10.0, 20.0]); // len 2
    let (gx, gy) = Tensor::meshgrid(&x, &y);

    assert_eq!(gx.shape(), &[2, 3]); // NOT [3, 2]
    assert_eq!(gy.shape(), &[2, 3]);

    // out_x[i][j] == x[j]: each row is a full copy of x.
    assert_eq!(gx.as_slice(), &[1.0, 2.0, 3.0, 1.0, 2.0, 3.0]);
    // out_y[i][j] == y[i]: each row is constant, equal to y[i].
    assert_eq!(gy.as_slice(), &[10.0, 10.0, 10.0, 20.0, 20.0, 20.0]);
}

#[test]
fn meshgrid_rank2_input_is_shape_error_not_flattened() {
    let matrix = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let vector = Tensor::from_vec(vec![1.0, 2.0]);

    let err_x = Tensor::try_meshgrid(&matrix, &vector).unwrap_err();
    assert!(matches!(
        err_x,
        MattenError::Shape {
            operation: "meshgrid",
            ..
        }
    ));

    let err_y = Tensor::try_meshgrid(&vector, &matrix).unwrap_err();
    assert!(matches!(
        err_y,
        MattenError::Shape {
            operation: "meshgrid",
            ..
        }
    ));
}

// ----- allocation guard: checked product trips Allocation, never a huge alloc -----

#[test]
fn repeat_respects_allocation_limit() {
    let a = Tensor::from_vec(vec![1.0]); // 1 element
    let n = crate::MattenLimits::default().max_elements + 1;
    let err = Tensor::try_repeat(&a, n).unwrap_err();
    assert!(matches!(err, MattenError::Allocation { .. }));
}

#[test]
fn repeat_axis_respects_allocation_limit() {
    let a = Tensor::new(vec![1.0, 2.0], &[1, 2]);
    let n = crate::MattenLimits::default().max_elements + 1;
    let err = Tensor::try_repeat_axis(&a, n, 0).unwrap_err();
    assert!(matches!(err, MattenError::Allocation { .. }));
}

#[test]
fn tile_respects_allocation_limit() {
    let a = Tensor::from_vec(vec![1.0, 2.0]); // 2 elements
    let reps = crate::MattenLimits::default().max_elements + 1;
    let err = Tensor::try_tile(&a, &[reps]).unwrap_err();
    assert!(matches!(err, MattenError::Allocation { .. }));
}

#[test]
fn meshgrid_respects_allocation_limit() {
    // m * n must exceed max_elements without either input itself being huge.
    let x = Tensor::new((0..2000).map(f64::from).collect(), &[2000]);
    let y = Tensor::new((0..600).map(f64::from).collect(), &[600]);
    assert!(2000usize * 600 > crate::MattenLimits::default().max_elements);
    let err = Tensor::try_meshgrid(&x, &y).unwrap_err();
    assert!(matches!(err, MattenError::Allocation { .. }));
}

// ----- dynamic rejection -----

#[cfg(feature = "dynamic")]
#[test]
fn repeat_tile_meshgrid_reject_dynamic() {
    use crate::dynamic::Element;
    let numeric = Tensor::from_vec(vec![1.0, 2.0]);
    let dynamic = Tensor::from_elements(vec![Element::Float(1.0), Element::Float(2.0)], &[2]);
    assert!(dynamic.is_dynamic());

    assert!(matches!(
        Tensor::try_repeat(&dynamic, 2).unwrap_err(),
        MattenError::Unsupported {
            operation: "repeat",
            ..
        }
    ));
    assert!(matches!(
        Tensor::try_repeat_axis(&dynamic, 2, 0).unwrap_err(),
        MattenError::Unsupported {
            operation: "repeat_axis",
            ..
        }
    ));
    assert!(matches!(
        Tensor::try_tile(&dynamic, &[2]).unwrap_err(),
        MattenError::Unsupported {
            operation: "tile",
            ..
        }
    ));
    assert!(matches!(
        Tensor::try_meshgrid(&dynamic, &numeric).unwrap_err(),
        MattenError::Unsupported {
            operation: "meshgrid",
            ..
        }
    ));
    assert!(matches!(
        Tensor::try_meshgrid(&numeric, &dynamic).unwrap_err(),
        MattenError::Unsupported {
            operation: "meshgrid",
            ..
        }
    ));
}

// ----- concatenate / stack / meshgrid accept a zero-sized result (RFC-111 T4) -----

#[test]
fn concatenate_accepts_a_zero_sized_result() {
    let a = empty_0x3();
    let b = empty_0x3();
    let r = Tensor::try_concatenate(&[&a, &b], 0).unwrap();
    assert_eq!(r.shape(), &[0, 3]);
    assert!(r.is_empty());
}

#[test]
fn stack_accepts_a_zero_sized_result() {
    let a = empty_0x3();
    let b = empty_0x3();
    let r = Tensor::try_stack(&[&a, &b], 0).unwrap();
    assert_eq!(r.shape(), &[2, 0, 3]);
    assert!(r.is_empty());
}

#[test]
fn meshgrid_accepts_a_zero_length_input() {
    let x = Tensor::new(vec![1.0, 2.0, 3.0], &[3])
        .slice()
        .range(0..0)
        .build()
        .unwrap();
    assert_eq!(x.shape(), &[0]);
    let y = Tensor::from_vec(vec![1.0, 2.0]);
    let (gx, gy) = Tensor::try_meshgrid(&x, &y).unwrap();
    assert_eq!(gx.shape(), &[2, 0]);
    assert_eq!(gy.shape(), &[2, 0]);
    assert!(gx.is_empty());
    assert!(gy.is_empty());
}
