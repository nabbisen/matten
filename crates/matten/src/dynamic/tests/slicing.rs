//! Slicing on dynamic tensors (RFC-102).
//!
//! Four of these tests exist specifically because the corresponding bug is
//! invisible to a value-only assertion: a slice that copies instead of
//! sharing, or nests views instead of composing, produces plausible-looking
//! output. Storage identity (`Arc::ptr_eq`) and the E9 outer-shape trap
//! (`get_element` on a slice) are asserted directly, not inferred.

mod dynamic_slicing_tests {
    use crate::Tensor;
    use crate::dynamic::Element;

    /// `Arc::ptr_eq` on the two tensors' dynamic storage — the only way to
    /// tell "shared" from "copied with identical contents" from the public
    /// API. Both fields are `pub(crate)`, reachable from this in-crate test.
    fn shares_storage(a: &Tensor, b: &Tensor) -> bool {
        std::sync::Arc::ptr_eq(
            &a.dynamic.as_ref().unwrap().storage,
            &b.dynamic.as_ref().unwrap().storage,
        )
    }

    fn grid_2x3() -> Tensor {
        // [ [0, 1, 2],
        //   [3, 4, 5] ]  -- flat storage order 0..6
        Tensor::from_elements((0..6).map(Element::Int).collect(), &[2, 3])
    }

    // T1 -- slice() on a dynamic tensor: is_dynamic() true, shape correct.
    #[test]
    fn builder_slice_on_dynamic_returns_dynamic() {
        let t = grid_2x3();
        let s = t.slice().range(0..1).all().build().unwrap();
        assert!(s.is_dynamic());
        assert_eq!(s.shape(), &[1, 3]);
    }

    // T2 -- slice_str() likewise.
    #[test]
    fn slice_str_on_dynamic_returns_dynamic() {
        let t = grid_2x3();
        let s = t.slice_str("0:1,:").unwrap();
        assert!(s.is_dynamic());
        assert_eq!(s.shape(), &[1, 3]);
    }

    // T3 -- storage is SHARED, asserted via Arc identity, not value equality.
    // A slice that copied instead of sharing would pass every assertion below
    // except this one.
    #[test]
    fn slice_shares_storage_with_the_source() {
        let t = grid_2x3();
        let s = t.slice().index(0).all().build().unwrap();
        assert!(
            shares_storage(&t, &s),
            "slice must Arc::clone the source's storage, not copy it"
        );
    }

    // T4 -- composition: slicing a slice must map through the existing view
    // rather than nesting. Asserts BOTH the values (a nesting bug is correct
    // for the first slice only) AND that the second slice still shares the
    // ORIGINAL tensor's storage (not just its immediate parent's).
    #[test]
    fn slicing_a_slice_composes_and_still_shares_the_original_storage() {
        // [ [0,1,2], [3,4,5], [6,7,8] ]
        let t = Tensor::from_elements((0..9).map(Element::Int).collect(), &[3, 3]);

        let rows_1_2 = t.slice().range(1..3).all().build().unwrap(); // [[3,4,5],[6,7,8]]
        assert_eq!(rows_1_2.shape(), &[2, 3]);
        assert!(shares_storage(&t, &rows_1_2));

        let second_row = rows_1_2.slice().index(1).all().build().unwrap(); // [6,7,8]
        assert_eq!(second_row.shape(), &[3]);
        assert_eq!(
            (0..3)
                .map(|i| second_row.get_element(&[i]).unwrap())
                .collect::<Vec<_>>(),
            vec![Element::Int(6), Element::Int(7), Element::Int(8)],
            "a nesting bug is correct for the first slice only -- this is the second"
        );
        assert!(
            shares_storage(&t, &second_row),
            "composition must map through rows_1_2's existing Indexed view into t's \
             storage, not nest a view over a view"
        );
    }

    // T5 -- rank-0 collapse: a fully-indexed slice yields shape [] and reads
    // its one element.
    #[test]
    fn fully_indexed_slice_collapses_to_rank_0() {
        let t = grid_2x3();
        let s = t.slice().index(1).index(2).build().unwrap();
        assert!(s.is_dynamic());
        assert_eq!(s.shape(), &[] as &[usize]);
        assert_eq!(s.get_element(&[]), Some(Element::Int(5)));
    }

