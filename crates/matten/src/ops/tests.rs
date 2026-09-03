use crate::{MattenLimits, Tensor};

// ---- broadcasting (M3) -------------------------------------------------

#[test]
fn broadcast_same_shape() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let b = Tensor::new(vec![10.0, 20.0, 30.0, 40.0], &[2, 2]);
    let c = &a + &b;
    assert_eq!(c.shape(), &[2, 2]);
    assert_eq!(c.as_slice(), &[11.0, 22.0, 33.0, 44.0]);
}

#[test]
fn broadcast_scalar_to_matrix() {
    let scalar = Tensor::scalar(2.0);
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let r = &scalar + &m;
    assert_eq!(r.shape(), &[2, 2]);
    assert_eq!(r.as_slice(), &[3.0, 4.0, 5.0, 6.0]);
}

#[test]
fn broadcast_vector_to_matrix() {
    // [4] + [3, 4] -> [3, 4]
    let row = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[4]);
    let mat = Tensor::new(
        vec![
            10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0, 110.0, 120.0,
        ],
        &[3, 4],
    );
    let r = &mat + &row;
    assert_eq!(r.shape(), &[3, 4]);
    assert_eq!(r.as_slice()[0], 11.0);
    assert_eq!(r.as_slice()[4], 51.0);
    assert_eq!(r.as_slice()[8], 91.0);
}

#[test]
fn broadcast_column_and_row() {
    // [3, 1] + [1, 4] -> [3, 4]
    let col = Tensor::new(vec![1.0, 2.0, 3.0], &[3, 1]);
    let row = Tensor::new(vec![10.0, 20.0, 30.0, 40.0], &[1, 4]);
    let r = &col + &row;
    assert_eq!(r.shape(), &[3, 4]);
    assert_eq!(
        r.as_slice(),
        &[
            11.0, 21.0, 31.0, 41.0, 12.0, 22.0, 32.0, 42.0, 13.0, 23.0, 33.0, 43.0
        ]
    );
}

#[test]
#[should_panic(expected = "matten broadcast error in add")]
fn broadcast_incompatible_panics() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let b = Tensor::new(vec![1.0, 2.0], &[2]);
    let _ = &a + &b;
}

#[test]
fn element_wise_sub_mul_div() {
    let a = Tensor::new(vec![10.0, 8.0, 6.0, 4.0], &[2, 2]);
    let b = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let sub = &a - &b;
    assert_eq!(sub.as_slice(), &[9.0, 6.0, 3.0, 0.0]);
    let mul = &a * &b;
    assert_eq!(mul.as_slice(), &[10.0, 16.0, 18.0, 16.0]);
    let div = &a / &b;
    assert_eq!(div.as_slice(), &[10.0, 4.0, 2.0, 1.0]);
}

#[test]
fn neg_unary() {
    let t = Tensor::new(vec![1.0, -2.0, 0.0], &[3]);
    let r = -&t;
    assert_eq!(r.as_slice(), &[-1.0, 2.0, 0.0]);
}

#[test]
fn division_by_zero_is_inf() {
    let a = Tensor::new(vec![1.0, 0.0], &[2]);
    let b = Tensor::new(vec![0.0, 0.0], &[2]);
    let r = &a / &b;
    assert!(r.as_slice()[0].is_infinite());
    assert!(r.as_slice()[1].is_nan());
}

// ---- scalar ops (M3) ---------------------------------------------------

#[test]
fn scalar_ops_tensor_on_left() {
    let t = Tensor::new(vec![2.0, 4.0, 6.0], &[3]);
    assert_eq!((&t + 1.0).as_slice(), &[3.0, 5.0, 7.0]);
    assert_eq!((&t - 1.0).as_slice(), &[1.0, 3.0, 5.0]);
    assert_eq!((&t * 2.0).as_slice(), &[4.0, 8.0, 12.0]);
    assert_eq!((&t / 2.0).as_slice(), &[1.0, 2.0, 3.0]);
}

#[test]
fn scalar_ops_scalar_on_left() {
    let t = Tensor::new(vec![1.0, 2.0, 4.0], &[3]);
    assert_eq!((10.0_f64 + &t).as_slice(), &[11.0, 12.0, 14.0]);
    assert_eq!((10.0_f64 - &t).as_slice(), &[9.0, 8.0, 6.0]);
    assert_eq!((3.0_f64 * &t).as_slice(), &[3.0, 6.0, 12.0]);
    assert_eq!((12.0_f64 / &t).as_slice(), &[12.0, 6.0, 3.0]);
}

#[test]
fn star_is_element_wise_not_matmul() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let b = Tensor::new(vec![5.0, 6.0, 7.0, 8.0], &[2, 2]);
    // element-wise, not matrix product
    let r = &a * &b;
    assert_eq!(r.as_slice(), &[5.0, 12.0, 21.0, 32.0]);
}

