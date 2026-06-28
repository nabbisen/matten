//! Dynamic JSON ingestion: parse JSON with mixed and missing values into a clean
//! numeric tensor.
//!
//! The JSON counterpart to the CSV on-ramp examples (`dynamic_02_missing_values`,
//! `dynamic_05_dirty_csv_cleanup`). `from_json_dynamic` accepts heterogeneous JSON —
//! integers, floats, and `null` — and lands it in a dynamic tensor, which a
//! `NumericPolicy` then coerces to a clean `f64` tensor. The format differs from CSV;
//! the on-ramp does not.
//!
//! Run: cargo run --example dynamic_08_json_ingestion --features dynamic,json

#[cfg(not(feature = "dynamic"))]
fn main() {
    println!("Requires --features dynamic");
}

#[cfg(feature = "dynamic")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(feature = "json"))]
    use matten::Element;
    use matten::{NumericPolicy, Tensor};

    // A small JSON table with mixed numeric kinds and a missing cell (`null`).
    // `from_json_dynamic` preserves each cell's original kind (Int / Float / None)
    // until you decide how to coerce — exactly like the CSV on-ramp.
    #[cfg(feature = "json")]
    let t = {
        let json = "[[1, 2.5, null], [4.0, 5, 6]]";
        Tensor::from_json_dynamic(json)?
    };
    // Without the `json` feature, build the same dynamic tensor directly so the rest
    // of the example still runs and asserts identically.
    #[cfg(not(feature = "json"))]
    let t = Tensor::from_elements(
        vec![
            Element::Int(1),
            Element::Float(2.5),
            Element::None,
            Element::Float(4.0),
            Element::Int(5),
            Element::Int(6),
        ],
        &[2, 3],
    );

    assert_eq!(t.shape(), &[2, 3]);
    assert!(t.is_dynamic());
    assert_eq!(t.count_none(), 1);
    println!(
        "parsed: shape={:?}, none_count={}",
        t.shape(),
        t.count_none()
    );

    // The missing cell is visible via none_mask (1.0 where the JSON had `null`).
    let mask = t.none_mask();
    assert_eq!(mask.get(&[0, 2]), Some(1.0)); // null at [0,2]
    assert_eq!(mask.get(&[0, 0]), Some(0.0)); // 1 is present
    println!("none_mask OK");

    // Strict conversion fails: a missing value has no numeric meaning on its own.
    assert!(t.try_numeric().is_err());
    println!("try_numeric (strict): Err (expected — missing value)");

    // A policy makes the coercion explicit: treat missing as 0.0; Int and Float both
    // become f64. The result is a plain numeric tensor, ready for computation.
    let clean = t.try_numeric_with(NumericPolicy::default().none_as(0.0))?;
    assert!(!clean.is_dynamic());
    assert_eq!(clean.shape(), &[2, 3]);
    assert_eq!(clean.as_slice(), &[1.0, 2.5, 0.0, 4.0, 5.0, 6.0]);
    println!("clean f64 = {:?}", clean.as_slice());

    println!("All assertions passed.");
    Ok(())
}
