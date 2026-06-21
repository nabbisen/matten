//! Unit tests for the broadcast module.

use super::{BroadcastCtx, broadcast_shape};

#[test]
fn broadcast_shape_same_rank_equal_dims() {
    assert_eq!(broadcast_shape(&[2, 3], &[2, 3]).unwrap(), vec![2, 3]);
}

#[test]
fn broadcast_shape_scalar_to_matrix() {
    assert_eq!(broadcast_shape(&[], &[2, 3]).unwrap(), vec![2, 3]);
    assert_eq!(broadcast_shape(&[2, 3], &[]).unwrap(), vec![2, 3]);
}

#[test]
fn broadcast_shape_vector_to_matrix() {
    assert_eq!(broadcast_shape(&[4], &[3, 4]).unwrap(), vec![3, 4]);
    assert_eq!(broadcast_shape(&[3, 4], &[4]).unwrap(), vec![3, 4]);
}

#[test]
fn broadcast_shape_unit_dim_expansion() {
    assert_eq!(broadcast_shape(&[3, 1], &[1, 4]).unwrap(), vec![3, 4]);
    assert_eq!(broadcast_shape(&[1, 4], &[3, 1]).unwrap(), vec![3, 4]);
}

#[test]
fn broadcast_shape_incompatible_returns_err() {
    assert!(broadcast_shape(&[2, 3], &[2]).is_err());
    assert!(broadcast_shape(&[3], &[4]).is_err());
}

#[test]
fn broadcast_ctx_same_shape_identity() {
    // For same-shape tensors the context should map each flat index to itself.
    let shape = &[2usize, 3];
    let ctx = BroadcastCtx::new(shape, shape, shape);
    assert_eq!(ctx.result_len(), 6);
    for i in 0..6 {
        assert_eq!(ctx.left_flat(i), i);
        assert_eq!(ctx.right_flat(i), i);
    }
}

#[test]
fn broadcast_ctx_scalar_repeats_for_all() {
    // Left is scalar [], right is [3]; result is [3].
    let left = &[][..];
    let right = &[3usize][..];
    let result = &[3usize][..];
    let ctx = BroadcastCtx::new(left, right, result);
    for i in 0..3 {
        assert_eq!(ctx.left_flat(i), 0, "scalar should always map to index 0");
        assert_eq!(ctx.right_flat(i), i);
    }
}

#[test]
fn broadcast_ctx_column_broadcast() {
    // left [3,1], right [1,4], result [3,4]
    let left = &[3usize, 1];
    let right = &[1usize, 4];
    let result = &[3usize, 4];
    let ctx = BroadcastCtx::new(left, right, result);
    assert_eq!(ctx.result_len(), 12);
    // Row 0 of result: left maps to row 0 col 0 (flat 0), right maps to row 0 cols 0-3 (0-3)
    assert_eq!(ctx.left_flat(0), 0);
    assert_eq!(ctx.left_flat(1), 0);
    assert_eq!(ctx.right_flat(0), 0);
    assert_eq!(ctx.right_flat(1), 1);
    // Row 1 of result: left maps to flat 1, right maps to 0-3
    assert_eq!(ctx.left_flat(4), 1);
    assert_eq!(ctx.right_flat(4), 0);
}
