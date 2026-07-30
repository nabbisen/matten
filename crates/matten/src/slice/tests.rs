use crate::{MattenError, Tensor};

// ---- SliceBuilder -------------------------------------------------------

#[test]
fn builder_all_all() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let s = t.slice().all().all().build().unwrap();
    assert_eq!(s, t);
}

#[test]
fn builder_index_first_row() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let row = t.slice().index(0).all().build().unwrap();
    assert_eq!(row.shape(), &[3]);
    assert_eq!(row.as_slice(), &[1.0, 2.0, 3.0]);
}

#[test]
fn builder_index_second_row() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let row = t.slice().index(1).all().build().unwrap();
    assert_eq!(row.shape(), &[3]);
    assert_eq!(row.as_slice(), &[4.0, 5.0, 6.0]);
}

#[test]
fn builder_range_rows() {
    // [3, 4] tensor; take rows 0..2 (all rows), all cols
    let t = Tensor::new((1..=12).map(|x| x as f64).collect(), &[3, 4]);
    let s = t.slice().range(0..2).all().build().unwrap();
    assert_eq!(s.shape(), &[2, 4]);
    assert_eq!(s.as_slice(), &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
}

#[test]
fn builder_range_cols() {
    let t = Tensor::new((1..=6).map(|x| x as f64).collect(), &[2, 3]);
    let s = t.slice().all().range(1..3).build().unwrap();
    assert_eq!(s.shape(), &[2, 2]);
    assert_eq!(s.as_slice(), &[2.0, 3.0, 5.0, 6.0]);
}

#[test]
fn builder_range_from() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[4]);
    let s = t.slice().range(2..).build().unwrap();
    assert_eq!(s.as_slice(), &[3.0, 4.0]);
}

#[test]
fn builder_range_to() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[4]);
    let s = t.slice().range(..2).build().unwrap();
    assert_eq!(s.as_slice(), &[1.0, 2.0]);
}

#[test]
fn builder_range_full() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[4]);
    let s = t.slice().range(..).build().unwrap();
    assert_eq!(s, t);
}

#[test]
fn builder_inclusive_range() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[4]);
    let s = t.slice().range(1..=2).build().unwrap();
    assert_eq!(s.as_slice(), &[2.0, 3.0]);
}

#[test]
fn builder_index_all_axes_gives_scalar() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let s = t.slice().index(1).index(0).build().unwrap();
    assert!(s.is_scalar());
    assert_eq!(s.as_slice(), &[3.0]);
}

#[test]
fn builder_rank_mismatch_is_err() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    // Only one spec for a rank-2 tensor
    let err = t.slice().all().build().unwrap_err();
    assert!(matches!(err, MattenError::Slice { .. }));
}

#[test]
fn builder_out_of_bounds_index_is_err() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let err = t.slice().index(5).all().build().unwrap_err();
    assert!(matches!(err, MattenError::Slice { .. }));
}

#[test]
fn builder_result_is_independent() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let s = t.slice().index(0).all().build().unwrap();
    // The original tensor is unchanged
    assert_eq!(t.len(), 6);
    assert_eq!(s.len(), 3);
}

// ---- slice_str ----------------------------------------------------------

#[test]
fn slice_str_all() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let s = t.slice_str(":, :").unwrap();
    assert_eq!(s, t);
}

#[test]
fn slice_str_first_row() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let s = t.slice_str("0, :").unwrap();
    assert_eq!(s.shape(), &[3]);
    assert_eq!(s.as_slice(), &[1.0, 2.0, 3.0]);
}

#[test]
fn slice_str_range() {
    let t = Tensor::new((1..=12).map(|x| x as f64).collect(), &[3, 4]);
    let s = t.slice_str("0:2, :").unwrap();
    assert_eq!(s.shape(), &[2, 4]);
}

#[test]
fn slice_str_range_from() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[4]);
    let s = t.slice_str("2:").unwrap();
    assert_eq!(s.as_slice(), &[3.0, 4.0]);
}

#[test]
fn slice_str_range_to() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[4]);
    let s = t.slice_str(":2").unwrap();
    assert_eq!(s.as_slice(), &[1.0, 2.0]);
}

