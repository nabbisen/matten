//! Mutable dynamic element access (RFC-104): `get_element_mut`.
//!
//! T8 and T9 are the pair that prove `materialize()` is used correctly
//! rather than unconditionally — neither is inferable from values alone.

mod dynamic_mutation_tests {
    use crate::Tensor;
    use crate::dynamic::Element;

    /// `Arc::ptr_eq` on the two tensors' dynamic storage — the only way to
    /// tell "still shared" from "detached with identical contents" from the
    /// public API. Both fields are `pub(crate)`, reachable from this
    /// in-crate test (same helper RFC-102's slicing tests use).
    fn shares_storage(a: &Tensor, b: &Tensor) -> bool {
        std::sync::Arc::ptr_eq(
            &a.dynamic.as_ref().unwrap().storage,
            &b.dynamic.as_ref().unwrap().storage,
        )
    }

    fn storage_ptr(t: &Tensor) -> *const Vec<Element> {
        std::sync::Arc::as_ptr(&t.dynamic.as_ref().unwrap().storage)
    }

    // T7: write lands, for every Element variant, not just Int/Float.
    #[test]
    fn get_element_mut_writes_every_variant() {
        let mut t = Tensor::from_elements(
            vec![
                Element::Int(1),
                Element::Float(2.0),
                Element::text("a"),
                Element::Bool(false),
                Element::None,
            ],
            &[5],
        );
        *t.get_element_mut(&[0]).unwrap() = Element::Int(99);
        *t.get_element_mut(&[1]).unwrap() = Element::Float(9.5);
        *t.get_element_mut(&[2]).unwrap() = Element::text("z");
        *t.get_element_mut(&[3]).unwrap() = Element::Bool(true);
        *t.get_element_mut(&[4]).unwrap() = Element::text("was none");

        assert_eq!(t.get_element(&[0]), Some(Element::Int(99)));
        assert_eq!(t.get_element(&[1]), Some(Element::Float(9.5)));
        assert_eq!(t.get_element(&[2]), Some(Element::text("z")));
        assert_eq!(t.get_element(&[3]), Some(Element::Bool(true)));
        assert_eq!(t.get_element(&[4]), Some(Element::text("was none")));
    }

    #[test]
    fn get_element_mut_out_of_range_is_none_and_leaves_tensor_unchanged() {
        let mut t = Tensor::from_elements(vec![Element::Int(1), Element::Int(2)], &[2]);
        assert_eq!(t.get_element_mut(&[9]), None);
        assert_eq!(t.get_element_mut(&[0, 0]), None); // wrong rank
        assert_eq!(t.to_elements(), vec![Element::Int(1), Element::Int(2)]);
    }

    // T8 -- the one that matters: slice a dynamic tensor, write through the
    // slice, and assert BOTH halves. (a) alone passes if materialize() just
    // copies but the source happened to look right anyway; (b) alone passes
    // if the two never shared storage in the first place (i.e. RFC-102's
    // sharing regressed silently). Only both together prove copy-on-write.
    #[test]
    fn writing_through_a_dynamic_slice_detaches_and_leaves_source_unchanged() {
        let source = Tensor::from_elements((0..6).map(Element::Int).collect(), &[2, 3]);
        let mut slice = source.slice().index(0).all().build().unwrap(); // [0, 1, 2]
        assert!(
            shares_storage(&source, &slice),
            "precondition: slice must share storage with source before any write"
        );

        *slice.get_element_mut(&[1]).unwrap() = Element::Float(42.0);

        assert_eq!(
            slice.to_elements(),
            vec![Element::Int(0), Element::Float(42.0), Element::Int(2)]
        );
        assert_eq!(
            source.to_elements(),
            (0..6).map(Element::Int).collect::<Vec<_>>(),
            "(a) the source's elements must be unchanged after writing through the slice"
        );
        assert!(
            !shares_storage(&source, &slice),
            "(b) the first write must detach the slice's storage (Arc::ptr_eq must now be false)"
        );
    }

    // T9: a tensor that is ALREADY uniquely owned and contiguous must not
    // reallocate on write -- materialize() is documented as a no-op in that
    // case (E6), and this proves it rather than trusting the doc comment.
    #[test]
    fn get_element_mut_on_a_unique_tensor_does_not_reallocate() {
        let mut t = Tensor::from_elements(vec![Element::Int(1), Element::Int(2)], &[2]);
        let before = storage_ptr(&t);
        *t.get_element_mut(&[0]).unwrap() = Element::Int(10);
        let after_first_write = storage_ptr(&t);
        *t.get_element_mut(&[1]).unwrap() = Element::Int(20);
        let after_second_write = storage_ptr(&t);

        assert_eq!(
            before, after_first_write,
            "materialize() must no-op (not reallocate) on an already-unique, \
             already-contiguous tensor"
        );
        assert_eq!(
            before, after_second_write,
            "stable across a second write too"
        );
    }

    // A coordinate out of range on a SHARED slice must not materialize it --
    // an out-of-range call that pays for a full copy before returning None
    // is a performance bug with no visible symptom (handoff SS4.2). This is
    // provably guaranteed here by coord_to_flat's own per-axis check, which
    // runs and returns None (short-circuiting via `?`) before get_element_mut
    // ever reaches `self.dynamic` -- see the review request for the source
    // proof. Kept as a black-box regression test: it does not care WHERE the
    // short-circuit happens, only that materialize() is never reached.
    #[test]
    fn out_of_range_write_on_a_slice_does_not_materialize() {
        let source = Tensor::from_elements((0..6).map(Element::Int).collect(), &[2, 3]);
        let mut slice = source.slice().index(0).all().build().unwrap();
        assert!(shares_storage(&source, &slice));

        assert_eq!(slice.get_element_mut(&[99]), None);

        assert!(
            shares_storage(&source, &slice),
            "an out-of-range get_element_mut must return before materialize() runs -- \
             if it detached storage anyway, the bounds check ran second, which pays \
             for a full copy on every failed call with no visible symptom"
        );
    }
}
