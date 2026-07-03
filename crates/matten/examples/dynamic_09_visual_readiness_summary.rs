//! Visual readiness summary for a dynamic tensor.
//!
//! Run: cargo run --example dynamic_09_visual_readiness_summary --features dynamic
//!
//! The report stays local to this example. It shows what is present, what is
//! missing, what is numeric under the default policy, and what an explicit
//! conversion policy changes.

#[cfg(not(feature = "dynamic"))]
fn main() {
    println!("Requires --features dynamic");
}

#[cfg(feature = "dynamic")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use matten::{Element, NumericPolicy, Tensor};

    let t = Tensor::from_elements(
        vec![
            Element::Float(1.0),
            Element::text("2.5"),
            Element::None,
            Element::Int(4),
            Element::text("6.0"),
            Element::Float(8.0),
        ],
        &[2, 3],
    );

    println!("== Dynamic values ==");
    println!("shape            {:?}", t.shape());
    println!("values           {:?}", t.to_elements());
    println!("summary          {}", t.schema_summary());
    assert!(t.is_dynamic());
    assert_eq!(t.shape(), &[2, 3]);
    assert_eq!(t.count_none(), 1);

    println!();
    println!("== Readiness masks ==");
    let none_mask = t.none_mask();
    let numeric_mask = t.numeric_mask();
    println!("none_mask        {:?}", none_mask.as_slice());
    println!("numeric_mask     {:?}", numeric_mask.as_slice());
    println!("numeric_ready    {}", t.is_numeric_convertible());
    assert_eq!(none_mask.as_slice(), &[0.0, 0.0, 1.0, 0.0, 0.0, 0.0]);
    assert_eq!(numeric_mask.as_slice(), &[1.0, 0.0, 0.0, 1.0, 0.0, 1.0]);
    assert!(!t.is_numeric_convertible());

    println!();
    println!("== Conversion ==");
    match t.try_numeric() {
        Err(e) => println!("strict          Err: {e}"),
        Ok(_) => panic!("strict conversion should reject Text and None"),
    }

    let clean = t.try_numeric_with(NumericPolicy::default().none_as(0.0).allow_text_parse())?;
    println!("explicit policy none_as(0.0) + allow_text_parse()");
    println!("clean shape     {:?}", clean.shape());
    println!("clean values    {:?}", clean.as_slice());
    assert!(!clean.is_dynamic());
    assert_eq!(clean.shape(), &[2, 3]);
    assert_eq!(clean.as_slice(), &[1.0, 2.5, 0.0, 4.0, 6.0, 8.0]);

    println!();
    println!("dynamic_09_visual_readiness_summary: OK");
    Ok(())
}
