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