#[test]
fn slice_str_step() {
    let t = Tensor::new((0..=9).map(|x| x as f64).collect(), &[10]);
    let s = t.slice_str("0:10:2").unwrap();
    assert_eq!(s.as_slice(), &[0.0, 2.0, 4.0, 6.0, 8.0]);
}

#[test]
fn slice_str_whitespace_ignored() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let a = t.slice_str("0,:").unwrap();
    let b = t.slice_str(" 0 , : ").unwrap();
    assert_eq!(a, b);
}

#[test]
fn slice_str_matches_builder() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let from_str = t.slice_str("0:2, :").unwrap();
    let from_builder = t.slice().range(0..2).all().build().unwrap();
    assert_eq!(from_str, from_builder);
}

#[test]
fn slice_str_malformed_is_err() {
    // All of these must return Err (never panic, never silently accept)
    // "0::" was previously accepted as "0:" — now rejected (trailing colon)
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    for bad in &["0::", "a:b", ":::", "", "x"] {
        assert!(
            t.slice_str(bad).is_err(),
            "expected Err for {:?} but got Ok",
            bad
        );
    }
}

#[test]
fn slice_str_too_many_dims_is_err() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let err = t.slice_str("0, 0, 0").unwrap_err();
    assert!(matches!(err, MattenError::Slice { .. }));
}

#[test]
fn slice_str_oversized_is_err() {
    let t = Tensor::new(vec![1.0, 2.0], &[2]);
    let long = "0:1, ".repeat(200);
    let err = t.slice_str(&long).unwrap_err();
    assert!(matches!(err, MattenError::Slice { .. }));
    assert!(err.to_string().contains("maximum length"));
}

// ---- negative indices (RFC-088) -----------------------------------------

#[test]
fn negative_index_and_range_forms_on_a_3_element_vector() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);

    assert_eq!(t.slice_str("-1").unwrap().as_slice(), &[3.0]);
    assert_eq!(t.slice_str("-2").unwrap().as_slice(), &[2.0]);
    assert_eq!(t.slice_str("-3").unwrap().as_slice(), &[1.0]); // n == dim
    assert_eq!(t.slice_str("0:-1").unwrap().as_slice(), &[1.0, 2.0]);
    assert_eq!(t.slice_str(":-1").unwrap().as_slice(), &[1.0, 2.0]);
    assert_eq!(t.slice_str("-2:").unwrap().as_slice(), &[2.0, 3.0]);
    assert_eq!(t.slice_str("-3:-1").unwrap().as_slice(), &[1.0, 2.0]);
}

#[test]
fn negative_index_on_every_axis_of_an_unequal_rank2_tensor() {
    // The analogue of RFC-087's unequal-length meshgrid test: a square [3, 3]
    // tensor cannot distinguish "resolved against axis 0" from "resolved
    // against axis 1", since both axes have the same size. [3, 2] can.
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[3, 2]);

    // "-1,:" -> last ROW (axis 0, size 3) -> row index 2 -> [5, 6]
    let last_row = m.slice_str("-1,:").unwrap();
    assert_eq!(last_row.shape(), &[2]);
    assert_eq!(last_row.as_slice(), &[5.0, 6.0]);

    // ":,-1" -> last COLUMN (axis 1, size 2) -> col index 1 -> [2, 4, 6]
    let last_col = m.slice_str(":,-1").unwrap();
    assert_eq!(last_col.shape(), &[3]);
    assert_eq!(last_col.as_slice(), &[2.0, 4.0, 6.0]);

    // "-1,-1" -> scalar, bottom-right element
    let corner = m.slice_str("-1,-1").unwrap();
    assert!(corner.is_scalar());
    assert_eq!(corner.as_slice(), &[6.0]);
}

#[test]
fn negative_index_mixed_signs_in_one_spec() {
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[3, 2]);
    // "0:-1,-1": rows 0..2 (dropping the last row), last column -> [2, 4]
    let s = m.slice_str("0:-1,-1").unwrap();
    assert_eq!(s.shape(), &[2]);
    assert_eq!(s.as_slice(), &[2.0, 4.0]);
}

