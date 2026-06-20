//! Detecting and handling missing values (Element::None).
//!
//! Run: cargo run --example dynamic_02_missing_values --features dynamic,csv
//!
//! In `matten`, missing data is always explicit. There is no automatic
//! coercion of None to 0 or NaN.

#[cfg(not(feature = "dynamic"))]
fn main() {
    println!("Requires --features dynamic");
}

#[cfg(feature = "dynamic")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(feature = "csv"))]
    use matten::Element;
    use matten::Tensor;

    // CSV with missing fields
    #[cfg(feature = "csv")]
    let t = {
        let csv = "10.0,20.0,30.0\n40.0,,60.0\n70.0,80.0,\n";
        Tensor::from_csv_dynamic(csv)?
    };
    #[cfg(not(feature = "csv"))]
    let t = Tensor::from_elements(
        vec![
            Element::Float(10.0),
            Element::Float(20.0),
            Element::Float(30.0),
            Element::Float(40.0),
            Element::None,
            Element::Float(60.0),
            Element::Float(70.0),
            Element::Float(80.0),
            Element::None,
        ],
        &[3, 3],
    );

    println!("shape      = {:?}", t.shape());
    println!("none count = {}", t.count_none()); // 2

    // none_mask: which cells are missing?
    let mask = t.none_mask();
    println!(
        "none mask:\n  row0={:?}\n  row1={:?}\n  row2={:?}",
        &mask.as_slice()[0..3],
        &mask.as_slice()[3..6],
        &mask.as_slice()[6..9],
    );

    // try_numeric fails because of None values
    assert!(t.try_numeric().is_err());
    println!("try_numeric on raw: Err (expected)");

    Ok(())
}
