#[cfg(feature = "dynamic")]
use crate::{MattenError, Tensor};

// ── dynamic rejection (RFC-055/056) ───────────────────────────────────────

#[cfg(feature = "dynamic")]
#[test]
fn reductions_reject_dynamic_with_unsupported() {
    use crate::dynamic::Element;
    let d = Tensor::from_elements(
        vec![
            Element::Float(1.0),
            Element::Float(2.0),
            Element::Float(3.0),
            Element::Float(4.0),
        ],
        &[2, 2],
    );
    assert!(d.is_dynamic());
    assert!(matches!(
        d.try_sum().unwrap_err(),
        MattenError::Unsupported {
            operation: "sum",
            ..
        }
    ));
    assert!(matches!(
        d.try_mean().unwrap_err(),
        MattenError::Unsupported {
            operation: "mean",
            ..
        }
    ));
    assert!(matches!(
        d.try_min().unwrap_err(),
        MattenError::Unsupported {
            operation: "min",
            ..
        }
    ));
    assert!(matches!(
        d.try_max().unwrap_err(),
        MattenError::Unsupported {
            operation: "max",
            ..
        }
    ));
    assert!(matches!(
        d.try_sum_axis(0).unwrap_err(),
        MattenError::Unsupported {
            operation: "sum_axis",
            ..
        }
    ));
    assert!(matches!(
        d.try_mean_axis(0).unwrap_err(),
        MattenError::Unsupported {
            operation: "mean_axis",
            ..
        }
    ));
    assert!(matches!(
        d.try_min_axis(0).unwrap_err(),
        MattenError::Unsupported {
            operation: "min_axis",
            ..
        }
    ));
    assert!(matches!(
        d.try_max_axis(0).unwrap_err(),
        MattenError::Unsupported {
            operation: "max_axis",
            ..
        }
    ));
    // Precedence: a dynamic tensor with an out-of-range axis still reports the
    // dynamic error first (mirrors try_var_axis / try_std_axis).
    assert!(matches!(
        d.try_sum_axis(99).unwrap_err(),
        MattenError::Unsupported {
            operation: "sum_axis",
            ..
        }
    ));
}

#[cfg(feature = "dynamic")]
#[test]
#[should_panic(expected = "dynamic")]
fn sum_panics_on_dynamic() {
    use crate::dynamic::Element;
    let d = Tensor::from_elements(vec![Element::Float(1.0), Element::Float(2.0)], &[2]);
    let _ = d.sum();
}

#[cfg(feature = "dynamic")]
#[test]
#[should_panic(expected = "out of range")]
fn sum_axis_panics_on_numeric_bad_axis_after_delegation() {
    // Sanity: panic form of an axis reduction still panics on a bad axis for a
    // numeric tensor, via the try_ delegation.
    let _ = Tensor::ones(&[2, 2]).max_axis(7);
}