// ---- broadcast shape helper (internal) ---------------------------------

#[test]
fn broadcast_shape_cases() {
    use crate::ops::broadcast::broadcast_shape;
    assert_eq!(broadcast_shape(&[], &[2, 3]).unwrap(), vec![2, 3]);
    assert_eq!(broadcast_shape(&[4], &[3, 4]).unwrap(), vec![3, 4]);
    assert_eq!(broadcast_shape(&[3, 1], &[1, 4]).unwrap(), vec![3, 4]);
    assert_eq!(broadcast_shape(&[2, 3], &[2, 3]).unwrap(), vec![2, 3]);
    assert!(broadcast_shape(&[2, 3], &[2]).is_err()); // incompatible
    assert!(broadcast_shape(&[3], &[4]).is_err());
}

// ---- try_add / try_sub / try_mul / try_div (RFC-129) --------------------

#[test]
fn try_ops_match_operator_for_ordinary_shapes() {
    let a = Tensor::new(vec![10.0, 8.0, 6.0, 4.0], &[2, 2]);
    let b = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);

    assert_eq!((&a + &b).as_slice(), a.try_add(&b).unwrap().as_slice());
    assert_eq!((&a - &b).as_slice(), a.try_sub(&b).unwrap().as_slice());
    assert_eq!((&a * &b).as_slice(), a.try_mul(&b).unwrap().as_slice());
    assert_eq!((&a / &b).as_slice(), a.try_div(&b).unwrap().as_slice());
}

#[test]
fn try_add_large_in_memory_pair_succeeds() {
    // A [2000, 1000] pair (2M elements) exceeds the default max_elements
    // (~1M), but arithmetic on already-in-memory, already-validated data is
    // not a boundary the RFC-132 limit model bounds — this must succeed.
    let n = 2000 * 1000;
    assert!(n > crate::MattenLimits::default().max_elements);
    let a = Tensor::new(vec![1.0; n], &[2000, 1000]);
    let b = Tensor::new(vec![2.0; n], &[2000, 1000]);
    let r = a.try_add(&b).unwrap();
    assert_eq!(r.shape(), &[2000, 1000]);
    assert_eq!(r.as_slice()[0], 3.0);
}

#[test]
fn try_add_genuine_broadcast_expansion_still_guarded() {
    // The escalation's exact reproduction (RFC-132 §12.0): a column vector
    // broadcast against a row vector produces a result whose size is the
    // PRODUCT of the two operands, not their sum or either one alone. Each
    // operand is individually small (well within max_elements); only the
    // broadcast result is enormous. Before RFC-132's correction this guard
    // would have been removed entirely, turning a catchable error into an
    // uncatchable allocator abort. It must stay a catchable `Err`.
    let big = MattenLimits {
        max_elements: 2_000_000,
        ..MattenLimits::default()
    };
    let a = Tensor::try_zeros_with_limits(&[1_048_576, 1], &big).expect("a is within budget");
    let b = Tensor::try_zeros_with_limits(&[1, 1_048_576], &big).expect("b is within budget");
    let err = a.try_add(&b).unwrap_err();
    assert!(matches!(err, crate::MattenError::Allocation { .. }));
}

#[test]
fn try_add_broadcast_incompatible_returns_err_not_panic() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let b = Tensor::new(vec![1.0, 2.0], &[2]);
    let err = a.try_add(&b).unwrap_err();
    assert!(matches!(err, crate::MattenError::Broadcast { .. }));
}

#[cfg(feature = "dynamic")]
#[test]
fn try_add_dynamic_returns_unsupported_not_panic() {
    use crate::dynamic::Element;
    let dynamic = Tensor::from_elements(vec![Element::Int(1), Element::Int(2)], &[2]);
    let numeric = Tensor::from_vec(vec![1.0, 2.0]);
    let err = dynamic.try_add(&numeric).unwrap_err();
    assert!(matches!(
        err,
        crate::MattenError::Unsupported {
            operation: "add",
            ..
        }
    ));
}

#[cfg(feature = "dynamic")]
#[test]
fn add_panics_with_byte_identical_dynamic_message() {
    // Pins the pre-RFC-129 message text exactly (R1/T5): a dynamic operand
    // must still panic with this message, unchanged by the try_add refactor.
    use crate::dynamic::Element;
    let dynamic = Tensor::from_elements(vec![Element::Int(1), Element::Int(2)], &[2]);
    let numeric = Tensor::from_vec(vec![1.0, 2.0]);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| &dynamic + &numeric));
    let err = result.unwrap_err();
    let message = err
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| err.downcast_ref::<&str>().map(|s| s.to_string()))
        .expect("panic payload should be a string");
    assert_eq!(
        message,
        "matten unsupported error in add: element-wise arithmetic is not supported on dynamic \
         tensors; call try_numeric() on each operand first"
    );
}
