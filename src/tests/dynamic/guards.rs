//! Guard and diagnostic tests for dynamic tensor accessor protection.

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

// ---- PR-4: additional dynamic accessor regression tests -----------------

#[cfg(feature = "dynamic")]
mod additional_accessor_tests {
    use crate::dynamic::Element;
    use crate::{MattenError, Tensor};

    #[test]
    fn into_vec_method_panics_on_dynamic() {
        // Tensor::into_vec() on a dynamic tensor must panic, not return vec![]
        let t = Tensor::from_elements(vec![Element::Int(1), Element::Int(2)], &[2]);
        let result = std::panic::catch_unwind(move || {
            let _: Vec<f64> = t.into_vec();
        });
        assert!(result.is_err(), "into_vec() on dynamic tensor must panic");
    }

    #[test]
    fn try_into_rows_returns_unsupported_on_dynamic() {
        // TryFrom<Tensor> for Vec<Vec<f64>> must return Err, not produce empty rows
        let t = Tensor::from_elements(
            vec![
                Element::Float(1.0),
                Element::Float(2.0),
                Element::Float(3.0),
                Element::Float(4.0),
            ],
            &[2, 2],
        );
        let result: Result<Vec<Vec<f64>>, MattenError> = t.try_into();
        assert!(
            matches!(result, Err(MattenError::Unsupported { .. })),
            "TryFrom<Tensor> for Vec<Vec<f64>> must return Unsupported for dynamic tensors"
        );
    }
}

// ---- RFC-020: diagnostic message quality tests --------------------------

#[cfg(feature = "dynamic")]
mod diagnostic_message_tests {
    use crate::Tensor;
    use crate::dynamic::Element;

    /// Numeric accessor guard messages follow the standard format:
    /// "matten unsupported error in <op>: ..."
    #[test]
    fn as_slice_message_format() {
        let t = Tensor::from_elements(vec![Element::Int(1)], &[1]);
        let result = std::panic::catch_unwind(|| {
            let _ = t.as_slice();
        });
        let msg = result
            .unwrap_err()
            .downcast::<String>()
            .map(|s| s.to_string())
            .or_else(|e| e.downcast::<&str>().map(|s| s.to_string()))
            .unwrap_or_default();
        assert!(
            msg.starts_with("matten unsupported error in as_slice:"),
            "expected 'matten unsupported error in as_slice:', got: {msg}"
        );
    }

    /// sum_skip_none with a non-numeric element produces a conforming panic message.
    #[test]
    fn sum_skip_none_message_format() {
        let t = Tensor::from_elements(vec![Element::Float(1.0), Element::text("x")], &[2]);
        let result = std::panic::catch_unwind(|| {
            let _ = t.sum_skip_none();
        });
        let msg = result
            .unwrap_err()
            .downcast::<String>()
            .map(|s| s.to_string())
            .or_else(|e| e.downcast::<&str>().map(|s| s.to_string()))
            .unwrap_or_default();
        assert!(
            msg.starts_with("matten unsupported error in sum_skip_none:"),
            "expected 'matten unsupported error in sum_skip_none:', got: {msg}"
        );
    }
}

// ---- RFC-017: NumericPolicy and try_numeric_with -------------------------
