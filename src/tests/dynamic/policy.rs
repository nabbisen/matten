//! NumericPolicy and inspection helper tests.

#[cfg(feature = "dynamic")]
mod numeric_policy_tests {
    use crate::dynamic::{Element, NumericPolicy};
    use crate::{MattenError, Tensor};

    fn mixed() -> Tensor {
        Tensor::from_elements(
            vec![
                Element::Float(1.5),
                Element::Int(2),
                Element::Bool(true),
                Element::text("3.0"),
                Element::None,
            ],
            &[5],
        )
    }

    #[test]
    fn strict_rejects_bool() {
        let t = Tensor::from_elements(vec![Element::Bool(true)], &[1]);
        assert!(matches!(
            t.try_numeric_with(NumericPolicy::strict()),
            Err(MattenError::Unsupported { .. })
        ));
    }

    #[test]
    fn strict_rejects_text() {
        let t = Tensor::from_elements(vec![Element::text("x")], &[1]);
        assert!(matches!(
            t.try_numeric_with(NumericPolicy::strict()),
            Err(MattenError::Unsupported { .. })
        ));
    }

    #[test]
    fn strict_rejects_none() {
        let t = Tensor::from_elements(vec![Element::None], &[1]);
        assert!(matches!(
            t.try_numeric_with(NumericPolicy::strict()),
            Err(MattenError::Unsupported { .. })
        ));
    }

    #[test]
    fn none_as_converts_none() {
        let t = Tensor::from_elements(
            vec![Element::Float(1.0), Element::None, Element::Float(3.0)],
            &[3],
        );
        let x = t
            .try_numeric_with(NumericPolicy::default().none_as(0.0))
            .unwrap();
        assert_eq!(x.as_slice(), &[1.0, 0.0, 3.0]);
    }

    #[test]
    fn allow_bool_converts_true_false() {
        let t = Tensor::from_elements(vec![Element::Bool(true), Element::Bool(false)], &[2]);
        let x = t
            .try_numeric_with(NumericPolicy::default().allow_bool())
            .unwrap();
        assert_eq!(x.as_slice(), &[1.0, 0.0]);
    }

    #[test]
    fn allow_text_parse_converts_numeric_strings() {
        let t = Tensor::from_elements(vec![Element::text("42.5")], &[1]);
        let x = t
            .try_numeric_with(NumericPolicy::default().allow_text_parse())
            .unwrap();
        assert!((x.as_slice()[0] - 42.5_f64).abs() < 1e-10);
    }

    #[test]
    fn allow_text_parse_rejects_non_numeric_strings() {
        let t = Tensor::from_elements(vec![Element::text("hello")], &[1]);
        assert!(
            t.try_numeric_with(NumericPolicy::default().allow_text_parse())
                .is_err()
        );
    }

    #[test]
    fn none_as_nan_produces_nan() {
        let t = Tensor::from_elements(vec![Element::Float(1.0), Element::None], &[2]);
        let x = t
            .try_numeric_with(NumericPolicy::default().none_as_nan())
            .unwrap();
        assert_eq!(x.as_slice()[0], 1.0);
        assert!(x.as_slice()[1].is_nan());
    }

    #[test]
    fn permissive_converts_all_variants() {
        let x = mixed()
            .try_numeric_with(NumericPolicy::permissive())
            .unwrap();
        assert_eq!(x.shape(), &[5]);
        assert_eq!(x.as_slice()[0], 1.5); // Float
        assert_eq!(x.as_slice()[1], 2.0); // Int
        assert_eq!(x.as_slice()[2], 1.0); // Bool(true)
        assert_eq!(x.as_slice()[3], 3.0); // Text("3.0")
        assert_eq!(x.as_slice()[4], 0.0); // None → 0.0
    }
}

// ---- RFC-016: dynamic inspection helpers ---------------------------------

#[cfg(feature = "dynamic")]
mod inspection_tests {
    use crate::Tensor;
    use crate::dynamic::Element;

    fn mixed() -> Tensor {
        Tensor::from_elements(
            vec![
                Element::Float(1.0),
                Element::Int(2),
                Element::text("x"),
                Element::None,
            ],
            &[4],
        )
    }

    #[test]
    fn numeric_mask_correct_values() {
        let mask = mixed().numeric_mask();
        assert_eq!(mask.shape(), &[4]);
        assert_eq!(mask.as_slice(), &[1.0, 1.0, 0.0, 0.0]);
    }

    #[test]
    fn numeric_mask_mirrors_none_mask_for_none_only() {
        let t = Tensor::from_elements(vec![Element::Float(1.0), Element::None], &[2]);
        let nm = t.none_mask();
        let numm = t.numeric_mask();
        // numeric_mask is 1 where none_mask is 0, for Float/Int
        assert_eq!(nm.as_slice(), &[0.0, 1.0]);
        assert_eq!(numm.as_slice(), &[1.0, 0.0]);
    }

    #[test]
    fn is_numeric_convertible_true_for_float_int_only() {
        let t = Tensor::from_elements(vec![Element::Float(1.0), Element::Int(2)], &[2]);
        assert!(t.is_numeric_convertible());
    }

    #[test]
    fn is_numeric_convertible_false_when_none_present() {
        assert!(!mixed().is_numeric_convertible());
    }

    #[test]
    fn schema_summary_contains_counts() {
        let s = mixed().schema_summary();
        assert!(s.contains("Float: 1"), "summary: {s}");
        assert!(s.contains("Int: 1"), "summary: {s}");
        assert!(s.contains("Text: 1"), "summary: {s}");
        assert!(s.contains("None: 1"), "summary: {s}");
    }

    #[test]
    fn schema_summary_shape_included() {
        let s = mixed().schema_summary();
        assert!(s.contains("shape=[4]"), "summary: {s}");
    }
}
