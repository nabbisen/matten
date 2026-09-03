use crate::{MattenLimits, Tensor};
use proptest::prelude::*;

// ── dot (vector) ──────────────────────────────────────────────────────────

#[test]
fn vv_dot_basic() {
    let a = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    let b = Tensor::from_vec(vec![4.0, 5.0, 6.0]);
    let d = a.dot(&b);
    assert!(d.is_scalar());
    assert_eq!(d.as_slice(), &[32.0]); // 1*4 + 2*5 + 3*6
}

#[test]
fn vv_dot_orthogonal() {
    let a = Tensor::from_vec(vec![1.0, 0.0, 0.0]);
    let b = Tensor::from_vec(vec![0.0, 1.0, 0.0]);
    assert_eq!(a.dot(&b).as_slice(), &[0.0]);
}

#[test]
#[should_panic(expected = "lengths must match")]
fn vv_dot_length_mismatch_panics() {
    let a = Tensor::from_vec(vec![1.0, 2.0]);
    let b = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    let _ = a.dot(&b);
}

// ── matmul ────────────────────────────────────────────────────────────────

#[test]
fn matrix_vector_mul() {
    // [[1,2,3],[4,5,6]] × [1,0,1] = [4,10]
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let v = Tensor::from_vec(vec![1.0, 0.0, 1.0]);
    let r = m.matmul(&v);
    assert_eq!(r.shape(), &[2]);
    assert_eq!(r.as_slice(), &[4.0, 10.0]);
}

#[test]
fn vector_matrix_mul() {
    // [1,2] × [[1,2,3],[4,5,6]] = [9,12,15]
    let v = Tensor::from_vec(vec![1.0, 2.0]);
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let r = v.matmul(&m);
    assert_eq!(r.shape(), &[3]);
    assert_eq!(r.as_slice(), &[9.0, 12.0, 15.0]);
}

#[test]
fn matrix_matrix_mul() {
    // [[1,2],[3,4]] × [[5,6],[7,8]] = [[19,22],[43,50]]
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let b = Tensor::new(vec![5.0, 6.0, 7.0, 8.0], &[2, 2]);
    let c = a.matmul(&b);
    assert_eq!(c.shape(), &[2, 2]);
    assert_eq!(c.as_slice(), &[19.0, 22.0, 43.0, 50.0]);
}

#[test]
fn matmul_non_square() {
    // [2,3] × [3,4] -> [2,4]
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let b = Tensor::new((1..=12).map(|x| x as f64).collect(), &[3, 4]);
    let c = a.matmul(&b);
    assert_eq!(c.shape(), &[2, 4]);
    // row 0: [1,2,3] × cols = [1*1+2*5+3*9, 1*2+2*6+3*10, …]
    assert_eq!(c.as_slice()[0], 38.0); // 1+10+27
    assert_eq!(c.as_slice()[1], 44.0); // 2+12+30
}

#[test]
fn dot_and_matmul_are_aliases() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let b = Tensor::new(vec![5.0, 6.0, 7.0, 8.0], &[2, 2]);
    assert_eq!(a.dot(&b), a.matmul(&b));
}

#[test]
#[should_panic(expected = "left columns")]
fn matmul_dimension_mismatch_panics() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let b = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0], &[4, 2]);
    let _ = a.matmul(&b);
}

#[test]
#[should_panic(expected = "unsupported rank")]
fn matmul_rank3_panics() {
    let a = Tensor::zeros(&[2, 2, 2]);
    let b = Tensor::zeros(&[2, 2, 2]);
    let _ = a.matmul(&b);
}

#[test]
fn star_is_still_element_wise_not_matmul() {
    // Regression: * must never become matmul
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let b = Tensor::new(vec![5.0, 6.0, 7.0, 8.0], &[2, 2]);
    assert_eq!((&a * &b).as_slice(), &[5.0, 12.0, 21.0, 32.0]); // element-wise
    assert_eq!(a.matmul(&b).as_slice(), &[19.0, 22.0, 43.0, 50.0]); // matrix product
}

// ── zero-column / zero-row matmul (RFC-106 SS2.10, RFC-108) ────────────────
//
// No constructor accepts a zero-sized shape, so every empty operand here is
// reached via slice().range(0..0), exactly as RFC-105/RFC-106's fixtures do.

fn zero_rows(cols: usize) -> Tensor {
    // shape [0, cols]
    Tensor::new(vec![0.0; cols], &[1, cols])
        .slice()
        .range(0..0)
        .all()
        .build()
        .unwrap()
}

fn zero_cols(rows: usize) -> Tensor {
    // shape [rows, 0]
    Tensor::new(vec![0.0; rows], &[rows, 1])
        .slice()
        .all()
        .range(0..0)
        .build()
        .unwrap()
}