    // T6 -- Text, None, and Bool survive a slice unchanged (round-trip via
    // get_element), not just Int/Float.
    #[test]
    fn text_none_and_bool_survive_a_slice_unchanged() {
        let t = Tensor::from_elements(
            vec![
                Element::text("a"),
                Element::None,
                Element::Bool(true),
                Element::text("b"),
                Element::Int(9),
                Element::Bool(false),
            ],
            &[2, 3],
        );
        let s = t.slice().index(1).all().build().unwrap(); // second row
        assert_eq!(s.get_element(&[0]), Some(Element::text("b")));
        assert_eq!(s.get_element(&[1]), Some(Element::Int(9)));
        assert_eq!(s.get_element(&[2]), Some(Element::Bool(false)));

        let first_col = t.slice().all().index(0).build().unwrap(); // first column: [a, b]
        assert_eq!(first_col.get_element(&[0]), Some(Element::text("a")));
        assert_eq!(first_col.get_element(&[1]), Some(Element::text("b")));

        let middle_col = t.slice().all().index(1).build().unwrap(); // middle column: [None, 9]
        assert_eq!(middle_col.get_element(&[0]), Some(Element::None));
        assert_eq!(middle_col.get_element(&[1]), Some(Element::Int(9)));
    }

    // T7 -- the E9 trap: get_element on a slice must resolve through the
    // SLICE's own shape, not the parent's. Column 1 of a 2x3 tensor makes the
    // two definitely differ: a wrong outer shape would misread as element
    // [0,0] of the parent (0) instead of [0,1] (1).
    #[test]
    fn get_element_on_a_slice_reads_the_slices_own_position() {
        let t = grid_2x3(); // [[0,1,2],[3,4,5]]
        let column_1 = t.slice().all().index(1).build().unwrap(); // [1, 4]
        assert_eq!(column_1.shape(), &[2]);
        assert_eq!(
            column_1.get_element(&[0]),
            Some(Element::Int(1)),
            "must read the slice's own position (t[0,1]=1), not the parent's t[0,0]=0"
        );
        assert_eq!(column_1.get_element(&[1]), Some(Element::Int(4)));
    }

    // T8 (numeric side) -- a numeric slice on a dynamic-feature build behaves
    // identically to the non-dynamic build; the full pre-existing numeric
    // suite (crates/matten/src/slice/tests.rs, 35 tests) is unmodified and
    // run in both feature profiles as part of the gate, not duplicated here.
    #[test]
    fn numeric_slice_is_unaffected_by_the_dynamic_branch() {
        let t = Tensor::new((0..6).map(|x| x as f64).collect(), &[2, 3]);
        let s = t.slice().index(1).all().build().unwrap();
        assert!(!s.is_dynamic());
        assert_eq!(s.as_slice(), &[3.0, 4.0, 5.0]);
    }

    // slice_str's grammar (negative indices, step, All) is shared code (E5) --
    // spot-check it reaches the dynamic path too, not just the builder.
    #[test]
    fn slice_str_step_and_negative_index_reach_the_dynamic_path() {
        let t = Tensor::from_elements((0..10).map(Element::Int).collect(), &[10]);
        let stepped = t.slice_str("0:10:2").unwrap();
        assert_eq!(
            (0..5)
                .map(|i| stepped.get_element(&[i]).unwrap())
                .collect::<Vec<_>>(),
            vec![0, 2, 4, 6, 8]
                .into_iter()
                .map(Element::Int)
                .collect::<Vec<_>>()
        );

        let last = t.slice_str("-1").unwrap();
        assert_eq!(last.shape(), &[] as &[usize]);
        assert_eq!(last.get_element(&[]), Some(Element::Int(9)));
    }

    // The old rejection is gone: neither entry point returns Unsupported for
    // a dynamic tensor any more (RFC-102 §8 risk 3 -- this is intended).
    #[test]
    fn dynamic_slice_no_longer_returns_unsupported() {
        let t = grid_2x3();
        let err = t.slice().index(0).all().build();
        assert!(err.is_ok());
        let err_str = t.slice_str("0,:");
        assert!(err_str.is_ok());
    }
}
