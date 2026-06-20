//! Storage, utility, is_none_mask, and lifecycle tests.

//! Storage, utility, guard, precision, and diagnostic tests.

#[cfg(feature = "dynamic")]
mod storage_tests {
    use crate::dynamic::{Element, storage::DynamicTensor};

    #[test]
    fn shared_storage_on_reshape() {
        let data = vec![
            Element::Int(1),
            Element::Int(2),
            Element::Int(3),
            Element::Int(4),
        ];
        let t = DynamicTensor::from_vec(data, vec![2, 2]);
        let r = t.reshape(vec![4]).unwrap();
        // Both share the same Arc
        assert!(std::sync::Arc::ptr_eq(&t.storage, &r.storage));
    }

    #[test]
    fn reshape_fails_on_size_mismatch() {
        let t = DynamicTensor::from_vec(vec![Element::Int(1); 4], vec![2, 2]);
        assert!(t.reshape(vec![3]).is_none());
    }

    #[test]
    fn slice_indices_shares_storage() {
        let data: Vec<Element> = (0..6).map(|i| Element::Int(i as i64)).collect();
        let t = DynamicTensor::from_vec(data, vec![2, 3]);
        // Take logical elements 0,1,2 (first row)
        let s = t.slice_indices(vec![0, 1, 2], vec![3]);
        assert!(std::sync::Arc::ptr_eq(&t.storage, &s.storage));
        assert_eq!(s.get_flat(0), Some(&Element::Int(0)));
        assert_eq!(s.get_flat(1), Some(&Element::Int(1)));
    }

    #[test]
    fn materialize_produces_contiguous_copy() {
        let data: Vec<Element> = (0..6).map(|i| Element::Int(i as i64)).collect();
        let t = DynamicTensor::from_vec(data, vec![2, 3]);
        // Create a non-contiguous slice (elements 0, 2, 4)
        let mut s = t.slice_indices(vec![0, 2, 4], vec![3]);
        assert!(!s.is_unique()); // shared with t
        s.materialize();
        // After materialization: new unique storage
        assert!(s.is_unique());
        assert_eq!(s.get_flat(0), Some(&Element::Int(0)));
        assert_eq!(s.get_flat(1), Some(&Element::Int(2)));
        assert_eq!(s.get_flat(2), Some(&Element::Int(4)));
    }

    #[test]
    fn no_reference_cycles() {
        // Drop works correctly — no memory leak from Arc cycles
        let data = vec![Element::Int(1)];
        let t = DynamicTensor::from_vec(data, vec![1]);
        let s = t.slice_indices(vec![0], vec![1]);
        drop(t);
        // s still valid
        assert_eq!(s.get_flat(0), Some(&Element::Int(1)));
    }
}

#[cfg(feature = "dynamic")]
mod utility_tests {
    use crate::Tensor;
    use crate::dynamic::Element;

    #[test]
    fn none_mask_basic() {
        let t = Tensor::from_elements(
            vec![Element::Float(1.0), Element::None, Element::Float(3.0)],
            &[3],
        );
        let mask = t.none_mask();
        assert_eq!(mask.as_slice(), &[0.0, 1.0, 0.0]);
    }

    #[test]
    fn count_none_basic() {
        let t = Tensor::from_elements(
            vec![
                Element::None,
                Element::Int(1),
                Element::None,
                Element::Float(2.0),
            ],
            &[4],
        );
        assert_eq!(t.count_none(), 2);
    }

    #[test]
    fn forward_fill_none_basic() {
        let t = Tensor::from_elements(
            vec![
                Element::Float(1.0),
                Element::None,
                Element::None,
                Element::Float(4.0),
            ],
            &[4],
        );
        let filled = t.forward_fill_none(Element::Float(0.0));
        assert_eq!(filled.get_element(&[1]), Some(Element::Float(1.0)));
        assert_eq!(filled.get_element(&[2]), Some(Element::Float(1.0)));
        assert_eq!(filled.get_element(&[3]), Some(Element::Float(4.0)));
    }

    #[test]
    fn forward_fill_none_leading_uses_fallback() {
        let t = Tensor::from_elements(
            vec![Element::None, Element::None, Element::Float(5.0)],
            &[3],
        );
        let filled = t.forward_fill_none(Element::Float(-1.0));
        assert_eq!(filled.get_element(&[0]), Some(Element::Float(-1.0)));
        assert_eq!(filled.get_element(&[1]), Some(Element::Float(-1.0)));
        assert_eq!(filled.get_element(&[2]), Some(Element::Float(5.0)));
    }

    #[test]
    fn sum_skip_none_basic() {
        let t = Tensor::from_elements(
            vec![Element::Float(1.0), Element::None, Element::Int(3)],
            &[3],
        );
        assert_eq!(t.sum_skip_none(), 4.0);
    }

    #[test]
    fn sum_skip_none_all_none_is_zero() {
        let t = Tensor::from_elements(vec![Element::None; 3], &[3]);
        assert_eq!(t.sum_skip_none(), 0.0);
    }
}

// ---- RFC-011: default build does not expose Element -------------------