#[test]
fn matmul_zero_output_columns_before_fix_would_panic() {
    // [2,3] x [3,0] -> [2,0]. Before RFC-108's fix, `mm_mul`'s
    // `out.chunks_mut(p)` panicked with "chunk size must be non-zero" here --
    // this exact fixture is what demonstrated the defect (RFC-106 SS2.10).
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let b = zero_cols(3);
    assert_eq!(b.shape(), &[3, 0]);

    let r = a.try_dot(&b).unwrap();
    assert_eq!(r.shape(), &[2, 0]);
    assert!(r.as_slice().is_empty());

    assert_eq!(a.try_matmul(&b).unwrap().shape(), &[2, 0]);
}

#[test]
fn matmul_zero_output_columns_panicking_forms_also_return() {
    // T5: the panicking `dot`/`matmul` forms must not panic either.
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let b = zero_cols(3);
    assert_eq!(a.dot(&b).shape(), &[2, 0]);
    assert_eq!(a.matmul(&b).shape(), &[2, 0]);
}

#[test]
fn matmul_zero_contraction_dim_unchanged() {
    // T2: n == 0 already worked before this fix and must stay identical:
    // [2,0] x [0,3] -> [2,3], all zero (sum over zero terms).
    let a = zero_cols(2);
    let b = zero_rows(3);
    assert_eq!(a.shape(), &[2, 0]);
    assert_eq!(b.shape(), &[0, 3]);

    let r = a.try_dot(&b).unwrap();
    assert_eq!(r.shape(), &[2, 3]);
    assert_eq!(r.as_slice(), &[0.0; 6]);
    assert_eq!(a.dot(&b), r);
}

#[test]
fn matmul_zero_rows_unchanged() {
    // T3: m == 0 already worked before this fix and must stay identical:
    // [0,3] x [3,2] -> [0,2].
    let a = zero_rows(3);
    let b = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[3, 2]);
    assert_eq!(a.shape(), &[0, 3]);

    let r = a.try_dot(&b).unwrap();
    assert_eq!(r.shape(), &[0, 2]);
    assert!(r.as_slice().is_empty());
    assert_eq!(a.dot(&b), r);
}

#[test]
fn matmul_both_dims_zero() {
    // T4: [0,3] x [3,0] -> [0,0]. Both m and p zero at once.
    let a = zero_rows(3);
    let b = zero_cols(3);
    assert_eq!(a.shape(), &[0, 3]);
    assert_eq!(b.shape(), &[3, 0]);

    let r = a.try_dot(&b).unwrap();
    assert_eq!(r.shape(), &[0, 0]);
    assert!(r.as_slice().is_empty());
    assert_eq!(a.dot(&b), r);
}

// ── P1: shape/data invariant for matmul (RFC-128) ──────────────────────────
//
// for any tensor produced by ANY public constructor or operation:
//     shape.iter().product() == data.len()
//
// `mm_mul` (math.rs) is one of the two sites RFC-127 actually fixed: two
// individually-cheap-to-construct operands ([m, n] and [n, p] with n kept
// small) can still combine to an m*p output too large for the default
// budget. This reproduces that shape directly rather than trusting the
// fixed guard stays in place.

fn matmul_extreme_dim() -> impl Strategy<Value = usize> {
    prop_oneof![
        6 => 0usize..6,
        1 => Just(2000usize),
        1 => Just(crate::limits::MAX_ELEMENTS + 1),
    ]
}

proptest! {
    #[test]
    fn p1_try_matmul_invariant(
        m in matmul_extreme_dim(),
        n in 0usize..4,
        p in matmul_extreme_dim(),
    ) {
        // A generous per-operand budget: only mm_mul's own hardcoded DEFAULT
        // budget check (not this one) should be able to reject the m*p output.
        let generous = MattenLimits {
            max_elements: usize::MAX / 8,
            ..MattenLimits::default()
        };
        let a = match Tensor::try_zeros_with_limits(&[m, n], &generous) {
            Ok(t) => t,
            Err(_) => return Ok(()), // operand itself unconstructible; nothing to test here
        };
        let b = match Tensor::try_zeros_with_limits(&[n, p], &generous) {
            Ok(t) => t,
            Err(_) => return Ok(()),
        };

        match a.try_matmul(&b) {
            Ok(r) => {
                let expected: usize = r.shape().iter().product();
                prop_assert_eq!(r.as_slice().len(), expected);
                prop_assert_eq!(r.shape(), &[m, p][..]);
            }
            Err(e) => {
                prop_assert!(
                    matches!(e, crate::MattenError::Allocation { .. }),
                    "unexpected error variant: {:?}",
                    e
                );
            }
        }
    }
}
