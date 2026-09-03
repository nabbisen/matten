use crate::shape::{coord_to_flat, flat_to_coord, strides_for_shape};
use proptest::prelude::*;

#[test]
fn strides_are_row_major() {
    assert_eq!(strides_for_shape(&[2, 3, 4]), vec![12, 4, 1]);
    assert_eq!(strides_for_shape(&[5]), vec![1]);
    assert_eq!(strides_for_shape(&[]), Vec::<usize>::new());
}

#[test]
fn coord_out_of_bounds_is_none() {
    assert_eq!(coord_to_flat(&[2, 0], &[2, 3]), None);
    assert_eq!(coord_to_flat(&[0], &[2, 3]), None); // rank mismatch
}

#[test]
fn index_round_trip() {
    let shapes: &[&[usize]] = &[&[], &[1], &[5], &[2, 3], &[3, 1, 4], &[2, 2, 2, 2]];
    for &shp in shapes {
        let len: usize = shp.iter().product();
        for flat in 0..len {
            let coord = flat_to_coord(flat, shp);
            assert_eq!(coord.len(), shp.len());
            assert_eq!(
                coord_to_flat(&coord, shp),
                Some(flat),
                "shape {shp:?} flat {flat}"
            );
        }
    }
}

// ---- P2: index round-trip (RFC-128) ----------------------------------------
//
// for any valid shape and any flat index in range:
//     coord_to_flat(flat_to_coord(i)) == i
//
// `index_round_trip` above already covers this for six hand-picked shapes;
// this property generalizes it to the shapes RFC-127's edge cases live in —
// rank 0, zero dimensions, and everything `proptest_support::small_shape`
// generates — while staying bounded so the test itself cannot allocate more
// than a few thousand elements.

proptest! {
    #[test]
    fn p2_index_round_trip_property(
        shp in crate::proptest_support::small_shape(),
        raw_flat in any::<usize>(),
    ) {
        let len: usize = shp.iter().product();
        if len == 0 {
            // No valid flat index exists for a zero-element shape; nothing to
            // round-trip. (A rank-0 scalar has len == 1, not 0 — see shape.rs.)
            return Ok(());
        }
        let flat = raw_flat % len;
        let coord = flat_to_coord(flat, &shp);
        prop_assert_eq!(coord.len(), shp.len());
        prop_assert_eq!(coord_to_flat(&coord, &shp), Some(flat));
    }
}