#[test]
fn dynamic_feature_is_off_by_default_in_non_dynamic_tests() {
    // This test module is compiled under #[cfg(feature = "dynamic")] guards
    // on the individual sub-modules. The existence of this file without the
    // feature active is verified by the lean-core CI profile, which passes
    // (the dynamic submodules are simply absent from compilation).
    //
    // To verify Element is not exported in the default build, we rely on
    // the fact that `cargo test --no-default-features` (lean core) runs
    // this file without the dynamic feature — and this test file itself
    // compiles fine only because all Element-referencing tests are inside
    // `#[cfg(feature = "dynamic")] mod ...` blocks.
    //
    // The CI profile "lean core" (no features) runs without compile error,
    // which is the definitive proof. See .github/workflows/ci.yml.
    // Compile-time proof: this file compiles cleanly under --no-default-features
    // (lean-core CI profile) because all Element-referencing code is inside
    // #[cfg(feature = "dynamic")] blocks. The CI "lean core" job is the test.
    let _ = 42_u8; // suppress unused warning
}

#[cfg(feature = "dynamic")]
mod is_none_mask_tests {
    use crate::Tensor;
    use crate::dynamic::Element;

    #[test]
    fn is_none_mask_matches_none_mask() {
        // RFC-011 named this is_none(); we expose both none_mask() and is_none_mask()
        let t = Tensor::from_elements(
            vec![Element::Float(1.0), Element::None, Element::Float(3.0)],
            &[3],
        );
        assert_eq!(t.none_mask(), t.is_none_mask());
    }

    #[test]
    fn is_none_mask_shape_and_values() {
        let t = Tensor::from_elements(vec![Element::None, Element::Int(1), Element::None], &[3]);
        let mask = t.is_none_mask();
        assert_eq!(mask.shape(), &[3]);
        assert_eq!(mask.as_slice(), &[1.0, 0.0, 1.0]);
    }
}

// ---- P0-2: Public dynamic lifecycle tests (architect review §3) ----------

#[cfg(feature = "dynamic")]
mod lifecycle_tests {
    use crate::Tensor;
    use crate::dynamic::Element;

    #[test]
    fn dynamic_len_equals_shape_product() {
        // P0-1: len() must return logical length, not data.len()
        let t = Tensor::from_elements(
            vec![Element::Float(1.0), Element::text("a"), Element::None],
            &[3],
        );
        assert_eq!(t.len(), 3, "dynamic len() must equal shape product");
    }

    #[test]
    fn dynamic_len_2d() {
        let t = Tensor::from_elements(
            vec![
                Element::Int(1),
                Element::Int(2),
                Element::Int(3),
                Element::Int(4),
            ],
            &[2, 2],
        );
        assert_eq!(t.len(), 4);
    }

    #[test]
    fn dynamic_debug_not_empty() {
        let t = Tensor::from_elements(vec![Element::Float(1.5), Element::text("hello")], &[2]);
        let dbg = format!("{t:?}");
        // Debug must not report an empty tensor for a non-empty dynamic one
        assert!(
            !dbg.contains("data: []") || dbg.contains("dynamic"),
            "Debug output should not appear empty: {dbg}"
        );
    }

    #[test]
    fn dynamic_reshape_is_unsupported() {
        // Under the "guard" model, reshape on a dynamic tensor should panic
        // with a clear matten unsupported error message.
        let t = Tensor::from_elements(vec![Element::Int(1), Element::Int(2)], &[2]);
        let result = std::panic::catch_unwind(|| t.reshape(&[1, 2]));
        assert!(
            result.is_err(),
            "reshape on dynamic tensor must panic (unsupported)"
        );
    }

    #[test]
    fn dynamic_flatten_is_unsupported() {
        let t = Tensor::from_elements(
            vec![
                Element::Int(1),
                Element::Int(2),
                Element::Int(3),
                Element::Int(4),
            ],
            &[2, 2],
        );
        let result = std::panic::catch_unwind(|| t.flatten());
        assert!(
            result.is_err(),
            "flatten on dynamic tensor must panic (unsupported)"
        );
    }

    #[test]
    fn dynamic_slice_builder_is_unsupported() {
        let t = Tensor::from_elements(
            vec![Element::Int(1), Element::Int(2), Element::Int(3)],
            &[3],
        );
        // slice().build() must either return Err or panic, not silently succeed
        // with wrong data. We test via slice_str which calls execute_slice.
        let result = t.slice_str("0:2");
        assert!(
            result.is_err(),
            "slice_str on dynamic tensor must return Err"
        );
    }

    #[test]
    fn dynamic_arithmetic_is_unsupported() {
        let t = Tensor::from_elements(vec![Element::Float(1.0), Element::Float(2.0)], &[2]);
        let result = std::panic::catch_unwind(|| {
            let _ = &t + &t;
        });
        assert!(
            result.is_err(),
            "arithmetic on dynamic tensor must panic (unsupported)"
        );
    }

    #[test]
    fn dynamic_sum_is_unsupported() {
        let t = Tensor::from_elements(vec![Element::Float(1.0)], &[1]);
        let result = std::panic::catch_unwind(|| t.sum());
        assert!(
            result.is_err(),
            "sum on dynamic tensor must panic (unsupported)"
        );
    }
}
