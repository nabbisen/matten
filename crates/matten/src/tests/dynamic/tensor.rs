//! Tensor construction, JSON, and CSV tests for dynamic.

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

    // RFC-031: is_dynamic() must return true for dynamic tensors and false for
    // numeric tensors, regardless of feature configuration.
    #[test]
    fn is_dynamic_distinguishes_storage_kind() {
        let dynamic = Tensor::from_elements(vec![Element::Float(1.0), Element::Float(2.0)], &[2]);
        assert!(
            dynamic.is_dynamic(),
            "dynamic tensor must report is_dynamic() = true"
        );

        let numeric = Tensor::new(vec![1.0, 2.0], &[2]);
        assert!(
            !numeric.is_dynamic(),
            "numeric tensor must report is_dynamic() = false"
        );
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
