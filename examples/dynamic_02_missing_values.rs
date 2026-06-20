//! Detecting and handling missing values (Element::None).
//!
//! Run: cargo run --example dynamic_02_missing_values --features dynamic,csv

#[cfg(not(feature = "dynamic"))]
fn main() {
    println!("Requires --features dynamic");
}

#[cfg(feature = "dynamic")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(feature = "csv"))]
    use matten::Element;
    use matten::Tensor;

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

    assert_eq!(t.shape(), &[3, 3]);
    assert_eq!(t.len(), 9);
    assert_eq!(t.count_none(), 2);
    println!("shape={:?}, none_count={}", t.shape(), t.count_none());

    // none_mask: which cells are missing?
    let mask = t.none_mask();
    assert_eq!(mask.shape(), &[3, 3]);
    assert_eq!(mask.get(&[1, 1]), Some(1.0)); // None at [1,1]
    assert_eq!(mask.get(&[2, 2]), Some(1.0)); // None at [2,2]
    assert_eq!(mask.get(&[0, 0]), Some(0.0)); // not None
    println!("none_mask OK");

    // try_numeric fails because of None values
    assert!(t.try_numeric().is_err());
    println!("try_numeric on raw: Err (expected)");

    println!("All assertions passed.");
    Ok(())
}
