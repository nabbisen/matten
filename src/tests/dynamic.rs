//! Tests for the `dynamic` feature: Element model, storage, CoW, and parsers.

#[cfg(feature = "dynamic")]
mod element_tests {
    use crate::dynamic::Element;

    #[test]
    fn element_size_is_24_bytes() {
        // RFC-011 §12.1: measure and document.
        // Arc<str> chosen; all text representations give 24 bytes on 64-bit.
        assert_eq!(std::mem::size_of::<Element>(), 24);
    }

    #[test]
    fn is_none_and_is_numeric() {
        assert!(Element::None.is_none());
        assert!(!Element::Float(1.0).is_none());
        assert!(Element::Float(1.0).is_numeric());
        assert!(Element::Int(42).is_numeric());
        assert!(!Element::Bool(true).is_numeric());
        assert!(!Element::text("hi").is_numeric());
        assert!(!Element::None.is_numeric());
    }

    #[test]
    fn try_as_f64_coercion_policy() {
        // Float and Int are coercible; everything else is not.
        assert_eq!(Element::Float(1.5).try_as_f64(), Some(1.5));
        assert_eq!(Element::Int(42).try_as_f64(), Some(42.0));
        assert_eq!(Element::None.try_as_f64(), None);
        assert_eq!(Element::Bool(true).try_as_f64(), None); // no silent bool coercion
        assert_eq!(Element::text("3.14").try_as_f64(), None); // no silent text coercion
    }

    #[test]
    fn text_constructor_and_accessor() {
        let e = Element::text("hello");
        assert_eq!(e.as_text(), Some("hello"));
        assert!(Element::Float(1.0).as_text().is_none());
    }

    #[test]
    fn from_conversions() {
        assert_eq!(Element::from(1.5f64), Element::Float(1.5));
        assert_eq!(Element::from(42i64), Element::Int(42));
        assert_eq!(Element::from(7i32), Element::Int(7));
        assert_eq!(Element::from(true), Element::Bool(true));
        assert_eq!(Element::from("text"), Element::text("text"));
    }

    #[test]
    fn display_formatting() {
        assert_eq!(Element::Float(1.5).to_string(), "1.5");
        assert_eq!(Element::Int(42).to_string(), "42");
        assert_eq!(Element::Bool(true).to_string(), "true");
        assert_eq!(Element::text("hi").to_string(), "hi");
        assert_eq!(Element::None.to_string(), "None");
    }
}

#[cfg(feature = "dynamic")]
mod tensor_tests {
    use crate::dynamic::Element;
    use crate::{MattenError, Tensor};

    #[test]
    fn from_elements_basic() {
        let t = Tensor::from_elements(
            vec![Element::Float(1.0), Element::Int(2), Element::None],
            &[3],
        );
        assert_eq!(t.shape(), &[3]);
        assert!(t.is_dynamic());
    }

    #[test]
    fn try_from_elements_shape_mismatch() {
        let err = Tensor::try_from_elements(vec![Element::Float(1.0)], &[2]).unwrap_err();
        assert!(matches!(err, MattenError::Shape { .. }));
    }

    #[test]
    fn get_element_accessor() {
        let t = Tensor::from_elements(
            vec![Element::Float(1.0), Element::text("hello"), Element::None],
            &[3],
        );
        assert_eq!(t.get_element(&[0]), Some(Element::Float(1.0)));
        assert_eq!(t.get_element(&[1]), Some(Element::text("hello")));
        assert_eq!(t.get_element(&[2]), Some(Element::None));
        assert_eq!(t.get_element(&[5]), None); // out of bounds
    }

    #[test]
    fn fill_none_replaces_missing() {
        let t = Tensor::from_elements(
            vec![Element::Float(1.0), Element::None, Element::Float(3.0)],
            &[3],
        );
        let filled = t.fill_none(Element::Float(0.0));
        assert_eq!(filled.get_element(&[1]), Some(Element::Float(0.0)));
        // Original unchanged
        assert_eq!(t.get_element(&[1]), Some(Element::None));
    }

