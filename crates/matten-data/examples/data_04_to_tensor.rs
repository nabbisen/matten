//! # data_04 — The output is a normal `matten::Tensor`
//!
//! Run: `cargo run -p matten-data --example data_04_to_tensor`
//!
//! ## What this shows
//! The shape and data order of the produced tensor, and that it is an ordinary
//! core [`matten::Tensor`] you can hand straight to core `matten` operations.
//!
//! ## Teaching points
//! - the shape is `[rows, columns]`;
//! - the data is **row-major**: row 0's values first, then row 1's, and so on;
//! - once converted, there is nothing `matten-data`-specific left — it is just a
//!   `Tensor`, so core reductions like `mean_axis` work directly.

// `matten-data` produces a core tensor; bring the core type into scope to use it.
use matten::Tensor;
use matten_data::Table;

fn main() -> Result<(), matten_data::MattenDataError> {
    let csv = "\
region,sales,cost
north,100,40
south,150,45
east,120,55";

    let tensor: Tensor = Table::from_csv_str(csv)?
        .select_columns(["sales", "cost"])?
        .try_numeric()?
        .to_tensor()?;

    // [rows, columns] = [3, 2].
    assert_eq!(tensor.shape(), &[3, 2]);

    // Row-major: [north.sales, north.cost, south.sales, south.cost, ...].
    assert_eq!(tensor.as_slice(), &[100.0, 40.0, 150.0, 45.0, 120.0, 55.0]);

    // It is a plain Tensor, so core `matten` operations apply. Mean over axis 0
    // (down the rows) gives the per-column mean: [mean(sales), mean(cost)].
    let column_means = tensor.mean_axis(0);
    println!("shape        : {:?}", tensor.shape());
    println!("column means : {:?}", column_means.as_slice());

    assert_eq!(column_means.shape(), &[2]);
    assert_eq!(
        column_means.as_slice(),
        &[(100.0 + 150.0 + 120.0) / 3.0, (40.0 + 45.0 + 55.0) / 3.0]
    );

    println!("data_04_to_tensor: OK");
    Ok(())
}