#[test]
fn negative_index_n_equals_dim_plus_1_errors() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);
    assert!(t.slice_str("-3").is_ok()); // n == dim
    assert!(t.slice_str("-4").is_err()); // n == dim + 1
}

#[test]
fn negative_out_of_range_errors_rather_than_clamps() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);

    let err_index = t.slice_str("-10").unwrap_err();
    assert!(matches!(err_index, MattenError::Slice { .. }));

    let err_range = t.slice_str("-10:").unwrap_err();
    assert!(matches!(err_range, MattenError::Slice { .. }));
    // Must NOT behave like Python's clamping "return the whole axis".
    assert_ne!(
        t.slice_str("-10:").ok().map(|r| r.as_slice().to_vec()),
        Some(vec![1.0, 2.0, 3.0])
    );
}

#[test]
fn negative_out_of_range_error_message_shows_written_and_resolved_forms() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);
    let err = t.slice_str("-10").unwrap_err();
    let message = err.to_string();
    assert!(
        message.contains("-10") && message.contains("-7"),
        "message should show both the written form (-10) and the resolved index (-7): {message}"
    );
}

#[test]
fn inverted_range_message_names_written_forms_when_negative() {
    // RFC-088 review round-1 R1: an inverted range whose bounds both resolve
    // in-range still needs to show what was WRITTEN, not just the resolved
    // numbers -- "-1:-3" and "-1:0" would otherwise produce byte-identical
    // messages, since both resolve to "2 > 0".
    let t = Tensor::new(vec![1.0, 2.0, 3.0], &[3]); // dim = 3

    let err = t.slice_str("-1:-3").unwrap_err();
    let message = err.to_string();
    assert!(
        message.contains("-1")
            && message.contains("-3")
            && message.contains('2')
            && message.contains('0'),
        "message should name both written forms (-1, -3) and both resolutions (2, 0): {message}"
    );

    // Mixed signs: only the negative bound needs "(resolves to ...)"; the
    // non-negative one is unchanged since resolved == written for it already.
    let mixed_err = t.slice_str("2:-3").unwrap_err();
    let mixed_message = mixed_err.to_string();
    assert!(
        mixed_message.contains("-3") && mixed_message.contains('0'),
        "message should name the written negative end (-3) and its resolution (0): {mixed_message}"
    );
}

#[test]
fn inverted_range_message_unchanged_when_both_bounds_non_negative() {
    // The pre-existing (positive-only) message must stay byte-identical.
    // resolve_spec's errors carry no input spec (that belongs to the parser),
    // so the message has no `for "..."` prefix -- unchanged from before RFC-088.
    let t = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);
    let err = t.slice_str("2:1").unwrap_err();
    assert_eq!(
        err.to_string(),
        "matten slice error: range start 2 > end 1 for axis 0 in slice_str"
    );
}

#[test]
fn negative_step_reversal_is_still_a_parse_error() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);
    assert!(t.slice_str("::-1").is_err());
}

#[test]
fn negative_zero_behaves_as_zero() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);
    assert_eq!(
        t.slice_str("-0").unwrap().as_slice(),
        t.slice_str("0").unwrap().as_slice()
    );
}

#[test]
fn preexisting_specs_still_parse_to_identical_results() {
    // Regression: every spec valid before RFC-088 must still parse to the
    // identical result now that index/start/end accept a leading '-'.
    let t = Tensor::new((0..=9).map(|x| x as f64).collect(), &[10]);
    assert_eq!(t.slice_str(":").unwrap(), t.slice_str(":").unwrap());
    assert_eq!(t.slice_str("0").unwrap().as_slice(), &[0.0]);
    assert_eq!(t.slice_str("0:2").unwrap().as_slice(), &[0.0, 1.0]);
    assert_eq!(
        t.slice_str("2:").unwrap().as_slice(),
        &[2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
    );
    assert_eq!(t.slice_str(":2").unwrap().as_slice(), &[0.0, 1.0]);
    assert_eq!(
        t.slice_str("0:10:2").unwrap().as_slice(),
        &[0.0, 2.0, 4.0, 6.0, 8.0]
    );
}