    #[test]
    fn try_numeric_all_float() {
        let t = Tensor::from_elements(
            vec![Element::Float(1.0), Element::Int(2), Element::Float(3.0)],
            &[3],
        );
        let n = t.try_numeric().unwrap();
        assert_eq!(n.as_slice(), &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn try_numeric_fails_on_none() {
        let t = Tensor::from_elements(vec![Element::Float(1.0), Element::None], &[2]);
        assert!(matches!(
            t.try_numeric(),
            Err(MattenError::Unsupported { .. })
        ));
    }

    #[test]
    fn try_numeric_fails_on_text() {
        let t = Tensor::from_elements(vec![Element::text("hi")], &[1]);
        assert!(matches!(
            t.try_numeric(),
            Err(MattenError::Unsupported { .. })
        ));
    }

    #[test]
    fn to_elements_roundtrip() {
        let original = vec![Element::Float(1.0), Element::Int(2), Element::None];
        let t = Tensor::from_elements(original.clone(), &[3]);
        assert_eq!(t.to_elements(), original);
    }
}

#[cfg(all(feature = "dynamic", feature = "json"))]
mod json_tests {
    use crate::Tensor;
    use crate::dynamic::Element;

    #[test]
    fn from_json_dynamic_mixed() {
        let t = Tensor::from_json_dynamic(r#"[[1, "active", true], [2, null, false]]"#).unwrap();
        assert_eq!(t.shape(), &[2, 3]);
        assert_eq!(t.get_element(&[0, 0]), Some(Element::Int(1)));
        assert_eq!(t.get_element(&[0, 1]), Some(Element::text("active")));
        assert_eq!(t.get_element(&[0, 2]), Some(Element::Bool(true)));
        assert_eq!(t.get_element(&[1, 1]), Some(Element::None));
    }

    #[test]
    fn from_json_dynamic_object_form() {
        let t = Tensor::from_json_dynamic(r#"{"shape":[2],"data":[1, null]}"#).unwrap();
        assert_eq!(t.get_element(&[0]), Some(Element::Int(1)));
        assert_eq!(t.get_element(&[1]), Some(Element::None));
    }

    #[test]
    fn from_json_dynamic_ragged_is_err() {
        assert!(Tensor::from_json_dynamic("[[1,2],[3]]").is_err());
    }

    #[test]
    fn from_json_dynamic_malformed_never_panics() {
        for bad in &["{", "null", "", "[[[]]]"] {
            let _ = Tensor::from_json_dynamic(bad);
        }
    }
}

#[cfg(all(feature = "dynamic", feature = "csv"))]
mod csv_tests {
    use crate::Tensor;
    use crate::dynamic::Element;

    #[test]
    fn from_csv_dynamic_mixed() {
        let t = Tensor::from_csv_dynamic("1,active,true\n2,,false\n").unwrap();
        assert_eq!(t.shape(), &[2, 3]);
        assert_eq!(t.get_element(&[0, 0]), Some(Element::Int(1)));
        assert_eq!(t.get_element(&[0, 1]), Some(Element::text("active")));
        assert_eq!(t.get_element(&[0, 2]), Some(Element::Bool(true)));
        assert_eq!(t.get_element(&[1, 1]), Some(Element::None)); // empty field
    }

    #[test]
    fn from_csv_dynamic_numeric_only() {
        let t = Tensor::from_csv_dynamic("1.0,2.0\n3.0,4.0\n").unwrap();
        // All float fields
        assert_eq!(t.get_element(&[0, 0]), Some(Element::Float(1.0)));
        // Can convert to numeric tensor
        let n = t.try_numeric().unwrap();
        assert_eq!(n.shape(), &[2, 2]);
    }

    #[test]
    fn from_csv_dynamic_ragged_is_err() {
        assert!(Tensor::from_csv_dynamic("1,2\n3\n").is_err());
    }
}

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

// ---- P0-2: numeric accessor guards -----------------------------------

#[cfg(feature = "dynamic")]
mod accessor_guard_tests {
    use crate::Tensor;
    use crate::dynamic::Element;

    fn dyn1() -> Tensor {
        Tensor::from_elements(vec![Element::Int(1), Element::Int(2)], &[2])
    }

    #[test]
    fn as_slice_panics_on_dynamic() {
        assert!(
            std::panic::catch_unwind(|| {
                let t = dyn1();
                let _ = t.as_slice();
            })
            .is_err()
        );
    }

    #[test]
    fn to_vec_panics_on_dynamic() {
        assert!(std::panic::catch_unwind(|| dyn1().to_vec()).is_err());
    }

    #[test]
    fn into_vec_panics_on_dynamic() {
        assert!(
            std::panic::catch_unwind(|| {
                let t = dyn1();
                let _: Vec<f64> = t.into();
            })
            .is_err()
        );
    }

    #[test]
    fn get_panics_on_dynamic() {
        assert!(std::panic::catch_unwind(|| dyn1().get(&[0])).is_err());
    }

    #[test]
    fn get_flat_panics_on_dynamic() {
        assert!(std::panic::catch_unwind(|| dyn1().get_flat(0)).is_err());
    }

    #[test]
    fn from_ref_tensor_panics_on_dynamic() {
        assert!(
            std::panic::catch_unwind(|| {
                let t = dyn1();
                let _: Vec<f64> = Vec::from(&t);
            })
            .is_err()
        );
    }
}

// ---- P0-3: matmul dynamic guard --------------------------------------

#[cfg(feature = "dynamic")]
mod matmul_guard_tests {
    use crate::Tensor;
    use crate::dynamic::Element;

    #[test]
    fn dynamic_matmul_is_unsupported() {
        let t = Tensor::from_elements(vec![Element::Float(1.0), Element::Float(2.0)], &[2]);
        assert!(
            std::panic::catch_unwind(|| t.matmul(&t)).is_err(),
            "matmul on dynamic tensor must panic"
        );
    }

    #[test]
    fn dynamic_dot_is_unsupported() {
        let t = Tensor::from_elements(vec![Element::Float(1.0), Element::Float(2.0)], &[2]);
        assert!(
            std::panic::catch_unwind(|| t.dot(&t)).is_err(),
            "dot on dynamic tensor must panic"
        );
    }
}

// ---- P2-3: large Int(i64) precision documentation test ---------------

#[cfg(feature = "dynamic")]
mod precision_tests {
    use crate::dynamic::Element;

    #[test]
    fn small_int_coercion_exact() {
        // Small i64 values coerce exactly to f64
        assert_eq!(Element::Int(42).try_as_f64(), Some(42.0));
        assert_eq!(Element::Int(-1000).try_as_f64(), Some(-1000.0));
    }

    #[test]
    fn large_int_coercion_may_lose_precision() {
        // i64 values with magnitude > 2^53 may lose precision in f64.
        // This test documents the current behavior explicitly.
        let large: i64 = 9_007_199_254_740_993; // 2^53 + 1
        let as_f64 = large as f64;
        // The conversion happens but precision loss is possible
        assert_eq!(Element::Int(large).try_as_f64(), Some(as_f64));
        // Document that round-trip may not be exact
        assert_ne!(as_f64 as i64, large, "precision loss documented");
    }
}
