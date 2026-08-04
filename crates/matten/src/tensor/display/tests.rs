use crate::Tensor;

// ── numeric: rank 0 / 1 / 2 ─────────────────────────────────────────────────

#[test]
fn rank0_renders_the_scalar_with_debug_float_format() {
    let t = Tensor::scalar(3.0);
    assert_eq!(t.to_string(), "3.0");
}

#[test]
fn rank0_non_whole_number() {
    let t = Tensor::scalar(3.5);
    assert_eq!(t.to_string(), "3.5");
}

#[test]
fn rank1_renders_a_right_aligned_row_with_debug_float_format() {
    let t = Tensor::new(vec![1.0, 22.0, 3.0], &[3]);
    assert_eq!(t.to_string(), " 1.0 22.0  3.0");
}

#[test]
fn rank1_whole_numbers_keep_the_decimal_point() {
    // The RFC-096/RFC-100 defect this exists to prevent: "1 2 3" reading as ints.
    let t = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);
    assert_eq!(t.to_string(), "1.0 2.0 3.0");
}

#[test]
fn rank2_square_renders_an_aligned_grid() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    assert_eq!(t.to_string(), "1.0 2.0\n3.0 4.0");
}

#[test]
fn rank2_non_square_renders_per_column_widths() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, -5.0, 6.0], &[2, 3]);
    assert_eq!(t.to_string(), "1.0  2.0 3.0\n4.0 -5.0 6.0");
}

#[test]
fn negative_values_stay_right_aligned_against_positive_ones() {
    let t = Tensor::new(vec![-1.0, 2.0], &[2]);
    assert_eq!(t.to_string(), "-1.0  2.0");
}

// ── truncation (§5.4) ────────────────────────────────────────────────────────

#[test]
fn rank1_beyond_max_preview_values_is_truncated_and_marked() {
    let values: Vec<f64> = (1..=13).map(|x| x as f64).collect();
    let t = Tensor::new(values, &[13]);
    assert_eq!(
        t.to_string(),
        " 1.0  2.0  3.0  4.0  5.0  6.0  7.0  8.0  9.0 10.0 11.0 12.0\n... 1 more values"
    );
}

#[test]
fn rank1_at_exactly_max_preview_values_is_not_truncated() {
    let values: Vec<f64> = (1..=12).map(|x| x as f64).collect();
    let t = Tensor::new(values, &[12]);
    assert!(!t.to_string().contains("more values"));
}

#[test]
fn rank2_beyond_max_display_columns_is_truncated_and_marked() {
    let values: Vec<f64> = (1..=13).map(|x| x as f64).collect();
    let t = Tensor::new(values, &[1, 13]);
    assert_eq!(
        t.to_string(),
        "1.0 2.0 3.0 4.0 5.0 6.0 7.0 8.0 9.0 10.0 11.0 12.0\n... 1 more columns"
    );
}

#[test]
fn rank2_at_exactly_max_display_columns_is_not_truncated() {
    let values: Vec<f64> = (1..=12).map(|x| x as f64).collect();
    let t = Tensor::new(values, &[1, 12]);
    assert!(!t.to_string().contains("more columns"));
}

// ── {:#} — this implementation's decision: alternate means untruncated ─────

#[test]
fn alternate_flag_disables_row_truncation() {
    let values: Vec<f64> = (1..=13).map(|x| x as f64).collect();
    let t = Tensor::new(values, &[13]);
    let out = format!("{t:#}");
    assert!(!out.contains("more values"));
    assert_eq!(
        out,
        " 1.0  2.0  3.0  4.0  5.0  6.0  7.0  8.0  9.0 10.0 11.0 12.0 13.0"
    );
}

