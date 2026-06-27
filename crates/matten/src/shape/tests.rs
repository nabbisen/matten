use crate::shape::{coord_to_flat, flat_to_coord, strides_for_shape};

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
