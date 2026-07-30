//! `repeat` vs `tile`, and evaluating a function over a `meshgrid` grid (RFC-087).
//!
//! Run: cargo run --example 58_repeat_tile_meshgrid
//!
//! ## Teaching points
//! - `repeat` repeats each **element**; `tile` repeats the **whole tensor** — the
//!   single most confused pair in this area, shown here on the same input.
//! - `repeat` is explicit allocation, unlike broadcasting, which materializes
//!   nothing implicitly.
//! - `meshgrid` builds the two coordinate grids used to evaluate `f(x, y)` over a
//!   rectangular grid without a nested loop at the call site.

use matten::Tensor;

fn main() {
    println!("== repeat vs tile: the classic confusion ==");
    let a = Tensor::from_vec(vec![1.0, 2.0, 3.0]);
    println!("input           {:?}", a.as_slice());

    let repeated = a.repeat(2);
    println!(
        "repeat(2)       {:?}   (each ELEMENT repeated in place)",
        repeated.as_slice()
    );
    assert_eq!(repeated.as_slice(), &[1.0, 1.0, 2.0, 2.0, 3.0, 3.0]);

    let tiled = a.tile(&[2]);
    println!(
        "tile(&[2])      {:?}   (the WHOLE tensor repeated)",
        tiled.as_slice()
    );
    assert_eq!(tiled.as_slice(), &[1.0, 2.0, 3.0, 1.0, 2.0, 3.0]);

    println!();
    println!("== repeat_axis: rank preserved ==");
    let m = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let along_rows = m.repeat_axis(2, 0);
    println!(
        "[[1,2],[3,4]].repeat_axis(2, 0) -> shape {:?}",
        along_rows.shape()
    );
    println!("  values {:?}", along_rows.as_slice());
    assert_eq!(along_rows.shape(), &[4, 2]);
    assert_eq!(
        along_rows.as_slice(),
        &[1.0, 2.0, 1.0, 2.0, 3.0, 4.0, 3.0, 4.0]
    );

    println!();
    println!("== meshgrid: evaluating f(x, y) over a grid ==");
    let x = Tensor::from_vec(vec![0.0, 1.0, 2.0]); // len 3
    let y = Tensor::from_vec(vec![0.0, 10.0]); // len 2 (unequal length, on purpose)
    let (gx, gy) = Tensor::meshgrid(&x, &y);
    println!("x = {:?}, y = {:?}", x.as_slice(), y.as_slice());
    println!(
        "meshgrid output shape: {:?} (== [len(y), len(x)])",
        gx.shape()
    );
    assert_eq!(gx.shape(), &[2, 3]);
    assert_eq!(gy.shape(), &[2, 3]);

    // f(x, y) = x + y, evaluated at every grid point without a manual loop.
    let f = &gx + &gy;
    println!("f(x, y) = x + y over the grid:");
    println!("  gx     {:?}", gx.as_slice());
    println!("  gy     {:?}", gy.as_slice());
    println!("  f      {:?}", f.as_slice());
    assert_eq!(f.shape(), &[2, 3]);
    assert_eq!(f.as_slice(), &[0.0, 1.0, 2.0, 10.0, 11.0, 12.0]);

    println!();
    println!("58_repeat_tile_meshgrid: OK");
}