#[test]
fn alternate_flag_disables_column_truncation() {
    let values: Vec<f64> = (1..=13).map(|x| x as f64).collect();
    let t = Tensor::new(values, &[1, 13]);
    let out = format!("{t:#}");
    assert!(!out.contains("more columns"));
    assert_eq!(
        out,
        "1.0 2.0 3.0 4.0 5.0 6.0 7.0 8.0 9.0 10.0 11.0 12.0 13.0"
    );
}

// ── rank > 2: unchanged flat form (§5.3) ─────────────────────────────────────

#[test]
fn rank3_falls_back_to_the_flat_form() {
    let t = Tensor::new((1..=8).map(|x| x as f64).collect(), &[2, 2, 2]);
    assert_eq!(
        t.to_string(),
        "shape=[2, 2, 2] values=[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]"
    );
}

// ── Debug is unchanged (§9, RFC-020 owns it) ────────────────────────────────

#[test]
fn debug_output_is_unchanged_by_displays_existence() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    assert_eq!(
        format!("{t:?}"),
        "Tensor(shape=[2, 3], data=[1.0, 2.0, 3.0, 4.0, 5.0, 6.0])"
    );
}

#[test]
fn debug_and_display_are_visibly_different_forms() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    assert_ne!(format!("{t:?}"), t.to_string());
}

// ── dynamic tensors (§5.5, `#[cfg(feature = "dynamic")]`) ──────────────────

#[cfg(feature = "dynamic")]
mod dynamic_tests {
    use crate::Tensor;
    use crate::dynamic::Element;

    #[test]
    fn dynamic_rank0_float_keeps_the_decimal_point() {
        // C1: Element::Float overrides Element's own Display (which would drop
        // the .0, indistinguishable from an Int) with {:?} on the inner f64.
        let t = Tensor::from_elements(vec![Element::Float(3.0)], &[]);
        assert_eq!(t.to_string(), "3.0");
    }

    #[test]
    fn dynamic_rank1_mixed_types_render_via_elements_own_display() {
        let t = Tensor::from_elements(
            vec![
                Element::Float(1.5),
                Element::Int(2),
                Element::text("hi"),
                Element::Bool(true),
                Element::None,
            ],
            &[5],
        );
        assert_eq!(t.to_string(), " 1.5    2   hi true None");
    }

    #[test]
    fn dynamic_float_and_int_stay_visually_distinct_at_the_same_value() {
        // C1's motivating case: a dynamic tensor is precisely where Int and
        // Float coexist, and the whole point of a mixed-type view is lost if
        // a whole-number Float renders identically to an Int of that value.
        let t = Tensor::from_elements(
            vec![
                Element::Float(2.0),
                Element::Int(2),
                Element::Float(1.5),
                Element::Int(7),
            ],
            &[2, 2],
        );
        assert_eq!(t.to_string(), "2.0 2\n1.5 7");
        // Element's own Display (unmodified, per the review) would have rendered
        // this Float indistinguishably from the Int beside it.
        assert_eq!(Element::Float(2.0).to_string(), "2");
    }

    #[test]
    fn dynamic_rank2_renders_as_a_grid() {
        let t = Tensor::from_elements(
            vec![
                Element::Int(1),
                Element::Int(2),
                Element::Int(3),
                Element::Int(4),
            ],
            &[2, 2],
        );
        assert_eq!(t.to_string(), "1 2\n3 4");
    }

    #[test]
    fn dynamic_rank3_falls_back_to_the_flat_form() {
        let t = Tensor::from_elements((1..=8).map(Element::Int).collect(), &[2, 2, 2]);
        assert_eq!(
            t.to_string(),
            "shape=[2, 2, 2] values=[1, 2, 3, 4, 5, 6, 7, 8]"
        );
    }

    #[test]
    fn dynamic_truncation_matches_the_numeric_constants() {
        let values: Vec<Element> = (1..=13).map(Element::Int).collect();
        let t = Tensor::from_elements(values, &[13]);
        assert!(t.to_string().contains("... 1 more values"));
        assert!(!format!("{t:#}").contains("more values"));
    }
}
